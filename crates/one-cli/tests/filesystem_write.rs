use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_one")).args(args).output().unwrap()
}

#[cfg(windows)]
#[test]
fn run_command_executes_filesystem_write_from_one_source() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let target = std::env::temp_dir().join(format!("one-filesystem-write-target-{id}.txt"));
    let source = std::env::temp_dir().join(format!("one-filesystem-write-source-{id}.one"));
    let target_arg = target.to_string_lossy().replace('\\', "/");
    fs::write(&source, format!("tulis ke berkas \"{target_arg}\" dengan \"halo dari CLI One\"\n")).unwrap();

    let output = run(&[
        "run",
        source.to_str().unwrap(),
        "--izinkan",
        "filesystem.write",
        "--target",
        &target_arg,
    ]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(fs::read_to_string(&target).unwrap(), "halo dari CLI One");
    let _ = fs::remove_file(&target);
    let _ = fs::remove_file(&source);
}

#[cfg(target_os = "linux")]
#[test]
fn run_command_executes_filesystem_write_for_direct_child_on_linux() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let target = std::env::temp_dir().join(format!("one-filesystem-write-target-{id}.txt"));
    let source = std::env::temp_dir().join(format!("one-filesystem-write-source-{id}.one"));
    let target_arg = target.to_string_lossy().replace('\\', "/");
    fs::write(&source, format!("tulis ke berkas \"{target_arg}\" dengan \"halo dari CLI One\"\n")).unwrap();

    let output = run(&[
        "run",
        source.to_str().unwrap(),
        "--izinkan",
        "filesystem.write",
        "--target",
        &target_arg,
    ]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(fs::read_to_string(&target).unwrap(), "halo dari CLI One");
    let _ = fs::remove_file(&target);
    let _ = fs::remove_file(&source);
}

#[cfg(target_os = "linux")]
#[test]
fn run_command_rejects_nested_linux_filesystem_write_without_side_effects() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let nested_name = format!(".one-filesystem-write-nested-{id}");
    let nested = std::env::temp_dir().join(&nested_name);
    fs::create_dir(&nested).unwrap();
    let target = nested.join("target.txt");
    fs::write(&target, "KEEP_ORIGINAL").unwrap();
    let source = std::env::temp_dir().join(format!("one-filesystem-write-nested-source-{id}.one"));
    let target_arg = format!("{nested_name}/target.txt");
    fs::write(&source, format!("tulis ke berkas \"{target_arg}\" dengan \"ATTACK\"\n")).unwrap();

    let output = run(&[
        "run",
        source.to_str().unwrap(),
        "--izinkan",
        "filesystem.write",
        "--target",
        &target_arg,
    ]);

    assert!(!output.status.success(), "nested Linux filesystem write unexpectedly succeeded");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("UnsupportedPlatform"), "expected fail-closed diagnostic, got: {stderr}");
    assert_eq!(fs::read(&target).unwrap(), b"KEEP_ORIGINAL");
    assert_eq!(fs::read_dir(&nested).unwrap().count(), 1);
    let _ = fs::remove_dir_all(&nested);
    let _ = fs::remove_file(&source);
}

#[cfg(all(not(windows), not(target_os = "linux")))]
#[test]
fn run_command_rejects_filesystem_write_when_backend_is_unsupported() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let target = std::env::temp_dir().join(format!("one-filesystem-write-target-{id}.txt"));
    let source = std::env::temp_dir().join(format!("one-filesystem-write-source-{id}.one"));
    let target_arg = target.to_string_lossy().replace('\\', "/");
    fs::write(&source, format!("tulis ke berkas \"{target_arg}\" dengan \"halo dari CLI One\"\n")).unwrap();

    let output = run(&[
        "run",
        source.to_str().unwrap(),
        "--izinkan",
        "filesystem.write",
        "--target",
        &target_arg,
    ]);

    assert!(!output.status.success(), "unsupported filesystem writes unexpectedly succeeded");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("UnsupportedPlatform"),
        "expected fail-closed unsupported-platform diagnostic, got: {stderr}"
    );
    assert!(!target.exists(), "unsupported filesystem write created or changed a target");
    let _ = fs::remove_file(&source);
}

#[cfg(any(windows, target_os = "linux"))]
#[test]
fn run_command_uses_nearest_one_manifest_as_filesystem_root() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let project_root = std::env::temp_dir().join(format!("one-filesystem-manifest-project-{id}"));
    let source_dir = project_root.join("src");
    fs::create_dir_all(&source_dir).unwrap();
    fs::write(project_root.join("one.toml"), "nama = \"root-test\"\nversi = \"0.1.0\"\n").unwrap();
    let source = source_dir.join("main.one");
    let target = project_root.join("output.txt");
    let target_arg = target.to_string_lossy().replace('\\', "/");
    fs::write(&source, format!("tulis ke berkas \"{target_arg}\" dengan \"MANIFEST_ROOT_OK\"\n")).unwrap();

    let output = run(&[
        "run",
        source.to_str().unwrap(),
        "--izinkan",
        "filesystem.write",
        "--target",
        &target_arg,
    ]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(fs::read(&target).unwrap(), b"MANIFEST_ROOT_OK");

    let _ = fs::remove_dir_all(&project_root);
}

