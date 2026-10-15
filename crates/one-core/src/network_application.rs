use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::capability::{Capability, CapabilitySet};

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
    let mut text = first[4..].trim_end().to_string();
    if multiline {
        loop {
            let line = read_line(stream)?;
            if line.len() >= 5 && line[..3].parse::<u16>().ok() == Some(code) && line.as_bytes()[3] == b' ' { text.push('\n'); text.push_str(line[4..].trim_end()); break; }
            text.push('\n'); text.push_str(line.trim_end());
        }
    }
    Ok((code, text))
}

fn ftp_command(stream: &mut TcpStream, command: &str) -> Result<(u16, String), ApplicationProtocolError> {
    stream.write_all(command.as_bytes()).map_err(io_error)?;
    stream.write_all(b"\r\n").map_err(io_error)?;
    read_ftp_reply(stream)
}

pub fn ftp_login(address: &str, user: &str, password: &str, timeout: Duration, caps: &CapabilitySet) -> Result<(), ApplicationProtocolError> {
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
    let mut stream = connect(address, timeout, caps)?;
    let (code, _) = read_ftp_reply(&mut stream)?;
    if code != 220 { return Err(ApplicationProtocolError::Server(code, "SMTP greeting rejected".into())); }
    for (expected, command) in [(250, "EHLO one".to_string()), (250, format!("MAIL FROM:<{sender}>")), (250, format!("RCPT TO:<{recipient}>"))] {
        let (code, text) = ftp_command(&mut stream, &command)?;
        if code != expected { return Err(ApplicationProtocolError::Server(code, text)); }
    }
    let (code, _) = ftp_command(&mut stream, "DATA")?;
    if code != 354 { return Err(ApplicationProtocolError::Server(code, "SMTP DATA rejected".into())); }
    let normalized = body.replace('\r', "").replace('\n', "\r\n").replace("\r\n.", "\r\n..");
    let message = format!("From: <{sender}>\r\nTo: <{recipient}>\r\nSubject: {subject}\r\n\r\n{normalized}\r\n.\r\n");
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

    #[test]
    fn ftp_login_speaks_real_control_channel() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            stream.write_all(b"220 ready\r\n").expect("greeting");
            let mut buf = [0u8; 256];
            stream.read(&mut buf).expect("user");
            assert!(String::from_utf8_lossy(&buf).starts_with("USER alice\r\n"));
            stream.write_all(b"331 password\r\n").expect("user reply");
            let n = stream.read(&mut buf).expect("pass");
            assert_eq!(&buf[..n], b"PASS secret\r\n");
            stream.write_all(b"230 logged in\r\n").expect("pass reply");
        });
        ftp_login(&addr.to_string(), "alice", "secret", Duration::from_secs(2), &caps()).expect("ftp login");
        handle.join().expect("server");
    }

    #[test]
    fn smtp_send_emits_real_smtp_transaction() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept");
            stream.write_all(b"220 ready\r\n").expect("greeting");
            let mut buf = [0u8; 4096];
            for reply in [b"250 hello\r\n".as_slice(), b"250 sender\r\n".as_slice(), b"250 recipient\r\n".as_slice()] {
                let _ = stream.read(&mut buf).expect("command");
                stream.write_all(reply).expect("reply");
            }
            let n = stream.read(&mut buf).expect("data");
            assert!(String::from_utf8_lossy(&buf[..n]).contains("Subject: test"));
            stream.write_all(b"250 queued\r\n").expect("queued");
            let _ = stream.read(&mut buf);
        });
        smtp_send(&addr.to_string(), "a@example.test", "b@example.test", "test", "hello", Duration::from_secs(2), &caps()).expect("smtp");
        handle.join().expect("server");
    }
}
