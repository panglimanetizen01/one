use one_core::{analyze, lower, parse, parse_human_core, CapabilitySet, Interpreter};

fn run(source: &str) -> Result<Vec<String>, String> {
    let program = parse(source).map_err(|error| format!("parse: {error:?}"))?;
    let typed = analyze(&program).map_err(|error| format!("analyze: {error:?}"))?;
    let ir = lower(&typed).map_err(|error| format!("lower: {error:?}"))?;
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    interpreter.execute(&ir).map(|result| result.output).map_err(|error| format!("runtime {}: {}", error.code, error.message))
}

#[test]
fn text_builtins_execute_through_the_canonical_pipeline() {
    let source = "teks adalah \"  kucing-kucing  \"\njumlah adalah panjang teks dengan teks\nbersih adalah pangkas teks dengan teks\nada adalah teks memuat dengan teks dan \"kucing\"\nhasil adalah ganti teks dengan bersih dan \"kucing\" dan \"anjing\"\ntampilkan jumlah\ntampilkan bersih\ntampilkan ada\ntampilkan hasil\n";
    assert_eq!(
        run(source).expect("builtins should execute"),
        vec!["17", "kucing-kucing", "benar", "anjing-anjing"]
    );
}

#[test]
fn split_and_integer_conversion_return_typed_values() {
    let source = "bagian adalah pisah teks dengan \"apel,jeruk\" dan \",\"\nangka adalah ubah teks ke angka dengan \"42\"\ntampilkan bagian\ntampilkan angka\n";
    assert_eq!(run(source).expect("conversion should execute"), vec!["[apel, jeruk]", "42"]);
}

#[test]
fn list_builtins_read_values_and_length() {
    let source = "buat daftar buah\ntambahkan \"apel\" ke buah\ntambahkan \"jeruk\" ke buah\njumlah adalah panjang daftar dengan buah\nbuah pertama adalah ambil indeks dengan buah dan 0\ntampilkan jumlah\ntampilkan buah pertama\n";
    assert_eq!(run(source).expect("list builtins should execute"), vec!["2", "apel"]);
}

#[test]
fn builtins_report_invalid_arguments_and_zero_division() {
    let bad_type = "hasil adalah panjang teks dengan 42\ntampilkan hasil\n";
    assert!(run(bad_type).unwrap_err().contains("E2043"));

    let zero_division = "hasil adalah bagi bulat dengan 7 dan 0\ntampilkan hasil\n";
    assert!(run(zero_division).unwrap_err().contains("R3022"));
}

#[test]
fn builtin_arity_is_checked_during_semantic_analysis() {
    let source = "hasil adalah ganti teks dengan \"satu\" dan \"dua\"\ntampilkan hasil\n";
    assert!(run(source).unwrap_err().contains("E2018"));
}

#[test]
fn integer_arithmetic_overflow_returns_a_runtime_diagnostic() {
    let add_overflow = "hasil adalah 9223372036854775807 ditambah 1\ntampilkan hasil\n";
    let add_error = run(add_overflow).unwrap_err();
    assert!(add_error.contains("R3023"), "unexpected addition overflow error: {add_error}");

    let multiply_overflow = "hasil adalah 9223372036854775807 dikali 2\ntampilkan hasil\n";
    let multiply_error = run(multiply_overflow).unwrap_err();
    assert!(multiply_error.contains("R3023"), "unexpected multiplication overflow error: {multiply_error}");
}

#[test]
fn unix_timestamp_formatter_runs_through_the_canonical_pipeline() {
    let source = "epoch adalah waktu utc dari unix dengan 0\nleap adalah waktu utc dari unix dengan 1709164800\ntampilkan epoch\ntampilkan leap\n";
    assert_eq!(
        run(source).expect("UTC timestamp formatter should execute"),
        vec![
            "1970-01-01T00:00:00Z",
            "2024-02-29T00:00:00Z",
        ]
    );

    let invalid_type = "hasil adalah waktu utc dari unix dengan \"0\"\ntampilkan hasil\n";
    assert!(run(invalid_type).unwrap_err().contains("E2043"));

    let out_of_range = "hasil adalah waktu utc dari unix dengan 253402300800\ntampilkan hasil\n";
    assert!(run(out_of_range).unwrap_err().contains("R3022"));
}

