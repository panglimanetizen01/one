use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::capability::{Capability, CapabilitySet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MailProtocolError {
    CapabilityDenied,
    InvalidResponse,
    Io(String),
    Timeout,
    Server(String),
}

fn permit(caps: &CapabilitySet) -> Result<(), MailProtocolError> {
    if caps.permits(Capability::NetworkConnect) { Ok(()) } else { Err(MailProtocolError::CapabilityDenied) }
}

fn io_error(error: std::io::Error) -> MailProtocolError {
    if matches!(error.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock) { MailProtocolError::Timeout } else { MailProtocolError::Io(error.to_string()) }
}

fn connect<A: ToSocketAddrs>(address: A, timeout: Duration, caps: &CapabilitySet) -> Result<TcpStream, MailProtocolError> {
    permit(caps)?;
    let address = address.to_socket_addrs().map_err(io_error)?.next().ok_or_else(|| MailProtocolError::Io("no address resolved".into()))?;
    let stream = TcpStream::connect_timeout(&address, timeout).map_err(io_error)?;
    stream.set_read_timeout(Some(timeout)).map_err(io_error)?;
    stream.set_write_timeout(Some(timeout)).map_err(io_error)?;
    Ok(stream)
}

fn read_line(stream: &mut TcpStream) -> Result<String, MailProtocolError> {
    let mut bytes = Vec::new();
    let mut byte = [0u8; 1];
    loop {
        stream.read_exact(&mut byte).map_err(io_error)?;
        bytes.push(byte[0]);
        if bytes.ends_with(b"\r\n") { break; }
        if bytes.len() > 64 * 1024 { return Err(MailProtocolError::InvalidResponse); }
    }
    String::from_utf8(bytes).map_err(|_| MailProtocolError::InvalidResponse)
}

fn command(stream: &mut TcpStream, command: &str) -> Result<String, MailProtocolError> {
    stream.write_all(command.as_bytes()).map_err(io_error)?;
    stream.write_all(b"\r\n").map_err(io_error)?;
    read_line(stream)
}

fn require_ok(line: &str, prefix: &str) -> Result<(), MailProtocolError> {
    if line.starts_with(prefix) { Ok(()) } else { Err(MailProtocolError::Server(line.trim_end().to_string())) }
}

pub struct Pop3Client {
    stream: TcpStream,
}

impl Pop3Client {
    pub fn connect(address: &str, timeout: Duration, caps: &CapabilitySet) -> Result<Self, MailProtocolError> {
        let mut stream = connect(address, timeout, caps)?;
        let greeting = read_line(&mut stream)?;
        require_ok(&greeting, "+OK")?;
        Ok(Self { stream })
    }

    pub fn login(&mut self, user: &str, password: &str) -> Result<(), MailProtocolError> {
        require_ok(&command(&mut self.stream, &format!("USER {user}"))?, "+OK")?;
        require_ok(&command(&mut self.stream, &format!("PASS {password}"))?, "+OK")
    }

    pub fn list(&mut self) -> Result<Vec<(u32, u64)>, MailProtocolError> {
        let first = command(&mut self.stream, "LIST")?;
        require_ok(&first, "+OK")?;
        let mut messages = Vec::new();
        loop {
            let line = read_line(&mut self.stream)?;
            let trimmed = line.trim_end();
            if trimmed == "." { break; }
            let mut fields = trimmed.split_whitespace();
            let number = fields.next().ok_or(MailProtocolError::InvalidResponse)?.parse().map_err(|_| MailProtocolError::InvalidResponse)?;
            let size = fields.next().ok_or(MailProtocolError::InvalidResponse)?.parse().map_err(|_| MailProtocolError::InvalidResponse)?;
            if fields.next().is_some() { return Err(MailProtocolError::InvalidResponse); }
            messages.push((number, size));
        }
        Ok(messages)
    }
}

pub struct ImapClient {
    stream: TcpStream,
    next_tag: u32,
}

impl ImapClient {
    pub fn connect(address: &str, timeout: Duration, caps: &CapabilitySet) -> Result<Self, MailProtocolError> {
        let mut stream = connect(address, timeout, caps)?;
        let greeting = read_line(&mut stream)?;
        if !greeting.starts_with("* OK") { return Err(MailProtocolError::Server(greeting.trim_end().to_string())); }
        Ok(Self { stream, next_tag: 1 })
    }

    fn tag(&mut self) -> String {
        let tag = format!("a{:03}", self.next_tag);
        self.next_tag += 1;
        tag
    }

    fn send(&mut self, command_text: &str) -> Result<(String, Vec<String>), MailProtocolError> {
        let tag = self.tag();
        self.stream.write_all(format!("{tag} {command_text}\r\n").as_bytes()).map_err(io_error)?;
        let mut untagged = Vec::new();
        loop {
            let line = read_line(&mut self.stream)?;
            if line.starts_with(&format!("{tag} ")) {
                let status = line[tag.len() + 1..].trim_end().to_string();
                return Ok((status, untagged));
            }
            untagged.push(line);
        }
    }

    pub fn login(&mut self, user: &str, password: &str) -> Result<(), MailProtocolError> {
        let (status, _) = self.send(&format!("LOGIN {user} {password}"))?;
        if status.starts_with("OK") { Ok(()) } else { Err(MailProtocolError::Server(status)) }
    }

    pub fn select(&mut self, mailbox: &str) -> Result<u32, MailProtocolError> {
        let (status, lines) = self.send(&format!("SELECT {mailbox}"))?;
        if !status.starts_with("OK") { return Err(MailProtocolError::Server(status)); }
        let mut exists = None;
        for line in lines {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 3 && fields[0] == "*" && fields[2].eq_ignore_ascii_case("EXISTS") {
                exists = Some(fields[1].parse().map_err(|_| MailProtocolError::InvalidResponse)?);
            }
        }
        exists.ok_or(MailProtocolError::InvalidResponse)
    }

    pub fn fetch_body(&mut self, sequence: u32) -> Result<Vec<u8>, MailProtocolError> {
        let tag = self.tag();
        self.stream.write_all(format!("{tag} FETCH {sequence} BODY[]\r\n").as_bytes()).map_err(io_error)?;
        let mut line = read_line(&mut self.stream)?;
        if !line.starts_with("* ") || !line.contains("FETCH") { return Err(MailProtocolError::InvalidResponse); }
        let open = line.rfind('{').ok_or(MailProtocolError::InvalidResponse)?;
        let close = line[open..].find('}').map(|offset| open + offset).ok_or(MailProtocolError::InvalidResponse)?;
        let length: usize = line[open + 1..close].parse().map_err(|_| MailProtocolError::InvalidResponse)?;
        let mut body = vec![0u8; length];
        self.stream.read_exact(&mut body).map_err(io_error)?;
        line.clear();
        line = read_line(&mut self.stream)?;
        if !line.trim_end().ends_with(')') { return Err(MailProtocolError::InvalidResponse); }
        let completion = read_line(&mut self.stream)?;
        if !completion.starts_with(&format!("{tag} OK")) { return Err(MailProtocolError::Server(completion.trim_end().to_string())); }
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mail_clients_require_network_connect_capability_before_io() {
        assert_eq!(Pop3Client::connect("127.0.0.1:1", Duration::from_millis(10), &CapabilitySet::none()), Err(MailProtocolError::CapabilityDenied));
        assert_eq!(ImapClient::connect("127.0.0.1:1", Duration::from_millis(10), &CapabilitySet::none()), Err(MailProtocolError::CapabilityDenied));
    }
}
