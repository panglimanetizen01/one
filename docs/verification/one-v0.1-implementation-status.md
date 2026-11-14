# One v0.1 Implementation Status

| Area | State | Evidence in canonical repository |
|---|---|---|
| Foundation | ACCEPTED / NORMATIVE | `docs/foundation/one-v0.1-foundation.md` + accepted language invariants; implementation conformance is tracked separately |
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
| Language-level filesystem write orchestration | EXPERIMENTAL | `tulis ke berkas` → parser → semantic validation → typed IR → `filesystem.write` governance → filesystem write adapter → CLI; Termux evidence plus source/core/CLI tests in Ubuntu/Windows exact-SHA matrix `39554661451` at `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` |
| Language-level database query orchestration | EXPERIMENTAL | `kueri database` → parser → semantic validation → typed IR → resource-scoped `database.query` governance → SQLite adapter → normalized result → CLI; Termux evidence plus source/core/CLI tests in Ubuntu/Windows exact-SHA matrix `39554661451` at `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` |
| Language-level HTTP GET orchestration | EXPERIMENTAL | `ambil dari` → parser → semantic validation → typed IR → target-scoped `network.connect` governance → `network.http.get` adapter → bounded HTTP/1.1 GET → normalized response body → CLI; Termux evidence plus source/core/CLI tests in Ubuntu/Windows exact-SHA matrix `39554661451` at `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` |
| General language-level orchestration | IN DEVELOPMENT | additional capabilities and source operations are not yet implemented through the same end-to-end path |
| CLI | VERIFIED / PLATFORM-BOUND | canonical exact-SHA evidence remains platform-bound; native CLI target selection is covered by PR #64 tests |
| Project scaffolding | IMPLEMENTED | CLI project tests |
| Conformance fixtures | ADDED | `tests/conformance/**/*.one` |
| Conformance runner | ADDED | `crates/one-core/tests/conformance.rs` |
| Golden output contracts | ADDED | `tests/golden/*.snap` |
| Offline Linux verifier | ADDED | `tools/verify_offline.sh` |
| Offline Windows verifier | ADDED | `tools/verify_offline.ps1` |
| Release gate workflow | CIRCLECI LINUX-ONLY | `.circleci/config.yml` parameter `run_release_checks` runs `release-linux`; Windows host checks are separate and do not imply release approval |
| Linux/Windows CI definition | CIRCLECI LINUX-ONLY; WINDOWS HOST VERIFICATION INDEPENDENT | `.circleci/config.yml` wires `one-linux` and `release-linux` only; Windows Server 2025 verification is in `panglimanetizen01/one-windows-verification` with `WINDOWS_SOURCE_COMMIT.txt` and a separate source/verifier manifest |
| Native Linux AArch64 backend | IMPLEMENTED / LATEST SOURCE SHA MATRIX VERIFIED | Last code-bearing source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` passed the exact-SHA Ubuntu/Windows matrix `39554661451`; AArch64-target execution remains bounded to its recorded AArch64/Linux evidence |
| Native Linux x86_64 backend | IMPLEMENTED / LATEST SOURCE SHA MATRIX VERIFIED | Last code-bearing source SHA passed build, Clippy, workspace and native-backend test coverage in matrix `39554661451`; retain target-specific execution boundaries |
| Native Windows x86_64 backend | IMPLEMENTED / INDEPENDENT EXACT-SHA WINDOWS VERIFICATION | Windows Server 2025 workflow run #9 passed for source SHA `195755054bb733f44ff30acf8d3281b4efc820bf`; subsequent source SHAs require their own run and manifest/artifact validation |
| Native backend contract | DEFINED + VERIFIED FOR LATEST SOURCE SHA TEST MATRIX | `docs/superpowers/specs/2026-10-19-native-backends-linux-windows.md`, native backend tests, exact-SHA matrix |
| Canonical main merge state | MERGED / EXACT-SHA MATRIX VERIFIED | Last code-bearing canonical source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` verified in run `39554661451` |
| Historical GitHub-hosted CI | INFRASTRUCTURE FAILURE / INCONCLUSIVE | runs `39372356906` and `39381125667` failed before executable steps (`steps: []`); independent matrix `39554661451` passed |
| Latest cross-platform exact-SHA verification | VERIFIED / LATEST SOURCE SHA | mirror run `39554661451` checked out exact source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` and passed on Ubuntu + Windows Server 2025 |
| Native backend PR #64 | MERGED / LATEST SOURCE SHA MATRIX VERIFIED | PR #64 source is on canonical `main`; exact-SHA source test matrix is green while individual target execution claims remain evidence-bounded |
| Native C/C++ FFI | FUTURE | contract required before support claim |
| Python binding | FUTURE | contract required before support claim |
| Node.js binding | FUTURE | contract required before support claim |
| Java/JVM binding | FUTURE | contract required before support claim |
| Full native compiler backend | FUTURE | current native work is a bounded C-codegen backend; broader compiler/runtime scope remains future work |
| Package registry | FUTURE | package contract not yet implemented |
| IDE/editor | FUTURE | not part of current executable core |
| Additional locales | FUTURE | Indonesian is the initial locale |

## Native backend evidence boundary

PR #64 added native hosted backends for Linux AArch64, Linux x86_64, and Windows x86_64 while keeping Semantic IR target-independent. The last code-bearing source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` passed the independent exact-SHA Ubuntu/Windows matrix `39554661451`, including build, Clippy, workspace/native-backend tests, toolchain provisioning, and platform-appropriate offline verification.

