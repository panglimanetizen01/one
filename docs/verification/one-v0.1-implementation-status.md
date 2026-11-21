# One v0.1 Implementation Status

## Live integration checkpoint — 2026-11-17

- PR #108 now includes the six implementation/test paths carrying the independently verified PR #98–#101 fixes. Integration source commit: `e8cd41ec212c3224bdbecf85eaf18c5850096e16`; subsequent commits synchronize normative specifications.
- Included: safe manifest name/version components and `one test` manifest validation (#98); SQLite read-only open and missing-file non-creation regression (#99); lexical-scope restoration after errors (#100); inherited filesystem denial/approval semantics (#101).
- The integrated candidate has NOT been executed. No parent-branch PASS result is transferred. Combined Ubuntu/Windows exact-SHA validation is PENDING; CircleCI status/job evidence is unavailable in this session.
- PR #108 remains OPEN/DRAFT and targets PR #102 branch `fix/commit-cargo-lockfile-20261117`. Main remains unchanged. Issue #104/#105/#82 remain open. DoD remains 3 checked / 10 unchecked; E6 remains NOT IMPLEMENTED.


## Current forensic status — 2026-11-17

This live checkpoint supersedes older "current" SHA/status statements below. Historical evidence remains preserved but is not transferred to later source revisions.

### Exact-SHA evidence
- PR #98: `a83321bc41b3d0d28378841e29a6169abed8ac15`, GitHub Actions run #2018 / `43923604400`: Ubuntu + Windows Server 2025 SUCCESS.
- PR #99: `8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9`, run #2019 / `43923948205`: Ubuntu + Windows Server 2025 SUCCESS.
- PR #100: `6375356c9c275ace5fd35cf4a81799a9546e1e1d`, run #2020 / `43924465030`: Ubuntu + Windows Server 2025 SUCCESS.
- PR #101: `cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f`, run #2138 / `44032422058`: Ubuntu + Windows Server 2025 SUCCESS.
- PR #102: `6c72d8e548bbbc28d85be39b8de427614989a038`, run #2229 / `44086429416`: Ubuntu + Windows Server 2025 SUCCESS; bounded subprocess capture/deadline and Windows Job Object regression passed.
- PR #107: `21e393157e52728d9510699ac6c9120987e3852c`, run #2230 / `44088328030`: Ubuntu + Windows Server 2025 SUCCESS; bounded Unix reader shutdown regression passed.
- PR #103 ledger: `041b359b8504551c2c1705093050d9c467368bac`, run #2231 / `44089449162`: Ubuntu + Windows Server 2025 SUCCESS.

### Integration and release boundaries
- All PRs above remain OPEN/DRAFT/UNMERGED. Their passing results are branch/SHA-specific and do not prove that the separate fixes are integrated together.
- PR #108 is the CircleCI migration candidate; its CircleCI job execution is NOT VERIFIED. GitHub `main` remains on the three GitHub Actions workflows and has no `.circleci/config.yml`.
- Issue #104 filesystem root confinement and Issue #105 strong Unix descendant containment remain OPEN. The 100 ms reader shutdown bounds waiting but does not kill an escaped descendant. Issue #82 Android AArch64 execution remains OPEN.
- E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED. DoD remains 3 checked / 10 unchecked / 13 total.

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

## Live security and CI checkpoint — 2026-11-18

- PR #110 HEAD `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b` has GitHub CircleCI status `ci/circleci: one-linux` = SUCCESS (job `c9916871-9746-4c65-b8fb-615d9f96dc78`). This is a Linux CircleCI status only; expanded job logs were not inspected in this checkpoint.
- Issue #104 remains OPEN and its required production behavior is NOT IMPLEMENTED at this SHA: `FileReadAdapter` still calls `std::fs::read(path)`; `FileWriteAdapter` still calls `std::fs::write(path, ...)`; both constructors remain parameterless. Do not label filesystem root confinement IMPLEMENTED, TESTED, VERIFIED, or SUPPORTED.
- PR #108 HEAD `286a0fd563e10611a3465cb6457ab069a0df8a52` has GitHub CircleCI status `ci/circleci: one-linux` = SUCCESS (job `99216205-62aa-4e98-af42-d4b742a5feaa`).
- PR #108 and #110 remain OPEN / DRAFT / UNMERGED. Canonical `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`.
- The independent verifier repository main SHA is `3d915528e7771d9443ccf0da7e4d7c9e25231537`; no verifier workflow or source-pinning changes were made in this checkpoint.
- DoD/E1–E6 and release status are not promoted by these scoped CircleCI statuses. E6 remains NOT IMPLEMENTED unless a newer authoritative ledger proves otherwise.



## Documentation commit CI — 2026-11-18

- Current PR #110 HEAD `26d29c61944e51919161f5360bccfef4a09d370d`: CircleCI `ci/circleci: one-linux` SUCCESS; job `c153ba39-4073-4538-aaa6-6370e6bed6bf`.
- This is a documentation-only commit; it does not change Issue #104 production source. The source assessment remains bound to parent `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b`.


## Issue #104 live implementation status — PR #113 candidate, 2026-11-18

This addendum supersedes the older Issue #104 status bullets above where they describe PR #110 / pre-implementation source. It does not change canonical `main`, acceptance pins, or release status.

- Canonical `main` remains unchanged and does not yet contain the PR #113 filesystem authority implementation.
- PR #113 is OPEN / DRAFT / UNMERGED. Its latest documentation/source HEAD at this checkpoint is `02d552123e3cf33017e2066fe62e304e1573476b`; its latest code-bearing change is in ancestor `5c5bfb53e086222f89dc8178426a8c6b65b50a04`.
- PR #113 candidate implements host-injected `FilesystemAuthority`, Linux handle-relative `openat2` reads, Windows handle-relative traversal, bounded I/O, and a Windows temporary-sibling + handle-relative rename write path. Linux write and unsupported platform writes remain fail-closed.
- The atomic write path is IMPLEMENTED in the candidate source, with positive write and denied-replacement cleanup tests. It is NOT VERIFIED on the latest code-bearing SHA until Windows build/tests are run there. Failure injection for write/flush before rename and the full concurrent replacement/reparse test set remain required.
- E4, Windows native reparse probe, full Ubuntu/Windows, SEC-05, Debian/Ubuntu, and Android artifact evidence passed on prior code SHA `4c3bccaf59d965ce8b446e0b84cb84c00ff2eed6`; those results do not transfer to the newer code SHA `5c5bfb53e086222f89dc8178426a8c6b65b50a04`.
- Android API 35 AArch64 runtime run `44327291656` still has its emulator boot step IN_PROGRESS and pins the earlier SHA. It is not runtime evidence for the newer candidate.
- CircleCI `ci/circleci: one-linux` is PENDING for latest HEAD `02d552123e3cf33017e2066fe62e304e1573476b`; job: https://app.circleci.com/workflow/a87975d6-b9c9-4fa4-b18a-bf92c2235045/job/49fb1abd-3870-4332-bcd0-eb370edc24ab. No CircleCI log access is available in this session; no root cause is inferred.
- User documentation and the Issue #104 design have been updated on PR #113 to describe the candidate/main boundary and the unverified status. The source/rendered architecture diagram was not inspected and is not claimed updated.
- Classification: filesystem root confinement is IMPLEMENTED / PARTIAL in the unmerged candidate; prior exact-SHA E4 checks are VERIFIED only for their recorded SHA; latest candidate is NOT VERIFIED; platform-wide support and acceptance remain NOT ACCEPTED. Keep Issue #104 OPEN.


## Issue #104 latest candidate checkpoint — 2026-11-18

The latest PR #113 source HEAD at this checkpoint is `f6acb23a2a0038281c613fb6fd187bc04d2f3dc9`. PR #113 remains OPEN / DRAFT / UNMERGED; canonical `main` is unchanged.

- Candidate implementation includes host-injected `FilesystemAuthority`, Linux root-handle-relative `openat2` reads, Windows handle-relative traversal, bounded I/O, and a Windows temporary-sibling atomic replacement path.
- The first Win32 rename form failed on Windows Server 2025 at source `5c5bfb...`; it was replaced with `NtSetInformationFile(FileRenameInformation)`.
- E4 run [#44329905887](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44329905887) PASS on exact source SHA `20ede9fb1d3e97c04f81d4fe8f79f642a8cc8e78`, including the positive write, size limit, denied replacement cleanup, Unix adversarial checks, and Windows native reparse probe.
- Latest code-bearing source `f6acb...` changes the rename target to a same-directory leaf name with `RootDirectory = NULL`. This code change has not yet been covered by the previous PASS. Fresh diagnostic verifier workflows are queued against `f6acb...`; do not classify latest HEAD as VERIFIED yet.
- CircleCI `ci/circleci: one-linux` is PENDING for `f6acb...`; job: https://app.circleci.com/workflow/a040bfd4-9db5-4fb4-9b33-4c1cda53e6c6/job/7931ea7a-5f7b-4d85-b8fa-ab26c7188273.
- Android runtime remains unverified for latest HEAD; the new artifact/runtime workflows are queued. The older Android runtime run pinned an earlier candidate and is not transferable.
- User guide and Issue #104 design addenda distinguish canonical main from the draft candidate and explicitly record the source-SHA boundary.
- Classification: candidate implementation PARTIAL / IMPLEMENTED; E4 VERIFIED on `20ede9...` only; latest candidate NOT VERIFIED; platform support and Issue #104 acceptance NOT ACCEPTED. Keep Issue #104 OPEN and preserve verifier protected pin.


## Windows atomic-denial regression cleanup — 2026-11-18

- Source SHA `f6acb23a2a0038281c613fb6fd187bc04d2f3dc9` passed E4 (run #44330374509) for its code at that revision, including the Windows atomic write success and denied replacement tests.
- The cross-platform Windows Clippy job then exposed a test-only lint: `permissions.set_readonly(false)` triggered `clippy::permissions_set_readonly_false`.
- Replaced the permission reset with a real Windows share-mode blocker that omits `FILE_SHARE_DELETE`, preserving the negative atomic replacement test's intended semantics.
- Fix source SHA: `a79665056149bf1ff302de79a7b86a346e0353d8`. Diagnostic verifier pin now targets this SHA; E4 [#44331243475](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44331243475) is QUEUED. CircleCI `ci/circleci: one-linux` is PENDING on this SHA.
- Classification: source fix IMPLEMENTED; latest SHA NOT VERIFIED pending E4 and cross-platform CI. Do not transfer the earlier `f6acb...` E4 PASS to `a796650...`. Issue #104 remains OPEN / NOT ACCEPTED.


## Linux direct-child atomic write candidate — 2026-11-18

- Source `cc54597b471493f4350d0dc0f3a2896babb33ea4` adds a Linux write backend for direct children of the retained trusted root. It validates the leaf through `openat2` with BENEATH / NO_SYMLINKS / NO_XDEV, creates an exclusive temporary sibling relative to the root fd, writes + syncs, and atomically replaces with `renameat`.
- Nested Linux write paths return `UnsupportedPlatform` intentionally. Opening a nested directory and retaining its handle does not prevent an attacker from moving that directory outside the root; no nested write is attempted until that race is safely addressed.
- New Linux tests cover relative and absolute in-root create/replace, symlink-leaf rejection with outside sentinel preservation, oversized-content preservation, nested-parent fail-closed behavior, and real `/dev/shm` mount-boundary read denial.
- Independent E4 script now requires five Linux `descriptor_relative_write_*` tests and three Linux `descriptor_relative_read_rejects_*` tests. Diagnostic verifier pin is `cc54597...`; exact-SHA E4 run was queued at last inspection.
- Classification: Linux direct-child atomic write IMPLEMENTED / PARTIAL; NOT VERIFIED on `cc54597...` pending exact-SHA E4 and cross-platform tests. Nested Linux write NOT IMPLEMENTED / fail-closed. Issue #104 remains OPEN / NOT ACCEPTED.


## Linux writer compile correction — 2026-11-18

- Candidate `cc54597...` failed the Android AArch64 artifact build at compile time with four stale `parent_handle` references in the new Linux writer. No artifact was uploaded and no runtime test ran.
- Corrected the Linux temp creation, cleanup guard, and rename calls to use the retained root fd in source SHA `090c84dbab02e48af0a493faeb8d4fe0ff80a876`.
- Diagnostic verifier pin now targets `090c84d...`. E4 [#44332808624](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808624), cross-platform [#44332808543](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808543), Linux distro [#44332808528](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808528), Android artifact [#44332808717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808717), Android runtime [#44332808569](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808569), SEC-05 [#44332808586](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808586), resource guard [#44332808575](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808575) are queued/pending.
- CircleCI `ci/circleci: one-linux` PENDING on `090c84d...`: https://app.circleci.com/workflow/60cfc357-04fc-47e2-8a5c-6cd85784c6af/job/a3279063-9cb6-4c54-8125-9b1b3b89ad8e.
- Classification: Linux direct-child atomic writer remains IMPLEMENTED / PARTIAL but NOT VERIFIED on `090c84d...`; nested writes remain NOT IMPLEMENTED / fail-closed. Issue #104 OPEN / NOT ACCEPTED.


## Android AArch64 artifact evidence on corrected Linux-writer source — 2026-11-18

- Android artifact run [#44332808717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808717) PASS on exact source SHA `090c84dbab02e48af0a493faeb8d4fe0ff80a876`.
- Job verified source pin, built and inspected an ELF64 AArch64 executable using the Android linker `/system/bin/linker64`, and uploaded the artifact (artifact ID 14438938784; SHA-256 `5971ef900ead08e6f571327594625ba2856a1917d59196d095c785bdf51a0ea4`).
- This is artifact/build evidence, not Android API 35 runtime verification or a platform support claim. Android runtime remains queued; E4/cross-platform/distro/SEC-05 and CircleCI remain pending.


## Strict Clippy correction — 2026-11-18

- Source `090c84dbab02e48af0a493faeb8d4fe0ff80a876` compiled and passed Android AArch64 artifact build [#44332808717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808717).
- Ubuntu 24.04 distro job [#44332808528](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808528) then failed Clippy with `clippy::redundant_field_names` in the Linux cleanup-guard initializer.
- Fixed with field shorthand in source SHA `3d8e0d0e3af54310837eb06d442d30a02df821d0`. Diagnostic verifier pin now targets this SHA; E4 [#44333965599](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44333965599) is QUEUED. Fresh exact-SHA artifact/platform checks remain pending.
- Focused filesystem architecture Mermaid source is now present at `docs/architecture/filesystem-root-confinement.md`, linked from `docs/ARCHITECTURE.md` and `docs/README.md`. Full PR #97 overview image remains visually NOT VERIFIED.
- Classification: Linux direct-child atomic writer IMPLEMENTED / PARTIAL; current source NOT VERIFIED. Nested Linux writes remain fail-closed. Issue #104 OPEN / NOT ACCEPTED.


## CLI integration alignment for Linux write scope — 2026-11-18

- Debian 12 workspace run [#44333980452](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44333980452) failed in `one-cli/tests/filesystem_write.rs` because a `cfg(not(windows))` test still expected all Linux writes to be unsupported. The Linux backend now supports direct-child writes, so that expectation was obsolete.
- Updated CLI tests: Linux direct-child absolute target must succeed; Linux nested relative target must fail closed and preserve the original file; other non-Windows/non-Linux targets remain unsupported.
- Source SHA: `2fa1436daa102d3cf32125da6c440ae19cc83ba7`. Diagnostic verifier pin now targets this SHA.
- Fresh E4 [#44334598268](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44334598268), cross-platform [#44334598774](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44334598774), distro [#44334598316](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44334598316), Android artifact [#44334598355](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44334598355), Android runtime [#44334598284](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44334598284), SEC-05 [#44334598278](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44334598278), and resource guard [#44334598250](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44334598250) are queued/pending.
- CircleCI `ci/circleci: one-linux` PENDING on `2fa1436...`: https://app.circleci.com/workflow/8f351193-5da8-4b6a-ad2f-75075c6a51e7/job/604385cb-7f5f-4c2f-8a38-92cc8330b83f.
- Classification: CLI test alignment IMPLEMENTED; current source NOT VERIFIED pending exact-SHA tests. Issue #104 remains OPEN / NOT ACCEPTED.


## CLI host-root integration — 2026-11-18

- Found and fixed a security mismatch: `filesystem_authority_for_target` previously derived the root from the requested absolute target's parent, allowing the target to influence its own authority boundary.
- CLI now derives root from source/project context (nearest `one.toml` ancestor; otherwise working directory for sources beneath it, or canonical source parent for standalone sources outside cwd). Requested targets are checked against this fixed root for both read and write.
- Added integration tests for outside-root absolute write rejection with sentinel preservation, outside-root read rejection without disclosure, relative source path handling, Linux direct-child success, and nested Linux fail-closed behavior.
- Source SHA: `79a98861080fed2ceaa0a7a2f56e07d6fbc19fca`; verifier pin updated to this exact SHA. Current exact-SHA workflows are pending.
- Classification: host-root integration IMPLEMENTED; NOT VERIFIED pending CI. Issue #104 remains OPEN / NOT ACCEPTED.


## Manifest-root selection regression coverage — 2026-11-18

- Added cross-platform CLI test `run_command_uses_nearest_one_manifest_as_filesystem_root`: source under `src/main.one`, nearest `one.toml` ancestor chosen as root, direct-child output written under that root.
- Source SHA: `1b7c3bb1dbc29a33c086b4df21a9996025c1e075`; verifier pin updated. Exact-SHA E4, cross-platform, distro, and Android workflows remain pending.
- Host-root integration remains IMPLEMENTED / NOT VERIFIED until the latest SHA's CI completes.


## Project-root ancestor lookup correction — 2026-11-18

- Source `722d662e7a81711ba3607d26571f4048a292b3ed` removes the premature stop at current working directory while searching for the nearest ancestor `one.toml`.
- This ensures a source located directly under a project subdirectory still resolves to a manifest above cwd when present.
- Diagnostic verifier pin targets `722d662...`; fresh E4 and workspace verification pending. Host-root integration remains IMPLEMENTED / NOT VERIFIED.


## Exact-SHA checkpoint refresh — 18 November 2026

The latest code-bearing candidate remains `722d662e7a81711ba3607d26571f4048a292b3ed`. The current PR #113 HEAD is a documentation-only descendant `1eebed322c696fe4f958578a3c7947abce75e173`; the focused architecture diagram now explicitly shows retained Windows root/parent handles opened without `FILE_SHARE_DELETE` to prevent parent-directory rename races during the handle-relative walk and atomic replacement.

The independent diagnostic verifier pin is confirmed as `722d662...`. Fresh exact-source verification is still incomplete: Linux distro run #44335797294 completed SUCCESS: Debian 12 and Ubuntu 24.04 jobs both passed; E4 #44335797344 is queued; cross-platform #44335797200 is pending; Android artifact #44335797320, Android runtime #44335797240, SEC-05 #44335797365, and resource guard #44335797191 are queued. CircleCI `ci/circleci: one-linux` is PENDING on source `722d662...`: https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94.

Classification remains IMPLEMENTED / NOT VERIFIED for the CLI host-root resolver and Linux direct-child writer; Windows handle-relative atomic replacement has historical E4 evidence only on earlier exact source SHA and needs the current exact-SHA run. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 OPEN / DRAFT / UNMERGED; canonical main and protected verifier pin remain unchanged.


## Exact-SHA verification refresh — 2026-11-18

Code candidate: 722d662e7a81711ba3607d26571f4048a292b3ed. The independent diagnostic verifier pin matches this exact source.

- E4 Unix + Windows filesystem adversarial matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797344.
- Windows Server 2025 build 26100 native probe: ordinary file open with OBJ_DONT_REPARSE succeeded; direct and intermediate junctions were denied with STATUS_REPARSE_POINT_ENCOUNTERED (0xC000050B); outside sentinel remained unchanged.
- Seven Windows filesystem regressions passed, including in-root absolute read, outside-root confinement without side effects, normal atomic write, oversized-content rejection, denied atomic replacement preserving original bytes and cleaning the temp sibling, junction escape rejection, and parent traversal rejection.
- Ubuntu + Windows Server 2025 cross-platform matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797200.
- Debian 12 + Ubuntu 24.04 distro matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797294.
- Android AArch64 artifact build: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797320; artifact ID 14442665831; archive SHA-256 3bc37c20a92087464abb087be1fe174fac314dbddd635116656e34ad1cee4123. Android API 35 ARM64 runtime, run #44335797240, remains IN PROGRESS.
- SEC-05 Linux + Windows: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797365. The targeted interop.rs formatting check passed; the separate whole-workspace formatting diagnostic reports diffs but is configured non-blocking.
- Subprocess resource guard: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797191.

CircleCI one-linux on source SHA 722d662 reports FAILURE: https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94. The prior documentation HEAD b94e85db5d83c9a4bec582609405d9c65e4db841 separately reports PENDING: https://app.circleci.com/workflow/a09cd902-a88f-4884-a212-148ba0c6b80c/job/125775a6-33ba-4bac-b5f5-74249b8e0a19. CircleCI logs are unavailable through the current tools; failure cause is unknown and is not inferred.

Classification: host-root resolver, Linux direct-child writer, Windows adapter regressions, E4, cross-platform, distro, SEC-05, and resource guard are VERIFIED on the exact source SHA and stated environments. Linux nested writes remain fail-closed. Windows native probe evidence is limited to Windows Server 2025 build 26100. Android artifact is BUILD-VERIFIED only; Android runtime is not yet verified. CircleCI remains unresolved. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 remains OPEN / DRAFT / UNMERGED; main and the protected verifier pin remain unchanged.


## CircleCI status correction — 2026-11-18

A fresh combined-status read confirms that CircleCI `ci/circleci: one-linux` is FAILURE on the code-bearing source `722d662e7a81711ba3607d26571f4048a292b3ed` (job https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94). The previously recorded documentation HEAD `b94e85db5d83c9a4bec582609405d9c65e4db841` is PENDING, and its descendant documentation HEAD `add130412cf42088ca9171fa2660295f7c30ae65` is also PENDING (job https://app.circleci.com/workflow/bfe4df8d-85a8-49bc-b7f2-21910a4692cd/job/d2aa7f1d-0c7a-4e7b-8ba4-ecaf595674c2). The failure cause on the code SHA remains UNKNOWN because CircleCI logs are inaccessible through the current tools. This correction does not change the exact-source independent verification results or Issue #104 acceptance classification.


## Issue #104 current implementation and verification override — 2026-11-19

This dated checkpoint supersedes earlier status summaries that predate code-bearing candidate SHA `722d662...` or say E4 is queued. Preserve historical test results as exact-SHA records.

- Host-root resolver, Linux direct-child writer, and Windows adapter regressions are implemented in the candidate source at `722d662e7a81711ba3607d26571f4048a292b3ed`; the candidate's exact test inventory was executed by the protected E4 workflow after promotion.
- Protected verifier PR #44 is CLOSED / MERGED into branch `verification` at merge commit `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`; protected canonical pin is `722d662...`.
- Protected post-merge E4 run #44519577693 is SUCCESS for all three jobs: Unix no-follow/resource bounds; Windows reparse/handle checks; Windows native `NtCreateFile + OBJ_DONT_REPARSE` contract probe.
- Scope limits remain: Linux nested writes fail closed; macOS/unsupported platforms fail closed; Windows native probe is only Windows Server 2025 build 26100; Android runtime evidence is API 35 ARM64 only.
- Separate SEC-05 run #44519577571 has Windows SUCCESS and Linux FAILED at native cross-compiler installation step 5. Exact failure logs are not yet available; do not classify root cause. Other independent E1/stdlib/provenance/health/artifact workflows also failed on the same push.
- Classification: E4 protected verification CONFIRMED for the recorded source and environments; Issue #104 remains OPEN / NOT ACCEPTED until remaining acceptance/ledger requirements are reconciled. PR #113 remains OPEN / DRAFT / UNMERGED.
- Latest PR #113 docs HEAD at this checkpoint: `67d3c14c70fc43c1eae26c7f21fea8e7a4918b07`; CircleCI exact-SHA status PENDING. Do not claim CI success based on the preceding SHA.


## Protected verifier auxiliary workflow classification — 2026-11-19

- E4 protected post-merge run #44519577693: SUCCESS across Unix no-follow/resource bounds, Windows reparse/handle checks, and Windows native `NtCreateFile + OBJ_DONT_REPARSE` probe.
- SEC-05 Linux run #44519577571 / job `135544437757`: failed before tests in native cross-compiler installation. APT update succeeded, but 194 MB of a 277 MB package set was downloaded before the Azure Ubuntu mirror backend `52.252.75.106:80` became unreachable; APT exit 100. Windows SEC-05 passed.
- E1 Windows job `135544437394`, E1 Linux job `135544437616`, and focused stdlib job `135544438572` all fail before executing tests because the configured Cargo target `stdlib_builtins` does not exist in `one-core`. The candidate has no `tests/stdlib_builtins.rs`; the workflow test inventory must be reconciled to actual source tests.
- These are separate verifier setup/harness defects, not E4 failures. Issue #104 remains OPEN / NOT ACCEPTED; no E1 acceptance is inferred.


## Live implementation and acceptance status — 2026-11-19

- Filesystem root-confinement implementation source: `722d662e7a81711ba3607d26571f4048a292b3ed` (candidate source in PR #113 ancestry; protected verifier pin now matches it after PR #44 merge).
- Protected E4: VERIFIED / PASS on run #44519577693 for its stated Unix and Windows checks. Windows native probe is specifically Windows Server 2025 build 26100.
- Linux nested writes and unsupported-platform filesystem operations remain fail-closed. Android API 35 ARM64 runtime evidence does not establish Android filesystem adapter support.
- PR #113 remains OPEN / DRAFT / UNMERGED. Issue #104 remains OPEN / NOT ACCEPTED because protected release gate #44519577312 failed and the full acceptance matrix is not complete.
- PR #45–#48 remediation workflows passed on their respective draft branches; they are not yet protected-branch evidence.

## Verifier remediation integration and latest exact-SHA evidence — 2026-11-19 16:28Z

The user authorized integration of independent verifier PRs #45–#48. They were squash-merged into panglimanetizen01/one-windows-verification protected branch verification; the protected branch now points to b14abf0020472dc45842be30854e0a09317d6cb4. Exact merge commits:

- PR #45 candidate artifact integrity; tested PR head 9a72e04b7e2992d655f8e3fc4e478892e91b7ae5; merge 42feff8bc11d9b17d04ac9cff4abea4e80a7a832.
- PR #46 SEC-05 bounded APT retries; tested PR head 8a880ad00225ed8a78a3fd58adfbc329f0df5b11; merge d539a162aeed9d22988c7ffb0c7a893e8d33f571.
- PR #47 Windows native runtime exact canonical pin/toolchain; tested PR head bd100d10c12c7b0849f01e9481111b0b0c859dd1; merge 4ae84bad84ddacf169b9f25fa4c78581ab2b4bc4.
- PR #48 E1 v0.1 test-target alignment; tested PR head 06bf1d7cbd10adadb74c79d7296f951244ae9839; merge b14abf0020472dc45842be30854e0a09317d6cb4.

Canonical source pin remains 722d662e7a81711ba3607d26571f4048a292b3ed. No pin, fixed-weight ledger, DoD checkbox, package-release dispatch, or source implementation was changed.

### Protected-branch push runs (do not conflate with PR-branch runs)

- Artifact integrity run #44566172841 on merge 42feff8: SUCCESS. Log verified Linux and Windows candidate metadata, including Rust/Cargo 1.100.0.
- SEC-05 run #44566187162 on merge d539a16: IN PROGRESS; Linux job 135708430394 SUCCESS, Windows job 135708430883 IN PROGRESS.
- Windows Native Backend Runtime #44566197814 on merge 4ae84ba: IN PROGRESS, Windows Server 2025 job 135708473420.
- E1 cross-platform #44566212975 on final protected HEAD b14abf0: IN PROGRESS, Ubuntu job 135708526288 and Windows job 135708526901.
- Focused stdlib/database #44566212982 on b14abf0: SUCCESS. Retrieved Linux log confirms source-pinned Rust/Cargo 1.100.0; stdlib 6/6, core pipeline 6/6, conformance 5/5, database unit 1/1, database vertical slice 9/9, full one-core library 148/148, and warnings-denied Clippy PASS.
- E1 Windows #44566212997 on b14abf0: SUCCESS. Retrieved Windows log confirms Rust/Cargo 1.100.0; stdlib 6/6, core pipeline 6/6, conformance 5/5, full one-core library 133/133, and warnings-denied Clippy PASS.

Only terminal runs above may be classified SUCCESS. In-progress scopes remain NOT VERIFIED until jobs finish and logs are inspected. Protected E4 run #44519577693 remains SUCCESS for its original declared scope and was not rerun because E4 source/test inputs did not change.

### CircleCI exact-SHA correction

ONE PR #113 HEAD 64c490e9045aa29506772f74e3697a63a66b9695 has now reached CircleCI ci/circleci: one-linux SUCCESS (workflow b16121d3-51f0-4297-bb05-16e15b0434ea, job 148c47ea-a1ac-4574-8a61-1980d6c5a55a, status updated 2026-11-19T16:28:21Z). This is exact-SHA status evidence; CircleCI provider step logs were not retrieved in this session. Any subsequent docs commit creates a new PR head and requires its own CircleCI result; this SUCCESS must not transfer.

### Release/acceptance boundary

The prior fixed-weight gate #44519577312 remains FAILURE with its retrieved log: 92/100 implementation, 0/100 verified, 18 ledger rows checked, 10 unchecked DoD items, exact-SHA wait timeout, missing protected workflow evidence and no protected ONE Package Provenance Release. Newly merged fixes do not make that historical gate green. Wait for protected runs above to become terminal, reconcile their exact-source evidence, then consider a new gate evaluation only when relevant prerequisites changed. Do not change weights or mark DoD items without their own evidence.

Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes and unsupported platform adapters remain fail-closed. No universal filesystem sandbox or full release acceptance is claimed. ONE main remains d35db95153f6ddd79885494c72f5aee16c899751. PR #113 remains OPEN / DRAFT / UNMERGED; no merge to ONE main is authorized.

Verifier documentation follow-up PR #50 is OPEN / DRAFT at head 8c1d4dad927397219be5c8803e214105fa99e30f, targeting protected verification; it records the integration and exact status boundaries.

## Final protected verifier integration result — 2026-11-19 16:33Z

All six protected-branch push workflows resulting from the authorized verifier PR #45–#48 integration are now terminal SUCCESS:

- Artifact integrity #44566172841 on verifier merge 42feff8bc11d9b17d04ac9cff4abea4e80a7a832: SUCCESS; exact job log confirms Linux and Windows artifact metadata with Rust/Cargo 1.100.0.
- SEC-05 #44566187162 on verifier merge d539a162aeed9d22988c7ffb0c7a893e8d33f571: SUCCESS; Linux and Windows jobs passed bounded APT setup, interop tests, full one-core suites (148 Linux / 133 Windows), and fail-closed static guards.
- Windows Native Backend Runtime #44566197814 on verifier merge 4ae84bad84ddacf169b9f25fa4c78581ab2b4bc4: SUCCESS; Windows Server 2025 checked out canonical ONE source 722d662, Rust/Cargo 1.100.0, built PE32+ AMD64, executed with exit 0 and output Budi, and wrote exact-SHA manifest.
- E1 cross-platform #44566212975 on verifier protected HEAD b14abf0020472dc45842be30854e0a09317d6cb4: SUCCESS; Ubuntu and Windows both terminal SUCCESS. Ubuntu: stdlib 6/6, core pipeline 6/6, conformance 5/5, one-core 148/148, workspace tests, Clippy warnings denied, repository contract checker. Windows: stdlib 6/6, core pipeline 6/6, conformance 5/5, one-core 133/133, workspace tests, Clippy warnings denied, repository contract checker.
- Focused stdlib/database #44566212982 on b14abf0: SUCCESS; Linux stdlib 6/6, core pipeline 6/6, conformance 5/5, database unit 1/1, database vertical slice 9/9, one-core 148/148, Clippy PASS.
- E1 Windows #44566212997 on b14abf0: SUCCESS; Windows stdlib 6/6, core pipeline 6/6, conformance 5/5, one-core 133/133, Clippy PASS.

The protected verification branch remains at b14abf0020472dc45842be30854e0a09317d6cb4. Canonical source pin remains 722d662e7a81711ba3607d26571f4048a292b3ed. No ledger weight, verified-point score, DoD checkbox, or protected release state was changed. Protected E4 #44519577693 remains SUCCESS for its original declared scope and was not rerun.

The fixed-weight gate #44519577312 remains historical FAILURE: 92/100 implementation, 0/100 verified, 18 rows checked, 10 unchecked DoD items, exact-SHA wait timeout, and missing protected source-bound Package Provenance Release. Do not treat the six remediation workflow successes as overall release acceptance. A new gate evaluation is deferred until the evidence index and all row-level/DoD requirements are legitimately reconciled. Do not change fixed weights or dispatch a release to force PASS.

Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes and unsupported platform filesystem adapters remain fail-closed. No universal filesystem sandbox, complete E1–E6 acceptance, or overall release is claimed.

ONE CircleCI status on previous PR #113 HEAD d20036c0ad7b00caf6b68e4807102c6ee3b895a4 remains PENDING at 2026-11-19T16:30:42Z (workflow fc7039a6-8cde-464b-8c06-387e291d6d67, job 1a030227-696e-490d-bb16-dd288947976d). No CircleCI CLI/provider action is exposed in this session; the direct CircleCI app URL is inaccessible here. The previous HEAD 64c490e... had SUCCESS, but status does not transfer to later docs commits. This new docs commit will require its own exact-SHA CircleCI result.

Verifier docs follow-up PR #50 is OPEN / DRAFT, targeting verification, with latest docs head 046d356527040f0375d91fabcc9678c683d969e0. ONE PR #113 remains OPEN / DRAFT / UNMERGED; ONE main remains d35db95153f6ddd79885494c72f5aee16c899751.

## CircleCI exact-HEAD terminal result — 2026-11-19 16:39Z

The latest preceding ONE PR #113 documentation HEAD eeafcab579cebb762af34d1daf4830f95cba87e7 has now reached CircleCI ci/circleci: one-linux SUCCESS. GitHub commit status was updated 2026-11-19T16:38:52Z; workflow 0621c480-6054-4fc1-8c82-b9028f105735; job 49178c86-d311-49ac-835a-6fa49985a980. This is exact-SHA status evidence for eeafcab only. CircleCI provider step logs were not separately retrieved, so no detailed step/test-count claim is made from CircleCI. Any later commit must have its own status checked.

The six protected verifier remediation workflows remain terminal SUCCESS on their exact verifier commits and pinned source SHA 722d662. E3 is VERIFIED 8/8 in draft ONE PR #103 ledger source 36bba2e45a629c4fa39677b0ea07a2489d37bc99; total is 92/100 implementation and 8/100 verified; DoD is 4 checked / 9 unchecked / 13. The corresponding verifier draft PR #50 updates LEDGER_SOURCE_COMMIT.txt to the exact ledger commit, but neither documentation PR has been merged to its base branch.

Issue #104 remains OPEN / NOT ACCEPTED. E1, E2, E4, E5 and E6 are not fully verified; protected source-bound Package Provenance Release is still absent. The historical fixed-weight gate #44519577312 remains FAILURE and is not overwritten. No fixed weights, canonical source pin, E4 acceptance pin, ONE main, or protected release state changed.


## Current checkpoint correction — 2026-11-19 16:44Z

- PR #113 exact HEAD `54b92ab9703b9c3450cb33f5eb4f34861aae107b` now has CircleCI `ci/circleci: one-linux` terminal SUCCESS (workflow `f9bd8ac7-a741-4bf6-8536-37af4798f3a1`, job `77951c98-1242-4c47-b45b-90011ecebfd6`). This supersedes the earlier PENDING observation for the same SHA. CircleCI provider logs are not exposed; this is job-level exact-SHA evidence only. No separate CLI config-validation or per-step test-count claim is made.
- Protected verifier branch `verification` remains `b14abf0020472dc45842be30854e0a09317d6cb4`, canonical source pin `722d662e7a81711ba3607d26571f4048a292b3ed`. All 12 required exact-source verifier workflow names plus Android runtime alternative have terminal SUCCESS.
- Draft ONE ledger/DoD PR #103 head `36bba2e45a629c4fa39677b0ea07a2489d37bc99` records E3 VERIFIED 8/8; ledger totals 92/100 implementation and 8/100 verified; DoD 4 checked / 9 unchecked / 13. This is still a draft source.
- Draft verifier PR #50 head `622637766b9fe7b673182e21faf6b2d8ce0a8767` proposes the exact-source evidence index and ledger source pointer to PR #103. It remains OPEN / DRAFT / UNMERGED. Protected `LEDGER_SOURCE_COMMIT.txt` remains `d6c382a245c49a028bc60809f22ca956c7f894cc`; the proposed pointer has not been applied.
- Historical fixed-weight gate #44519577312 remains FAILURE on the old protected ledger source: 92/100 implementation, 0/100 verified, 18 rows checked, 10 DoD unchecked, and no protected source-bound ONE Package Provenance Release. No new gate run has been triggered. Do not alter fixed weights/DoD or merge PR #50 without explicit authorization.
- Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes and unsupported filesystem adapters remain fail-closed. Issue #82 protected Android runtime #44519577666 succeeded (API 35 arm64-v8a, ELF exit 0), but PR #49 metadata runtime job #135708475823 remains IN_PROGRESS. Issue #105 escaped-descendant containment remains unresolved.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`; PR #113 remains OPEN / DRAFT / UNMERGED.


## Exact-SHA platform metadata and host-selection verification — 20 November 2026

- Code-bearing candidate source: `c7f460a4e38be61de635ad5af9f7ff987a941f01`. Net source/test changes from protected E4 pin `722d662e7a81711ba3607d26571f4048a292b3ed` include `crates/one-core/src/adapter.rs`, `crates/one-core/src/interpreter.rs`, and `crates/one-core/tests/filesystem_vertical_slice.rs`.
- Filesystem adapters advertise only the native backend compiled for the host: Linux on Linux, Windows on Windows, and no filesystem backend on Android/macOS/other unsupported targets. `PlatformKind::current_host()` now passes the actual host identity to orchestration.
- E4 run [#44585270928](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44585270928) SUCCESS: Unix no-follow/resource-bound and Windows reparse/handle tests passed; Windows native `NtCreateFile + OBJ_DONT_REPARSE` probe passed on Windows Server 2025 build 26100. The metadata regression `filesystem_adapters_advertise_only_platforms_with_native_backends` was explicitly executed on Linux and Windows (1 passed / 0 failed each).
- Linux distro run [#44585282690](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44585282690) SUCCESS on Ubuntu 24.04 and Debian 12, including workspace tests and the corrected filesystem vertical-slice CLI regression. E1 Windows acceptance [#44585282432](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44585282432) SUCCESS.
- Remaining candidate workflows (aggregate cross-platform, Windows native backend runtime, Android runtime, SEC-05, E1 milestone, artifact integrity) were still running/pending. CircleCI `ci/circleci: one-linux` on exact source `c7f460a...` remained PENDING; last terminal SUCCESS was on parent docs SHA `26a34ef...` and does not transfer. Provider step logs were not available.
- Verifier diagnostic PR #51 is OPEN / DRAFT at `e9dd717f36bda85bb63e716490e692651fa06126`, branch-local canonical pin `c7f460a...`. Protected verifier branch remains `b14abf0020472dc45842be30854e0a09317d6cb4`; protected canonical pin remains `722d662...`; protected ledger source pin remains `d6c382a245c49a028bc60809f22ca956c7f894cc`.
- Issue #82 is CLOSED / COMPLETED for Android API 35 AArch64 runtime evidence only. This does not imply Android filesystem adapter support.
- Issue #104 remains OPEN / NOT ACCEPTED; Linux nested writes and unsupported platform filesystem adapters remain fail-closed. Fixed-weight gate #44519577312 remains historical FAILURE. No main update, merge, protected pin promotion, ledger weight change, or release action occurred.
- This documentation synchronization creates a new PR #113 HEAD. Keep test evidence attributed to code source `c7f460a...` and check the resulting documentation HEAD independently for CI.

## LIVE EXACT-SHA CHECKPOINT — 2026-11-20

This checkpoint supersedes older live-status statements above where they conflict. Historical run outcomes remain preserved.

### Exact source and repository state

- ONE PR #113 remains OPEN / DRAFT / UNMERGED on `security/issue-104-root-confinement-20261118`.
- Exact source HEAD: `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`; it was not modified.
- Independent verifier PR #51 remains OPEN / DRAFT / UNMERGED; current verifier HEAD: `27207b85ed205d2cf12e6fd66b918193a215d8fd`.
- Verifier PR #51 branch-local `CANONICAL_COMMIT.txt` is `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. Protected `verification` branch and its canonical source pin `722d662e7a81711ba3607d26571f4048a292b3ed` remain unchanged.

### Exact-source verification

- CircleCI `ci/circleci: one-linux`: SUCCESS on source `fb5c8bc...`; workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`. This is status-context evidence; provider step logs were not inspected.
- E4 adversarial filesystem run [#44651796986](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796986): SUCCESS on verifier HEAD `27207b85...`, pinned source `fb5c8bc...`. Windows job #136008224371 explicitly passed the seven named E4 Windows filesystem regressions, the atomic-replacement failure-preservation regression, the hard-link regression, and the platform-metadata regression (9 selected tests, 0 failed). Unix job #136008224711 passed the no-follow, mount-boundary, traversal, race, direct-child write, hard-link, platform metadata, policy, and related resource-bound selected tests.
- Linux distro run [#44651796952](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796952): SUCCESS on Ubuntu 24.04 and Debian 12, including contract checks, build, Clippy, workspace tests, and real-network tests.
- Windows native backend runtime [#44651796852](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796852): SUCCESS.
- E1 Windows acceptance [#44651797000](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651797000): SUCCESS.
- Focused standard-library/database [#44651796941](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796941): SUCCESS.
- SEC-05 subprocess timeout/output-limit [#44651796634](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796634): SUCCESS.
- E4 subprocess resource guard [#44651796650](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796650): SUCCESS.
- Artifact integrity [#44651796712](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796712): SUCCESS. Linux artifact #14563276002 SHA-256 `59aa04887ffa5c2cb66d65e4ef7936622c6e0a4f6164ea8ea458457d10b8a940`; Windows artifact #14561664789 SHA-256 `7d3c51123f73c398032ef0c636c018c12ea501340b822b8e3b4edc9777645a3a`. Embedded manifests both match source SHA `fb5c8bc...`; ELF64 x86-64 and PE32+ x86-64 checks passed.
- Android AArch64 build artifact [#44651796635](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796635): SUCCESS. Android API 35 ARM64 runtime [#44651796976](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796976) remained IN_PROGRESS at this checkpoint.
- Cross-platform aggregate [#44651796654](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796654) remained PENDING; E1 Milestone 1 aggregate [#44651796950](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796950) remained IN_PROGRESS. Do not infer completion from individual platform jobs.

### Artifact-integrity historical failure reconciliation

- Earlier artifact-integrity run [#44589012615](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44589012615) FAILED while waiting for a successful cross-platform bundle run for source `fb5c8bc...`; its log did not identify archive corruption.
- Later exact-source run #44651796712 successfully discovered both artifacts, checked GitHub SHA-256 digests, verified embedded source manifests and toolchain metadata, and validated ELF/PE executable formats. Preserve the old failure as historical evidence; the later success is a separate terminal verification, not a deletion or rewrite of that failure.

### Acceptance boundary

- The hard-link test closes a previously missing regression-evidence gap: an in-root file with an outside hard link is rejected for both read and write, and both names retain their original bytes.
- Linux mount-boundary, symlink, traversal, race, and direct-child write controls are exercised by the exact-source Unix E4 run. Linux nested writes remain fail-closed.
- Windows junction/reparse-point and handle-relative checks are exercised by the exact-source Windows E4 run and native backend runtime.
- Filesystem adapter metadata remains Linux-only on Linux and Windows-only on Windows; Android/macOS filesystem adapters remain unavailable. Android native runtime evidence does not imply Android filesystem adapter support.
- Issue #104 remains OPEN / NOT ACCEPTED. Do not promote protected pins, ledger weights, or DoD items; do not merge PR #113 or PR #51; do not change `main` or protected `verification`.

## Live exact-SHA status refresh — 2026-11-20

This is a later GitHub status readback than earlier dated entries in this document. Historical checkpoints remain unchanged.

- ONE PR #113 is OPEN / DRAFT / UNMERGED. Exact HEAD: `5daf3337b4bcc93fa3ee6072f9633915d5fea85b`; base: `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Code-bearing source: `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. The 16 commits from this source to current PR HEAD are documentation-only; no runtime implementation or test paths changed in that range.
- CircleCI `ci/circleci: one-linux` on exact code source `fb5c8bc...`: SUCCESS (workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`). This is GitHub status-context evidence; provider step logs were not inspected.
- CircleCI `ci/circleci: one-linux` on exact PR HEAD `5daf333...`: FAILURE (workflow `5ed6d9e7-976e-4322-b7ba-6deea915e1a9`, job `8d2d756e-9297-4a00-9a7d-a0c9bd734cba`, job #833). Available diagnostic: step #101 CANCELED, `Task information unavailable`, stdout/stderr empty. Root cause UNKNOWN; do not infer a timeout, infrastructure fault, or source/config regression without provider logs.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`. Its separate `CircleCI Pipeline` status is ERROR at pipeline #399 (`https://app.circleci.com/pipelines/gh/panglimanetizen01/one/399`). Provider logs and cause are UNKNOWN; this status must not be conflated with PR #113's `ci/circleci: one-linux` context.
- Verifier PR #51 remains OPEN / DRAFT / UNMERGED at `6c0979e62eb01e4b929c48b4224a3d4f742bc8af`. Its branch-local source pin is `fb5c8bc...`; 12 associated PR-triggered workflows are terminal SUCCESS. Protected `verification` pin remains `722d662e7a81711ba3607d26571f4048a292b3ed` and is unchanged.
- Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes and unavailable Android/macOS filesystem adapters remain fail-closed. Verifier successes do not constitute Issue #104 acceptance.
- Provider-side CircleCI logs are not accessible through the connected session. Collect job #833 logs by direct job UUID and pipeline #399 logs using the local diagnostic script; verify each run's source revision before attribution. Never expose `CIRCLECI_TOKEN`.
- No merge, force-push, `main` update, protected pin promotion, ledger-weight/DoD change, or blind retry is authorized by this status refresh.



## Live exact-SHA refresh — 2026-11-20 (B-001 / B-002 / release-gate reconciliation)

This is the newest readback. Preserve all earlier snapshots as history; this block supersedes only conflicting live-state claims.

### Repository and pull requests
- ONE PR #113: OPEN / DRAFT / UNMERGED; exact HEAD `7296c573cbd7eab4a1d7ff08bced2b397a5c555f`; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. Compare from code-bearing source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` shows 48 commits and documentation-only changes across 18 existing docs paths; no runtime source, test, or CI workflow path changed.
- PR #115 is MERGED at `3adace165890bb69de346880e3c612c5cd6c2ff8`.
- Verifier PR #51: OPEN / DRAFT / UNMERGED; HEAD `6c0979e62eb01e4b929c48b4224a3d4f742bc8af`; branch-local `CANONICAL_COMMIT.txt` pins ONE source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. Compare from verifier HEAD `226360d00bc26f858128d3127591fce13ecd52d9` is three documentation-only commits (CANDIDATE_VERIFICATION.md, MIRROR_STATUS.md, README.md, Android evidence note); no workflow/source implementation change.
- Protected verifier branch remains `verification` at `b14abf0020472dc45842be30854e0a09317d6cb4`; protected canonical source pin remains `722d662e7a81711ba3607d26571f4048a292b3ed`. Protected `LEDGER_SOURCE_COMMIT.txt` remains `d6c382a245c49a028bc60809f22ca956c7f894cc`. No protected pin was changed.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`; no main mutation.

### B-001 — CircleCI pre-execution termination
- Historical pipeline #1201 on `509b7b197115adb1ba80d730342e9f6fbf981645` remains a confirmed pre-execution termination; provider root cause UNKNOWN.
- Code-bearing ONE source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` has GitHub combined status `ci/circleci: one-linux = SUCCESS`, workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`. Provider step logs were not inspected; claim only exact-SHA status success.
- Current PR #113 HEAD `7296c573...` has no combined-status entries. Its predecessor docs SHA `a0d94bc9a26cf9effd4b202f29af50f81c8e4ee5` had FAILURE at workflow `916983e9-dbde-4db2-a967-438a666ebb19`, job `a130c5d9-c078-4c18-8ae5-571470f46d60`; available step #101 was CANCELED with unavailable task information and empty logs. Do not call it a test timeout or source failure.
- Exact main SHA `d35db951...` separately shows context `CircleCI Pipeline = ERROR`, target pipeline #1202. Pipeline-to-workflow/job/revision attribution and provider logs remain unavailable. Do not conflate this with the PR context or the source-SHA SUCCESS.
- B-001 is operationally resolved only for the tested code-bearing source/workflow state; provider root cause remains UNKNOWN and the latest docs HEAD has no new CircleCI result.

### B-002 — Issue #104 exact-source verification and remaining product gap
- Twelve PR-triggered verifier workflows associated with verifier HEAD `6c0979e62eb01e4b929c48b4224a3d4f742bc8af` are terminal SUCCESS, all using branch-local source pin `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`:
  - E4 adversarial #44658361321: Unix job #136030099861; Windows reparse/handle job #136030099812; Windows native probe #136030099584 — all SUCCESS.
  - Linux distro #44658361778: Ubuntu 24.04 job #136030102055; Debian 12 job #136030102321 — SUCCESS.
  - Windows native runtime #44658361296: job #136030100371 — SUCCESS.
  - E1 Windows #44658361571: job #136030101715 — SUCCESS.
  - E1 Milestone 1 #44658361518: Windows job #136030101394 and Ubuntu job #136030101588 — SUCCESS.
  - SEC-05 #44658361512: Linux job #136030100674 and Windows job #136030100987 — SUCCESS.
  - Cross-platform #44658361514: Windows job #136030101518 and Ubuntu job #136030101784 — SUCCESS.
  - Artifact integrity #44658361320: job #136030099580 — SUCCESS.
  - Resource guard #44658361457: job #136030100840 — SUCCESS.
  - Android AArch64 build #44658361365: job #136030100171 — SUCCESS.
  - Focused stdlib/database #44658361557: job #136030101561 — SUCCESS.
  - Android API 35 ARM64 runtime #44658361350: build job #136030099698 and runtime job #136030558684 — SUCCESS.
- E4 job logs were inspected, not just run summaries. Linux executed 31 named tests (3 read, 5 write, 1 walk, race, mount-boundary, hard-link, database, 11 governance, 5 network, policy-denial, platform metadata), 0 failed. Windows executed 9 named regressions, 0 failed. The Linux write suite explicitly includes `descriptor_relative_write_fails_closed_for_nested_parent`; this is proof of fail-closed behavior, not positive nested-write support.
- Android API 35 ARM64 runtime is native-backend execution evidence only; it does not establish an Android filesystem adapter.
- Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Do not promote full filesystem support from the scoped E4 pass.

### Release gate / B-003 boundary
- The protected ledger document pin remains `d6c382a245c49a028bc60809f22ca956c7f894cc`; the protected ledger gate #44519577312 remains historical FAILURE (92/100 provisional implementation, 0/100 verified at that gate's ledger source, 18 rows checked, 10 DoD items unchecked, no protected exact-source ONE Package Provenance Release).
- ONE PR #103 remains OPEN / DRAFT / UNMERGED at HEAD `5f616ae08e303081666a5e0d6a4b8b63b9d69c16`. Its branch does not replace the protected ledger source pin. Keep fixed weights, verified points, DoD checkboxes, canonical pins, and release criteria unchanged. No new gate evaluation is justified until relevant acceptance/evidence inputs change.
- No source code, CI configuration, main branch, protected verifier branch/pin, ledger weights, DoD checkbox, merge, or release was changed in this refresh.

### Next executable work
1. Continue B-002 from the remaining Linux nested-write acceptance gap. Keep the current fail-closed behavior; do not weaken it. Any implementation proposal must prove parent-directory move/race confinement with real Linux and Windows runners before changing security-sensitive code.
2. Keep unsupported Android/macOS filesystem adapters explicitly unavailable unless native backends and their own adversarial matrices are implemented and verified.
3. Continue B-003 only after the exact release-candidate SHA, complete E1–E6 evidence, source-bound package provenance, and ledger/DoD reconciliation are genuinely available. Do not rerun the unchanged historical gate or promote scores.


## B-003 exact-head refresh — PR #103 ledger draft (2026-11-20)

- ONE PR #103 remains OPEN / DRAFT / UNMERGED at HEAD `5f616ae08e303081666a5e0d6a4b8b63b9d69c16`. It is 7 commits ahead of protected ledger source pin `d6c382a245c49a028bc60809f22ca956c7f894cc`, and 2 commits ahead of prior CI-verified docs commit `e053b5ffae46a7e1d696bea5eb5a13999c722ee0`. The 2-commit delta changes only `docs/verification/one-completion-ledger.md` and `docs/verification/DEFINITION_OF_DONE.md`.
- GitHub Actions run #44573696031 is COMPLETED / SUCCESS on exact PR #103 HEAD `5f616ae...`: Ubuntu job #135735331182 and Windows Server 2025 job #135735332010 both terminal SUCCESS. This is documentation-branch CI evidence, not a release gate or E1–E6 acceptance promotion.
- CircleCI combined status for exact PR #103 HEAD is `CircleCI Pipeline = ERROR`, target pipeline #1168. Provider logs and pipeline-to-workflow/job/revision mapping are not available; classify the CircleCI cause as UNKNOWN, not a source/test failure.
- The unmerged PR #103 draft ledger records 92/100 IMPLEMENTED, 8/100 VERIFIED (E3 8/8), and DoD 4 checked / 9 unchecked / 13. These are draft values only. The protected verifier ledger source pin remains `d6c382a...`; the protected snapshot at that pin remains 92/100 implementation, 0/100 verified, DoD 3 checked / 10 unchecked / 13.
- Historical protected fixed-weight gate #44519577312 remains FAILURE. Do not treat PR #103's GHA success as a protected ledger-gate pass, and do not merge PR #103, move the ledger source pin, change weights, or check DoD boxes without explicit authorization and full acceptance evidence.
- No product source, CI configuration, main branch, protected verifier branch/pin, ledger weights, or acceptance criteria changed in this refresh.


## B-001 classification correction — 2026-11-20 (pipeline mapping not verified)

This correction supersedes any earlier wording in this document that called B-001 fully or operationally resolved. Do not delete the historical notes.

- Exact ONE source SHA `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` has GitHub combined status `ci/circleci: one-linux = success`. The status target URL identifies workflow `3a9c0613-a1bc-43ae-b2d6-319841061048` and job `c4faf162-80e9-4f17-a83a-a72634bb592f`.
- The CircleCI pipeline UUID/number for that job, pipeline `vcs.revision`, and independent proof that the job belongs to that pipeline/workflow were not retrieved. The GitHub status is exact-SHA context evidence, but it does not complete the required job → workflow → pipeline → `vcs.revision` chain.
- The historical #1201 pre-execution termination remains documented; its provider root cause remains UNKNOWN. Current PR #113 docs HEAD `abfae2b95c179650b362426a001a47e164f7ac20` has no CircleCI status context. Main pipeline #1202 and PR #103 pipeline #1168 show separate ERROR contexts, but their `vcs.revision`, job mapping, and provider logs are unavailable.
- CircleCI UI pipeline pages #1202, #1168, and #1199 were not accessible through the available web session. No authenticated CircleCI API/CLI provider-log access was available. Do not infer timeout, test failure, source regression, or root cause.
- **B-001 status: PARTIAL / NOT CLOSED.** The exact-source GitHub status is SUCCESS, but pipeline mapping and provider diagnostics are still required before claiming full operational resolution. Do not trigger blind retries or change CI configuration to hide the gap.


## Exact-SHA Windows parent-rename regression — 2026-11-20

- New code-bearing test SHA: `c4f3800bc12b70a4d59d72fbc5a80029cb053cea`; the only source change relative to the prior candidate is a Windows-only test in `crates/one-core/src/filesystem.rs`, not production behavior.
- Verifier PR #51 exact HEAD: `7c3ded0d811fe71bd14034880b24b0f8e396349e`; branch-local canonical source pin: `c4f3800...`. Protected verifier branch and canonical pin remain unchanged.
- New test: `filesystem::windows_native::tests::retained_parent_handle_blocks_directory_rename`. It opens a nested parent with the same `FILE_SHARE_READ | FILE_SHARE_WRITE` share flags used by `write_relative` (no `FILE_SHARE_DELETE`), proves rename fails while the handle is retained, then proves rename succeeds after handle close.
- E4 push run #44760172674 is terminal SUCCESS on exact source `c4f3800...` / verifier `7c3ded0...`: Unix job #136370280796 SUCCESS; Windows reparse/handle job #136370280317 SUCCESS; Windows native probe job #136370280733 SUCCESS. Windows log shows the new test executed exactly once and passed (1 passed, 0 failed).
- Duplicate E4 pull-request run #44760180858 is also terminal SUCCESS on the same exact source/verifier pair; Unix job #136370312121, Windows reparse/handle job #136370312012, and native probe job #136370311602 all SUCCESS. Count the duplicate event as confirmation, not a second acceptance point.
- Other exact-verifier-SHA PR workflows: SUCCESS — E4 resource guard #44760181208, focused stdlib/database #44760180912, Android AArch64 artifact build #44760180867, Linux distro #44760180826, E1 Windows #44760181124. IN_PROGRESS — Windows native runtime #44760181204, candidate artifact integrity #44760180923, SEC-05 #44760180951, E1 Milestone 1 #44760180919 (Ubuntu job #136370312623 SUCCESS; Windows job #136370313083 IN_PROGRESS), cross-platform #44760181947, Android API35 runtime #44760180953. Do not call the complete verifier matrix green yet.
- ONE source PR #113 GitHub Actions run #44760138732 is IN_PROGRESS on exact source `c4f3800...`: Ubuntu job #136370164396 SUCCESS; Windows job #136370165192 IN_PROGRESS. CircleCI `ci/circleci: one-linux` is PENDING on this exact SHA, workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Pipeline UUID/number, `vcs.revision`, and job→workflow→pipeline membership remain unverified.
- B-001 remains PARTIAL / NOT CLOSED pending CircleCI pipeline mapping/provider logs. Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes fail-closed; Android/macOS filesystem adapters unavailable. This test verifies one Windows parent-handle exclusion primitive, not complete E4/Issue #104 acceptance.
- No production runtime code, CI configuration, main, protected verifier branch/pin, ledger/DoD, acceptance criteria, merge, or release changed.


## Exact-SHA verification refresh — 2026-11-20 15:14Z

### Candidate identity
- Code-bearing source: `c4f3800bc12b70a4d59d72fbc5a80029cb053cea` (only source delta from `fb5c8bc...` is the Windows-only parent-handle rename regression test).
- Verifier PR #51 HEAD: `7c3ded0d811fe71bd14034880b24b0f8e396349e`; branch-local source pin: `c4f3800...`.
- ONE PR #113 current docs-only HEAD before this sync: `043390cf0673247524ffb44c444cbbe4745be91f`. This sync changes existing docs only and uses `[skip ci]`; no CI result transfers to the resulting docs HEAD.

### Terminal results on exact source/verifier pair
- ONE GitHub Actions run #44760138732 on source `c4f3800...`: Ubuntu job #136370164396 SUCCESS; Windows Server 2025 job #136370165192 SUCCESS. Windows log explicitly ran and passed `interop::tests::subprocess_execution_is_bounded_by_a_deadline`, `subprocess_stdout_capture_has_a_hard_limit`, `subprocess_stderr_capture_has_a_hard_limit`, and `windows_job_object_terminates_descendants_when_parent_exits`.
- Verifier PR matrix on verifier SHA `7c3ded0...` / source pin `c4f3800...`: 11 of 12 named workflows are terminal SUCCESS:
  - E4 adversarial #44760180858: Unix #136370312121, Windows #136370312012, native probe #136370311602 — SUCCESS. New `retained_parent_handle_blocks_directory_rename` test executed once and passed.
  - E4 subprocess resource guard #44760181208 / job #136370313217 — SUCCESS.
  - Focused stdlib/database #44760180912 / job #136370312551 — SUCCESS.
  - Android AArch64 artifact build #44760180867 / job #136370311618 — SUCCESS.
  - Linux distro #44760180826: Debian #136370312102 and Ubuntu #136370312874 — SUCCESS.
  - Windows native runtime #44760181204 / job #136370312887 — SUCCESS; Windows Server 2025, Rust 1.100.0, runtime exit 0, output `Budi`.
  - SEC-05 #44760180951: Linux #136370311990 and Windows #136370312357 — SUCCESS. Source PR Windows GHA also proves the dynamic deadline/stdout/stderr/Job Object tests executed and passed.
  - E1 Windows #44760181124 / job #136370313677 — SUCCESS.
  - E1 Milestone 1 #44760180919: Ubuntu #136370312623 and Windows #136370313083 — SUCCESS.
  - Cross-platform #44760181947: Windows #136370329101 and Ubuntu #136370329521 — SUCCESS.
  - Candidate artifact integrity #44760180923 / job #136370311819 — SUCCESS.
- E4 push event #44760172674 is also SUCCESS on the same exact source/verifier pair (Unix #136370280796, Windows #136370280317, native probe #136370280733); it is a duplicate trigger and is not a second acceptance point.

### Artifact provenance and remaining runtime
- Artifact integrity selected cross-platform run #44760181947 at verifier SHA `7c3ded0...` for exact source `c4f3800...`.
- Linux artifact #14640868829, SHA-256 `7e8e443e2ae7c736f18243043f355840801afebdddca46a5efe41c159bac5732`; metadata target `x86_64-unknown-linux-gnu`, Rust/Cargo 1.100.0, ONE CLI 0.1.0, ELF64 little-endian EM_X86_64 PASS.
- Windows artifact #14641018247, SHA-256 `f68c7325ff39ae33b2cfadf0b896896c0ba4c3eeb581b033f8e1a75974900846`; metadata target `x86_64-pc-windows-msvc`, Rust/Cargo 1.100.0, ONE CLI 0.1.0, PE32+ x86-64 PASS.
- The only verifier workflow not terminal is Android API 35 ARM64 runtime #44760180953. ELF build job #136370311867 is SUCCESS; runtime job #136371143664 is still executing the “Boot Android API 35 AArch64 on ARM64 host” step. Workflow timeout is 150 minutes; no timeout/failure is claimed.
- Do not claim Android filesystem adapter support from this native-backend runtime. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

### CircleCI and release gate
- CircleCI `ci/circleci: one-linux` on source `c4f3800...` remains PENDING; workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Pipeline ID/`vcs.revision` mapping and provider logs remain unavailable. B-001 remains PARTIAL / NOT CLOSED.
- Protected verifier branch/pin, protected ledger pin `d6c382a...`, fixed weights, DoD, main, acceptance criteria, merge, and release state remain unchanged. Historical fixed-weight gate #44519577312 remains FAILURE.

## Implementation-first continuation — 2026-11-20

Owner direction: complete the actual v0.1 product implementation before spending another cycle on broad test/verification runs. Add or update necessary regression tests as part of implementation, but defer the full matrix and repetitive audits until the implementation scope is complete. Do not claim a feature complete merely because tests or docs are green.

### Current Issue #104 implementation delta

- PR #113 exact HEAD at this checkpoint: `7cbcb1bf575defe03a2d9991dc5094569de95f03` (OPEN / DRAFT / UNMERGED; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`).
- CLI filesystem authority now derives only from the host process current working directory; source-file location and program-controlled `--target` no longer select or widen the root.
- Linux root acquisition now opens the supplied directory with `O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC` and checks canonical-path device/inode identity against the retained handle; a mismatch fails closed.
- CLI regression fixture updated to place the source file outside the working directory while targeting an in-root file.
- Implementation commits: `8b1d13e02c181fc06b97fb7f147e1ccd013d3aa1`, `5e39430a341629eb9fda11ee4e11b0f6f40487ab`, `7cbcb1bf575defe03a2d9991dc5094569de95f03`.
- These changes are NOT BUILT OR TESTED yet. Existing prior-SHA results do not transfer. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

### B-001 operational status

- Exact SHA `979973cd6fb6f76802e13dbbbe21530d68b4cf53` has a terminal successful CircleCI workflow/job mapping: pipeline #1233 revision matches the SHA; workflow `934cd811-3a44-4737-bb62-943e677d979b` SUCCESS; job #865 `4b9419f9-9b1a-4823-902e-dcd872073e39` SUCCESS, started `2026-11-20T15:49:44.969Z`.
- The diagnostic classifier correction is locally 20/20 on that source. Historical pipeline #1201 cause remains UNKNOWN, but no longer blocks the current implementation queue. Do not rerun it without a new relevant failure.

### Next implementation priority

Continue Issue #104 production work and reconcile the normative contract against the current source. Preserve fail-closed behavior where a race-resistant primitive is not available; do not weaken the boundary to make nested Linux writes appear supported. Then continue remaining v0.1 product implementation gaps. Only after implementation is complete should the integrated final platform/security/release matrix run.

## Linux root no-follow regression — 2026-11-20

- Added `filesystem::tests::authority_rejects_symlink_root_without_following_final_component` in commit `ab72ec5d65a5e237cdc1b10e40ea209149f42e74` to prove that a symlink cannot be supplied as the final trusted-root component.
- This test has NOT been run. Latest code-bearing candidate is `ab72ec5d65a5e237cdc1b10e40ea209149f42e74`; current PR #113 HEAD is documentation-only `b8f5569333723cf916ad918735934b9b5d774095` (OPEN / DRAFT / UNMERGED). No exact-SHA test result is claimed.
- The current implementation delta comprises CLI cwd-only root selection, Linux no-follow root-handle acquisition with identity check, and the two new CLI/filesystem regression fixtures. Linux nested writes remain fail-closed; Android/macOS adapters remain unavailable; Issue #104 remains OPEN / NOT ACCEPTED.

## Windows root no-reparse implementation — 2026-11-20

- Windows root acquisition no longer canonicalizes through the final component before opening it. It canonicalizes only the parent, preserves the final name, opens it with the existing `OBJ_DONT_REPARSE` / `FILE_OPEN_REPARSE_POINT` native path, then compares volume serial and file index against the path identity.
- Added `filesystem::windows_root_tests::authority_rejects_reparse_root_without_following_final_component` in commit `8dd92aebc033a94abb14a8d7ec498f6933d4ba8e`; it creates a directory junction and requires root acquisition to fail closed.
- Windows implementation commit: `b208a1a5bdf4a9ffb00a7c17aa4a401ed4a237ce`. This and the Linux/CLI root changes are NOT BUILT OR TESTED. Latest code-bearing candidate: `8dd92aebc033a94abb14a8d7ec498f6933d4ba8e`.
- Preserve Issue #104 OPEN / NOT ACCEPTED. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable.

## CLI root-widening negative regression — 2026-11-20

- Added `run_command_does_not_allow_source_location_to_widen_filesystem_root` in test commit `ea376531e99f37b874d3d3a8edd4e8b4d656095f`. It places both the `.one` source and target under a parent outside the host working directory and asserts that the filesystem read is denied.
- Together with `run_command_uses_working_directory_as_filesystem_root_not_source_location`, the CLI regressions cover both positive in-root access from an external source file and denial of source-parent widening.
- These tests have NOT been run. Latest candidate source/test SHA: `ea376531e99f37b874d3d3a8edd4e8b4d656095f`; prior-SHA results do not transfer.

## Preserve host CWD path through root open — 2026-11-20

- Follow-up review found that the CLI was canonicalizing `cwd` before passing it to `FilesystemAuthority::open`, which could bypass the new final-component no-follow check on hosts where `current_dir()` preserves a reparse/junction path.
- Fixed in `crates/one-cli/src/main.rs` commit `56bd643571f8a371c5d3cabb5175113672d9198f`: pass the host `cwd` path directly to `FilesystemAuthority::open`; platform authority code performs safe parent normalization and final-component no-follow/identity validation itself.
- This source change is NOT BUILT OR TESTED. Latest source/test candidate is `56bd643571f8a371c5d3cabb5175113672d9198f`. Issue #104 remains OPEN / NOT ACCEPTED.

## Linux atomic-write adversarial coverage added — 2026-11-20

- Added `descriptor_relative_write_race_never_modifies_outside_symlink_target` in `crates/one-core/src/adapter.rs` (commits `7ca8e84230383cc84684539ee87dc2d27e72d4fa` and `ab772225832c8b3cacc1772e9b384b974c1a2884`). It repeatedly races leaf symlink/regular-entry replacement against atomic writes and asserts the outside sentinel remains unchanged and no `.one-write-*` temp entry leaks.
- Refactored the Linux atomic writer with internal deterministic failure checkpoints (`TempCreated`, `Written`, `Synced`, `BeforeRename`) in `c7235ab925f2aff7d248b54f9ea74686236b4668`; added `authority_atomic_write_failures_cleanup_temp_and_preserve_target` in `8b95e3bb8b739083eb4e6f52b6aee81ed4cc088a` to assert cleanup and target preservation at every stage.
- These tests have NOT been run. Latest source/test candidate: `8b95e3bb8b739083eb4e6f52b6aee81ed4cc088a`; current PR #113 HEAD at readback: `8b95e3bb8b739083eb4e6f52b6aee81ed4cc088a`. No prior exact-SHA evidence transfers.
- Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

## Android host identity — 2026-11-20

- `crates/one-core/src/platform.rs` now exposes `Platform::Android` and returns it for `target_os = "android"` instead of collapsing Android into `Platform::Other` (`f8e46856803c9f45f54402e71d7e19853bdf46bc`).
- This is host identity only; it does NOT enable Android filesystem operations. The filesystem adapter remains fail-closed on Android until a safe resolver is available that avoids process-killing seccomp behavior and preserves root/symlink/mount/race guarantees.
- The platform unit test was updated but has NOT been run.

## Explicit desktop/mobile host identity — 2026-11-20

- `Platform` now identifies Windows, Linux, Android, and macOS explicitly rather than collapsing Android/macOS into `Other`; the platform unit test covers all four target cfgs (`5846eb70f2a462331873cc0023d1f1ab1106f86a`).
- This is platform identification only. It does not imply filesystem adapter support on Android/macOS; those adapters remain fail-closed pending platform-safe resolver implementations.
- The platform test has NOT been run.

## Android filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `7dd7b0049a7f5d71ac8a3d59570c2975fed835ec` (NOT BUILT OR TESTED).
- Android now has a native `FilesystemAuthority` root open and a handle-relative direct-child read/atomic-replace backend. Root and leaf are opened without following final symlinks; `statx(STATX_MNT_ID)` is required to reject mount crossings; reads reject non-regular and hard-linked files; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The backend deliberately avoids `openat2` because Android seccomp may terminate the process with `SIGSYS` for that syscall. It requires kernel `statx` mount-ID support; if unavailable it fails closed.
- Scope is intentionally limited to direct children of the trusted root. Nested Android paths return `UnsupportedPlatform`; no claim of full Android filesystem support or Issue #104 acceptance is made.
- Three Android-targeted regression tests cover direct-child read/write, leaf-symlink and nested-path denial, and final-component root-symlink rejection. They have NOT been run. Actual Termux/device execution is still required.
- Linux nested writes remain fail-closed; macOS filesystem adapter remains unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

## macOS filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `4be1b8fd579092d8195c3fb775b7c765f6221814` (NOT BUILT OR TESTED).
- macOS now has a native retained-root open and direct-child read/atomic-replace backend. Root and leaf operations use directory/file descriptors with `O_NOFOLLOW`; file and root device identity are compared; hard-linked and non-regular targets are rejected; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The macOS adapter is deliberately limited to direct children of the trusted root. Nested paths return `UnsupportedPlatform`. This implementation has no target-host execution evidence yet; its mount/device and race behavior still require platform-specific review.
- Three macOS-only regression tests cover direct-child read/write, symlink/nested-path denial, and final-component root-symlink rejection. They have NOT been run.
- Android direct-child backend remains statx-mount-ID dependent; Linux nested writes remain fail-closed. Issue #104 remains OPEN / NOT ACCEPTED.

## Issue #105 — Linux process-group containment implementation candidate — 2026-11-20

- Latest code-bearing candidate: `0a9685bded788ae46596227bd61457d10db1179c` (NOT BUILT OR TESTED).
- Linux subprocess startup now installs an inherited seccomp filter in the child before `exec`. It denies `setsid`, `setpgid`, `unshare`, `setns`, clone namespace flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` so compatible runtimes can fall back to `clone`. The existing dedicated process group is therefore intended to remain non-escapable for descendants; group cleanup still runs on normal return, timeout, and output overflow.
- If the kernel refuses `PR_SET_NO_NEW_PRIVS` or filter installation, process spawn fails closed. Windows Job Object containment remains unchanged.
- Process execution is now explicitly unavailable on Android and macOS until equivalent non-escapable containment exists; `ProcessAdapter` advertises only Linux and Windows.
- Replaced the old escaped-session reader-only probes with three Linux adversarial regressions for attempted `setsid` on normal completion, timeout, and output overflow. These tests have NOT been run.
- This is an implementation candidate, not Issue #105 acceptance. Exact-SHA build/execution and adversarial evidence are still required. Preserve bounded reader shutdown and all existing process limits.

## Android process containment extension — 2026-11-20

- Code candidate `58ae7831c7d17e60e53fa6a933dc2f170196c585` enables the same inherited Linux seccomp process-group filter on Android as on Linux and advertises `ProcessAdapter` on Android. It denies session/group changes and namespace escapes; filter setup failure is fail-closed.
- macOS process execution remains unavailable until equivalent containment exists. Windows Job Object behavior is unchanged.
- This Android extension is NOT BUILT OR TESTED. Android seccomp policy compatibility and the three adversarial process tests still require execution on an actual Android/Termux target. Issue #105 remains OPEN / NOT ACCEPTED.

## Android process containment regression coverage — 2026-11-20

- Latest code-bearing candidate: `418d9642778d975440b81df2bdb5e1bf9aa1d8a5` (NOT BUILT/TESTED).
- The inherited seccomp pre-exec filter is now installed on both Linux and Android. ProcessAdapter advertises Linux, Android, and Windows; macOS remains fail-closed. Android uses the same `setsid`/`setpgid`/namespace restrictions and fails closed if the child cannot install the filter.
- The three adversarial `setsid` regression tests and bounded-reader test are now target-gated for Linux and Android, so they can run in Termux as well as Linux CI. No Android/Termux execution evidence is claimed.
- Issue #105 remains OPEN / NOT ACCEPTED pending exact-SHA build and adversarial execution on Linux and a real Android/Termux target, plus Windows Job Object evidence reconciliation.

## IMPLEMENTATION CHECKPOINT — 2026-11-21 (supersedes older platform-status bullets)

Code-bearing source/test candidate: `4c46a41fe45baa7627557a2a25b9c71812840479`. PR #113 was OPEN / DRAFT / UNMERGED at this exact SHA before documentation synchronization. This candidate has NOT been built or tested; no previous exact-SHA result transfers.

### Issue #104 — filesystem implementation
- Linux: retained root handle, `openat2` no-follow/beneath/no-cross-device reads; atomic direct-child replacement; nested writes use the detached-mount + `mount_setattr(MOUNT_ATTR_NOSYMFOLLOW)` + `openat2` path and fail closed when the kernel/security profile cannot provide those primitives.
- Windows: existing native handle-relative/reparse-point backend and retained-parent rename protection remain unchanged.
- Android: direct-child reads and atomic replacement use retained root `dirfd`, `O_NOFOLLOW`, mandatory `statx(STATX_MNT_ID)` equality, regular-file/single-link checks, exclusive temporary sibling, file sync, root-directory sync, and `renameat`. Nested paths remain fail-closed.
- macOS: direct-child reads and atomic replacement use retained root `dirfd`, `O_NOFOLLOW`, regular-file/single-link checks, `fstatfs` filesystem-ID plus mount-point comparison, exclusive temporary sibling, file sync, root-directory sync, and `renameat`. Nested paths remain fail-closed.
- Android/macOS hard-link side-effect regressions were added. Existing Android/macOS direct-child, symlink, and root-symlink regressions remain in source. These tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED until the fixed acceptance criteria have exact-SHA Unix and Windows execution evidence, including race/side-effect proof.

### Issue #105 — process containment implementation
- Linux and Android subprocess startup install an inherited seccomp filter before `exec`; it denies `setsid`, `setpgid`, `unshare`, `setns`, namespace-bearing `clone` flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` for compatible fallback. Filter setup failure is fail-closed.
- Windows Job Object containment is unchanged. macOS process execution remains fail-closed until equivalent non-escapable containment exists.
- Three adversarial `setsid` regressions cover normal completion, timeout, and output overflow on Linux/Android; they are NOT RUN. Issue #105 remains OPEN / NOT ACCEPTED pending Linux + real Android/Termux execution and reconciliation of Windows Job Object evidence.

### Execution priority
- Continue production implementation against the existing v0.1 contract and release blockers. Add regression coverage with each code change, but do not launch broad matrix/audit cycles until the implementation scope is complete.
- Preserve B-001's prior exact-SHA success; do not repeat its investigation without a new relevant provider failure.
- Preserve `main`, protected verifier pins, fixed ledger weights, DoD, acceptance criteria, and release authorization boundaries.


## Implementation continuation refresh — 2026-11-21

Latest code-bearing candidate and PR #113 HEAD at this checkpoint: `fd1ccc5ca56f48a306ac8f8c551189f46b8fe44c` (OPEN / DRAFT / UNMERGED; NOT BUILT/TESTED). This supersedes the previous 2026-11-21 candidate pointer.

Issue #105 regression coverage now uses ignored helper tests in the same Rust test binary. The helper forks a descendant and calls `setsid(2)` directly, avoiding dependence on an external `setsid` utility on Android/Termux. The parent tests cover normal completion, timeout, and output overflow, record any escaped PID, and clean up the escaped group before reporting failure. The tests are NOT RUN.

Issue #104 implementation changes from the previous checkpoint remain: Linux guarded detached-mount nested writer; Android direct-child `statx(STATX_MNT_ID)` backend; macOS direct-child `fstatfs` filesystem-ID/mount-point backend; exclusive temp replacement with file and directory sync; hard-link regressions. Windows native backend remains unchanged. Nested Android/macOS operations fail closed.

Issues #104 and #105 remain OPEN / NOT ACCEPTED. No exact-SHA build/test or broad verification is claimed. Continue implementation-first and preserve all release/security gates.

## Process-group cleanup hardening — 2026-11-21

Latest code-bearing source/test candidate: `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` (NOT BUILT/TESTED). Linux/Android process supervision now observes direct-child exit using `waitid(P_PID, WEXITED | WNOHANG | WNOWAIT)` rather than reaping via `try_wait()` before process-group cleanup. The child remains a zombie (its PID/PGID cannot be recycled) while the wrapper sends SIGKILL to the invocation process group; only then does `Child::wait()` reap it. The redundant post-reap group kill was removed. All timeout, output-overflow, wait-error and normal-completion paths terminate the group before reaping.

Process escape regressions use in-binary ignored helpers with direct `fork` + `setsid(2)` syscalls, record any escaped PID and clean up before failing. Three paths: normal completion, timeout, output overflow. NOT RUN. Issue #105 remains OPEN / NOT ACCEPTED.

## Implementation source refresh — 2026-11-21

The latest Issue #104/#105 code-bearing source reviewed is `72eae103bc3304b0961c66a0cf61f716ddfa9ac4`. It contains the native filesystem backends and bounded process-containment implementation described in the release blocker register. This establishes source presence, not successful build/runtime behavior. The CLI filesystem integration fixtures were aligned with the host-working-directory authority contract in commits `46c777e70f396cfee6d796d0667769dabe2d7208` and `2600ec18d4e3fb05558ca50d7cf466638e1ab015`; those commits change tests only and skip CI. Broad verification is intentionally deferred until the implementation inventory is complete. Do not promote IMPLEMENTED to VERIFIED or SUPPORTED, and retain the explicit Android/macOS nested-path and macOS process fail-closed boundaries.


## Scope reconciliation — fixed milestone ledger vs v0.1 release — 2026-11-21

The protected verifier's `LEDGER_SOURCE_COMMIT.txt` pins documentation SHA `d6c382a245c49a028bc60809f22ca956c7f894cc`, which contains the fixed 100-point milestone ledger and Definition of Done. Its 92/100 implementation subtotal (E1 7/10, E2 7/8, E3 8/8, E4 6/6, E5 4/4, E6 0/4) is PROVISIONAL and belongs to the separate milestone-100 workstream; it is not a completion score for PR #113's current v0.1 candidate. The current PR #113 source branch is `security/issue-104-root-confinement-20261118`, with code-bearing candidate `72eae103bc3304b0961c66a0cf61f716ddfa9ac4`. The separate `milestone/100-ecosystem-20261023` branch is not safe to merge as-is and is not the current PR #113 integration base.

The approved v0.1 package/build contract explicitly scopes the executable subset to an empty external dependency set and deterministic debug/intermediate `.oneir` / `.onepkg` artifacts; it does not claim a full external package registry or complete signed provenance envelope. Do not import future vNext/milestone-100 scope into v0.1 merely to chase ledger points, and do not alter the protected ledger, fixed weights, DoD, or release acceptance. Finish implementation against the approved v0.1 contract, then perform the required exact-SHA verification and gate reconciliation.


## Subprocess resource-bound implementation refresh — 2026-11-21

The latest code-bearing source for Issue #81/#105 is `cd95e5ec2234a2909ef79bf8b25fe65296fab5ed`. It explicitly enforces a 10-second deadline, 1 MiB stdout cap, 1 MiB stderr cap, and 2 MiB aggregate output ceiling before returning a process result. Source tests cover stdout overflow, stderr overflow, timeout, normal exit/status/stream preservation, and Linux/Android direct-fork + `setsid` escape attempts for normal completion, timeout, and output overflow. No build or test was run on this source. Issues #81 and #105 remain OPEN; no security or release acceptance is claimed.


## Linux special-file read hardening — 2026-11-21

Code change: `50923b545f2c804b24aa773ea5bb60129d1e0ff2` updates the Linux `openat2` read flags to include `O_NONBLOCK` before the opened object is checked with `fstat` for regular-file type. This prevents opening a FIFO with blocking semantics while retaining the existing handle-relative `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV` policy. A Linux regression `authority_rejects_fifo_without_blocking` creates a FIFO with no writer and requires `FilesystemError::NotRegularFile`. The regression has NOT RUN; this is source/test presence only, not execution evidence or Issue #104 acceptance.


## Filesystem read-allocation and resource-limit hardening — 2026-11-21

Code commit: `8792f7a1f707f444d00026d916fa8c18b7f3e7f1`.

- Replaced the unchecked `u64 as usize` conversion used to preallocate filesystem read buffers with `usize::try_from` and `Vec::try_reserve_exact`. Oversized capacities now fail with a defined filesystem error instead of truncating on 32-bit hosts or panicking on reservation failure.
- Added focused Linux regression `authority_enforces_read_and_write_limits_without_partial_replacement`; it verifies an oversized read is rejected, an oversized write is rejected, and the existing target bytes remain unchanged.
- This regression is NOT RUN. No build, test, CI, verifier, or platform matrix is claimed. Issue #104 remains OPEN / NOT ACCEPTED.


## Linux nested-write fallback and verifier-inventory alignment — 2026-11-21

Code commits:
- `0fd434ad46e6b8e93c3b9ed887447baf10d0f697`: classify Linux `EOPNOTSUPP` from required detached-mount primitives as `FilesystemError::UnsupportedPlatform`, preserving fail-closed behavior when the backing filesystem/kernel cannot provide the required operation.
- `858a5da09fe2ea74ebd17b0798f13fac848c7a28`: replace the stale test that required every Linux nested write to fail with `nested_write_uses_native_backend_or_fails_closed_without_side_effects`. It accepts a successful native detached-mount write or an explicit `UnsupportedPlatform` fallback, and checks target integrity and temp-file cleanup.

The independent verifier's expected source test-filter inventory is now structurally matched in `adapter.rs`: 3 `descriptor_relative_read_rejects_`, 5 `descriptor_relative_write_`, and 1 `descriptor_relative_walk_rejects_` tests. This is a source inventory check only; no tests were run and no verifier PASS is claimed. Issue #104 remains OPEN / NOT ACCEPTED.


## Duplicate subprocess-test helper compile blocker removed — 2026-11-21

- Source commit: `0cceeb447fbe2ae59b8e3da78dee3ce8d9959830` (`crates/one-core/src/interop.rs`).
- Removed the second identical `cleanup_test_escaped_group` definition from the `interop.rs` test module. The duplicate would produce a Rust duplicate-definition error when compiling the test module; only one definition remains in source.
- This is a source-level compile-blocker correction, not a compile/test result. No build, test, CI, verifier, or platform matrix was run. Issue #105 remains OPEN / NOT ACCEPTED.


## Android/macOS filesystem helper repair and nested-path security boundary — 2026-11-21

Latest code-bearing source: `5f98db9f70d28a0d5a206a3e6e37f986efc3574b` (`crates/one-core/src/filesystem.rs`).

- Removed the duplicate Android `create_temp` definition and supplied the missing macOS `create_temp` helper. These were source-visible compile blockers; no build was run.
- Android/macOS direct-child read/write backends remain. Their root-relative direct-child operations retain no-follow checks, regular-file/single-link checks, Android `statx(STATX_MNT_ID)` or macOS `fstatfs` identity checks, and same-directory atomic replacement.
- Nested Android/macOS paths remain fail-closed. A proposed generic openat parent walk was deliberately not enabled: ordinary POSIX directory handles do not pin an opened parent beneath the retained root, so an attacker may rename that directory outside the root before the leaf operation. The current resolver rejects multi-component paths with `UnsupportedPlatform` until an anti-move primitive is implemented.
- Android/macOS regression fixtures now cover nested-path fail-closed behavior, leaf symlinks, and an outside sentinel. These tests are NOT RUN.
- This is source implementation hardening and a precise capability boundary, not acceptance. Issue #104 remains OPEN / NOT ACCEPTED. No build/test/CI/verifier/platform matrix was run and no status was promoted.


## 2026-11-21 — Android/macOS filesystem source repair and confinement boundary

Latest code-bearing source: `0f5859426273a61174d6d1b08ed8771a6f4b136a` (`crates/one-core/src/filesystem.rs`).

- Android's duplicate `create_temp` definition has been removed; macOS now has its missing exclusive `create_temp` helper. Static source inventory shows one helper definition in each native module. No compiler was run.
- Android/macOS direct-child operations retain root-handle-relative no-follow reads and same-directory exclusive-temp atomic replacement; Android requires `statx(STATX_MNT_ID)`, while macOS compares `f_fsid` and mount point with `fstatfs`. Regular-file and hard-link checks remain.
- Nested paths on both POSIX backends explicitly return `UnsupportedPlatform`. Ordinary POSIX directory descriptors do not prevent a parent directory from being renamed outside the retained root, so a multi-component openat walk would not satisfy the root-confinement race contract. No such unsafe nested support is enabled.
- Android/macOS source regressions cover direct-child operation, leaf symlink rejection, nested-path fail-closed behavior, root symlink rejection, and hard-link side-effect preservation. These tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED. This source correction is not a build/test/verifier result; no platform support or release approval is promoted.


## 2026-11-21 — bounded HTTP response ingestion and filesystem payload allocation

Latest relevant source commits:
- `d32bb207f8ea6f24740560f374a4f698c687e172` — `FileWriteAdapter` now passes a borrowed text slice to `FilesystemAuthority::write`, so the configured write-byte limit is enforced before an owned output copy is made. The bounded echo copy happens only after a successful write. Added `filesystem_write_adapter_rejects_oversized_content_without_side_effects` (NOT RUN).
- `ce39630fc9abdc9ba7c6accdf8db161a5cb002fa`, `8c219882aa37ee271874f58de172072531d80ddc`, `b636f0bc1e22008fc62dafe46a378cc2fb2b599b`, and `5fe958cad798b3d05c3ceb21f875f1f90074cb35` — HTTP/1.1 response ingestion now enforces a 64 KiB header ceiling and 32 MiB wire-body ceiling while reading, with fallible buffer growth; decoded response bodies remain capped at 16 MiB. The parser rejects duplicate Content-Length, unsupported/duplicate Transfer-Encoding, and simultaneous Content-Length + chunked framing; chunk-size arithmetic is checked and chunk allocation is fallible.
- Added source regressions for oversized headers, ambiguous framing, duplicate lengths, chunk-size overflow, and bounded socket reads. NOT RUN.

These are implementation/resource-boundary changes only. No build, test, CI, verifier, or platform matrix was run. No prior exact-SHA result transfers to this candidate.


## HTTP request serialization resource bounds — 2026-11-21

Latest source commit: `198fbf4ccdbb7e219efbe3e296e98462a385c384` (`crates/one-core/src/network_runtime.rs`).

- HTTP request bodies are capped at 16 MiB before serialization; request line + headers are capped at 64 KiB with checked size arithmetic and fallible preallocation.
- Explicit Content-Length must parse and exactly match the body size; duplicate Content-Length and request Transfer-Encoding are rejected (chunked request serialization is not implemented).
- Added source regressions for oversized request bodies, mismatched Content-Length, and unsupported Transfer-Encoding. NOT RUN.
- No build, test, CI, verifier, or platform matrix was run. This remains an implementation candidate, not verification evidence.


## HTTP request syntax validation — 2026-11-21

Latest source commit: `b0a3379a1fb49feb18dcf61a2521446f199a8e2b` (`crates/one-core/src/network_runtime.rs`).

- Request method and header names must be valid HTTP tokens.
- Authority rejects whitespace/control bytes, user-info separators and fragments; request-target must use origin-form or `*`, and rejects whitespace/control bytes and fragments.
- Header values reject control bytes other than horizontal tab.
- These checks complement the 16 MiB request-body / 64 KiB header caps, Content-Length consistency checks, and response framing limits. Source-only change; no build or tests run.


## HTTP exact framing and trailer validation — 2026-11-21

Latest source commit: `eea3115ba5e70ff83f8ed78a01dc70d95e012ff0` (`crates/one-core/src/network_runtime.rs`).

- Content-Length response bodies must now match the declared length exactly; trailing bytes are rejected.
- Chunked responses bound chunk-size lines, require valid terminating CRLF, validate trailer syntax, reject framing/Host fields in trailers, and reject trailing bytes after the trailer terminator.
- Added regressions for valid trailers, forbidden Content-Length trailers, and extra bytes after Content-Length. NOT RUN.
- No build/test/CI/verifier/platform matrix was run; source-only candidate, no acceptance promoted.


## HTTP header token/control validation regressions — 2026-11-21

Latest source commit: `11bf5172ed060818dbd55d2ecee3cddf3cd99875` (`crates/one-core/src/network_runtime.rs`).

- Response header names are validated as HTTP tokens and values reject control bytes except HTAB. Request header names/values and request-target syntax have equivalent validation.
- Added regressions for invalid response header names/values and invalid request header names/targets. NOT RUN.
- No build/test/CI/verifier/platform matrix was run; this remains source-only implementation work.


## Exact fallible HTTP buffer reservations — 2026-11-21

Latest source commit: `bbd4febbac3a4ba2a19195582e7e5992d1953b1c` (`crates/one-core/src/network_runtime.rs`).

- HTTP request serialization, response ingestion, and chunk-body decoding now use `try_reserve_exact` within the explicit request/response size ceilings, avoiding geometric capacity growth beyond the intended bounds.
- This is source-only resource hardening. No build/test/CI/verifier/platform matrix ran.


## IPv6 HTTP authority and query-only target support — 2026-11-21

Latest source commits:
- `f8631323b7c987c69f2ab8072bbf081deb626fb9` — shared HTTP authority parser accepts bracketed IPv6 literals with default/explicit ports; generic network resolution now formats IPv6 socket addresses correctly.
- `4b0749fbd413233ee45259942a334190474ac89e` and `5bc86f6110145074cbee96942ad7942e25318b9c` — HTTP GET target parsing uses the shared authority parser, supports bracketed IPv6 and query-only paths, and rejects malformed/whitespace request paths.
- `b0e034b3c98248e6f33823bd1ff9f2242c89fa8f` and `be5566435394cc27bbeb863b78c69b0fe52f774c` add source regressions for IPv6 authorities, query-only paths, and invalid paths. NOT RUN.
- No build/test/CI/verifier/platform matrix was run.


## Process adapter smoke-fixture correction — 2026-11-21

Latest source commit: `b4b4b3ee82e013eb3c001606820deaa76d50a9df` (`crates/one-core/src/adapter.rs`).

- Replaced the no-argument `rustc` smoke target (which can exit nonzero when no input file is supplied) with `whoami`, a no-argument command that emits output on Linux, Android and Windows.
- Cfg-gated the smoke test to the platforms where ProcessAdapter advertises native containment, and included Android in the process-platform metadata regression.
- Test source only; NOT RUN. No build/CI/verifier evidence is claimed. Issue #105 remains OPEN / NOT ACCEPTED.


## SQLite query resource boundaries — 2026-11-21

Latest source/test candidate: `76f76c9a7a5884c08d851afc203cb1cae7a5827a` (NOT BUILT/TESTED).

- SQL text is capped at 64 KiB; parameters at 128 values / 1 MiB total; result rows at 100,000; serialized output at 16 MiB.
- SQLite engine limits are now configured per connection through rusqlite's `limits` feature: 16 MiB value/row length, 64 KiB SQL, 256 columns, 128 variables, bounded compound SELECT/function args/expression depth/LIKE pattern, zero attached databases, and zero auxiliary worker threads.
- Result serialization uses fallible exact growth; BLOB hex size is checked before allocating/copying, and invalid UTF-8 text is streamed through bounded lossy conversion.
- Added regressions for oversized SQL and BLOB hex output. NOT RUN. No build/test/CI/verifier/platform matrix was run.


## SQLite query execution-time and lock-wait limits — 2026-11-21

Latest source/dependency commits:
- `f6716d2c9dc83208de695c699804da7ccde9aec0` enables rusqlite `hooks` alongside `limits`.
- `91d2a6c3a49041284944a4dd12ae090bb783bd0d` sets a 5-second SQLite VM progress deadline and a 250 ms busy/lock wait timeout for each query connection.

These controls complement the SQL/parameter/row/output caps and per-connection SQLite engine limits. Source-only change; no build/test/CI/verifier/platform matrix was run.


## SQLite deadline covers output serialization — 2026-11-21

Latest source commit: `75bf347d7bbb9cce89a488b02cc285c7f605e984` (`crates/one-core/src/database.rs`).

- The 5-second query deadline now applies both to SQLite VM execution (progress handler) and to Rust-side row/result serialization, so time spent formatting large result sets is also bounded.
- Source-only change; no build/test/CI/verifier/platform matrix ran.


## Additional SQLite parser/VM limits — 2026-11-21

Latest source commit: `4c01b6eb2b8ba2d75a316a1a792352657a74c73a` (`crates/one-core/src/database.rs`).

- Added per-connection SQLite limits for parser depth (1,000), VDBE instruction count (1,000,000), and trigger recursion depth (0), complementing the previously recorded length, SQL, column, variable, expression, compound-select, function, LIKE, attachment, and worker-thread limits.
- Source-only hardening; no build/test/CI/verifier/platform matrix ran.


## Absolute HTTP response read deadline — 2026-11-21

Latest source commits: `981467252a95198adfc72872db2dc652fa9f505e` and `a911431be5a1dad31dc2344e09a357e2d5e91212` (`crates/one-core/src/network_runtime.rs`).

- Bounded response ingestion now uses one absolute deadline across all socket reads, updating the socket timeout to the remaining budget each iteration. This prevents a slow-trickle peer from resetting the full per-read timeout indefinitely.
- The bounded-reader regression fixture now supplies its explicit deadline. NOT RUN.
- No build/test/CI/verifier/platform matrix ran.


## 2026-11-21 — Android nested-read implementation with kernel-enforced confinement

Latest source candidate: `52fcfb2706fcb48e9599d75f9086578279f1bbac` (`crates/one-core/src/filesystem.rs`, NOT BUILT/TESTED).

- Static source inventory at this candidate finds exactly one `create_temp` definition in `android_native` and exactly one in `macos_native`. The earlier duplicate Android helper was already removed; macOS's missing helper was added. This is source inspection only, not Android/macOS compile evidence.
- Android nested reads now use Linux/Android `openat2` with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`. The kernel performs the complete path resolution relative to the retained root descriptor and rejects symlink, mount-crossing, escape, and unsafe rename-race outcomes. The opened leaf is still checked for regular-file type, single hard-link, device identity, and `STATX_MNT_ID` equality.
- Android nested writes remain fail-closed (`UnsupportedPlatform`) until an anti-move atomic replacement backend is implemented. Direct-child atomic writes remain unchanged.
- macOS nested reads and writes remain fail-closed. Ordinary `openat` directory-descriptor walks cannot prevent a parent directory from being renamed outside the root; no unsafe traversal has been enabled. Direct-child operations remain unchanged.
- Added Android source regression for a valid nested read and a symlinked-parent escape with outside-sentinel preservation. Existing Android expectations now distinguish missing nested files (`NotFound`) from symlink-parent denial (`ResolutionFailed`). Tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED. No build, test, CI, verifier, or platform matrix ran; no platform support or release approval is promoted.


## Filesystem implementation refresh — 2026-11-21

- Source candidate `9fe53815f83701a62536472f8a3c87cebac22e7a` adds Android nested writes using detached mount cloning, recursive `MOUNT_ATTR_NOSYMFOLLOW`, `openat2` resolution with `BENEATH | NO_SYMLINKS | NO_XDEV`, and atomic `renameat2`; unsupported or denied primitives fail closed. macOS nested reads use XNU `O_RESOLVE_BENEATH | O_NOFOLLOW_ANY | O_UNIQUE` and `fstatfs` mount checks, while nested writes remain fail-closed until a no-cross-mount atomic publication primitive is proven. Source tests were updated but have not been run. Do not promote build/test/platform support.


## 2026-11-21 — nested filesystem and CI lint follow-up

- Source candidate `568d9360f0beec63c10fbeae9cac63f384bd082f` contains Android detached-mount nested writes and macOS kernel-bounded nested reads (supported XNU kernels), preserving the old direct-child macOS read path. macOS nested writes remain fail-closed.
- Follow-up Clippy fixes propagate every SQLite `set_limit` and `progress_handler` error as `AdapterError::BackendFailure` and use `str::find(['/', '?'])` for the HTTP authority delimiter. They preserve fail-closed resource policy and `-D warnings`.
- Run #2410 on parent source SHA `9fe53815f83701a62536472f8a3c87cebac22e7a` had Ubuntu Build PASS / Clippy FAIL / Tests SKIPPED. Descendant fixes have NOT been re-run successfully. Latest candidate NOT BUILT/TESTED.


## 2026-11-21 — cross-platform compile correction

- Source candidate `74df3aa901732c8eb3adaac728162715fc3a4961` gates the Linux direct/nested write wrapper to Linux, preventing Unix-only APIs and `libc` from compiling on Windows. Android uses only the shared nested helper where intended.
- Windows root path identity now reopens the candidate path through the native no-reparse `NtCreateFile` path and compares `BY_HANDLE_FILE_INFORMATION` volume serial/file index; unstable `MetadataExt` APIs were removed.
- Run #2410 on parent source `9fe53815f83701a62536472f8a3c87cebac22e7a` failed Clippy on Ubuntu and Build on Windows. Corrections are source-only; no rerun on `74df3aa...` is available. NOT BUILT/TESTED.

## 2026-11-21 — Filesystem delete native implementation boundary

Source commit ffa51679e998e97bca0730a182d8887a245e03d0 adds a Windows-only host API FilesystemAuthority::delete. Windows opens every parent component relative to the retained root with no delete sharing, opens the leaf with DELETE_ACCESS without following reparse points, verifies volume/type/link count, and applies FileDispositionInformation to the opened handle. It does not perform a second path lookup after validation.

The API intentionally returns UnsupportedPlatform on Linux, Android, and macOS. POSIX unlinkat remains name-based and is not treated as an equivalent handle-bound primitive. No language semantic capability or adapter is exposed yet. Two Windows tests were added but are NOT BUILT / NOT RUN. Issue #104 remains OPEN / NOT ACCEPTED; this is not v0.1 completion evidence.
Additional source test retained_delete_handle_blocks_leaf_rename checks that the Windows DELETE-access/no-delete-share leaf handle blocks rename while retained. NOT BUILT / NOT RUN.

## 2026-11-21 — Windows delete implementation verified on CI

Windows handle-bound file deletion is implemented and has exact-SHA GitHub Actions evidence. Run #44870781889 on source/test SHA 6f2d5e2d0b02d6aed6c692a8a7b7fa184a70fcb0 completed SUCCESS on Ubuntu and Windows Server 2025. Both jobs passed Build, Clippy, Tests, and deterministic artifact steps. Windows tests delete_uses_validated_handle_and_removes_nested_regular_file, delete_rejects_hard_links_without_removing_either_name, and retained_delete_handle_blocks_leaf_rename passed; the existing retained_parent_handle_blocks_directory_rename passed too. Windows one-core library summary: 156 passed / 0 failed.

This verifies only the Windows native host API slice. Linux, Android, and macOS delete still return UnsupportedPlatform; no semantic capability or adapter has been added. Issue #104 remains OPEN / NOT ACCEPTED.

## 2026-11-21 — filesystem.delete language bridge (unverified)

The Windows native delete host API has its own exact-SHA CI evidence (run #44870781889 on `6f2d5e2d0b02d6aed6c692a8a7b7fa184a70fcb0`). A subsequent language bridge now adds:

- `Capability::FilesystemDelete`, not implied by coarse filesystem/read/write grants;
- semantic ID `filesystem.delete` and effect `Delete`;
- source action `hapus berkas "path"` with text-only target validation;
- IR lowering to `filesystem.file.delete`;
- Windows-only `FileDeleteAdapter` using the injected `FilesystemAuthority`;
- interpreter registration, default-denial/no-side-effect test, and Windows explicit-grant execution test.

Current source checkpoint `0fa43eb0be53c8e79424597ece055147872b35c4` is NOT YET BUILT / TESTED. Native Windows CI evidence at the earlier SHA must not be reused as proof for this newer language bridge. Linux/Android/macOS remain unadvertised and fail-closed. Issue #104 remains OPEN / NOT ACCEPTED.

## 2026-11-21 — filesystem.delete language bridge verified (Windows-only)

Source/test SHA 0fa43eb0be53c8e79424597ece055147872b35c4 passed GitHub Actions One CI run #44871756593. Ubuntu job #136747608726 passed Build, Clippy, and Tests with one-core 181 passed / 0 failed / 3 ignored. Windows Server 2025 job #136747608925 passed Build, Clippy, and Tests with one-core 167 passed / 0 failed.

The exact-SHA tests prove the Indonesian action parses and lowers to filesystem.delete/Delete, default policy denies deletion without changing the file, an explicit scoped grant reaches the Windows native adapter, and Windows adapter metadata is Windows-only. Windows logs also prove the actual directory-junction escape test rejects the target and preserves the outside sentinel. GitHub status context ci/circleci: one-linux is SUCCESS on the same SHA; provider logs were not independently reviewed.

Status: Implemented and Verified for the narrow Windows-only file-delete slice. Not Supported cross-platform. Linux, Android, and macOS remain unavailable/fail-closed. Issue #104 remains OPEN / NOT ACCEPTED because POSIX delete, host-root integration, remaining E4 adversarial matrix, and release acceptance are still outstanding.

## 2026-11-21 — filesystem.delete CLI and process containment source update

- The Windows handle-bound delete host API and the one-core language bridge retain their prior exact-SHA PASS evidence (`6f2d5e2...` / run #44870781889; `0fa43eb...` / run #44871756593). Those results do not verify the newer CLI integration.
- CLI now exposes an explicit `filesystem.delete` grant and exact `--target` scope through the host-created `FilesystemAuthority`. Source/test commits `48e87fd...` and `17a64b2...`; CLI integration is NOT BUILT / NOT TESTED. Linux, Android, and macOS remain fail-closed for delete because no handle-bound native primitive equivalent to Windows disposition has been proven.
- Linux/Android process containment remains inherited seccomp plus invocation process-group cleanup; Windows uses Job Objects; macOS remains fail-closed. The source adds unique per-test working directories and `setpgid(0,0)` adversarial probes across normal completion, timeout, and output overflow (`5e79f35...`, `18bd42f...`, `477537c...`). These tests are NOT BUILT / NOT TESTED.
- Issue #104 and #105 remain OPEN / NOT ACCEPTED. Do not promote the feature state, evidence scores, or DoD based on source-only additions.

## 2026-11-21 — XNU nested write candidate and Android source correction

- Latest code-bearing source candidate: `c7a2d45a0e218c7e26a3f4b2dad0a206e024307e` (NOT BUILT / NOT TESTED).
- macOS nested write source now stages a unique temp file directly under retained root and publishes with XNU `renameatx_np(RENAME_RESOLVE_BENEATH | RENAME_NOFOLLOW_ANY)`. Existing target and parent are checked with XNU `O_RESOLVE_BENEATH | O_NOFOLLOW_ANY | O_UNIQUE` plus `fstatfs`; root-mount staging means cross-mount publication fails `EXDEV`. Source regressions cover positive nested replace, symlink-parent outside sentinel, hardlink preservation, and parent moved outside root immediately before publish. Not yet executed on macOS.
- Android source correction: current `android_native` rejects multi-component read/write paths. It uses direct-child `openat` and statx mount-ID checks only. Observed Termux seccomp kills `openat2` with SIGSYS; earlier nested `openat2`/detached-mount design notes are not evidence of current source support.
- macOS delete remains fail-closed: XNU `unlinkat` can enforce beneath/no-symlink/unique path checks, but no atomic no-cross-mount flag was found; a separate `fstatfs` precheck is raceable. Linux/Android delete remain fail-closed; Windows handle-bound delete unchanged.
- macOS process containment research: Seatbelt `process-fork` denial may prevent fork/vfork, but `posix_spawn` and all descendant creation routes have not been proven blocked. No production backend added; macOS remains fail-closed.
- Issue #104 and #105 remain OPEN / NOT ACCEPTED. No test evidence or support claim is promoted.
