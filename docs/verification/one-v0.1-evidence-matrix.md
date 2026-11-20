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


## Fixed-weight gate log detail — 2026-11-19

The complete GitHub Actions job log for run #44519577312 / job 135544436703 has now been retrieved. The failure is not an E4 regression. The verifier reported:

- Exact candidate source SHA: 722d662e7a81711ba3607d26571f4048a292b3ed.
- Fixed weights: 100 total; implementation 92/100; verified 0/100; 18 ledger rows checked.
- Definition of Done: 10 unchecked acceptance items.
- The independent evidence index did not contain the exact candidate SHA.
- Every non-zero fixed-weight ledger row was still recorded as 0 verified points (F1–F6, C1–C5, E1–E6); no row acceptance points were promoted.
- Exact-SHA matrix wait timed out after 10,200 seconds. The protected run list had failed SEC-05 #44519577571, artifact #44519577533, E1 Windows #44519577311, focused stdlib/database #44519577640, and E1 cross-platform #44519577315; no exact-source protected Windows Native Backend Runtime run was found.
- No protected ONE Package Provenance Release run existed for the exact source SHA, so E2 could not be accepted from candidate bundles.

This is the actual logged gate output, not a hypothesis. PR #45–#48 each validate a narrow fix on its own draft branch, but none is merged into protected verification; their successes cannot retroactively satisfy the failed protected run. Do not edit fixed weights, mark rows VERIFIED, check DoD items, or dispatch protected release merely to make the gate green. Issue #104 remains OPEN / NOT ACCEPTED and the overall release gate remains NOT PASSED.

## Current PR #113 status after documentation sync

- PR #113 HEAD: 3ebb877c5e2f5b97e2154a7b4643b478af940879, OPEN / DRAFT / UNMERGED.
- CircleCI ci/circleci: one-linux on this exact SHA: PENDING at 2026-11-19T16:21:13Z; workflow abe252f3-c181-441c-a5f8-732d3bf9b785; job b233706c-6421-491b-8730-603443e4b196. Parent SHA 519f0f6856e0ae6242746581b9579cce33fca8cd had SUCCESS, which does not transfer.
- ONE main remains d35db95153f6ddd79885494c72f5aee16c899751. No merge or protected-branch change was made.

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

## Release sprint exact-SHA refresh — 2026-11-20

- PR #113 HEAD `a0d94bc9a26cf9effd4b202f29af50f81c8e4ee5` maps to CircleCI pipeline #1199, workflow `916983e9-dbde-4db2-a967-438a666ebb19`, job #834 / `a130c5d9-c078-4c18-8ae5-571470f46d60`. Pipeline state: `created`; workflow/job: failed; job `started_at=null`; available step #101: CANCELED / `Task information unavailable`, stdout/stderr empty.
- Prior docs HEAD `5daf3337b4bcc93fa3ee6072f9633915d5fea85b` maps to pipeline #1198, workflow `5ed6d9e7-976e-4322-b7ba-6deea915e1a9`, job #833 / `8d2d756e-9297-4a00-9a7d-a0c9bd734cba`; `started_at=null` and the same no-output canceled-step signature.
- Classification for both: **pre-execution termination / ROOT CAUSE UNKNOWN**. Do not label as timeout, test failure, compiler failure, or source regression without provider evidence.
- Local Termux diagnostic script blob equals remote PR #113 blob `751c25e0b4a4227638c898043ce175d62d523834`. Local checkout HEAD `70ae27cd9dab4429c528d6f01a42377f474c725c` is 76 commits behind; do not destructively synchronize it.
- CircleCI SUCCESS on code source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` remains valid only for that SHA (workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`); provider step logs were not inspected. It does not transfer to docs HEAD `a0d94bc...`.
- Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Existing verifier results remain attributed to their recorded source and verifier SHAs.
- PR #113 remains OPEN / DRAFT / UNMERGED; ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`. Verifier PR #51 remains OPEN / DRAFT / UNMERGED at `6c0979e62eb01e4b929c48b4224a3d4f742bc8af`; protected verifier pin `722d662...` unchanged.
- See [ONE_RELEASE_BLOCKER_REGISTER.md](ONE_RELEASE_BLOCKER_REGISTER.md) for actions and [MASTER_CONTINUATION_PROMPT.md](MASTER_CONTINUATION_PROMPT.md) for session startup.


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

## Implementation-first candidate delta — 2026-11-20

- Current PR #113 HEAD at this checkpoint: `748d2b79eb5f62440f6f37007a2150926b04985e` (OPEN / DRAFT / UNMERGED; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`).
- Implementation change: CLI filesystem authority is now derived solely from the host process working directory, not the `.one` source path or program-controlled `--target`.
- Linux root acquisition opens the supplied directory with `O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC`, then compares canonical-path device/inode identity to the retained root handle and fails closed on mismatch.
- Regression fixture updated to run a source file outside the host working directory while reading a file inside that working directory.
- Source commits: `8b1d13e02c181fc06b97fb7f147e1ccd013d3aa1` and `7cbcb1bf575defe03a2d9991dc5094569de95f03`; fixture commit: `5e39430a341629eb9fda11ee4e11b0f6f40487ab`.
- No build, test, or verifier run has been performed after these changes. All prior exact-SHA evidence remains historical and does not transfer to this candidate. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

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
