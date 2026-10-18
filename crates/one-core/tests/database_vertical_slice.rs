use one_core::adapter::{AdapterArgument, AdapterRequest, AdapterResult, ToolAdapter};
use one_core::capability::{Capability, CapabilitySet};
use one_core::database::SqliteAdapter;
use one_core::governance::{ExecutionPolicy, ScopeRule};
use one_core::orchestration::{CapabilityRequest, EffectClass, ExecutionScope, SemanticCapabilityId};
use one_core::orchestration_result::{normalize_result, NormalizationExpectation, NormalizedValue};
use one_core::{analyze, lower, parse, Interpreter};

fn compile(source: &str) -> one_core::IrProgram {
    let program = parse(source).expect("source should parse");
    let typed = analyze(&program).expect("source should analyze");
    lower(&typed).expect("source should lower")
}

fn request(resource: &str) -> CapabilityRequest {
    CapabilityRequest::new(
        SemanticCapabilityId::DatabaseQuery,
        Capability::DatabaseQuery,
        ExecutionScope::resource(resource),
        EffectClass::Read,
    )
}

#[test]
fn sqlite_database_adapter_executes_parameterized_query() {
    let adapter = SqliteAdapter::new();
    let result = adapter
        .invoke(
            &AdapterRequest::new(request(":memory:"))
                .with_argument(AdapterArgument::Text("SELECT ?1".into()))
                .with_argument(AdapterArgument::Text("42".into())),
        )
        .expect("sqlite query should execute");
    assert!(result.success);
    assert_eq!(result.output, b"42\n");
}

#[test]
fn one_source_database_query_reaches_runtime_and_returns_normalized_result() {
    let ir = compile("kueri database \":memory:\" dengan \"SELECT ?1\" dan \"42\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::DatabaseQuery)
            .allow_scope(Capability::DatabaseQuery, ScopeRule::Resource(":memory:".into())),
    );
    let result = interpreter
        .execute(&ir)
        .expect("database orchestration should execute");
    assert_eq!(result.output, vec!["42"]);
}

#[test]
fn database_query_is_denied_without_explicit_capability() {
    let ir = compile("kueri database \":memory:\" dengan \"SELECT ?1\" dan \"42\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(ExecutionPolicy::default());
    let error = interpreter
        .execute(&ir)
        .expect_err("database query must be denied by default");
    assert_eq!(error.code, "N3209");
}

#[test]
fn database_query_is_denied_when_resource_is_out_of_scope() {
    let ir = compile("kueri database \":memory:\" dengan \"SELECT ?1\" dan \"42\"\n");
    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::DatabaseQuery)
            .allow_scope(Capability::DatabaseQuery, ScopeRule::Resource("other.db".into())),
    );
    let error = interpreter
        .execute(&ir)
        .expect_err("database resource must be denied out of scope");
    assert_eq!(error.code, "N3209");
}

#[test]
fn sqlite_adapter_reports_backend_failure_for_unavailable_database_path() {
    let missing = std::env::temp_dir()
        .join("one-database-missing-parent-for-test")
        .join("database.sqlite");
    let adapter = SqliteAdapter::new();
    let result = adapter.invoke(
        &AdapterRequest::new(request(&missing.display().to_string()))
            .with_argument(AdapterArgument::Text("SELECT 1".into())),
    );
    assert_eq!(result, Err(one_core::AdapterError::BackendFailure));
}

#[test]
fn database_result_normalization_is_deterministic() {
    let first = normalize_result(
        one_core::AdapterId::new("database.sqlite.query"),
        AdapterResult { success: true, output: b"42\n".to_vec() },
        NormalizationExpectation::Utf8Text,
    )
    .expect("normalization should succeed");
    let second = normalize_result(
        one_core::AdapterId::new("database.sqlite.query"),
        AdapterResult { success: true, output: b"42\n".to_vec() },
        NormalizationExpectation::Utf8Text,
    )
    .expect("normalization should succeed");
    assert_eq!(first, second);
    assert_eq!(first.value(), &NormalizedValue::Text("42\n".into()));
}

#[test]
fn database_query_requires_explicit_capability() {
    let capabilities = CapabilitySet::none();
    assert!(!capabilities.permits(Capability::DatabaseQuery));
}
