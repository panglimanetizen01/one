#![allow(clippy::possible_missing_else)]

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::time::Duration;

use crate::capability::CapabilitySet;
use crate::network::{NetworkAddressKind, NetworkConnectionId, NetworkOperation, NetworkTarget, NetworkTransport};
use crate::resource_budget::ResourcePermit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkRuntimeError { InvalidOperation(String), CapabilityDenied(String), Resolution(String), Connection(String), Io(String), Protocol(String), Identity(String), Unsupported(String) }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkEndpoint { pub requested: String, pub resolved: SocketAddr }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkRuntimeResult { Connected { id: NetworkConnectionId, endpoint: NetworkEndpoint }, Sent { id: NetworkConnectionId, bytes: usize }, Received { id: NetworkConnectionId, data: Vec<u8> } }
enum Connection { Tcp { stream: TcpStream, _permit: ResourcePermit }, Udp { socket: UdpSocket, _permit: ResourcePermit } }
const MAX_NETWORK_CONNECTIONS: usize = 1024;
const MAX_NETWORK_PAYLOAD_BYTES: usize = 16 * 1024 * 1024;

fn ensure_connection_capacity(active: usize) -> Result<(), NetworkRuntimeError> {
    if active >= MAX_NETWORK_CONNECTIONS {
        Err(NetworkRuntimeError::InvalidOperation("Jumlah koneksi jaringan aktif melebihi batas.".into()))
    } else {
        Ok(())
    }
}

