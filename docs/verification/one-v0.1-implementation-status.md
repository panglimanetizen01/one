# One v0.1 Implementation Status

| Area | State | Evidence in canonical repository |
|---|---|---|
| Foundation | COMPLETE | approved architecture/spec docs |
| Lexical contract | COMPLETE | `docs/spec/one-v0.1-lexical.md` |
| Concrete grammar | COMPLETE | `docs/spec/one-v0.1-grammar.md` |
| AST | IMPLEMENTED | `crates/one-core/src/ast.rs` + AST contract |
| Lexer | IMPLEMENTED | `crates/one-core/src/lexer.rs` + lexer tests |
| Parser | IMPLEMENTED | `crates/one-core/src/parser.rs` + pipeline/name-boundary tests |
| Name/scope analysis | IMPLEMENTED | `crates/one-core/src/semantic.rs` |
| Initial type system | IMPLEMENTED | `crates/one-core/src/types.rs` |
| Semantic IR | IMPLEMENTED | `crates/one-core/src/ir.rs` |
| Reference interpreter | IMPLEMENTED | `crates/one-core/src/interpreter.rs` |
| Diagnostics | IMPLEMENTED | `crates/one-core/src/diagnostic.rs` |
| Capability boundary | IMPLEMENTED | `capability.rs`, `platform.rs`, security tests |
| Standard library core | IMPLEMENTED | `stdlib.rs` and stdlib contract |
| Process interoperability | IMPLEMENTED | `interop.rs` and interoperability tests |
| One Network foundation | VERIFIED | typed network IR, capability checks, deterministic diagnostics, and verified native runtime slices |
| HTTP/2 native runtime | VERIFIED | `network_http2.rs` + `http2_runtime.rs`; independent Linux/Windows real-network evidence |
| Mail/application runtime hardening | IMPLEMENTED | FTP/SMTP/POP3/IMAP validation and bounded framing/body handling with tests |
| Human-language orchestration foundation | VERIFIED | typed capability requests, deterministic governance, governed adapters, result normalization, audit records, localization fixtures, and `network.dns.resolve` |
| Language-level DNS orchestration | EXPERIMENTAL | `cari alamat` → parser → semantic validation → typed orchestration IR → governed DNS adapter → normalized CLI result; explicit target-scoped permission required |
| Language-level filesystem read orchestration | EXPERIMENTAL | `baca berkas` → parser → semantic validation → typed orchestration IR → governed `filesystem.read` adapter → normalized UTF-8 text result; explicit target-scoped permission required; independently verified on Ubuntu + Windows Server 2025 at `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` via run `38901366447` |
| General language-level orchestration | IN DEVELOPMENT | additional capabilities and source operations are not yet implemented through the same end-to-end path |
| CLI | IMPLEMENTED | conversational commands, technical aliases, and explicit target-scoped orchestration permission surfaces |
| Project scaffolding | IMPLEMENTED | CLI project tests |
| Conformance fixtures | ADDED | `tests/conformance/**/*.one` |
| Conformance runner | ADDED | `crates/one-core/tests/conformance.rs` |
| Golden output contracts | ADDED | `tests/golden/*.snap` |
| Offline Linux verifier | ADDED | `tools/verify_offline.sh` |
| Offline Windows verifier | ADDED | `tools/verify_offline.ps1` |
| Release gate workflow | DEFINED | `.github/workflows/release-check.yml` |
| Linux/Windows CI definition | COMPLETE | `.github/workflows/ci.yml` |
| Latest independently verified implementation revision | VERIFIED | exact-SHA `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` in independent run `38901366447` |
| Current canonical main exact-SHA verification | GAP | current main has moved beyond the independently verified revision; a new exact-SHA run is required |
| Canonical hosted CI execution | BLOCKED / INCONCLUSIVE | latest main run `38903908051` reports failed Ubuntu/Windows jobs, but no job steps were exposed and log retrieval returned `404 BlobNotFound`; this is not evidence of a Rust source failure |
| Native C/C++ FFI | FUTURE | contract required before support claim |
| Python binding | FUTURE | contract required before support claim |
| Node.js binding | FUTURE | contract required before support claim |
| Java/JVM binding | FUTURE | contract required before support claim |
| Full native compiler backend | FUTURE | reference interpreter is the current execution oracle |
| Package registry | FUTURE | package contract not yet implemented |
| IDE/editor | FUTURE | not part of current executable core |
| Additional locales | FUTURE | Indonesian is the initial locale |

## Verified network boundary

The verified native runtime inventory is DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit runtime limits. Independent run `38901366447` passes contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, deterministic artifact checks, orchestration runtime, and platform-specific offline verification on Ubuntu and Windows Server 2025 for exact SHA `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b`.

## Orchestration boundary

Two language-level vertical slices are independently verified and remain experimental:

```text
One source: cari alamat "127.0.0.1"
→ parser
→ semantic validation
→ typed orchestration IR
→ explicit target-scoped governance
→ network.dns.resolve adapter
→ normalized text result
→ CLI output

One source: baca berkas "data/contoh.txt"
→ parser
→ semantic validation
→ typed orchestration IR
→ explicit target-scoped governance
→ filesystem.read adapter
→ normalized UTF-8 text result
→ CLI output
```

The CLI requires explicit permission for both slices. No permission means denial; a different target is outside the granted scope. This is not a claim that the complete orchestration catalog is supported.

## Current canonical verification gap

The current canonical `main` has advanced beyond exact SHA `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b`. The canonical comparison shows the intervening changes were limited to documentation/changelog files, but the evidence policy still requires a fresh exact-SHA verification before the current main commit can be labeled `VERIFIED`.

The independent verification infrastructure is the temporary public mirror `panglimanetizen01/one-windows-verification`. No GitHub paid upgrade was used.
