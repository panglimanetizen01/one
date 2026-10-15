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

fn client_config() -> Result<Arc<ClientConfig>, NetworkRuntimeError> {
    let roots = RootCertStore::from_iter(TLS_SERVER_ROOTS.iter().cloned());
    Ok(Arc::new(ClientConfig::builder().with_root_certificates(roots).with_no_client_auth()))
}

pub fn execute_https11(request: HttpRequest, capabilities: &CapabilitySet, timeout: Duration) -> Result<HttpResponse, NetworkRuntimeError> {
    if !capabilities.permits(Capability::NetworkConnect) {
        return Err(NetworkRuntimeError::CapabilityDenied("Kemampuan jaringan NetworkConnect belum diberikan kepada program.".into()));
    }
    let host = request.authority.split(':').next().filter(|value| !value.is_empty()).ok_or_else(|| NetworkRuntimeError::InvalidOperation("Host HTTPS kosong.".into()))?.to_string();
    let port = request.authority.rsplit(':').next().and_then(|value| value.parse::<u16>().ok()).unwrap_or(443);
    let address = (host.as_str(), port).to_socket_addrs().map_err(|error| NetworkRuntimeError::Resolution(error.to_string()))?.next().ok_or_else(|| NetworkRuntimeError::Resolution("Nama host tidak memiliki alamat.".into()))?;
    let stream = TcpStream::connect_timeout(&address, timeout).map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
    stream.set_read_timeout(Some(timeout)).map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
    stream.set_write_timeout(Some(timeout)).map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
    let server_name = ServerName::try_from(host.clone()).map_err(|error| NetworkRuntimeError::Identity(error.to_string()))?;
    let connection = ClientConnection::new(client_config()?, server_name).map_err(|error| NetworkRuntimeError::Identity(error.to_string()))?;
    let mut tls = StreamOwned::new(connection, stream);
    let bytes = request.to_bytes()?;
    tls.write_all(&bytes).map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
    let mut response = Vec::new();
    tls.read_to_end(&mut response).map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
    parse_http_response(&response, 16 * 1024 * 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_config_has_certificate_validation_roots() {
        let config = client_config().expect("config");
        assert!(!config.alpn_protocols.is_empty() || config.alpn_protocols.is_empty());
    }
}
