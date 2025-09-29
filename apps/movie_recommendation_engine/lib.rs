use reqwest::Client;
use serde::Serialize;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};
use time::{OffsetDateTime, UtcOffset};

pub mod batch_movies_request;
pub mod server;
use lib::clients::openai::get_recommendations;

// ----------------- helpers -----------------

fn write_batches(path: &Path, batches: &[serde_json::Value]) -> std::io::Result<()> {
    let mut tmp = PathBuf::from(path);
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        tmp.set_extension(format!("{}.tmp", ext));
    } else {
        tmp.set_extension("tmp");
    }

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tmp)?;
    for b in batches {
        writeln!(f, "{}", b)?;
    }
    std::fs::rename(&tmp, path)?;
    Ok(())
}

fn read_batches(path: &Path) -> Vec<serde_json::Value> {
    let mut batches = Vec::new();
    if path.exists() {
        if let Ok(f) = File::open(path) {
            for l in BufReader::new(f).lines().flatten() {
                match serde_json::from_str::<serde_json::Value>(&l) {
                    Ok(v) => batches.push(v),
                    Err(_) => batches.push(serde_json::json!({ "raw": l })),
                }
            }
        } else {
            log::warn!("Could not open NDJSON for read ({})", path.display());
        }
    }
    batches
}

// Keep the LLM input compact to avoid token bloat.
#[derive(Serialize)]
struct LlmMovie {
    tmdb_id: u32,
    title: String,
    year: Option<u16>,
    genres: Vec<String>,
    runtime_min: Option<u32>,
    vote_avg: Option<f64>,
    overview: Option<String>,
}

// Output schema we expect back from OpenAI (JSON mode)
#[derive(serde::Deserialize)]
struct RecOut {
    bucket: String,
    count: usize,
    recommendations: Vec<RecItem>,
}
#[derive(serde::Deserialize, serde::Serialize)]
struct RecItem {
    tmdb_id: u32,
    reason: String,
}

// Helper to pull a year out of "YYYY-MM-DD"
fn year_from_release_date(d: &Option<String>) -> Option<u16> {
    d.as_ref()
        .and_then(|s| s.get(0..4))
        .and_then(|y| y.parse::<u16>().ok())
}

fn six_hour_bucket_tag(now_utc: OffsetDateTime) -> String {
    let hour = now_utc.hour();
    let bucket_start = (hour / 6) * 6; // 0,6,12,18
    format!(
        "{:04}-{:02}-{:02}T{:02}Z",
        now_utc.year(),
        u8::from(now_utc.month()) as u8,
        now_utc.day(),
        bucket_start
    )
}

/* ---------------------------
1) EVENT-DRIVEN: append only
--------------------------- */

/// Append/mutate NDJSON so each 6h window is one line. **No OpenAI here.**
pub fn append_event_to_ndjson(path: &str, line: &str) {
    // Parse event
    let event: serde_json::Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(e) => {
            log::error!("event JSON not valid; skipping: {e}");
            return;
        }
    };

    // Bucket values
    let now_utc = OffsetDateTime::now_utc().to_offset(UtcOffset::UTC);
    let bucket = six_hour_bucket_tag(now_utc);
    let now_iso = now_utc
        .format(&time::format_description::well_known::Rfc3339)
        .ok();

    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    // Read, upsert, write
    let mut batches = read_batches(p);

    let mut found = false;
    for b in &mut batches {
        if b.get("bucket").and_then(|v| v.as_str()) == Some(bucket.as_str()) {
            let events = if let Some(ev) = b.get_mut("events").and_then(|ev| ev.as_array_mut()) {
                ev
            } else {
                b["events"] = serde_json::json!([]);
                b.get_mut("events").unwrap().as_array_mut().unwrap()
            };
            let event_count = {
                events.push(event.clone());
                events.len()
            };
            b["updated_at"] = serde_json::json!(now_iso.clone());
            b["count"] = serde_json::json!(event_count);
            found = true;
            break;
        }
    }
    if !found {
        let batch = serde_json::json!({
            "bucket": bucket,
            "started_at": now_iso,
            "updated_at": now_iso,
            "count": 1usize,
            "events": [ event ]
        });
        batches.push(batch);
    }

    if let Err(e) = write_batches(p, &batches) {
        log::error!("write tmp NDJSON failed: {e}");
    }
}

/* ---------------------------------------
2) CRON-DRIVEN: generate recommendations
--------------------------------------- */

