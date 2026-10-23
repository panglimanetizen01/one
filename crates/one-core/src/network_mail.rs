use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::capability::{Capability, CapabilitySet};

const MAX_MAIL_LINE: usize = 64 * 1024;
const MAX_IMAP_BODY: usize = 16 * 1024 * 1024;
const MAX_MAIL_RESPONSE_BYTES: usize = 16 * 1024 * 1024;
const MAX_MAIL_RESPONSE_LINES: usize = 100_000;

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

fn validate_command_field(value: &str) -> Result<(), MailProtocolError> {
    if value.bytes().any(|byte| byte < 0x20 || byte == 0x7f) {
        Err(MailProtocolError::InvalidResponse)
    } else {
        Ok(())
    }
}

fn validate_pop3_username(value: &str) -> Result<(), MailProtocolError> {
    validate_command_field(value)?;
    if value.is_empty() || value.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return Err(MailProtocolError::InvalidResponse);
    }
    Ok(())
}

fn quote_imap_string(value: &str) -> Result<String, MailProtocolError> {
    validate_command_field(value)?;
    let mut quoted = String::with_capacity(value.len().saturating_add(2));
    quoted.push('"');
    for character in value.chars() {
        if character == '"' || character == '\\' {
            quoted.push('\\');
        }
        quoted.push(character);
    }
    quoted.push('"');
    Ok(quoted)
}

fn read_line(stream: &mut TcpStream) -> Result<String, MailProtocolError> {
    let mut bytes = Vec::new();
    let mut byte = [0u8; 1];
    loop {
        stream.read_exact(&mut byte).map_err(io_error)?;
        bytes.push(byte[0]);
        if bytes.ends_with(b"\r\n") { break; }
        if bytes.len() > MAX_MAIL_LINE { return Err(MailProtocolError::InvalidResponse); }
    }
    String::from_utf8(bytes).map_err(|_| MailProtocolError::InvalidResponse)
}

fn command(stream: &mut TcpStream, command: &str) -> Result<String, MailProtocolError> {
    validate_command_field(command)?;
    stream.write_all(command.as_bytes()).map_err(io_error)?;
    stream.write_all(b"\r\n").map_err(io_error)?;
    read_line(stream)
}

fn require_ok(line: &str, prefix: &str) -> Result<(), MailProtocolError> {
    if line.starts_with(prefix) { Ok(()) } else { Err(MailProtocolError::Server(line.trim_end().to_string())) }
}

pub struct Pop3Client { stream: TcpStream }

impl Pop3Client {
    pub fn connect(address: &str, timeout: Duration, caps: &CapabilitySet) -> Result<Self, MailProtocolError> {
        let mut stream = connect(address, timeout, caps)?;
        let greeting = read_line(&mut stream)?;
        require_ok(&greeting, "+OK")?;
        Ok(Self { stream })
    }

    pub fn login(&mut self, user: &str, password: &str) -> Result<(), MailProtocolError> {
        validate_pop3_username(user)?;
        validate_command_field(password)?;
        require_ok(&command(&mut self.stream, &format!("USER {user}"))?, "+OK")?;
        require_ok(&command(&mut self.stream, &format!("PASS {password}"))?, "+OK")
    }

