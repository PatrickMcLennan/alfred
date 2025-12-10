use actix_web::{get, HttpResponse, Responder};
use futures::{stream, StreamExt};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    sync::Mutex,
    time::{Duration, Instant},
};
use v_htmlescape::escape;

// default to your project path
// const DEFAULT_FILE: &str = "db/movie_recommendation_engine/movie_recommendation_engine.ndjson";

#[derive(Debug, Deserialize)]
struct RecItem {
    tmdb_id: u32,
}

#[derive(Debug, Deserialize)]
struct BatchLine {
    // we only deserialize what we need; unknown fields are ignored
    bucket: Option<String>,
    count: Option<usize>,
    updated_at: Option<String>,
    recommendations_generated_at: Option<String>,
    recommendations: Option<Vec<RecItem>>,
}

#[derive(Clone, Debug, Deserialize)]
struct TmdbMovie {
    title: Option<String>,
    release_date: Option<String>,
    poster_path: Option<String>,
    vote_average: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct RadarrMovie {
    #[serde(rename = "tmdbId")]
    tmdb_id: Option<u32>,
    #[serde(rename = "hasFile")]
    has_file: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct PlexGuid {
    #[serde(rename = "id")]
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PlexSearchMetadata {
    #[serde(rename = "ratingKey")]
    rating_key: Option<String>,
    #[serde(rename = "Guid")]
    guid: Option<Vec<PlexGuid>>,
}

#[derive(Debug, Deserialize)]
struct PlexSearchContainer {
    #[serde(rename = "Metadata")]
    metadata: Option<Vec<PlexSearchMetadata>>,
}

#[derive(Debug, Deserialize)]
struct PlexSearchResponse {
    #[serde(rename = "MediaContainer")]
    media_container: Option<PlexSearchContainer>,
}

#[derive(Debug, Deserialize)]
struct PlexMetaEntry {
    #[serde(rename = "viewCount")]
    view_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct PlexMetaContainer {
    #[serde(rename = "Metadata")]
    metadata: Option<Vec<PlexMetaEntry>>,
}

#[derive(Debug, Deserialize)]
struct PlexMetaResponse {
    #[serde(rename = "MediaContainer")]
    media_container: Option<PlexMetaContainer>,
}

#[derive(Clone, Debug)]
struct CardMeta {
    tmdb: Option<TmdbMovie>,
    has_file: Option<bool>,
    watched: Option<bool>,
}

struct CacheEntry<T> {
    ts: Instant,
    val: Option<T>,
}

static TMDB_CACHE: Lazy<Mutex<std::collections::HashMap<u32, CacheEntry<TmdbMovie>>>> =
    Lazy::new(|| Mutex::new(std::collections::HashMap::new()));
static RADARR_CACHE: Lazy<Mutex<std::collections::HashMap<u32, CacheEntry<bool>>>> =
    Lazy::new(|| Mutex::new(std::collections::HashMap::new()));
static PLEX_CACHE: Lazy<Mutex<std::collections::HashMap<u32, CacheEntry<bool>>>> =
    Lazy::new(|| Mutex::new(std::collections::HashMap::new()));

fn get_cached<T: Clone>(
    map: &Mutex<std::collections::HashMap<u32, CacheEntry<T>>>,
    tmdb_id: u32,
    ttl: Duration,
) -> Option<Option<T>> {
    let guard = map.lock().ok()?;
    guard.get(&tmdb_id).and_then(|e| {
        if Instant::now().duration_since(e.ts) < ttl {
            Some(e.val.clone())
        } else {
            None
        }
    })
}

fn set_cached<T: Clone>(
    map: &Mutex<std::collections::HashMap<u32, CacheEntry<T>>>,
    tmdb_id: u32,
    val: Option<T>,
) {
    if let Ok(mut guard) = map.lock() {
        guard.insert(
            tmdb_id,
            CacheEntry {
                ts: Instant::now(),
                val,
            },
        );
    }
}

fn read_batches(path: &str) -> Vec<BatchLine> {
    let mut out = Vec::new();
    if let Ok(f) = File::open(path) {
        for line in BufReader::new(f).lines().flatten() {
            if let Ok(b) = serde_json::from_str::<BatchLine>(&line) {
                if b.bucket.is_some() {
                    out.push(b);
                }
            }
        }
    }
    // newest first by bucket string (lexicographically works with your tag format)
    out.sort_by(|a, b| b.bucket.cmp(&a.bucket));
    out
}

fn year_from_release_date(d: &Option<String>) -> Option<String> {
    d.as_ref().and_then(|s| s.get(0..4)).map(|y| y.to_string())
}

async fn fetch_tmdb_movie(client: &Client, api_key: &str, id: u32) -> Option<TmdbMovie> {
    if let Some(cached) = get_cached(&TMDB_CACHE, id, Duration::from_secs(3)) {
        return cached;
    }
    let url = format!(
        "https://api.themoviedb.org/3/movie/{id}?api_key={api_key}&language=en-US",
        id = id,
        api_key = api_key
    );
    match client
        .get(url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            let val = resp.json::<TmdbMovie>().await.ok();
            set_cached(&TMDB_CACHE, id, val.clone());
            val
        }
        _ => None,
    }
}

async fn fetch_radarr_has_file(
    client: &Client,
    base_url: &str,
    api_key: &str,
    tmdb_id: u32,
) -> Option<bool> {
    if let Some(cached) = get_cached(&RADARR_CACHE, tmdb_id, Duration::from_secs(3)) {
        return cached;
    }
    let url = format!(
        "{}/api/v3/movie/lookup?term=tmdb%3A{}",
        base_url.trim_end_matches('/'),
        tmdb_id
    );
    match client
        .get(url)
        .header("X-Api-Key", api_key)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            if let Ok(list) = resp.json::<Vec<RadarrMovie>>().await {
                for m in list {
                    if m.tmdb_id == Some(tmdb_id) {
                        set_cached(&RADARR_CACHE, tmdb_id, m.has_file);
                        return m.has_file;
                    }
                }
            }
            set_cached(&RADARR_CACHE, tmdb_id, None);
            None
        }
        _ => {
            set_cached(&RADARR_CACHE, tmdb_id, None);
            None
        }
    }
}

async fn fetch_plex_rating_key(
    client: &Client,
    base_url: &str,
    token: &str,
    section: Option<&str>,
    tmdb_id: u32,
) -> Option<String> {
    if let Some(cached) = get_cached(&PLEX_CACHE, tmdb_id, Duration::from_secs(3)) {
        // Reuse the watched cache to also gate rating-key attempts; if we cached watched status,
        // we already resolved a rating key earlier.
        if cached.is_some() {
            // Cached watched info exists; skip re-fetching rating key
            return None;
        }
    }
    // Prefer section-guid query; fallback to global search.
    let urls = if let Some(sec) = section {
        vec![format!(
            "{}/library/sections/{}/all?guid=tmdb%3A%2F%2F{}&type=1&includeGuids=1&X-Plex-Token={}&format=json&X-Plex-Container-Size=1",
            base_url.trim_end_matches('/'),
            sec,
            tmdb_id,
            token
        )]
    } else {
        vec![format!(
            "{}/search?query=tmdb://{}&includeGuids=1&X-Plex-Token={}&format=json&X-Plex-Container-Size=1",
            base_url.trim_end_matches('/'),
            tmdb_id,
            token
        )]
    };

    for url in urls {
        if let Ok(resp) = client
            .get(&url)
            .header("Accept", "application/json")
            .header("X-Plex-Client-Identifier", "alfred-dashboard")
            .header("X-Plex-Product", "alfred-dashboard")
            .header("X-Plex-Version", "1.0")
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            if !resp.status().is_success() {
                println!(
                    "[dashboard] plex rating-key request failed status={} url={}",
                    resp.status(),
                    url
                );
                continue;
            }
            let body_txt = resp.text().await.unwrap_or_default();
            let parsed = serde_json::from_str::<PlexSearchResponse>(&body_txt).ok();
            if parsed.is_none() {
                println!(
                    "[dashboard] plex rating-key parse failed tmdb_id={} url={} body_len={} body_preview={}",
                    tmdb_id,
                    url,
                    body_txt.len(),
                    &body_txt.chars().take(200).collect::<String>()
                );
                continue;
            }
            let body = parsed.unwrap();
            let meta_opt = body.media_container.and_then(|c| c.metadata);
            if meta_opt.is_none() {
                println!(
                    "[dashboard] plex rating-key no metadata tmdb_id={} url={}",
                    tmdb_id, url
                );
                continue;
            }
            let meta = meta_opt.unwrap();
            for m in meta {
                if let Some(guid_list) = &m.guid {
                    let matches_tmdb = guid_list.iter().any(|g| {
                        g.id.as_deref()
                            .map(|s| s == format!("tmdb://{}", tmdb_id))
                            .unwrap_or(false)
                    });
                    if matches_tmdb {
                        if let Some(rk) = m.rating_key {
                            println!(
                                "[dashboard] tmdb_id={} plex rating key resolved via url={}",
                                tmdb_id, url
                            );
                            return Some(rk);
                        }
                    }
                }
            }
            println!(
                "[dashboard] plex rating-key no match tmdb_id={} url={}",
                tmdb_id, url
            );
        }
    }
    None
}