/// Generate/overwrite recommendations for a specific bucket tag.
/// Example bucket tag: "2025-09-29T12Z".
pub async fn generate_recommendations_for_bucket(path: &str, bucket_tag: &str, client: &Client) {
    let p = Path::new(path);
    let mut batches = read_batches(p);

    // Find target bucket (mutable)
    let mut_bucket = match batches
        .iter_mut()
        .find(|b| b.get("bucket").and_then(|v| v.as_str()) == Some(bucket_tag))
    {
        Some(b) => b,
        None => {
            log::warn!("No batch found for bucket {}", bucket_tag);
            return;
        }
    };

    // Collect tmdb_ids
    let mut ids: Vec<u32> = Vec::new();
    if let Some(events) = mut_bucket.get("events").and_then(|e| e.as_array()) {
        for ev in events {
            if let Some(id_val) = ev.get("tmdb_id") {
                if let Some(id) = id_val.as_u64().map(|n| n as u32) {
                    ids.push(id);
                } else if let Some(s) = id_val.as_str() {
                    if let Ok(n) = s.parse::<u32>() {
                        ids.push(n);
                    }
                }
            }
        }
    }
    ids.sort_unstable();
    ids.dedup();

    if ids.is_empty() {
        log::info!(
            "No tmdb_ids in bucket {}; skipping OpenAI call.",
            bucket_tag
        );
        return;
    }

    // Fetch TMDB details in parallel
    let (movies, _failures) = batch_movies_request::fetch_movies_batch(client, &ids).await;

    // Prepare compact movie set for LLM
    let llm_movies: Vec<LlmMovie> = movies
        .iter()
        .map(|m| LlmMovie {
            tmdb_id: m.id,
            title: m.title.clone(),
            year: year_from_release_date(&m.release_date),
            genres: m.genres.iter().map(|g| g.name.clone()).collect(),
            runtime_min: m.runtime,
            vote_avg: m.vote_average,
            overview: m.overview.as_ref().map(|o| {
                let o = o.trim();
                if o.len() > 320 {
                    format!("{}…", &o[..320])
                } else {
                    o.to_string()
                }
            }),
        })
        .collect();

    let watched_ids_json = serde_json::to_string(&ids).unwrap_or("[]".to_string());
    let llm_movies_json = serde_json::to_string(&llm_movies).unwrap_or("[]".to_string());

    // Use the batch's updated_at if present (nice to stamp output), else now
    let now_iso = OffsetDateTime::now_utc()
        .to_offset(UtcOffset::UTC)
        .format(&time::format_description::well_known::Rfc3339)
        .ok();

    let prompt = format!(
        r#"
        You are a movie recommendation engine. Return **ONLY** a JSON object (no prose).

        Input:
        - bucket: "{bucket}"
        - watched_tmdb_ids: {watched_tmdb_ids}
        - watched_details: {watched_details}

        Task:
        Given the user's recently watched movies, return **20** recommended movies.

        Rules:
        - Output **exactly** this JSON shape (no extra fields):
        {{
            "bucket": "string",
            "count": 10,
            "recommendations": [
              {{ "tmdb_id": <integer>, "reason": "short string (≤140 chars)" }},
              ...
            ]
        }}
        - All `tmdb_id` values must be integers.
        - Do **not** include any id in `watched_tmdb_ids` or duplicate any suggestion.
        - Prefer diversity across genres/years; mix obvious picks with a few surprises.
        - **Never** recommend adult or X-rated content.
        - Only recommend non-english movies already popular in North American culture (e.g. Bollywood, etc).
        - Keep `reason` concise and specific (theme, tone, cast, director, vibe).
        - Mix seasonality in as well: for example, if it's September or October, recommend more horror movies, or if it's November or December, recommend more christmas movies, etc.

        Return only the JSON object.
        "#,
        bucket = bucket_tag,
        watched_tmdb_ids = watched_ids_json,
        watched_details = llm_movies_json
    );

    let t0 = std::time::Instant::now();
    log::info!(
        "Calling OpenAI for bucket {} ({} ids)…",
        bucket_tag,
        ids.len()
    );
    match get_recommendations(client, &prompt).await {
        Ok(json_only) => {
            let parsed: Result<RecOut, _> = serde_json::from_str(&json_only);
            match parsed {
                Ok(rec_out) => {
                    if rec_out.bucket != bucket_tag {
                        log::warn!(
                            "OpenAI bucket mismatch: expected {}, got {}",
                            bucket_tag,
                            rec_out.bucket
                        );
                    }
                    // Overwrite recommendations
                    let rec_val = serde_json::to_value(&rec_out.recommendations)
                        .unwrap_or(serde_json::json!([]));
                    mut_bucket["recommendations"] = rec_val;
                    mut_bucket["recommendations_generated_at"] = serde_json::json!(now_iso);

                    if let Err(e) = write_batches(p, &batches) {
                        log::error!("Failed to write recommendations to NDJSON: {e}");
                    } else {
                        log::info!(
                            "Wrote {} recommendations to bucket {} in {:?}",
                            rec_out.count,
                            bucket_tag,
                            t0.elapsed()
                        );
                    }
                }
                Err(e) => {
                    log::error!("OpenAI JSON parse failed: {e}. Raw: {}", json_only);
                }
            }
        }
        Err(e) => {
            log::error!("OpenAI call failed after {:?}: {:#}", t0.elapsed(), e);
        }
    }
}

/// Convenience for cron: operate on the *latest* bucket (by `updated_at` or `started_at`).
pub async fn generate_recommendations_for_latest_bucket(path: &str, client: &Client) {
    let p = Path::new(path);
    let batches = read_batches(p);
    if batches.is_empty() {
        log::info!("No batches found.");
        return;
    }

    // Pick the latest by `updated_at` (fallback to `started_at` if missing)
    let mut latest: Option<(&str, OffsetDateTime)> = None;

    for b in &batches {
        if let Some(bucket_str) = b.get("bucket").and_then(|v| v.as_str()) {
            // Skip buckets that already have recommendations
            if b.get("recommendations").is_some() {
                continue;
            }

            // choose updated_at > started_at fallback
            let ts_str = b
                .get("updated_at")
                .and_then(|v| v.as_str())
                .or_else(|| b.get("started_at").and_then(|v| v.as_str()));

            if let Some(ts_str) = ts_str {
                if let Ok(ts) =
                    OffsetDateTime::parse(ts_str, &time::format_description::well_known::Rfc3339)
                {
                    if latest.map(|(_, t)| ts > t).unwrap_or(true) {
                        latest = Some((bucket_str, ts));
                    }
                }
            }
        }
    }

    if let Some((bucket, _)) = latest {
        generate_recommendations_for_bucket(path, bucket, client).await;
    } else {
        log::info!("Could not resolve a latest bucket to process.");
    }
}
