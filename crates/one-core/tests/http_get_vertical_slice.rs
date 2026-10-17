use one_core::{lower, parse, Capability, ExecutionPolicy, Interpreter, IrStatement, ScopeRule, SemanticCapabilityId};

#[test]
fn source_http_get_reaches_real_http_adapter_with_target_scoped_network_connect() {
    let source = "ambil dari \"http://127.0.0.1:18080/\"\n";
    let program = parse(source).expect("HTTP source should parse");
    let typed = one_core::analyze(&program).expect("HTTP source should analyze");
    let ir = lower(&typed).expect("HTTP source should lower");

    assert!(matches!(
        ir.statements.first(),
        Some(IrStatement::Orchestration {
            semantic: SemanticCapabilityId::HttpGet,
            capability: Capability::NetworkConnect,
            target: one_core::IrExpr::Text(target),
            adapter,
            ..
        }) if target == "http://127.0.0.1:18080/" && adapter == "network.http.get"
    ));

    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .allow_scope(
                Capability::NetworkConnect,
                ScopeRule::Target("http://127.0.0.1:18080/".into()),
            ),
    );
    let result = interpreter
        .execute(&ir)
        .expect("HTTP source should execute through the governed adapter");
    assert_eq!(result.output, vec!["hello from One HTTP"]);
}
