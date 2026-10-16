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