    pub fn list(&mut self) -> Result<Vec<(u32, u64)>, MailProtocolError> {
        let first = command(&mut self.stream, "LIST")?;
        require_ok(&first, "+OK")?;
        let mut messages = Vec::new();
        let mut response_bytes = first.len();
        loop {
            let line = read_line(&mut self.stream)?;
            response_bytes = response_bytes.checked_add(line.len()).ok_or(MailProtocolError::InvalidResponse)?;
            if response_bytes > MAX_MAIL_RESPONSE_BYTES || messages.len() >= MAX_MAIL_RESPONSE_LINES {
                return Err(MailProtocolError::InvalidResponse);
            }
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

pub struct ImapClient { stream: TcpStream, next_tag: u32 }

impl ImapClient {
    pub fn connect(address: &str, timeout: Duration, caps: &CapabilitySet) -> Result<Self, MailProtocolError> {
        let mut stream = connect(address, timeout, caps)?;
        let greeting = read_line(&mut stream)?;
        if !greeting.starts_with("* OK") { return Err(MailProtocolError::Server(greeting.trim_end().to_string())); }
        Ok(Self { stream, next_tag: 1 })
    }

    fn tag(&mut self) -> Result<String, MailProtocolError> {
        let current = self.next_tag;
        self.next_tag = self.next_tag.checked_add(1).ok_or(MailProtocolError::InvalidResponse)?;
        Ok(format!("a{:03}", current))
    }

    fn send(&mut self, command_text: &str) -> Result<(String, Vec<String>), MailProtocolError> {
        validate_command_field(command_text)?;
        let tag = self.tag()?;
        self.stream.write_all(format!("{tag} {command_text}\r\n").as_bytes()).map_err(io_error)?;
        let mut untagged = Vec::new();
        let mut response_bytes = 0usize;
        loop {
            let line = read_line(&mut self.stream)?;
            response_bytes = response_bytes.checked_add(line.len()).ok_or(MailProtocolError::InvalidResponse)?;
            if response_bytes > MAX_MAIL_RESPONSE_BYTES || untagged.len() >= MAX_MAIL_RESPONSE_LINES {
                return Err(MailProtocolError::InvalidResponse);
            }
            if line.starts_with(&format!("{tag} ")) {
                return Ok((line[tag.len() + 1..].trim_end().to_string(), untagged));
            }
            untagged.push(line);
        }
    }

    pub fn login(&mut self, user: &str, password: &str) -> Result<(), MailProtocolError> {
        let user = quote_imap_string(user)?;
        let password = quote_imap_string(password)?;
        let (status, _) = self.send(&format!("LOGIN {user} {password}"))?;
        if status.starts_with("OK") { Ok(()) } else { Err(MailProtocolError::Server(status)) }
    }

    pub fn select(&mut self, mailbox: &str) -> Result<u32, MailProtocolError> {
        let mailbox = quote_imap_string(mailbox)?;
        let (status, lines) = self.send(&format!("SELECT {mailbox}"))?;
        if !status.starts_with("OK") { return Err(MailProtocolError::Server(status)); }
        for line in lines {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 3 && fields[0] == "*" && fields[2].eq_ignore_ascii_case("EXISTS") {
                return fields[1].parse().map_err(|_| MailProtocolError::InvalidResponse);
            }
        }
        Err(MailProtocolError::InvalidResponse)
    }

    pub fn fetch_body(&mut self, sequence: u32) -> Result<Vec<u8>, MailProtocolError> {
        let tag = self.tag();
        self.stream.write_all(format!("{tag} FETCH {sequence} BODY[]\r\n").as_bytes()).map_err(io_error)?;
        let line = read_line(&mut self.stream)?;
        if !line.starts_with("* ") || !line.contains("FETCH") { return Err(MailProtocolError::InvalidResponse); }
        let open = line.rfind('{').ok_or(MailProtocolError::InvalidResponse)?;
        let close = line[open..].find('}').map(|offset| open + offset).ok_or(MailProtocolError::InvalidResponse)?;
        let length: usize = line[open + 1..close].parse().map_err(|_| MailProtocolError::InvalidResponse)?;
        if length > MAX_IMAP_BODY { return Err(MailProtocolError::InvalidResponse); }
        let mut body = vec![0u8; length];
        self.stream.read_exact(&mut body).map_err(io_error)?;
        if !read_line(&mut self.stream)?.trim_end().ends_with(')') { return Err(MailProtocolError::InvalidResponse); }
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
        assert!(matches!(Pop3Client::connect("127.0.0.1:1", Duration::from_millis(10), &CapabilitySet::none()), Err(MailProtocolError::CapabilityDenied)));
        assert!(matches!(ImapClient::connect("127.0.0.1:1", Duration::from_millis(10), &CapabilitySet::none()), Err(MailProtocolError::CapabilityDenied)));
    }

    #[test]
    fn imap_arguments_are_quoted_and_escape_protocol_metacharacters() {
        assert_eq!(quote_imap_string("safe mailbox").expect("quoted"), "\"safe mailbox\"");
        assert_eq!(quote_imap_string("a\"b\\c").expect("escaped"), "\"a\\\"b\\\\c\"");
        assert!(quote_imap_string("bad\r\nLOGIN attacker").is_err());
        assert!(validate_pop3_username("user name").is_err());
    }

    #[test]
    fn mail_commands_reject_crlf_injection() {
        assert!(matches!(validate_command_field("safe\r\nNOOP"), Err(MailProtocolError::InvalidResponse)));
        assert!(validate_command_field("safe value").is_ok());
    }
}
