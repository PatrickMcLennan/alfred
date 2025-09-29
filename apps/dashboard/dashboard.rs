use actix_web::{App, HttpServer};
use std::env;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind = env::var("DASHBOARD_BIND").unwrap();
    println!("dashboard listening on http://{bind}");
    HttpServer::new(|| App::new().service(routes::index::index))
        .bind(bind)?
        .run()
        .await
}
