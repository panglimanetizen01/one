use one_core::{analyze, lower, parse, CapabilitySet, Interpreter};

fn run(source: &str) -> Vec<String> {
    let program = parse(source).expect("parse");
    let typed = analyze(&program).expect("semantic analysis");
    let ir = lower(&typed).expect("lowering");
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    interpreter.execute(&ir).expect("execution").output
}

#[test]
fn executes_data_and_action() {
    assert_eq!(run("nama adalah \"Budi\"\ntampilkan nama\n"), vec!["Budi"]);
}

#[test]
fn executes_condition() {
    let source = "umur adalah 18\nkalau umur sudah 17 tahun\n    tampilkan \"Boleh masuk\"\nkalau tidak\n    tampilkan \"Belum boleh masuk\"\n";
    assert_eq!(run(source), vec!["Boleh masuk"]);
}

#[test]
fn executes_list_loop() {
    let source = "buat daftar belanja\ntambahkan beras ke belanja\ntambahkan telur ke belanja\nuntuk setiap item di belanja\n    tampilkan item\n";
    assert_eq!(run(source), vec!["beras", "telur"]);
}

#[test]
fn executes_multi_word_loop_variable() {
    let source = "buat daftar belanja\ntambahkan beras ke belanja\nuntuk setiap nama barang di belanja\n    tampilkan nama barang\n";
    assert_eq!(run(source), vec!["beras"]);
}

#[test]
fn rejects_undefined_name() {
    let program = parse("tampilkan nama\n").expect("parse");
    assert!(analyze(&program).is_err());
}

#[test]
fn rejects_assignment_before_declaration() {
    let program = parse("nilai menjadi 7\n").expect("parse");
    let error = analyze(&program).expect_err("assignment before declaration must fail");
    assert_eq!(error.0[0].code, "E2002");
}

#[test]
fn rejects_incompatible_reassignment() {
    let program = parse("nilai adalah 7\nnilai menjadi \"teks\"\n").expect("parse");
    let error = analyze(&program).expect_err("incompatible reassignment must fail");
    assert_eq!(error.0[0].code, "E2003");
}

#[test]
fn rejects_bad_indentation() {
    let source = "kalau benar\n  tampilkan \"salah indentasi\"\n";
    assert!(parse(source).is_err());
}

#[test]
fn rejects_list_declaration_when_scalar_name_already_exists() {
    let program = parse("belanja adalah 7\nbuat daftar belanja\n").expect("parse");
    let error = analyze(&program).expect_err("list declaration must not replace scalar");
    assert_eq!(error.0[0].code, "E2001");
}

#[test]
fn rejects_duplicate_list_declaration() {
    let program = parse("buat daftar belanja\nbuat daftar belanja\n").expect("parse");
    let error = analyze(&program).expect_err("duplicate list must be rejected");
    assert_eq!(error.0[0].code, "E2001");
}

#[test]
fn allows_shadowing_in_a_nested_scope() {
    let source = "nilai adalah 1\nkalau benar\n    nilai adalah 2\n    tampilkan nilai\ntampilkan nilai\n";
    assert_eq!(run(source), vec!["2", "1"]);
}

#[test]
fn runtime_rejects_list_action_that_would_replace_existing_value() {
    let program = one_core::IrProgram {
        statements: vec![
            one_core::IrStatement::Declaration {
                name: "belanja".into(),
                value: one_core::IrExpr::Number(7),
            },
            one_core::IrStatement::Action {
                name: "buat daftar".into(),
                args: vec![one_core::IrExpr::Identifier("belanja".into())],
            },
        ],
    };
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let error = interpreter.execute(&program).expect_err("runtime must not overwrite");
    assert_eq!(error.code, "R3022");
}


#[test]
fn runtime_rejects_assignment_to_undefined_name() {
    let program = one_core::IrProgram {
        statements: vec![one_core::IrStatement::Assignment {
            name: "hilang".into(),
            value: one_core::IrExpr::Number(1),
        }],
    };
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let error = interpreter.execute(&program).expect_err("runtime must reject undefined assignment");
    assert_eq!(error.code, "R3003");
}

#[test]
fn runtime_rejects_duplicate_ir_declaration() {
    let program = one_core::IrProgram {
        statements: vec![
            one_core::IrStatement::Declaration {
                name: "nilai".into(),
                value: one_core::IrExpr::Number(1),
            },
            one_core::IrStatement::Declaration {
                name: "nilai".into(),
                value: one_core::IrExpr::Number(2),
            },
        ],
    };
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let error = interpreter.execute(&program).expect_err("runtime must reject duplicate");
    assert_eq!(error.code, "R3022");
}

#[test]
fn rejects_scalar_declaration_when_list_name_already_exists() {
    let program = parse("buat daftar belanja\nbelanja adalah 7\n").expect("parse");
    let error = analyze(&program).expect_err("scalar declaration must not replace list");
    assert_eq!(error.0[0].code, "E2001");
}
