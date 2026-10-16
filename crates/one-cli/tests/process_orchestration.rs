use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn fixture(source: &str) -> std::path::PathBuf {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("one-process-{id}.one"));
    fs::write(&path, source).unwrap();
    path
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_one")).args(args).output().unwrap()
}

#[test]
fn cli_process_orchestration_executes_only_with_explicit_target_scoped_permission() {
    let path = fixture("jalankan proses \"rustc\"\n");
    let output = run(&["run", path.to_str().unwrap(), "--izinkan", "process.execute", "--target", "rustc"]);
    let _ = fs::remove_file(&path);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("rustc"));
}

#[test]
fn cli_process_orchestration_is_denied_without_permission() {
    let path = fixture("jalankan proses \"rustc\"\n");
    let output = run(&["run", path.to_str().unwrap()]);
    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("N3205"));
}