pub struct RealNetworkRuntime { next_id: u64, connections: std::collections::BTreeMap<NetworkConnectionId, Connection> }
impl Default for RealNetworkRuntime { fn default() -> Self { Self::new() } }
impl RealNetworkRuntime {
    pub fn new() -> Self { Self { next_id: 1, connections: std::collections::BTreeMap::new() } }
    pub fn execute(&mut self, operation: NetworkOperation, capabilities: &CapabilitySet) -> Result<NetworkRuntimeResult, NetworkRuntimeError> { for capability in operation.required_capabilities() { if !capabilities.permits(capability) { return Err(NetworkRuntimeError::CapabilityDenied(format!("Kemampuan jaringan {:?} belum diberikan kepada program.", capability))); } } operation.validate().map_err(|bundle| NetworkRuntimeError::InvalidOperation(bundle.0.first().map(|d| d.message.clone()).unwrap_or_else(|| "Operasi jaringan tidak valid.".into())))?; match operation { NetworkOperation::Connect { target, constraints } => self.connect(target, constraints), NetworkOperation::Send { connection, data } => self.send(connection, &data), NetworkOperation::Receive { connection } => self.receive(connection) } }
    fn connect(&mut self, target: NetworkTarget, constraints: crate::network::NetworkConstraints) -> Result<NetworkRuntimeResult, NetworkRuntimeError> { let (host, port) = match target { NetworkTarget::Service { host, port } => (host, port), NetworkTarget::Host(host) => (host, 0), NetworkTarget::Address(address) => (address, 0) }; let timeout = Duration::from_millis(constraints.timeout_ms); let permit = ResourcePermit::operation().ok_or_else(|| NetworkRuntimeError::InvalidOperation("Jumlah operasi jaringan aktif melampaui batas global.".into()))?; let resolved = resolve_target(&host, port, constraints.address_kind)?; ensure_connection_capacity(self.connections.len())?; let id = NetworkConnectionId(self.next_id); let next_id = self.next_id.checked_add(1).ok_or_else(|| NetworkRuntimeError::InvalidOperation("ID koneksi jaringan melampaui batas integer.".into()))?; match constraints.transport { NetworkTransport::Reliable => { let stream = TcpStream::connect_timeout(&resolved, timeout).map_err(|e| NetworkRuntimeError::Connection(e.to_string()))?; stream.set_read_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; stream.set_write_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; self.connections.insert(id, Connection::Tcp { stream, _permit: permit }); }, NetworkTransport::Unreliable => { let socket = UdpSocket::bind(match resolved { SocketAddr::V4(_) => "0.0.0.0:0", SocketAddr::V6(_) => "[::]:0" }).map_err(|e| NetworkRuntimeError::Connection(e.to_string()))?; socket.connect(resolved).map_err(|e| NetworkRuntimeError::Connection(e.to_string()))?; socket.set_read_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; socket.set_write_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; self.connections.insert(id, Connection::Udp { socket, _permit: permit }); } } self.next_id = next_id; Ok(NetworkRuntimeResult::Connected { id, endpoint: NetworkEndpoint { requested: host, resolved } }) }
    fn send(&mut self, id: NetworkConnectionId, data: &[u8]) -> Result<NetworkRuntimeResult, NetworkRuntimeError> { if data.len() > MAX_NETWORK_PAYLOAD_BYTES { return Err(NetworkRuntimeError::InvalidOperation("Payload jaringan melebihi batas 16 MiB.".into())); } let connection = self.connections.get_mut(&id).ok_or_else(|| NetworkRuntimeError::Connection(format!("Koneksi {} tidak ditemukan.", id.0)))?; let bytes = match connection { Connection::Tcp { stream, .. } => { stream.write_all(data).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?; data.len() }, Connection::Udp { socket, .. } => socket.send(data).map_err(|e| NetworkRuntimeError::Io(e.to_string()))? }; Ok(NetworkRuntimeResult::Sent { id, bytes }) }
    fn receive(&mut self, id: NetworkConnectionId) -> Result<NetworkRuntimeResult, NetworkRuntimeError> { let connection = self.connections.get_mut(&id).ok_or_else(|| NetworkRuntimeError::Connection(format!("Koneksi {} tidak ditemukan.", id.0)))?; let mut buffer = vec![0u8; 64 * 1024]; let size = match connection { Connection::Tcp { stream, .. } => stream.read(&mut buffer).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?, Connection::Udp { socket, .. } => socket.recv(&mut buffer).map_err(|e| NetworkRuntimeError::Io(e.to_string()))? }; buffer.truncate(size); Ok(NetworkRuntimeResult::Received { id, data: buffer }) }
    pub fn close(&mut self, id: NetworkConnectionId) -> bool { self.connections.remove(&id).is_some() }
}
pub(crate) fn resolve_target(host: &str, port: u16, address_kind: Option<NetworkAddressKind>) -> Result<SocketAddr, NetworkRuntimeError> { let service = if host.contains(':') { format!("[{host}]:{port}") } else { format!("{host}:{port}") }; let mut addresses = service.to_socket_addrs().map_err(|e| NetworkRuntimeError::Resolution(e.to_string()))?; addresses.find(|a| match address_kind { Some(NetworkAddressKind::Ipv4) => a.is_ipv4(), Some(NetworkAddressKind::Ipv6) => a.is_ipv6(), Some(NetworkAddressKind::Name) | None => true }).ok_or_else(|| NetworkRuntimeError::Resolution(format!("Tidak ada alamat yang cocok untuk {host}."))) }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest { pub method: String, pub authority: String, pub path: String, pub headers: Vec<(String, String)>, pub body: Vec<u8> }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse { pub status: u16, pub reason: String, pub headers: Vec<(String, String)>, pub body: Vec<u8> }
impl HttpRequest {
    pub fn get(authority: impl Into<String>, path: impl Into<String>) -> Self {
        Self { method: "GET".into(), authority: authority.into(), path: path.into(), headers: Vec::new(), body: Vec::new() }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, NetworkRuntimeError> {
        let invalid = || NetworkRuntimeError::Protocol("Permintaan HTTP memiliki token atau framing yang tidak valid.".into());
        if !valid_http_header_name(&self.method)
            || self.method.len() > MAX_HTTP_METHOD_BYTES
            || self.path.is_empty()
            || self.path.len() > MAX_HTTP_PATH_BYTES
            || self.body.len() > MAX_HTTP_BODY_BYTES
            || self.headers.len() > MAX_HTTP_HEADER_COUNT
            || !(self.path.starts_with('/') || self.path == "*")
            || self.path.bytes().any(|byte| byte <= 0x20 || byte == 0x7f)
            || self.authority.is_empty()
            || self.authority.bytes().any(|byte| byte <= 0x20 || byte == 0x7f || byte == b'\r' || byte == b'\n')
            || self.authority.contains('@')
            || self.authority.contains('/')
            || self.authority.contains('#')
            || self.authority.contains('?')
        {
            return Err(invalid());
        }
        parse_http_authority(&self.authority)?;
        let mut header_bytes = 0usize;
        for (name, value) in &self.headers {
            if !valid_http_header_name(name)
                || name.eq_ignore_ascii_case("host")
                || name.eq_ignore_ascii_case("transfer-encoding")
                || value.bytes().any(|byte| (byte < 0x20 && byte != b'\t') || byte == 0x7f)
            {
                return Err(invalid());
            }
            header_bytes = header_bytes
                .checked_add(name.len())
                .and_then(|total| total.checked_add(value.len()))
                .and_then(|total| total.checked_add(4))
                .ok_or_else(invalid)?;
            if header_bytes > MAX_HTTP_HEADER_BYTES {
                return Err(invalid());
            }
        }

        let mut bytes = format!("{} {} HTTP/1.1\r\nHost: {}\r\n", self.method, self.path, self.authority).into_bytes();
        let mut has_length = false;
        for (name, value) in &self.headers {
            if !valid_http_header_name(name)
                || name.eq_ignore_ascii_case("host")
                || name.eq_ignore_ascii_case("transfer-encoding")
                || value.bytes().any(|byte| (byte < 0x20 && byte != b'\t') || byte == 0x7f)
            {
                return Err(invalid());
            }
            if name.eq_ignore_ascii_case("content-length") {
                if has_length || value.parse::<usize>().ok() != Some(self.body.len()) {
                    return Err(invalid());
                }
                has_length = true;
            }
            bytes.extend_from_slice(format!("{}: {}\r\n", name, value).as_bytes());
        }
        if !has_length && (!self.body.is_empty() || self.method.eq_ignore_ascii_case("POST") || self.method.eq_ignore_ascii_case("PUT") || self.method.eq_ignore_ascii_case("PATCH")) {
            bytes.extend_from_slice(format!("Content-Length: {}\r\n", self.body.len()).as_bytes());
        }
        bytes.extend_from_slice(b"Connection: close\r\n\r\n");
        bytes.extend_from_slice(&self.body);
        Ok(bytes)
    }
}
fn valid_http_header_name(name: &str) -> bool {
    !name.is_empty()
        && name.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte)
        })
}

