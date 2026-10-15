use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn fixture(name: &str, source: &str) -> std::path::PathBuf {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("one-{name}-{id}.one"));
    fs::write(&path, source).unwrap();
    path
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_one")).args(args).output().unwrap()
}

#[test]
fn run_command_executes_program() {
    let path = fixture("run", "nama adalah \"Budi\"\ntampilkan nama\n");
    let output = run(&["run", path.to_str().unwrap()]);
    let _ = fs::remove_file(&path);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "Budi");
}

#[test]
fn check_command_rejects_invalid_program() {
    let path = fixture("invalid", "tampilkan nama\n");
    let output = run(&["check", path.to_str().unwrap()]);
    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
}

#[test]
fn help_and_version_commands_are_standalone() {
    let help = run(&["help"]);
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("one new"));
    let version = run(&["version"]);
    assert!(version.status.success());
    assert!(String::from_utf8_lossy(&version.stdout).contains("0.1.0"));
}

#[test]
fn conversational_commands_are_equivalent_to_technical_aliases() {
    let dir = std::env::temp_dir().join(format!("one-alias-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("main.one");
    fs::write(&file, "tampilkan \"Halo\"\n").unwrap();
    let path = file.to_str().unwrap();
    for (technical, conversational) in [("run", "jalankan"), ("check", "periksa"), ("build", "bangun")] {
        let a = run(&[technical, path]);
        let b = run(&[conversational, path]);
        assert_eq!(a.status.code(), b.status.code());
        assert_eq!(a.stdout, b.stdout);
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn conversational_project_creation_works() {
    let root = std::env::temp_dir().join(format!("one-buat-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    let output = run(&["buat", root.to_str().unwrap()]);
    assert!(output.status.success());
    assert!(root.join("one.toml").is_file());
    assert!(root.join("src/main.one").is_file());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn test_command_runs_project_tests() {
    let root = std::env::temp_dir().join(format!("one-project-test-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("one.toml"), "nama = \"uji\"\nversi = \"0.1.0\"\n").unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"uji\"\n").unwrap();
    let output = run(&["test", root.to_str().unwrap()]);
    let _ = fs::remove_dir_all(&root);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Tests berhasil"));
}

#[test]
fn package_command_writes_manifested_artifact() {
    let root = std::env::temp_dir().join(format!("one-project-package-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("one.toml"), "nama = \"paket\"\nversi = \"0.1.0\"\n").unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap();
    let output = run(&["package", root.to_str().unwrap()]);
    let artifact = root.join("target/paket-0.1.0.onepkg");
    assert!(output.status.success());
    assert!(artifact.is_file());
    let _ = fs::remove_dir_all(&root);
}