async fn fetch_plex_watched(
    client: &Client,
    base_url: &str,
    token: &str,
    rating_key: &str,
    tmdb_id: u32,
) -> Option<bool> {
    if let Some(cached) = get_cached(&PLEX_CACHE, tmdb_id, Duration::from_secs(3)) {
        return cached;
    }
    let url = format!(
        "{}/library/metadata/{}?X-Plex-Token={}",
        base_url.trim_end_matches('/'),
        rating_key,
        token
    );
    let resp = client
        .get(&url)
        .header("Accept", "application/json")
        .header("X-Plex-Client-Identifier", "alfred-dashboard")
        .header("X-Plex-Product", "alfred-dashboard")
        .header("X-Plex-Version", "1.0")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        println!(
            "[dashboard] plex watched request failed status={} url={}",
            resp.status(),
            url
        );
        return None;
    }
    let body_txt = resp.text().await.unwrap_or_default();
    let parsed = serde_json::from_str::<PlexMetaResponse>(&body_txt).ok();
    if parsed.is_none() {
        println!(
            "[dashboard] plex watched parse failed tmdb_id={} url={} body_len={}",
            tmdb_id,
            url,
            body_txt.len()
        );
        return None;
    }
    let body = parsed.unwrap();
    let meta = body.media_container?.metadata?;
    let view_count = meta.iter().find_map(|m| m.view_count);
    let watched = view_count.map(|c| c > 0);
    println!(
        "[dashboard] tmdb_id={} plex watched view_count={:?} url={}",
        tmdb_id, view_count, url
    );
    set_cached(&PLEX_CACHE, tmdb_id, watched);
    watched
}

