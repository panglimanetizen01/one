use one_core::{analyze, lower, parse, Capability, ExecutionPolicy, Interpreter, ScopeRule};

fn compile(source: &str) -> one_core::IrProgram {
    let program = parse(source).expect("source should parse");
    let typed = analyze(&program).expect("source should analyze");
    lower(&typed).expect("source should lower")
}

#[test]
fn one_source_process_operation_reaches_runtime_and_returns_process_output() {
    let ir = compile("jalankan proses \"rustc\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::Process)
            .allow_scope(Capability::Process, ScopeRule::Target("rustc".into())),
    );
    let result = interpreter.execute(&ir).expect("process orchestration should execute");
    assert!(result.output.iter().any(|line| line.contains("rustc")));
}

#[test]
fn one_source_process_operation_is_denied_without_explicit_process_grant() {
    let ir = compile("jalankan proses \"rustc\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(ExecutionPolicy::default());
    let error = interpreter.execute(&ir).expect_err("process orchestration must be denied by default");
    assert_eq!(error.code, "N3209");
}
