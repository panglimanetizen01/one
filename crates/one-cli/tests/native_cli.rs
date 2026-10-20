use std::process::Command;

fn native_cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_one-native"))
}

#[test]
fn android_target_explicit_mempertahankan_build_native() {
    let dir = std::env::temp_dir();
    let source = dir.join(format!(
        "one-native-cli-android-{}.one",
        std::process::id()
    ));
    let artifact = dir.join(format!(
        "one-native-cli-android-{}",
        std::process::id()
    ));

    std::fs::write(&source, "tampilkan \"ONE_NATIVE_CLI_ANDROID\"\n")
        .expect("source fixture harus dapat ditulis");

    let result = native_cli()
        .args([
            "build",
            "--target",
            "android-aarch64",
            source.to_str().unwrap(),
            artifact.to_str().unwrap(),
        ])
        .output()
        .expect("one-native harus dapat dijalankan");

    assert!(
        result.status.success(),
        "explicit android target gagal: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(artifact.is_file(), "artifact Android tidak dibuat");

    let _ = std::fs::remove_file(&source);
    let _ = std::fs::remove_file(&artifact);
}

#[test]
fn target_tidak_dikenal_ditolak_dengan_usage() {
    let result = native_cli()
        .args(["build", "--target", "target-tidak-ada"])
        .output()
        .expect("one-native harus dapat dijalankan");

    assert_eq!(result.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("android-aarch64"));
    assert!(stderr.contains("linux-aarch64"));
    assert!(stderr.contains("linux-x86_64"));
    assert!(stderr.contains("windows-x86_64"));
}

#[test]
fn windows_x86_64_target_mempertahankan_build_native() {
    let dir = std::env::temp_dir();
    let source = dir.join(format!(
        "one-native-cli-windows-{}.one",
        std::process::id()
    ));
    let artifact = dir.join(format!(
        "one-native-cli-windows-{}.exe",
        std::process::id()
    ));

    std::fs::write(&source, "tampilkan \"ONE_NATIVE_CLI_WINDOWS\"\n")
        .expect("source fixture harus dapat ditulis");

    let result = native_cli()
        .args([
            "build",
            "--target",
            "windows-x86_64",
            source.to_str().unwrap(),
            artifact.to_str().unwrap(),
        ])
        .output()
        .expect("one-native harus dapat dijalankan");

    assert!(
        result.status.success(),
        "explicit windows target gagal: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(artifact.is_file(), "artifact Windows tidak dibuat");

    let _ = std::fs::remove_file(&source);
    let _ = std::fs::remove_file(&artifact);
}
