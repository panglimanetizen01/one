use std::{fs, process::Command, time::{SystemTime, UNIX_EPOCH}};

fn fixture(source: &str) -> std::path::PathBuf {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("one-human-core-{id}.one"));
    fs::write(&path, source).unwrap();
    path
}

fn run(path: &std::path::Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_one"))
        .args(["jalankan-human-core", path.to_str().unwrap()])
        .output().unwrap()
}

#[test]
fn human_frontend_runs_functions_conditions_and_values_through_canonical_ir() {
    let path = fixture(concat!(
        "Buat program kalkulator\n",
        "buat pekerjaan jumlahkan\n",
        "    terima kiri\n",
        "    terima kanan\n",
        "    hasilnya adalah kiri ditambah kanan\n",
        "hasil adalah jumlahkan dengan 20 dan 22\n",
        "kalau hasil sama dengan 42\n",
        "    tampilkan hasil\n",
        "kalau tidak\n",
        "    tampilkan \"hasil salah\"\n",
    ));
    let output = run(&path);
    let _ = fs::remove_file(&path);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "42");
}

#[test]
fn human_frontend_reports_unknown_names_instead_of_succeeding_silently() {
    let path = fixture("Buat program tidak valid\ntampilkan nama_yang_tidak_ada\n");
    let output = run(&path);
    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("E2014"));
}

#[test]
fn human_frontend_requires_explicit_program_declaration() {
    let path = fixture("tampilkan \"halo\"\n");
    let output = run(&path);
    let _ = fs::remove_file(&path);
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("HV1001"));
}

#[test]
fn human_frontend_resolves_relative_modules_and_executes_shared_functions() {
    let root = std::env::temp_dir().join(format!(
        "one-human-modules-{}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&root).unwrap();
    let module = root.join("matematika.one");
    let program = root.join("main.one");
    fs::write(&module, concat!(
        "Buat modul matematika\n",
        "buat pekerjaan jumlahkan\n",
        "    terima kiri\n",
        "    terima kanan\n",
        "    hasilnya adalah kiri ditambah kanan\n",
    )).unwrap();
    fs::write(&program, concat!(
        "Buat program modular\n",
        "Gunakan modul \"matematika.one\"\n",
        "hasil adalah jumlahkan dengan 20 dan 22\n",
        "tampilkan hasil\n",
    )).unwrap();
    let output = run(&program);
    let _ = fs::remove_dir_all(&root);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "42");
}

#[test]
fn human_frontend_rejects_module_path_traversal() {
    let root = std::env::temp_dir().join(format!(
        "one-human-module-traversal-{}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&root).unwrap();
    let program = root.join("main.one");
    fs::write(&program, concat!(
        "Buat program tidak aman\n",
        "Gunakan modul \"../rahasia.one\"\n",
        "tampilkan \"tidak boleh berjalan\"\n",
    )).unwrap();
    let output = run(&program);
    let _ = fs::remove_dir_all(&root);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("HV1016"));
}

#[test]
fn human_frontend_executes_collection_operations_through_canonical_runtime() {
    let path = fixture(concat!(
        "Buat program daftar belanja\n",
        "buat daftar belanja\n",
        "tambahkan \"roti\" ke belanja\n",
        "tambahkan \"susu\" ke belanja\n",
        "tampilkan belanja\n",
    ));
    let output = run(&path);
    let _ = fs::remove_file(&path);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("roti"));
    assert!(stdout.contains("susu"));
}

#[test]
fn human_frontend_rejects_invalid_type_operations_with_semantic_diagnostic() {
    let path = fixture("Buat program tipe salah\nnilai adalah \"empat\" ditambah 1\ntampilkan nilai\n");
    let output = run(&path);
    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("E20"));
}

#[test]
fn human_frontend_enforces_block_scope() {
    let path = fixture(concat!(
        "Buat program ruang lingkup\n",
        "kalau benar\n",
        "    rahasia adalah 7\n",
        "tampilkan rahasia\n",
    ));
    let output = run(&path);
    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("E2014"));
}

#[test]
fn human_frontend_executes_bounded_while_loop_and_assignment() {
    let path = fixture(concat!(
        "Buat program hitung\n",
        "jumlah adalah 0\n",
        "selama jumlah lebih kecil dari 3\n",
        "    jumlah menjadi jumlah ditambah 1\n",
        "tampilkan jumlah\n",
    ));
    let output = run(&path);
    let _ = fs::remove_file(&path);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "3");
}

#[test]
fn human_frontend_rejects_duplicate_module_imports() {
    let root = std::env::temp_dir().join(format!(
        "one-human-module-duplicate-{}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("shared.one"), "Buat modul shared\n").unwrap();
    let program = root.join("main.one");
    fs::write(&program, concat!(
        "Buat program duplikat\n",
        "Gunakan modul \"shared.one\"\n",
        "Gunakan modul \"shared.one\"\n",
        "tampilkan \"tidak boleh berjalan\"\n",
    )).unwrap();
    let output = run(&program);
    let _ = fs::remove_dir_all(&root);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("HV1013"));
}

#[test]
fn human_frontend_rejects_import_cycles() {
    let root = std::env::temp_dir().join(format!(
        "one-human-module-cycle-{}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&root).unwrap();
    let program = root.join("main.one");
    fs::write(&program, "Buat program siklus\nGunakan modul \"modul.one\"\ntampilkan \"x\"\n").unwrap();
    fs::write(root.join("modul.one"), "Buat modul modul\nGunakan modul \"main.one\"\n").unwrap();
    let output = run(&program);
    let _ = fs::remove_dir_all(&root);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("HV1012"));
}

#[test]
fn second_real_program_runs_through_the_same_human_compiler_and_runtime() {
    let path = fixture(concat!(
        "Buat program pencatat skor\n",
        "skor awal adalah 10\n",
        "bonus adalah 5\n",
        "skor akhir adalah skor awal ditambah bonus\n",
        "kalau skor akhir sama dengan 15\n",
        "    tampilkan \"Skor tervalidasi\"\n",
        "kalau tidak\n",
        "    tampilkan \"Skor tidak valid\"\n",
    ));
    let output = run(&path);
    let _ = fs::remove_file(&path);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "Skor tervalidasi");
}
