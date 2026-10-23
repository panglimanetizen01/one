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
    assert!(String::from_utf8_lossy(&output.stderr).contains("E2002"));
}

#[test]
fn human_frontend_requires_explicit_program_declaration() {
    let path = fixture("tampilkan \"halo\"\n");
    let output = run(&path);
    let _ = fs::remove_file(&path);
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("HV1001"));
}
