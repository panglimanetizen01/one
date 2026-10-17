use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn fixture(source: &str) -> std::path::PathBuf {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("one-database-{id}.one"));
    fs::write(&path, source).unwrap();
    path
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_one")).args(args).output().unwrap()
}

#[test]
fn cli_database_orchestration_executes_with_explicit_resource_permission() {
    let path = fixture("kueri database \":memory:\" dengan \"SELECT ?1\" dan \"42\"\n");
    let output = run(&["run", path.to_str().unwrap(), "--izinkan", "database.query", "--target", ":memory:"]);
    let _ = fs::remove_file(&path);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "42\n");
}

#[test]
fn cli_database_orchestration_is_denied_without_permission() {
    let path = fixture("kueri database \":memory:\" dengan \"SELECT ?1\" dan \"42\"\n");
    let output = run(&["run", path.to_str().unwrap()]);
    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("N3209"));
}

#[test]
fn cli_database_orchestration_is_denied_when_resource_is_out_of_scope() {
    let path = fixture("kueri database \":memory:\" dengan \"SELECT ?1\" dan \"42\"\n");
    let output = run(&["run", path.to_str().unwrap(), "--izinkan", "database.query", "--target", "other.db"]);
    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("N3209"));
}