pub fn parse_http_response(mut bytes: &[u8], max_body: usize) -> Result<HttpResponse, NetworkRuntimeError> { let marker = b"\r\n\r\n"; let header_end = bytes.windows(marker.len()).position(|w| w == marker).ok_or_else(|| NetworkRuntimeError::Protocol("Respons HTTP tidak memiliki akhir header yang valid.".into()))?; if header_end > 64 * 1024 { return Err(NetworkRuntimeError::Protocol("Header HTTP melebihi batas ukuran.".into())); } let header_bytes = &bytes[..header_end]; bytes = &bytes[header_end + marker.len()..]; let text = std::str::from_utf8(header_bytes).map_err(|_| NetworkRuntimeError::Protocol("Header HTTP bukan UTF-8 yang valid.".into()))?; let mut lines = text.split("\r\n"); let status_line = lines.next().ok_or_else(|| NetworkRuntimeError::Protocol("Respons HTTP kosong.".into()))?; let mut parts = status_line.splitn(3, ' '); if parts.next() != Some("HTTP/1.1") { return Err(NetworkRuntimeError::Protocol("Hanya HTTP/1.1 yang diterima pada adapter ini.".into())); } let status = parts.next()
    .filter(|value| value.len() == 3 && value.bytes().all(|byte| byte.is_ascii_digit()))
    .and_then(|value| value.parse::<u16>().ok())
    .filter(|status| (100..=599).contains(status))
    .ok_or_else(|| NetworkRuntimeError::Protocol("Status HTTP tidak valid.".into()))?;
let reason = parts.next().unwrap_or_default();
if reason.bytes().any(|byte| (byte < 0x20 && byte != b'\t') || byte == 0x7f) {
    return Err(NetworkRuntimeError::Protocol("Reason phrase HTTP tidak valid.".into()));
}
let reason = reason.to_string();
let mut headers = Vec::new(); let mut content_length = None; let mut chunked = false; for line in lines { let (name, value) = line.split_once(':').ok_or_else(|| NetworkRuntimeError::Protocol("Header HTTP tidak memiliki pemisah ':'.".into()))?; if !valid_http_header_name(name) || value.bytes().any(|byte| (byte < 0x20 && byte != b'\t') || byte == 0x7f) { return Err(NetworkRuntimeError::Protocol("Nama atau nilai header HTTP tidak valid.".into())); } let value = value.trim().to_string(); if name.eq_ignore_ascii_case("content-length") { if content_length.is_some() { return Err(NetworkRuntimeError::Protocol("Header Content-Length duplikat tidak diizinkan.".into())); } if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(NetworkRuntimeError::Protocol("Content-Length tidak valid.".into()));
        }
        content_length = Some(value.parse::<usize>().map_err(|_| NetworkRuntimeError::Protocol("Content-Length melampaui batas integer.".into()))?); } if name.eq_ignore_ascii_case("transfer-encoding") { if !value.eq_ignore_ascii_case("chunked") || chunked { return Err(NetworkRuntimeError::Protocol("Transfer-Encoding tidak didukung atau duplikat.".into())); } chunked = true; } if headers.len() >= MAX_HTTP_HEADER_COUNT { return Err(NetworkRuntimeError::Protocol("Jumlah header HTTP melebihi batas.".into())); } headers.push((name.to_string(), value)); } if chunked && content_length.is_some() { return Err(NetworkRuntimeError::Protocol("Content-Length dan Transfer-Encoding tidak boleh hadir bersamaan.".into())); } let body = if chunked { parse_chunked_body(bytes, max_body)? } else if let Some(length) = content_length { if length > max_body { return Err(NetworkRuntimeError::Protocol("HTTP body melebihi batas ukuran.".into())); } if bytes.len() < length { return Err(NetworkRuntimeError::Protocol("HTTP body terpotong.".into())); } if bytes.len() > length { return Err(NetworkRuntimeError::Protocol("Data setelah Content-Length tidak diizinkan.".into())); } bytes.to_vec() } else { if bytes.len() > max_body { return Err(NetworkRuntimeError::Protocol("HTTP body melebihi batas ukuran.".into())); } bytes.to_vec() }; Ok(HttpResponse { status, reason, headers, body }) }
