#![allow(clippy::possible_missing_else)]

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::time::Duration;

use crate::capability::CapabilitySet;
use crate::network::{NetworkAddressKind, NetworkConnectionId, NetworkOperation, NetworkTarget, NetworkTransport};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkRuntimeError { InvalidOperation(String), CapabilityDenied(String), Resolution(String), Connection(String), Io(String), Protocol(String), Identity(String), Unsupported(String) }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkEndpoint { pub requested: String, pub resolved: SocketAddr }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkRuntimeResult { Connected { id: NetworkConnectionId, endpoint: NetworkEndpoint }, Sent { id: NetworkConnectionId, bytes: usize }, Received { id: NetworkConnectionId, data: Vec<u8> } }
enum Connection { Tcp(TcpStream), Udp(UdpSocket) }
pub struct RealNetworkRuntime { next_id: u64, connections: std::collections::BTreeMap<NetworkConnectionId, Connection> }
impl Default for RealNetworkRuntime { fn default() -> Self { Self::new() } }
impl RealNetworkRuntime {
    pub fn new() -> Self { Self { next_id: 1, connections: std::collections::BTreeMap::new() } }
    pub fn execute(&mut self, operation: NetworkOperation, capabilities: &CapabilitySet) -> Result<NetworkRuntimeResult, NetworkRuntimeError> { for capability in operation.required_capabilities() { if !capabilities.permits(capability) { return Err(NetworkRuntimeError::CapabilityDenied(format!("Kemampuan jaringan {:?} belum diberikan kepada program.", capability))); } } operation.validate().map_err(|bundle| NetworkRuntimeError::InvalidOperation(bundle.0.first().map(|d| d.message.clone()).unwrap_or_else(|| "Operasi jaringan tidak valid.".into())))?; match operation { NetworkOperation::Connect { target, constraints } => self.connect(target, constraints), NetworkOperation::Send { connection, data } => self.send(connection, &data), NetworkOperation::Receive { connection } => self.receive(connection) } }
    fn connect(&mut self, target: NetworkTarget, constraints: crate::network::NetworkConstraints) -> Result<NetworkRuntimeResult, NetworkRuntimeError> { let (host, port) = match target { NetworkTarget::Service { host, port } => (host, port), NetworkTarget::Host(host) => (host, 0), NetworkTarget::Address(address) => (address, 0) }; let timeout = Duration::from_millis(constraints.timeout_ms); let resolved = resolve_target(&host, port, constraints.address_kind)?; let id = NetworkConnectionId(self.next_id); self.next_id += 1; match constraints.transport { NetworkTransport::Reliable => { let stream = TcpStream::connect_timeout(&resolved, timeout).map_err(|e| NetworkRuntimeError::Connection(e.to_string()))?; stream.set_read_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; stream.set_write_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; self.connections.insert(id, Connection::Tcp(stream)); }, NetworkTransport::Unreliable => { let socket = UdpSocket::bind(match resolved { SocketAddr::V4(_) => "0.0.0.0:0", SocketAddr::V6(_) => "[::]:0" }).map_err(|e| NetworkRuntimeError::Connection(e.to_string()))?; socket.connect(resolved).map_err(|e| NetworkRuntimeError::Connection(e.to_string()))?; socket.set_read_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; socket.set_write_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; self.connections.insert(id, Connection::Udp(socket)); } } Ok(NetworkRuntimeResult::Connected { id, endpoint: NetworkEndpoint { requested: host, resolved } }) }
    fn send(&mut self, id: NetworkConnectionId, data: &[u8]) -> Result<NetworkRuntimeResult, NetworkRuntimeError> { let connection = self.connections.get_mut(&id).ok_or_else(|| NetworkRuntimeError::Connection(format!("Koneksi {} tidak ditemukan.", id.0)))?; let bytes = match connection { Connection::Tcp(stream) => stream.write(data).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?, Connection::Udp(socket) => socket.send(data).map_err(|e| NetworkRuntimeError::Io(e.to_string()))? }; Ok(NetworkRuntimeResult::Sent { id, bytes }) }
    fn receive(&mut self, id: NetworkConnectionId) -> Result<NetworkRuntimeResult, NetworkRuntimeError> { let connection = self.connections.get_mut(&id).ok_or_else(|| NetworkRuntimeError::Connection(format!("Koneksi {} tidak ditemukan.", id.0)))?; let mut buffer = vec![0u8; 64 * 1024]; let size = match connection { Connection::Tcp(stream) => stream.read(&mut buffer).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?, Connection::Udp(socket) => socket.recv(&mut buffer).map_err(|e| NetworkRuntimeError::Io(e.to_string()))? }; buffer.truncate(size); Ok(NetworkRuntimeResult::Received { id, data: buffer }) }
    pub fn close(&mut self, id: NetworkConnectionId) -> bool { self.connections.remove(&id).is_some() }
}
fn resolve_target(host: &str, port: u16, address_kind: Option<NetworkAddressKind>) -> Result<SocketAddr, NetworkRuntimeError> { let service = format!("{host}:{port}"); let mut addresses = service.to_socket_addrs().map_err(|e| NetworkRuntimeError::Resolution(e.to_string()))?; addresses.find(|a| match address_kind { Some(NetworkAddressKind::Ipv4) => a.is_ipv4(), Some(NetworkAddressKind::Ipv6) => a.is_ipv6(), Some(NetworkAddressKind::Name) | None => true }).ok_or_else(|| NetworkRuntimeError::Resolution(format!("Tidak ada alamat yang cocok untuk {host}."))) }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest { pub method: String, pub authority: String, pub path: String, pub headers: Vec<(String, String)>, pub body: Vec<u8> }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse { pub status: u16, pub reason: String, pub headers: Vec<(String, String)>, pub body: Vec<u8> }
impl HttpRequest { pub fn get(authority: impl Into<String>, path: impl Into<String>) -> Self { Self { method: "GET".into(), authority: authority.into(), path: path.into(), headers: Vec::new(), body: Vec::new() } } pub fn to_bytes(&self) -> Result<Vec<u8>, NetworkRuntimeError> { if self.method.is_empty() || self.path.is_empty() || self.path.contains('\r') || self.path.contains('\n') || self.authority.contains('\r') || self.authority.contains('\n') { return Err(NetworkRuntimeError::Protocol("Permintaan HTTP memiliki token awal yang tidak valid.".into())); } let mut bytes = format!("{} {} HTTP/1.1\r\nHost: {}\r\n", self.method, self.path, self.authority).into_bytes(); let mut has_length = false; for (name, value) in &self.headers { if name.eq_ignore_ascii_case("host") || name.contains('\r') || name.contains('\n') || value.contains('\r') || value.contains('\n') { return Err(NetworkRuntimeError::Protocol("Header HTTP tidak valid.".into())); } if name.eq_ignore_ascii_case("content-length") { has_length = true; } bytes.extend_from_slice(format!("{}: {}\r\n", name, value).as_bytes()); } if !self.body.is_empty() && !has_length { bytes.extend_from_slice(format!("Content-Length: {}\r\n", self.body.len()).as_bytes()); } bytes.extend_from_slice(b"Connection: close\r\n\r\n"); bytes.extend_from_slice(&self.body); Ok(bytes) } }
const MAX_HTTP_RESPONSE_HEADER_BYTES: usize = 64 * 1024;
const MAX_HTTP_RESPONSE_BODY_BYTES: usize = 16 * 1024 * 1024;
const MAX_HTTP_RESPONSE_WIRE_BODY_BYTES: usize = 32 * 1024 * 1024;
const MAX_HTTP_RESPONSE_BYTES: usize = MAX_HTTP_RESPONSE_HEADER_BYTES + MAX_HTTP_RESPONSE_WIRE_BODY_BYTES;