The earlier AArch64/Linux execution result remains valid only for its recorded AArch64/Linux host and implementation SHA. The host matrix verifies that source revision's build and test suites, but does not by itself establish execution of every cross-compiled target architecture. Keep individual target claims limited to the artifacts and executors actually tested.

Canonical hosted CI runs `39372356906` and `39381125667` failed on both matrix jobs before executable steps (`steps: []`). These are infrastructure/runner failures and must not be classified as Rust source failures; the independent exact-SHA matrix is the current executable evidence.

## Release interpretation

`IMPLEMENTED` means the canonical source contains the implementation and associated tests. `VERIFIED` means executable evidence exists for the exact recorded source revision and stated platform boundary. `EXPERIMENTAL` means the declared vertical slice is executable and verified but intentionally limited in scope. `BUILD TERBUKTI` means the native artifact was produced and structurally validated but target execution evidence is still missing. `Supported` is reserved for a stronger support claim.

 
## CircleCI migration — latest forensic correction (2026-11-13)

The migration branch now removes all three legacy GitHub Actions workflows and has a static guard for shell-specific commands. A forensic inspection found and removed seven PowerShell `$env:PATH` assignments from the Bash `one-linux` job. The contract checker now fails if `$env:` appears in that Linux job. The corrected candidate SHA `2095098ea0e97da1c06c3f260f1f95218a06a339` triggered workflow `05a7e587-645d-4809-95a6-97d528c33507`; Linux and Windows were both pending at last poll. This is not yet CircleCI PASS evidence. Prior Linux failure on `ebf6a41f...` returned failure-report pipeline `0cb3e736-20ac-452c-a566-7e7b5a2f33d6`; exact failed step remains unknown because logs are not accessible in this session.

 
The Windows native-verification gate was strengthened to validate the PE32+ optional-header magic (`0x20B`) in addition to the DOS MZ signature, PE signature, and AMD64 machine field (`0x8664`). The static contract checker requires this guard. This is a verification-contract improvement; CircleCI execution remains unproven until the exact-SHA job passes.

 
The latest exact-SHA CircleCI retry on `36a4051913c0063f0237bf41cf4b25ebc250e493` started workflow `e42b0304-1241-4327-807d-03926d7d0422`: Linux job `65187706-c4ac-4142-89b0-d1c93d7fa36c` and Windows job `d51c952e-6484-4f6a-9375-bc57576110c6` were PENDING at last poll. The candidate includes the Bash shell correction and PE32+ optional-header guard. Previous candidate `2095098...` failed both jobs; failure-report pipeline `991dcbb1-49e9-4b65-acb1-9083caf1f24f`; exact root cause unknown without logs.

 
Latest CircleCI candidate `54386f415062d4c21a7acb5508d90fea4ba5cdef` triggered workflow `ca11e9e9-d870-43d8-8ea0-88949a1f3f62`; Linux `d5947530-f1bb-4136-b137-f037a72b99a3` and Windows `1739d1cc-b23c-40a6-924b-201616b4af6a` were pending at last poll. Structural audit passed (60 required paths, 78 index refs, fixtures 6/6/3, all CI markers, Linux Bash guard, PE32+ guard, zero GitHub Actions files); the Python checker itself has not been run outside CircleCI.
