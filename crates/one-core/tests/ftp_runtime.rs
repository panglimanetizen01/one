use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use one_core::{Capability, CapabilitySet, FtpClient, FtpDataMode};

fn caps() -> CapabilitySet {
    CapabilitySet::none()
        .allow(Capability::NetworkConnect)
        .allow(Capability::NetworkListen)
}

#[test]
fn ftp_active_store_completes_real_data_channel() {
    let control_listener = TcpListener::bind("127.0.0.1:0").expect("bind control");
    let control_addr = control_listener.local_addr().expect("control addr");
    let server = thread::spawn(move || {
        let (mut control, _) = control_listener.accept().expect("accept control");
        control.write_all(b"220 ready\r\n").expect("greeting");

        let mut line = Vec::new();
        let mut byte = [0u8; 1];
        loop {
            control.read_exact(&mut byte).expect("command");
            line.push(byte[0]);
            if line.ends_with(b"\r\n") { break; }
        }
        let command = String::from_utf8(std::mem::take(&mut line)).expect("utf8");
        assert!(command.starts_with("PORT 127,0,0,1,"));
        let port_fields: Vec<u16> = command.trim_end_matches("\r\n").split(' ').nth(1).expect("port args").split(',').map(|v| v.parse().expect("port field")).collect();
        assert_eq!(port_fields.len(), 6);
        let data_port = port_fields[4] * 256 + port_fields[5];
        control.write_all(b"200 PORT command okay\r\n").expect("port reply");

        line.clear();
        loop {
            control.read_exact(&mut byte).expect("transfer command");
            line.push(byte[0]);
            if line.ends_with(b"\r\n") { break; }
        }
        assert_eq!(line, b"STOR hello.txt\r\n");
        control.write_all(b"150 opening\r\n").expect("preliminary");

        let mut data = TcpStream::connect(("127.0.0.1", data_port)).expect("active data connection");
        let mut payload = Vec::new();
        data.read_to_end(&mut payload).expect("read data");
        assert_eq!(payload, b"hello");
        control.write_all(b"226 done\r\n").expect("final");
    });

    let mut client = FtpClient::connect(&control_addr.to_string(), Duration::from_secs(2), &caps()).expect("client");
    client.store("hello.txt", b"hello", FtpDataMode::Active, &caps()).expect("active store");
    server.join().expect("server");
}
