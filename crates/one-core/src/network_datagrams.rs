use std::net::{SocketAddr, UdpSocket};
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

fn permit(capabilities: &CapabilitySet) -> Result<(), DatagramProtocolError> {
    if capabilities.permits(Capability::NetworkConnect) { Ok(()) } else { Err(DatagramProtocolError::CapabilityDenied) }
}

fn timeout(error: std::io::Error) -> DatagramProtocolError {
    if matches!(error.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock) { DatagramProtocolError::Timeout } else { DatagramProtocolError::Io(error.to_string()) }
}

fn tftp_request(opcode: u16, filename: &str) -> Result<Vec<u8>, DatagramProtocolError> {
    if filename.is_empty() || filename.as_bytes().contains(&0) { return Err(DatagramProtocolError::InvalidPacket); }
    let mut packet = Vec::with_capacity(filename.len() + 9);
    packet.extend_from_slice(&opcode.to_be_bytes());
    packet.extend_from_slice(filename.as_bytes());
    packet.push(0);
    packet.extend_from_slice(b"octet");
    packet.push(0);
    Ok(packet)
}

fn parse_tftp_packet(packet: &[u8]) -> Result<(u16, u16, &[u8]), DatagramProtocolError> {
    if packet.len() < 4 { return Err(DatagramProtocolError::InvalidPacket); }
    let opcode = u16::from_be_bytes([packet[0], packet[1]]);
    let value = u16::from_be_bytes([packet[2], packet[3]]);
    Ok((opcode, value, &packet[4..]))
}

fn tftp_error(payload: &[u8]) -> DatagramProtocolError {
    let code = if payload.len() >= 2 { u16::from_be_bytes([payload[0], payload[1]]) } else { 0 };
    let message = String::from_utf8(payload.get(2..).unwrap_or_default().split(|b| *b == 0).next().unwrap_or_default().to_vec()).unwrap_or_else(|_| "TFTP server error".into());
    DatagramProtocolError::ServerError(code, message)
}

pub fn tftp_read(socket: &UdpSocket, server: SocketAddr, filename: &str, timeout: Duration, capabilities: &CapabilitySet) -> Result<Vec<u8>, DatagramProtocolError> {
    permit(capabilities)?;
    socket.set_read_timeout(Some(timeout)).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
    socket.set_write_timeout(Some(timeout)).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
    let request = tftp_request(1, filename)?;
    socket.send_to(&request, server).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
    let mut expected_block = 1u16;
    let mut output = Vec::new();
    let mut buffer = [0u8; 4 + 512];
    loop {
        let (size, from) = socket.recv_from(&mut buffer).map_err(timeout)?;
        let peer = from;
        let (opcode, block, payload) = parse_tftp_packet(&buffer[..size])?;
        match opcode {
            3 if block == expected_block => {
                output.extend_from_slice(payload);
                let ack = [0, 4, (block >> 8) as u8, block as u8];
                socket.send_to(&ack, peer).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
                if payload.len() < 512 { return Ok(output); }
                expected_block = expected_block.wrapping_add(1);
            }
            5 => return Err(tftp_error(payload)),
            3 if block == expected_block.wrapping_sub(1) => {
                let ack = [0, 4, (block >> 8) as u8, block as u8];
                socket.send_to(&ack, peer).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
            }
            _ => return Err(DatagramProtocolError::InvalidPacket),
        }
    }
}

pub fn tftp_write(socket: &UdpSocket, server: SocketAddr, filename: &str, data: &[u8], timeout: Duration, capabilities: &CapabilitySet) -> Result<(), DatagramProtocolError> {
    permit(capabilities)?;
    socket.set_read_timeout(Some(timeout)).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
    socket.set_write_timeout(Some(timeout)).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
    let request = tftp_request(2, filename)?;
    socket.send_to(&request, server).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
    let mut buffer = [0u8; 4 + 512];
    let (size, peer) = socket.recv_from(&mut buffer).map_err(timeout)?;
    let (opcode, block, payload) = parse_tftp_packet(&buffer[..size])?;
    if opcode == 5 { return Err(tftp_error(payload)); }
    if opcode != 4 || block != 0 { return Err(DatagramProtocolError::InvalidPacket); }
    let mut block_number = 1u16;
    let mut offset = 0usize;
    loop {
        let remaining = data.len().saturating_sub(offset);
        let chunk_len = remaining.min(512);
        let final_block = chunk_len < 512;
        let mut packet = Vec::with_capacity(4 + chunk_len);
        packet.extend_from_slice(&3u16.to_be_bytes());
        packet.extend_from_slice(&block_number.to_be_bytes());
        packet.extend_from_slice(&data[offset..offset + chunk_len]);
        socket.send_to(&packet, peer).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
        let (size, _) = socket.recv_from(&mut buffer).map_err(timeout)?;
        let (ack_opcode, ack_block, ack_payload) = parse_tftp_packet(&buffer[..size])?;
        if ack_opcode == 5 { return Err(tftp_error(ack_payload)); }
        if ack_opcode != 4 || ack_block != block_number { return Err(DatagramProtocolError::InvalidPacket); }
        offset += chunk_len;
        if final_block { return Ok(()); }
        block_number = block_number.wrapping_add(1);
    }
}

pub fn ntp_query(socket: &UdpSocket, server: SocketAddr, timeout: Duration, capabilities: &CapabilitySet) -> Result<u64, DatagramProtocolError> {
    permit(capabilities)?;
    socket.set_read_timeout(Some(timeout)).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
    socket.set_write_timeout(Some(timeout)).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
    let mut request = [0u8; 48];
    request[0] = 0x23;
    socket.send_to(&request, server).map_err(|e| DatagramProtocolError::Io(e.to_string()))?;
    let mut response = [0u8; 512];
    let (size, _) = socket.recv_from(&mut response).map_err(timeout)?;
    if size < 48 || (response[0] & 0x07) != 4 { return Err(DatagramProtocolError::InvalidPacket); }
    if response[1] > 15 { return Err(DatagramProtocolError::InvalidPacket); }
    Ok(u64::from_be_bytes(response[40..48].try_into().expect("NTP timestamp is 8 bytes")))
}

#[cfg(test)]
mod tests {
    use super::*;
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
            server.send_to(&[0, 3, 0, 1, b'h', b'e', b'l', b'l', b'o'], peer).expect("data");
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
            server.send_to(&[0, 4, 0, 1], peer).expect("ack1");
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
            assert_eq!(request[0], 0x23);
            let mut response = [0u8; 48];
            response[0] = 0x24;
            response[40..48].copy_from_slice(&(1_700_000_000u64 << 32).to_be_bytes());
            server.send_to(&response, peer).expect("ntp response");
        });
        let socket = UdpSocket::bind("127.0.0.1:0").expect("bind");
        let timestamp = ntp_query(&socket, server_addr, Duration::from_secs(2), &caps()).expect("ntp");
        assert_eq!(timestamp, 1_700_000_000u64 << 32);
        handle.join().expect("server");
    }

    #[test]
    fn datagram_protocols_are_capability_gated_before_io() {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("bind");
        let server = "127.0.0.1:9".parse().expect("addr");
        assert_eq!(tftp_read(&socket, server, "x", Duration::from_millis(1), &CapabilitySet::none()), Err(DatagramProtocolError::CapabilityDenied));
        assert_eq!(ntp_query(&socket, server, Duration::from_millis(1), &CapabilitySet::none()), Err(DatagramProtocolError::CapabilityDenied));
    }
}
