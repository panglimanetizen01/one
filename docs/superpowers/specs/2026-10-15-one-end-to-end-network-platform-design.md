# One — End-to-End Platform and Network Language Architecture

**Status:** Proposed architectural specification — user-approved design direction
**Date:** 2026-10-15
**Scope:** One language, application/UI platform, network language, runtime, data, testing, build, deployment, and interoperability architecture

## 1. Purpose

One is intended to become a general-purpose programming platform that can replace the developer-facing positions of a fragmented application stack while remaining understandable to ordinary people.

The platform target includes:

- design and UI/UX modeling;
- web, mobile, and desktop application development;
- backend/server development;
- data and database operations;
- networking from everyday connectivity through advanced network engineering;
- testing and QA;
- build, packaging, automation, and deployment;
- native platform integration;
- interoperability with existing ecosystems.

"Replace" means that One owns the source language, semantic model, developer-facing abstractions, project model, and toolchain for the capability. Existing technologies may remain implementation backends, platform targets, or interoperability adapters. One must not merely become a collection of wrappers whose semantics are defined by another language or framework.

## 2. Existing One invariants remain authoritative

This architecture extends, but does not replace, the existing One v0.1 language contracts.

The canonical semantic pipeline remains:

`source → lexer → parser → AST → name/scope analysis → type/semantic analysis → CSIR → reference interpreter`

Each stage remains deterministic. LLMs and network services are never semantic authorities. Ambiguity is rejected rather than guessed. Capability/effect enforcement remains the security boundary, and the reference interpreter remains the semantic oracle.

## 3. Core architectural model

```text
                              ONE
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
   LANGUAGE CORE        APPLICATION MODEL       NETWORK MODEL
        │                      │                      │
   Lexer / Parser          UI / UX               Network
   AST                     State                 Address
   Types                   Events                Connection
   Semantics               Data                  Service
   CSIR                    Accessibility         Protocol
                           Animation              Security
        │                      │                   Routing
        └──────────────────────┼─────────────────────┘
                               │
                         COMMON ONE IR
                               │
              ┌────────────────┼────────────────┐
              │                │                │
           Runtime          Compiler        Capability
              │                │                │
      ┌───────┼───────┐       │       ┌────────┼────────┐
      │       │       │       │       │        │        │
     Web    Mobile Desktop  Native  Network   Data    System
      │       │       │       │       │        │        │
      └───────┴───────┴───────┴───────┴────────┴────────┘
                               │
                  Windows / Linux / other targets
```

## 4. One Network Language

One shall provide a first-class network semantic layer whose vocabulary is understandable without requiring knowledge of protocol names.

The user-facing model is capability-oriented and progressive:

```text
Pemula
  "hubungkan ke server"

Pengguna lanjut
  "hubungkan dengan aman"

Developer
  "gunakan koneksi reliable dan batasi waktunya"

Network engineer
  control transport, addressing, routing, interfaces,
  tunnels, VLANs, telemetry, QoS, protocol parameters,
  and explicit capabilities
```

The compiler/runtime maps these semantic requests to protocol implementations and platform capabilities. The protocol name is not required in ordinary source code unless the programmer explicitly needs protocol-level control.

### 4.1 Network semantic domains

The One Network model contains these first-class domains:

1. Network discovery
2. Device discovery and communication
3. Addressing and naming
4. Service discovery and service exposure
5. Connection establishment and termination
6. Reliable/unreliable transport
7. Request/response and messaging
8. Streaming and real-time communication
9. Local-network communication
10. Internet communication
11. Routing and path control
12. Sessions
13. Identity and authentication
14. Encryption and certificate handling
15. Authorization and network capabilities
16. Broadcast and multicast
17. VPN and tunneling
18. Industrial and IoT communication
19. Data-center and storage networking
20. Mesh and peer-to-peer networking
21. Delay-tolerant networking
22. Network management and telemetry
23. Remote access
24. Physical and serial communication

### 4.2 Protocol capability registry

The architecture maintains a machine-readable registry mapping external protocols and standards to One semantic capabilities and implementation adapters.

The initial registry inventory includes, but is not limited to:

