# ONE Completion Ledger — Human Instruction vNext Roadmap

**Ledger version:** 1  
**Created:** 2026-10-22  
**Branch:** `spec/human-instruction-grammar-vnext-20261022`  
**Scope:** Human-instruction vNext foundation, its first runnable reference application, and subsequent language/runtime ecosystem work.  
**Rule:** Weights are fixed in this ledger. Do not change them to make a milestone pass. Add a versioned ledger change only after explicit review of the roadmap.

## Percentage method

The cumulative project scale is divided into three fixed tranches:

- **Milestone 30% — Foundation (30 points):** formal starter grammar, parser/semantic validation, isolated official CLI entry point, first runnable application with persistence, and evidence/docs.
- **Milestone 60% — Core language/runtime (30 additional points):** canonical AST/IR integration, general frontend/runtime path, expanded semantics, diagnostics, reusable persistence APIs, and additional real programs.
- **Milestone 100% — Ecosystem and release quality (40 additional points):** complete required standard library/tooling, conformance and compatibility suite, cross-platform verification, security/reliability hardening, release documentation and final forensic audit.

For each row, **implemented points** are provisional progress based on code actually present; **verified points** are earned only when the row's acceptance evidence passes on a recorded SHA. Implemented-but-unverified work does not count toward a PASS gate. The public completion percentage is the sum of verified points out of 100. The implementation subtotal is reported separately so unverified work is visible but cannot masquerade as completion.

## Fixed weighted ledger

| ID | Requirement / weight | Acceptance evidence | Current implementation points | Current verified points | Status |
|---|---|---|---:|---:|---|
| F1 | Versioned grammar contract, lexical/name rules, block and newline rules — **8** | Reviewed formal grammar; Unicode, identifier, ambiguity and versioning rules; positive/negative fixtures | 8/8 | 8/8 | VERIFIED |
| F2 | Deterministic parser and semantic rejection — **7** | Exact text-block tests, CRLF, blank lines, indentation, Unicode, duplicate names, unresolved references, stable diagnostics | 7/7 | 7/7 | VERIFIED |
| F3 | Explicit CLI integration isolated from v0.1 — **5** | CLI command invokes vNext parser and supported runtime; v0.1 regression tests pass | 5/5 | 5/5 | VERIFIED |
| F4 | Task-list app actions — **5** | Source example supports add/list/complete with input validation through public CLI | 5/5 | 5/5 | VERIFIED |
| F5 | Durable task persistence — **3** | Data survives a new process; corrupt storage and write failures are reported | 3/3 | 3/3 | VERIFIED |
| F6 | Beginner docs, example, exact-SHA build/CI evidence and checkpoint — **2** | Reproducible commands, docs agree with implementation, build/test/Clippy/CI green at exact SHA | 2/2 | 2/2 | VERIFIED |
| C1 | Canonical vNext AST/IR integration — **8** | Human frontend lowers into canonical IR with golden semantic tests | 8/8 | 8/8 | VERIFIED — PASS GREEN LOCKED |
| C2 | General frontend-to-runtime execution pipeline — **8** | More than the task-list special-case; ordinary programs run through canonical compiler/interpreter | 8/8 | 8/8 | VERIFIED — PASS GREEN LOCKED |
| C3 | Expanded semantic/type/scope model and diagnostics — **6** | Explicit type/scope rules, list/map operations, source-located diagnostics and positive/negative fixtures | 6/6 | 6/6 | VERIFIED — PASS GREEN LOCKED |
| C4 | Reusable persistence abstraction and recovery policy — **4** | Versioned storage contract, corruption handling, restart, rollback and bounded-size replacement tests | 4/4 | 4/4 | VERIFIED — PASS GREEN LOCKED |
| C5 | Additional real application vertical slices — **4** | Calculator, score tracker, module and mapping examples executed through the same compiler/runtime | 4/4 | 4/4 | VERIFIED — PASS GREEN LOCKED |
| C6 | Milestone 60 evidence and checkpoint — **0** | Documentation-only reporting; earns no points by itself | 0/0 | 0/0 | NOT APPLICABLE |
| E1 | Required standard library and reusable APIs — **10** | Scope-defined API contracts and executable tests | 7/10 | 0/10 | IMPLEMENTED — PARTIAL; UTC parser integrated; full stdlib acceptance pending |
| E2 | Packaging, dependency and project/build tooling for vNext — **8** | Deterministic artifacts, version metadata, supported project workflow | 5/8 | 0/8 | IMPLEMENTED — PARTIAL; EXTERNAL DEPENDENCY RESOLUTION PENDING |
| E3 | Conformance, compatibility and cross-platform suite — **8** | Exact-SHA Linux/Windows plus other declared targets where applicable | 8/8 | 0/8 | IMPLEMENTED — PARTIAL; Linux/Windows pass, Android ELF build/run test remains ignored |
| E4 | Security, resource bounds, diagnostics and failure recovery — **6** | Threat review and tests for unsafe/invalid inputs and runtime failures | 3/6 | 0/6 | IMPLEMENTED — PARTIAL; SQLite budgets added; full threat review pending |
| E5 | Documentation, examples and developer tooling — **4** | Reviewed reference docs and tested user workflows | 1/4 | 0/4 | IMPLEMENTED — PARTIAL; FULL DOCS/TOOLING REVIEW PENDING |
| E6 | Final forensic audit, release evidence and checkpoint — **4** | Definition-of-Done audit at release SHA; no unresolved required gates | 0/4 | 0/4 | NOT IMPLEMENTED |
|  | **TOTAL** |  | **84/100** | **60/100** | **MILESTONE 100% IN PROGRESS — GATE NOT PASSED** |

