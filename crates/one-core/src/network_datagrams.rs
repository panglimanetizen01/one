use std::net::SocketAddr;
use std::time::Duration;

use crate::capability::{Capability, CapabilitySet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatagramProtocolError {
    CapabilityDenied,
    InvalidPacket,
    Io(String),
    Timeout,
    ServerError(u16, String),
}

pub fn tftp_read(socket: &std::net::UdpSocket, server: SocketAddr, filename: &str, timeout: Duration, capabilities: &CapabilitySet) -> Result<Vec<u8>, DatagramProtocolError> {
    todo!("TFTP read wire implementation")
}

pub fn tftp_write(socket: &std::net::UdpSocket, server: SocketAddr, filename: &str, data: &[u8], timeout: Duration, capabilities: &CapabilitySet) -> Result<(), DatagramProtocolError> {
    todo!("TFTP write wire implementation")
}

pub fn ntp_query(socket: &std::net::UdpSocket, server: SocketAddr, timeout: Duration, capabilities: &CapabilitySet) -> Result<u64, DatagramProtocolError> {
    todo!("NTP query wire implementation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::UdpSocket;
    use std::thread;

    fn caps() -> CapabilitySet { CapabilitySet::none().allow(Capability::NetworkConnect) }

    #[test]
    fn tftp_read_decodes_real_rrq_data_and_ack_exchange() {
        let server = UdpSocket::bind("127.0.0.1:0").expect("bind");
        let server_addr = server.local_addr().expect("addr");
        let handle = thread::spawn(move || {
            let mut buf = [0u8; 1024];
            let (n, peer) = server.recv_from(&mut buf).expect("rrq");
            assert_eq!(&buf[..2], b"\0\1");
            assert_eq!(&buf[2..n], b"hello.txt\0octet\0");
            let mut packet = Vec::new();
            packet.extend_from_slice(&[0, 3, 0, 1]);
            packet.extend_from_slice(b"hello");
            server.send_to(&packet, peer).expect("data");
            let (n, _) = server.recv_from(&mut buf).expect("ack");
            assert_eq!(&buf[..n], &[0, 4, 0, 1]);
        });
        let socket = UdpSocket::bind("127.0.0.1:0").expect("bind");
        let data = tftp_read(&socket, server_addr, "hello.txt", Duration::from_secs(2), &caps()).expect("tftp read");
        assert_eq!(data, b"hello");
        handle.join().expect("server");
    }

    #[test]
    fn tftp_write_emits_real_wrq_data_and_waits_for_ack() {
        let server = UdpSocket::bind("127.0.0.1:0").expect("bind");
        let server_addr = server.local_addr().expect("addr");
        let handle = thread::spawn(move || {
            let mut buf = [0u8; 1024];
            let (n, peer) = server.recv_from(&mut buf).expect("wrq");
            assert_eq!(&buf[..2], b"\0\2");
            assert_eq!(&buf[2..n], b"hello.txt\0octet\0");
            server.send_to(&[0, 4, 0, 0], peer).expect("ack0");
            let (n, _) = server.recv_from(&mut buf).expect("data");
            assert_eq!(&buf[..n], &[0, 3, 0, 1, b'h', b'e', b'l', b'l', b'o']);
        });
        let socket = UdpSocket::bind("127.0.0.1:0").expect("bind");
        tftp_write(&socket, server_addr, "hello.txt", b"hello", Duration::from_secs(2), &caps()).expect("tftp write");
        handle.join().expect("server");
    }

    #[test]
    fn ntp_query_parses_real_transmit_timestamp() {
        let server = UdpSocket::bind("127.0.0.1:0").expect("bind");
        let server_addr = server.local_addr().expect("addr");
        let handle = thread::spawn(move || {
            let mut request = [0u8; 48];
            let (_, peer) = server.recv_from(&mut request).expect("ntp request");
            assert_eq!(request[0] & 0x07, 3);
            let mut response = [0u8; 48];
            response[0] = 0x24;
            response[40..48].copy_from_slice(&1_700_000_000u64.to_be_bytes());
            server.send_to(&response, peer).expect("ntp response");
        });
        let socket = UdpSocket::bind("127.0.0.1:0").expect("bind");
        let timestamp = ntp_query(&socket, server_addr, Duration::from_secs(2), &caps()).expect("ntp");
        assert_eq!(timestamp, 1_700_000_000);
        handle.join().expect("server");
    }
}
