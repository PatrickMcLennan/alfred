use anyhow::Result;
use clap::Parser;
use std::thread;
use std::time::Duration as StdDuration;
use std::{path::PathBuf, time::Duration};

#[derive(Parser, Debug)]
#[command(
    name = "dirwatch",
    about = "Log new entries created directly under a directory"
)]
struct Args {
    /// Directory to watch
    #[arg(short, long)]
    path: PathBuf,
    /// Debounce seconds
    #[arg(long, default_value_t = 2)]
    debounce_secs: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let root = args.path.clone();

    let rx = dirwatch::watch_dir(&args.path, Duration::from_secs(args.debounce_secs))?;
    println!("Watching {}", args.path.display());

    for (name, ts) in rx.iter() {
        // tiny verify step to avoid printing when item was moved/removed
        let full = root.join(&name);
        // brief grace period to avoid race on quick renames
        thread::sleep(StdDuration::from_millis(100));
        if !full.exists() {
            continue; // skip events where the entry no longer exists here
        }
        println!("[{ts}] NEW: {name}");
    }

    #[allow(unreachable_code)]
    Ok(())
}
