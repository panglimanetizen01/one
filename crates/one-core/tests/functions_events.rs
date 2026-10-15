use one_core::{analyze, lower, parse, CapabilitySet, Interpreter};

fn compile(source: &str) -> one_core::IrProgram {
    let program = parse(source).expect("parse");
    let typed = analyze(&program).expect("analyze");
    lower(&typed).expect("lower")
}

#[test]
fn executes_function_result() {
    let source = "buat pekerjaan hitung\n    terima harga\n    terima pajak\n    hasilnya adalah harga ditambah pajak\n\ntotal adalah hitung dengan 100 dan 10\ntampilkan total\n";
    let ir = compile(source);
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("execute");
    assert_eq!(result.output, vec!["110"]);
}

#[test]
fn registers_and_dispatches_event() {
    let source = "ketika pengguna menekan tombol beli\n    tampilkan \"dibeli\"\n";
    let ir = compile(source);
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    interpreter.execute(&ir).expect("register event");
    let result = interpreter.dispatch_event("pengguna menekan tombol beli").expect("dispatch");
    assert_eq!(result.output, vec!["dibeli"]);
}

#[test]
fn dispatched_event_uses_program_state() {
    let source = "nama adalah \"Budi\"\nketika pengguna menekan tombol beli\n    tampilkan nama\n";
    let ir = compile(source);
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    interpreter.execute(&ir).expect("execute");
    let result = interpreter.dispatch_event("pengguna menekan tombol beli").expect("dispatch");
    assert_eq!(result.output, vec!["Budi"]);
}
