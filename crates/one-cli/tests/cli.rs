use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn fixture(name: &str, source: &str) -> std::path::PathBuf {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("one-{name}-{id}.one"));
    fs::write(&path, source).unwrap();
    path
}

#[test]
fn run_command_executes_program() {
    let path = fixture("run", "nama adalah \"Budi\"\ntampilkan nama\n");
    let output = Command::new(env!("CARGO_BIN_EXE_one")).arg("run").arg(&path).output().unwrap();
    let _ = fs::remove_file(&path);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "Budi");
}

#[test]
fn check_command_rejects_invalid_program() {
    let path = fixture("invalid", "tampilkan nama\n");
    let output = Command::new(env!("CARGO_BIN_EXE_one")).arg("check").arg(&path).output().unwrap();
    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
}
