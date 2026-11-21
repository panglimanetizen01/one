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


## HTTP request serialization resource bounds — 2026-11-21

Latest source commit: `198fbf4ccdbb7e219efbe3e296e98462a385c384` (`crates/one-core/src/network_runtime.rs`).

- HTTP request bodies are capped at 16 MiB before serialization; request line + headers are capped at 64 KiB with checked size arithmetic and fallible preallocation.
- Explicit Content-Length must parse and exactly match the body size; duplicate Content-Length and request Transfer-Encoding are rejected (chunked request serialization is not implemented).
- Added source regressions for oversized request bodies, mismatched Content-Length, and unsupported Transfer-Encoding. NOT RUN.
- No build, test, CI, verifier, or platform matrix was run. This remains an implementation candidate, not verification evidence.


## HTTP request syntax validation — 2026-11-21

Latest source commit: `b0a3379a1fb49feb18dcf61a2521446f199a8e2b` (`crates/one-core/src/network_runtime.rs`).

- Request method and header names must be valid HTTP tokens.
- Authority rejects whitespace/control bytes, user-info separators and fragments; request-target must use origin-form or `*`, and rejects whitespace/control bytes and fragments.
- Header values reject control bytes other than horizontal tab.
- These checks complement the 16 MiB request-body / 64 KiB header caps, Content-Length consistency checks, and response framing limits. Source-only change; no build or tests run.


## HTTP exact framing and trailer validation — 2026-11-21

Latest source commit: `eea3115ba5e70ff83f8ed78a01dc70d95e012ff0` (`crates/one-core/src/network_runtime.rs`).

- Content-Length response bodies must now match the declared length exactly; trailing bytes are rejected.
- Chunked responses bound chunk-size lines, require valid terminating CRLF, validate trailer syntax, reject framing/Host fields in trailers, and reject trailing bytes after the trailer terminator.
- Added regressions for valid trailers, forbidden Content-Length trailers, and extra bytes after Content-Length. NOT RUN.
- No build/test/CI/verifier/platform matrix was run; source-only candidate, no acceptance promoted.


## HTTP header token/control validation regressions — 2026-11-21

Latest source commit: `11bf5172ed060818dbd55d2ecee3cddf3cd99875` (`crates/one-core/src/network_runtime.rs`).

- Response header names are validated as HTTP tokens and values reject control bytes except HTAB. Request header names/values and request-target syntax have equivalent validation.
- Added regressions for invalid response header names/values and invalid request header names/targets. NOT RUN.
- No build/test/CI/verifier/platform matrix was run; this remains source-only implementation work.


## Exact fallible HTTP buffer reservations — 2026-11-21

Latest source commit: `bbd4febbac3a4ba2a19195582e7e5992d1953b1c` (`crates/one-core/src/network_runtime.rs`).

- HTTP request serialization, response ingestion, and chunk-body decoding now use `try_reserve_exact` within the explicit request/response size ceilings, avoiding geometric capacity growth beyond the intended bounds.
- This is source-only resource hardening. No build/test/CI/verifier/platform matrix ran.


## IPv6 HTTP authority and query-only target support — 2026-11-21

Latest source commits:
- `f8631323b7c987c69f2ab8072bbf081deb626fb9` — shared HTTP authority parser accepts bracketed IPv6 literals with default/explicit ports; generic network resolution now formats IPv6 socket addresses correctly.
- `4b0749fbd413233ee45259942a334190474ac89e` and `5bc86f6110145074cbee96942ad7942e25318b9c` — HTTP GET target parsing uses the shared authority parser, supports bracketed IPv6 and query-only paths, and rejects malformed/whitespace request paths.
- `b0e034b3c98248e6f33823bd1ff9f2242c89fa8f` and `be5566435394cc27bbeb863b78c69b0fe52f774c` add source regressions for IPv6 authorities, query-only paths, and invalid paths. NOT RUN.
- No build/test/CI/verifier/platform matrix was run.


## Absolute HTTP response read deadline — 2026-11-21

Latest source commits: `981467252a95198adfc72872db2dc652fa9f505e` and `a911431be5a1dad31dc2344e09a357e2d5e91212` (`crates/one-core/src/network_runtime.rs`).

- Bounded response ingestion now uses one absolute deadline across all socket reads, updating the socket timeout to the remaining budget each iteration. This prevents a slow-trickle peer from resetting the full per-read timeout indefinitely.
- The bounded-reader regression fixture now supplies its explicit deadline. NOT RUN.
- No build/test/CI/verifier/platform matrix ran.
