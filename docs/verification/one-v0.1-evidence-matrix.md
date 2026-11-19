# One v0.1 Evidence Matrix

## Live integration checkpoint — 2026-11-17

- Integrated code/test source commit on PR #108: `e8cd41ec212c3224bdbecf85eaf18c5850096e16`, based on prior PR #108 HEAD `735fe6d9814534ac5699846f4ed611dc3ca92b85`.
- Paths integrated: `crates/one-cli/src/main.rs`, `crates/one-cli/tests/cli.rs`, `crates/one-core/src/database.rs`, `crates/one-core/tests/database_vertical_slice.rs`, `crates/one-core/src/interpreter.rs`, `crates/one-core/src/governance.rs`.
- Evidence status for the combined candidate: NOT RUN / NOT VERIFIED. No Ubuntu or Windows run ID exists yet; no CircleCI job-level evidence is available. The individual PR #98–#101 runs remain historical parent-SHA evidence only.
- Do not promote implementation scores, DoD boxes, or E1–E6 from this integration alone.


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

This matrix ties each release claim to a canonical artifact. A claim is not considered verified merely because an implementation file exists.

| Requirement | Canonical artifact | Evidence required | Current state |
|---|---|---|---|
| Lexical rules | `docs/spec/one-v0.1-lexical.md` | lexer tests + negative lexical fixtures | IMPLEMENTED / independently exercised |
| Concrete grammar | `docs/spec/one-v0.1-grammar.md` | parser tests + valid/invalid fixtures | IMPLEMENTED / independently exercised |
| AST | `docs/spec/one-v0.1-ast.md`, `crates/one-core/src/ast.rs` | AST parser tests | IMPLEMENTED / independently exercised |
| Semantic analysis | `crates/one-core/src/semantic.rs` | scope/type tests | IMPLEMENTED / independently exercised |
| Semantic IR | `crates/one-core/src/ir.rs` | IR tests | IMPLEMENTED / independently exercised |
| Reference interpreter | `crates/one-core/src/interpreter.rs` | core execution + golden tests | IMPLEMENTED / independently exercised |
| Diagnostics | `docs/spec/one-v0.1-diagnostics.md`, `diagnostic.rs` | diagnostic-code tests | IMPLEMENTED / independently exercised |
| Capability boundary | `capability.rs`, `platform.rs`, `SECURITY.md` | denied-capability tests | IMPLEMENTED / independently exercised |
| Standard library | `docs/spec/one-v0.1-stdlib.md`, `stdlib.rs` | behavior tests | IMPLEMENTED / independently exercised |
| Process interoperability | `docs/spec/one-v0.1-interop.md`, `interop.rs` | process-boundary tests | IMPLEMENTED / independently exercised |
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli-contract.md` | CLI integration + smoke tests | VERIFIED within recorded exact-SHA platform boundaries |
| Contract consistency | `tools/check_contract.py` | artifact/index/fixture scan | VERIFIED when executed against recorded verification revisions |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | IMPLEMENTED / independently exercised |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | IMPLEMENTED / independently exercised |
| Deterministic `.oneir` | `tools/verify_exact_sha.sh`, deterministic CLI test | byte-deterministic build/package evidence | VERIFIED within recorded exact-SHA platform boundaries |
| Deterministic `.onepkg` | `tools/verify_exact_sha.sh`, deterministic CLI test | byte-deterministic build/package evidence | VERIFIED within recorded exact-SHA platform boundaries |
| Orchestration domain/governance | `orchestration.rs`, `governance.rs`, `orchestrator.rs` | unit/integration tests | VERIFIED within recorded exact-SHA platform boundaries |
| Orchestration adapter boundary | `adapter.rs`, `network_dns_adapter.rs`, `database.rs` | adapter tests + runtime | VERIFIED within recorded exact-SHA platform boundaries |
| Orchestration audit/result normalization | `orchestration_audit.rs`, `orchestration_result.rs` | audit/normalization tests | VERIFIED within recorded exact-SHA platform boundaries |
| Language-level DNS orchestration | parser → semantic → typed IR → governance → DNS adapter → normalized result → CLI | end-to-end source execution + explicit permission | EXPERIMENTAL / verified within current recorded boundary |
| Language-level filesystem-read orchestration | parser → semantic → typed IR → governance → filesystem.read adapter → normalized result → CLI | end-to-end source execution + allow/deny/out-of-scope/backend tests | EXPERIMENTAL / verified within recorded Ubuntu/Windows evidence |
| Language-level process execution orchestration | parser → semantic → typed IR → governance → process.execute adapter → normalized result → CLI | source execution + permission + denial + backend-failure tests | EXPERIMENTAL / verified within recorded Ubuntu/Windows evidence |
| Language-level filesystem-write orchestration | parser → semantic → typed IR → governance → filesystem.write adapter → CLI | source execution + explicit permission + exact-target write + denial | EXPERIMENTAL / Termux evidence plus exact-SHA Ubuntu/Windows workspace matrix at `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` |
| Language-level database-query orchestration | parser → semantic → typed IR → resource-scoped governance → `database.sqlite.query` → normalized result → CLI | source execution + explicit permission + out-of-scope denial + backend failure + deterministic normalization | EXPERIMENTAL / Termux evidence plus exact-SHA Ubuntu/Windows matrix `39554661451` at `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` |
| Database-query CLI regression | `crates/one-cli/tests/database_orchestration.rs` | source + `--izinkan database.query` + matching resource + denial/scope tests | VERIFIED in exact-SHA Ubuntu/Windows matrix `39554661451` (`77a89a6552c4d4306bd1a97752789b04ae1bbb0c`) and historical Termux evidence |
| Language-level HTTP GET orchestration | parser → semantic → typed IR → target-scoped governance → `network.http.get` → normalized result → CLI | loopback source execution + explicit permission + out-of-scope denial + adapter validation | EXPERIMENTAL / Termux evidence plus exact-SHA Ubuntu/Windows matrix `39554661451` at `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` |
| HTTP GET CLI regression | `crates/one-cli/tests/http_get_orchestration.rs` | real loopback GET + `--izinkan network.connect` + matching target + denial/scope tests | VERIFIED in exact-SHA Ubuntu/Windows matrix `39554661451` (`77a89a6552c4d4306bd1a97752789b04ae1bbb0c`) and historical Termux evidence |
| Filesystem-write CLI regression | `crates/one-cli/tests/filesystem_write.rs` | source file + `--izinkan filesystem.write` + actual target content | VERIFIED within recorded Termux/Android boundary |
| Filesystem-write policy/runtime | `crates/one-core/tests/filesystem_write_runtime.rs` | default denial + target-scoped grant + exact text output | VERIFIED within recorded Termux/Android boundary |
| Native backend contract | `docs/superpowers/specs/2026-10-19-native-backends-linux-windows.md` | target/toolchain contract + backend tests + artifact verification | VERIFIED for PR #64 scope |
| Native Linux AArch64 | `NativeLinuxAarch64Backend` + native backend tests | real ELF AArch64 build + target execution | AArch64 execution is verified for the recorded earlier implementation SHA and AArch64/Linux host; the exact-SHA source matrix passes but does not itself prove AArch64-target execution |
| Native Linux x86_64 | `NativeLinuxX86_64Backend` + `linux_x86_64_build_menggunakan_cross_linker_x86_64` | real ELF x86-64 build + target execution | Exact-SHA source matrix passed for source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`; preserve the specific target execution boundary recorded by the native backend tests |
| Native Windows x86_64 | `NativeWindowsX86_64Backend` + Windows native verification workflow | real PE32+ x86-64 build + target execution | Exact-SHA Windows matrix passed for source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`; preserve the specific target execution boundary recorded by the native backend tests |
| Native CLI target selection | `crates/one-cli/src/native_main.rs`, `crates/one-cli/tests/native_cli.rs` | explicit target selection + regression tests | IMPLEMENTED; covered by the exact-SHA workspace test matrix for source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` |
| Broader language-level orchestration | same source-to-runtime chain for additional capabilities | per-capability end-to-end tests | IN DEVELOPMENT |
| Linux portability | verification mirror workflow | actual Linux run | VERIFIED for canonical SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` in matrix run `39554661451` |
| Windows portability | verification mirror workflow | actual Windows run | VERIFIED for canonical SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` in matrix run `39554661451` |
| Release gate | verification mirror workflow | green exact-SHA matrix | Exact-SHA cross-platform matrix passed for the last code-bearing source revision; release approval remains a separate project-level decision |

