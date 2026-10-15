#![cfg(feature = "real-network")]

use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;

use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use webpki_roots::TLS_SERVER_ROOTS;

use crate::capability::{Capability, CapabilitySet};
use crate::network_runtime::{HttpRequest, HttpResponse, NetworkRuntimeError, parse_http_response};

fn client_config_with_roots(roots: RootCertStore) -> Result<Arc<ClientConfig>, NetworkRuntimeError> {
    Ok(Arc::new(ClientConfig::builder().with_root_certificates(roots).with_no_client_auth()))
}

fn client_config() -> Result<Arc<ClientConfig>, NetworkRuntimeError> {
    client_config_with_roots(RootCertStore::from_iter(TLS_SERVER_ROOTS.iter().cloned()))
}

fn execute_https11_with_config(request: HttpRequest, capabilities: &CapabilitySet, timeout: Duration, config: Arc<ClientConfig>) -> Result<HttpResponse, NetworkRuntimeError> {
    if !capabilities.permits(Capability::NetworkConnect) { return Err(NetworkRuntimeError::CapabilityDenied("Kemampuan jaringan NetworkConnect belum diberikan kepada program.".into())); }
    let host = request.authority.split(':').next().filter(|value| !value.is_empty()).ok_or_else(|| NetworkRuntimeError::InvalidOperation("Host HTTPS kosong.".into()))?.to_string();
    let port = request.authority.rsplit(':').next().and_then(|value| value.parse::<u16>().ok()).unwrap_or(443);
    let address = (host.as_str(), port).to_socket_addrs().map_err(|error| NetworkRuntimeError::Resolution(error.to_string()))?.next().ok_or_else(|| NetworkRuntimeError::Resolution("Nama host tidak memiliki alamat.".into()))?;
    let stream = TcpStream::connect_timeout(&address, timeout).map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
    stream.set_read_timeout(Some(timeout)).map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
    stream.set_write_timeout(Some(timeout)).map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
    let server_name = ServerName::try_from(host.clone()).map_err(|error| NetworkRuntimeError::Identity(error.to_string()))?;
    let connection = ClientConnection::new(config, server_name).map_err(|error| NetworkRuntimeError::Identity(error.to_string()))?;
    let mut tls = StreamOwned::new(connection, stream);
    tls.write_all(&request.to_bytes()?).map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
    let mut response = Vec::new(); tls.read_to_end(&mut response).map_err(|error| NetworkRuntimeError::Io(error.to_string()))?; parse_http_response(&response, 16 * 1024 * 1024)
}

pub fn execute_https11(request: HttpRequest, capabilities: &CapabilitySet, timeout: Duration) -> Result<HttpResponse, NetworkRuntimeError> {
    execute_https11_with_config(request, capabilities, timeout, client_config()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
    use rustls::ServerConfig;
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn client_config_has_certificate_validation_roots() { let config = client_config().expect("config"); assert!(config.alpn_protocols.is_empty()); }

    #[test]
    fn https_loopback_round_trip_uses_real_tls() {
        let certified = rcgen::generate_simple_self_signed(vec!["localhost".into()]).expect("certificate");
        let cert = CertificateDer::from(certified.cert.der().to_vec());
        let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(certified.signing_key.serialize_der()));
        let server_config = ServerConfig::builder().with_no_client_auth().with_single_cert(vec![cert.clone()], key).expect("server config");
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind"); let address = listener.local_addr().expect("address");
        let server = thread::spawn(move || {
            let (stream, _) = listener.accept().expect("accept"); let mut tls = StreamOwned::new(rustls::ServerConnection::new(Arc::new(server_config)).expect("connection"), stream);
            let mut request = Vec::new(); let mut buffer = [0u8; 1024];
            loop { let read = tls.read(&mut buffer).expect("read"); if read == 0 { break; } request.extend_from_slice(&buffer[..read]); if request.windows(4).any(|w| w == b"\r\n\r\n") { break; } }
            assert!(request.starts_with(b"GET / HTTP/1.1\r\n")); tls.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello").expect("write");
        });
        let mut roots = RootCertStore::empty(); roots.add(certified.cert.der().clone()).expect("root");
        let capabilities = CapabilitySet::none().allow(Capability::NetworkConnect); let authority = format!("localhost:{}", address.port());
        let response = execute_https11_with_config(HttpRequest::get(authority, "/"), &capabilities, Duration::from_secs(3), client_config_with_roots(roots).expect("client config")).expect("https");
        assert_eq!(response.status, 200); assert_eq!(response.body, b"hello"); server.join().expect("server");
    }
}
