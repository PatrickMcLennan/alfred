use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_sns::{types::MessageAttributeValue, Client};
use clap::Parser;
use std::{path::PathBuf, time::Duration};
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(name = "dispatch", about = "Watch and publish NEW events to SNS")]
struct Args {
    /// SNS topic ARN (must match the region you use)
    #[arg(long)]
    topic_arn: String,

    /// AWS region override (optional). If omitted, uses your CLI/default config.
    #[arg(long)]
    region: Option<String>,

    /// Debounce seconds
    #[arg(long, default_value_t = 2)]
    debounce_secs: u64,
}

fn expand_tilde(p: &str) -> PathBuf {
    if let Some(rest) = p.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }
    PathBuf::from(p)
}

fn resolve_watch_root() -> PathBuf {
    if let Ok(val) = std::env::var("MOVIE_DIR") {
        let pb = expand_tilde(val.trim());
        if !pb.as_os_str().is_empty() {
            return pb;
        }
    }
    let mut home = dirs::home_dir().expect("Could not resolve home directory");
    home.push("Desktop");
    home
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Resolve watch root from env with fallback
    let root = resolve_watch_root();

    // --- AWS config (fix deprecations) ---
    let mut cfg_loader = aws_config::defaults(BehaviorVersion::latest());
    if let Some(r) = &args.region {
        cfg_loader = cfg_loader.region(aws_sdk_sns::config::Region::new(r.clone()));
    }
    let cfg = cfg_loader.load().await;
    let sns = Client::new(&cfg);

    // --- Dir watcher (blocking channel) -> async bridge ---
    let rx_blocking =
        lib::dirwatch::dirwatch::watch_dir(&root, Duration::from_secs(args.debounce_secs))?;
    let (tx_async, mut rx_async) = mpsc::unbounded_channel::<(String, String)>();
    std::thread::spawn(move || {
        while let Ok(ev) = rx_blocking.recv() {
            let _ = tx_async.send(ev);
        }
    });

    println!(
        "notify_new_movie watching {} → SNS topic {}",
        root.display(),
        &args.topic_arn
    );

    while let Some((name, ts)) = rx_async.recv().await {
        let msg = format!("NEW: {name}\nTime: {ts}\nPath: {}", root.display());

        // Mark as Transactional (helps delivery; not strictly required)
        let sms_type = MessageAttributeValue::builder()
            .data_type("String")
            .string_value("Transactional")
            .build()
            .unwrap();

        let resp = sns
            .publish()
            .topic_arn(&args.topic_arn)
            .message(msg)
            .message_attributes("AWS.SNS.SMS.SMSType", sms_type)
            .send()
            .await;

        match resp {
            Ok(out) => println!(
                "Published: {} ({name})",
                out.message_id().unwrap_or("no-id")
            ),
            Err(e) => eprintln!("SNS publish failed for {name}: {e:?}"),
        }
    }

    Ok(())
}
