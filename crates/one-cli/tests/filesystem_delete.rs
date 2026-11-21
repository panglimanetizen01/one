use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn run_in(directory: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_one"))
        .current_dir(directory)
        .args(args)
        .output()
        .unwrap()
}

fn fixture_root(label: &str) -> std::path::PathBuf {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("one-filesystem-delete-{label}-{id}"));
    fs::create_dir_all(&root).unwrap();
    root
}

#[test]
fn filesystem_read_grant_does_not_grant_delete_and_preserves_target() {
    let root = fixture_root("read-denied");
    let source = root.join("main.one");
    let target = root.join("target.txt");
    fs::write(&source, "hapus berkas \"target.txt\"\n").unwrap();
    fs::write(&target, b"KEEP_DELETE_DENIED").unwrap();

    let output = run_in(&root, &[
        "run", "main.one",
        "--izinkan", "filesystem.read",
        "--target", "target.txt",
    ]);

    assert!(!output.status.success(), "read-only grant unexpectedly allowed deletion");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("N3209"), "expected policy denial, got: {stderr}");
    assert_eq!(fs::read(&target).unwrap(), b"KEEP_DELETE_DENIED");
    let _ = fs::remove_dir_all(&root);
}

#[cfg(windows)]
#[test]
fn explicit_filesystem_delete_grant_deletes_only_the_scoped_target() {
    let root = fixture_root("windows-allowed");
    let source = root.join("main.one");
    let target = root.join("target.txt");
    let other = root.join("keep.txt");
    fs::write(&source, "hapus berkas \"target.txt\"\n").unwrap();
    fs::write(&target, b"DELETE_ME").unwrap();
    fs::write(&other, b"KEEP_OTHER").unwrap();

    let output = run_in(&root, &[
        "run", "main.one",
        "--izinkan", "filesystem.delete",
        "--target", "target.txt",
    ]);

    assert!(output.status.success(), "explicit delete failed: {}", String::from_utf8_lossy(&output.stderr));
    assert!(!target.exists(), "scoped target should be deleted");
    assert_eq!(fs::read(&other).unwrap(), b"KEEP_OTHER");
    let _ = fs::remove_dir_all(&root);
}

#[cfg(not(windows))]
#[test]
fn filesystem_delete_grant_fails_closed_without_native_delete_backend() {
    let root = fixture_root("unsupported");
    let source = root.join("main.one");
    let target = root.join("target.txt");
    fs::write(&source, "hapus berkas \"target.txt\"\n").unwrap();
    fs::write(&target, b"KEEP_UNSUPPORTED_DELETE").unwrap();

    let output = run_in(&root, &[
        "run", "main.one",
        "--izinkan", "filesystem.delete",
        "--target", "target.txt",
    ]);

    assert!(!output.status.success(), "unsupported native delete unexpectedly succeeded");
    assert_eq!(fs::read(&target).unwrap(), b"KEEP_UNSUPPORTED_DELETE");
    let _ = fs::remove_dir_all(&root);
}
