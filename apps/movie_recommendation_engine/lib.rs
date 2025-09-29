pub mod batch_movies_request;
pub mod server;

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};
use time::{OffsetDateTime, UtcOffset};

use reqwest::Client;

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

pub async fn append_ndjson_and_probe_tmdb(path: &str, line: &str, client: &Client) {
    // --- 1) Parse the incoming event JSON; if bad, bail early ---
    let event: serde_json::Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(e) => {
            log::error!("event JSON not valid; skipping: {e}");
            return;
        }
    };

    // --- 2) Determine current bucket (UTC) ---
    let now_utc = OffsetDateTime::now_utc().to_offset(UtcOffset::UTC);
    let bucket = six_hour_bucket_tag(now_utc);
    let now_iso = now_utc
        .format(&time::format_description::well_known::Rfc3339)
        .ok();

    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    // --- 3) Read all existing batches (lines) ---
    let mut batches: Vec<serde_json::Value> = Vec::new();
    if p.exists() {
        if let Ok(f) = File::open(p) {
            for l in BufReader::new(f).lines().flatten() {
                match serde_json::from_str::<serde_json::Value>(&l) {
                    Ok(v) => batches.push(v),
                    Err(_) => batches.push(serde_json::json!({ "raw": l })),
                }
            }
        } else {
            log::warn!("Could not open NDJSON for read ({})", p.display());
        }
    }

    // --- 4) Upsert our event into the current bucket ---
    let mut found = false;
    for b in &mut batches {
        if b.get("bucket").and_then(|v| v.as_str()) == Some(bucket.as_str()) {
            // ensure events is an array
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
            "events": [ event.clone() ]
        });
        batches.push(batch);
    }

    // --- 5) Atomically rewrite NDJSON file ---
    let mut tmp = PathBuf::from(path);
    tmp.set_extension("ndjson.tmp");
    if let Ok(mut f) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tmp)
    {
        for b in &batches {
            if let Err(e) = writeln!(f, "{}", b) {
                log::error!("write tmp NDJSON failed: {e}");
                return;
            }
        }
        if let Err(e) = std::fs::rename(&tmp, p) {
            log::error!("atomic rename failed: {e}");
            return;
        }
    } else {
        log::error!("open tmp NDJSON failed");
        return;
    }

    // --- 6) Collect ALL tmdb_ids in *current* bucket and probe TMDB in parallel ---
    // Re-scan the in-memory `batches` we already have.
    let current_bucket = batches
        .iter()
        .find(|b| b.get("bucket").and_then(|v| v.as_str()) == Some(bucket.as_str()));

    let mut ids: Vec<u32> = Vec::new();
    if let Some(b) = current_bucket {
        if let Some(events) = b.get("events").and_then(|e| e.as_array()) {
            for ev in events {
                if let Some(id_val) = ev.get("tmdb_id") {
                    // tmdb_id might be string or number; accept both
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
    }
    ids.sort_unstable();
    ids.dedup();

    if ids.is_empty() {
        log::info!("TMDB probe: no tmdb_id values found in bucket {}", bucket);
        return;
    }

    log::info!(
        "TMDB probe: fetching {} ids from bucket {} …",
        ids.len(),
        bucket
    );

    let (movies, failures) = batch_movies_request::fetch_movies_batch(client, &ids).await;

    // --- 7) Log raw JSON output so you can confirm it’s working ---
    match serde_json::to_string_pretty(&movies) {
        Ok(s) => log::info!("TMDB probe results (movies):\n{}", s),
        Err(e) => log::error!("Could not serialize TMDB movies: {e}"),
    }
    if !failures.is_empty() {
        match serde_json::to_string_pretty(&failures) {
            Ok(s) => log::warn!("TMDB probe failures:\n{}", s),
            Err(e) => log::error!("Could not serialize TMDB failures: {e}"),
        }
    }
}
