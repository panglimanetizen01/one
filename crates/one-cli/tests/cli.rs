use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};
fn fixture(name: &str, source: &str) -> std::path::PathBuf { let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let path = std::env::temp_dir().join(format!("one-{name}-{id}.one")); fs::write(&path, source).unwrap(); path }
fn run(args: &[&str]) -> std::process::Output { Command::new(env!("CARGO_BIN_EXE_one")).args(args).output().unwrap() }
fn portable_path(path: &std::path::Path) -> String { path.to_string_lossy().replace('\\', "/") }
#[test] fn run_command_executes_program() { let path = fixture("run", "nama adalah \"Budi\"\ntampilkan nama\n"); let output = run(&["run", path.to_str().unwrap()]); let _ = fs::remove_file(&path); assert!(output.status.success()); assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "Budi"); }
#[test] fn check_command_rejects_invalid_program() { let path = fixture("invalid", "tampilkan nama\n"); let output = run(&["check", path.to_str().unwrap()]); let _ = fs::remove_file(&path); assert!(!output.status.success()); }
#[test] fn help_and_version_commands_are_standalone() { let help = run(&["help"]); assert!(help.status.success()); assert!(String::from_utf8_lossy(&help.stdout).contains("one new")); let version = run(&["version"]); assert!(version.status.success()); assert!(String::from_utf8_lossy(&version.stdout).contains("0.1.0")); }
#[test] fn conversational_commands_are_equivalent_to_technical_aliases() { let dir = std::env::temp_dir().join(format!("one-alias-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(&dir).unwrap(); let file = dir.join("main.one"); fs::write(&file, "tampilkan \"Halo\"\n").unwrap(); let path = file.to_str().unwrap(); for (technical, conversational) in [("run", "jalankan"), ("check", "periksa"), ("build", "bangun")] { let a = run(&[technical, path]); let b = run(&[conversational, path]); assert_eq!(a.status.code(), b.status.code()); assert_eq!(a.stdout, b.stdout); } let _ = fs::remove_dir_all(&dir); }
#[test] fn conversational_project_creation_works() { let root = std::env::temp_dir().join(format!("one-buat-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); let output = run(&["buat", root.to_str().unwrap()]); assert!(output.status.success()); assert!(root.join("one.toml").is_file()); assert!(root.join("src/main.one").is_file()); let _ = fs::remove_dir_all(&root); }
#[test] fn test_command_runs_project_tests() { let root = std::env::temp_dir().join(format!("one-project-test-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::write(root.join("one.toml"), "nama = \"uji\"\nversi = \"0.1.0\"\n").unwrap(); fs::write(root.join("src/main.one"), "tampilkan \"uji\"\n").unwrap(); let output = run(&["test", root.to_str().unwrap()]); let _ = fs::remove_dir_all(&root); assert!(output.status.success()); assert!(String::from_utf8_lossy(&output.stdout).contains("Tests berhasil")); }
#[test] fn package_command_writes_manifested_artifact() { let root = std::env::temp_dir().join(format!("one-project-package-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::write(root.join("one.toml"), "nama = \"paket\"\nversi = \"0.1.0\"\n").unwrap(); fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap(); let output = run(&["package", root.to_str().unwrap()]); let artifact = root.join("target/paket-0.1.0.onepkg"); assert!(output.status.success()); assert!(artifact.is_file()); let _ = fs::remove_dir_all(&root); }
#[test] fn build_and_package_artifacts_are_byte_deterministic() { let root = std::env::temp_dir().join(format!("one-project-deterministic-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::write(root.join("one.toml"), "nama = \"deterministik\"\nversi = \"0.1.0\"\n").unwrap(); fs::write(root.join("src/main.one"), "nama adalah \"Budi\"\ntampilkan nama\n").unwrap(); let source = root.join("src/main.one"); let source_arg = source.to_str().unwrap(); assert!(run(&["bangun", source_arg]).status.success()); let build_artifact = fs::read(root.join("src/target/main.oneir")).unwrap(); assert!(run(&["bangun", source_arg]).status.success()); assert_eq!(build_artifact, fs::read(root.join("src/target/main.oneir")).unwrap()); assert!(run(&["package", root.to_str().unwrap()]).status.success()); let package_artifact = fs::read(root.join("target/deterministik-0.1.0.onepkg")).unwrap(); assert!(run(&["package", root.to_str().unwrap()]).status.success()); assert_eq!(package_artifact, fs::read(root.join("target/deterministik-0.1.0.onepkg")).unwrap()); let _ = fs::remove_dir_all(&root); }
#[test] fn package_rejects_missing_required_manifest_fields() { let root = std::env::temp_dir().join(format!("one-project-bad-manifest-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::write(root.join("one.toml"), "nama = \"paket\"\n").unwrap(); fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap(); let output = run(&["package", root.to_str().unwrap()]); let _ = fs::remove_dir_all(&root); assert_eq!(output.status.code(), Some(2)); assert!(String::from_utf8_lossy(&output.stderr).contains("ETOOL002")); }
#[test] fn package_rejects_duplicate_manifest_fields() { let root = std::env::temp_dir().join(format!("one-project-duplicate-manifest-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::write(root.join("one.toml"), "nama = \"a\"\nnama = \"b\"\nversi = \"0.1.0\"\n").unwrap(); fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap(); let output = run(&["package", root.to_str().unwrap()]); let _ = fs::remove_dir_all(&root); assert_eq!(output.status.code(), Some(2)); assert!(String::from_utf8_lossy(&output.stderr).contains("ETOOL002")); }
#[test] fn test_command_executes_sorted_conformance_fixtures_when_present() { let root = std::env::temp_dir().join(format!("one-project-conformance-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::create_dir_all(root.join("tests/conformance/valid")).unwrap(); fs::create_dir_all(root.join("tests/conformance/invalid")).unwrap(); fs::write(root.join("one.toml"), "nama = \"uji\"\nversi = \"0.1.0\"\n").unwrap(); fs::write(root.join("src/main.one"), "tampilkan \"main\"\n").unwrap(); fs::write(root.join("tests/conformance/valid/b.one"), "tampilkan \"b\"\n").unwrap(); fs::write(root.join("tests/conformance/valid/a.one"), "tampilkan \"a\"\n").unwrap(); fs::write(root.join("tests/conformance/invalid/b.one"), "tampilkan tidak_ada\n").unwrap(); let output = run(&["test", root.to_str().unwrap()]); let _ = fs::remove_dir_all(&root); assert!(output.status.success()); assert!(String::from_utf8_lossy(&output.stdout).contains("4 fixture")); }
#[test] fn run_command_accepts_explicit_dns_capability_and_scope() { let path = fixture("orchestration", "cari alamat \"127.0.0.1\"\n"); let output = run(&["run", path.to_str().unwrap(), "--izinkan", "network.discover", "--target", "127.0.0.1"]); let _ = fs::remove_file(&path); assert!(output.status.success()); assert!(String::from_utf8_lossy(&output.stdout).contains("127.0.0.1")); }
#[test] fn run_command_executes_filesystem_read_from_one_source() { let source_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let data = std::env::temp_dir().join(format!("one-filesystem-data-{source_id}.txt")); let source = std::env::temp_dir().join(format!("one-filesystem-source-{source_id}.one")); let data_arg = portable_path(&data); fs::write(&data, "isi One\nbaris kedua\n").unwrap(); fs::write(&source, format!("baca berkas \"{}\"\n", data_arg)).unwrap(); let output = run(&["run", source.to_str().unwrap(), "--izinkan", "filesystem.read", "--target", &data_arg]); let _ = fs::remove_file(&data); let _ = fs::remove_file(&source); assert!(output.status.success()); assert_eq!(String::from_utf8_lossy(&output.stdout), "isi One\nbaris kedua\n"); }
#[test] fn filesystem_read_is_denied_without_explicit_capability() { let source_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let data = std::env::temp_dir().join(format!("one-filesystem-denied-data-{source_id}.txt")); let source = std::env::temp_dir().join(format!("one-filesystem-denied-source-{source_id}.one")); let data_arg = portable_path(&data); fs::write(&data, "rahasia\n").unwrap(); fs::write(&source, format!("baca berkas \"{}\"\n", data_arg)).unwrap(); let output = run(&["run", source.to_str().unwrap()]); let _ = fs::remove_file(&data); let _ = fs::remove_file(&source); assert!(!output.status.success()); assert!(String::from_utf8_lossy(&output.stderr).contains("N3205")); }
#[test] fn filesystem_read_rejects_out_of_scope_target() { let source_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let data = std::env::temp_dir().join(format!("one-filesystem-scope-data-{source_id}.txt")); let source = std::env::temp_dir().join(format!("one-filesystem-scope-source-{source_id}.one")); let data_arg = portable_path(&data); let scope_arg = portable_path(&std::env::temp_dir()); fs::write(&data, "di luar\n").unwrap(); fs::write(&source, format!("baca berkas \"{}\"\n", data_arg)).unwrap(); let output = run(&["run", source.to_str().unwrap(), "--izinkan", "filesystem.read", "--target", &scope_arg]); let _ = fs::remove_file(&data); let _ = fs::remove_file(&source); assert!(!output.status.success()); assert!(String::from_utf8_lossy(&output.stderr).contains("N3209")); }


