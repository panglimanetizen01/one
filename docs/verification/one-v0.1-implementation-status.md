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
| HTTP/2 native runtime | VERIFIED | `network_http2.rs` + `http2_runtime.rs`; independent Linux/Windows real-network loopback evidence in verification run `38781871063` |
| CLI | IMPLEMENTED | conversational commands plus technical aliases (`buat/new`, `periksa/check`, `jalankan/run`, `test`, `bangun/build`, `package`, `lihat struktur/ast`, `version`, `help`) |
| Project scaffolding | IMPLEMENTED | CLI project tests |
| Conformance fixtures | ADDED | `tests/conformance/**/*.one` |
| Conformance runner | ADDED | `crates/one-core/tests/conformance.rs` |
| Golden output contracts | ADDED | `tests/golden/*.snap` |
| Offline Linux verifier | ADDED | `tools/verify_offline.sh` |
| Offline Windows verifier | ADDED | `tools/verify_offline.ps1` |
| Release gate workflow | DEFINED | `.github/workflows/release-check.yml` |
| Linux/Windows CI definition | COMPLETE | `.github/workflows/ci.yml` |
| Current independent Linux/Windows runtime verification | VERIFIED | verification run `38781871063`, jobs `116583878787` and `116583878563`, against canonical implementation commit `c2fecc0a488d335c73ff3abcc37f2f5d28a59a1f` |
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

The current verified native runtime inventory is:

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

Independent Linux and Windows execution in verification run `38781871063` passed contract-artifact checks, build, Clippy, all workspace tests, real-network runtime tests, and the platform-specific offline verification paths. HTTP/2 was included in the real-network stage.

The registry's remaining entries are not represented as verified runtime support. Hardware- and platform-dependent protocols require native/platform adapters and separate evidence.

## Release interpretation

`IMPLEMENTED` means the canonical source contains the implementation and associated tests. `VERIFIED` means independent execution evidence exists for the exact recorded source commit or an equivalent canonical source tree. `Supported` is reserved for a stronger support claim and is not inferred from registry metadata alone.

The independent verification infrastructure is the temporary public mirror `panglimanetizen01/one-windows-verification`; it checks out the canonical private repository at an exact commit using a read-only repository secret. No GitHub paid upgrade was used.
