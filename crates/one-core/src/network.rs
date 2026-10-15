use crate::capability::Capability;
use crate::diagnostic::{Diagnostic, DiagnosticBundle};

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
    pub fn validate(&self) -> Result<(), DiagnosticBundle> {
        let NetworkOperation::Connect { target, constraints } = self else { return Ok(()); };
        if constraints.protocol.is_none() { return Err(DiagnosticBundle::single(Diagnostic::error("N1001", "Permintaan jaringan belum menentukan keluarga protokol secara eksplisit.", None))); }
        if constraints.timeout_ms == 0 { return Err(DiagnosticBundle::single(Diagnostic::error("N1002", "Batas waktu koneksi harus lebih besar dari nol.", None))); }
        if matches!(constraints.protocol, Some(NetworkProtocolFamily::Tls)) && constraints.security.is_none() {
            return Err(DiagnosticBundle::single(Diagnostic::error("N1003", "Koneksi TLS harus menyatakan mode keamanan secara eksplisit.", None)));
        }
        let empty = match target {
            NetworkTarget::Host(value) | NetworkTarget::Address(value) => value.trim().is_empty(),
            NetworkTarget::Service { host, .. } => host.trim().is_empty(),
        };
        if empty { return Err(DiagnosticBundle::single(Diagnostic::error("N1004", "Tujuan jaringan tidak boleh kosong.", None))); }
        if let Some(protocol) = constraints.protocol {
            let valid_transport = match protocol {
                NetworkProtocolFamily::Tcp | NetworkProtocolFamily::Tls | NetworkProtocolFamily::Http | NetworkProtocolFamily::Quic => constraints.transport == NetworkTransport::Reliable,
                NetworkProtocolFamily::Udp => constraints.transport == NetworkTransport::Unreliable,
                NetworkProtocolFamily::NameResolution => true,
            };
            if !valid_transport { return Err(DiagnosticBundle::single(Diagnostic::error("N1005", "Transport yang dipilih bertentangan dengan keluarga protokol yang diminta.", None))); }
        }
        Ok(())
    }

    pub fn required_capabilities(&self) -> Vec<Capability> {
        match self {
            NetworkOperation::Connect { .. } | NetworkOperation::Send { .. } | NetworkOperation::Receive { .. } => vec![Capability::NetworkConnect],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolSupportStatus { Specified, Designed, Implemented, Verified, Supported }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolCapability {
    pub name: &'static str,
    pub family: NetworkProtocolFamily,
    pub status: ProtocolSupportStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolRegistry { entries: Vec<ProtocolCapability> }

impl ProtocolRegistry {
    pub fn builtin() -> Self {
        Self { entries: vec![
            ProtocolCapability { name: "DNS", family: NetworkProtocolFamily::NameResolution, status: ProtocolSupportStatus::Designed },
            ProtocolCapability { name: "TCP", family: NetworkProtocolFamily::Tcp, status: ProtocolSupportStatus::Designed },
            ProtocolCapability { name: "UDP", family: NetworkProtocolFamily::Udp, status: ProtocolSupportStatus::Designed },
            ProtocolCapability { name: "TLS", family: NetworkProtocolFamily::Tls, status: ProtocolSupportStatus::Designed },
            ProtocolCapability { name: "HTTP", family: NetworkProtocolFamily::Http, status: ProtocolSupportStatus::Designed },
            ProtocolCapability { name: "QUIC/HTTP3", family: NetworkProtocolFamily::Quic, status: ProtocolSupportStatus::Specified },
        ] }
    }

    pub fn entries(&self) -> &[ProtocolCapability] { &self.entries }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkDiagnosticStage { Validation, Capability, Adapter, Resolution, Connection, Identity }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkFailure {
    pub code: &'static str,
    pub stage: NetworkDiagnosticStage,
    pub explanation: String,
    pub technical_detail: String,
}

impl NetworkFailure {
    pub fn to_diagnostic(&self) -> Diagnostic {
        Diagnostic::error(self.code, self.explanation.clone(), None).with_note(format!("Tahap jaringan: {:?}.", self.stage)).with_note(format!("Detail teknis: {}", self.technical_detail))
    }
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
        let mut c = constraints(); c.protocol = None;
        let operation = NetworkOperation::Connect { target: NetworkTarget::Host("example.com".into()), constraints: c };
        let error = operation.validate().expect_err("missing protocol must be rejected");
        assert_eq!(error.0[0].code, "N1001");
    }

    #[test]
    fn invalid_timeout_is_rejected() {
        let mut c = constraints(); c.timeout_ms = 0;
        let operation = NetworkOperation::Connect { target: NetworkTarget::Host("example.com".into()), constraints: c };
        let error = operation.validate().expect_err("zero timeout must be rejected");
        assert_eq!(error.0[0].code, "N1002");
    }

    #[test]
    fn secure_connection_requires_explicit_security_mode_when_required() {
        let mut c = constraints(); c.security = None; c.protocol = Some(NetworkProtocolFamily::Tls);
        let operation = NetworkOperation::Connect { target: NetworkTarget::Host("example.com".into()), constraints: c };
        let error = operation.validate().expect_err("TLS must require explicit security mode");
        assert_eq!(error.0[0].code, "N1003");
    }
}

#[cfg(test)]
mod registry_tests {
    use super::*;

    #[test]
    fn registry_is_deterministically_ordered() {
        let names: Vec<_> = ProtocolRegistry::builtin().entries().iter().map(|entry| entry.name).collect();
        assert_eq!(names, vec!["DNS", "TCP", "UDP", "TLS", "HTTP", "QUIC/HTTP3"]);
    }

    #[test]
    fn registry_entries_do_not_define_language_actions() {
        assert!(ProtocolRegistry::builtin().entries().iter().all(|entry| !["tampilkan", "sapa", "buat", "jalankan"].contains(&entry.name)));
    }

    #[test]
    fn unsupported_protocol_is_not_reported_as_supported() {
        assert!(ProtocolRegistry::builtin().entries().iter().all(|entry| !matches!(entry.status, ProtocolSupportStatus::Supported)));
    }
}

#[cfg(test)]
mod diagnostic_tests {
    use super::*;

    #[test]
    fn network_failure_diagnostic_has_stable_layers() {
        let failure = NetworkFailure { code: "N3001", stage: NetworkDiagnosticStage::Connection, explanation: "Koneksi tidak dapat dibuat.".into(), technical_detail: "backend returned connection refused".into() };
        let diagnostic = failure.to_diagnostic();
        assert_eq!(diagnostic.code, "N3001");
        assert_eq!(diagnostic.message, "Koneksi tidak dapat dibuat.");
        assert_eq!(diagnostic.notes, vec!["Tahap jaringan: Connection.", "Detail teknis: backend returned connection refused"]);
    }
}
