#![cfg(feature = "real-network")]

use std::net::SocketAddr;
use std::time::Duration;

use crate::capability::{Capability, CapabilitySet};
use crate::network_runtime::{HttpResponse, NetworkRuntimeError, parse_http_authority, resolve_target};

const MAX_BODY: usize = 16 * 1024 * 1024;

pub fn execute_http2(
    authority: &str,
    path: &str,
    capabilities: &CapabilitySet,
    timeout: Duration,
) -> Result<HttpResponse, NetworkRuntimeError> {
    if !capabilities.permits(Capability::NetworkConnect) {
        return Err(NetworkRuntimeError::CapabilityDenied(
            "Kemampuan jaringan NetworkConnect belum diberikan kepada program.".into(),
        ));
    }
    if path.is_empty()
        || !(path.starts_with('/') || path == "*")
        || path.bytes().any(|byte| byte <= 0x20 || byte == 0x7f || byte == b'#')
    {
        return Err(NetworkRuntimeError::InvalidOperation(
            "Path HTTP/2 tidak valid.".into(),
        ));
    }

    let (host, port) = parse_http_authority(authority)?;
    let address = resolve_target(&host, port, None)?;
    let authority = authority.to_owned();
    let path = path.to_owned();

    let runtime = tokio::runtime::Runtime::new()
        .map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
    runtime.block_on(async move {
        tokio::time::timeout(timeout, execute_http2_async(address, authority, path))
            .await
            .map_err(|_| NetworkRuntimeError::Io("Waktu tunggu HTTP/2 habis.".into()))?
    })
}

const MAX_HTTP2_HEADER_COUNT: usize = 256;
const MAX_HTTP2_HEADER_BYTES: usize = 64 * 1024;

fn collect_bounded_headers(headers: &http::HeaderMap) -> Result<Vec<(String, String)>, NetworkRuntimeError> {
    if headers.len() > MAX_HTTP2_HEADER_COUNT {
        return Err(NetworkRuntimeError::Protocol("Jumlah header HTTP/2 melebihi batas.".into()));
    }
    let mut total_bytes = 0usize;
    let mut output = Vec::with_capacity(headers.len());
    for (name, value) in headers {
        let value = value.to_str()
            .map_err(|error| NetworkRuntimeError::Protocol(format!("Nilai header HTTP/2 bukan teks yang valid: {error}")))?;
        total_bytes = total_bytes
            .checked_add(name.as_str().len())
            .and_then(|total| total.checked_add(value.len()))
            .and_then(|total| total.checked_add(4))
            .ok_or_else(|| NetworkRuntimeError::Protocol("Ukuran header HTTP/2 melampaui batas integer.".into()))?;
        if total_bytes > MAX_HTTP2_HEADER_BYTES {
            return Err(NetworkRuntimeError::Protocol("Header HTTP/2 melebihi batas ukuran.".into()));
        }
        output.push((name.to_string(), value.to_owned()));
    }
    Ok(output)
}

