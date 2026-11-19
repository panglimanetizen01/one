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
