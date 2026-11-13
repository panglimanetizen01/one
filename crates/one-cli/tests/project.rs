use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn temp_dir() -> std::path::PathBuf {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    fs::canonicalize(std::env::temp_dir()).expect("canonicalize system temporary directory").join(format!("one-project-{id}"))
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


#[cfg(unix)]
#[test]
fn package_rejects_symlinked_target_directory_without_writing_outside_project() {
    use std::os::unix::fs::symlink;

    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = temp_dir();
    let outside = root.parent().unwrap().join(format!("one-package-outside-target-{id}"));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(&outside).unwrap();
    symlink(&outside, root.join("target")).unwrap();
    fs::write(root.join("one.toml"), "nama = \"proyek-one\"\nversi = \"0.1.0\"\n").unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["package", root.to_str().unwrap()])
        .output()
        .unwrap();
    let escaped_artifact = outside.join("proyek-one-0.1.0.onepkg");
    let escaped_was_created = escaped_artifact.exists();
    let _ = fs::remove_file(root.join("target"));
    let _ = fs::remove_dir_all(&outside);
    let _ = fs::remove_dir_all(&root);

    assert!(!output.status.success(), "package must reject a symlinked target directory");
    assert!(!escaped_was_created, "package wrote its artifact outside the project");
}

#[cfg(windows)]
#[test]
fn package_rejects_target_directory_junction_without_writing_outside_project() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = temp_dir();
    let outside = root.parent().unwrap().join(format!("one-package-outside-target-{id}"));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(&outside).unwrap();
    let status = Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(root.join("target"))
        .arg(&outside)
        .status()
        .unwrap();
    assert!(status.success(), "target junction must be created for regression test");
    fs::write(root.join("one.toml"), "nama = \"proyek-one\"\nversi = \"0.1.0\"\n").unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["package", root.to_str().unwrap()])
        .output()
        .unwrap();
    let escaped_artifact = outside.join("proyek-one-0.1.0.onepkg");
    let escaped_was_created = escaped_artifact.exists();
    let _ = fs::remove_dir(root.join("target"));
    let _ = fs::remove_dir_all(&outside);
    let _ = fs::remove_dir_all(&root);

    assert!(!output.status.success(), "package must reject a target junction");
    assert!(!escaped_was_created, "package wrote its artifact outside the project");
}

#[test]
fn package_rejects_hard_linked_artifact_without_modifying_outside_file() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = temp_dir();
    let outside = root.parent().unwrap().join(format!("one-package-outside-artifact-{id}.onepkg"));
    let artifact = root.join("target/proyek-one-0.1.0.onepkg");
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("target")).unwrap();
    fs::write(&outside, b"outside package data").unwrap();
    fs::hard_link(&outside, &artifact).unwrap();
    fs::write(root.join("one.toml"), "nama = \"proyek-one\"\nversi = \"0.1.0\"\n").unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["package", root.to_str().unwrap()])
        .output()
        .unwrap();
    let outside_contents = fs::read(&outside).unwrap();
    let _ = fs::remove_dir_all(&root);
    let _ = fs::remove_file(&outside);

    assert!(!output.status.success(), "package must reject a hard-linked artifact target");
    assert_eq!(outside_contents, b"outside package data");
}

#[cfg(unix)]
#[test]
fn build_rejects_symlinked_target_directory_without_writing_outside_project() {
    use std::os::unix::fs::symlink;

    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = temp_dir();
    let outside = root.parent().unwrap().join(format!("one-build-outside-target-{id}"));
    fs::create_dir_all(&root).unwrap();
    fs::create_dir_all(&outside).unwrap();
    fs::write(root.join("main.one"), "tampilkan \"build\"\n").unwrap();
    symlink(&outside, root.join("target")).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["build", root.join("main.one").to_str().unwrap()])
        .output()
        .unwrap();
    let escaped_artifact = outside.join("main.oneir");
    let escaped_was_created = escaped_artifact.exists();
    let _ = fs::remove_file(root.join("target"));
    let _ = fs::remove_dir_all(&outside);
    let _ = fs::remove_dir_all(&root);

    assert!(!output.status.success(), "build must reject a symlinked target directory");
    assert!(!escaped_was_created, "build wrote its artifact outside the source project");
}