fn parse_chunked_body(mut bytes: &[u8], max_body: usize) -> Result<Vec<u8>, NetworkRuntimeError> { let mut output = Vec::new(); loop { let line_end = bytes.windows(2).position(|w| w == b"\r\n").ok_or_else(|| NetworkRuntimeError::Protocol("Ukuran HTTP chunk tidak lengkap.".into()))?;
        if line_end == 0 || line_end > 1024 {
            return Err(NetworkRuntimeError::Protocol("Baris ukuran HTTP chunk kosong atau melebihi batas.".into()));
        }
        let size_text = std::str::from_utf8(&bytes[..line_end]).map_err(|_| NetworkRuntimeError::Protocol("Ukuran HTTP chunk tidak valid.".into()))?;
        if !size_text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(NetworkRuntimeError::Protocol("Ukuran HTTP chunk bukan heksadesimal kanonis.".into()));
        }
        let size = usize::from_str_radix(size_text, 16).map_err(|_| NetworkRuntimeError::Protocol("Ukuran HTTP chunk melampaui batas integer.".into()))?; bytes = &bytes[line_end + 2..]; if size == 0 {
            if bytes == b"\r\n" {
                return Ok(output);
            }
            let trailer_end = bytes.windows(4).position(|window| window == b"\r\n\r\n")
                .ok_or_else(|| NetworkRuntimeError::Protocol("Trailer HTTP chunk tidak lengkap.".into()))?;
            if trailer_end > MAX_HTTP_HEADER_BYTES || trailer_end + 4 != bytes.len() {
                return Err(NetworkRuntimeError::Protocol("Trailer HTTP chunk melebihi batas atau memiliki data tambahan.".into()));
            }
            let trailers = std::str::from_utf8(&bytes[..trailer_end])
                .map_err(|_| NetworkRuntimeError::Protocol("Trailer HTTP bukan UTF-8 yang valid.".into()))?;
            for line in trailers.split("\r\n") {
                let (name, value) = line.split_once(':')
                    .ok_or_else(|| NetworkRuntimeError::Protocol("Trailer HTTP tidak valid.".into()))?;
                if !valid_http_header_name(name)
                    || name.eq_ignore_ascii_case("content-length")
                    || name.eq_ignore_ascii_case("transfer-encoding")
                    || value.bytes().any(|byte| (byte < 0x20 && byte != b'\t') || byte == 0x7f)
                {
                    return Err(NetworkRuntimeError::Protocol("Trailer HTTP tidak valid atau mengubah framing.".into()));
                }
            }
            return Ok(output);
        } if size > max_body.saturating_sub(output.len()) {
            return Err(NetworkRuntimeError::Protocol("HTTP chunk melebihi batas ukuran.".into()));
        }
        let framed_size = size.checked_add(2)
            .ok_or_else(|| NetworkRuntimeError::Protocol("Ukuran HTTP chunk melampaui batas integer.".into()))?;
        if bytes.len() < framed_size {
            return Err(NetworkRuntimeError::Protocol("HTTP chunk terpotong.".into()));
        }
        output.extend_from_slice(&bytes[..size]);
        if &bytes[size..framed_size] != b"\r\n" {
            return Err(NetworkRuntimeError::Protocol("HTTP chunk tidak diakhiri CRLF.".into()));
        }
        bytes = &bytes[framed_size..]; } }
pub(crate) const MAX_HTTP_BODY_BYTES: usize = 16 * 1024 * 1024;
pub(crate) const MAX_HTTP_HEADER_BYTES: usize = 64 * 1024;
pub(crate) const MAX_HTTP_HEADER_COUNT: usize = 256;
const MAX_HTTP_PATH_BYTES: usize = 8 * 1024;
const MAX_HTTP_METHOD_BYTES: usize = 32;
pub(crate) const MAX_HTTP_RESPONSE_BYTES: usize = MAX_HTTP_BODY_BYTES + MAX_HTTP_HEADER_BYTES;

pub(crate) fn read_bounded_response<R: Read>(reader: R, max_bytes: usize) -> Result<Vec<u8>, NetworkRuntimeError> {
    let limit = u64::try_from(max_bytes).unwrap_or(u64::MAX).saturating_add(1);
    let mut response = Vec::new();
    reader.take(limit).read_to_end(&mut response).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?;
    if response.len() > max_bytes {
        return Err(NetworkRuntimeError::Protocol("Respons HTTP melebihi batas ukuran total.".into()));
    }
    Ok(response)
}

pub(crate) fn parse_http_authority(authority: &str) -> Result<(String, u16), NetworkRuntimeError> {
    parse_http_authority_with_default(authority, 80)
}

pub(crate) fn parse_http_authority_with_default(authority: &str, default_port: u16) -> Result<(String, u16), NetworkRuntimeError> {
    let invalid = || NetworkRuntimeError::Protocol("Authority HTTP tidak valid.".into());
    if authority.is_empty()
        || authority.chars().any(|character| character.is_ascii_control() || character.is_ascii_whitespace())
        || authority.contains('@')
        || authority.contains('/')
        || authority.contains('?')
        || authority.contains('#')
        || authority.contains('\\')
    {
        return Err(invalid());
    }

    let (host, port) = if let Some(bracketed) = authority.strip_prefix('[') {
        let close = bracketed.find(']').ok_or_else(invalid)?;
        let host = &bracketed[..close];
        host.parse::<std::net::Ipv6Addr>().map_err(|_| invalid())?;
        let suffix = &bracketed[close + 1..];
        let port = if suffix.is_empty() {
            default_port
        } else {
            let port_text = suffix.strip_prefix(':').ok_or_else(invalid)?;
            if port_text.is_empty() || !port_text.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(invalid());
            }
            port_text.parse::<u16>().map_err(|_| invalid())?
        };
        (host.to_owned(), port)
    } else {
        if authority.matches(':').count() > 1 {
            return Err(invalid());
        }
        let (host, port) = match authority.split_once(':') {
            Some((host, port_text)) => {
                if port_text.is_empty() || !port_text.bytes().all(|byte| byte.is_ascii_digit()) {
                    return Err(invalid());
                }
                (host, port_text.parse::<u16>().map_err(|_| invalid())?)
            }
            None => (authority, default_port),
        };
        if host.is_empty()
            || host.len() > 253
            || host.split('.').any(|label| {
                label.is_empty()
                    || label.len() > 63
                    || label.starts_with('-')
                    || label.ends_with('-')
                    || !label.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            })
        {
            return Err(invalid());
        }
        (host.to_owned(), port)
    };
    Ok((host, port))
}

