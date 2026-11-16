# One v0.1 Implementation Status


## Current exact-SHA CI candidate — 2026-11-16

- PR #95 candidate: `panglimanetizen01/one`, branch `ci/circleci-migration-20261113`, exact source SHA `07d8cbf7422b8340f8efda3c2f9d4425f85b7537`; base `main` `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI `ci/circleci: one-linux`: PASS; workflow `0e03e28b-dfb9-4198-b897-3ce5d16dc2d8`; job `b2f536a1-6c87-49b5-9a3f-2db2f2908595`.
- Independent Windows Server 2025 run [#28](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43865524749): PASS, 32/32 steps; native exit code 0; output `Budi`; verifier SHA `a83e05437a7d0612f1bf6f5dcc0c7b39483a8173`.
- Windows artifact ZIP SHA-256 `7dd4435e41bc8f0660f454768711c0fc460e2ca0fe38f2dee1034c59c0f767a5`; executable SHA-256 `9004e87f0c9d0357ee0197a1bce2a26474cb714aa8d71d54990236c9266dcfeb`; manifest SHA-256 `d95ae44ee24c5b793aeef59fb006e48d4c5b3e62de4cd5012b2b5a769cbf92a4`.
- Both platform gates PASS for this exact source SHA. The optional `release-check` has not run. PR #95 remains OPEN / DRAFT / UNMERGED; `main` is unchanged and still contains the legacy GitHub Actions workflows. This is candidate evidence, not merge/release approval or proof of 100% project completion.

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
| Release gate workflow | DEFINED | `.github/workflows/release-check.yml` |
| Linux/Windows CI definition | COMPLETE | `.github/workflows/ci.yml` |
| Native Linux AArch64 backend | IMPLEMENTED / LATEST SOURCE SHA MATRIX VERIFIED | Last code-bearing source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` passed the exact-SHA Ubuntu/Windows matrix `39554661451`; AArch64-target execution remains bounded to its recorded AArch64/Linux evidence |
| Native Linux x86_64 backend | IMPLEMENTED / LATEST SOURCE SHA MATRIX VERIFIED | Last code-bearing source SHA passed build, Clippy, workspace and native-backend test coverage in matrix `39554661451`; retain target-specific execution boundaries |
| Native Windows x86_64 backend | IMPLEMENTED / LATEST SOURCE SHA MATRIX VERIFIED | Last code-bearing source SHA passed Windows build, Clippy, workspace and native-backend test coverage in matrix `39554661451`; retain target-specific execution boundaries |
| Native backend contract | DEFINED + VERIFIED FOR LATEST SOURCE SHA TEST MATRIX | `docs/superpowers/specs/2026-10-19-native-backends-linux-windows.md`, native backend tests, exact-SHA matrix |
| Canonical main merge state | MERGED / EXACT-SHA MATRIX VERIFIED | Last code-bearing canonical source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` verified in run `39554661451` |
| Post-merge hosted CI | INFRASTRUCTURE FAILURE / INCONCLUSIVE | runs `39372356906` and `39381125667` failed before executable steps (`steps: []`); independent matrix `39554661451` passed |
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

## Hosted CI infrastructure status — 2026-11-16

Hosted-CI evidence is tracked separately from product implementation status. Historical Windows job #254 at source SHA e44d16b2b5f4ef50c33674ccf711bd11b410effa executed the Windows environment and Rust setup, then failed in MSYS2 package synchronization because the extracted environment's first-run/keyring initialization was missing. This is a toolchain-bootstrap failure on a superseded workflow, not a ONE source/test failure. Do not reintroduce that Windows/MSYS2 job into the current ONE CircleCI configuration; Windows host verification is separated into the independent verification repository.

CircleCI jobs #409, #424, and #426 have explicit `free-plan-no-credits-available` responses and are PLAN-CREDIT-BLOCKED; jobs #407 and #408 did not demonstrate executable test steps and retain unknown root causes. Do not label any of these as source failures. Independent Windows verifier run #21 succeeded only for its pinned ONE source SHA f85375e279257dcbb7d4ded9632800fcd1b78cc9. Full job-level evidence and identifiers are in [the incident record](circleci-infrastructure-incident-2026-11-16.md). Apply [the CI verification evidence contract](../spec/one-v0.1-ci-verification-contract.md) to future classifications.