#[cfg(unix)]
#[test]
fn package_rejects_symlinked_main_source_without_packaging_external_content() {
    use std::os::unix::fs::symlink;

    let root = temp_dir();
    let outside = root.parent().unwrap().join(format!(
        "one-package-outside-main-{}.one",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(&outside, "tampilkan \"outside source\"\n").unwrap();
    symlink(&outside, root.join("src/main.one")).unwrap();
    fs::write(root.join("one.toml"), "nama = \"proyek-one\"\nversi = \"0.1.0\"\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["package", root.to_str().unwrap()])
        .output()
        .unwrap();
    let artifact_exists = root.join("target/proyek-one-0.1.0.onepkg").exists();
    let _ = fs::remove_file(root.join("src/main.one"));
    let _ = fs::remove_file(&outside);
    let _ = fs::remove_dir_all(&root);

    assert!(!output.status.success(), "package must reject a symlinked main source");
    assert!(!artifact_exists, "package must not produce an artifact from external source content");
}

#[test]
fn package_rejects_hard_linked_manifest() {
    let root = temp_dir();
    let outside = root.parent().unwrap().join(format!(
        "one-package-outside-manifest-{}.toml",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(&outside, "nama = \"proyek-one\"\nversi = \"0.1.0\"\n").unwrap();
    fs::hard_link(&outside, root.join("one.toml")).unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["package", root.to_str().unwrap()])
        .output()
        .unwrap();
    let artifact_exists = root.join("target/proyek-one-0.1.0.onepkg").exists();
    let outside_contents = fs::read(&outside).unwrap();
    let _ = fs::remove_dir_all(&root);
    let _ = fs::remove_file(&outside);

    assert!(!output.status.success(), "package must reject a hard-linked manifest");
    assert!(!artifact_exists, "package must not proceed after an unconfined manifest read");
    assert_eq!(outside_contents, b"nama = \"proyek-one\"\nversi = \"0.1.0\"\n");
}

#[cfg(unix)]
#[test]
fn build_rejects_symlinked_source_file() {
    use std::os::unix::fs::symlink;

    let root = temp_dir();
    fs::create_dir_all(&root).unwrap();
    let outside = root.parent().unwrap().join(format!(
        "one-build-outside-source-{}.one",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::write(&outside, "tampilkan \"outside source\"\n").unwrap();
    symlink(&outside, root.join("main.one")).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["build", root.join("main.one").to_str().unwrap()])
        .output()
        .unwrap();
    let artifact_exists = root.join("target/main.oneir").exists();
    let _ = fs::remove_file(root.join("main.one"));
    let _ = fs::remove_file(&outside);
    let _ = fs::remove_dir_all(&root);

    assert!(!output.status.success(), "build must reject a symlinked source file");
    assert!(!artifact_exists, "build must not create an artifact from external source content");
}

#[cfg(unix)]
#[test]
fn project_test_rejects_symlinked_conformance_fixture() {
    use std::os::unix::fs::symlink;

    let root = temp_dir();
    let outside = root.parent().unwrap().join(format!(
        "one-test-outside-fixture-{}.one",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("tests/conformance/valid")).unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"main\"\n").unwrap();
    fs::write(&outside, "tampilkan \"outside fixture\"\n").unwrap();
    symlink(&outside, root.join("tests/conformance/valid/external.one")).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["test", root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = fs::remove_file(root.join("tests/conformance/valid/external.one"));
    let _ = fs::remove_file(&outside);
    let _ = fs::remove_dir_all(&root);

    assert!(!output.status.success(), "project test must reject a symlinked conformance fixture");
}


#[test]
fn build_supports_spaces_in_source_file_names() {
    let root = temp_dir();
    fs::create_dir_all(&root).unwrap();
    let source = root.join("program saya.one");
    fs::write(&source, "tampilkan \"aman\"\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["build", source.to_str().unwrap()])
        .output()
        .unwrap();
    let artifact_exists = root.join("target/program saya.oneir").is_file();
    let _ = fs::remove_dir_all(&root);

    assert!(output.status.success(), "safe filenames with spaces must remain supported: {}", String::from_utf8_lossy(&output.stderr));
    assert!(artifact_exists, "build artifact should preserve the safe source filename");
}
