use actix_web::{get, HttpResponse, Responder};
use serde::Deserialize;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};
use v_htmlescape::escape;

// default to your project path
// const DEFAULT_FILE: &str = "db/movie_recommendation_engine/movie_recommendation_engine.ndjson";

#[derive(Debug, Deserialize)]
struct RecItem {
    tmdb_id: u32,
    reason: String,
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

#[get("/")]
async fn index() -> impl Responder {
    let path = env::var("NDJSON_PATH").unwrap();
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
            ul{list-style:none;padding:0;margin:.5rem 0 0;display:grid;gap:.5rem}
            li{border:1px dashed #5554;border-radius:10px;padding:.6rem .8rem}
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
                for r in recs {
                    // link to TMDB for convenience
                    let url = format!("https://www.themoviedb.org/movie/{}", r.tmdb_id);
                    html.push_str(&format!(
                        r#"<li><a href="{url}" target="_blank" rel="noopener">{tmdb}</a>
                           — {reason}</li>"#,
                        url = escape(&url),
                        tmdb = escape(&format!("TMDB {}", r.tmdb_id)),
                        reason = escape(&r.reason)
                    ));
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
