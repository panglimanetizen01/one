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
    if !capabilities.permits(Capability::NetworkConnect) {
        return Err(NetworkRuntimeError::CapabilityDenied("Kemampuan jaringan NetworkConnect belum diberikan kepada program.".into()));
    }
    let host = host.to_owned();
    let payload = payload.to_vec();
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
    runtime.block_on(async move {
        let mut roots = RootCertStore::empty();
        roots.extend(TLS_SERVER_ROOTS.iter().cloned());
        let client_config = ClientConfig::with_root_certificates(Arc::new(roots)).map_err(|error| NetworkRuntimeError::Identity(error.to_string()))?;
        let mut endpoint = Endpoint::client("0.0.0.0:0".parse().map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?).map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
        endpoint.set_default_client_config(client_config);
        let connecting = endpoint.connect(address, &host).map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
        let connection = tokio::time::timeout(timeout, connecting).await.map_err(|_| NetworkRuntimeError::Connection("Batas waktu QUIC habis.".into()))?.map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
        let (mut send, mut recv) = tokio::time::timeout(timeout, connection.open_bi()).await.map_err(|_| NetworkRuntimeError::Connection("Pembukaan QUIC stream melewati batas waktu.".into()))?.map_err(|error| NetworkRuntimeError::Connection(error.to_string()))?;
        tokio::io::AsyncWriteExt::write_all(&mut send, &payload).await.map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
        send.finish().map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
        let response = tokio::time::timeout(timeout, recv.read_to_end(16 * 1024 * 1024)).await.map_err(|_| NetworkRuntimeError::Connection("Penerimaan QUIC melewati batas waktu.".into()))?.map_err(|error| NetworkRuntimeError::Io(error.to_string()))?;
        endpoint.close(0u32.into(), b"done");
        Ok(response)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quic_denial_happens_before_endpoint_creation() {
        let result = execute_quic("localhost", "127.0.0.1:4433".parse().unwrap(), b"ping", &CapabilitySet::none(), Duration::from_millis(50));
        assert!(matches!(result, Err(NetworkRuntimeError::CapabilityDenied(_))));
    }
}
