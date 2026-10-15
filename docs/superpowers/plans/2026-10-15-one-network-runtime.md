# One Network Runtime Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the verified One Network semantic foundation into real, testable network I/O for sockets/TCP, DNS resolution, HTTP/1.1, TLS-backed HTTPS, UDP, and QUIC/HTTP3, while preserving deterministic semantics, explicit capabilities, platform boundaries, and offline semantic tests.

**Architecture:** Keep One's canonical semantic pipeline and network IR. Add a concrete network runtime adapter behind the existing capability gate. The adapter owns real I/O; the reference interpreter remains side-effect free. Standard-library transports are used where practical; protocol crates are isolated behind runtime features so the core semantic build remains deterministic and the actual adapters have explicit dependency/build evidence.

**Tech Stack:** Rust 2024, `std::net` for TCP/UDP/DNS system resolution, `rustls` + `webpki-roots` for TLS, `h1`-compatible manual HTTP/1.1 framing where feasible, and `quinn` for QUIC/HTTP3. Integration tests use loopback-only servers and must never require the public Internet.

**Spec:** `docs/superpowers/specs/2026-10-15-one-end-to-end-network-platform-design.md` and `docs/spec/one-network-foundation.md`

## Global Constraints

- GitHub `main` remains canonical; local/Termux is verification only.
- No hosted GitHub paid upgrade.
- No LLM/network service is semantic authority.
- No implicit network capability or administrative privilege.
- Reference interpreter performs no external I/O.
- Real adapter tests use loopback/local fixtures only.
- DNS tests must include deterministic localhost resolution and explicit failure handling; no public DNS dependency.
- HTTP tests must exercise actual request/response bytes through a real loopback socket.
- TLS tests must use an in-repository deterministic test certificate or an ephemeral local certificate fixture; never depend on public CA/network state.
- QUIC tests must use loopback and a deterministic local endpoint.
- Existing v0.1 language behavior must remain unchanged.
- A protocol is not marked `Implemented`, `Verified`, or `Supported` until code and tests prove that exact status.

## Review Focus

- Capability denial occurs before any socket/DNS/TLS/HTTP/QUIC I/O.
- Timeouts, address family, transport, and security settings survive lowering without semantic loss.
- Resource ownership is deterministic: sockets and connections close predictably.
- Errors expose stable stage/code information and never invent causes.
- HTTP parsing rejects malformed framing rather than guessing.
- TLS certificate validation is explicit; insecure mode is never silently substituted.
- QUIC is a real transport implementation, not a registry placeholder or subprocess wrapper.

## File Map

- Modify `crates/one-core/Cargo.toml` for runtime dependencies/features.
- Create `crates/one-core/src/network_runtime.rs` for concrete socket/DNS/TCP/UDP/TLS/HTTP/QUIC adapters.
- Modify `crates/one-core/src/network.rs` to expose runtime-facing typed requests/results and implementation statuses.
- Modify `crates/one-core/src/lib.rs` to export the runtime module.
- Modify `crates/one-core/src/capability.rs` only where runtime-specific capability checks require it.
- Create `crates/one-core/tests/network_runtime.rs` for loopback integration tests.
- Create deterministic local certificate fixtures under `crates/one-core/tests/fixtures/` if TLS tests require them.
- Modify `docs/spec/one-network-foundation.md` to distinguish real adapter status from semantic-only status.
- Modify `README.md`, `CHANGELOG.md`, and `docs/verification/one-v0.1-implementation-status.md` only from verification evidence.
- Modify offline verification scripts only if the new feature matrix requires an explicit adapter check.

---

### Task 1: Establish concrete runtime request/result boundaries

- [ ] Add a `NetworkRuntime` trait and typed result/error model that accepts validated `NetworkOperation` plus capabilities.
- [ ] Ensure capability checks happen before backend calls.
- [ ] Add fail-first tests for denied connect/send/receive.
- [ ] Verify no socket is created on denial using a test backend/mock counter.
- [ ] Commit `feat: establish One Network runtime boundary`.

### Task 2: Implement real TCP sockets and DNS resolution