- Application: HTTP, HTTPS, HTTP/2, HTTP/3/QUIC, FTP, SFTP, FTPS, TFTP, SMTP, POP3, IMAP, XMPP, IRC, SIP, H.323, DNS, mDNS, LLMNR, DHCP, DHCPv6, SNMP, NTP, PTP, SSH, Telnet, RDP, VNC.
- Presentation/security/serialization: TLS, SSL, ASN.1, MIME, JSON, XML, Protobuf.
- Session: NetBIOS, PPTP, RPC, SOCKS, L2TP.
- Transport: TCP, UDP, SCTP, DCCP, QUIC.
- Network: IPv4, IPv6, BGP, OSPF, IS-IS, RIP, EIGRP, ICMP, ICMPv6, IGMP, IPsec.
- Data link: Ethernet/IEEE 802.3, Wi-Fi/IEEE 802.11 families, ARP, RARP, NDP, PPP, PPPoE, HDLC, Frame Relay, ATM, STP, VLAN/802.1Q, LACP, LLDP, CDP.
- Physical/connectivity: Bluetooth, Zigbee, Z-Wave, LoRaWAN, 3G, 4G LTE, 5G NR, 6G, DSL, DOCSIS, PON/GPON/EPON, USB, RS-232, RS-485.
- Industrial/IoT: Modbus, PROFINET, EtherCAT, CAN bus, MQTT, CoAP.
- Data-center/storage: Fibre Channel, iSCSI, RoCE, NVMe-oF, MPLS, Segment Routing.
- Mesh/P2P: BATMAN, OLSR, AODV, IPFS, BitTorrent Wire Protocol.
- Space: DTN/Bundle Protocol.
- VPN/modern cryptography: WireGuard, OpenVPN, IPsec IKEv2.
- Management/telemetry: NETCONF, YANG, gRPC, sFlow, NetFlow, IPFIX.

The registry is a compatibility inventory, not a promise that every listed protocol is implemented in the first release. Each implementation must have evidence and conformance tests before being marked supported.

## 5. Network language examples

Beginner-level source:

```text
hubungkan ke server.example.com dengan aman
kirim data pengguna ke server
terima jawaban dari server
```

Local discovery:

```text
cari perangkat di jaringan lokal
ketika menemukan printer
    tampilkan nama printer
```

Explicit engineering control:

```text
buat koneksi ke server
gunakan transport reliable
batasi waktu koneksi menjadi 5 detik
amankan koneksi dengan identitas server
```

The exact grammar and vocabulary remain subject to the formal language specification. Examples are illustrative and are not themselves a grammar contract.

## 6. One UI/UX and design platform

One shall own a unified model for:

- visual hierarchy;
- components;
- layout;
- typography;
- color and theme tokens;
- spacing and sizing;
- responsive behavior;
- accessibility;
- interaction states;
- navigation and application flow;
- animation and transitions;
- vector graphics;
- rich graphics and shaders;
- design-to-implementation consistency.

External concepts map as follows:

| Existing technology | One-owned capability |
|---|---|
| Figma / Adobe XD / Sketch / Miro | One design, flow, component and collaboration model |
| JSON design tokens | One typed design-system tokens |
| SVG | One vector graphics model |
| Lottie JSON | One animation model |
| GLSL / Three.js | One graphics/shader abstraction |
| HTML | One UI structure model |
| CSS / SCSS / PostCSS | One layout/style system |
| React / Vue / Svelte | One component/reactivity model |
| ARIA | One accessibility semantics model |

Existing tools may be import/export or interoperability targets, not the semantic foundation of One.

## 7. One application runtime

One shall target a common application model across:

- Web
- Android
- iOS
- Windows desktop
- Linux desktop
- other supported platforms as backends mature

The developer should not have to maintain separate language stacks for frontend, mobile, and desktop when the application semantics are portable.

Platform-specific functionality remains explicit through capability declarations and platform adapters.

## 8. One backend and service model

One shall provide a unified service model covering:

- HTTP/API services;
- background services;
- event-driven services;
- scheduled work;
- authentication and authorization;
- file/object handling;
- streaming;
- observability;
- process and system integration.

Node/Express/Nest, Python/FastAPI/Django/Flask, PHP/Laravel, Go/Rust, Java/JVM, and .NET are interoperability or implementation targets rather than required application-language dependencies.

## 9. One data model

One shall expose a unified data model for:

- structured relational data;
- documents;
- key/value data;
- cache/state;
- streams;
- local application storage;
- remote service data.

SQL, Postgres, MySQL, SQLite, Oracle, SQL Server, MongoDB, Firestore, Redis, and future stores are represented through typed data capabilities and adapters.

The abstraction must not hide material differences when they affect correctness, consistency, transactions, durability, query semantics, or security.

## 10. One testing and QA model

One shall provide one language-level testing model covering:

- unit tests;
- semantic tests;
- integration tests;
- UI tests;
- network tests;
- API tests;
- end-to-end tests;
- regression tests;
- property/conformance tests;
- deterministic fixture testing.

