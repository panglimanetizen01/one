use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};
fn fixture(name: &str, source: &str) -> std::path::PathBuf { let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let path = std::env::temp_dir().join(format!("one-{name}-{id}.one")); fs::write(&path, source).unwrap(); path }
fn run(args: &[&str]) -> std::process::Output { Command::new(env!("CARGO_BIN_EXE_one")).args(args).output().unwrap() }
fn run_in(directory: &std::path::Path, args: &[&str]) -> std::process::Output { Command::new(env!("CARGO_BIN_EXE_one")).current_dir(directory).args(args).output().unwrap() }
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
#[test]
fn run_command_uses_working_directory_as_filesystem_root_not_source_location() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("one-filesystem-root-{id}"));
    fs::create_dir_all(&root).unwrap();
    let data = root.join("data.txt");
    let source = std::env::temp_dir().join(format!("one-filesystem-source-outside-root-{id}.one"));
    let data_arg = portable_path(&data);
    fs::write(&data, "isi One\nbaris kedua\n").unwrap();
    fs::write(&source, format!("baca berkas \"{}\"\n", data_arg)).unwrap();

    // The source file is outside the working directory. It must not widen
    // the trusted filesystem root beyond the host-selected current directory.
    let output = run_in(&root, &["run", source.to_str().unwrap(), "--izinkan", "filesystem.read", "--target", &data_arg]);
    let _ = fs::remove_file(&source);
    let _ = fs::remove_dir_all(&root);
    assert!(output.status.success(), "filesystem read command failed: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "isi One\nbaris kedua\n");
}
#[test]
fn run_command_does_not_allow_source_location_to_widen_filesystem_root() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("one-filesystem-deny-root-{id}"));
    fs::create_dir_all(&root).unwrap();
    let outside = std::env::temp_dir().join(format!("one-filesystem-outside-{id}.txt"));
    let source = std::env::temp_dir().join(format!("one-filesystem-source-widen-{id}.one"));
    let outside_arg = portable_path(&outside);
    fs::write(&outside, "rahasia di luar root\n").unwrap();
    fs::write(&source, format!("baca berkas \"{}\"\n", outside_arg)).unwrap();

    // The source and target share a parent outside cwd. Neither may widen
    // the host-selected root to that parent directory.
    let output = run_in(&root, &["run", source.to_str().unwrap(), "--izinkan", "filesystem.read", "--target", &outside_arg]);
    let _ = fs::remove_file(&source);
    let _ = fs::remove_file(&outside);
    let _ = fs::remove_dir_all(&root);
    assert!(!output.status.success(), "filesystem authority widened to the source parent");
    assert!(output.stdout.is_empty());
}
#[test] fn filesystem_read_is_denied_without_explicit_capability() { let source_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let data = std::env::temp_dir().join(format!("one-filesystem-denied-data-{source_id}.txt")); let source = std::env::temp_dir().join(format!("one-filesystem-denied-source-{source_id}.one")); let data_arg = portable_path(&data); fs::write(&data, "rahasia\n").unwrap(); fs::write(&source, format!("baca berkas \"{}\"\n", data_arg)).unwrap(); let output = run(&["run", source.to_str().unwrap()]); let _ = fs::remove_file(&data); let _ = fs::remove_file(&source); assert!(!output.status.success()); assert!(String::from_utf8_lossy(&output.stderr).contains("N3205")); }
#[test] fn filesystem_read_rejects_out_of_scope_target() { let source_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(); let data = std::env::temp_dir().join(format!("one-filesystem-scope-data-{source_id}.txt")); let source = std::env::temp_dir().join(format!("one-filesystem-scope-source-{source_id}.one")); let data_arg = portable_path(&data); let scope_arg = portable_path(&std::env::temp_dir()); fs::write(&data, "di luar\n").unwrap(); fs::write(&source, format!("baca berkas \"{}\"\n", data_arg)).unwrap(); let output = run(&["run", source.to_str().unwrap(), "--izinkan", "filesystem.read", "--target", &scope_arg]); let _ = fs::remove_file(&data); let _ = fs::remove_file(&source); assert!(!output.status.success()); assert!(String::from_utf8_lossy(&output.stderr).contains("N3209")); }

#[test] fn test_command_rejects_missing_project_manifest() {
    let root = std::env::temp_dir().join(format!("one-project-test-missing-manifest-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"uji\"\n").unwrap();
    let output = run(&["test", root.to_str().unwrap()]);
    let _ = fs::remove_dir_all(&root);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("ETOOL002"));
}

#[test] fn test_command_rejects_invalid_project_manifest() {
    let root = std::env::temp_dir().join(format!("one-project-test-invalid-manifest-{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("one.toml"), "nama = \"uji\"\n").unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"uji\"\n").unwrap();
    let output = run(&["test", root.to_str().unwrap()]);
    let _ = fs::remove_dir_all(&root);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("ETOOL002"));
}


#[test] fn package_rejects_path_traversal_in_project_name() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("one-project-package-path-traversal-{id}"));
    let escaped = std::env::temp_dir().join(format!("one-package-escape-{id}-0.1.0.onepkg"));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("one.toml"), format!("nama = \"../../one-package-escape-{id}\"\nversi = \"0.1.0\"\n")).unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"uji\"\n").unwrap();
    let output = run(&["package", root.to_str().unwrap()]);
    let escaped_created = escaped.exists();
    let _ = fs::remove_file(&escaped);
    let _ = fs::remove_dir_all(&root);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("ETOOL002"));
    assert!(!escaped_created, "package output must not escape the project target directory");
}

#[test] fn package_rejects_path_traversal_in_project_version() {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!("one-project-package-version-traversal-{id}"));
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("one.toml"), "nama = \"paket\"\nversi = \"../../escape\"\n").unwrap();
    fs::write(root.join("src/main.one"), "tampilkan \"uji\"\n").unwrap();
    let output = run(&["package", root.to_str().unwrap()]);
    let _ = fs::remove_dir_all(&root);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("ETOOL002"));
}
