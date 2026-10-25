use one_core::runtime::Value;
use one_core::stdlib;
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
fn text_prefix_and_suffix_builtins_execute_through_the_canonical_pipeline() {
    let source = "teks adalah \"kucing\"\nawal adalah teks diawali dengan teks dan \"ku\"\nakhir adalah teks diakhiri dengan teks dan \"ing\"\nbukan awal adalah teks diawali dengan teks dan \"anjing\"\nbukan akhir adalah teks diakhiri dengan teks dan \"ing!\"\ntampilkan awal\ntampilkan akhir\ntampilkan bukan awal\ntampilkan bukan akhir\n";
    assert_eq!(
        run(source).expect("prefix and suffix builtins should execute"),
        vec!["benar", "benar", "salah", "salah"]
    );

    let bad_type = "hasil adalah teks diawali dengan 42 dan \"4\"\ntampilkan hasil\n";
    assert!(run(bad_type).unwrap_err().contains("E2043"));
}

#[test]
fn case_conversion_and_numeric_builtins_execute_through_the_canonical_pipeline() {
    let source = "kecil adalah huruf kecil dengan \"KUCING\"\nbesar adalah huruf besar dengan \"kucing\"\nminimum adalah angka minimum dengan 7 dan 3\nmaksimum adalah angka maksimum dengan 7 dan 3\nmutlak adalah nilai mutlak dengan 7\nbuat daftar nilai\ntambahkan 2 ke nilai\ntambahkan 3 ke nilai\njumlah adalah jumlah daftar angka dengan nilai\ntampilkan kecil\ntampilkan besar\ntampilkan minimum\ntampilkan maksimum\ntampilkan mutlak\ntampilkan jumlah\n";
    assert_eq!(
        run(source).expect("case conversion and numeric builtins should execute"),
        vec!["kucing", "KUCING", "3", "7", "7", "5"]
    );

    let bad_type = "hasil adalah angka minimum dengan \"7\" dan 3\ntampilkan hasil\n";
    assert!(run(bad_type).unwrap_err().contains("E2043"));
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
fn utc_timestamp_parser_runs_through_the_canonical_pipeline() {
    let source = "epoch adalah unix dari waktu utc dengan \"1970-01-01T00:00:00Z\"\npre_epoch adalah unix dari waktu utc dengan \"1969-12-31T23:59:59Z\"\nleap adalah unix dari waktu utc dengan \"2024-02-29T00:00:00Z\"\ntampilkan epoch\ntampilkan pre_epoch\ntampilkan leap\n";
    assert_eq!(
        run(source).expect("UTC timestamp parser should execute"),
        vec!["0", "-1", "1709164800"]
    );

    let invalid_type = "hasil adalah unix dari waktu utc dengan 0\ntampilkan hasil\n";
    assert!(run(invalid_type).unwrap_err().contains("E2043"));

    let invalid_date = "hasil adalah unix dari waktu utc dengan \"2023-02-29T00:00:00Z\"\ntampilkan hasil\n";
    assert!(run(invalid_date).unwrap_err().contains("R3022"));
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

#[test]
fn checked_integer_remainder_runs_through_the_canonical_pipeline() {
    let source = "positif adalah angka sisa bagi dengan 7 dan 3\nnegatif adalah 0 dikurangi 7\nnegatif sisa adalah angka sisa bagi dengan negatif dan 3\ntampilkan positif\ntampilkan negatif sisa\n";
    assert_eq!(
        run(source).expect("integer remainder should execute"),
        vec!["1", "-1"]
    );

    let wrong_type = "hasil adalah angka sisa bagi dengan \"7\" dan 3\ntampilkan hasil\n";
    assert!(run(wrong_type).unwrap_err().contains("E2043"));

    let zero_divisor = "hasil adalah angka sisa bagi dengan 7 dan 0\ntampilkan hasil\n";
    assert!(run(zero_divisor).unwrap_err().contains("R3022"));

    let overflow = "minimum adalah 0 dikurangi 9223372036854775807 dikurangi 1\nnegatif satu adalah 0 dikurangi 1\nhasil adalah angka sisa bagi dengan minimum dan negatif satu\ntampilkan hasil\n";
    assert!(run(overflow).unwrap_err().contains("R3022"));
}


#[test]
fn json_contract_rejects_duplicate_keys_exponents_and_excessive_nesting() {
    let duplicate_keys = r#"data adalah "{\"a\":1,\"a\":2}"
nilai adalah nilai dari json dengan data
tampilkan nilai
"#;
    assert!(
        run(duplicate_keys).unwrap_err().contains("R3022"),
        "duplicate JSON object keys must be rejected"
    );

    let exponent = r#"data adalah "1e2"
nilai adalah nilai dari json dengan data
tampilkan nilai
"#;
    assert!(
        run(exponent).unwrap_err().contains("R3022"),
        "JSON exponents are outside the integer-only v0.1 contract"
    );

    let nested_json = format!("{}0{}", "[".repeat(129), "]".repeat(129));
    let nested_source = format!(
        "data adalah \"{}\"\nnilai adalah nilai dari json dengan data\ntampilkan nilai\n",
        nested_json
    );
    assert!(
        run(&nested_source).unwrap_err().contains("R3022"),
        "JSON nesting deeper than 128 levels must be rejected"
    );
}

#[test]
fn mapping_insert_and_replace_preserve_deterministic_json_output() {
    let source = r#"buat pemetaan data
data menjadi atur pemetaan dengan data dan "kunci" dan 1
data menjadi atur pemetaan dengan data dan "kunci" dan 2
data menjadi atur pemetaan dengan data dan "alpha" dan "nilai"
hasil adalah ubah nilai ke json dengan data
tampilkan hasil
"#;
    let program = parse_human_core(source).expect("mapping grammar should parse");
    let typed = analyze(&program).expect("mapping operations should type-check");
    let ir = lower(&typed).expect("mapping operations should lower");
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("mapping operations should execute");
    assert_eq!(result.output, vec![r#"{"alpha":"nilai","kunci":2}"#]);
}


#[test]
fn text_and_integer_conversion_edge_contracts_reject_invalid_inputs() {
    for source in [
        "hasil adalah ganti teks dengan \"abc\" dan \"\" dan \"x\"\ntampilkan hasil\n",
        "hasil adalah pisah teks dengan \"a,b\" dan \"\"\ntampilkan hasil\n",
        "hasil adalah ubah teks ke angka dengan \" 42\"\ntampilkan hasil\n",
        "hasil adalah ubah teks ke angka dengan \"9223372036854775808\"\ntampilkan hasil\n",
    ] {
        let error = run(source).expect_err("invalid text/integer input must fail closed");
        assert!(error.contains("R3022"), "unexpected diagnostic for {source:?}: {error}");
    }

    let source = "awal epoch adalah 0 dikurangi 62167219200\nawal adalah waktu utc dari unix dengan awal epoch\nakhir adalah waktu utc dari unix dengan 253402300799\ntampilkan awal\ntampilkan akhir\n";
    assert_eq!(
        run(source).expect("UTC formatter must cover the full documented year range"),
        vec!["0000-01-01T00:00:00Z", "9999-12-31T23:59:59Z"]
    );
}

#[test]
fn utc_parser_rejects_noncanonical_time_forms() {
    for value in [
        "2024-01-01T00:00:00+00:00",
        "2024-01-01T00:00:00.000Z",
        "2024-01-01T00:00:60Z",
        "2024-1-01T00:00:00Z",
        "2023-02-29T00:00:00Z",
    ] {
        let source = format!(
            "hasil adalah unix dari waktu utc dengan \"{}\"\ntampilkan hasil\n",
            value
        );
        let error = run(&source).expect_err("noncanonical UTC form must be rejected");
        assert!(error.contains("R3022"), "unexpected diagnostic for {value:?}: {error}");
    }
}

#[test]
fn checked_numeric_and_collection_edges_return_typed_runtime_errors() {
    let min_i64 = "minimum adalah 0 dikurangi 9223372036854775807 dikurangi 1\n";
    for suffix in [
        "hasil adalah nilai mutlak dengan minimum\ntampilkan hasil\n",
        "minus satu adalah 0 dikurangi 1\nhasil adalah bagi bulat dengan minimum dan minus satu\ntampilkan hasil\n",
        "buat daftar angka\ntambahkan 9223372036854775807 ke angka\ntambahkan 1 ke angka\nhasil adalah jumlah daftar angka dengan angka\ntampilkan hasil\n",
        "buat daftar angka\nhasil adalah ambil indeks dengan angka dan 0\ntampilkan hasil\n",
    ] {
        let error = run(&format!("{min_i64}{suffix}"))
            .expect_err("overflow, invalid index, or non-integer collection contract must fail");
        assert!(
            error.contains("R3022") || error.contains("R3023"),
            "unexpected diagnostic for {suffix:?}: {error}"
        );
    }

    let wrong_list_sum = "buat daftar angka\ntambahkan \"teks\" ke angka\nhasil adalah jumlah daftar angka dengan angka\ntampilkan hasil\n";
    let error = run(wrong_list_sum).expect_err("non-integer sum item must fail");
    assert!(error.contains("R3022") || error.contains("R3023"), "unexpected diagnostic: {error}");
}

#[test]
fn text_builtins_cover_empty_values_unicode_scalars_and_i64_boundaries() {
    let source = r#"teks adalah "猫😀"
jumlah adalah panjang teks dengan teks
kosong adalah ""
awal adalah teks diawali dengan teks dan kosong
akhir adalah teks diakhiri dengan teks dan kosong
minimum adalah ubah teks ke angka dengan "-9223372036854775808"
maksimum adalah ubah teks ke angka dengan "9223372036854775807"
tampilkan jumlah
tampilkan awal
tampilkan akhir
tampilkan minimum
tampilkan maksimum
"#;
    assert_eq!(
        run(source).expect("Unicode, empty text, and signed-i64 boundaries should execute"),
        vec!["2", "benar", "benar", "-9223372036854775808", "9223372036854775807"]
    );

    for source in [
        r#"hasil adalah ganti teks dengan "abc" dan "" dan "x"
tampilkan hasil
"#,
        r#"hasil adalah pisah teks dengan "a,b" dan ""
tampilkan hasil
"#,
        r#"hasil adalah ubah teks ke angka dengan "-9223372036854775809"
tampilkan hasil
"#,
    ] {
        let error = run(source).expect_err("invalid text or integer boundary must fail closed");
        assert!(error.contains("R3022"), "unexpected diagnostic for {source:?}: {error}");
    }
}

#[test]
fn collection_builtins_cover_empty_join_and_invalid_indices() {
    let source = r#"buat daftar kosong
hasil adalah gabung teks dengan kosong dan ","
tampilkan hasil
"#;
    assert_eq!(
        run(source).expect("joining an empty list should return empty text"),
        vec![""]
    );

    for (setup, index_name) in [
        ("negatif adalah 0 dikurangi 1\n", "negatif"),
        ("di luar adalah 2\n", "di luar"),
    ] {
        let source = format!(
            "buat daftar nilai\ntambahkan \"satu\" ke nilai\n{setup}hasil adalah ambil indeks dengan nilai dan {index_name}\ntampilkan hasil\n"
        );
        let error = run(&source).expect_err("negative and out-of-range indices must fail closed");
        assert!(
            error.contains("R3022"),
            "unexpected diagnostic for index {index_name}: {error}"
        );
    }

    let wrong_join_member = r#"buat daftar nilai
tambahkan 1 ke nilai
hasil adalah gabung teks dengan nilai dan ","
tampilkan hasil
"#;
    let error = run(wrong_join_member).expect_err("join must reject non-text members");
    assert!(
        error.contains("R3022") || error.contains("E2043"),
        "unexpected diagnostic for non-text join member: {error}"
    );
}

#[test]
fn utc_parser_source_contract_includes_year_zero_and_upper_boundary() {
    let source = r#"awal adalah unix dari waktu utc dengan "0000-01-01T00:00:00Z"
akhir adalah unix dari waktu utc dengan "9999-12-31T23:59:59Z"
tampilkan awal
tampilkan akhir
"#;
    assert_eq!(
        run(source).expect("UTC parser should support the documented complete year range"),
        vec!["-62167219200", "253402300799"]
    );
}

#[test]
fn json_parser_rejects_malformed_escapes_surrogates_trailing_input_and_integer_overflow() {
    for source in [
        r#"data adalah "\"\\q\""
nilai adalah nilai dari json dengan data
tampilkan nilai
"#,
        r#"data adalah "\"\\uD800\""
nilai adalah nilai dari json dengan data
tampilkan nilai
"#,
        r#"data adalah "\"\\uDC00\""
nilai adalah nilai dari json dengan data
tampilkan nilai
"#,
        r#"data adalah "true false"
nilai adalah nilai dari json dengan data
tampilkan nilai
"#,
        r#"data adalah "01"
nilai adalah nilai dari json dengan data
tampilkan nilai
"#,
        r#"data adalah "9223372036854775808"
nilai adalah nilai dari json dengan data
tampilkan nilai
"#,
    ] {
        let error = run(source).expect_err("malformed JSON must be rejected");
        assert!(error.contains("R3022"), "unexpected diagnostic for {source:?}: {error}");
    }
}

#[test]
fn json_round_trip_preserves_unicode_escapes_and_deterministic_integer_values() {
    let source = r#"data adalah "{\"z\":\"line\\n\\u732b\",\"a\":-0}"
nilai adalah nilai dari json dengan data
hasil adalah ubah nilai ke json dengan nilai
tampilkan hasil
"#;
    assert_eq!(
        run(source).expect("valid escaped JSON should parse and serialize deterministically"),
        vec![r#"{"a":0,"z":"line\n猫"}"#]
    );

    let surrogate_pair = r#"data adalah "\"\\uD83D\\uDE00\""
nilai adalah nilai dari json dengan data
hasil adalah ubah nilai ke json dengan nilai
tampilkan hasil
"#;
    assert_eq!(
        run(surrogate_pair).expect("valid UTF-16 surrogate pairs should decode to a Unicode scalar"),
        vec![r#""😀""#]
    );
}

#[test]
fn mapping_builtin_rejects_missing_keys_and_invalid_key_types() {
    let missing_key = r#"buat pemetaan data
hasil adalah nilai pemetaan dengan data dan "tidak_ada"
tampilkan hasil
"#;
    let program = parse_human_core(missing_key).expect("mapping grammar should parse");
    let typed = analyze(&program).expect("mapping key lookup should type-check");
    let ir = lower(&typed).expect("mapping lookup should lower");
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let error = interpreter.execute(&ir).expect_err("missing map keys must fail closed");
    assert_eq!(error.code, "R3022");

    let invalid_key_type = r#"buat pemetaan data
hasil adalah nilai pemetaan dengan data dan 1
tampilkan hasil
"#;
    let program = parse_human_core(invalid_key_type).expect("mapping grammar should parse");
    let error = analyze(&program).expect_err("mapping keys must be text");
    assert!(
        format!("{error:?}").contains("E2043"),
        "unexpected diagnostic for non-text mapping key: {error:?}"
    );
}

#[test]
fn runtime_diagnostics_are_deterministic_for_identical_invalid_calls() {
    let invalid_division = "hasil adalah bagi bulat dengan 7 dan 0\ntampilkan hasil\n";
    let first = run(invalid_division).expect_err("division by zero must fail");
    let second = run(invalid_division).expect_err("same invalid call must fail again");
    assert_eq!(first, second, "identical invalid input must produce stable diagnostics");
    assert!(first.contains("R3022"), "expected the documented runtime diagnostic: {first}");
}

#[test]
fn mapping_builtins_work_in_default_parser_after_json_construction() {
    let source = r#"data adalah nilai dari json dengan "{\"z\":1,\"a\":2}"
lama adalah nilai pemetaan dengan data dan "a"
baru adalah atur pemetaan dengan data dan "b" dan 3
hasil adalah ubah nilai ke json dengan baru
tampilkan lama
tampilkan hasil
"#;
    assert_eq!(
        run(source).expect("mapping built-ins must work through the default parser and canonical runtime"),
        vec!["2", r#"{"a":2,"b":3,"z":1}"#]
    );
}


#[test]
fn public_rust_stdlib_helpers_have_reusable_typed_contracts() {
    let list = Value::List(vec![Value::Text("a".into()), Value::Text("b".into())]);
    let appended = stdlib::list_append(list.clone(), Value::Text("c".into()))
        .expect("append accepts a list");
    assert_eq!(
        appended,
        Value::List(vec![Value::Text("a".into()), Value::Text("b".into()), Value::Text("c".into())])
    );
    assert_eq!(
        stdlib::list_append(Value::Text("not a list".into()), Value::Text("c".into())),
        None,
        "wrong receiver type must fail closed"
    );

    let removed = stdlib::list_remove(appended.clone(), &Value::Text("b".into()))
        .expect("remove accepts a list");
    assert_eq!(
        removed,
        Value::List(vec![Value::Text("a".into()), Value::Text("c".into())]),
        "remove deletes only the first equal value"
    );
    assert_eq!(
        stdlib::list_remove(appended.clone(), &Value::Text("missing".into())),
        Some(appended.clone()),
        "removing a missing value is a deterministic no-op"
    );
    assert_eq!(stdlib::list_remove(Value::Number(1), &Value::Number(1)), None);

    assert_eq!(stdlib::text_length(&Value::Text("猫😀".into())), Some(Value::Number(2)));
    assert_eq!(stdlib::text_length(&Value::Number(2)), None);
    assert_eq!(stdlib::list_length(&appended), Some(Value::Number(3)));
    assert_eq!(stdlib::list_length(&Value::Text("abc".into())), None);
    assert_eq!(stdlib::list_get(&appended, &Value::Number(2)), Some(Value::Text("c".into())));
    assert_eq!(stdlib::list_get(&appended, &Value::Number(-1)), None);
    assert_eq!(stdlib::list_get(&appended, &Value::Number(i64::MAX)), None);

    assert_eq!(stdlib::number_abs(&Value::Number(i64::MIN)), None);
    assert_eq!(stdlib::number_divide(&Value::Number(i64::MIN), &Value::Number(-1)), None);
    assert_eq!(stdlib::number_modulo(&Value::Number(i64::MIN), &Value::Number(-1)), None);
    assert_eq!(stdlib::number_divide(&Value::Number(1), &Value::Number(0)), None);
    assert_eq!(stdlib::number_modulo(&Value::Number(1), &Value::Number(0)), None);
}

#[test]
fn public_rust_mapping_helpers_are_sorted_and_reject_wrong_types() {
    let first = stdlib::map_from_entries([
        ("z".to_owned(), Value::Number(1)),
        ("a".to_owned(), Value::Number(2)),
    ]);
    let second = stdlib::map_from_entries([
        ("a".to_owned(), Value::Number(2)),
        ("z".to_owned(), Value::Number(1)),
    ]);
    assert_eq!(first, second, "mapping construction must not depend on insertion order");
    assert_eq!(
        stdlib::value_to_json(&first),
        Some(Value::Text(r#"{"a":2,"z":1}"#.into()))
    );
    assert_eq!(
        stdlib::map_get(&first, &Value::Text("a".into())),
        Some(Value::Number(2))
    );
    assert_eq!(stdlib::map_get(&first, &Value::Number(1)), None);
    assert_eq!(
        stdlib::map_set(first.clone(), &Value::Text("a".into()), Value::Number(9)),
        Some(stdlib::map_from_entries([
            ("a".to_owned(), Value::Number(9)),
            ("z".to_owned(), Value::Number(1)),
        ]))
    );
    assert_eq!(stdlib::map_set(first, &Value::Number(1), Value::Number(9)), None);
}

#[test]
fn public_rust_json_helpers_fail_closed_and_are_deterministic() {
    let value = Value::Text(r#"{"z":1,"a":"猫😀"}"#.into());
    let parsed = stdlib::json_to_value(&value).expect("valid JSON should parse");
    let serialized = stdlib::value_to_json(&parsed).expect("parsed value should serialize");
    assert_eq!(serialized, Value::Text(r#"{"a":"猫😀","z":1}"#.into()));
    assert_eq!(stdlib::json_to_value(&Value::Number(1)), None);
    assert_eq!(stdlib::json_to_value(&Value::Text(r#"{"x":1,"x":2}"#.into())), None);
    assert_eq!(stdlib::json_to_value(&Value::Text("1e2".into())), None);
    assert_eq!(stdlib::json_to_value(&Value::Text(r#""\uD800""#.into())), None);
}