- [ ] Implement TCP connect/send/receive with `std::net::TcpStream`.
- [ ] Implement UDP bind/send/receive with `std::net::UdpSocket`.
- [ ] Implement DNS/name resolution using `ToSocketAddrs` behind the resolver adapter, preserving IPv4/IPv6 results.
- [ ] Implement deterministic timeout propagation and address-family filtering.
- [ ] Add loopback TCP and UDP integration tests with real sockets.
- [ ] Add localhost resolution and deterministic invalid-host failure tests.
- [ ] Commit `feat: implement One TCP UDP and DNS adapters`.

### Task 3: Implement TLS and HTTPS transport

- [ ] Add `rustls` and root-store dependencies behind a runtime feature.
- [ ] Implement explicit TLS client configuration with certificate verification enabled by default.
- [ ] Implement deterministic insecure-mode rejection unless an explicit capability/configuration permits it.
- [ ] Implement HTTPS over the concrete TCP+TLS stream.
- [ ] Add loopback TLS server/client integration tests with deterministic local certificates.
- [ ] Verify hostname/certificate mismatch fails at identity stage.
- [ ] Commit `feat: implement One TLS and HTTPS adapters`.

### Task 4: Implement HTTP/1.1 framing and client semantics

- [ ] Implement request construction from typed method/path/headers/body values.
- [ ] Implement response status/header/body parsing with strict Content-Length and chunked framing validation.
- [ ] Enforce maximum header/body limits and timeout behavior.
- [ ] Add loopback HTTP tests for GET, POST, headers, content length, chunked response, malformed response, and timeout.
- [ ] Add HTTPS integration through the TLS adapter.
- [ ] Commit `feat: implement One HTTP client`.

### Task 5: Implement QUIC and HTTP/3

- [ ] Add `quinn`/HTTP3 dependencies behind an explicit runtime feature.
- [ ] Implement QUIC endpoint creation, TLS configuration, stream open/send/receive, timeout, and orderly close.
- [ ] Implement HTTP/3 request/response on the QUIC connection where the dependency API supports it without semantic leakage.
- [ ] Add loopback QUIC integration tests with deterministic certificates.
- [ ] Test capability denial before endpoint creation and certificate failure at identity stage.
- [ ] Commit `feat: implement One QUIC and HTTP3 adapters`.

### Task 6: Integrate real adapters with One execution and CLI boundaries

- [ ] Add a runtime selection boundary that chooses reference/no-I/O or real adapter explicitly.
- [ ] Keep ordinary `check`/semantic validation free of network I/O.
- [ ] Add a dedicated runtime invocation path for real network operations.
- [ ] Ensure errors map into the existing deterministic network diagnostic model.
- [ ] Add end-to-end tests from typed operation → capability gate → real loopback adapter → typed result.
- [ ] Commit `feat: integrate real One Network execution`.

### Task 7: Verify, classify, and publish support status

- [ ] Run workspace build/clippy/tests.
- [ ] Run loopback network integration suite.
- [ ] Run Linux offline verification.
- [ ] Run independent Windows verification, including socket/DNS/HTTP/TLS tests and QUIC feature tests where supported.
- [ ] Update registry statuses only from actual evidence.
- [ ] Update docs to clearly separate Implemented, Verified, and Supported.
- [ ] Commit `test: verify One real network runtime`.

## Execution Order

Tasks 1–2 establish real socket/DNS I/O. Tasks 3–4 build secure HTTP on top of those primitives. Task 5 adds QUIC/HTTP3 as a separate transport implementation. Task 6 connects real adapters to One execution without contaminating semantic checking. Task 7 is the final evidence gate and documentation alignment.

## Completion Criteria

The work is complete only when One has actual loopback-tested implementations for TCP sockets, UDP sockets, DNS resolution, TLS, HTTP/1.1, HTTPS, and QUIC/HTTP3; capability denial is proven to occur before I/O; malformed network data is rejected deterministically; Linux verification passes; and Windows verification provides evidence for every status claimed. If any adapter cannot be supported on a target, its status remains explicitly limited rather than being claimed as complete.
