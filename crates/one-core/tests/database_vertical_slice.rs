use one_core::adapter::{AdapterArgument, AdapterRequest, ToolAdapter};
use one_core::capability::{Capability, CapabilitySet};
use one_core::database::SqliteAdapter;
use one_core::orchestration::{CapabilityRequest, EffectClass, ExecutionScope, SemanticCapabilityId};

#[test]
fn sqlite_database_adapter_executes_parameterized_query() {
    let adapter = SqliteAdapter::new();
    let request = CapabilityRequest::new(
        SemanticCapabilityId::DatabaseQuery,
        Capability::DatabaseQuery,
        ExecutionScope::resource(":memory:"),
        EffectClass::Read,
    );
    let result = adapter
        .invoke(
            &AdapterRequest::new(request)
                .with_argument(AdapterArgument::Text("SELECT ?1".into()))
                .with_argument(AdapterArgument::Text("42".into())),
        )
        .expect("sqlite query should execute");
    assert!(result.success);
    assert_eq!(result.output, b"42\n");
}

#[test]
fn database_query_requires_explicit_capability() {
    let capabilities = CapabilitySet::none();
    assert!(!capabilities.permits(Capability::DatabaseQuery));
}
