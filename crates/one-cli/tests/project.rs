use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn temp_dir() -> std::path::PathBuf {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("one-project-{id}"))
}

#[test]
fn new_command_creates_project_layout() {
    let dir = temp_dir();
    let output = Command::new(env!("CARGO_BIN_EXE_one")).args(["new", dir.to_str().unwrap()]).output().unwrap();
    assert!(output.status.success());
    assert!(dir.join("one.toml").exists());
    assert!(dir.join("src/main.one").exists());
    let _ = fs::remove_dir_all(dir);
}

#[test]
fn build_command_creates_ir_artifact() {
    let dir = temp_dir();
    fs::create_dir_all(&dir).unwrap();
    let source = dir.join("main.one");
    fs::write(&source, "tampilkan \"Halo\"\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_one")).args(["build", source.to_str().unwrap()]).output().unwrap();
    assert!(output.status.success());
    assert!(dir.join("target/main.oneir").exists());
    let _ = fs::remove_dir_all(dir);
}
