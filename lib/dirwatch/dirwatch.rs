use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use chrono::Local;
use crossbeam_channel::{unbounded, Receiver};
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    path::Path,
    time::{Duration, Instant},
};

#[derive(Parser, Debug)]
#[command(
    name = "dirwatch",
    about = "Log new entries created directly under a directory"
)]
struct Args {
    /// Directory to watch (defaults to Desktop)
    #[arg(short, long)]
    path: Option<PathBuf>,
    /// Debounce seconds
    #[arg(long, default_value_t = 2)]
    debounce_secs: u64,
}

pub fn watch_dir(path: &Path, debounce: Duration) -> Result<Receiver<(String, String)>> {
    if !path.exists() {
        anyhow::bail!("watch path does not exist: {}", path.display());
    }

    // outbound channel for consumers
    let (out_tx, out_rx) = unbounded::<(String, String)>();

    // raw FS events channel
    let (tx, raw_rx) = std::sync::mpsc::channel();
    let base = PathBuf::from(path);

    // Create watcher in main thread to ensure it's ready before returning
    let mut watcher = recommended_watcher(tx.clone())?;
    let canonical_base = base.canonicalize()?;
    watcher.watch(&canonical_base, RecursiveMode::NonRecursive)?;

    std::thread::spawn(move || {
        // ♻️ keep watcher alive inside the thread
        let _watcher = watcher;

        let tick = Duration::from_millis(250);
        let mut last_flush = Instant::now();
        let mut pending: HashMap<PathBuf, Instant> = HashMap::new();

        loop {
            // poll FS events
            match raw_rx.recv_timeout(tick) {
                Ok(Ok(Event { kind, paths, .. })) => {
                    let interesting = matches!(
                        kind,
                        EventKind::Create(_)
                            | notify::event::EventKind::Modify(notify::event::ModifyKind::Name(_))
                    );
                    if interesting {
                        for p in paths {
                            if let Ok(canonical_parent) =
                                p.parent().map(|parent| parent.canonicalize()).transpose()
                            {
                                if canonical_parent == Some(canonical_base.clone()) {
                                    pending.entry(p).or_insert_with(Instant::now);
                                }
                            }
                        }
                    }
                }
                Ok(Err(_e)) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
            }

            // debounce flush
            if last_flush.elapsed() >= tick {
                let now = Instant::now();
                let mut ready = Vec::new();
                pending.retain(|p, first| {
                    if now.duration_since(*first) >= debounce {
                        ready.push(p.clone());
                        false
                    } else {
                        true
                    }
                });
                for p in ready {
                    let name = p
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("(unknown)")
                        .to_string();
                    let ts = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    let _ = out_tx.send((name, ts));
                }
                last_flush = now;
            }
        }
    });

    Ok(out_rx)
}
