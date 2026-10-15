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
| One Network foundation | IMPLEMENTED | `network.rs`, network IR, capability tests, reference boundary, `docs/spec/one-network-foundation.md` |
| CLI | IMPLEMENTED | conversational commands plus technical aliases (`buat/new`, `periksa/check`, `jalankan/run`, `test`, `bangun/build`, `package`, `lihat struktur/ast`, `version`, `help`) |
| Project scaffolding | IMPLEMENTED | CLI project tests |
| Conformance fixtures | ADDED | `tests/conformance/**/*.one` |
| Conformance runner | ADDED | `crates/one-core/tests/conformance.rs` |
| Golden output contracts | ADDED | `tests/golden/*.snap` |
| Offline Linux verifier | ADDED | `tools/verify_offline.sh` |
| Offline Windows verifier | ADDED | `tools/verify_offline.ps1` |
| Release gate workflow | DEFINED | `.github/workflows/release-check.yml` |
| Linux/Windows CI definition | COMPLETE | `.github/workflows/ci.yml` |
| Linux independent execution verification | VERIFIED | run `38753437740`, job `116485529987` |
| Windows independent execution verification | VERIFIED | run `38753437740`, job `116485530381` |
| Private canonical hosted CI execution | BLOCKED | GitHub runner jobs fail before usable step/log evidence |
| Native C/C++ FFI | FUTURE | contract required before support claim |
| Python binding | FUTURE | contract required before support claim |
| Node.js binding | FUTURE | contract required before support claim |
| Java/JVM binding | FUTURE | contract required before support claim |
| Full native compiler backend | FUTURE | reference interpreter is the current execution oracle |
| Package registry | FUTURE | package contract not yet implemented |
| IDE/editor | FUTURE | not part of current executable core |
| Additional locales | FUTURE | Indonesian is the initial locale |

## Network foundation limits

The network foundation is implemented in source and tests, but the current status does not claim independent cross-platform verification for this new checkpoint until a fresh Linux and Windows run has completed. It deliberately does not claim real DNS resolution, sockets, HTTP/QUIC runtime, discovery, routing, VPN, industrial, storage, mesh, DTN, telemetry, remote-access, or physical network-driver support.

## Release interpretation

`IMPLEMENTED` means the canonical source contains the implementation and associated tests. `VERIFIED` means independent execution evidence exists for the exact recorded source commit.

The previously independently verified implementation commit is `d17f2829187ec28031a4b4a94dd0e0af282f66b8`. The verification run completed successfully on Linux and Windows Server 2025, including contract checks, build, Clippy, all workspace tests, and platform-specific offline verification.

The new One Network foundation must receive a fresh verification run before this document may be updated to claim that the new checkpoint is independently verified.

No GitHub paid upgrade is required for the verification result.