#[test]
fn human_vnext_cli_runs_task_app_and_reopens_persistent_data() {
    use std::io::Write;
    use std::process::Stdio;
    let root = std::env::temp_dir().join(format!(
        "one-human-vnext-cli-{}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&root).unwrap();
    let source = root.join("daftar-tugas.one");
    let data = root.join("tugas.db");
    fs::write(&source, concat!(
        "Buat aplikasi daftar tugas\n",
        "Pengguna bisa menambahkan tugas\n",
        "    Terima judul tugas\n",
        "    Simpan tugas baru\n",
        "Pengguna bisa melihat semua tugas\n",
        "    Tampilkan semua tugas\n",
        "Pengguna bisa menandai tugas sebagai selesai\n",
        "    Terima tugas yang dipilih\n",
        "    Tandai tugas sebagai selesai\n",
        "Simpan semua tugas agar tidak hilang saat aplikasi ditutup\n",
    )).unwrap();

    let mut first = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["jalankan-human-vnext", source.to_str().unwrap(), "--data", data.to_str().unwrap()])
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    first.stdin.take().unwrap().write_all(b"tambah\nBelanja\nkeluar\n").unwrap();
    let first_output = first.wait_with_output().unwrap();
    assert!(first_output.status.success(), "{}", String::from_utf8_lossy(&first_output.stderr));
    assert!(data.is_file());

    let mut second = Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["jalankan-human-vnext", source.to_str().unwrap(), "--data", data.to_str().unwrap()])
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    second.stdin.take().unwrap().write_all(b"lihat\nselesai\n1\nkeluar\n").unwrap();
    let second_output = second.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&second_output.stdout);
    assert!(second_output.status.success(), "{}", String::from_utf8_lossy(&second_output.stderr));
    assert!(stdout.contains("Belanja"), "task must survive a new CLI process");
    assert!(stdout.contains("Tugas 1 ditandai selesai"), "completion must execute through the CLI");
    let _ = fs::remove_dir_all(&root);
}


