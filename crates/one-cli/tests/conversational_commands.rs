use std::process::Command;

fn one(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_one"))
        .args(args)
        .output()
        .expect("One CLI should start")
}

#[test]
fn usage_teaches_conversational_commands() {
    let output = one(&[]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    for command in ["one buat", "one jalankan", "one periksa", "one bangun", "one lihat struktur"] {
        assert!(stderr.contains(command), "usage must mention '{command}'");
    }
}

#[test]
fn conversational_project_creation_works() {
    let dir = std::env::temp_dir().join(format!(
        "one-conversational-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock should be after epoch")
            .as_nanos()
    ));
    let path = dir.to_string_lossy().into_owned();

    let output = one(&["buat", &path]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(dir.join("one.toml").is_file());
    assert!(dir.join("src/main.one").is_file());

    std::fs::remove_dir_all(dir).expect("test project should be removable");
}

#[test]
fn conversational_file_commands_match_technical_aliases() {
    let dir = std::env::temp_dir().join(format!(
        "one-conversational-file-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock should be after epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("test directory should be created");
    let file = dir.join("main.one");
    std::fs::write(&file, "tampilkan \"Halo\"\n").expect("source should be written");
    let path = file.to_string_lossy().into_owned();

    for (technical, conversational) in [
        ("run", "jalankan"),
        ("check", "periksa"),
        ("build", "bangun"),
        ("ast", "lihat struktur"),
    ] {
        let technical_output = one(&[technical, &path]);
        let conversational_output = if conversational == "lihat struktur" {
            one(&["lihat", "struktur", &path])
        } else {
            one(&[conversational, &path])
        };

        assert_eq!(
            conversational_output.status.code(),
            technical_output.status.code(),
            "status differs for {technical} / {conversational}"
        );
        assert_eq!(
            conversational_output.stdout,
            technical_output.stdout,
            "stdout differs for {technical} / {conversational}"
        );
    }

    std::fs::remove_dir_all(dir).expect("test directory should be removable");
}
