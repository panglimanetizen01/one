use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_one")).args(args).output().unwrap()
}

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
    assert!(stderr.contains("capability") || stderr.contains("izin"), "unexpected stderr: {stderr}");
    let _ = fs::remove_file(&target);
    let _ = fs::remove_file(&source);
}
