use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::thread;
use std::time::{Duration, Instant};

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

fn permit_listen(caps: &CapabilitySet) -> Result<(), ApplicationProtocolError> {
    if caps.permits(Capability::NetworkListen) { Ok(()) } else { Err(ApplicationProtocolError::CapabilityDenied) }
}

fn io_error(e: std::io::Error) -> ApplicationProtocolError {
    if matches!(e.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock) { ApplicationProtocolError::Timeout } else { ApplicationProtocolError::Io(e.to_string()) }
}

fn connect<A: ToSocketAddrs>(address: A, timeout: Duration, caps: &CapabilitySet) -> Result<TcpStream, ApplicationProtocolError> {
    permit(caps)?;
    let address = address.to_socket_addrs().map_err(io_error)?.next().ok_or_else(|| ApplicationProtocolError::Io("no address resolved".into()))?;
    let stream = TcpStream::connect_timeout(&address, timeout).map_err(io_error)?;
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

fn read_server_reply(stream: &mut TcpStream) -> Result<(u16, String), ApplicationProtocolError> {
    let first = read_line(stream)?;
    if first.len() < 5 { return Err(ApplicationProtocolError::InvalidResponse); }
    let code: u16 = first[..3].parse().map_err(|_| ApplicationProtocolError::InvalidResponse)?;
    let multiline = first.as_bytes()[3] == b'-';
    let mut text = first[4..].trim_end().to_string();
    if multiline {
        loop {
            let line = read_line(stream)?;
            if line.len() >= 5 && line[..3].parse::<u16>().ok() == Some(code) && line.as_bytes()[3] == b' ' {
                text.push('\n');
                text.push_str(line[4..].trim_end());
                break;
            }
            text.push('\n');
            text.push_str(line.trim_end());
        }
    }
    Ok((code, text))
}

fn ftp_command(stream: &mut TcpStream, command: &str) -> Result<(u16, String), ApplicationProtocolError> {
    stream.write_all(command.as_bytes()).map_err(io_error)?;
    stream.write_all(b"\r\n").map_err(io_error)?;
    read_server_reply(stream)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtpDataMode { Passive, Active }

pub struct FtpClient {
    control: TcpStream,
    timeout: Duration,
}

impl FtpClient {
    pub fn connect(address: &str, timeout: Duration, caps: &CapabilitySet) -> Result<Self, ApplicationProtocolError> {
        let mut control = connect(address, timeout, caps)?;
        let (code, text) = read_server_reply(&mut control)?;
        if code != 220 { return Err(ApplicationProtocolError::Server(code, text)); }
        Ok(Self { control, timeout })
    }

    pub fn login(&mut self, user: &str, password: &str) -> Result<(), ApplicationProtocolError> {
        let (code, text) = ftp_command(&mut self.control, &format!("USER {user}"))?;
        match code {
            230 => Ok(()),
            331 => {
                let (pass_code, pass_text) = ftp_command(&mut self.control, &format!("PASS {password}"))?;
                if pass_code == 230 { Ok(()) } else { Err(ApplicationProtocolError::Server(pass_code, pass_text)) }
            }
            _ => Err(ApplicationProtocolError::Server(code, text)),
        }
    }

    pub fn list(&mut self, path: Option<&str>, mode: FtpDataMode, caps: &CapabilitySet) -> Result<Vec<u8>, ApplicationProtocolError> {
        self.transfer("LIST", path, mode, None, caps)
    }

    pub fn retrieve(&mut self, path: &str, mode: FtpDataMode, caps: &CapabilitySet) -> Result<Vec<u8>, ApplicationProtocolError> {
        self.transfer("RETR", Some(path), mode, None, caps)
    }

    pub fn store(&mut self, path: &str, data: &[u8], mode: FtpDataMode, caps: &CapabilitySet) -> Result<(), ApplicationProtocolError> {
        self.transfer("STOR", Some(path), mode, Some(data), caps).map(|_| ())
    }

    pub fn quit(&mut self) -> Result<(), ApplicationProtocolError> {
        let (code, text) = ftp_command(&mut self.control, "QUIT")?;
        if code == 221 { Ok(()) } else { Err(ApplicationProtocolError::Server(code, text)) }
    }

    fn transfer(&mut self, command: &str, path: Option<&str>, mode: FtpDataMode, payload: Option<&[u8]>, caps: &CapabilitySet) -> Result<Vec<u8>, ApplicationProtocolError> {
        let data_connection = self.prepare_data(mode, caps)?;
        let command = match path { Some(path) => format!("{command} {path}"), None => command.to_string() };
        let (code, text) = ftp_command(&mut self.control, &command)?;
        if code != 125 && code != 150 { return Err(ApplicationProtocolError::Server(code, text)); }
        let mut stream = match data_connection {
            FtpDataConnection::Passive(stream) => stream,
            FtpDataConnection::Active(listener) => accept_with_timeout(listener, self.timeout)?,
        };
        let mut output = Vec::new();
        if let Some(payload) = payload {
            stream.write_all(payload).map_err(io_error)?;
            stream.shutdown(Shutdown::Write).map_err(io_error)?;
        } else {
            stream.read_to_end(&mut output).map_err(io_error)?;
        }
        drop(stream);
        let (final_code, final_text) = read_server_reply(&mut self.control)?;
        if final_code != 226 && final_code != 250 { return Err(ApplicationProtocolError::Server(final_code, final_text)); }
        Ok(output)
    }

    fn prepare_data(&mut self, mode: FtpDataMode, caps: &CapabilitySet) -> Result<FtpDataConnection, ApplicationProtocolError> {
        match mode {
            FtpDataMode::Passive => {
                permit(caps)?;
                let peer = self.control.peer_addr().map_err(io_error)?;
                let (code, text) = ftp_command(&mut self.control, "EPSV")?;
                let port = if code == 229 {
                    parse_epsv_port(&text)?
                } else if peer.ip().is_ipv4() {
                    let (pasv_code, pasv_text) = ftp_command(&mut self.control, "PASV")?;
                    if pasv_code != 227 { return Err(ApplicationProtocolError::Server(pasv_code, pasv_text)); }
                    parse_pasv_port(&pasv_text)?
                } else {
                    return Err(ApplicationProtocolError::Server(code, text));
                };
                let data_addr = SocketAddr::new(peer.ip(), port);
                let stream = TcpStream::connect_timeout(&data_addr, self.timeout).map_err(io_error)?;
                stream.set_read_timeout(Some(self.timeout)).map_err(io_error)?;
                stream.set_write_timeout(Some(self.timeout)).map_err(io_error)?;
                Ok(FtpDataConnection::Passive(stream))
            }
            FtpDataMode::Active => {
                permit(caps)?;
                permit_listen(caps)?;
                let local = self.control.local_addr().map_err(io_error)?;
                let listener = TcpListener::bind(SocketAddr::new(local.ip(), 0)).map_err(io_error)?;
                let endpoint = listener.local_addr().map_err(io_error)?;
                let (code, text) = if endpoint.ip().is_ipv4() {
                    let octets = match endpoint.ip() { std::net::IpAddr::V4(ip) => ip.octets(), _ => unreachable!() };
                    let p1 = endpoint.port() / 256;
                    let p2 = endpoint.port() % 256;
                    ftp_command(&mut self.control, &format!("PORT {},{},{},{},{},{}", octets[0], octets[1], octets[2], octets[3], p1, p2))?
                } else {
                    ftp_command(&mut self.control, &format!("EPRT |2|{}|{}|", endpoint.ip(), endpoint.port()))?
                };
                if code != 200 { return Err(ApplicationProtocolError::Server(code, text)); }
                listener.set_nonblocking(true).map_err(io_error)?;
                Ok(FtpDataConnection::Active(listener))
            }
        }
    }
}

enum FtpDataConnection { Passive(TcpStream), Active(TcpListener) }

fn accept_with_timeout(listener: TcpListener, timeout: Duration) -> Result<TcpStream, ApplicationProtocolError> {
    let deadline = Instant::now() + timeout;
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                stream.set_read_timeout(Some(timeout)).map_err(io_error)?;
                stream.set_write_timeout(Some(timeout)).map_err(io_error)?;
                return Ok(stream);
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock && Instant::now() < deadline => thread::sleep(Duration::from_millis(5)),
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => return Err(ApplicationProtocolError::Timeout),
            Err(error) => return Err(io_error(error)),
        }
    }
}