#[test]
fn checked_in_stdlib_example_runs_through_one() {
    let output = run(include_str!("../../../examples/stdlib.one")).expect("example should execute");
    assert_eq!(output, vec!["2", r#"["apel","jeruk"]"#, "benar", "pir,jeruk", "42", "7", "3"]);
}

#[test]
fn mapping_builtins_work_in_the_opt_in_human_core_pipeline() {
    let source = r#"buat pemetaan kontak
masukkan "nama" dan "Budi" ke kontak
nama awal adalah nilai pemetaan dengan kontak dan "nama"
kontak menjadi atur pemetaan dengan kontak dan "nama" dan "Ani"
nama baru adalah nilai pemetaan dengan kontak dan "nama"
json adalah ubah nilai ke json dengan kontak
tampilkan nama awal
tampilkan nama baru
tampilkan json
"#;
    let program = parse_human_core(source).expect("human-core mapping grammar should parse");
    let typed = analyze(&program).expect("mapping builtin types should analyze");
    let ir = lower(&typed).expect("mapping program should lower");
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("mapping builtins should execute");
    assert_eq!(result.output, vec!["Budi", "Ani", r#"{"nama":"Ani"}"#]);
}

#[test]
fn checked_builtin_failures_are_runtime_errors_not_guessed_values() {
    let bad_conversion = r#"hasil adalah ubah teks ke angka dengan "bukan angka"
tampilkan hasil
"#;
    assert!(run(bad_conversion).unwrap_err().contains("R3022"));

    let invalid_index = r#"buat daftar buah
negatif adalah 0 dikurangi 1
hasil adalah ambil indeks dengan buah dan negatif
tampilkan hasil
"#;
    assert!(run(invalid_index).unwrap_err().contains("R3022"));
}

#[test]
fn mapping_lookup_of_a_missing_key_returns_a_runtime_error() {
    let source = r#"buat pemetaan kontak
hasil adalah nilai pemetaan dengan kontak dan "tidak ada"
tampilkan hasil
"#;
    let program = parse_human_core(source).expect("mapping source should parse");
    let typed = analyze(&program).expect("mapping source should analyze");
    let ir = lower(&typed).expect("mapping source should lower");
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    assert_eq!(interpreter.execute(&ir).unwrap_err().code, "R3022");
}

#[test]
fn recursive_calls_stop_at_the_runtime_depth_limit() {
    let source = r#"buat pekerjaan ulang
    terima nilai
    hasilnya adalah ulang dengan nilai
hasil adalah ulang dengan 1
tampilkan hasil
"#;
    let error = run(source).unwrap_err();
    assert!(error.contains("R3024"), "unexpected recursion error: {error}");
}

#[test]
fn unicode_text_and_collection_builtins_execute_through_one() {
    let source = r#"teks adalah "Äpel"
kecil adalah huruf kecil dengan teks
besar adalah huruf besar dengan teks
awal adalah teks diawali dengan teks dan "Ä"
akhir adalah teks diakhiri dengan teks dan "el"
buat daftar kata
tambahkan "apel" ke kata
tambahkan "jeruk" ke kata
gabungan adalah gabung teks dengan kata dan " | "
buat daftar angka
tambahkan 2 ke angka
tambahkan 3 ke angka
jumlah adalah jumlah daftar angka dengan angka
tampilkan kecil
tampilkan besar
tampilkan awal
tampilkan akhir
tampilkan gabungan
tampilkan jumlah
"#;
    assert_eq!(
        run(source).expect("text and collection builtins should execute"),
        vec!["äpel", "ÄPEL", "benar", "benar", "apel | jeruk", "5"]
    );
}

#[test]
fn json_parse_and_serialize_builtins_round_trip_through_one() {
    let source = r#"data adalah "{\"a\":1}"
nilai adalah nilai dari json dengan data
hasil adalah ubah nilai ke json dengan nilai
tampilkan nilai
tampilkan hasil
"#;
    assert_eq!(run(source).expect("JSON builtins should execute"), vec!["{a: 1}", r#"{"a":1}"#]);

    let invalid = r#"data adalah "1.5"
nilai adalah nilai dari json dengan data
tampilkan nilai
"#;
    assert!(run(invalid).unwrap_err().contains("R3022"));
}
