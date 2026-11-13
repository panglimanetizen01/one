use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};
use ed25519_dalek::SigningKey;

// Windows runners may expose TEMP through an 8.3 alias (for example RUNNER~1).
// The filesystem adapter intentionally rejects non-canonical paths; fixtures
// must use the canonical temp root rather than weakening that security check.
fn canonical_temp_dir() -> std::path::PathBuf {
    fs::canonicalize(std::env::temp_dir()).expect("canonicalize system temporary directory")
}
fn fixture(name: &str, source: &str) -> std::path::PathBuf { let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let path = canonical_temp_dir().join(format!("one-{name}-{id}.one")); fs::write(&path, source).unwrap(); path }
fn run(args: &[&str]) -> std::process::Output { Command::new(env!("CARGO_BIN_EXE_one")).args(args).output().unwrap() }
fn portable_path(path: &std::path::Path) -> String { path.to_string_lossy().replace('\\', "/") }
#[test] fn run_command_executes_program() { let path = fixture("run", "nama adalah \"Budi\"\ntampilkan nama\n"); let output = run(&["run", path.to_str().unwrap()]); let _ = fs::remove_file(&path); assert!(output.status.success()); assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "Budi"); }
#[test] fn check_command_rejects_invalid_program() { let path = fixture("invalid", "tampilkan nama\n"); let output = run(&["check", path.to_str().unwrap()]); let _ = fs::remove_file(&path); assert!(!output.status.success()); }
#[test] fn help_and_version_commands_are_standalone() { let help = run(&["help"]); assert!(help.status.success()); assert!(String::from_utf8_lossy(&help.stdout).contains("one new")); assert!(String::from_utf8_lossy(&help.stdout).contains("sign-package")); assert!(String::from_utf8_lossy(&help.stdout).contains("verify-package")); let version = run(&["version"]); assert!(version.status.success()); assert!(String::from_utf8_lossy(&version.stdout).contains("0.1.0")); }
#[test] fn conversational_commands_are_equivalent_to_technical_aliases() { let dir = canonical_temp_dir().join(format!("one-alias-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(&dir).unwrap(); let file = dir.join("main.one"); fs::write(&file, "tampilkan \"Halo\"\n").unwrap(); let path = file.to_str().unwrap(); for (technical, conversational) in [("run", "jalankan"), ("check", "periksa"), ("build", "bangun")] { let a = run(&[technical, path]); let b = run(&[conversational, path]); assert_eq!(a.status.code(), b.status.code()); assert_eq!(a.stdout, b.stdout); } let _ = fs::remove_dir_all(&dir); }
#[test] fn conversational_project_creation_works() { let root = canonical_temp_dir().join(format!("one-buat-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); let output = run(&["buat", root.to_str().unwrap()]); assert!(output.status.success()); assert!(root.join("one.toml").is_file()); assert!(root.join("src/main.one").is_file()); let _ = fs::remove_dir_all(&root); }
#[test] fn test_command_runs_project_tests() { let root = canonical_temp_dir().join(format!("one-project-test-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::write(root.join("one.toml"), "nama = \"uji\"\nversi = \"0.1.0\"\n").unwrap(); fs::write(root.join("src/main.one"), "tampilkan \"uji\"\n").unwrap(); let output = run(&["test", root.to_str().unwrap()]); let _ = fs::remove_dir_all(&root); assert!(output.status.success()); assert!(String::from_utf8_lossy(&output.stdout).contains("Tests berhasil")); }
#[test] fn package_command_writes_manifested_artifact() { let root = canonical_temp_dir().join(format!("one-project-package-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::write(root.join("one.toml"), "nama = \"paket\"\nversi = \"0.1.0\"\n").unwrap(); fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap(); let output = run(&["package", root.to_str().unwrap()]); let artifact = root.join("target/paket-0.1.0.onepkg"); assert!(output.status.success()); assert!(artifact.is_file()); let _ = fs::remove_dir_all(&root); }
#[test] fn build_and_package_artifacts_are_byte_deterministic() { let root = canonical_temp_dir().join(format!("one-project-deterministic-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::write(root.join("one.toml"), "nama = \"deterministik\"\nversi = \"0.1.0\"\n").unwrap(); fs::write(root.join("src/main.one"), "nama adalah \"Budi\"\ntampilkan nama\n").unwrap(); let source = root.join("src/main.one"); let source_arg = source.to_str().unwrap(); assert!(run(&["bangun", source_arg]).status.success()); let build_artifact = fs::read(root.join("src/target/main.oneir")).unwrap(); assert!(run(&["bangun", source_arg]).status.success()); assert_eq!(build_artifact, fs::read(root.join("src/target/main.oneir")).unwrap()); assert!(run(&["package", root.to_str().unwrap()]).status.success()); let package_artifact = fs::read(root.join("target/deterministik-0.1.0.onepkg")).unwrap(); assert!(run(&["package", root.to_str().unwrap()]).status.success()); assert_eq!(package_artifact, fs::read(root.join("target/deterministik-0.1.0.onepkg")).unwrap()); let _ = fs::remove_dir_all(&root); }
#[test] fn package_rejects_missing_required_manifest_fields() { let root = canonical_temp_dir().join(format!("one-project-bad-manifest-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::write(root.join("one.toml"), "nama = \"paket\"\n").unwrap(); fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap(); let output = run(&["package", root.to_str().unwrap()]); let _ = fs::remove_dir_all(&root); assert_eq!(output.status.code(), Some(2)); assert!(String::from_utf8_lossy(&output.stderr).contains("ETOOL002")); }
#[test] fn package_rejects_duplicate_manifest_fields() { let root = canonical_temp_dir().join(format!("one-project-duplicate-manifest-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::write(root.join("one.toml"), "nama = \"a\"\nnama = \"b\"\nversi = \"0.1.0\"\n").unwrap(); fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap(); let output = run(&["package", root.to_str().unwrap()]); let _ = fs::remove_dir_all(&root); assert_eq!(output.status.code(), Some(2)); assert!(String::from_utf8_lossy(&output.stderr).contains("ETOOL002")); }
#[test] fn test_command_executes_sorted_conformance_fixtures_when_present() { let root = canonical_temp_dir().join(format!("one-project-conformance-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())); fs::create_dir_all(root.join("src")).unwrap(); fs::create_dir_all(root.join("tests/conformance/valid")).unwrap(); fs::create_dir_all(root.join("tests/conformance/invalid")).unwrap(); fs::write(root.join("one.toml"), "nama = \"uji\"\nversi = \"0.1.0\"\n").unwrap(); fs::write(root.join("src/main.one"), "tampilkan \"main\"\n").unwrap(); fs::write(root.join("tests/conformance/valid/b.one"), "tampilkan \"b\"\n").unwrap(); fs::write(root.join("tests/conformance/valid/a.one"), "tampilkan \"a\"\n").unwrap(); fs::write(root.join("tests/conformance/invalid/b.one"), "tampilkan tidak_ada\n").unwrap(); let output = run(&["test", root.to_str().unwrap()]); let _ = fs::remove_dir_all(&root); assert!(output.status.success()); assert!(String::from_utf8_lossy(&output.stdout).contains("4 fixture")); }
#[test] fn run_command_accepts_explicit_dns_capability_and_scope() { let path = fixture("orchestration", "cari alamat \"127.0.0.1\"\n"); let output = run(&["run", path.to_str().unwrap(), "--izinkan", "network.discover", "--target", "127.0.0.1"]); let _ = fs::remove_file(&path); assert!(output.status.success()); assert!(String::from_utf8_lossy(&output.stdout).contains("127.0.0.1")); }
#[test] fn run_command_executes_filesystem_read_from_one_source() { let source_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let root = std::env::current_dir().unwrap(); let data = root.join(format!("one-filesystem-data-{source_id}.txt")); let source = root.join(format!("one-filesystem-source-{source_id}.one")); let data_arg = portable_path(&data); fs::write(&data, "isi One\nbaris kedua\n").unwrap(); fs::write(&source, format!("baca berkas \"{}\"\n", data_arg)).unwrap(); let output = run(&["run", source.to_str().unwrap(), "--izinkan", "filesystem.read", "--target", &data_arg]); let _ = fs::remove_file(&data); let _ = fs::remove_file(&source); assert!(output.status.success(), "filesystem read failed: stdout={} stderr={}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)); assert_eq!(String::from_utf8_lossy(&output.stdout), "isi One\nbaris kedua\n"); }
#[test] fn filesystem_read_is_denied_without_explicit_capability() { let source_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let data = canonical_temp_dir().join(format!("one-filesystem-denied-data-{source_id}.txt")); let source = canonical_temp_dir().join(format!("one-filesystem-denied-source-{source_id}.one")); let data_arg = portable_path(&data); fs::write(&data, "rahasia\n").unwrap(); fs::write(&source, format!("baca berkas \"{}\"\n", data_arg)).unwrap(); let output = run(&["run", source.to_str().unwrap()]); let _ = fs::remove_file(&data); let _ = fs::remove_file(&source); assert!(!output.status.success()); assert!(String::from_utf8_lossy(&output.stderr).contains("N3205")); }
#[test] fn filesystem_read_rejects_out_of_scope_target() { let source_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let data = canonical_temp_dir().join(format!("one-filesystem-scope-data-{source_id}.txt")); let source = canonical_temp_dir().join(format!("one-filesystem-scope-source-{source_id}.one")); let data_arg = portable_path(&data); let scope_arg = portable_path(&canonical_temp_dir()); fs::write(&data, "di luar\n").unwrap(); fs::write(&source, format!("baca berkas \"{}\"\n", data_arg)).unwrap(); let output = run(&["run", source.to_str().unwrap(), "--izinkan", "filesystem.read", "--target", &scope_arg]); let _ = fs::remove_file(&data); let _ = fs::remove_file(&source); assert!(!output.status.success()); assert!(String::from_utf8_lossy(&output.stderr).contains("N3209")); }
#[test]
fn filesystem_root_scope_allows_inside_and_denies_outside_targets() {
    let source_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    // Use the checked-out workspace rather than the Windows temp directory,
    // which may itself resolve through a reparse point rejected by the adapter.
    let workspace = std::env::current_dir().unwrap();
    let root = workspace.join(format!("one-filesystem-root-{source_id}"));
    let inside = root.join("inside.txt");
    let outside = workspace.join(format!("one-filesystem-outside-{source_id}.txt"));
    let source = workspace.join(format!("one-filesystem-root-source-{source_id}.one"));
    fs::create_dir_all(&root).unwrap();
    fs::write(&inside, "inside root\n").unwrap();
    fs::write(&outside, "outside root\n").unwrap();

    let root_arg = portable_path(&root);
    let inside_arg = portable_path(&inside);
    let outside_arg = portable_path(&outside);
    fs::write(&source, format!("baca berkas \"{}\"\n", inside_arg)).unwrap();
    let allowed = run(&[
        "run", source.to_str().unwrap(), "--izinkan", "filesystem.read",
        "--root", &root_arg, "--target", &inside_arg,
    ]);
    assert!(
        allowed.status.success(),
        "root-contained read failed: stdout={} stderr={}",
        String::from_utf8_lossy(&allowed.stdout),
        String::from_utf8_lossy(&allowed.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&allowed.stdout), "inside root\n");

    fs::write(&source, format!("baca berkas \"{}\"\n", outside_arg)).unwrap();
    let denied = run(&[
        "run", source.to_str().unwrap(), "--izinkan", "filesystem.read",
        "--root", &root_arg, "--target", &outside_arg,
    ]);
    assert!(!denied.status.success());
    assert!(String::from_utf8_lossy(&denied.stderr).contains("N3209"));

    let _ = fs::remove_dir_all(&root);
    let _ = fs::remove_file(&outside);
    let _ = fs::remove_file(&source);
}



