pub mod adapter;
pub mod ast;
pub mod capability;
pub mod database;
pub mod diagnostic;
pub mod governance;
pub mod interop;
pub mod interpreter;
/// Experimental, isolated parser for the proposed human-instruction surface.
pub mod human_vnext;
/// Human-language source adapter compiled through the canonical AST, semantic analyzer and IR.
pub mod human_compiler;
pub mod ir;
pub mod lexer;
pub mod native_backend;
pub mod network;
pub mod network_application;
pub mod network_datagrams;
pub mod network_dns_adapter;
#[cfg(feature = "real-network")]
pub mod network_http2;
pub mod network_mail;
#[cfg(feature = "real-network")]
pub mod network_quic;
pub mod network_runtime;
#[cfg(feature = "real-network")]
pub mod network_tls;
pub mod orchestration;
pub mod orchestration_audit;
pub mod orchestration_result;
pub mod orchestrator;
pub mod parser;
pub mod persistence;
pub mod platform;
pub mod runtime;
mod resource_budget;
pub mod semantic;
pub mod stdlib;
pub mod target;
pub mod token;
pub mod types;

pub use adapter::{
    AdapterArgument, AdapterError, AdapterMetadata, AdapterRequest, AdapterResult, FileReadAdapter,
    FileWriteAdapter, HttpGetAdapter, PlatformKind, ProcessAdapter, ToolAdapter,
    write_file_within_project,
};
pub use ast::{BinaryOp, Expr, LoopKind, Program, Statement};
pub use capability::{Capability, CapabilitySet};
pub use database::SqliteAdapter;
pub use diagnostic::{Diagnostic, DiagnosticBundle, Severity};
pub use governance::{DenyReason, ExecutionPolicy, PolicyDecision, ScopeRule};
pub use interop::{ProcessResult, ProcessSpec, run_process};
pub use interpreter::{ExecutionResult, Interpreter, NetworkExecutionResult, RuntimeError};
pub use ir::{
    IrExpr, IrLoopKind, IrNetworkOperation, IrProgram, IrStatement, lower, lower_network,
};
pub use lexer::lex;
pub use native_backend::{
    NativeAndroidAarch64Backend, NativeArtifact, NativeBackendError, NativeLinuxAarch64Backend,
    NativeLinuxX86_64Backend, NativeWindowsX86_64Backend,
};
pub use network::{
    NetworkAddressKind, NetworkConnectionId, NetworkConstraints, NetworkDiagnosticStage,
    NetworkFailure, NetworkOperation, NetworkProtocolFamily, NetworkSecurity, NetworkTarget,
    NetworkTransport, ProtocolCapability, ProtocolRegistry, ProtocolSupportStatus,
};
pub use network_application::{ApplicationProtocolError, ftp_login, smtp_send};
pub use network_datagrams::{DatagramProtocolError, ntp_query, tftp_read, tftp_write};
pub use network_dns_adapter::DnsResolveAdapter;
#[cfg(feature = "real-network")]
pub use network_http2::execute_http2;
pub use network_mail::{ImapClient, MailProtocolError, Pop3Client};
#[cfg(feature = "real-network")]
pub use network_quic::execute_quic;
pub use network_runtime::{
    HttpRequest, HttpResponse, NetworkEndpoint, NetworkRuntimeError, NetworkRuntimeResult,
    RealNetworkRuntime, execute_http11, parse_http_response,
};
#[cfg(feature = "real-network")]
pub use network_tls::execute_https11;
pub use orchestration::{
    AdapterId, AdapterStatus, CapabilityRequest, CapabilityResolutionError, EffectClass,
    ExecutionScope, OrchestrationValidationError, ResolvedCapability, SemanticCapabilityId,
    resolve_candidates, resolve_capability_request, resolve_localized_alias,
};
pub use orchestration_audit::{AuditEvent, AuditEvidence, AuditRecord};
pub use orchestration_result::{
    EvidenceCertainty, NormalizationExpectation, NormalizedResult, NormalizedValue,
    OrchestrationResultError, normalize_result,
};
pub use orchestrator::{OrchestrationError, Orchestrator};
pub use parser::{parse, parse_human_core};
pub use semantic::{TypedProgram, analyze, analyze_network};
pub use target::{
    ArtifactFormat, ExecutionProfile, LinkerStrategy, ReferenceTargetBackend, ReferenceToolchain,
    RuntimeClass, TargetArchitecture, TargetBackend, TargetDefinition, TargetEnvironment,
    TargetSpec, TargetValidationError, ToolchainCapability, ToolchainOperation, ToolchainSpec,
    ToolchainTarget, ToolchainValidationError, target_definition,
};
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
    #[test]
    fn one_source_dns_operation_reaches_orchestration_runtime_and_returns_normalized_result() {
        let ir = compile("cari alamat \"127.0.0.1\"\n");
        let mut interpreter = Interpreter::with_orchestration_policy(
            ExecutionPolicy::default()
                .grant(Capability::NetworkDiscover)
                .allow_scope(
                    Capability::NetworkDiscover,
                    ScopeRule::Target("127.0.0.1".into()),
                ),
        );
        let result = interpreter
            .execute(&ir)
            .expect("source orchestration should execute");
        assert!(result.output.iter().any(|line| line.contains("127.0.0.1")));
    }
    #[test]
    fn one_source_dns_operation_is_denied_without_explicit_capability_grant() {
        let ir = compile("cari alamat \"127.0.0.1\"\n");
        let mut interpreter = Interpreter::with_orchestration_policy(ExecutionPolicy::default());
        let error = interpreter
            .execute(&ir)
            .expect_err("orchestration must be denied by default");
        assert_eq!(error.code, "N3209");
    }

    #[test]
    fn mapping_grammar_is_opt_in_and_does_not_change_v01_parser() {
        let source = "buat pemetaan profil\n";
        assert!(parse(source).is_err(), "v0.1 must not accept experimental mapping syntax");
        assert!(parse_human_core(source).is_ok(), "explicit human-core parser accepts mapping syntax");
    }

}