## Baseline and evidence

- This ledger is the first weighted completion ledger located for the vNext workstream; no prior canonical weighted formula was found during repository discovery.
- Current source branch: `milestone/100-ecosystem-20261023`.
- Historical code-changing SHA at the initial ledger checkpoint: `26199c0114ea0da5f9784ea4d000ab185799e4fc`. The latest continuation evidence and tested source SHA are recorded in the 2026-10-24 section below.
- Historical Foundation checkpoint update commit: `d936b96283162eea3eff7b4bccec7c6abbcb5f2b`; later documentation-only commits may advance the branch head without changing the tested source snapshot.
- Current code includes the experimental parser, the dedicated task-list runner, an explicit CLI command, a runnable example, and regression tests. These changes are not credited as verified until they execute successfully.
- The GitHub Actions runs inspected for earlier commits failed before any steps started; the jobs reported an empty step list and logs were unavailable. This is an infrastructure/runner blocker, not evidence of a Rust source failure or a pass.
- Exact-code-SHA Actions run: [39752818112](https://github.com/panglimanetizen01/one/actions/runs/39752818112), triggered for `8c4f7c4706365972797446382989b81335434d92`; both OS jobs failed before any steps started. No build/test result was produced.

## Milestone gate definitions

### 30% — Foundation

PASS only when F1–F6 are fully implemented, relevant checks pass on the exact SHA, v0.1 behavior remains compatible, and all six rows earn their full **30 verified points**. An open PR or committed source alone is insufficient.

### 60% — Core language/runtime

PASS only when all F1–F6 and C1–C5 are verified (60 points total) and the checkpoint is updated.

### 100% — Ecosystem and release

PASS only when every weighted row earns its full verified points, all required CI/release gates pass on the recorded release SHA, and the final forensic audit finds no outstanding mandatory Definition-of-Done requirement.

## Milestone 60 — exact-SHA cross-platform verification (2026-10-23)

- **Canonical source branch:** `spec/human-instruction-grammar-vnext-20261022`
- **PR:** [#78](https://github.com/panglimanetizen01/one/pull/78), open and unmerged.
- **Code-bearing source SHA:** `3027e43b9e2481d73bdb67dbbbbb2e27c3419639`
- **Independent verification run:** [39936123376](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39936123376)
- **Result:** `ubuntu-latest` **SUCCESS** and `windows-2025` **SUCCESS** on the exact selected source SHA. Contract artifacts, build, AST contract regression tests, strict Clippy, workspace tests, real-network runtime tests, and platform-appropriate offline verification all completed successfully.
- Workspace test summaries include 160 core tests passed on Linux and 156 on Windows; integration suites passed. One Android AArch64 native execution test is explicitly **ignored** on each platform because the hosted environment lacks the required Android execution-capable Clang setup. That test is not counted as passed and is outside the milestone-60 required gate.
- Canonical repository Actions for the same SHA ([run 39936118585](https://github.com/panglimanetizen01/one/actions/runs/39936118585)) reported failure with `runner_id=0` and zero steps on both OS jobs. The repository's [exact-SHA verification contract](human-vnext-exact-sha-verification-contract.md) authorizes the independent mirror fallback; the canonical run is not misreported as green.
- **Implementation subtotal:** 60/100. **Verified completion:** 60/100. F1–F6 contribute 30 verified points and C1–C5 contribute the next 30, using unchanged ledger weights.
- Integrated scope: explicit opt-in human-core parser/CLI through canonical parse → semantic analysis → IR → interpreter; source-located semantic diagnostics; functions, control flow, lists and maps; relative multi-file modules with duplicate/cycle/path traversal rejection; reusable bounded `ONE-KV-1` persistence; and executable calculator, score-tracker, module, and mapping examples.
- Compatibility: experimental mapping grammar is gated behind `parse_human_core` and the explicit human-core CLI; default v0.1 parsing does not accept the new mapping actions. The existing task-list app's persistence format was not silently migrated.
- **Gate:** **PASS GREEN LOCKED — milestone 60% only.** No claim is made that milestone 100% or the full ONE ecosystem is complete.

## Current result

- **Security hardening added after the previous checkpoint:** HTTP responses are read with a 16 MiB body limit and a 64 KiB header allowance; HTTP headers are limited to 256 fields, duplicate Content-Length and Content-Length/Transfer-Encoding conflicts are rejected, and unsupported Transfer-Encoding is rejected. Filesystem adapters limit reads/writes to 16 MiB and reject parent-directory components and existing symlink path components. These changes are defensive bounds, not a complete sandbox; race-resistant path resolution and the broader threat review remain open.
- **SHA-256 module lock v2:** module content digests now use the sha2 crate and known-vector tests. This improves content mismatch detection against a trusted lock, but does not provide signed package identity or authenticated provenance. E2 remains partial.


- **Implementation progress:** 84/100 provisional points (60 core/Foundation points plus 7 provisional E1, 5 provisional E2, 8 provisional E3, 3 provisional E4, and 1 provisional E5 implementation points).
- **Verified completion:** 60/100 points. E3's 8 implementation points are not yet verified: the Linux/Windows matrix passes, but the declared Android AArch64 ELF build-and-execute test remains ignored because Android AArch64 Clang/runtime setup is unavailable. The prior 68/100 verified subtotal overstated E3 acceptance and is superseded by this forensic correction.
- **Milestone target:** 100%.
- **Gate:** **NOT PASSED** for milestone 100%. The prior 60% gate remains scope-limited to code SHA `3027e43b9e2481d73bdb67dbbbbb2e27c3419639`.
- **E1 partial work:** public deterministic text/list/map/integer/JSON helper APIs, UTC formatting and strict UTC parsing are integrated through source-level built-ins. Filesystem read/write adapters enforce 16 MiB bounds and reject parent-directory and existing symlink components; HTTP GET uses bounded response reads and strict response framing. JSON floating-point/exponent parsing remains unsupported because ONE has no float value type. Broader time/date, concurrency, logging/configuration, external dependency resolution, and complete persistence integration remain incomplete. E1 verified points remain 0/10.
- **E2 partial work:** `one build`, `one test`, and `one package` process human-core projects through the canonical compiler path; package emits a deterministic versioned artifact and module lock v2 with SHA-256 content digests. Build/test reject stale locks with `ETOOL006`. The lock is not signed and does not authenticate publisher identity or provenance. External dependency declarations/registry resolution, signed package identity, authenticated provenance, target-specific artifacts, and full provenance remain incomplete; E2 verified points remain 0/8.
- **E3 implementation; verification incomplete:** valid/invalid human-core conformance fixtures are dispatched through the matching compiler path. Exact-SHA Linux/Windows workflows pass, but the declared Android AArch64 ELF build-and-execute test `build_native_menghasilkan_dan_menjalankan_elf_android_aarch64` is marked ignored on both platforms. The Android-target CLI test only verifies artifact creation; it does not execute the Android ELF. Therefore E3 remains 8/8 implemented but 0/8 verified until the Android execution acceptance is satisfied or the contract explicitly excludes it.
- **E4 partial work:** checked integer arithmetic, recursion-depth bounds, bounded JSON parsing/serialization, HTTP response/body/header/count limits, strict HTTP framing, filesystem read/write bounds, and SQLite read-only/query-only enforcement with SQL/parameter budgets are implemented with regression tests. FTP/SMTP protocol line reads retry `Interrupted` system calls. Filesystem adapters still lack race-resistant path resolution; general memory/open-resource quotas, comprehensive threat review, and broader recovery remain incomplete. E4 verified points remain 0/6.
- **E5 partial work:** standard-library/runtime/package contracts, runnable examples, and CLI integration tests exist. Complete tutorials, API reference coverage, debugging workflow, and end-to-end developer workflow review remain incomplete; E5 verified points remain 0/4.
- **E6:** final forensic audit and release evidence have not been completed; 0/4.
- **Latest exact-SHA verification:** source snapshot `ebf254d1194a903a3b582f3a18abfc402f2003d2` passed [focused run 40009389100](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389100), [Linux distro run 40009389164](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389164), and [Ubuntu/Windows cross-platform run 40009389190](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389190). The cross-platform run checked out the exact SHA, passed build/Clippy/workspace/real-network/offline steps, had no `test result: FAILED` markers, and logged one `OFFLINE VERIFICATION: PASS` per platform. The separate Android AArch64 ELF build-and-execute test remains ignored; it is not counted as passed. Canonical repository Actions on this SHA still fail before steps (`runner_id=0`, empty step lists), so the mirror supplies the actual test evidence.
- **Historical verification:** mirror run [39958882035](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39958882035) passed its Linux/Windows matrix on source SHA `d9d696fa87683027b78fdcbbec504696904055aa`; it predates the current security and SHA-256 lock changes. Run [39963120976](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39963120976) targets the earlier source SHA `3c0febb47409f0af6c43c6a46996c784bcd8a0a9`, before the SHA-256 module-lock implementation. Exact-SHA mirror run [39963778066](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39963778066) checked out `d74eda614be6688164dbd294fa07bcdf9c204ecf`. Ubuntu build, AST contract tests, and strict Clippy passed, but workspace tests failed: `native_backend::linker_discovery_tests::linux_x86_64_linker_override_diprioritaskan` tried to execute shell scripts in the temporary directory, which is mounted `noexec` on this runner (181 passed, 1 failed). The test fixture was corrected in commit `8ee1b34dd0bba1955840581c5932135ae32b76a9` to symlink `/bin/true` and `/bin/false`, preserving the override-priority assertion without depending on temp-directory execution. The follow-up exact-SHA mirror run [39964166884](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39964166884) completed successfully on canonical source SHA `985bce88498daa0bbb6566226353251fe6e3c4ac`: Linux and Windows build, AST contract regression tests, strict Clippy, workspace tests, real-network tests, and their platform-specific offline verification all passed. This verifies the security/package code at that SHA. The subsequent exact-SHA mirror run [39964952853](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39964952853) checked out documentation-reconciled SHA `e6ed974bcb19f6cf659508b9ac905bcfe8f8fed1` and again failed the same linker override test because the expected executable was still a symlink located inside the noexec temporary directory. The follow-up fix in commit `ab09f8c6a8dc5bf16d39f3fc8f1df15248d77749` moved the test executable outside temp, but run [39965400077](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39965400077) still failed because the Rust test executable did not exit successfully for the probe's --version argument. Commit `e14b63ba0e40e45eb8a0c0f79a78886d0a83bdf1` now uses `/bin/true` directly as the Linux override and `/bin/false` as the PATH candidate, with the test scoped to Linux; neither executable is run from the noexec temp path. A fresh exact-SHA Linux/Windows matrix is required on the updated branch head. No additional verified milestone points are awarded because the full E1–E6 acceptance criteria remain incomplete.
- **Not claimed complete:** ONE is not 100% complete. No ledger weights have changed.

## Repository work checkpoint — 2026-10-22

Changes committed directly to the feature branch after the baseline above:

- `c31258c98932293203507f0566c258d519632668` — added an implementation-bounded human-vNext starter grammar contract.
- `c73950e16acb1df32be32b4f52898fe001ca878c` — added parser tests for Unicode identifiers, invalid indentation, duplicate data/action names, and orphaned nested instructions.
- `212a48ac37522d16c76cb525f043aed6adf26db3` — aligned the text-block contract with observed parser behavior (empty text is accepted).
- `e364af0b5768628f40e3f3de5c52fd3f0833607d` — linked the experimental contract from the specification index.
- `48afad1a35f4911e3a415de3fdfc1463091a8aaf` — made the contract checker require the new grammar document.

- `f7ca5c5d35a59b6928de20fe722e308a7f11fb19` — applied the four strict-Clippy fixes to the GitHub source.
- `7f02a4563024cb60e37d95bf3e2639ddffa8a4f3` — added runtime tests for blank-title rejection, invalid/unknown task IDs, and a storage write failure.

## Exact-SHA local verification — 2026-10-23

Verified in Debian 12/Termux against the clean GitHub worktree at commit `217815f1e16771b730d69f752c64018fd2faa250` (branch `spec/human-instruction-grammar-vnext-20261022`):

- Contract checker: PASS — 61 required artifacts, 73 indexed artifacts, 6 valid fixtures, 6 invalid fixtures, 3 golden fixtures.
- `git diff --check`: PASS.
- `cargo build --workspace --all-targets`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `ONE_ANDROID_CLANG=/tmp/one-android-clang-wrapper cargo test --workspace --all-targets`: PASS for every executed test suite, including 135 one-core unit tests and 16 CLI tests. One Android AArch64 native execution test was ignored because the required Android AArch64 Clang environment was unavailable.
- Human-vNext regression coverage includes Unicode names/text, indentation and text-block boundaries, declaration binding, duplicate names/persistence, restart persistence, blank-title rejection, invalid/unknown task IDs, corrupt storage, and a parent-path write failure.
- GitHub Actions run [39814473056](https://github.com/panglimanetizen01/one/actions/runs/39814473056) still failed before steps began on both Ubuntu and Windows; no hosted build/test evidence was produced.

**Evidence boundary:** local verification is tied to the exact tested commit above. It does not turn the hosted CI failure into a pass. Verified-point assignments remain unchanged pending a reviewed replacement verification contract and completion of every F1–F6 acceptance criterion. The 30% gate remains NOT PASSED.


## Exact-SHA local verification — 2026-10-23 (fixture checkpoint)

Verified in Debian 12 from the fetched GitHub branch at exact commit `742d30c506eabde0eb9c1ccb258eeb2b2b9d9b36`:

- `python3 tools/check_contract.py`: PASS — 68 required artifacts, 73 indexed concrete artifacts, 6 existing valid fixtures, 6 existing invalid fixtures, 3 golden fixtures, 3 human-vNext valid fixtures, and 4 human-vNext invalid fixtures.
- `git diff --check`: PASS.
- `cargo build --workspace --all-targets`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `ONE_ANDROID_CLANG=/tmp/one-android-clang-wrapper cargo test --workspace --all-targets`: PASS for all executed suites. CLI: 16 passed; one-core unit tests: 137 passed; conformance: 5 passed; remaining displayed suites passed. One Android AArch64 native execution test was ignored because the Android AArch64 Clang verification environment was unavailable.
- New human-vNext fixture tests passed for all 3 valid and 4 invalid fixtures.
- GitHub Actions has not been shown passing for this SHA; this evidence is local Debian verification only.

**Evidence boundary:** this confirms the fixture checkpoint's local contract/build/Clippy/test checks, not hosted CI. It does not alone satisfy F1–F6. Verified points and the 30% gate remain unchanged until every acceptance criterion and the reviewed CI/replacement-verification contract are satisfied.

## Exact-SHA local verification — 2026-10-23 (expanded fixture checkpoint)

Verified in Debian 12/Termux from a clean worktree fetched from GitHub at exact commit `d76360b661a16caceb840d2a1eba8c7beb60882f` on branch `spec/human-instruction-grammar-vnext-20261022`:

- `python3 tools/check_contract.py`: PASS — 75 required artifacts, 73 indexed concrete artifacts, 6 existing valid fixtures, 6 existing invalid fixtures, 3 golden fixtures, 4 human-vNext valid fixtures, and 10 human-vNext invalid fixtures.
- `git diff --check`: PASS.
- `cargo build --workspace --all-targets`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `ONE_ANDROID_CLANG=/tmp/one-android-clang-wrapper cargo test --workspace --all-targets`: all executed test suites passed. CLI: 16 passed; one-core unit tests: 137 passed; conformance: 5 passed; other displayed suites passed. One Android AArch64 native execution test was ignored because the Android AArch64 Clang verification environment was unavailable.
- The expanded human-vNext fixture tests passed for all 4 valid and 10 invalid fixtures.
- GitHub Actions is not confirmed passing at this SHA. This is local verification only; earlier Actions runs failed before steps began.

**Evidence boundary:** this checkpoint provides exact-SHA local contract/build/Clippy/test evidence, but does not by itself satisfy all F1–F6 acceptance criteria or substitute for hosted CI. Verified points remain unchanged pending the remaining acceptance work and an explicitly reviewed CI/replacement-verification contract. The 30% gate remains NOT PASSED.

## Exact-SHA local verification — 2026-10-23 (grammar clarification checkpoint)

Verified by the user in Debian 12/Termux from the clean GitHub branch worktree at code commit `89b6ace035450511496764510867d3bd82cc109d`:

- `python3 tools/check_contract.py`: PASS — 75 required artifacts, 73 indexed concrete artifacts, 6 existing valid fixtures, 6 existing invalid fixtures, 3 golden fixtures, 4 human-vNext valid fixtures, and 10 human-vNext invalid fixtures.
- `git diff --check`: PASS.
- `cargo build --workspace --all-targets`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `ONE_ANDROID_CLANG=/tmp/one-android-clang-wrapper cargo test --workspace --all-targets`: all executed suites passed, including 16 CLI tests, 137 one-core unit tests, 5 conformance tests, and the remaining displayed integration suites. One Android AArch64 native execution test was ignored because the Android AArch64 Clang environment was unavailable.
- Human-vNext fixture suite passed for all 4 valid and 10 invalid fixtures.

**Evidence boundary:** this is user-run local verification, not hosted GitHub Actions evidence. The prior Actions failures before any steps started remain unresolved. No verified points are assigned by this entry; F1–F6 and the 30% gate remain NOT PASSED until the replacement verification contract is explicitly reviewed and each acceptance criterion is satisfied.



## Exact-SHA cross-platform verification — 2026-10-23 (30% Foundation gate)

Verified canonical source SHA `26199c0114ea0da5f9784ea4d000ab185799e4fc` on branch `spec/human-instruction-grammar-vnext-20261022` through the independent free verification mirror, pinned by `CANONICAL_COMMIT.txt` in commit `d03eeecbae523237c66aa553aebfcda68036df5b`:

- Workflow: [One Cross-Platform Verification run 39929244544](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39929244544) — **overall SUCCESS**.
- `ubuntu-latest`: **SUCCESS** — contract artifacts, build, AST contract regression tests, strict Clippy, all workspace tests, real-network runtime tests, and offline verification all passed.
- `windows-2025`: **SUCCESS** — contract artifacts, build, AST contract regression tests, strict Clippy, all workspace tests, real-network runtime tests, and Windows offline verification all passed.
- Both jobs checked out and printed the same canonical source SHA `26199c0114ea0da5f9784ea4d000ab185799e4fc`.
- Human-vNext fixture inventory at this SHA: 6 valid and 18 invalid fixtures; the contract checker passed (91 required artifacts and 77 indexed concrete artifacts).
- CLI tests: 22 passed on both platforms. Core tests: 153 passed on Ubuntu and 149 passed on Windows. All other displayed test suites passed.
- One Android AArch64 native execution test remained explicitly ignored on both platforms because the test requires an Android execution-capable verification environment. It is not claimed as passed; this does not block the scoped Foundation milestone.
- The canonical ONE repository's own Actions still is not claimed as passing; the independent mirror is the documented cross-platform replacement, and its Linux/Windows matrix actually executed successfully on the exact source SHA.

### F1–F6 acceptance review at the verified SHA

- **F1 — 8/8:** reviewed versioned starter grammar, Unicode/name and binding rules, structural indentation and newline/text-block rules, explicit compatibility/versioning boundaries, and required positive/negative fixtures.
- **F2 — 7/7:** deterministic parser and semantic rejection tests cover opaque text-block fidelity, LF/CRLF, blank lines, Unicode, canonical indentation, duplicate data/action and persistence names, unresolved references, and stable diagnostics.
- **F3 — 5/5:** explicit `jalankan-human-vnext` CLI path; invalid/missing argument diagnostics and unsupported task-app contracts are tested; default v0.1 does not auto-detect human-vNext and regression tests pass.
- **F4 — 5/5:** the documented runnable task-list example adds, lists, and completes tasks through the public CLI with title and task-ID validation.
- **F5 — 3/3:** task data survives a new process; corrupt records, write/replace failures, temporary-file cleanup, invalid titles/IDs, and atomic replacement behavior have tests.
- **F6 — 2/2:** beginner documentation, runnable example, contract checker, exact-SHA verifier, reviewed fallback contract, and successful exact-SHA Linux/Windows hosted evidence are recorded above.

**Gate decision:** award 30 Foundation points plus 30 Core/Runtime points. The 60% target is reached at the exact code SHA recorded above without changing weights or counting the ignored Android-native test as passed. E1–E6 remain unimplemented and uncredited.


## 2026-10-24 continuation — exact-SHA hardening and verification

**Tested source SHA:** `64985b51599c1531b6b8e07aa0aa9d16551e6e38`  
**Latest code-changing commit in this slice:** `cec949631f3c91a5419957b1a8820904daa6bcd7`; the tested SHA above adds verification-contract documentation after that code commit.  
**Branch / PR:** `milestone/100-ecosystem-20261023` / [PR #80](https://github.com/panglimanetizen01/one/pull/80), still open and draft; no merge performed.

### Implemented and integrated

- SQLite `database.query` now opens file-backed databases with `SQLITE_OPEN_READ_ONLY` and enables `PRAGMA query_only`. The `:memory:` resource remains an isolated ephemeral connection. A regression test verifies a read query against a missing file returns a backend error and does not create that file.
- SQLite query resource limits are explicit and enforced before prepare/bind work: SQL statement 1 MiB, each text parameter 1 MiB, aggregate parameter bytes 16 MiB, parameter count 256, result rows 100,000, serialized output 16 MiB. Regression tests cover SQL length, individual parameter length, aggregate parameter bytes, and parameter count.
- Added `utc_text_to_unix_seconds` and the source-level builtin `unix dari waktu utc`, integrated through semantic type/arity validation and the canonical interpreter. The parser accepts only Gregorian UTC `YYYY-MM-DDTHH:MM:SSZ` for years 0000–9999 and rejects invalid dates/times, offsets, fractional seconds, leap seconds, and non-canonical forms. Unit and parse→semantic→IR→interpreter tests cover valid boundaries, leap dates, pre-epoch values, invalid types and malformed dates.
- Updated the standard-library contract, standard-library boundary document, user guide, and focused verification workflow to document and exercise the new behavior.
- Fixed a false-green in `tools/verify_offline.ps1`: it now enables `$PSNativeCommandUseErrorActionPreference = $true` so failed native commands terminate the script, and skips only `android_target_explicit_mempertahankan_build_native` on Windows. The Android-target CLI build test passed on Ubuntu but only checks artifact creation; the separate Android AArch64 ELF build-and-execute test remains ignored on both platforms. No Android runtime pass is claimed.

### Exact-SHA evidence

All three independent verification workflows checked out the exact source SHA above from the canonical repository:

- Focused standard-library/database regressions: [run 40006470057](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470057) — **SUCCESS**. UTC formatter/parser unit and integration tests, JSON output boundary, SQLite SQL/parameter resource-limit tests, missing-file read-only regression, and strict one-core Clippy passed.
- Linux distro matrix: [run 40006470246](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470246) — **SUCCESS** on Ubuntu 24.04 and Debian 12; contract checks, build, Clippy, workspace tests, and real-network tests passed.
- Cross-platform matrix: [run 40006470017](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470017) — **SUCCESS** on both `ubuntu-latest` and `windows-2025`. Both jobs passed contract artifacts, build, AST contract tests, Clippy, workspace tests, real-network runtime tests, and platform-appropriate offline verification. Main workspace suite summaries show 215 passed / 0 failed on Ubuntu and 210 passed / 0 failed on Windows; a separate integration suite has an ignored Android AArch64 ELF build-and-execute test on both platforms. The Android-target CLI build test passed on Ubuntu but only checks artifact creation. The Windows-only CLI suite filtered out its Android-target test because the Windows runner is not an Android toolchain. No Android ELF runtime pass is claimed.
- The source repository's own Actions on this same SHA remain infrastructure failures: [CI run 40006462899](https://github.com/panglimanetizen01/one/actions/runs/40006462899), [Linux distribution run 40006463227](https://github.com/panglimanetizen01/one/actions/runs/40006463227), and [focused run 40006463315](https://github.com/panglimanetizen01/one/actions/runs/40006463315) failed with `runner_id=0` and empty step lists. They produced no source build/test result. The documented independent mirror was used instead; no paid plan upgrade or fabricated CI result was used.

### Forensic correction

The earlier mirror run [40005692552](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40005692552) was initially marked successful, but its Windows offline-script log contained a failed Android-target native CLI test followed by `OFFLINE VERIFICATION: PASS`. That run is **not credited as valid full verification**. The failure-propagation and platform-specific skip fix was committed, and the exact-SHA cross-platform run 40006470017 was then rerun successfully. Its Windows offline log has no `test result: FAILED` marker and ends with `OFFLINE VERIFICATION: PASS`.

### Updated point accounting

- E1 implementation: **7/10** provisional; verified remains **0/10** because the full required standard-library acceptance set is incomplete.
- E4 implementation: **3/6** provisional; verified remains **0/6** because race-resistant filesystem resolution, broader quotas, and full threat review remain incomplete.
- E2 remains 5/8; E3 remains 8/8 implemented but 0/8 verified until Android AArch64 ELF build-and-execute acceptance is satisfied; E5 remains 1/4; E6 remains 0/4.
- **Implementation subtotal: 84/100. Verified completion: 60/100.** Fixed row weights are unchanged. The 24-point implementation/verification gap does not count toward a PASS gate.

**Remaining mandatory work:** external dependency resolution and authenticated package provenance; broader standard-library coverage; race-resistant filesystem confinement and wider resource quotas; compatibility/release review; full developer-tooling/docs acceptance; Android AArch64 ELF build-and-execute verification in a toolchain-capable environment if required by the declared target contract; and the E6 forensic audit.  
**Gate:** **NOT PASSED — Milestone 100% remains in progress.**

**Forensic scoring correction (2026-10-24):** The earlier 68/100 verified subtotal credited E3's 8 points based on Linux/Windows matrix results and an Android-target CLI build check. Inspection of `crates/one-core/tests/native_backend_android_aarch64.rs` shows the separate Android ELF build-and-execute test is `#[ignore]`; the CLI check only asserts that an artifact file is created. Since ignored tests are not passes and Android AArch64 is a declared target, E3's verified points are reset from 8/8 to 0/8 until the runtime acceptance is satisfied. This does not change E3's 8 implementation points or any row weight; verified completion is now 60/100.


## Post-checkpoint exact-SHA confirmation — `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974`

The documentation-only checkpoint snapshot `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974` was also checked out and verified exactly:

- [Focused stdlib/database run 40007463554](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463554) — SUCCESS.
- [Linux distro run 40007463563](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463563) — Ubuntu 24.04 and Debian 12 SUCCESS.
- [Cross-platform run 40007463527](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463527) — Ubuntu and Windows SUCCESS. Logs verify exact source SHA `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974`, no `test result: FAILED` markers, and one clean `OFFLINE VERIFICATION: PASS` per platform.
- Main workspace logs show 215 passed / 0 failed on Ubuntu and 210 passed / 0 failed on Windows. The Android-target CLI build test passed on Ubuntu (logged as `test android_target_explicit_mempertahankan_build_native ... ok`), but the separate `build_native_menghasilkan_dan_menjalankan_elf_android_aarch64` test remains ignored on both platforms. The Windows-only CLI variant is filtered out on Windows; no Android ELF runtime pass is claimed.
- Canonical repository workflows on the same source SHA still failed before steps with `runner_id=0`; independent mirror runs provide the actual test evidence.

The later ledger/checkpoint corrections are documentation-only and do not change the tested code snapshot. **Gate remains NOT PASSED** because the full E1/E2/E4/E5/E6 Definition of Done is not complete.


## Final exact-SHA run set — `ebf254d1194a903a3b582f3a18abfc402f2003d2`

- Focused stdlib/database [40009389100](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389100) — SUCCESS (25 successful steps).
- Linux distro [40009389164](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389164) — SUCCESS on Ubuntu 24.04 and Debian 12.
- Cross-platform [40009389190](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389190) — SUCCESS on Ubuntu and Windows; exact source SHA confirmed in both logs. Both jobs passed build, Clippy, workspace tests, real-network tests and offline verification. Logs contain zero `test result: FAILED` markers and one `OFFLINE VERIFICATION: PASS` per platform.
- Android AArch64 ELF build-and-execute remains ignored on both platforms; the Android-target CLI check verifies artifact creation only. Therefore E3 stays 8/8 implemented, 0/8 verified. Implementation is 84/100, verified completion 60/100, gate NOT PASSED.
- The subsequent ledger/checkpoint edit is documentation-only; source implementation remains the tested snapshot.