pub fn execute_http11(_runtime: &mut RealNetworkRuntime, request: HttpRequest, capabilities: &CapabilitySet, timeout: Duration) -> Result<HttpResponse, NetworkRuntimeError> {
    if !capabilities.permits(crate::capability::Capability::NetworkConnect) {
        return Err(NetworkRuntimeError::CapabilityDenied("Kemampuan jaringan NetworkConnect belum diberikan kepada program.".into()));
    }
    let _resource_permit = ResourcePermit::operation().ok_or_else(|| NetworkRuntimeError::InvalidOperation("Jumlah operasi jaringan aktif melampaui batas global.".into()))?;
    let (host, port) = parse_http_authority(&request.authority)?;
    let address = resolve_target(&host, port, None)?;
    let mut stream = TcpStream::connect_timeout(&address, timeout).map_err(|e| NetworkRuntimeError::Connection(e.to_string()))?;
    stream.set_read_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?;
    stream.set_write_timeout(Some(timeout)).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?;
    stream.write_all(&request.to_bytes()?).map_err(|e| NetworkRuntimeError::Io(e.to_string()))?;
    let response = read_bounded_response(stream, MAX_HTTP_RESPONSE_BYTES)?;
    parse_http_response(&response, MAX_HTTP_BODY_BYTES)
}
#[cfg(test)]
mod tests { use super::*; use crate::network::NetworkProtocolFamily; use std::io::{Read, Write}; use std::net::TcpListener; use std::sync::Mutex; use std::thread;
    // These tests consume the process-global network operation budget. Serialize them so the quota-exhaustion test cannot starve unrelated loopback tests.
    static GLOBAL_NETWORK_PERMIT_TEST_LOCK: Mutex<()> = Mutex::new(());
    #[test]
    fn http_request_rejects_request_smuggling_and_header_injection() {
        for request in [
            HttpRequest { method: "GET\r\nInjected: yes".into(), authority: "example.com".into(), path: "/".into(), headers: vec![], body: vec![] },
            HttpRequest { method: "GET".into(), authority: "example.com".into(), path: "/\r\nInjected: yes".into(), headers: vec![], body: vec![] },
            HttpRequest { method: "GET".into(), authority: "user@example.com".into(), path: "/".into(), headers: vec![], body: vec![] },
            HttpRequest { method: "POST".into(), authority: "example.com".into(), path: "/".into(), headers: vec![("Content-Length".into(), "1".into()), ("content-length".into(), "2".into())], body: b"x".to_vec() },
            HttpRequest { method: "POST".into(), authority: "example.com".into(), path: "/".into(), headers: vec![("Transfer-Encoding".into(), "chunked".into())], body: b"x".to_vec() },
            HttpRequest { method: "POST".into(), authority: "example.com".into(), path: "/".into(), headers: vec![("Content-Length".into(), "999".into())], body: b"x".to_vec() },
        ] {
            assert!(request.to_bytes().is_err());
        }
    }

    #[test]
    fn http_request_rejects_oversized_path_body_and_header_collection() {
        let mut request = HttpRequest::get("example.com", "/");
        request.path = format!("/{}", "x".repeat(MAX_HTTP_PATH_BYTES));
        assert!(request.to_bytes().is_err());

        let mut request = HttpRequest::get("example.com", "/");
        request.body = vec![b'x'; MAX_HTTP_BODY_BYTES + 1];
        assert!(request.to_bytes().is_err());

        let mut request = HttpRequest::get("example.com", "/");
        request.headers = (0..=MAX_HTTP_HEADER_COUNT)
            .map(|index| (format!("x-{index}"), "v".into()))
            .collect();
        assert!(request.to_bytes().is_err());
    }

    #[test]
    fn http_request_generates_content_length_for_empty_body_methods_that_carry_content() {
        let request = HttpRequest { method: "POST".into(), authority: "example.com".into(), path: "/".into(), headers: vec![], body: vec![] };
        let wire = String::from_utf8(request.to_bytes().expect("valid request")).expect("UTF-8");
        assert!(wire.contains("Content-Length: 0\r\n"));
    }

    #[test]
    fn authority_parser_applies_protocol_specific_default_ports() {
        assert_eq!(parse_http_authority_with_default("example.com", 443), Ok(("example.com".into(), 443)));
        assert_eq!(parse_http_authority_with_default("[::1]", 443), Ok(("::1".into(), 443)));
        assert_eq!(parse_http_authority_with_default("example.com:8080", 443), Ok(("example.com".into(), 8080)));
    }