#[test]
fn run_command_rejects_absolute_filesystem_target_outside_source_project_root() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let project_root = std::env::temp_dir().join(format!("one-filesystem-project-{id}"));
    fs::create_dir(&project_root).unwrap();
    let source = project_root.join("main.one");
    let target = std::env::temp_dir().join(format!("one-filesystem-outside-{id}.txt"));
    let target_arg = target.to_string_lossy().replace('\\', "/");
    fs::write(&target, "KEEP_OUTSIDE_SENTINEL").unwrap();
    fs::write(&source, format!("tulis ke berkas \"{target_arg}\" dengan \"ATTACK\"\n")).unwrap();

    let output = run(&[
        "run",
        source.to_str().unwrap(),
        "--izinkan",
        "filesystem.write",
        "--target",
        &target_arg,
    ]);

    assert!(!output.status.success(), "absolute target outside the source project root unexpectedly succeeded");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ResolutionFailed"), "expected root-confinement diagnostic, got: {stderr}");
    assert_eq!(fs::read(&target).unwrap(), b"KEEP_OUTSIDE_SENTINEL");

    let _ = fs::remove_file(&target);
    let _ = fs::remove_file(&source);
    let _ = fs::remove_dir(&project_root);
}

#[test]
fn run_command_rejects_absolute_filesystem_read_outside_source_project_root() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let project_root = std::env::temp_dir().join(format!("one-filesystem-read-project-{id}"));
    fs::create_dir(&project_root).unwrap();
    let source = project_root.join("main.one");
    let target = std::env::temp_dir().join(format!("one-filesystem-read-outside-{id}.txt"));
    let target_arg = target.to_string_lossy().replace('\\', "/");
    fs::write(&target, "OUTSIDE_READ_SENTINEL").unwrap();
    fs::write(&source, format!("baca berkas \"{target_arg}\"\n")).unwrap();

    let output = run(&[
        "run",
        source.to_str().unwrap(),
        "--izinkan",
        "filesystem.read",
        "--target",
        &target_arg,
    ]);

    assert!(!output.status.success(), "absolute read target outside the source project root unexpectedly succeeded");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ResolutionFailed"), "expected root-confinement diagnostic, got: {stderr}");
    assert!(!String::from_utf8_lossy(&output.stdout).contains("OUTSIDE_READ_SENTINEL"));
    assert_eq!(fs::read(&target).unwrap(), b"OUTSIDE_READ_SENTINEL");

    let _ = fs::remove_file(&target);
    let _ = fs::remove_file(&source);
    let _ = fs::remove_dir(&project_root);
}

#[cfg(any(windows, target_os = "linux"))]
#[test]
fn run_command_resolves_relative_source_from_host_working_root() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let source_name = format!(".one-relative-source-{id}.one");
    let target_name = format!(".one-relative-target-{id}.txt");
    fs::write(&source_name, format!("tulis ke berkas \"{target_name}\" dengan \"RELATIVE_ROOT_OK\"\n")).unwrap();

    let output = run(&[
        "run",
        &source_name,
        "--izinkan",
        "filesystem.write",
        "--target",
        &target_name,
    ]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(fs::read(&target_name).unwrap(), b"RELATIVE_ROOT_OK");
    let _ = fs::remove_file(&target_name);
    let _ = fs::remove_file(&source_name);
}

#[test]
fn filesystem_write_permission_does_not_grant_filesystem_read() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let target = std::env::temp_dir().join(format!("one-filesystem-read-target-{id}.txt"));
    let source = std::env::temp_dir().join(format!("one-filesystem-read-source-{id}.one"));
    let target_arg = target.to_string_lossy().replace('\\', "/");
    fs::write(&target, "rahasia").unwrap();
    fs::write(&source, format!("baca berkas \"{target_arg}\"\n")).unwrap();

    let output = run(&[
        "run",
        source.to_str().unwrap(),
        "--izinkan",
        "filesystem.write",
        "--target",
        &target_arg,
    ]);

    assert!(!output.status.success(), "filesystem.write unexpectedly granted read: {}", String::from_utf8_lossy(&output.stdout));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("N3209"), "unexpected stderr: {stderr}");
    assert!(stderr.contains("MissingCapability"), "unexpected stderr: {stderr}");
    let _ = fs::remove_file(&target);
    let _ = fs::remove_file(&source);
}
