use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use log::{error, info, warn};
use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
struct Hook {
    #[serde(default)]
    event: String,
    #[serde(default)]
    user: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    rating_key: String,
    #[serde(default)]
    guid: String,
    #[serde(flatten)]
    rest: serde_json::Value,
}

fn extract_tmdb_id(guid: &str) -> Option<String> {
    static RE1: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r"tmdb://(?P<id>\d+)").unwrap());
    static RE2: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
        Regex::new(r"com\.plexapp\.agents\.themoviedb://(?P<id>\d+)").unwrap()
    });
    RE1.captures(guid)
        .or_else(|| RE2.captures(guid))
        .and_then(|c| c.name("id").map(|m| m.as_str().to_string()))
}

fn append_ndjson(path: &str, line: &str) {
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = create_dir_all(parent);
    }
    match OpenOptions::new().append(true).create(true).open(path) {
        Ok(mut f) => {
            if let Err(e) = writeln!(f, "{}", line) {
                error!("ndjson write error: {e}");
            }
        }
        Err(e) => error!("ndjson open error: {e}"),
    }
}

#[post("/tautulli")]
async fn tautulli(body: String) -> impl Responder {
    let mut tmdb_id: Option<String> = None;
    let mut title_debug = String::new();

    match serde_json::from_str::<Hook>(&body) {
        Ok(h) => {
            tmdb_id = extract_tmdb_id(&h.guid);
            title_debug = h.title.clone();
            info!(
                "Webhook: event={} user={} title={} tmdb_id={:?}",
                h.event, h.user, h.title, tmdb_id
            );
        }
        Err(e) => {
            warn!("JSON parse failed ({e}). Raw body will be stored.");
        }
    }

    let record = serde_json::json!({
        "ts": OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).ok(),
        "title": title_debug,
        "tmdb_id": tmdb_id,
        "raw": body,
    });

    let log_path = env::var("NDJSON_PATH").unwrap_or_else(|_| "/data/reco_events.ndjson".into());
    append_ndjson(&log_path, &record.to_string());

    HttpResponse::Ok().finish()
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let bind = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8088".into());
    println!("movie_recommendation_engine up on http://{bind}");
    HttpServer::new(|| App::new().service(tautulli).service(healthz))
        .bind(bind)?
        .run()
        .await
}