    #[test]
    fn http_authority_parser_handles_ipv6_and_explicit_ports() {
        assert_eq!(parse_http_authority("example.com"), Ok(("example.com".into(), 80)));
        assert_eq!(parse_http_authority("example.com:8080"), Ok(("example.com".into(), 8080)));
        assert_eq!(parse_http_authority("[::1]:8080"), Ok(("::1".into(), 8080)));
        assert_eq!(parse_http_authority("[::1]"), Ok(("::1".into(), 80)));
    }

    #[test]
    fn http_authority_parser_rejects_ambiguous_or_injected_authorities() {
        for authority in ["", "example.com:", "example.com:abc", "example.com:80:90", "::1", "user@example.com", "example.com/path", "bad host", "[not-ipv6]:80", "example.com:65536"] {
            assert!(parse_http_authority(authority).is_err(), "authority should be rejected: {authority}");
        }
    }

    #[test]
    fn network_runtime_enforces_active_connection_quota() {
        assert!(ensure_connection_capacity(MAX_NETWORK_CONNECTIONS - 1).is_ok());
        assert!(matches!(
            ensure_connection_capacity(MAX_NETWORK_CONNECTIONS),
            Err(NetworkRuntimeError::InvalidOperation(message)) if message.contains("koneksi")
        ));
    }

    #[test]
    fn network_connections_hold_global_resource_permits_until_closed() {
        let _guard = GLOBAL_NETWORK_PERMIT_TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let listeners: Vec<_> = (0..9)
            .map(|_| TcpListener::bind("127.0.0.1:0").expect("bind loopback listener"))
            .collect();
        let capabilities = CapabilitySet::none().allow(crate::capability::Capability::NetworkConnect);
        let mut runtime = RealNetworkRuntime::new();
        let connect = |runtime: &mut RealNetworkRuntime, listener: &TcpListener| {
            let address = listener.local_addr().expect("listener address");
            runtime.execute(
                NetworkOperation::Connect {
                    target: NetworkTarget::Service {
                        host: address.ip().to_string(),
                        port: address.port(),
                    },
                    constraints: crate::network::NetworkConstraints {
                        address_kind: Some(NetworkAddressKind::Ipv4),
                        transport: NetworkTransport::Reliable,
                        security: Some(crate::network::NetworkSecurity::Insecure),
                        timeout_ms: 1_000,
                        protocol: Some(NetworkProtocolFamily::Tcp),
                    },
                },
                &capabilities,
            )
        };
        let deadline = std::time::Instant::now() + Duration::from_secs(10);
        let first_id = loop {
            match connect(&mut runtime, &listeners[0]) {
                Ok(NetworkRuntimeResult::Connected { id, .. }) => break id,
                Err(NetworkRuntimeError::InvalidOperation(message))
                    if message.contains("global") && std::time::Instant::now() < deadline =>
                {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("first connection must eventually acquire a permit: {error:?}"),
                Ok(_) => panic!("connect must return a connection id"),
            }
        };

        let mut ids = vec![first_id];
        let mut denied_at_capacity = false;
        for listener in listeners.iter().skip(1) {
            match connect(&mut runtime, listener) {
                Ok(NetworkRuntimeResult::Connected { id, .. }) => ids.push(id),
                Err(NetworkRuntimeError::InvalidOperation(message))
                    if message.contains("global") =>
                {
                    denied_at_capacity = true;
                    break;
                }
                Err(error) => panic!("connection failed for a reason other than quota denial: {error:?}"),
                Ok(_) => panic!("connect must return a connection id"),
            }
        }
        assert!(denied_at_capacity, "global resource cap must reject a connection when full");

        assert!(runtime.close(first_id), "closing a connection releases its resource permit");
        let reopen_deadline = std::time::Instant::now() + Duration::from_secs(10);
        let reopened_id = loop {
            match connect(&mut runtime, &listeners[8]) {
                Ok(NetworkRuntimeResult::Connected { id, .. }) => break id,
                Err(NetworkRuntimeError::InvalidOperation(message))
                    if message.contains("global") && std::time::Instant::now() < reopen_deadline =>
                {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("closing a connection must restore admission: {error:?}"),
                Ok(_) => panic!("connect must return a connection id"),
            }
        };
        assert!(runtime.close(reopened_id));
        for id in ids.into_iter().skip(1) {
            runtime.close(id);
        }
    }
    #[test]
    fn network_runtime_rejects_oversized_send_payload_before_connection_lookup() {
        let mut runtime = RealNetworkRuntime::new();
        let capabilities = CapabilitySet::none().allow(crate::capability::Capability::NetworkConnect);
        let error = runtime.execute(
            NetworkOperation::Send {
                connection: NetworkConnectionId(u64::MAX),
                data: vec![b'x'; MAX_NETWORK_PAYLOAD_BYTES + 1],
            },
            &capabilities,
        ).expect_err("oversized payload must be rejected");
        assert!(matches!(error, NetworkRuntimeError::InvalidOperation(message) if message.contains("16 MiB")));
    }

    #[test]
    fn tcp_send_reports_all_bytes_after_write_all() {
        let _guard = GLOBAL_NETWORK_PERMIT_TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let address = listener.local_addr().expect("address");
        let payload = vec![b'x'; 128 * 1024];
        let expected = payload.clone();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            let mut received = vec![0; expected.len()];
            stream.read_exact(&mut received).expect("read full payload");
            assert_eq!(received, expected);
        });
        let mut runtime = RealNetworkRuntime::new();
        let capabilities = CapabilitySet::none().allow(crate::capability::Capability::NetworkConnect);
        let connected = runtime.execute(NetworkOperation::Connect {
            target: NetworkTarget::Service { host: address.ip().to_string(), port: address.port() },
            constraints: crate::network::NetworkConstraints {
                address_kind: Some(NetworkAddressKind::Ipv4),
                transport: NetworkTransport::Reliable,
                security: Some(crate::network::NetworkSecurity::Insecure),
                timeout_ms: 2_000,
                protocol: Some(NetworkProtocolFamily::Tcp),
            },
        }, &capabilities).expect("connect");
        let id = match connected { NetworkRuntimeResult::Connected { id, .. } => id, _ => panic!("unexpected result") };
        assert_eq!(runtime.execute(NetworkOperation::Send { connection: id, data: payload.clone() }, &capabilities), Ok(NetworkRuntimeResult::Sent { id, bytes: payload.len() }));
        server.join().expect("server");
    }

