use one_core::{analyze, lower, parse, CapabilitySet, Interpreter};

#[test]
fn declaration_and_reference_support_multi_word_names() {
    let source = "nama lengkap adalah \"Budi Santoso\"\ntampilkan nama lengkap\n";
    let program = parse(source).expect("multi-word declaration should parse");
    assert_eq!(program.statements.len(), 2);
    let typed = analyze(&program).expect("multi-word declaration should analyze");
    let ir = lower(&typed).expect("multi-word declaration should lower");
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("multi-word declaration should execute");
    assert_eq!(result.output, vec!["Budi Santoso"]);
}

#[test]
fn assignment_keeps_multi_word_name_boundary() {
    let source = "nama lengkap adalah \"Budi\"\nnama lengkap menjadi \"Siti\"\ntampilkan nama lengkap\n";
    let program = parse(source).expect("multi-word assignment should parse");
    let typed = analyze(&program).expect("multi-word assignment should analyze");
    let ir = lower(&typed).expect("multi-word assignment should lower");
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("multi-word assignment should execute");
    assert_eq!(result.output, vec!["Siti"]);
}
