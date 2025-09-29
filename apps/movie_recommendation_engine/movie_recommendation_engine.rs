use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(
    name = "movie_recommendation_engine",
    about = "Tautulli webhook listener + (future) batch LLM job"
)]
struct Args {
    /// Run mode: "serve" (default). We can add "batch" later.
    #[arg(long, default_value = "serve")]
    mode: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Args::parse();

    match args.mode.as_str() {
        "serve" => movie_recommendation_engine::server::run_server().await,
        other => {
            eprintln!("Unknown mode: {other}. Try --mode serve");
            Ok(())
        }
    }
}
