use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::capability::{Capability, CapabilitySet};

const MAX_APPLICATION_REPLY_BYTES: usize = 16 * 1024 * 1024;
const MAX_APPLICATION_REPLY_LINES: usize = 100_000;
const MAX_APPLICATION_FIELD_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationProtocolError {
    CapabilityDenied,
    InvalidResponse,
    Io(String),
    Timeout,
    Server(u16, String),
}

fn permit(caps: &CapabilitySet) -> Result<(), ApplicationProtocolError> {
    if caps.permits(Capability::NetworkConnect) { Ok(()) } else { Err(ApplicationProtocolError::CapabilityDenied) }
}

fn io_error(e: std::io::Error) -> ApplicationProtocolError {
    if matches!(e.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock) { ApplicationProtocolError::Timeout } else { ApplicationProtocolError::Io(e.to_string()) }
}

fn connect<A: ToSocketAddrs>(address: A, timeout: Duration, caps: &CapabilitySet) -> Result<TcpStream, ApplicationProtocolError> {
    permit(caps)?;
    let stream = TcpStream::connect_timeout(&address.to_socket_addrs().map_err(io_error)?.next().ok_or_else(|| ApplicationProtocolError::Io("no address resolved".into()))?, timeout).map_err(io_error)?;
    stream.set_read_timeout(Some(timeout)).map_err(io_error)?;
    stream.set_write_timeout(Some(timeout)).map_err(io_error)?;
    Ok(stream)
}

fn read_line(stream: &mut TcpStream) -> Result<String, ApplicationProtocolError> {
    let mut data = Vec::new();
    let mut byte = [0u8; 1];
    loop {
        let n = stream.read(&mut byte).map_err(io_error)?;
        if n == 0 { return Err(ApplicationProtocolError::InvalidResponse); }
        data.push(byte[0]);
        if data.ends_with(b"\r\n") { break; }
        if data.len() > 64 * 1024 { return Err(ApplicationProtocolError::InvalidResponse); }
    }
    String::from_utf8(data).map_err(|_| ApplicationProtocolError::InvalidResponse)
}

fn read_ftp_reply(stream: &mut TcpStream) -> Result<(u16, String), ApplicationProtocolError> {
    let first = read_line(stream)?;
    if first.len() < 5 { return Err(ApplicationProtocolError::InvalidResponse); }
    let code: u16 = first[..3].parse().map_err(|_| ApplicationProtocolError::InvalidResponse)?;
    let multiline = first.as_bytes()[3] == b'-';
    let mut total_bytes = first.len();
    let mut line_count = 1usize;
    let mut text = first[4..].trim_end().to_string();
    if multiline {
        loop {
            let line = read_line(stream)?;
            total_bytes = total_bytes.checked_add(line.len()).ok_or(ApplicationProtocolError::InvalidResponse)?;
            line_count = line_count.checked_add(1).ok_or(ApplicationProtocolError::InvalidResponse)?;
            if total_bytes > MAX_APPLICATION_REPLY_BYTES || line_count > MAX_APPLICATION_REPLY_LINES {
                return Err(ApplicationProtocolError::InvalidResponse);
            }
            if line.len() >= 5 && line[..3].parse::<u16>().ok() == Some(code) && line.as_bytes()[3] == b' ' { text.push('\n'); text.push_str(line[4..].trim_end()); break; }
            text.push('\n'); text.push_str(line.trim_end());
        }
    }
    Ok((code, text))
}

fn validate_command_field(value: &str) -> Result<(), ApplicationProtocolError> {
    if value.len() > MAX_APPLICATION_FIELD_BYTES
        || value.bytes().any(|byte| byte < 0x20 || byte == 0x7f)
    {
        Err(ApplicationProtocolError::InvalidResponse)
    } else {
        Ok(())
    }
}

fn ftp_command(stream: &mut TcpStream, command: &str) -> Result<(u16, String), ApplicationProtocolError> {
    validate_command_field(command)?;
    stream.write_all(command.as_bytes()).map_err(io_error)?;
    stream.write_all(b"\r\n").map_err(io_error)?;
    read_ftp_reply(stream)
}

pub fn ftp_login(address: &str, user: &str, password: &str, timeout: Duration, caps: &CapabilitySet) -> Result<(), ApplicationProtocolError> {
    validate_command_field(user)?;
    validate_command_field(password)?;
    if user.is_empty() { return Err(ApplicationProtocolError::InvalidResponse); }
    let mut stream = connect(address, timeout, caps)?;
    let (code, _) = read_ftp_reply(&mut stream)?;
    if code / 100 != 2 { return Err(ApplicationProtocolError::Server(code, "FTP greeting rejected".into())); }
    let (code, _) = ftp_command(&mut stream, &format!("USER {user}"))?;
    if code / 100 != 3 && code / 100 != 2 { return Err(ApplicationProtocolError::Server(code, "FTP USER rejected".into())); }
    let (code, _) = ftp_command(&mut stream, &format!("PASS {password}"))?;
    if code / 100 != 2 { return Err(ApplicationProtocolError::Server(code, "FTP authentication rejected".into())); }
    Ok(())
}

