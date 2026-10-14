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
fn rejects_undefined_name() {
    let program = parse("tampilkan nama\n").expect("parse");
    assert!(analyze(&program).is_err());
}

#[test]
fn rejects_bad_indentation() {
    let source = "kalau benar\n  tampilkan \"salah indentasi\"\n";
    assert!(parse(source).is_err());
}