fn read_http_response_bounded(stream: &mut TcpStream) -> Result<Vec<u8>, NetworkRuntimeError> {
    let mut response = Vec::new();
    let mut buffer = [0u8; 16 * 1024];
    loop {
        let count = match stream.read(&mut buffer) {
            Ok(count) => count,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(NetworkRuntimeError::Io(error.to_string())),
        };
        if count == 0 { break; }
        let next_len = response.len().checked_add(count)
            .ok_or_else(|| NetworkRuntimeError::Protocol("Respons HTTP melebihi batas ukuran.".into()))?;
        if next_len > MAX_HTTP_RESPONSE_BYTES {
            return Err(NetworkRuntimeError::Protocol("Respons HTTP melebihi batas ukuran wire.".into()));
        }
        response.try_reserve(count)
            .map_err(|_| NetworkRuntimeError::Io("Memori tidak cukup untuk membaca respons HTTP.".into()))?;
        response.extend_from_slice(&buffer[..count]);
        let marker = b"\r\n\r\n";
        if let Some(header_end) = response.windows(marker.len()).position(|window| window == marker) {
            if header_end > MAX_HTTP_RESPONSE_HEADER_BYTES {
                return Err(NetworkRuntimeError::Protocol("Header HTTP melebihi batas ukuran.".into()));
            }
            let body_start = header_end + marker.len();
            if response.len().saturating_sub(body_start) > MAX_HTTP_RESPONSE_WIRE_BODY_BYTES {
                return Err(NetworkRuntimeError::Protocol("HTTP wire body melebihi batas ukuran.".into()));
            }
        } else if response.len() > MAX_HTTP_RESPONSE_HEADER_BYTES {
            return Err(NetworkRuntimeError::Protocol("Header HTTP melebihi batas ukuran.".into()));
        }
    }
    Ok(response)
}

