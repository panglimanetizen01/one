pub mod adapter;
pub mod ast;
pub mod capability;
pub mod diagnostic;
pub mod governance;
pub mod interop;
pub mod interpreter;
pub mod ir;
pub mod lexer;
pub mod network;
pub mod network_application;
pub mod network_datagrams;
pub mod network_dns_adapter;
pub mod network_mail;
pub mod network_runtime;
pub mod orchestration;
pub mod orchestration_audit;
pub mod orchestration_result;
pub mod orchestrator;
#[cfg(feature = "real-network")]
pub mod network_http2;
#[cfg(feature = "real-network")]
pub mod network_quic;
#[cfg(feature = "real-network")]
pub mod network_tls;
pub mod parser;
pub mod platform;
pub mod runtime;
pub mod semantic;
pub mod stdlib;
pub mod token;
pub mod types;

pub use adapter::{AdapterArgument, AdapterError, AdapterMetadata, AdapterRequest, AdapterResult, PlatformKind, ToolAdapter};
pub use ast::{BinaryOp, Expr, LoopKind, Program, Statement};
pub use capability::{Capability, CapabilitySet};
pub use diagnostic::{Diagnostic, DiagnosticBundle, Severity};
pub use governance::{DenyReason, ExecutionPolicy, PolicyDecision, ScopeRule};
pub use interop::{run_process, ProcessResult, ProcessSpec};
pub use interpreter::{ExecutionResult, Interpreter, NetworkExecutionResult, RuntimeError};
pub use ir::{lower, lower_network, IrExpr, IrLoopKind, IrNetworkOperation, IrProgram, IrStatement};
pub use lexer::lex;
pub use network::{NetworkAddressKind, NetworkConnectionId, NetworkConstraints, NetworkDiagnosticStage, NetworkFailure, NetworkOperation, NetworkProtocolFamily, NetworkSecurity, NetworkTarget, NetworkTransport, ProtocolCapability, ProtocolRegistry, ProtocolSupportStatus};
pub use network_application::{ftp_login, smtp_send, ApplicationProtocolError};
pub use network_datagrams::{ntp_query, tftp_read, tftp_write, DatagramProtocolError};
pub use network_mail::{ImapClient, MailProtocolError, Pop3Client};
pub use network_runtime::{execute_http11, parse_http_response, HttpRequest, HttpResponse, NetworkEndpoint, NetworkRuntimeError, NetworkRuntimeResult, RealNetworkRuntime};
#[cfg(feature = "real-network")]
pub use network_http2::execute_http2;
#[cfg(feature = "real-network")]
pub use network_quic::execute_quic;
#[cfg(feature = "real-network")]
pub use network_tls::execute_https11;
pub use network_dns_adapter::DnsResolveAdapter;
pub use orchestration::{
    resolve_candidates,
    resolve_capability_request,
    resolve_localized_alias,
    AdapterId,
    AdapterStatus,
    CapabilityRequest,
    CapabilityResolutionError,
    EffectClass,
    ExecutionScope,
    OrchestrationValidationError,
    ResolvedCapability,
    SemanticCapabilityId,
};
pub use orchestration_audit::{AuditEvent, AuditEvidence, AuditRecord};
pub use orchestration_result::{normalize_result, EvidenceCertainty, NormalizationExpectation, NormalizedResult, NormalizedValue, OrchestrationResultError};
pub use orchestrator::{OrchestrationError, Orchestrator};
pub use parser::parse;
pub use semantic::{analyze, analyze_network, TypedProgram};
pub use token::{Span, Token, TokenKind};
pub use types::Type;

#[cfg(test)]
mod tests {
    use super::*;

    fn compile(source: &str) -> IrProgram {
        let program = parse(source).expect("source should parse");
        let typed = analyze(&program).expect("source should analyze");
        lower(&typed).expect("source should lower")
    }

    #[test]
    fn core_pipeline_smoke_contract() {
        let ir = compile("nama adalah \"Budi\"\ntampilkan nama\n");
        let mut interpreter = Interpreter::new(CapabilitySet::none());
        let result = interpreter.execute(&ir).expect("source should execute");
        assert_eq!(result.output, vec!["Budi"]);
    }
}