fn parse_epsv_port(text: &str) -> Result<u16, ApplicationProtocolError> {
    let start = text.rfind('(').ok_or(ApplicationProtocolError::InvalidResponse)?;
    let end = text[start..].find(')').map(|offset| start + offset).ok_or(ApplicationProtocolError::InvalidResponse)?;
    let fields = &text[start + 1..end];
    let delimiter = fields.chars().next().ok_or(ApplicationProtocolError::InvalidResponse)?;
    let parts: Vec<&str> = fields.split(delimiter).collect();
    if parts.len() != 5 || !parts[1].is_empty() || !parts[2].is_empty() || !parts[4].is_empty() { return Err(ApplicationProtocolError::InvalidResponse); }
    parts[3].parse().map_err(|_| ApplicationProtocolError::InvalidResponse)
}

fn parse_pasv_port(text: &str) -> Result<u16, ApplicationProtocolError> {
    let start = text.rfind('(').ok_or(ApplicationProtocolError::InvalidResponse)?;
    let end = text[start..].find(')').map(|offset| start + offset).ok_or(ApplicationProtocolError::InvalidResponse)?;
    let values: Vec<u16> = text[start + 1..end].split(',').map(|value| value.trim().parse().map_err(|_| ApplicationProtocolError::InvalidResponse)).collect::<Result<_, _>>()?;
    if values.len() != 6 || values[..4].iter().any(|value| *value > 255) || values[4] > 255 || values[5] > 255 { return Err(ApplicationProtocolError::InvalidResponse); }
    Ok(values[4] * 256 + values[5])
}