## Last code-bearing canonical revision verified

The last code-bearing source revision in canonical `main`, `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`, was verified by independent matrix run [`39554661451`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451). The matrix passed on Ubuntu and Windows Server 2025; its workspace test step plus the exact-SHA verifier's targeted suites cover the filesystem-write, database-query, and HTTP GET source-level tests. Subsequent commits in this evidence-sync series modify documentation only and do not change source or tests.

The matrix completed successfully on Ubuntu and Windows Server 2025. Both jobs checked out the canonical One source at the requested target SHA and passed contract artifacts, build, AST contract regression tests, Clippy, workspace tests, real-network runtime tests, and platform-appropriate offline verification. Windows also provisioned the Windows cross-compilers and Android NDK; Ubuntu provisioned cross-compilers.

Canonical hosted `One CI` runs `39372356906` and `39381125667` failed before executable steps (`steps: []`). Those runs are infrastructure evidence and do not negate the successful independent exact-SHA matrix or establish a source-code failure.

## Cross-platform evidence boundary

Run `39554661451` verifies the exact code-bearing source SHA above on Ubuntu and Windows Server 2025. Earlier evidence such as `38929072800` remains historical and must not be substituted for current-SHA evidence. A passing host matrix does not automatically prove execution on every cross-compiled target architecture; target-specific statements remain bounded to the artifacts and execution evidence recorded for each target.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline or local verification is valid independent execution evidence only when its environment, toolchain, command, exact commit, and result are recorded.
- `VERIFIED` is reserved for claims backed by actual evidence for the exact recorded source SHA and stated platform boundary.
- Evidence for one commit does not automatically verify later commits.
- Filesystem write, database query, and HTTP GET retain their historical Termux/Android evidence; their source/core/CLI tests also passed in the Ubuntu/Windows Server 2025 exact-SHA matrix `39554661451` for `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`. This does not imply every target architecture was executed.
- Native Linux x86_64 and Windows x86_64 are build-proven on PR #64 but are not execution-verified until a real target executor succeeds.
- The last code-bearing source revision `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` has host-level Ubuntu/Windows exact-SHA verification in run `39554661451`. This does not certify every cross-compiled target artifact as executed, and does not by itself constitute release approval. Later evidence-sync commits changed documentation only.

