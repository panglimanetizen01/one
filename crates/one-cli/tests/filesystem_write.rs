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