#[test]
fn default_v01_cli_does_not_auto_detect_human_vnext_source() {
    let source = fixture("human-vnext-no-autodetect", concat!(
        "Buat aplikasi daftar tugas\n",
        "Pengguna bisa menambahkan tugas\n",
        "    Terima judul tugas\n",
        "    Simpan tugas baru\n",
        "Pengguna bisa melihat semua tugas\n",
        "    Tampilkan semua tugas\n",
        "Pengguna bisa menandai tugas sebagai selesai\n",
        "    Terima tugas yang dipilih\n",
        "    Tandai tugas sebagai selesai\n",
        "Simpan semua tugas agar tidak hilang saat aplikasi ditutup\n",
    ));
    let output = run(&["periksa", source.to_str().unwrap()]);
    let _ = fs::remove_file(&source);
    assert!(!output.status.success(), "v0.1 must not silently select the human-vNext parser");
    assert!(!String::from_utf8_lossy(&output.stderr).contains("EHV002"));
}

#[test]
fn human_vnext_cli_rejects_missing_source_argument() {
    let output = run(&["jalankan-human-vnext"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Penggunaan: one jalankan-human-vnext"));
}

#[test]
fn human_vnext_cli_rejects_unknown_option_without_running_program() {
    let source = fixture("human-vnext-option", "Buat aplikasi catatan\n");
    let output = run(&["jalankan-human-vnext", source.to_str().unwrap(), "--unknown"]);
    let _ = fs::remove_file(&source);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("EHV001"));
}

#[test]
fn human_vnext_cli_reports_missing_source_file() {
    let path = std::env::temp_dir().join(format!(
        "one-human-vnext-missing-{}.one",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    let output = run(&["jalankan-human-vnext", path.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("EHV001"));
}

#[test]
fn human_vnext_cli_rejects_invalid_source_with_parser_diagnostic() {
    let source = fixture("human-vnext-invalid", "Buat aplikasi catatan\nInstruksi tidak dikenal\n");
    let output = run(&["jalankan-human-vnext", source.to_str().unwrap()]);
    let _ = fs::remove_file(&source);
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("EHV002"));
    assert!(stderr.contains("H0016"));
}

#[test]
fn human_vnext_cli_rejects_task_app_without_persistence_contract() {
    let source = fixture("human-vnext-no-persist", concat!(
        "Buat aplikasi daftar tugas\n",
        "Pengguna bisa menambahkan tugas\n",
        "    Terima judul tugas\n",
        "    Simpan tugas baru\n",
        "Pengguna bisa melihat semua tugas\n",
        "    Tampilkan semua tugas\n",
        "Pengguna bisa menandai tugas sebagai selesai\n",
        "    Terima tugas yang dipilih\n",
        "    Tandai tugas sebagai selesai\n",
    ));
    let output = run(&["jalankan-human-vnext", source.to_str().unwrap()]);
    let _ = fs::remove_file(&source);
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("EHV003"));
    assert!(String::from_utf8_lossy(&output.stderr).contains("wajib mendeklarasikan"));
}

#[test]
fn stdlib_example_runs_through_the_official_cli() {
    let path = fixture("stdlib", include_str!("../../../examples/stdlib.one"));
    let output = run(&["run", path.to_str().unwrap()]);
    let _ = fs::remove_file(&path);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).lines().collect::<Vec<_>>(),
        vec!["2", "benar", "pir,jeruk", "42", "7", "3"]
    );
}

