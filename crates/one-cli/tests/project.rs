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


#[test]
fn package_rejects_absolute_and_traversal_manifest_names() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = temp_dir();
    let escaped = root
        .parent()
        .unwrap()
        .join(format!("one-package-name-escape-{id}-0.1.0.onepkg"));
    assert!(!escaped.exists());

    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("one.toml"),
        format!("nama = \"../../one-package-name-escape-{id}\"\nversi = \"0.1.0\"\n"),
    )
    .unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["package", root.to_str().unwrap()])
        .output()
        .unwrap();
    let escaped_was_created = escaped.exists();
    let _ = fs::remove_file(&escaped);
    let _ = fs::remove_dir_all(&root);

    assert!(!output.status.success(), "unsafe package name was accepted");
    assert!(!escaped_was_created, "package escaped the project target directory");
}

#[test]
fn package_rejects_absolute_manifest_names() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = temp_dir();
    let absolute_name = root
        .parent()
        .unwrap()
        .join(format!("one-package-absolute-escape-{id}"));
    let escaped = std::path::PathBuf::from(format!("{}-0.1.0.onepkg", absolute_name.display()));
    assert!(!escaped.exists());

    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("one.toml"),
        format!("nama = \"{}\"\nversi = \"0.1.0\"\n", absolute_name.display()),
    )
    .unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["package", root.to_str().unwrap()])
        .output()
        .unwrap();
    let escaped_was_created = escaped.exists();
    let _ = fs::remove_file(&escaped);
    let _ = fs::remove_dir_all(&root);

    assert!(!output.status.success(), "absolute package name was accepted");
    assert!(!escaped_was_created, "absolute package name escaped the project");
}

#[test]
fn package_rejects_manifest_versions_with_path_separators() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = temp_dir();
    let target_component = root.join("target").join("valid-1.0.0");
    fs::create_dir_all(&target_component).unwrap();
    let escaped = root
        .parent()
        .unwrap()
        .join(format!("one-package-version-escape-{id}.onepkg"));
    assert!(!escaped.exists());

    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("one.toml"),
        format!("nama = \"valid\"\nversi = \"1.0.0/../../../one-package-version-escape-{id}\"\n"),
    )
    .unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["package", root.to_str().unwrap()])
        .output()
        .unwrap();
    let escaped_was_created = escaped.exists();
    let _ = fs::remove_file(&escaped);
    let _ = fs::remove_dir_all(&root);

    assert!(!output.status.success(), "unsafe package version was accepted");
    assert!(!escaped_was_created, "package version escaped the project target directory");
}