#[get("/")]
async fn index() -> impl Responder {
    let path = env::var("NDJSON_PATH").unwrap();
    let tmdb_key = env::var("TMDB_API_KEY").ok();
    let radarr_url = env::var("RADARR_URL").ok();
    let radarr_key = env::var("RADARR_API_KEY").ok();
    let plex_url = env::var("PLEX_URL").ok();
    let plex_token = env::var("PLEX_TOKEN").ok();
    let plex_section = env::var("PLEX_SECTION").ok(); // e.g., movies library key "1"
    let client = Client::new();
    let batches = read_batches(&path);

    let mut html = String::new();
    html.push_str(
        r#"<!doctype html><meta charset="utf-8"><title>Alfred · Recommendations</title>"#,
    );
    html.push_str(
        r#"<style>
            :root{color-scheme:dark light}
            body{font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;margin:2rem;max-width:1100px}
            h1{margin:0 0 .5rem;font-size:1.6rem}
            .meta{color:#888;margin-bottom:1rem}
            .bucket{border:1px solid #7773;border-radius:12px;padding:1rem;margin:1rem 0}
            .hdr{display:flex;gap:.75rem;align-items:baseline;flex-wrap:wrap}
            .tag{background:#9992;border-radius:999px;padding:.2rem .6rem;font-size:.85rem}
            ul{list-style:none;padding:0;margin:.5rem 0 0;display:grid;gap:.75rem;grid-template-columns:repeat(auto-fill,minmax(240px,1fr))}
            li{border:1px dashed #5554;border-radius:12px;padding:.6rem .8rem;display:flex;gap:.75rem;align-items:flex-start}
            .poster-wrap{position:relative;flex-shrink:0}
            .poster{width:80px;height:120px;object-fit:cover;border-radius:8px;background:#3335;flex-shrink:0}
            .badge{position:absolute;top:6px;right:6px;border-radius:999px;width:28px;height:28px;display:grid;place-items:center;font-size:.9rem;font-weight:700;color:#fff;box-shadow:0 1px 6px #0006}
            .badge-watched{background:#2e7dd7;top:40px;}
            .badge-not-watched{background:#757575;top:40px;}
            .card-text{display:flex;flex-direction:column;gap:.35rem}
            .title{font-weight:600}
            .meta-line{font-size:.85rem;color:#aaa;display:flex;gap:.5rem;align-items:center}
            .tag-downloaded{background:#1dbf73;color:#000;border-radius:999px;padding:.15rem .55rem;font-weight:700}
            .tag-not-downloaded{background:#c62828;color:#fff;border-radius:999px;padding:.15rem .55rem;font-weight:700}
            .tag-watched{background:#2e7dd7;color:#fff;border-radius:999px;padding:.15rem .55rem;font-weight:700}
            .tag-not-watched{background:#757575;color:#fff;border-radius:999px;padding:.15rem .55rem;font-weight:700}
            a{color:inherit}
        </style>"#,
    );

    html.push_str("<h1>Alfred · Movie Recommendations</h1>");
    html.push_str(&format!(
        r#"<div class="meta">Source: <code>{}</code></div>"#,
        escape(&path)
    ));

    if batches.is_empty() {
        html.push_str("<p>No batches found.</p>");
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html);
    }

    for b in batches.iter().take(8) {
        // show the newest 8 buckets
        let bucket = b.bucket.as_deref().unwrap_or("-");
        let count = b.count.unwrap_or(0);
        let gen_ts = b
            .recommendations_generated_at
            .as_deref()
            .or(b.updated_at.as_deref())
            .unwrap_or("-");

        html.push_str(r#"<div class="bucket">"#);
        html.push_str(&format!(
            r#"<div class="hdr"><strong>Bucket:</strong> <span class="tag">{}</span>
               <span class="tag">plays: {}</span>
               <span class="tag">generated: {}</span></div>"#,
            escape(bucket),
            count,
            escape(gen_ts)
        ));

        match &b.recommendations {
            Some(recs) if !recs.is_empty() => {
                html.push_str("<ul>");
                let meta = {
                    let fetches = stream::iter(recs.iter().enumerate().map(|(idx, r)| {
                        let client = client.clone();
                        let tmdb_key = tmdb_key.clone();
                        let radarr_url = radarr_url.clone();
                        let radarr_key = radarr_key.clone();
                        let plex_url = plex_url.clone();
                        let plex_token = plex_token.clone();
                        let plex_section = plex_section.clone();
                        async move {
                            let tmdb = if let Some(k) = tmdb_key.as_ref() {
                                let t = fetch_tmdb_movie(&client, k, r.tmdb_id).await;
                                if t.is_none() {
                                    println!(
                                        "[dashboard] tmdb_id={} tmdb lookup failed",
                                        r.tmdb_id
                                    );
                                }
                                t
                            } else {
                                None
                            };
                            let has_file = if let (Some(u), Some(k)) =
                                (radarr_url.as_ref(), radarr_key.as_ref())
                            {
                                let hf = fetch_radarr_has_file(&client, u, k, r.tmdb_id).await;
                                println!(
                                    "[dashboard] tmdb_id={} radarr has_file={:?}",
                                    r.tmdb_id, hf
                                );
                                hf
                            } else {
                                None
                            };
                            let watched = if let (Some(u), Some(t)) =
                                (plex_url.as_ref(), plex_token.as_ref())
                            {
                                if let Some(rk) = fetch_plex_rating_key(
                                    &client,
                                    u,
                                    t,
                                    plex_section.as_deref(),
                                    r.tmdb_id,
                                )
                                .await
                                {
                                    let w = fetch_plex_watched(&client, u, t, &rk, r.tmdb_id).await;
                                    println!(
                                        "[dashboard] tmdb_id={} plex_rating_key={} watched={:?}",
                                        r.tmdb_id, rk, w
                                    );
                                    w
                                } else {
                                    println!(
                                        "[dashboard] tmdb_id={} plex rating key not found",
                                        r.tmdb_id
                                    );
                                    None
                                }
                            } else {
                                None
                            };
                            println!(
                                "[dashboard] tmdb_id={} has_file={:?} watched={:?}",
                                r.tmdb_id, has_file, watched
                            );
                            (
                                idx,
                                CardMeta {
                                    tmdb,
                                    has_file,
                                    watched,
                                },
                            )
                        }
                    }))
                    .buffer_unordered(8)
                    .collect::<Vec<_>>()
                    .await;
                    let mut m = vec![
                        CardMeta {
                            tmdb: None,
                            has_file: None,
                            watched: None,
                        };
                        recs.len()
                    ];
                    for (idx, cm) in fetches {
                        if idx < m.len() {
                            m[idx] = cm;
                        }
                    }
                    m
                };

                for (i, r) in recs.iter().enumerate() {
                    let cm = meta.get(i);
                    let tmdb = cm.and_then(|m| m.tmdb.as_ref());
                    if tmdb.is_none() {
                        // Skip entries where we couldn't fetch TMDB details (avoid blank posters).
                        continue;
                    }
                    let title = tmdb
                        .and_then(|t| t.title.clone())
                        .unwrap_or_else(|| format!("TMDB {}", r.tmdb_id));
                    let year = tmdb
                        .and_then(|t| year_from_release_date(&t.release_date))
                        .unwrap_or_else(|| "—".to_string());
                    let poster_url = tmdb
                        .and_then(|t| t.poster_path.clone())
                        .map(|p| format!("https://image.tmdb.org/t/p/w342{}", p));
                    let _rating = tmdb
                        .and_then(|t| t.vote_average)
                        .map(|v| format!("{:.1}", v));
                    let has_file = cm.and_then(|m| m.has_file);
                    let watched = cm.and_then(|m| m.watched);
                    let url = format!("https://www.themoviedb.org/movie/{}", r.tmdb_id);

                    html.push_str("<li>");
                    html.push_str(r#"<div class="poster-wrap">"#);
                    if let Some(p) = poster_url {
                        html.push_str(&format!(
                            r#"<a href="{url}" target="_blank" rel="noopener"><img class="poster" src="{p}" alt=""></a>"#,
                            url = escape(&url),
                            p = escape(&p)
                        ));
                    } else {
                        html.push_str(&format!(
                            r#"<a href="{url}" target="_blank" rel="noopener"><div class="poster"></div></a>"#,
                            url = escape(&url)
                        ));
                    }
                    if let Some(w) = watched {
                        let (cls, icon) = if w {
                            ("badge badge-watched", "✓")
                        } else {
                            ("badge badge-not-watched", "○")
                        };
                        html.push_str(&format!(
                            r#"<span class="{cls}">{icon}</span>"#,
                            cls = cls,
                            icon = icon
                        ));
                    }
                    html.push_str("</div>");
                    html.push_str(r#"<div class="card-text">"#);
                    html.push_str(&format!(
                        r#"<div class="title"><a href="{url}" target="_blank" rel="noopener">{title}</a> ({year})</div>"#,
                        url = escape(&url),
                        title = escape(&title),
                        year = escape(&year)
                    ));
                    if has_file.is_some() || watched.is_some() {
                        html.push_str(r#"<div class="meta-line">"#);
                        if let Some(hf) = has_file {
                            let icon = if hf { "⬇" } else { "⌁" };
                            let cls = if hf {
                                "tag-downloaded"
                            } else {
                                "tag-not-downloaded"
                            };
                            html.push_str(&format!(
                                r#"<span class="{cls}">{icon}</span>"#,
                                cls = cls,
                                icon = icon
                            ));
                        }
                        if let Some(w) = watched {
                            let icon = if w { "✓" } else { "○" };
                            let cls = if w { "tag-watched" } else { "tag-not-watched" };
                            html.push_str(&format!(
                                r#"<span class="{cls}">{icon}</span>"#,
                                cls = cls,
                                icon = icon
                            ));
                        }
                        html.push_str("</div>");
                    }
                    html.push_str("</div>");
                    html.push_str("</li>");
                }
                html.push_str("</ul>");
            }
            _ => {
                html.push_str("<p><em>No recommendations for this bucket.</em></p>");
            }
        }

        html.push_str("</div>");
    }

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
