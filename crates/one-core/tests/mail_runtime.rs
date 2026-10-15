use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use one_core::{Capability, CapabilitySet, ImapClient, Pop3Client};

fn caps() -> CapabilitySet {
    CapabilitySet::none().allow(Capability::NetworkConnect)
}

#[test]
fn pop3_authenticates_and_lists_messages() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");
        stream.write_all(b"+OK ready\r\n").expect("greeting");
        let mut buf = [0u8; 256];
        let _ = stream.read(&mut buf).expect("USER");
        stream.write_all(b"+OK user\r\n").expect("USER reply");
        let _ = stream.read(&mut buf).expect("PASS");
        stream.write_all(b"+OK logged in\r\n").expect("PASS reply");
        let _ = stream.read(&mut buf).expect("LIST");
        stream.write_all(b"+OK 2 messages\r\n1 42\r\n2 84\r\n.\r\n").expect("LIST reply");
    });

    let mut client = Pop3Client::connect(&addr.to_string(), Duration::from_secs(2), &caps()).expect("connect");
    client.login("alice", "secret").expect("login");
    assert_eq!(client.list().expect("list"), vec![(1, 42), (2, 84)]);
    server.join().expect("server");
}

#[test]
fn imap_authenticates_selects_and_fetches_message() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().expect("addr");
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");
        stream.write_all(b"* OK ready\r\n").expect("greeting");
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf).expect("LOGIN");
        stream.write_all(b"a001 OK LOGIN completed\r\n").expect("LOGIN reply");
        let _ = stream.read(&mut buf).expect("SELECT");
        stream.write_all(b"* 2 EXISTS\r\na002 OK SELECT completed\r\n").expect("SELECT reply");
        let _ = stream.read(&mut buf).expect("FETCH");
        stream.write_all(b"* 1 FETCH (BODY[] {5}\r\nhello)\r\na003 OK FETCH completed\r\n").expect("FETCH reply");
    });

    let mut client = ImapClient::connect(&addr.to_string(), Duration::from_secs(2), &caps()).expect("connect");
    client.login("alice", "secret").expect("login");
    assert_eq!(client.select("INBOX").expect("select"), 2);
    assert_eq!(client.fetch_body(1).expect("fetch"), b"hello");
    server.join().expect("server");
}
