# One Network Foundation v0.1

Status: **The network foundation and selected runtime slices are implemented; the 2026-11-21 bounded-response hardening candidate is NOT BUILT/TESTED. Broader network support remains future work.**

## Scope

This contract defines the first One-owned network semantic slice and the currently verified native runtime slices. It does not claim that One v0.1 already implements the complete Internet, industrial, storage, mesh, DTN, management, or physical protocol inventory.

The canonical pipeline remains:

`source → lexer → parser → AST → name/scope analysis → type/semantic analysis → CSIR → runtime/backend`

The network foundation adds a typed semantic boundary that can be lowered into network IR and checked against explicit capabilities before a backend executes.

## Typed semantic model

The foundation defines typed values for:

- address kind: IPv4, IPv6, or name;
- transport: reliable or unreliable;
- security: explicit insecure or TLS mode;
- protocol family: name resolution, TCP, UDP, TLS, HTTP, QUIC, application, session, network, data-link, physical, industrial, storage, mesh, delay-tolerant, or management;
- target: host, address, or host/service port;
- validated timeout;
- connection identity;
- operations: connect, send, and receive.

Contradictory or incomplete network intent is rejected. The semantic layer never guesses a protocol or transport.

The architecture examples such as `hubungkan ke server.example.com dengan aman` remain illustrative until a corresponding concrete grammar production is added to the language contract.

## Capabilities

Network authority is explicit:

- `NetworkConnect`
- `NetworkListen`
- `NetworkDiscover`
- `NetworkAdmin`

The existing coarse `Network` capability remains a compatibility grant for ordinary connect/send/receive operations. It never implies `NetworkAdmin`.

## Verified native runtime boundary

The canonical registry currently marks these protocol/runtime slices as `Verified` only after executable evidence:

- DNS/system resolution;
- TCP;
- UDP;
- TLS/HTTPS;
- HTTP/1.1;
- HTTP/2;
- QUIC;
- TFTP;
- NTP;
- FTP;
- SMTP;
- POP3;
- IMAP.

HTTP/2 uses the native `h2` implementation with real loopback HTTP/2 frames. Its runtime acceptance coverage includes capability-gate ordering, request/response round-trip, and cross-platform loopback execution.

The registry remains compatibility metadata, not language syntax. All other inventory entries remain explicitly unverified/future unless corresponding implementation and evidence are added.

## Network IR

Network operations are represented as typed IR. Canonical rendering preserves target, protocol family, transport, security, address kind, timeout, and payload/connection identity deterministically. No hash-map or platform ordering may affect the canonical representation.

## Reference execution boundary

The reference interpreter validates a network operation, checks capabilities, and returns a deterministic delegation result. It does **not** perform external network I/O. Capability denial occurs before backend delegation.

Native runtime adapters are separate from the reference interpreter. Their real I/O is feature-gated and must pass capability, framing, timeout, malformed-input, resource-cleanup, and platform-portability tests before a registry entry may become `Verified`.

## Diagnostics

Network failures carry a stable code, semantic stage, human explanation, and technical detail. The explanation is derived from structured runtime information. One does not invent a failure cause through heuristics or an LLM.

## Support-state rules

A protocol or platform capability may only advance to `Implemented`, `Verified`, or `Supported` when corresponding code and evidence exist. A registry entry alone is never evidence of implementation.

## Explicit limits

This foundation does not yet claim implementation of:

- HTTP/3 runtime;
- local service discovery;
- routing, VPN, industrial, storage, mesh, DTN, telemetry, or remote-access adapters;
- platform-specific network drivers;
- a complete network source grammar;
- the full protocol inventory described by the end-to-end network platform architecture;
- production-complete support for every RFC extension of the verified application protocols.

Those are subsequent vertical slices and must preserve the same deterministic semantic and capability boundaries.


## 2026-11-21 — bounded HTTP response ingestion and filesystem payload allocation

Latest relevant source commits:
- `d32bb207f8ea6f24740560f374a4f698c687e172` — `FileWriteAdapter` now passes a borrowed text slice to `FilesystemAuthority::write`, so the configured write-byte limit is enforced before an owned output copy is made. The bounded echo copy happens only after a successful write. Added `filesystem_write_adapter_rejects_oversized_content_without_side_effects` (NOT RUN).
- `ce39630fc9abdc9ba7c6accdf8db161a5cb002fa`, `8c219882aa37ee271874f58de172072531d80ddc`, `b636f0bc1e22008fc62dafe46a378cc2fb2b599b`, and `5fe958cad798b3d05c3ceb21f875f1f90074cb35` — HTTP/1.1 response ingestion now enforces a 64 KiB header ceiling and 32 MiB wire-body ceiling while reading, with fallible buffer growth; decoded response bodies remain capped at 16 MiB. The parser rejects duplicate Content-Length, unsupported/duplicate Transfer-Encoding, and simultaneous Content-Length + chunked framing; chunk-size arithmetic is checked and chunk allocation is fallible.
- Added source regressions for oversized headers, ambiguous framing, duplicate lengths, chunk-size overflow, and bounded socket reads. NOT RUN.

These are implementation/resource-boundary changes only. No build, test, CI, verifier, or platform matrix was run. No prior exact-SHA result transfers to this candidate.
