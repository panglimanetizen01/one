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
| One Network foundation | VERIFIED | `network.rs`, network IR, capability tests, reference boundary, `docs/spec/one-network-foundation.md`; independent run `38763329481` |
| CLI | IMPLEMENTED | conversational commands plus technical aliases (`buat/new`, `periksa/check`, `jalankan/run`, `test`, `bangun/build`, `package`, `lihat struktur/ast`, `version`, `help`) |
| Project scaffolding | IMPLEMENTED | CLI project tests |
| Conformance fixtures | ADDED | `tests/conformance/**/*.one` |
| Conformance runner | ADDED | `crates/one-core/tests/conformance.rs` |
| Golden output contracts | ADDED | `tests/golden/*.snap` |
| Offline Linux verifier | ADDED | `tools/verify_offline.sh` |
| Offline Windows verifier | ADDED | `tools/verify_offline.ps1` |
| Release gate workflow | DEFINED | `.github/workflows/release-check.yml` |
| Linux/Windows CI definition | COMPLETE | `.github/workflows/ci.yml` |
| Linux independent execution verification | VERIFIED | run `38763329481`, job `116520018095` |
| Windows independent execution verification | VERIFIED | run `38763329481`, job `116520018367` |
| Private canonical hosted CI execution | BLOCKED | GitHub runner jobs fail before usable step/log evidence |
| Native C/C++ FFI | FUTURE | contract required before support claim |
| Python binding | FUTURE | contract required before support claim |
| Node.js binding | FUTURE | contract required before support claim |
| Java/JVM binding | FUTURE | contract required before support claim |
| Full native compiler backend | FUTURE | reference interpreter is the current execution oracle |
| Package registry | FUTURE | package contract not yet implemented |
| IDE/editor | FUTURE | not part of current executable core |
| Additional locales | FUTURE | Indonesian is the initial locale |

## Network foundation verified boundary

The verified checkpoint is `d05eab5a103b63949e1dfaa9b97a479115c27a8c`. Independent Linux and Windows Server 2025 execution completed successfully in verification run `38763329481`, including contract-artifact checks, build, Clippy, all workspace tests, and the platform-specific offline verification path.

The network foundation is verified only for its implemented semantic boundary: typed network operations, deterministic network IR, explicit granular capabilities, protocol-registry metadata, structured diagnostics, and a reference-interpreter boundary that performs no external network I/O.

It does not claim real DNS resolution, sockets, HTTP/QUIC runtime, discovery, routing, VPN, industrial, storage, mesh, DTN, telemetry, remote-access, or physical network-driver support.

## Release interpretation

`IMPLEMENTED` means the canonical source contains the implementation and associated tests. `VERIFIED` means independent execution evidence exists for the exact recorded source commit.

The independent verification infrastructure is the temporary public mirror `panglimanetizen01/one-windows-verification`; it checks out the canonical private repository at an exact commit using a read-only repository secret. No GitHub paid upgrade was used.
