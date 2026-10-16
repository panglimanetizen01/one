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
| Language-level filesystem read orchestration | EXPERIMENTAL | `baca berkas` → parser → semantic validation → typed orchestration IR → governed `filesystem.read` adapter → normalized UTF-8 text result; explicit target-scoped permission required; independently verified on Ubuntu + Windows Server 2025 at canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` via mirror run `38903950612` |
| Language-level process execution orchestration | IN DEVELOPMENT | `jalankan proses` → parser → semantic validation → typed orchestration IR → governed `process.execute` adapter → normalized process output; explicit target-scoped permission required; implementation and TDD tests are present on the current development branch, but exact-SHA execution evidence is still pending |
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
| Current exact-SHA cross-platform verification | VERIFIED | independent mirror run `38903950612` checked out canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` exactly and passed Ubuntu + Windows contract checks, build, Clippy, workspace tests, real-network tests, deterministic artifacts, orchestration runtime, and platform-specific offline verification |
| Private canonical hosted CI execution | NOT REQUIRED FOR P0 EVIDENCE | independent mirror provides exact-SHA executable evidence; canonical main remains source of truth |
| Native C/C++ FFI | FUTURE | contract required before support claim |
| Python binding | FUTURE | contract required before support claim |
| Node.js binding | FUTURE | contract required before support claim |
| Java/JVM binding | FUTURE | contract required before support claim |
| Full native compiler backend | FUTURE | reference interpreter is the current execution oracle |
| Package registry | FUTURE | package contract not yet implemented |
| IDE/editor | FUTURE | not part of current executable core |
| Additional locales | FUTURE | Indonesian is the initial locale |

## Verified network boundary

The verified native runtime inventory is DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit runtime limits. Independent mirror run `38903950612` passes contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, deterministic artifact checks, orchestration runtime, and platform-specific offline verification on Ubuntu and Windows Server 2025 for canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb`.

## Orchestration boundary

Two language-level vertical slices are independently verified and remain experimental. A third process-execution slice is implemented and under verification:

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

One source: jalankan proses "rustc"
→ parser
→ semantic validation
→ typed orchestration IR
→ explicit target-scoped governance
→ process.execute adapter
→ normalized process output
→ CLI output
```

The CLI requires explicit permission for all three slices. No permission means denial; a different target is outside the granted scope. The process slice currently executes one explicitly targeted program without arguments. Its exact-SHA executable verification is not yet recorded, so it must not be promoted to `EXPERIMENTAL` or `VERIFIED` until that evidence exists. This is not a claim that the complete orchestration catalog is supported.

## Current checkpoint

- Canonical `main`: `ab7df1e2a0dbcc8c3fab3b425137194d1ca0ccbe`.
- Latest proven exact-SHA baseline: `f482c6b75505d9965dd33f4f25bb0cd04072cedb`, mirror run `38903950612`.
- Process/System branch: `feature/process-system-vertical-slice`.
- Current process branch head: `09aff2680661316f66222923863eae8f0e422296`.
- PR: #48, open and mergeable; source merge is blocked by the exact-SHA evidence rule, not by a documented source failure.
- CI run `38927364159` failed on Ubuntu and Windows with zero job steps; retry also failed with zero steps/logs. Classified as infrastructure evidence failure.
- Public verification mirror remains non-executable until its source tree exactly matches the recorded canonical SHA.
- Process execution status: `IN DEVELOPMENT`.
- Next gate: fresh exact-SHA Linux/Windows executable verification of the process branch, then reverify → review → merge → read-back main → checkpoint.

## Release interpretation

`IMPLEMENTED` means the canonical source contains the implementation and associated tests. `VERIFIED` means independent execution evidence exists for the exact recorded source revision. `EXPERIMENTAL` means the declared vertical slice is executable and verified but intentionally limited in scope. `Supported` is reserved for a stronger support claim.

The independent verification infrastructure is the temporary public mirror `panglimanetizen01/one-windows-verification`. No GitHub paid upgrade was used.
