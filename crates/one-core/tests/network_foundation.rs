use one_core::{Capability, CapabilitySet, IrNetworkOperation, NetworkAddressKind, NetworkConstraints, NetworkOperation, NetworkProtocolFamily, NetworkSecurity, NetworkTarget, NetworkTransport};

fn operation() -> NetworkOperation {
    NetworkOperation::Connect {
        target: NetworkTarget::Service { host: "example.com".into(), port: 443 },
        constraints: NetworkConstraints {
            address_kind: Some(NetworkAddressKind::Ipv6),
            transport: NetworkTransport::Reliable,
            security: Some(NetworkSecurity::Tls),
            timeout_ms: 5_000,
            protocol: Some(NetworkProtocolFamily::Http),
        },
    }
}

#[test]
fn capability_denial_precedes_backend() {
    let mut interpreter = one_core::Interpreter::new(CapabilitySet::none());
    let result = interpreter.execute_network(&IrNetworkOperation::semantic(operation()));
    let error = result.expect_err("network capability must be required");
    assert_eq!(error.code, "N3101");
}

#[test]
fn network_operation_with_granted_capability_reaches_reference_boundary() {
    let capabilities = CapabilitySet::none().allow(Capability::NetworkConnect);
    let mut interpreter = one_core::Interpreter::new(capabilities);
    let result = interpreter.execute_network(&IrNetworkOperation::semantic(operation())).expect("granted capability should reach reference boundary");
    assert_eq!(result.description, "network operation delegated to reference backend");
}

#[test]
fn reference_interpreter_does_not_perform_external_network_io() {
    let capabilities = CapabilitySet::none().allow(Capability::NetworkConnect);
    let mut interpreter = one_core::Interpreter::new(capabilities);
    let result = interpreter.execute_network(&IrNetworkOperation::semantic(operation())).expect("reference execution should be side-effect free");
    assert!(!result.performed_external_io);
}
