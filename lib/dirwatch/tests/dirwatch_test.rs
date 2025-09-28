use std::{fs, thread, time::Duration};
use tempfile::tempdir;

#[test]
fn detects_new_file_in_root() {
    let td = tempdir().unwrap();
    let rx = dirwatch::watch_dir(td.path(), Duration::from_millis(50)).unwrap();

    // create a file at the root
    let file_path = td.path().join("sample.mkv");
    fs::write(&file_path, b"test").unwrap();

    // wait up to ~5s for an event
    let mut got = None;
    for _ in 0..50 {
        if let Ok((name, _ts)) = rx.try_recv() {
            got = Some(name);
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }

    assert_eq!(got.as_deref(), Some("sample.mkv"));
}
