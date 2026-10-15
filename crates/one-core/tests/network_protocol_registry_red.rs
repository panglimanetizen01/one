use one_core::network::{NetworkProtocolFamily, ProtocolRegistry, ProtocolSupportStatus};

#[test]
fn complete_inventory_contains_the_architectural_protocol_set() {
    let names: Vec<&str> = ProtocolRegistry::builtin().entries().iter().map(|entry| entry.name).collect();
    for required in [
        "DNS", "TCP", "UDP", "TLS", "HTTP", "HTTP/2", "HTTP/3", "QUIC",
        "FTP", "SFTP", "FTPS", "TFTP", "SMTP", "POP3", "IMAP", "XMPP", "IRC", "SIP", "H.323",
        "mDNS", "LLMNR", "DHCP", "DHCPv6", "SNMP", "NTP", "PTP", "SSH", "Telnet", "RDP", "VNC",
        "ASN.1", "MIME", "JSON", "XML", "Protobuf", "NetBIOS", "PPTP", "RPC", "SOCKS", "L2TP",
        "SCTP", "DCCP", "IPv4", "IPv6", "BGP", "OSPF", "IS-IS", "RIP", "EIGRP", "ICMP", "ICMPv6", "IGMP", "IPsec",
        "Ethernet", "Wi-Fi", "ARP", "RARP", "NDP", "PPP", "PPPoE", "HDLC", "Frame Relay", "ATM", "STP", "VLAN", "LACP", "LLDP", "CDP",
        "Bluetooth", "Zigbee", "Z-Wave", "LoRaWAN", "3G", "4G LTE", "5G NR", "6G", "DSL", "DOCSIS", "PON", "USB", "RS-232", "RS-485",
        "Modbus", "PROFINET", "EtherCAT", "CAN", "MQTT", "CoAP", "Fibre Channel", "iSCSI", "RoCE", "NVMe-oF", "MPLS", "Segment Routing",
        "BATMAN", "OLSR", "AODV", "IPFS", "BitTorrent", "DTN Bundle", "WireGuard", "OpenVPN", "IPsec IKEv2",
        "NETCONF", "YANG", "gRPC", "sFlow", "NetFlow", "IPFIX",
    ] {
        assert!(names.contains(&required), "missing protocol inventory entry: {required}");
    }
}

#[test]
fn only_real_implementations_can_be_verified_or_supported() {
    let registry = ProtocolRegistry::builtin();
    for entry in registry.entries() {
        if matches!(entry.status, ProtocolSupportStatus::Verified | ProtocolSupportStatus::Supported) {
            assert!(matches!(entry.family, NetworkProtocolFamily::NameResolution | NetworkProtocolFamily::Tcp | NetworkProtocolFamily::Udp | NetworkProtocolFamily::Tls | NetworkProtocolFamily::Http | NetworkProtocolFamily::Quic));
        }
    }
}