Cypress, Playwright, and Jest become optional interoperability targets. One tests must not depend on them to define One semantics.

## 11. One build, package, automation, and deployment model

One shall own a unified project lifecycle:

```text
create
  → develop
  → check
  → test
  → build
  → package
  → publish
  → deploy
  → monitor
  → update
```

The model covers capabilities currently commonly distributed across shell scripts, YAML CI, Dockerfiles, reverse proxies, cloud build systems, and deployment platforms.

Bash/Shell, YAML CI, Docker, Nginx, Vercel, Netlify, AWS, and GitHub/GitLab CI become integration/deployment backends rather than requirements of the One language.

## 12. Interoperability rule

One may interoperate with an existing technology without becoming dependent on its developer-facing semantics.

Examples:

```text
One source
    ↓
One semantic model
    ↓
One IR
    ↓
optional backend/adapter
    ↓
React / native UI / SQL / HTTP stack / cloud target
```

The reverse direction is also possible where technically justified:

```text
legacy technology
    ↓
One adapter
    ↓
One semantic capability
```

## 13. Security and capability boundaries

Network and system operations are capabilities, not unrestricted language primitives.

Examples of controlled capabilities include:

- network access;
- local-network discovery;
- listening on ports;
- outbound connections;
- raw packet operations where supported;
- interface configuration;
- routing changes;
- VPN/tunnel creation;
- device access;
- filesystem access;
- process execution;
- native/FFI access.

A program must receive only the capabilities explicitly granted by its execution environment or project policy.

One must distinguish ordinary application networking from privileged network administration. The language must not silently grant administrative authority merely because a program requests a network operation.

## 14. Human-readable diagnostics

Network failures must be translated into explanations understandable by ordinary users while retaining precise technical diagnostics for advanced users.

Example:

```text
Tidak bisa terhubung ke server.

Kemungkinan:
- alamat server tidak ditemukan
- jaringan tidak tersedia
- server tidak menjawab
- koneksi ditolak
- identitas server tidak dapat diverifikasi

Detail teknis:
DNS → timeout
Transport → tidak dibuat
TLS → tidak dimulai
```

The human explanation is derived from deterministic runtime/diagnostic information. It must not be generated by guessing.

## 15. Localization

The network semantic model is independent of the surface language.

For Indonesian:

```text
hubungkan ke server
cari perangkat
kirim data
terima data
amankan koneksi
```

Another localization may express the same semantic operations differently while lowering to the same One Network IR.

Thus localization changes vocabulary and grammar, not network semantics.

## 16. Progressive implementation strategy

The full target is too broad to implement as one atomic release. Implementation therefore proceeds through vertical, verifiable slices.

Priority order:

1. Network semantic vocabulary and capability model.
2. Network IR types and deterministic diagnostics.
3. Basic client networking: address/name resolution, connection, request/response.
4. Secure networking and identity.
5. Local-network discovery and service discovery.
6. Streaming and messaging.
7. HTTP/web service model.
8. UI/design semantic model.
9. Unified application runtime.
10. Data model and database adapters.
11. Testing/QA model.
12. Build/package/deployment model.
13. Advanced network engineering capabilities.
14. Native mobile/desktop platform backends.
15. Graphics/shader and advanced design tooling.
16. Additional protocol and platform adapters.

Each slice requires deterministic tests, conformance fixtures, capability tests, and end-to-end evidence before being declared supported.

## 17. Non-goals

One is not intended to:

- pretend every external protocol is identical;
- hide protocol-specific behavior that affects correctness;
- grant privileged network access implicitly;
- use an LLM to infer network operations at runtime;
- replace standards with informal descriptions;
- claim support for an adapter without implementation evidence;
- force users to learn protocol internals merely to perform ordinary tasks.

## 18. Success criteria

The architecture is successful when a user can build a meaningful application with One without being forced to assemble separate frontend, backend, database, testing, networking, and deployment languages for ordinary use cases, while an expert can progressively descend into precise technical control without leaving the One semantic model.

For networking specifically, a beginner must be able to perform common networking tasks using understandable One terminology, while a network engineer must be able to access protocol-specific controls, routing, security, telemetry, and physical/link capabilities through explicit advanced constructs.

## 19. Verification rule

No capability is considered implemented merely because its name appears in this specification or registry.

Support status must distinguish:

- **Specified** — semantic contract exists.
- **Designed** — implementation boundary is defined.
- **Implemented** — code exists.
- **Verified** — deterministic tests and required end-to-end evidence pass.
- **Supported** — verified on declared platforms/backends.

This preserves One's existing evidence-first development discipline.
