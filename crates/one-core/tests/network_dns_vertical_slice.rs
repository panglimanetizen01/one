use one_core::{analyze, lower, parse, Capability, ExecutionPolicy, Interpreter, ScopeRule};

fn compile(source: &str) -> one_core::IrProgram {
    let program = parse(source).expect("source should parse");
    let typed = analyze(&program).expect("source should analyze");
    lower(&typed).expect("source should lower")
}

#[test]
fn source_dns_discovery_resolves_hostname_with_explicit_capability_and_scope() {
    let ir = compile("cari alamat \"localhost\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::NetworkDiscover)
            .allow_scope(
                Capability::NetworkDiscover,
                ScopeRule::Target("localhost".into()),
            ),
    );

    let result = interpreter
        .execute(&ir)
        .expect("source DNS discovery should execute through its governed adapter");

    assert!(
        result.output.iter().any(|line| line.contains("127.0.0.1") || line.contains("::1")),
        "localhost should resolve to a loopback address: {:?}",
        result.output
    );
}

#[test]
fn source_dns_discovery_is_denied_without_explicit_capability() {
    let ir = compile("cari alamat \"localhost\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(ExecutionPolicy::default());

    let error = interpreter
        .execute(&ir)
        .expect_err("DNS discovery must be denied without a capability grant");

    assert_eq!(error.code, "N3209");
    assert!(error.message.contains("MissingCapability"));
}

#[test]
fn source_dns_discovery_is_denied_when_target_is_out_of_scope() {
    let ir = compile("cari alamat \"localhost\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::NetworkDiscover)
            .allow_scope(
                Capability::NetworkDiscover,
                ScopeRule::Target("127.0.0.1".into()),
            ),
    );

    let error = interpreter
        .execute(&ir)
        .expect_err("DNS discovery outside the granted target scope must be denied");

    assert_eq!(error.code, "N3209");
    assert!(error.message.contains("OutOfScope"));
}