    #[test] fn localhost_resolution_returns_a_real_socket_address() { let address = resolve_target("localhost", 80, None).expect("localhost must resolve"); assert!(address.ip().is_loopback()); }
    #[test] fn tcp_round_trip_uses_real_loopback_socket() { let _guard = GLOBAL_NETWORK_PERMIT_TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner()); let listener = TcpListener::bind("127.0.0.1:0").expect("bind"); let address = listener.local_addr().expect("address"); let server = thread::spawn(move || { let (mut stream, _) = listener.accept().expect("accept"); let mut input = [0u8; 4]; stream.read_exact(&mut input).expect("read"); assert_eq!(&input, b"ping"); stream.write_all(b"pong").expect("write"); }); let mut runtime = RealNetworkRuntime::new(); let capabilities = CapabilitySet::none().allow(crate::capability::Capability::NetworkConnect); let connected = runtime.execute(NetworkOperation::Connect { target: NetworkTarget::Service { host: address.ip().to_string(), port: address.port() }, constraints: crate::network::NetworkConstraints { address_kind: Some(NetworkAddressKind::Ipv4), transport: NetworkTransport::Reliable, security: Some(crate::network::NetworkSecurity::Insecure), timeout_ms: 2_000, protocol: Some(NetworkProtocolFamily::Tcp) } }, &capabilities).expect("connect"); let id = match connected { NetworkRuntimeResult::Connected { id, .. } => id, _ => panic!("unexpected result") }; runtime.execute(NetworkOperation::Send { connection: id, data: b"ping".to_vec() }, &capabilities).expect("send"); let received = runtime.execute(NetworkOperation::Receive { connection: id }, &capabilities).expect("receive"); assert_eq!(received, NetworkRuntimeResult::Received { id, data: b"pong".to_vec() }); server.join().expect("server"); }
    #[test] fn capability_denial_happens_before_connection() { let mut runtime = RealNetworkRuntime::new(); let result = runtime.execute(NetworkOperation::Connect { target: NetworkTarget::Service { host: "127.0.0.1".into(), port: 9 }, constraints: crate::network::NetworkConstraints { address_kind: Some(NetworkAddressKind::Ipv4), transport: NetworkTransport::Reliable, security: Some(crate::network::NetworkSecurity::Insecure), timeout_ms: 50, protocol: Some(NetworkProtocolFamily::Tcp) } }, &CapabilitySet::none()); assert!(matches!(result, Err(NetworkRuntimeError::CapabilityDenied(_)))); }
    #[test] fn http_request_and_response_are_real_bytes() { let _guard = GLOBAL_NETWORK_PERMIT_TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner()); let listener = TcpListener::bind("127.0.0.1:0").expect("bind"); let address = listener.local_addr().expect("address"); let server = thread::spawn(move || { let (mut stream, _) = listener.accept().expect("accept"); let mut request = Vec::new(); let mut buffer = [0u8; 1024]; loop { let read = stream.read(&mut buffer).expect("read"); if read == 0 { break; } request.extend_from_slice(&buffer[..read]); if request.windows(4).any(|w| w == b"\r\n\r\n") { break; } } assert!(request.starts_with(b"GET / HTTP/1.1\r\n")); stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello").expect("write"); }); let mut runtime = RealNetworkRuntime::new(); let capabilities = CapabilitySet::none().allow(crate::capability::Capability::NetworkConnect); let authority = format!("{}:{}", address.ip(), address.port()); let response = execute_http11(&mut runtime, HttpRequest::get(authority, "/"), &capabilities, Duration::from_secs(2)).expect("http"); assert_eq!(response.status, 200); assert_eq!(response.body, b"hello"); server.join().expect("server"); }
    #[test]
    fn http_response_rejects_invalid_status_headers_and_trailing_body_bytes() {
        for response in [
            &b"HTTP/1.1 20 OK\r\nContent-Length: 0\r\n\r\n"[..],
            &b"HTTP/1.1 2000 OK\r\nContent-Length: 0\r\n\r\n"[..],
            &b"HTTP/1.1 200 OK\r\n: value\r\nContent-Length: 0\r\n\r\n"[..],
            &b"HTTP/1.1 200 OK\r\nBad Header: value\r\nContent-Length: 0\r\n\r\n"[..],
            &b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\nxy"[..],
            &b"HTTP/1.1 200 OK\r\nContent-Length: +1\r\n\r\n"[..],
            &b"HTTP/1.1 200 Bad\x01Reason\r\nContent-Length: 0\r\n\r\n"[..],
        ] {
            assert!(parse_http_response(response, 16).is_err());
        }
    }

