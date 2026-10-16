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
| Human-language orchestration foundation | VERIFIED | typed capability requests, deterministic governance, governed adapters, result normalization, audit records, localization fixtures, and `network.dns.resolve` runtime evidence in baseline `dba4fad49854952e256ae41744f57f93e39167a8` |
| Language-level One-source orchestration | IN DEVELOPMENT | CLI/runtime still uses the established parse → analyze → lower → interpreter path; an end-to-end source-to-orchestrator bridge is the next boundary |
| CLI | IMPLEMENTED | conversational commands plus technical aliases |
| Project scaffolding | IMPLEMENTED | CLI project tests |
| Conformance fixtures | ADDED | `tests/conformance/**/*.one` |
| Conformance runner | ADDED | `crates/one-core/tests/conformance.rs` |
| Golden output contracts | ADDED | `tests/golden/*.snap` |
| Offline Linux verifier | ADDED | `tools/verify_offline.sh` |
| Offline Windows verifier | ADDED | `tools/verify_offline.ps1` |
| Release gate workflow | DEFINED | `.github/workflows/release-check.yml` |
| Linux/Windows CI definition | COMPLETE | `.github/workflows/ci.yml` |
| Last independently verified source revision | VERIFIED | `dba4fad49854952e256ae41744f57f93e39167a8`, independent run `38872070590` |
| Current documentation synchronization | IN PROGRESS | documentation-only changes are followed by a fresh exact-SHA verification |
| Private canonical hosted CI execution | BLOCKED | canonical runner jobs currently fail before usable workflow-step/log evidence |
| Native C/C++ FFI | FUTURE | contract required before support claim |
| Python binding | FUTURE | contract required before support claim |
| Node.js binding | FUTURE | contract required before support claim |
| Java/JVM binding | FUTURE | contract required before support claim |
| Full native compiler backend | FUTURE | reference interpreter is the current execution oracle |
| Package registry | FUTURE | package contract not yet implemented |
| IDE/editor | FUTURE | not part of current executable core |
| Additional locales | FUTURE | Indonesian is the initial locale |

## Verified network boundary

The verified native runtime inventory is DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit runtime limits. The independent baseline verification passed contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, and platform-specific offline verification on Ubuntu and Windows Server 2025.

## Orchestration boundary

The orchestration foundation is verified as a typed runtime library boundary. It includes semantic-to-capability resolution, deterministic governance, scope enforcement, typed adapter selection/invocation, normalized evidence, audit records, and a real DNS adapter.

It is **not yet language-level supported**: a normal One source program does not currently have verified parser/semantic → capability → governance → adapter → normalized One result execution. That end-to-end path is the next implementation target.

## Release interpretation

`IMPLEMENTED` means the canonical source contains the implementation and associated tests. `VERIFIED` means independent execution evidence exists for the exact recorded source revision. `Supported` is reserved for a stronger support claim and is not inferred from registry metadata alone.

The independent verification infrastructure is the temporary public mirror `panglimanetizen01/one-windows-verification`. No GitHub paid upgrade was used.