## Live evidence reconciliation — 2026-11-18

| Item | Exact revision | Evidence | Classification |
|---|---|---|---|
| PR #108 CircleCI Linux | `286a0fd563e10611a3465cb6457ab069a0df8a52` | GitHub status `ci/circleci: one-linux` SUCCESS; job `99216205-62aa-4e98-af42-d4b742a5feaa` | Exact-SHA CircleCI status verified; expanded logs not inspected |
| PR #110 CircleCI Linux | `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b` | GitHub status `ci/circleci: one-linux` SUCCESS; job `c9916871-9746-4c65-b8fb-615d9f96dc78` | Exact-SHA CircleCI status verified; expanded logs not inspected |
| PR #110 filesystem confinement | `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b` | Direct source read: adapters still call `std::fs::read(path)` / `std::fs::write(path, ...)` | NOT IMPLEMENTED / acceptance OPEN |
| ONE canonical main | `d35db95153f6ddd79885494c72f5aee16c899751` | Live GitHub commit read | Unchanged; do not modify |

These CircleCI status results are specific to their listed exact SHAs and do not prove cross-platform Windows execution, adversarial filesystem confinement, or the complete release gate. Historical Ubuntu/Windows PASS results listed above remain bound to their original source SHAs.



## Documentation HEAD verification — 2026-11-18

