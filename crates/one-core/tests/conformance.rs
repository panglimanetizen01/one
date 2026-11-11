use one_core::{analyze, lower, parse, CapabilitySet, Interpreter};

fn compile(source: &str) -> one_core::IrProgram {
    let program = parse(source).expect("fixture should parse");
    let typed = analyze(&program).expect("fixture should analyze");
    lower(&typed).expect("fixture should lower")
}

#[test]
fn valid_fixtures_parse_analyze_and_lower() {
    let fixtures = [
        include_str!("../../../tests/conformance/valid/core.one"),
        include_str!("../../../tests/conformance/valid/lists.one"),
        include_str!("../../../tests/conformance/valid/functions.one"),
        include_str!("../../../tests/conformance/valid/loops.one"),
        include_str!("../../../tests/conformance/valid/while.one"),
        include_str!("../../../tests/conformance/valid/events.one"),
    ];

    for source in fixtures {
        let _ = compile(source);
    }
}

#[test]
fn invalid_fixtures_fail_with_stable_codes() {
    let cases = [
        (include_str!("../../../tests/conformance/invalid/missing-expression.one"), "E1013"),
        (include_str!("../../../tests/conformance/invalid/missing-comparison.one"), "E1014"),
        (include_str!("../../../tests/conformance/invalid/undefined-name.one"), "E2014"),
        (include_str!("../../../tests/conformance/invalid/duplicate-list-name.one"), "E2001"),
        (include_str!("../../../tests/conformance/invalid/assignment-before-declaration.one"), "E2002"),
        (include_str!("../../../tests/conformance/invalid/incompatible-reassignment.one"), "E2003"),
        (include_str!("../../../tests/conformance/invalid/bad-indentation.one"), "E0002"),
        (include_str!("../../../tests/conformance/invalid/empty-block.one"), "E1002"),
        (include_str!("../../../tests/conformance/invalid/implicit-action.one"), "E1016"),
    ];

    for (source, expected_code) in cases {
        let error = match parse(source) {
            Ok(program) => analyze(&program).expect_err("fixture should be rejected"),
            Err(error) => error,
        };
        assert_eq!(error.0[0].code, expected_code);
    }
}

#[test]
fn golden_core_output_is_stable() {
    let ir = compile(include_str!("../../../tests/conformance/valid/core.one"));
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("fixture should execute");
    let expected = include_str!("../../../tests/golden/core.snap")
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<_>>();
    assert_eq!(result.output, expected);
}

#[test]
fn golden_function_output_is_stable() {
    let ir = compile(include_str!("../../../tests/conformance/valid/functions.one"));
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("fixture should execute");
    let expected = include_str!("../../../tests/golden/functions.snap")
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<_>>();
    assert_eq!(result.output, expected);
}

#[test]
fn golden_list_output_is_stable() {
    let ir = compile(include_str!("../../../tests/conformance/valid/lists.one"));
    let mut interpreter = Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute(&ir).expect("fixture should execute");
    let expected = include_str!("../../../tests/golden/lists.snap")
        .lines()
        .filter(|line| !line.starts_with('#'))
        .collect::<Vec<_>>();
    assert_eq!(result.output, expected);
}
