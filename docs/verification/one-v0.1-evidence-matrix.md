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
