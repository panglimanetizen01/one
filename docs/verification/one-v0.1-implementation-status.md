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
| Language-level filesystem write orchestration | EXPERIMENTAL | `tulis ke berkas` → parser → semantic validation → typed IR → `filesystem.write` governance → filesystem write adapter → CLI; exact-SHA executable evidence remains Termux/Android-bound |
| Language-level database query orchestration | EXPERIMENTAL | `kueri database` → parser → semantic validation → typed IR → resource-scoped `database.query` governance → SQLite adapter → normalized result → CLI; exact-SHA executable evidence remains Termux/Android-bound |
| Language-level HTTP GET orchestration | EXPERIMENTAL | `ambil dari` → parser → semantic validation → typed IR → target-scoped `network.connect` governance → `network.http.get` adapter → bounded HTTP/1.1 GET → normalized response body → CLI; exact-SHA executable evidence remains Termux/Android-bound |
| General language-level orchestration | IN DEVELOPMENT | additional capabilities and source operations are not yet implemented through the same end-to-end path |
| CLI | VERIFIED / PLATFORM-BOUND | canonical exact-SHA evidence remains platform-bound; native CLI target selection is covered by PR #64 tests |
| Project scaffolding | IMPLEMENTED | CLI project tests |
| Conformance fixtures | ADDED | `tests/conformance/**/*.one` |
| Conformance runner | ADDED | `crates/one-core/tests/conformance.rs` |
| Golden output contracts | ADDED | `tests/golden/*.snap` |
| Offline Linux verifier | ADDED | `tools/verify_offline.sh` |
| Offline Windows verifier | ADDED | `tools/verify_offline.ps1` |
| Release gate workflow | DEFINED | `.github/workflows/release-check.yml` |
| Linux/Windows CI definition | COMPLETE | `.github/workflows/ci.yml` |
| Native Linux AArch64 backend | IMPLEMENTED / CURRENT-HEAD VERIFICATION PENDING | PR #64 implementation evidence at `797c6fe75bd3fa9081329f73dd59e34278f27ddd`; merged into `main` at `ec844aa652a4d35c3891fc810d344c9c9db2f5cf`; current-main executable verification not yet recorded |
| Native Linux x86_64 backend | IMPLEMENTED / BUILD EVIDENCE PRE-MERGE | PR #64 produced ELF x86-64 at `797c6fe75bd3fa9081329f73dd59e34278f27ddd`; target execution unavailable; current `main` still requires exact-SHA verification |
| Native Windows x86_64 backend | IMPLEMENTED / BUILD EVIDENCE PRE-MERGE | PR #64 produced PE32+ x86-64 at `797c6fe75bd3fa9081329f73dd59e34278f27ddd`; Windows/Wine execution unavailable; current `main` still requires exact-SHA verification |
| Native backend contract | DEFINED + VERIFIED FOR PR #64 SCOPE | `docs/superpowers/specs/2026-10-19-native-backends-linux-windows.md` and native backend test suite |
| Canonical main merge state | MERGED / CURRENT-HEAD VERIFICATION PENDING | PR #64 merged at `ec844aa652a4d35c3891fc810d344c9c9db2f5cf`; later documentation-evidence commits are on `main` |
| Post-merge hosted CI | INFRASTRUCTURE FAILURE / INCONCLUSIVE | runs `39372356906`, `39381125667`, and latest observed `39440575308` at `main` SHA `34725aa393ca31759eb88c96688063f6358ce54a`; both matrix jobs failed before executable steps (`steps: []`) |
| Latest cross-platform exact-SHA verification | VERIFIED / HISTORICAL | mirror run `38929072800` checked out `9038ac75f197753ff1ab6af7478e63e4831903a2` exactly and passed Ubuntu + Windows verification; it predates later database, HTTP, filesystem-write, and native-backend changes |
| Native backend PR #64 | MERGED / CURRENT-HEAD VERIFICATION GATE | merge commit `ec844aa652a4d35c3891fc810d344c9c9db2f5cf`; native evidence remains bounded to the recorded pre-merge SHA until current-head verification exists |
| Native C/C++ FFI | FUTURE | contract required before support claim |
| Python binding | FUTURE | contract required before support claim |
| Node.js binding | FUTURE | contract required before support claim |
| Java/JVM binding | FUTURE | contract required before support claim |
| Full native compiler backend | FUTURE | current native work is a bounded C-codegen backend; broader compiler/runtime scope remains future work |
| Package registry | FUTURE | package contract not yet implemented |
| IDE/editor | FUTURE | not part of current executable core |
| Additional locales | FUTURE | Indonesian is the initial locale |

## Native backend evidence boundary

PR #64 exact revision `797c6fe75bd3fa9081329f73dd59e34278f27ddd` adds native hosted backends for Linux AArch64, Linux x86_64, and Windows x86_64. The semantic IR remains target-independent.

Linux AArch64 is **VERIFIED** for the stated AArch64/Linux host boundary because a native ELF artifact was built and executed successfully. Linux x86_64 is **BUILD TERBUKTI** because the artifact was produced and identified as ELF x86-64, but no x86_64 executor was available. Windows x86_64 is **BUILD TERBUKTI** because PE32+ x86-64 was proven, but no Windows/Wine executor was available.

These are implementation-SHA facts, not current-`main` support claims. PR #64 was merged at `ec844aa652a4d35c3891fc810d344c9c9db2f5cf`; current-head exact-SHA verification is required before promoting the native backend statuses for canonical `main`.

At merge SHA `100d0f9553be6352b8dbc76b3e6dad67f5a7a9e2`, post-merge CI run `39372356906` and its retry failed on both matrix jobs with zero executable steps. After documentation repair commit `5e618c2514a99093c8a7c1d40faf5af7f0baf2de`, run `39381125667` independently reproduced the same no-step failure. These are infrastructure/runner failures and must not be classified as Rust source failures.

## Release interpretation

`IMPLEMENTED` means the canonical source contains the implementation and associated tests. `VERIFIED` means executable evidence exists for the exact recorded source revision and stated platform boundary. `EXPERIMENTAL` means the declared vertical slice is executable and verified but intentionally limited in scope. `BUILD TERBUKTI` means the native artifact was produced and structurally validated but target execution evidence is still missing. `Supported` is reserved for a stronger support claim.