pub fn parse_http_response(mut bytes: &[u8], max_body: usize) -> Result<HttpResponse, NetworkRuntimeError> { let marker = b"\r\n\r\n"; let header_end = bytes.windows(marker.len()).position(|w| w == marker).ok_or_else(|| NetworkRuntimeError::Protocol("Respons HTTP tidak memiliki akhir header yang valid.".into()))?; if header_end > MAX_HTTP_RESPONSE_HEADER_BYTES { return Err(NetworkRuntimeError::Protocol("Header HTTP melebihi batas ukuran.".into())); } let header_bytes = &bytes[..header_end]; bytes = &bytes[header_end + marker.len()..]; let text = std::str::from_utf8(header_bytes).map_err(|_| NetworkRuntimeError::Protocol("Header HTTP bukan UTF-8 yang valid.".into()))?; let mut lines = text.split("\r\n"); let status_line = lines.next().ok_or_else(|| NetworkRuntimeError::Protocol("Respons HTTP kosong.".into()))?; let mut parts = status_line.splitn(3, ' '); if parts.next() != Some("HTTP/1.1") { return Err(NetworkRuntimeError::Protocol("Hanya HTTP/1.1 yang diterima pada adapter ini.".into())); } let status = parts.next().and_then(|v| v.parse::<u16>().ok()).ok_or_else(|| NetworkRuntimeError::Protocol("Status HTTP tidak valid.".into()))?; let reason = parts.next().unwrap_or_default().to_string(); let mut headers = Vec::new(); let mut content_length = None; let mut chunked = false; for line in lines { let (name, value) = line.split_once(':').ok_or_else(|| NetworkRuntimeError::Protocol("Header HTTP tidak memiliki pemisah ':'.".into()))?; let value = value.trim().to_string(); if name.eq_ignore_ascii_case("content-length") { if content_length.is_some() { return Err(NetworkRuntimeError::Protocol("Content-Length duplikat.".into())); } content_length = Some(value.parse::<usize>().map_err(|_| NetworkRuntimeError::Protocol("Content-Length tidak valid.".into()))?); } if name.eq_ignore_ascii_case("transfer-encoding") { if chunked || !value.eq_ignore_ascii_case("chunked") { return Err(NetworkRuntimeError::Protocol("Transfer-Encoding tidak didukung atau duplikat.".into())); } chunked = true; } headers.push((name.to_string(), value)); } if chunked && content_length.is_some() { return Err(NetworkRuntimeError::Protocol("Content-Length dan Transfer-Encoding tidak boleh digunakan bersamaan.".into())); } let body = if chunked { parse_chunked_body(bytes, max_body)? } else if let Some(length) = content_length { if length > max_body { return Err(NetworkRuntimeError::Protocol("HTTP body melebihi batas ukuran.".into())); } if bytes.len() < length { return Err(NetworkRuntimeError::Protocol("HTTP body terpotong.".into())); } bytes[..length].to_vec() } else { if bytes.len() > max_body { return Err(NetworkRuntimeError::Protocol("HTTP body melebihi batas ukuran.".into())); } bytes.to_vec() }; Ok(HttpResponse { status, reason, headers, body }) }
fn parse_chunked_body(mut bytes: &[u8], max_body: usize) -> Result<Vec<u8>, NetworkRuntimeError> { let mut output = Vec::new(); loop { let line_end = bytes.windows(2).position(|w| w == b"\r\n").ok_or_else(|| NetworkRuntimeError::Protocol("Ukuran HTTP chunk tidak lengkap.".into()))?; let size_text = std::str::from_utf8(&bytes[..line_end]).map_err(|_| NetworkRuntimeError::Protocol("Ukuran HTTP chunk tidak valid.".into()))?; let size = usize::from_str_radix(size_text.trim(), 16).map_err(|_| NetworkRuntimeError::Protocol("Ukuran HTTP chunk bukan heksadesimal.".into()))?; bytes = &bytes[line_end + 2..]; if size == 0 { return Ok(output); } let next_len = output.len().checked_add(size).filter(|length| *length <= max_body).ok_or_else(|| NetworkRuntimeError::Protocol("HTTP chunk melebihi batas ukuran.".into()))?; let framed_len = size.checked_add(2).ok_or_else(|| NetworkRuntimeError::Protocol("Ukuran HTTP chunk overflow.".into()))?; if bytes.len() < framed_len { return Err(NetworkRuntimeError::Protocol("HTTP chunk terpotong.".into())); } output.try_reserve(next_len - output.len()).map_err(|_| NetworkRuntimeError::Io("Memori tidak cukup untuk HTTP chunk.".into()))?; output.extend_from_slice(&bytes[..size]); if &bytes[size..size + 2] != b"\r\n" { return Err(NetworkRuntimeError::Protocol("HTTP chunk tidak diakhiri CRLF.".into())); } bytes = &bytes[size + 2..]; } }
pub fn execute_http11(_runtime: &mut RealNetworkRuntime, request: HttpRequest, capabilities: &CapabilitySet, timeout: Duration) -> Result<HttpResponse, NetworkRuntimeError> { if !capabilities.permits(crate::capability::Capability::NetworkConnect) { return Err(NetworkRuntimeError::CapabilityDenied("Kemampuan jaringan NetworkConnect belum diberikan kepada program.".into())); } let host = request.authority.split(':').next().filter(|v| !v.is_empty()).ok_or_else(|| NetworkRuntimeError::InvalidOperation("Host HTTP kosong.".into()))?.to_string(); let port = request.authority.rsplit(':').next().and_then(|v| v.parse::<u16>().ok()).unwrap_or(80); let address = resolve_target(&host, port, None)?; let mut stream = TcpStream::connect_timeout(&address, timeout).map_err(|e| NetworkRuntimeError::Connection(e.to_string()))?; stream.set_read_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; stream.set_write_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; stream.write_all(&request.to_bytes()?).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; let response = read_http_response_bounded(&mut stream)?; parse_http_response(&response, MAX_HTTP_RESPONSE_BODY_BYTES) }
#[cfg(test)]
mod tests { use super::*; use crate::network::NetworkProtocolFamily; use std::io::{Read, Write}; use std::net::TcpListener; use std::thread;
    #[test] fn localhost_resolution_returns_a_real_socket_address() { let address = resolve_target("localhost", 80, None).expect("localhost must resolve"); assert!(address.ip().is_loopback()); }
    #[test] fn tcp_round_trip_uses_real_loopback_socket() { let listener = TcpListener::bind("127.0.0.1:0").expect("bind"); let address = listener.local_addr().expect("address"); let server = thread::spawn(move || { let (mut stream, _) = listener.accept().expect("accept"); let mut input = [0u8; 4]; stream.read_exact(&mut input).expect("read"); assert_eq!(&input, b"ping"); stream.write_all(b"pong").expect("write"); }); let mut runtime = RealNetworkRuntime::new(); let capabilities = CapabilitySet::none().allow(crate::capability::Capability::NetworkConnect); let connected = runtime.execute(NetworkOperation::Connect { target: NetworkTarget::Service { host: address.ip().to_string(), port: address.port() }, constraints: crate::network::NetworkConstraints { address_kind: Some(NetworkAddressKind::Ipv4), transport: NetworkTransport::Reliable, security: Some(crate::network::NetworkSecurity::Insecure), timeout_ms: 2_000, protocol: Some(NetworkProtocolFamily::Tcp) } }, &capabilities).expect("connect"); let id = match connected { NetworkRuntimeResult::Connected { id, .. } => id, _ => panic!("unexpected result") }; runtime.execute(NetworkOperation::Send { connection: id, data: b"ping".to_vec() }, &capabilities).expect("send"); let received = runtime.execute(NetworkOperation::Receive { connection: id }, &capabilities).expect("receive"); assert_eq!(received, NetworkRuntimeResult::Received { id, data: b"pong".to_vec() }); server.join().expect("server"); }
    #[test] fn capability_denial_happens_before_connection() { let mut runtime = RealNetworkRuntime::new(); let result = runtime.execute(NetworkOperation::Connect { target: NetworkTarget::Service { host: "127.0.0.1".into(), port: 9 }, constraints: crate::network::NetworkConstraints { address_kind: Some(NetworkAddressKind::Ipv4), transport: NetworkTransport::Reliable, security: Some(crate::network::NetworkSecurity::Insecure), timeout_ms: 50, protocol: Some(NetworkProtocolFamily::Tcp) } }, &CapabilitySet::none()); assert!(matches!(result, Err(NetworkRuntimeError::CapabilityDenied(_)))); }
    #[test] fn http_request_and_response_are_real_bytes() { let listener = TcpListener::bind("127.0.0.1:0").expect("bind"); let address = listener.local_addr().expect("address"); let server = thread::spawn(move || { let (mut stream, _) = listener.accept().expect("accept"); let mut request = Vec::new(); let mut buffer = [0u8; 1024]; loop { let read = stream.read(&mut buffer).expect("read"); if read == 0 { break; } request.extend_from_slice(&buffer[..read]); if request.windows(4).any(|w| w == b"\r\n\r\n") { break; } } assert!(request.starts_with(b"GET / HTTP/1.1\r\n")); stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello").expect("write"); }); let mut runtime = RealNetworkRuntime::new(); let capabilities = CapabilitySet::none().allow(crate::capability::Capability::NetworkConnect); let authority = format!("{}:{}", address.ip(), address.port()); let response = execute_http11(&mut runtime, HttpRequest::get(authority, "/"), &capabilities, Duration::from_secs(2)).expect("http"); assert_eq!(response.status, 200); assert_eq!(response.body, b"hello"); server.join().expect("server"); }
    #[test] fn chunked_http_body_is_decoded_without_guessing() { let response = parse_http_response(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n0\r\n\r\n", 100).expect("parse"); assert_eq!(response.body, b"hello"); }
    #[test]
    fn http_response_rejects_ambiguous_framing_and_duplicate_lengths() {
        let ambiguous = parse_http_response(b"HTTP/1.1 200 OK\\r\\nContent-Length: 5\\r\\nTransfer-Encoding: chunked\\r\\n\\r\\n0\\r\\n\\r\\n", 100);
        assert!(matches!(ambiguous, Err(NetworkRuntimeError::Protocol(_))));
        let duplicate = parse_http_response(b"HTTP/1.1 200 OK\\r\\nContent-Length: 1\\r\\nContent-Length: 1\\r\\n\\r\\nx", 100);
        assert!(matches!(duplicate, Err(NetworkRuntimeError::Protocol(_))));
    }