#[test]
fn human_vnext_cli_runs_task_app_and_reopens_persistent_data() {
    use std::io::Write;
    use std::process::Stdio;
    let root = canonical_temp_dir().join(format!(
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
    let path = canonical_temp_dir().join(format!(
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
        vec!["2", r#"["apel","jeruk"]"#, "benar", "pir,jeruk", "42", "7", "3"]
    );
}

#[test]
fn package_command_supports_modular_human_core_projects_deterministically() {
    let root = canonical_temp_dir().join(format!("one-human-package-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("one.toml"), "nama = \"humanapp\"\nversi = \"1.0.0\"\n").unwrap();
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
    fs::create_dir_all(root.join("tests/conformance/valid")).unwrap();
    fs::write(root.join("tests/conformance/valid/valid.one"), r#"Buat program fixture valid
tampilkan "fixture valid"
"#).unwrap();
    fs::create_dir_all(root.join("tests/conformance/invalid")).unwrap();
    fs::write(root.join("tests/conformance/invalid/invalid.one"), r#"Buat program fixture invalid
hasil adalah belum dibuat
"#).unwrap();

    let main_path = root.join("src/main.one");
    let build_output = run(&["build", main_path.to_str().unwrap()]);
    assert!(build_output.status.success(), "{}", String::from_utf8_lossy(&build_output.stderr));
    assert!(root.join("src/target/main.oneir").is_file());

    let test_output = run(&["test", root.to_str().unwrap()]);
    assert!(test_output.status.success(), "{}", String::from_utf8_lossy(&test_output.stderr));
    assert!(String::from_utf8_lossy(&test_output.stdout).contains("Tests berhasil"));

    let output = run(&["package", root.to_str().unwrap()]);
    assert!(output.status.success(), "stdout: {}\nstderr: {}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    let artifact = root.join("target/humanapp-1.0.0.onepkg");
    let first = fs::read(&artifact).expect("package artifact should exist");
    let text = String::from_utf8_lossy(&first);
    assert!(text.starts_with("One human-core package v1"));
    assert!(text.contains("[frontend]\nhuman-core"));
    assert!(text.contains("[module-lock]"));
    assert!(text.contains("main.one sha256="));
    assert!(text.contains("math.one sha256="));
    assert!(text.contains("tambah"));
    let lock_path = root.join("target/humanapp-1.0.0.one.lock");
    let first_lock = fs::read(&lock_path).expect("module lock should exist");
    let lock_text = String::from_utf8_lossy(&first_lock);
    assert!(lock_text.contains("One module lock v2"));
    assert!(lock_text.contains("main.one sha256="));
    assert!(lock_text.contains("math.one sha256="));
    assert!(run(&["package", root.to_str().unwrap()]).status.success());
    assert_eq!(first, fs::read(&artifact).unwrap(), "human-core package must be deterministic");
    assert_eq!(first_lock, fs::read(&lock_path).unwrap(), "module lock must be deterministic");

    fs::write(root.join("src/math.one"), r#"Buat modul math
buat pekerjaan tambah
    terima kiri
    terima kanan
    hasilnya adalah kiri dikurangi kanan
"#).unwrap();
    let stale_test = run(&["test", root.to_str().unwrap()]);
    assert!(!stale_test.status.success());
    assert!(String::from_utf8_lossy(&stale_test.stderr).contains("ETOOL006"));
    let stale_build = run(&["build", main_path.to_str().unwrap()]);
    assert!(!stale_build.status.success());
    assert!(String::from_utf8_lossy(&stale_build.stderr).contains("ETOOL006"));

    assert!(run(&["package", root.to_str().unwrap()]).status.success());
    assert!(run(&["test", root.to_str().unwrap()]).status.success());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn package_rejects_unsupported_dependency_declarations_instead_of_ignoring_them() {
    let root = canonical_temp_dir().join(format!(
        "one-project-unsupported-dependency-{}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(
        root.join("one.toml"),
        "nama = \"paket\"\nversi = \"0.1.0\"\ndependensi = \"contoh = 1.0.0\"\n",
    )
    .unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"paket\"\n").unwrap();

    let output = run(&["package", root.to_str().unwrap()]);
    assert!(!output.status.success(), "unsupported dependency metadata must fail closed");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("field tidak valid atau duplikat"),
        "unexpected diagnostic: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!root.join("target/paket-0.1.0.onepkg").exists());
    let _ = fs::remove_dir_all(root);
}

#[test]
fn package_signature_cli_authenticates_trusted_publisher_and_detects_tampering() {
    let root = canonical_temp_dir().join(format!(
        "one-package-signature-cli-{}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    let source_root = root.join("project");
    fs::create_dir_all(&root).unwrap();
    fs::create_dir_all(source_root.join("src")).unwrap();
    fs::write(source_root.join("one.toml"), "nama = \"demo\"\nversi = \"0.1.0\"\n").unwrap();
    fs::write(source_root.join("src/main.one"), "Buat program demo\ntampilkan \"halo\"\n").unwrap();
    for args in [
        vec!["init"],
        vec!["config", "user.name", "ONE CLI Test"],
        vec!["config", "user.email", "one-cli-test@example.invalid"],
        vec!["config", "core.autocrlf", "false"],
        vec!["add", "--", "one.toml", "src/main.one"],
        vec!["commit", "-m", "source fixture"],
    ] {
        let output = Command::new("git").arg("-C").arg(&source_root).args(args).output().unwrap();
        assert!(output.status.success(), "git fixture setup failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let packaged = run(&["package", source_root.to_str().unwrap()]);
    assert!(
        packaged.status.success(),
        "package command failed: {}",
        String::from_utf8_lossy(&packaged.stderr)
    );
    let artifact = source_root.join("target/demo-0.1.0.onepkg");
    assert!(artifact.is_file(), "package command did not produce the final artifact");
    let artifact_bytes = fs::read(&artifact).unwrap();

    let private_key = root.join("publisher.seed");
    let trust_store = root.join("trusted-publishers.txt");
    let untrusted_store = root.join("untrusted-publishers.txt");

    let seed = [7u8; 32];
    let signing_key = SigningKey::from_bytes(&seed);
    let seed_hex = seed.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let public_hex = signing_key.verifying_key().to_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    fs::write(&private_key, seed_hex).unwrap();
    fs::write(&trust_store, format!("example-publisher={public_hex}\n")).unwrap();
    fs::write(&untrusted_store, format!("other-publisher={}\n", "00".repeat(32))).unwrap();

    let signed = run(&[
        "sign-package",
        artifact.to_str().unwrap(),
        "--publisher",
        "example-publisher",
        "--private-key",
        private_key.to_str().unwrap(),
        "--source-root",
        source_root.to_str().unwrap(),
    ]);
    assert!(signed.status.success(), "{}", String::from_utf8_lossy(&signed.stderr));
    assert!(artifact.with_extension("onepkg.provenance").is_file());

    let verified = run(&[
        "verify-package",
        artifact.to_str().unwrap(),
        "--trusted-publishers",
        trust_store.to_str().unwrap(),
        "--source-root",
        source_root.to_str().unwrap(),
    ]);
    assert!(verified.status.success(), "{}", String::from_utf8_lossy(&verified.stderr));
    assert!(String::from_utf8_lossy(&verified.stdout).contains("example-publisher"));
    assert!(String::from_utf8_lossy(&verified.stdout).contains("source tree"));

    let untrusted = run(&[
        "verify-package",
        artifact.to_str().unwrap(),
        "--trusted-publishers",
        untrusted_store.to_str().unwrap(),
        "--source-root",
        source_root.to_str().unwrap(),
    ]);
    assert!(!untrusted.status.success());
    assert!(String::from_utf8_lossy(&untrusted.stderr).contains("not trusted"));

    fs::write(&artifact, b"tampered package payload").unwrap();
    let tampered = run(&[
        "verify-package",
        artifact.to_str().unwrap(),
        "--trusted-publishers",
        trust_store.to_str().unwrap(),
        "--source-root",
        source_root.to_str().unwrap(),
    ]);
    assert!(!tampered.status.success());
    assert!(String::from_utf8_lossy(&tampered.stderr).contains("SHA-256"));

    fs::write(&artifact, artifact_bytes).unwrap();
    fs::write(source_root.join("src/main.one"), "Buat program demo\ntampilkan \"changed\"\n").unwrap();
    let output = Command::new("git").arg("-C").arg(&source_root).args(["add", "--", "src/main.one"]).output().unwrap();
    assert!(output.status.success());
    let output = Command::new("git").arg("-C").arg(&source_root).args(["commit", "-m", "change source"]).output().unwrap();
    assert!(output.status.success());
    let source_mismatch = run(&[
        "verify-package",
        artifact.to_str().unwrap(),
        "--trusted-publishers",
        trust_store.to_str().unwrap(),
        "--source-root",
        source_root.to_str().unwrap(),
    ]);
    assert!(!source_mismatch.status.success());
    assert!(String::from_utf8_lossy(&source_mismatch.stderr).contains("source tree digest"));

    let _ = fs::remove_dir_all(root);
}

