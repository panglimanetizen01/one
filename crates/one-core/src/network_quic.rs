#![cfg(feature = "real-network")]

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use quinn::{ClientConfig, Endpoint};
use rustls::RootCertStore;
use webpki_roots::TLS_SERVER_ROOTS;

use crate::capability::{Capability, CapabilitySet};
use crate::network_runtime::NetworkRuntimeError;

pub fn execute_quic(host: &str, address: SocketAddr, payload: &[u8], capabilities: &CapabilitySet, timeout: Duration) -> Result<Vec<u8>, NetworkRuntimeError> {
    if !capabilities.permits(Capability::NetworkConnect) { return Err(NetworkRuntimeError::CapabilityDenied("Kemampuan jaringan NetworkConnect belum diberikan kepada program.".into())); }
    let host = host.to_owned(); let payload = payload.to_vec();
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
    runtime.block_on(async move {
        let mut roots = RootCertStore::empty(); roots.extend(TLS_SERVER_ROOTS.iter().cloned());
        let client_config = ClientConfig::with_root_certificates(Arc::new(roots)).map_err(|error| NetworkRuntimeError::Identity(error.to_string()))?;
        let bind_addr: SocketAddr = "0.0.0.0:0".parse().map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
        let mut endpoint = Endpoint::client(bind_addr).map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
        endpoint.set_default_client_config(client_config);
        let connecting = endpoint.connect(address, &host).map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
        let connection = tokio::time::timeout(timeout, connecting).await.map_err(|_| NetworkRuntimeError::Connection("Batas waktu QUIC habis.".into()))?.map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
        let (mut send, mut recv) = tokio::time::timeout(timeout, connection.open_bi()).await.map_err(|_| NetworkRuntimeError::Connection("Pembukaan QUIC stream melewati batas waktu.".into()))?.map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
        tokio::io::AsyncWriteExt::write_all(&mut send, &payload).await.map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
        send.finish().map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
        let response = tokio::time::timeout(timeout, recv.read_to_end(16 * 1024 * 1024)).await.map_err(|_| NetworkRuntimeError::Connection("Penerimaan QUIC melewati batas waktu.".into()))?.map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
        endpoint.close(0u32.into(), b"done"); Ok(response)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

    #[test]
    fn quic_denial_happens_before_endpoint_creation() { let result = execute_quic("localhost", "127.0.0.1:4433".parse().unwrap(), b"ping", &CapabilitySet::none(), Duration::from_millis(50)); assert!(matches!(result, Err(NetworkRuntimeError::CapabilityDenied(_)))); }

    #[test]
    fn quic_loopback_round_trip_is_real_transport() {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("runtime");
        runtime.block_on(async {
            let certified = rcgen::generate_simple_self_signed(vec!["localhost".into()]).expect("certificate");
            let cert = CertificateDer::from(certified.cert.der().to_vec());
            let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(certified.signing_key.serialize_der()));
            let server_config = quinn::ServerConfig::with_single_cert(vec![cert.clone()], key).expect("server config");
            let mut server = Endpoint::server(server_config, "127.0.0.1:0".parse().unwrap()).expect("server"); let server_addr = server.local_addr().expect("server address");
            let mut roots = RootCertStore::empty(); roots.add(cert).expect("root");
            let client_config = ClientConfig::with_root_certificates(Arc::new(roots)).expect("client config");
            let mut client = Endpoint::client("127.0.0.1:0".parse().unwrap()).expect("client"); client.set_default_client_config(client_config);
            let server_task = tokio::spawn(async move {
                let incoming = server.accept().await.expect("incoming"); let connection = incoming.await.expect("connection"); let (mut send, mut recv) = connection.accept_bi().await.expect("stream"); let request = recv.read_to_end(1024).await.expect("request"); tokio::io::AsyncWriteExt::write_all(&mut send, &request).await.expect("response"); send.finish().expect("finish"); server.close(0u32.into(), b"done");
            });
            let connection = client.connect(server_addr, "localhost").expect("connect").await.expect("handshake"); let (mut send, mut recv) = connection.open_bi().await.expect("stream"); tokio::io::AsyncWriteExt::write_all(&mut send, b"ping").await.expect("send"); send.finish().expect("finish"); let response = recv.read_to_end(1024).await.expect("receive"); assert_eq!(response, b"ping"); client.close(0u32.into(), b"done"); server_task.await.expect("server task");
        });
    }
}