    #[test]
    fn http_response_rejects_headers_over_64_kib() {
        let mut bytes = b"HTTP/1.1 200 OK\\r\\nX-Fill: ".to_vec();
        bytes.extend(std::iter::repeat_n(b'a', MAX_HTTP_RESPONSE_HEADER_BYTES));
        bytes.extend_from_slice(b"\\r\\n\\r\\n");
        assert!(matches!(parse_http_response(&bytes, 100), Err(NetworkRuntimeError::Protocol(_))));
    }

    #[test]
    fn http_chunk_size_overflow_is_rejected() {
        let response = parse_http_response(b"HTTP/1.1 200 OK\\r\\nTransfer-Encoding: chunked\\r\\n\\r\\nFFFFFFFFFFFFFFFFFFFFFFFF\\r\\n", 100);
        assert!(matches!(response, Err(NetworkRuntimeError::Protocol(_))));
    }

    #[test]
    fn bounded_http_reader_stops_on_oversized_header() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let address = listener.local_addr().expect("address");
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            let mut oversized = vec![b'X'; MAX_HTTP_RESPONSE_HEADER_BYTES + 8192];
            oversized.extend_from_slice(b"\\r\\n\\r\\n");
            let _ = stream.write_all(&oversized);
        });
        let mut stream = TcpStream::connect(address).expect("connect");
        stream.set_read_timeout(Some(Duration::from_secs(2))).expect("timeout");
        assert!(matches!(read_http_response_bounded(&mut stream), Err(NetworkRuntimeError::Protocol(_))));
        server.join().expect("server");
    }

}
