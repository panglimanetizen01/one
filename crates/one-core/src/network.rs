use crate::capability::Capability;
use crate::diagnostic::DiagnosticBundle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkAddressKind { Ipv4, Ipv6, Name }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkTransport { Reliable, Unreliable }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkSecurity { Insecure, Tls }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkProtocolFamily { NameResolution, Tcp, Udp, Tls, Http, Quic }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkTarget { Host(String), Address(String), Service { host: String, port: u16 } }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetworkConnectionId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkConstraints {
    pub address_kind: Option<NetworkAddressKind>,
    pub transport: NetworkTransport,
    pub security: Option<NetworkSecurity>,
    pub timeout_ms: u64,
    pub protocol: Option<NetworkProtocolFamily>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkOperation {
    Connect { target: NetworkTarget, constraints: NetworkConstraints },
    Send { connection: NetworkConnectionId, data: Vec<u8> },
    Receive { connection: NetworkConnectionId },
}

impl NetworkOperation {
    pub fn validate(&self) -> Result<(), DiagnosticBundle> { todo!() }
    pub fn required_capabilities(&self) -> Vec<Capability> { todo!() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn constraints() -> NetworkConstraints {
        NetworkConstraints { address_kind: Some(NetworkAddressKind::Ipv6), transport: NetworkTransport::Reliable, security: Some(NetworkSecurity::Tls), timeout_ms: 5_000, protocol: Some(NetworkProtocolFamily::Http) }
    }

    #[test]
    fn typed_constraints_round_trip_without_loss() {
        let operation = NetworkOperation::Connect { target: NetworkTarget::Service { host: "example.com".into(), port: 443 }, constraints: constraints() };
        assert_eq!(operation, NetworkOperation::Connect { target: NetworkTarget::Service { host: "example.com".into(), port: 443 }, constraints: constraints() });
    }

    #[test]
    fn ambiguous_operation_is_rejected() {
        let mut c = constraints();
        c.protocol = None;
        let operation = NetworkOperation::Connect { target: NetworkTarget::Host("example.com".into()), constraints: c };
        let error = operation.validate().expect_err("missing protocol must be rejected");
        assert_eq!(error.0[0].code, "N1001");
    }

    #[test]
    fn invalid_timeout_is_rejected() {
        let mut c = constraints();
        c.timeout_ms = 0;
        let operation = NetworkOperation::Connect { target: NetworkTarget::Host("example.com".into()), constraints: c };
        let error = operation.validate().expect_err("zero timeout must be rejected");
        assert_eq!(error.0[0].code, "N1002");
    }

    #[test]
    fn secure_connection_requires_explicit_security_mode_when_required() {
        let mut c = constraints();
        c.security = None;
        c.protocol = Some(NetworkProtocolFamily::Tls);
        let operation = NetworkOperation::Connect { target: NetworkTarget::Host("example.com".into()), constraints: c };
        let error = operation.validate().expect_err("TLS must require explicit security mode");
        assert_eq!(error.0[0].code, "N1003");
    }
}

#[cfg(test)]
mod registry_tests {
    #[test]
    fn registry_is_deterministically_ordered() { todo!() }
    #[test]
    fn registry_entries_do_not_define_language_actions() { todo!() }
    #[test]
    fn unsupported_protocol_is_not_reported_as_supported() { todo!() }
}

#[cfg(test)]
mod diagnostic_tests {
    #[test]
    fn network_failure_diagnostic_has_stable_layers() { todo!() }
}