#[test]
fn package_command_supports_modular_human_core_projects_deterministically() {
    let root = std::env::temp_dir().join(format!("one-human-package-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("one.toml"), "nama = \"humanapp\"\\nversi = \"1.0.0\"\\n").unwrap();
    fs::write(root.join("src/main.one"), r#"Buat program paket modular
Gunakan modul "math.one"
hasil adalah tambah dengan 2 dan 3
tampilkan hasil
"#).unwrap();
    fs::write(root.join("src/math.one"), r#"Buat modul math
buat pekerjaan tambah
    terima kiri
    terima kanan
    hasilnya adalah kiri ditambah kanan
"#).unwrap();

    let output = run(&["package", root.to_str().unwrap()]);
    assert!(output.status.success(), "stdout: {}\\nstderr: {}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    let artifact = root.join("target/humanapp-1.0.0.onepkg");
    let first = fs::read(&artifact).expect("package artifact should exist");
    let text = String::from_utf8_lossy(&first);
    assert!(text.starts_with("One human-core package v1"));
    assert!(text.contains("[frontend]\\nhuman-core"));
    assert!(text.contains("tambah"));
    assert!(run(&["package", root.to_str().unwrap()]).status.success());
    assert_eq!(first, fs::read(&artifact).unwrap(), "human-core package must be deterministic");
    let _ = fs::remove_dir_all(&root);
}
