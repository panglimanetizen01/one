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
| Language-level filesystem read orchestration | EXPERIMENTAL | `baca berkas` → parser → semantic validation → typed orchestration IR → governed `filesystem.read` adapter → normalized UTF-8 text result; independently verified on Ubuntu + Windows Server 2025 at `f482c6b75505d9965dd33f4f25bb0cd04072cedb` via `38903950612` |
| Language-level process execution orchestration | EXPERIMENTAL | `jalankan proses` → parser → semantic validation → typed orchestration IR → governed `process.execute` adapter → normalized process output; exact-SHA Linux + Windows verified at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; one targeted program without arguments |
| Language-level filesystem write orchestration | EXPERIMENTAL | `tulis ke berkas` → parser → semantic validation → typed IR → `filesystem.write` governance → filesystem write adapter → CLI; latest executable evidence `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android; cross-platform evidence for this executable revision is still pending |
| General language-level orchestration | IN DEVELOPMENT | additional capabilities and source operations are not yet implemented through the same end-to-end path |
| CLI | VERIFIED | existing CLI/process boundary cross-platform verified at `9038ac75f197753ff1ab6af7478e63e4831903a2`; latest filesystem.write executable verification recorded at `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android |
| Project scaffolding | IMPLEMENTED | CLI project tests |
| Conformance fixtures | ADDED | `tests/conformance/**/*.one` |
| Conformance runner | ADDED | `crates/one-core/tests/conformance.rs` |
| Golden output contracts | ADDED | `tests/golden/*.snap` |
| Offline Linux verifier | ADDED | `tools/verify_offline.sh` |
| Offline Windows verifier | ADDED | `tools/verify_offline.ps1` |
| Release gate workflow | DEFINED | `.github/workflows/release-check.yml` |
| Linux/Windows CI definition | COMPLETE | `.github/workflows/ci.yml` |
| Latest exact-SHA executable verification | VERIFIED / PLATFORM-BOUND | latest executable evidence `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8`; Termux/Android contract, build, Clippy, targeted filesystem-write CLI test, workspace tests, and offline verification all passed; this is not Ubuntu/Windows evidence |
| Latest cross-platform exact-SHA verification | VERIFIED | mirror run `38929072800` checked out `9038ac75f197753ff1ab6af7478e63e4831903a2` exactly and passed Ubuntu + Windows contract checks, build, Clippy, workspace tests, real-network tests, and offline verification |
| Private canonical hosted CI execution | NOT REQUIRED FOR P0 EVIDENCE | independent execution paths provide evidence; canonical main remains source of truth |
| Native C/C++ FFI | FUTURE | contract required before support claim |
| Python binding | FUTURE | contract required before support claim |
| Node.js binding | FUTURE | contract required before support claim |
| Java/JVM binding | FUTURE | contract required before support claim |
| Full native compiler backend | FUTURE | reference interpreter is the current execution oracle |
| Package registry | FUTURE | package contract not yet implemented |
| IDE/editor | FUTURE | not part of current executable core |
| Additional locales | FUTURE | Indonesian is the initial locale |

## Verified network boundary

The verified native runtime inventory remains DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit runtime limits. The latest cross-platform evidence is mirror run `38929072800` for `9038ac75f197753ff1ab6af7478e63e4831903a2` on Ubuntu and Windows Server 2025.

## Orchestration boundary

Four language-level vertical slices now exist, but their evidence boundaries differ:

```text
cari alamat "127.0.0.1"
→ parser → semantic → typed IR → target-scoped governance → network.dns.resolve → CLI

baca berkas "data/contoh.txt"
→ parser → semantic → typed IR → target-scoped governance → filesystem.read → CLI

jalankan proses "rustc"
→ parser → semantic → typed IR → target-scoped governance → process.execute → CLI

tulis ke berkas "<path>" dengan "<isi>"
→ parser → semantic → typed IR → target-scoped governance → filesystem.write → CLI
```

DNS and filesystem-read are backed by the recorded cross-platform evidence. Process execution is backed by exact-SHA Ubuntu + Windows evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2`. Filesystem write is backed by latest executable evidence at `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android; it is intentionally not labeled cross-platform verified.

## Current checkpoint

- Latest exact-SHA executable verification: `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android; contract, build, Clippy, targeted filesystem-write CLI, workspace tests, and offline verification all PASS.
- Latest cross-platform evidence remains `9038ac75f197753ff1ab6af7478e63e4831903a2` / mirror `38929072800`.
- PR #49: filesystem-write vertical slice merged before the CLI work.
- PR #50: filesystem.write CLI support merged into `main`.
- PR #51: documentation synchronized with filesystem.write verification evidence.
- PR #52: consistency audit synchronized the normative CLI contract and grammar with the implementation.
- GitHub hosted CI for the filesystem-write PR remained an infrastructure evidence failure with zero executed steps; it is not source-failure evidence.
- Filesystem write status: `EXPERIMENTAL`, limited to one text payload to one explicitly targeted file, with explicit `filesystem.write` permission.

## Release interpretation

`IMPLEMENTED` means the canonical source contains the implementation and associated tests. `VERIFIED` means executable evidence exists for the exact recorded source revision and stated platform boundary. `EXPERIMENTAL` means the declared vertical slice is executable and verified but intentionally limited in scope. `Supported` is reserved for a stronger support claim.

The independent verification infrastructure is the temporary public mirror `panglimanetizen01/one-windows-verification`. No GitHub paid upgrade was used.