async fn execute_http2_async(
    address: SocketAddr,
    authority: String,
    path: String,
) -> Result<HttpResponse, NetworkRuntimeError> {
    let stream = tokio::net::TcpStream::connect(address)
        .await
        .map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;

    let (mut sender, connection) = h2::client::Builder::new()
        .handshake::<_, bytes::Bytes>(stream)
        .await
        .map_err(|error| NetworkRuntimeError::Protocol(error.to_string()))?;
    tokio::spawn(async move {
        let _ = connection.await;
    });

    let request = http::Request::builder()
        .method("GET")
        .version(http::Version::HTTP_2)
        .uri(format!("http://{authority}{path}"))
        .header("host", &authority)
        .body(())
        .map_err(|error| NetworkRuntimeError::Protocol(error.to_string()))?;
    let (response_future, _send_stream) = sender
        .send_request(request, true)
        .map_err(|error| NetworkRuntimeError::Protocol(error.to_string()))?;
    let mut response = response_future
        .await
        .map_err(|error| NetworkRuntimeError::Protocol(error.to_string()))?;

    let status = response.status().as_u16();
    let headers = collect_bounded_headers(response.headers())?;

    let mut body = Vec::new();
    while let Some(chunk) = response.body_mut().data().await {
        let chunk = chunk.map_err(|error| NetworkRuntimeError::Protocol(error.to_string()))?;
        if body.len().saturating_add(chunk.len()) > MAX_BODY {
            return Err(NetworkRuntimeError::Protocol(
                "HTTP/2 body melebihi batas ukuran.".into(),
            ));
        }
        body.extend_from_slice(&chunk);
    }

    Ok(HttpResponse {
        status,
        reason: String::new(),
        headers,
        body,
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http2_rejects_malformed_authority_before_dns_or_socket_work() {
        let capabilities = CapabilitySet::none().allow(Capability::NetworkConnect);
        for authority in ["", "user@example.com:80", "example.com:", "example.com:abc", "example.com:65536", "2001:db8::1"] {
            assert!(
                execute_http2(authority, "/", &capabilities, Duration::from_millis(10)).is_err(),
                "authority should be rejected: {authority}"
            );
        }
    }

    #[test]
    fn http2_rejects_path_controls_spaces_and_fragments() {
        let capabilities = CapabilitySet::none().allow(Capability::NetworkConnect);
        for path in ["", "relative", "/has space", "/bad\rpath", "/bad\npath", "/path#fragment"] {
            assert!(
                execute_http2("127.0.0.1:80", path, &capabilities, Duration::from_millis(10)).is_err(),
                "path should be rejected: {path:?}"
            );
        }
    }

    #[test]
    fn http2_loopback_performs_real_request_response_over_h2() {
        use std::net::TcpListener;
        use std::thread;

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        listener.set_nonblocking(true).expect("nonblocking");
        let address = listener.local_addr().expect("address");
        let server = thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("runtime");
            runtime.block_on(async move {
                let listener = tokio::net::TcpListener::from_std(listener).expect("tokio listener");
                let (stream, _) = listener.accept().await.expect("accept");
                let mut connection = h2::server::handshake(stream).await.expect("h2 handshake");
                while let Some(result) = connection.accept().await {
                    let (request, respond) = result.expect("h2 request");
                    assert_eq!(request.uri().path(), "/probe");
                    tokio::spawn(async move {
                        let response = http::Response::builder()
                            .status(200)
                            .header("x-test", "yes")
                            .body(())
                            .expect("response");
                        let mut stream = respond.send_response(response, false).expect("send headers");
                        stream.send_data(bytes::Bytes::from_static(b"pong"), true).expect("send body");
                    });
                }
            });
        });

        let capabilities = CapabilitySet::none().allow(Capability::NetworkConnect);
        let response = execute_http2(
            &format!("127.0.0.1:{}", address.port()),
            "/probe",
            &capabilities,
            Duration::from_secs(3),
        ).expect("HTTP/2 round trip");
        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"pong");
        assert!(response.headers.iter().any(|(name, value)| name == "x-test" && value == "yes"));
        server.join().expect("server");
    }

    #[test]
    fn http2_response_headers_enforce_count_and_byte_budgets() {
        let mut headers = http::HeaderMap::new();
        for index in 0..257 {
            headers.insert(
                http::HeaderName::from_bytes(format!("x-{index}").as_bytes()).expect("header name"),
                http::HeaderValue::from_static("v"),
            );
        }
        assert!(collect_bounded_headers(&headers).is_err());

        let mut oversized = http::HeaderMap::new();
        oversized.insert(
            http::HeaderName::from_static("x-large"),
            http::HeaderValue::from_bytes(&vec![b'x'; MAX_HTTP2_HEADER_BYTES]).expect("header value"),
        );
        assert!(collect_bounded_headers(&oversized).is_err());
    }

    #[test]
    fn http2_requires_network_capability_before_target_processing() {
        assert!(matches!(
            execute_http2("bad authority", "bad path", &CapabilitySet::none(), Duration::from_millis(10)),
            Err(NetworkRuntimeError::CapabilityDenied(_))
        ));
    }
}