PR #110 current documentation HEAD `26d29c61944e51919161f5360bccfef4a09d370d` has CircleCI `ci/circleci: one-linux` SUCCESS, job `c153ba39-4073-4548-aaa6-6370e6bed6bf`. Production filesystem source assessment remains bound to parent source SHA `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b`; the child commit is documentation-only.

## Independent baseline cross-platform verification — 2026-11-18

| Source SHA | Platform / provider | Run / job | Result |
|---|---|---|---|
| `a20676be53e5f78c465832379d86c3675a2ed7c8` | Ubuntu 24.04.5 / GitHub Actions | [44267886845](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44267886845), job `134676844501` | 239 passed, 0 failed, 1 ignored; 27 result summaries |
| `a20676be53e5f78c465832379d86c3675a2ed7c8` | Windows / GitHub Actions | [44263674485](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44263674485/job/134662858839) | 232 passed, 0 failed, 1 ignored; 27 result summaries |
| `2ab82ea9cccb1c9aa0fef25c3591a1310ab84b5c` | CircleCI one-linux | PR #110 commit status | SUCCESS; status context `ci/circleci: one-linux` |

The first two rows are independent cross-platform tests of the same PR #108 candidate SHA. The CircleCI row is a separate PR #110 source SHA and is Linux-only. None of these results implement or accept Issue #104 filesystem confinement. The adapters still require explicit trusted-root authority and adversarial tests before E4/DoD changes.


