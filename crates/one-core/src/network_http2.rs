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
    let headers = response
        .headers()
        .iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                value.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect::<Vec<_>>();

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
