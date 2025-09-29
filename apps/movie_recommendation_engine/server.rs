use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use log::{info, warn};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use std::env;
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
    guid: String,
}

fn extract_tmdb_id(guid: &str) -> Option<String> {
    static RE1: Lazy<Regex> = Lazy::new(|| Regex::new(r"tmdb://(?P<id>\d+)").unwrap());
    static RE2: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"com\.plexapp\.agents\.themoviedb://(?P<id>\d+)").unwrap());
    RE1.captures(guid)
        .or_else(|| RE2.captures(guid))
        .and_then(|c| c.name("id").map(|m| m.as_str().to_string()))
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
    crate::append_ndjson_and_probe_tmdb(&log_path, &record.to_string(), &reqwest::Client::new())
        .await;

    HttpResponse::Ok().finish()
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

/// Launch the Actix server. Reads `BIND_ADDR` (default `0.0.0.0:8088`).
pub async fn run_server() -> std::io::Result<()> {
    let bind = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8088".into());
    println!("movie_recommendation_engine up on http://{bind}");
    HttpServer::new(|| App::new().service(tautulli).service(healthz))
        .bind(bind)?
        .run()
        .await
}
