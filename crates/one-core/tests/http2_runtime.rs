#![cfg(feature = "real-network")]

use std::time::Duration;

use one_core::{execute_http2, CapabilitySet, HttpResponse};

#[test]
fn http2_capability_gate_rejects_before_io() {
    let result = execute_http2(
        "127.0.0.1:1",
        "/",
        &CapabilitySet::none(),
        Duration::from_millis(50),
    );
    assert!(matches!(result, Err(one_core::NetworkRuntimeError::CapabilityDenied(_))));
}

#[test]
fn http2_loopback_round_trip_uses_real_h2_frames() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    listener.set_nonblocking(true).expect("nonblocking");
    let address = listener.local_addr().expect("address");
    let expected_authority = format!("{}:{}", address.ip(), address.port());
    let server_expected_authority = expected_authority.clone();

    let server = std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        runtime.block_on(async move {
            let stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        tokio::time::sleep(Duration::from_millis(5)).await;
                    }
                    Err(error) => panic!("accept: {error}"),
                }
            };
            let mut stream = stream;
            stream.set_nonblocking(true).expect("accepted stream nonblocking");
            let mut connection = h2::server::handshake(
                tokio::net::TcpStream::from_std(stream).expect("tcp"),
            )
            .await
            .expect("h2 handshake");
            if let Some(Ok((request, mut respond))) = connection.accept().await {
                assert_eq!(request.version(), http::Version::HTTP_2);
                assert_eq!(request.method(), "GET");
                assert_eq!(request.uri().path(), "/hello");
                assert_eq!(
                    request.uri().authority().map(|value| value.as_str()),
                    Some(server_expected_authority.as_str())
                );
                let response = http::Response::builder()
                    .status(200)
                    .body(())
                    .expect("response");
                let mut send = respond
                    .send_response(response, false)
                    .expect("response headers");
                send.send_data(bytes::Bytes::from_static(b"hello"), true)
                    .expect("response body");
            }
        });
    });

    let capabilities = CapabilitySet::none().allow(one_core::Capability::NetworkConnect);
    let response: HttpResponse = execute_http2(
        &expected_authority,
        "/hello",
        &capabilities,
        Duration::from_secs(2),
    )
    .expect("http2");

    assert_eq!(response.status, 200);
    assert_eq!(response.body, b"hello");
    server.join().expect("server");
}