## Issue #104 candidate evidence reconciliation — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| PR #113 E4 Unix + Windows filesystem adversarial suite | `4c3bccaf59d965ce8b446e0b84cb84c00ff2eed6` | [Verifier run #44327291926](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44327291926) | PASS / VERIFIED for this SHA only |
| PR #113 full Ubuntu + Windows Server 2025 matrix | `4c3bccaf59d965ce8b446e0b84cb84c00ff2eed6` | [Verifier run #44327291454](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44327291454) | PASS / VERIFIED for this SHA only |
| SEC-05, distro matrix, Android artifact | `4c3bccaf59d965ce8b446e0b84cb84c00ff2eed6` | [#44327291414](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44327291414), [#44327292023](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44327292023), [#44327291459](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44327291459) | PASS / VERIFIED for this SHA only |
| Android API 35 AArch64 runtime | Earlier pinned candidate SHA; run #44327291656 | [Runtime workflow #44327291656](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44327291656); emulator boot step remained IN_PROGRESS at last inspection | IN PROGRESS; not runtime VERIFIED |
| Windows atomic write implementation | `5c5bfb53e086222f89dc8178426a8c6b65b50a04` | Source includes exclusive sibling temp creation, handle-relative rename, cleanup guard, positive write test, and denied-replacement preservation test | IMPLEMENTED; NOT VERIFIED on this SHA |
| Latest PR #113 HEAD after documentation synchronization | `360b673256a09182150eeb2cc396b1bdb19be2c8` | User guide, Issue #104 design, and implementation-status documentation updates only after code-bearing SHA `5c5bfb5...` | Documentation HEAD; does not replace required source-code verification |
| CircleCI Linux | `02d552123e3cf33017e2066fe62e304e1573476b` | GitHub commit status `ci/circleci: one-linux` PENDING; [job](https://app.circleci.com/workflow/a87975d6-b9c9-4fa4-b18a-bf92c2235045/job/49fb1abd-3870-4332-bcd0-eb370edc24ab) | PENDING; no job-log/root-cause claim |

The verifier results on `4c3bcc...` are historical valid evidence for that exact SHA. They must not be reused as verification of the Windows atomic write implementation introduced in `5c5bfb...`. The later documentation commits do not change production source, but the candidate's final PR HEAD still requires exact-SHA policy/contract checks and a clear distinction between source SHA and documentation HEAD.

Issue #104 remains OPEN and NOT ACCEPTED. The independent verifier pin and canonical `main` are unchanged. Do not promote acceptance ledgers or pins until the latest candidate source and every acceptance criterion are verified.


## Issue #104 latest candidate evidence — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| E4 Windows + Unix, including atomic replacement positive and denied-replacement tests | `20ede9fb1d3e97c04f81d4fe8f79f642a8cc8e78` | [Independent E4 run #44329905887](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44329905887); Windows and Unix jobs PASS, Windows probe PASS | VERIFIED for this SHA only |
| Initial Win32 atomic rename attempt | `5c5bfb53e086222f89dc8178426a8c6b65b50a04` | [E4 run #44329034647](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44329034647); positive write test failed with BackendFailure; diagnostic identified ERROR_INVALID_PARAMETER (87) for Win32 relative RootDirectory | FAILED; retained as historical evidence |
| Latest same-directory rename implementation | `f6acb23a2a0038281c613fb6fd187bc04d2f3dc9` | Source changes rename to `NtSetInformationFile` with `RootDirectory = NULL` and a same-directory leaf name | IMPLEMENTED; NOT VERIFIED pending new runs |
| E4 rerun for latest candidate | `f6acb...` pinned by diagnostic verifier branch | [Run #44330374509](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44330374509) | QUEUED at last inspection |
| Full cross-platform, SEC-05, distro, Android artifact/runtime | `f6acb...` pinned by diagnostic verifier branch | [Cross-platform #44330374474](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44330374474), [SEC-05 #44330374539](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44330374539), [Distro #44330374537](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44330374537), [Android artifact #44330374446](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44330374446), [Android runtime #44330374580](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44330374580) | QUEUED at last inspection |
| CircleCI Linux | `f6acb...` | GitHub status `ci/circleci: one-linux` PENDING; [job](https://app.circleci.com/workflow/a040bfd4-9db5-4fb4-9b33-4c1cda53e6c6/job/7931ea7a-5f7b-4d85-b8fa-ab26c7188273) | PENDING |

The independent verifier's diagnostic pin is branch-local to PR #44 on `one-windows-verification`; protected `verification` branch accepted pin remains unchanged. The current main repository PR #113 remains draft/unmerged. No evidence from `20ede9...` is transferred to `f6acb...`.


## Latest test-only correction — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| E4 atomic rename + Windows/Unix adversarial tests | `f6acb23a2a0038281c613fb6fd187bc04d2f3dc9` | [Run #44330374509](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44330374509), Windows and Unix jobs PASS | VERIFIED for this SHA only |
| Cross-platform Windows Clippy | `f6acb...` | [Run #44330374474](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44330374474), Windows Clippy failed on test-only `permissions_set_readonly_false`; later Windows tests were skipped | FAILED; test-only lint |
| Test cleanup correction | `a79665056149bf1ff302de79a7b86a346e0353d8` | Replaces readonly attribute reset with a real share-mode lock omitting `FILE_SHARE_DELETE` | IMPLEMENTED; NOT VERIFIED pending new exact-SHA runs |
| New E4 verification | `a796650...` diagnostic pin | [Run #44331243475](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44331243475) | QUEUED |
| CircleCI Linux | `a796650...` | GitHub status `ci/circleci: one-linux` PENDING; job https://app.circleci.com/workflow/ed36bb6b-0da1-4569-aa15-94835802af12/job/f6e4f563-1608-40d8-9842-1aea1c573585 | PENDING |

Do not transfer the E4 PASS on `f6acb...` to the changed test source `a796650...`. Keep the PR draft and issue open until exact-SHA verification and the remaining acceptance criteria are complete.


## Linux atomic write candidate evidence — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| Linux direct-child atomic write implementation | `cc54597b471493f4350d0dc0f3a2896babb33ea4` | `openat2` leaf validation, exclusive temp sibling under retained root fd, write + sync, dirfd-relative `renameat`; nested parent writes fail closed | IMPLEMENTED / PARTIAL; NOT VERIFIED |
| Linux direct-child write regression coverage | `cc54597...` | Five tests required by verifier: create/replace (relative + absolute), symlink rejection, oversized content, nested parent fail-closed, and outside-path rejection | Queued pending exact-SHA E4 |
| Linux mount-boundary read regression | `cc54597...` | Three `descriptor_relative_read_rejects_*` tests, including actual `/dev/shm` mount boundary with outside sentinel | Queued pending exact-SHA E4 |
| Windows atomic replacement | `f6acb23a2a0038281c613fb6fd187bc04d2f3dc9` | [E4 run #44330374509](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44330374509) PASS on that exact SHA | VERIFIED for prior SHA only; latest test cleanup changes not yet verified |
| Windows Clippy test cleanup fix | `a79665056149bf1ff302de79a7b86a346e0353d8` | Uses a Windows share-mode blocker instead of clearing readonly permission | Implemented; new exact-SHA runs pending |
| CircleCI Linux | `cc54597...` and docs descendants | Prior GitHub status for source `a796650...` was PENDING; a new exact-source status must be checked | NOT VERIFIED until terminal |

Nested Linux writes remain fail-closed because retained subdirectory handles can be moved outside the root. Do not claim Linux full filesystem-write support or Issue #104 acceptance from direct-child coverage.


## Linux writer compile failure and correction — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| Initial Linux writer | `cc54597b471493f4350d0dc0f3a2896babb33ea4` | Android artifact run [#44332315952](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332315952) failed at compile time with four E0425 unresolved `parent_handle` references; no artifact uploaded | FAILED / compile-only |
| Root-fd correction | `090c84dbab02e48af0a493faeb8d4fe0ff80a876` | Replaced stale `parent_handle.as_raw_fd()` references with retained `parent_fd` | IMPLEMENTED; NOT VERIFIED |
| E4 / cross-platform / Android revalidation | `090c84d...` diagnostic pin | E4 [#44332808624](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808624), cross-platform [#44332808543](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808543), Android artifact [#44332808717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808717) | QUEUED/PENDING at last inspection |
| CircleCI Linux | `090c84d...` | `ci/circleci: one-linux` PENDING; [job](https://app.circleci.com/workflow/60cfc357-04fc-47e2-8a5c-6cd85784c6af/job/a3279063-9cb6-4c54-8125-9b1b3b89ad8e) | PENDING |

The Android compile failure at `cc54597...` is retained as evidence and is not misclassified as a runtime or security-test failure. The corrected source must be tested on its own exact SHA.


## Android AArch64 artifact evidence — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| Android AArch64 device artifact build | `090c84dbab02e48af0a493faeb8d4fe0ff80a876` | [Run #44332808717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808717), ELF64 AArch64 inspected, Android linker `/system/bin/linker64`, artifact uploaded (ID 14438938784) | PASS / VERIFIED for artifact build only |
| Android API 35 runtime | `090c84d...` | [Run #44332808569](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808569) | QUEUED; runtime NOT VERIFIED |

Artifact build PASS does not imply runtime execution or full Android filesystem capability support.


## Strict Clippy correction and focused architecture diagram — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| Android AArch64 artifact build | `090c84dbab02e48af0a493faeb8d4fe0ff80a876` | [Run #44332808717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808717), ELF64 AArch64, Android linker, artifact uploaded | PASS for `090c84d...` only |
| Ubuntu Clippy | `090c84d...` | [Distro run #44332808528](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808528), failed `clippy::redundant_field_names` in Linux cleanup-guard initializer | FAILED / lint only |
| Strict Clippy correction | `3d8e0d0e3af54310837eb06d442d30a02df821d0` | Field initializer changed to shorthand; no production behavior change | IMPLEMENTED; NOT VERIFIED |
| Fresh E4 | `3d8e0d...` diagnostic pin | [Run #44333965599](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44333965599) | QUEUED |
| Focused architecture diagram | PR documentation HEAD | `docs/architecture/filesystem-root-confinement.md` Mermaid source linked from architecture and docs indexes | SOURCE PRESENT / rendered by GitHub Markdown; full PR #97 PNG still NOT VERIFIED |

Do not transfer Android artifact PASS on `090c84d...` to source `3d8e0d...`. Issue #104 remains OPEN / NOT ACCEPTED.


## Linux CLI integration expectation correction — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| Linux CLI direct-child write | `2fa1436daa102d3cf32125da6c440ae19cc83ba7` | Updated integration test expects absolute direct-child target to succeed | IMPLEMENTED; NOT VERIFIED |
| Linux CLI nested write | `2fa1436...` | Updated integration test expects `UnsupportedPlatform`, preserves original bytes, and checks no extra directory entry | IMPLEMENTED; NOT VERIFIED |
| Previous Debian workspace failure | `3d8e0d0...` | [Run #44333980452](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44333980452) failed only at stale CLI test expectation after earlier lint stage | FAILED / test expectation stale |
| Fresh exact-SHA cross-platform and distro | `2fa1436...` | [Cross-platform #44334598774](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44334598774), [Distro #44334598316](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44334598316) | QUEUED/PENDING |

This test-only correction does not broaden Linux writes beyond direct children of the retained root. Nested Linux writes remain fail-closed.


## CLI host-root integration evidence — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| Root derived from source/project context, not target | `79a98861080fed2ceaa0a7a2f56e07d6fbc19fca` | `project_root_for_source` selects nearest `one.toml` ancestor or host working/source context; authority construction no longer inspects target parent | IMPLEMENTED; NOT VERIFIED |
| Absolute write outside root rejected | `79a98861080fed2ceaa0a7a2f56e07d6fbc19fca` | CLI integration test asserts failure and preserves outside sentinel | IMPLEMENTED; NOT VERIFIED |
| Absolute read outside root rejected | `79a98861080fed2ceaa0a7a2f56e07d6fbc19fca` | CLI integration test asserts failure, no sentinel disclosure, outside file unchanged | IMPLEMENTED; NOT VERIFIED |
| Relative source root resolution | `79a98861080fed2ceaa0a7a2f56e07d6fbc19fca` | CLI integration test executes a relative source and writes to a relative direct-child target | IMPLEMENTED; NOT VERIFIED |

Do not claim project-root confinement until these host integration tests and the full filesystem E4 contract pass on the exact source SHA.


## Manifest-root CLI evidence — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| Nearest `one.toml` ancestor is selected as authority root | `1b7c3bb1dbc29a33c086b4df21a9996025c1e075` | CLI integration test places source under `src/main.one` and writes a direct-child file under project root | IMPLEMENTED; NOT VERIFIED |
| Outside-root absolute read/write denied | `79a9886...` and descendants | CLI integration tests preserve outside write sentinel and prevent outside read disclosure | IMPLEMENTED; latest SHA pending verification |

Verifier diagnostic pin now targets `1b7c3bb...`. Do not mark host-root integration VERIFIED until the exact-SHA cross-platform workflow passes.


## Ancestor manifest lookup correction — 2026-11-18

| Claim | Exact source SHA | Evidence | Classification |
|---|---|---|---|
| Nearest project manifest is searched above cwd | `722d662e7a81711ba3607d26571f4048a292b3ed` | Source resolver walks canonical source-parent ancestors without stopping at cwd | IMPLEMENTED; NOT VERIFIED |
| Exact-SHA verifier pin | `722d662...` | Diagnostic branch `CANONICAL_COMMIT.txt` updated | PINNED; workflows pending |


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


## Issue #104 protected-verifier reconciliation — 2026-11-19

This checkpoint supersedes earlier statements in this matrix that the protected verifier pin was still `e50748a...`, PR #44 remained unmerged, or protected E4 execution was still queued. Historical per-SHA entries remain historical; do not silently rewrite their original scope.

- Code-bearing source and protected canonical pin: `722d662e7a81711ba3607d26571f4048a292b3ed`.
- PR #44 was explicitly authorized and merged into protected branch `verification`; merge commit `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`.
- Post-merge protected E4 run #44519577693: SUCCESS. Jobs: Unix no-follow/resource bounds (135544438313), Windows reparse/handle checks (135544438225), Windows native `NtCreateFile + OBJ_DONT_REPARSE` contract probe (135544437734), all SUCCESS.
- Windows native probe is evidenced only on Windows Server 2025 build 26100. Linux nested writes remain fail-closed; unsupported platforms remain fail-closed; Android runtime evidence is API 35 ARM64 only.
- Same-push SEC-05 run #44519577571: Windows job SUCCESS; Linux job FAILED at `Install Linux native cross-compilers` (step 5). Exact failing command/log not yet retrieved; root cause UNKNOWN. Separate E1, stdlib/database, package provenance, health scanner, and candidate-artifact workflows also failed; these do not negate the E4 job result but remain distinct verification failures.
- PR #113 remains OPEN / DRAFT / UNMERGED; Issue #104 remains OPEN / NOT ACCEPTED pending remaining acceptance/ledger gates. Latest PR #113 HEAD at this checkpoint: `67d3c14c70fc43c1eae26c7f21fea8e7a4918b07`; CircleCI status on that exact SHA was PENDING.


## Protected push failure classification — 2026-11-19

- SEC-05 Linux run #44519577571 / job `135544437757`: apt update succeeded, but installation of the 277 MB cross-toolchain package set failed after 194 MB downloaded because Azure Ubuntu mirror backend `azure.archive.ubuntu.com` (`52.252.75.106:80`) became unreachable. APT exit 100. This is an infrastructure/package retrieval failure before tests, not a source regression. SEC-05 Windows passed.
- E1 Windows run #44519577311 / job `135544437394`, Milestone 1 E1 Linux run #44519577315 / job `135544437616`, and focused stdlib run #44519577640 / job `135544438572` all failed before intended integration tests: Cargo reports no test target `stdlib_builtins` in `one-core`. Source tree `722d662...` has no corresponding integration test file; do not treat these as assertion failures or passes.
- The intended E1 test inventory must be mapped to real source tests or implemented explicitly before updating workflow commands. Do not blindly substitute an unrelated test target.
- These independent workflow failures do not change protected E4 run #44519577693, whose three jobs all passed. Full repository push remains NOT GREEN; Issue #104 remains OPEN / NOT ACCEPTED.


## E4 log-level execution audit — 2026-11-19

Protected run #44519577693 was checked beyond its green summary:

- Unix workflow helper enforces exact expected passing-line counts (3 read rejection, 5 write, 1 walk, 1 parent-symlink race, 1 mount-boundary no-truncate). All 11 required named tests appear as passing in the Unix job log.
- Windows helper requires each named test to appear, exactly one test to run, and exactly one pass. All seven required filesystem tests appear as passing in the Windows job log.
- Native probe log confirms ordinary files open successfully while direct and intermediate junction traversal with OBJ_DONT_REPARSE returns STATUS_REPARSE_POINT_ENCOUNTERED (0xC000050B); success sentinel is present.
- Cargo emits zero-test summaries for unrelated binaries while a filter is active; these are expected and are not false-positive passes because the workflow helper separately validates the exact matched test count.
- E4 protected verification remains CONFIRMED for the source SHA and stated OS environments. This does not make the separate SEC-05/E1/stdlib failures green.


## Live evidence reconciliation — 2026-11-19 16:14Z

- Protected source pin after merged verifier PR #44: `722d662e7a81711ba3607d26571f4048a292b3ed`; protected E4 run #44519577693 SUCCESS across three jobs.
- Draft verifier PR branch evidence: #45 run #44538012071 SUCCESS; #46 run #44533999931 SUCCESS Linux + Windows; #47 run #44535174043 SUCCESS Windows native runtime; #48 run #44538540682 SUCCESS Ubuntu + Windows. All remain OPEN / DRAFT / UNMERGED and do not satisfy protected-branch evidence rows until integrated and rerun.
- Protected fixed-weight gate #44519577312 FAILED after 10,200s waiting for terminal exact-SHA matrix; five protected workflows were reported failed (SEC-05, artifact, E1 Windows, focused stdlib/database, E1 cross-platform). The individual candidate fixes are validated only on their respective draft PR branches.
- PR #113 exact HEAD `519f0f6856e0ae6242746581b9579cce33fca8cd` has CircleCI `ci/circleci: one-linux` SUCCESS. This is not a verifier release-gate PASS.
- Issue #104 remains OPEN / NOT ACCEPTED; do not promote E4 or release acceptance from scoped evidence.