    #[test]
    fn chunked_http_body_rejects_extra_bytes_and_framing_trailers() {
        assert!(parse_http_response(
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n1\r\nx\r\n0\r\n\r\nextra",
            16,
        ).is_err());
        assert!(parse_http_response(
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n0\r\nContent-Length: 1\r\n\r\n",
            16,
        ).is_err());
    }

    #[test]
    fn chunked_http_body_accepts_bounded_non_framing_trailers() {
        let response = parse_http_response(
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n0\r\nX-Trace: ok\r\n\r\n",
            16,
        ).expect("valid chunked response with a trailer");
        assert_eq!(response.body, b"hello");
    }

    #[test]
    fn chunked_http_body_rejects_whitespace_and_overlong_size_lines() {
        for body in [
            &b"1 \r\nx\r\n0\r\n\r\n"[..],
            &b" 1\r\nx\r\n0\r\n\r\n"[..],
            &b"1;extension=yes\r\nx\r\n0\r\n\r\n"[..],
        ] {
            let response = [b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n".as_slice(), body].concat();
            assert!(parse_http_response(&response, 16).is_err());
        }
        let huge_line = format!("{}\r\n", "a".repeat(1025));
        let response = format!("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n{huge_line}");
        assert!(parse_http_response(response.as_bytes(), usize::MAX).is_err());
    }

    #[test]
    fn chunked_http_body_rejects_size_integer_overflow_without_panicking() {
        let error = parse_http_response(
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nffffffffffffffff\r\n",
            usize::MAX,
        ).expect_err("chunk framing integer overflow must be rejected");
        assert!(matches!(error, NetworkRuntimeError::Protocol(_)));
    }

    #[test] fn chunked_http_body_is_decoded_without_guessing() { let response = parse_http_response(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n0\r\n\r\n", 100).expect("parse"); assert_eq!(response.body, b"hello"); }
    #[test] fn bounded_http_reader_rejects_oversized_responses() { let input = vec![b'x'; 1025]; let error = read_bounded_response(std::io::Cursor::new(input), 1024).expect_err("oversized response must be rejected"); assert!(matches!(error, NetworkRuntimeError::Protocol(message) if message.contains("batas ukuran total"))); }
    #[test] fn bounded_http_reader_accepts_responses_at_the_limit() { let input = vec![b'x'; 1024]; assert_eq!(read_bounded_response(std::io::Cursor::new(input.clone()), 1024).expect("response at limit"), input); }

    #[test] fn oversized_http_headers_are_rejected() { let response = format!("HTTP/1.1 200 OK\r\nX-Large: {}\r\nContent-Length: 0\r\n\r\n", "x".repeat(65 * 1024)); let error = parse_http_response(response.as_bytes(), 10).expect_err("oversized header must be rejected"); assert!(matches!(error, NetworkRuntimeError::Protocol(message) if message.contains("batas ukuran"))); }
    #[test] fn excessive_http_header_count_is_rejected() { let mut response = String::from("HTTP/1.1 200 OK\r\n"); for index in 0..257 { response.push_str(&format!("X-{index}: v\r\n")); } response.push_str("Content-Length: 0\r\n\r\n"); let error = parse_http_response(response.as_bytes(), 10).expect_err("excessive headers must be rejected"); assert!(matches!(error, NetworkRuntimeError::Protocol(message) if message.contains("Jumlah header"))); }
    #[test] fn duplicate_content_length_headers_are_rejected() { let error = parse_http_response(b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\nContent-Length: 1\r\n\r\nx", 10).expect_err("duplicate length must be rejected"); assert!(matches!(error, NetworkRuntimeError::Protocol(message) if message.contains("duplikat"))); }
    #[test] fn transfer_encoding_and_content_length_conflicts_are_rejected() { let error = parse_http_response(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n\r\n", 10).expect_err("ambiguous framing must be rejected"); assert!(matches!(error, NetworkRuntimeError::Protocol(message) if message.contains("tidak boleh hadir bersamaan"))); }
    #[test] fn unsupported_transfer_encoding_is_rejected() { let error = parse_http_response(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip\r\n\r\nx", 10).expect_err("unsupported transfer encoding must be rejected"); assert!(matches!(error, NetworkRuntimeError::Protocol(message) if message.contains("tidak didukung"))); }
}
