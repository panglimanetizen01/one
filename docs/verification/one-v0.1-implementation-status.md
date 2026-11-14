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
| Native Windows x86_64 backend | IMPLEMENTED / INDEPENDENT EXACT-SHA WINDOWS VERIFICATION | Windows Server 2025 workflow run #11 passed for source SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858`; ZIP, manifest, executable hashes, PE32+ AMD64 header, and native output were independently checked |
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


## CI provider split status — 2026-11-14

### Latest exact-SHA provider-split evidence — 2026-11-14

- Canonical ONE candidate: PR #95, branch `ci/circleci-migration-20261113`, exact source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642`, based on `main` `d35db95153f6ddd79885494c72f5aee16c899751`. PR #95 remains OPEN / DRAFT / UNMERGED.
- Independent verifier: PR #42, branch `verify/windows-pr95-20261114`, PR head `bc460d92d5e90e2808e1d835adf86bffad8a3463`. Windows run [#10](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43591754838) completed terminal SUCCESS on Windows Server 2025 / AMD64 for the exact canonical source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642`. Job: `132437775362`; manifest verifier checkout SHA: `07cd54d440c4f22910e4ed8417f4e6515339fb8d`.
- The manifest records run `43591754838`, attempt `1`, `job_status=success`, target `x86_64-pc-windows-gnu`, native exit code `0`, and output `Budi`. The workflow's contract/user-doc checks, workspace build, Clippy, complete tests, deterministic artifact checks, DNS/orchestration/database/HTTP checks, real-network tests, offline verification, native build, PE inspection, and execution all completed successfully.
- Artifact ID: `14113026885`; ZIP size: `15866` bytes; ZIP SHA-256: `9188dc0f81636e0e8dac536939eff9fcf0792e8d24b8b7f904beb74ff4fd1192`; manifest SHA-256: `b018c3719ff01fc58876250b32c9663f6f46b44dc1137e3f8e2b36f4d71a6ce8`; executable SHA-256: `22ef1752296a2cf1ce4ccc8a88e99274c73fd0eed65b5b27d34c9879eb6f3bc7`. The ZIP digest was independently recomputed; the manifest and executable hashes matched. The 52,918-byte executable independently passed MZ, PE, AMD64 `0x8664`, and PE32+ `0x20B` checks.
- CircleCI on the same canonical source SHA reports `ci/circleci: one-linux` FAILURE: workflow `bb9174a8-ac7f-42e1-84ed-1ce816267d64`, job `301edf07-748d-4adb-83b0-ba4dc8594ac9`, failure report `9c6078ca-186a-41e1-8d0e-386ac601952f`. No Windows CircleCI context is expected under the new ownership split.
- The authenticated CircleCI first-failure log is not accessible in this execution session; the failed step and root cause remain UNKNOWN. Do not infer a product-code defect or claim Linux PASS.
- Result: Windows is PASS for source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642` only. Overall cross-platform verification is NOT PASS while CircleCI Linux is failed. Keep PR #95 draft/unmerged and `main` unchanged.


## Latest post-split exact-SHA Windows verification — 2026-11-14

- Canonical source snapshot: `panglimanetizen01/one` SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858`, which was the PR #95 head when the run was triggered. The independent verifier PR #42 head is `09d5ce0c9653be91bceb6c99110802e138f7660a`; `WINDOWS_SOURCE_COMMIT.txt` pins the canonical source SHA exactly.
- Windows run [#11](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43592922789) completed terminal SUCCESS. Job `132441279904` ran on Windows Server 2025 / AMD64. The manifest records verifier checkout SHA `4435dbac04103c140b2ec14abc97d816ade28be9`, target `x86_64-pc-windows-gnu`, `job_status=success`, native exit code `0`, and output `Budi`.
- The workflow executed the canonical contract and user-document checks, including the new guard that reports `legacy GitHub Actions workflow files: 0`, then completed workspace build, Clippy, full workspace tests, deterministic artifacts, DNS/orchestration/database/HTTP checks, real-network tests, offline verification, Windows native build, PE inspection, and execution.
- Artifact ID: `14112773100`; ZIP size: `15869` bytes; ZIP SHA-256: `8ca9e99edd8ea7df80fd5cd1dc06d6d402bbddc13479bf0607bd8f2691fe1e56`; manifest SHA-256: `e9d9290dbfae95284b96435d3ad79f245d3d7fbe90b42a3e5994cdb352dff197`; executable SHA-256: `d2b0afa0f71fd2af831d75cdf02f1a8c7c5156654cf5a0b7fc6bcaa14cebdeaf`. The downloaded ZIP digest was independently recomputed and matched; the embedded manifest and executable hashes were independently checked. The 52,918-byte executable passed MZ, PE, AMD64 `0x8664`, and PE32+ `0x20B` checks.
- CircleCI on the same canonical source SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858` reports `ci/circleci: one-linux` FAILURE: workflow `ae7233e9-6146-417e-bef9-17b236a93716`, job `a8c73406-8277-4c6b-83d1-5d0ed85670a2`, failure report `47dd0f34-7bc4-43a7-8136-8a975b947fd6`. No Windows CircleCI job is expected under the split.
- The CircleCI job page and authenticated first-failure output are inaccessible through the current tools. The first failed Linux step/root cause remain UNKNOWN; do not infer a product-code defect or claim Linux PASS.
- Result: Windows PASS for canonical source SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858` only. Overall cross-platform verification is NOT PASS because CircleCI Linux is failed. PR #95 and PR #42 remain OPEN / DRAFT / UNMERGED; `main` is unchanged.
- The subsequent PR #95 update containing this evidence is documentation-only; the tested source snapshot remains the exact SHA pinned above. The pin is not a floating branch name.
