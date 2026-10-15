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

#[test]
fn expression_name_can_contain_structural_words() {
    let source = "nama ke adalah \"Kunci\"\ntampilkan nama ke\n";
    let program = parse(source).expect("structural word should remain name content outside its active boundary");
    let typed = analyze(&program).expect("name should analyze");
    let ir = lower(&typed).expect("name should lower");
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("name should execute");
    assert_eq!(result.output, vec!["Kunci"]);
}

#[test]
fn expression_name_can_contain_dan() {
    let source = "nama dan catatan adalah \"Kunci\"\ntampilkan nama dan catatan\n";
    let program = parse(source).expect("dan should remain name content outside call argument context");
    let typed = analyze(&program).expect("name should analyze");
    let ir = lower(&typed).expect("name should lower");
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("name should execute");
    assert_eq!(result.output, vec!["Kunci"]);
}

#[test]
fn list_name_can_contain_dan() {
    let source = "buat daftar nama dan catatan\ntampilkan nama dan catatan\n";
    let program = parse(source).expect("list name should parse");
    let typed = analyze(&program).expect("list name should analyze");
    let ir = lower(&typed).expect("list name should lower");
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("list name should execute");
    assert_eq!(result.output, vec!["[]"]);
}
