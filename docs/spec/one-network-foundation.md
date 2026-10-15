# One Network Foundation v0.1

Status: **Implemented only for the foundation slice; broader network support remains future work.**

## Scope

This contract defines the first One-owned network semantic slice. It does not claim that One v0.1 already implements the complete Internet, industrial, storage, mesh, DTN, or management protocol inventory.

The canonical pipeline remains:

`source → lexer → parser → AST → name/scope analysis → type/semantic analysis → CSIR → reference interpreter`

The network foundation adds a typed semantic boundary that can be lowered into network IR and checked against explicit capabilities before a backend is considered.

## Typed semantic model

The foundation defines typed values for:

- address kind: IPv4, IPv6, or name;
- transport: reliable or unreliable;
- security: explicit insecure or TLS mode;
- protocol family: name resolution, TCP, UDP, TLS, HTTP, or QUIC;
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

## Protocol registry

The registry is compatibility metadata, not language syntax. The initial entries are DNS, TCP, UDP, TLS, HTTP, and QUIC/HTTP3. Registry status is represented as `Specified`, `Designed`, `Implemented`, `Verified`, or `Supported`.

The foundation registry intentionally does not claim real protocol adapters are implemented or supported.

## Network IR

Network operations are represented as typed IR. Canonical rendering preserves target, protocol family, transport, security, address kind, timeout, and payload/connection identity deterministically. No hash-map or platform ordering may affect the canonical representation.

## Reference execution boundary

The reference interpreter validates a network operation, checks capabilities, and returns a deterministic delegation result. It does **not** perform external network I/O. Capability denial occurs before backend delegation.

This boundary is an execution oracle for semantics, not a claim of production network transport support.

## Diagnostics

Network failures carry a stable code, semantic stage, human explanation, and technical detail. The explanation is derived from structured runtime information. One does not invent a failure cause through heuristics or an LLM.

## Support-state rules

A protocol or platform capability may only advance to `Implemented`, `Verified`, or `Supported` when corresponding code and evidence exist. A registry entry alone is never evidence of implementation.

## Explicit limits

This foundation does not yet claim implementation of:

- real DNS resolution or socket I/O;
- HTTP client/server runtime;
- QUIC/HTTP3 runtime;
- local service discovery;
- routing, VPN, industrial, storage, mesh, DTN, telemetry, or remote-access adapters;
- platform-specific network drivers;
- a complete network source grammar;
- the full protocol inventory described by the end-to-end network platform architecture.

Those are subsequent vertical slices and must preserve the same deterministic semantic and capability boundaries.
