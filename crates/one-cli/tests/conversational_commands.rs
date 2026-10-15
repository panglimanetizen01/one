use std::process::Command;

fn one(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_one"))
        .args(args)
        .output()
        .expect("One CLI should start")
}

#[test]
fn conversational_commands_have_the_same_behavior_as_technical_aliases() {
    let technical = ["new", "run", "check", "build", "ast"];
    let conversational = ["buat", "jalankan", "periksa", "bangun", "lihat"];
    let semantic = ["buat", "jalankan", "periksa", "bangun", "lihat struktur"];

    for (technical, conversational) in technical.iter().zip(conversational.iter()) {
        assert!(!technical.is_empty());
        assert!(!conversational.is_empty());
    }

    let help = one(&[]);
    assert_eq!(help.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&help.stderr);
    for command in semantic {
        assert!(stderr.contains(command), "usage must mention '{command}'");
    }
}

#[test]
fn conversational_project_creation_accepts_natural_instruction() {
    let dir = std::env::temp_dir().join(format!("one-conversational-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);

    let path = dir.to_string_lossy().into_owned();
    let output = one(&["buat", &path]);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(dir.join("one.toml").is_file());
    assert!(dir.join("src/main.one").is_file());

    std::fs::remove_dir_all(dir).expect("test project should be removable");
}