pub fn smtp_send(address: &str, sender: &str, recipient: &str, subject: &str, body: &str, timeout: Duration, caps: &CapabilitySet) -> Result<(), ApplicationProtocolError> {
    validate_command_field(sender)?;
    validate_command_field(recipient)?;
    validate_command_field(subject)?;
    if sender.is_empty() || recipient.is_empty() {
        return Err(ApplicationProtocolError::InvalidResponse);
    }
    if body.len() > MAX_APPLICATION_REPLY_BYTES
        || body.bytes().any(|byte| byte == 0 || byte == 0x7f || (byte < 0x20 && !matches!(byte, b'\r' | b'\n' | b'\t')))
    {
        return Err(ApplicationProtocolError::InvalidResponse);
    }
    let mut stream = connect(address, timeout, caps)?;
    let (code, _) = read_ftp_reply(&mut stream)?;
    if code != 220 { return Err(ApplicationProtocolError::Server(code, "SMTP greeting rejected".into())); }
    for (expected, command) in [(250, "EHLO one".to_string()), (250, format!("MAIL FROM:<{sender}>")), (250, format!("RCPT TO:<{recipient}>"))] {
        let (code, text) = ftp_command(&mut stream, &command)?;
        if code != expected { return Err(ApplicationProtocolError::Server(code, text)); }
    }
    let (code, _) = ftp_command(&mut stream, "DATA")?;
    if code != 354 { return Err(ApplicationProtocolError::Server(code, "SMTP DATA rejected".into())); }
    let normalized = body.replace('\r', "").replace('\n', "\r\n");
    let stuffed = normalized
        .split_inclusive("\r\n")
        .map(|line| if line.starts_with('.') { format!(".{line}") } else { line.to_string() })
        .collect::<String>();
    let message = format!("From: <{sender}>\r\nTo: <{recipient}>\r\nSubject: {subject}\r\n\r\n{stuffed}\r\n.\r\n");
    if message.len() > MAX_APPLICATION_REPLY_BYTES {
        return Err(ApplicationProtocolError::InvalidResponse);
    }
    stream.write_all(message.as_bytes()).map_err(io_error)?;
    let (code, text) = read_ftp_reply(&mut stream)?;
    if code != 250 { return Err(ApplicationProtocolError::Server(code, text)); }
    let _ = ftp_command(&mut stream, "QUIT");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;

    fn caps() -> CapabilitySet { CapabilitySet::none().allow(Capability::NetworkConnect) }

    fn read_test_line(stream: &mut TcpStream) -> Vec<u8> {
        let mut line = Vec::new();
        let mut byte = [0u8; 1];
        loop {
            stream.read_exact(&mut byte).expect("line");
            line.push(byte[0]);
            if line.ends_with(b"\r\n") { return line; }
        }
    }

    fn read_test_until(stream: &mut TcpStream, marker: &[u8]) -> Vec<u8> {
        let mut data = Vec::new();
        let mut byte = [0u8; 1];
        while !data.ends_with(marker) {
            stream.read_exact(&mut byte).expect("data");
            data.push(byte[0]);
        }
        data
    }

    #[test]
    fn ftp_login_speaks_real_control_channel() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            stream.write_all(b"220 ready\r\n").expect("greeting");
            assert_eq!(read_test_line(&mut stream), b"USER alice\r\n");
            stream.write_all(b"331 password\r\n").expect("user reply");
            assert_eq!(read_test_line(&mut stream), b"PASS secret\r\n");
            stream.write_all(b"230 logged in\r\n").expect("pass reply");
        });
        ftp_login(&addr.to_string(), "alice", "secret", Duration::from_secs(2), &caps()).expect("ftp login");
        handle.join().expect("server");
    }

    #[test]
    fn ftp_login_rejects_command_injection_before_connecting() {
        let result = ftp_login("127.0.0.1:1", "alice\r\nNOOP", "secret", Duration::from_millis(10), &caps());
        assert!(matches!(result, Err(ApplicationProtocolError::InvalidResponse)));
    }

    #[test]
    fn smtp_send_rejects_header_injection_before_connecting() {
        let result = smtp_send("127.0.0.1:1", "a@example.test", "b@example.test", "safe\r\nBcc: attacker@example.test", "hello", Duration::from_millis(10), &caps());
        assert!(matches!(result, Err(ApplicationProtocolError::InvalidResponse)));
    }

    #[test]
    fn smtp_send_emits_real_smtp_transaction() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            stream.write_all(b"220 ready\r\n").expect("greeting");
            for reply in [b"250 hello\r\n".as_slice(), b"250 sender\r\n".as_slice(), b"250 recipient\r\n".as_slice()] {
                let _ = read_test_line(&mut stream);
                stream.write_all(reply).expect("reply");
            }
            assert_eq!(read_test_line(&mut stream), b"DATA\r\n");
            stream.write_all(b"354 continue\r\n").expect("data reply");
            let data = read_test_until(&mut stream, b"\r\n.\r\n");
            assert!(String::from_utf8_lossy(&data).contains("Subject: test"));
            stream.write_all(b"250 queued\r\n").expect("queued");
            assert_eq!(read_test_line(&mut stream), b"QUIT\r\n");
        });
        smtp_send(&addr.to_string(), "a@example.test", "b@example.test", "test", "hello", Duration::from_secs(2), &caps()).expect("smtp");
        handle.join().expect("server");
    }
}
