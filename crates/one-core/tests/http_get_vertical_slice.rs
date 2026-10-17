use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use one_core::{lower, parse, Capability, ExecutionPolicy, Interpreter, IrExpr, IrStatement, ScopeRule, SemanticCapabilityId};

#[test]
fn source_http_get_reaches_real_http_adapter_with_target_scoped_network_connect() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("local HTTP listener should bind");
    let address = listener.local_addr().expect("listener address should exist");
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("HTTP client should connect");
        let mut request = Vec::new();
        let mut buffer = [0u8; 1024];
        loop {
            let read = stream.read(&mut buffer).expect("HTTP request should be readable");
            if read == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..read]);
            if request.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        assert!(request.starts_with(b"GET / HTTP/1.1\r\n"));
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 18\r\n\r\nhello from One HTTP")
            .expect("HTTP response should be writable");
    });

    let target = format!("http://{}:{}/", address.ip(), address.port());
    let source = format!("ambil dari \"{target}\"\n");
    let program = parse(&source).expect("HTTP source should parse");
    let typed = one_core::analyze(&program).expect("HTTP source should analyze");
    let ir = lower(&typed).expect("HTTP source should lower");

    assert!(matches!(
        ir.statements.first(),
        Some(IrStatement::Orchestration {
            semantic: SemanticCapabilityId::NetworkConnect,
            capability: Capability::NetworkConnect,
            target: IrExpr::Text(value),
            adapter,
            ..
        }) if value == &target && adapter == "network.http.get"
    ));

    let mut interpreter = Interpreter::with_orchestration_policy(
        ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Target(target)),
    );
    let result = interpreter
        .execute(&ir)
        .expect("HTTP source should execute through the governed adapter");
    assert_eq!(result.output, vec!["hello from One HTTP"]);
    server.join().expect("HTTP server should finish cleanly");
}