pub fn ftp_login(address: &str, user: &str, password: &str, timeout: Duration, caps: &CapabilitySet) -> Result<(), ApplicationProtocolError> {
    let mut client = FtpClient::connect(address, timeout, caps)?;
    client.login(user, password)
}

pub fn smtp_send(address: &str, sender: &str, recipient: &str, subject: &str, body: &str, timeout: Duration, caps: &CapabilitySet) -> Result<(), ApplicationProtocolError> {
    let mut stream = connect(address, timeout, caps)?;
    let (code, _) = read_server_reply(&mut stream)?;
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
    let (code, text) = read_server_reply(&mut stream)?;
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
    fn ftp_passive_retr_reads_data_channel_and_final_reply() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        let handle = thread::spawn(move || {
            let (mut control, _) = listener.accept().expect("accept control");
            control.write_all(b"220 ready\r\n").expect("greeting");
            let mut line = String::new();
            let mut byte = [0u8; 1];
            while control.read_exact(&mut byte).is_ok() { line.push(byte[0] as char); if line.ends_with("\r\n") { break; } }
            assert_eq!(line, "EPSV\r\n");
            let data_listener = TcpListener::bind("127.0.0.1:0").expect("bind data");
            let data_port = data_listener.local_addr().expect("data addr").port();
            control.write_all(format!("229 Entering Extended Passive Mode (|||{data_port}|)\r\n").as_bytes()).expect("epsv");
            line.clear();
            while control.read_exact(&mut byte).is_ok() { line.push(byte[0] as char); if line.ends_with("\r\n") { break; } }
            assert_eq!(line, "RETR hello.txt\r\n");
            control.write_all(b"150 opening\r\n").expect("preliminary");
            let (mut data, _) = data_listener.accept().expect("data accept");
            data.write_all(b"hello\n").expect("payload");
            drop(data);
            control.write_all(b"226 done\r\n").expect("final");
        });
        let mut client = FtpClient::connect(&addr.to_string(), Duration::from_secs(2), &caps()).expect("connect");
        let data = client.retrieve("hello.txt", FtpDataMode::Passive, &caps()).expect("retrieve");
        assert_eq!(data, b"hello\n");
        handle.join().expect("server");
    }

    #[test]
    fn ftp_active_store_requires_listen_capability() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let addr = listener.local_addr().expect("addr");
        let handle = thread::spawn(move || {
            let (mut control, _) = listener.accept().expect("accept control");
            control.write_all(b"220 ready\r\n").expect("greeting");
        });
        let mut client = FtpClient::connect(&addr.to_string(), Duration::from_secs(2), &caps()).expect("connect");
        assert_eq!(client.store("hello.txt", b"hello", FtpDataMode::Active, &caps()), Err(ApplicationProtocolError::CapabilityDenied));
        handle.join().expect("server");
    }

    #[test]
    fn ftp_passive_parser_rejects_malformed_response() {
        assert_eq!(parse_pasv_port("227 bad"), Err(ApplicationProtocolError::InvalidResponse));
        assert_eq!(parse_epsv_port("229 bad"), Err(ApplicationProtocolError::InvalidResponse));
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
            let message = read_test_until(&mut stream, b"\r\n.\r\n");
            assert!(String::from_utf8_lossy(&message).contains("Subject: test"));
            stream.write_all(b"250 queued\r\n").expect("queued");
            assert_eq!(read_test_line(&mut stream), b"QUIT\r\n");
            stream.write_all(b"221 bye\r\n").expect("quit reply");
        });
        smtp_send(&addr.to_string(), "a@example.test", "b@example.test", "test", "hello", Duration::from_secs(2), &caps()).expect("smtp");
        handle.join().expect("server");
    }
}