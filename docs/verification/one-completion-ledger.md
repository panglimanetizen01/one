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
| E2 | Packaging, dependency and project/build tooling for vNext — **8** | Deterministic artifacts, version metadata, supported project workflow | 7/8 | 0/8 | IMPLEMENTED — PARTIAL; deterministic artifacts, Cargo.lock/--locked checks, V2 source-bound provenance, trusted-publisher verification, and a protected release workflow are present; protected environment dispatch and exact-SHA release acceptance remain pending. External registry/version-conflict resolution is explicitly outside the executable v0.1 subset and must fail closed, not be counted as a missing v0.1 feature |
| E3 | Conformance, compatibility and cross-platform suite — **8** | Exact-SHA Linux/Windows plus other declared targets where applicable | 8/8 | 0/8 | IMPLEMENTED — PARTIAL; dedicated exact-SHA Android arm64 emulator workflow invokes the ignored ELF runtime test explicitly; runtime result pending |
| E4 | Security, resource bounds, diagnostics and failure recovery — **6** | Threat review and tests for unsafe/invalid inputs and runtime failures | 5/6 | 0/6 | IMPLEMENTED — PARTIAL; SQLite budgets, Unix descriptor-relative no-follow opens (Linux/Android/macOS), Windows reparse-point/open-handle checks, and bounded package-provenance reads/tests are present; explicit root confinement, broader runtime quotas, and full threat review remain pending |
| E5 | Documentation, examples and developer tooling — **4** | Reviewed reference docs and tested user workflows | 4/4 | 0/4 | IMPLEMENTED; manual user-workflow review and documentation reconciliation are recorded in the independent acceptance review; final exact-SHA test acceptance remains pending |
| E6 | Final forensic audit, release evidence and checkpoint — **4** | Definition-of-Done audit at release SHA; no unresolved required gates | 0/4 | 0/4 | NOT IMPLEMENTED |
|  | **TOTAL** |  | **91/100** | **60/100** | **MILESTONE 100% IN PROGRESS — GATE NOT PASSED** |

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

## 2026-10-24 continuation — Android runtime gate and filesystem no-follow hardening

- **Code-bearing source SHA:** `b5c3827840a691868a69428ae4032614407f1062` (latest source branch head before this ledger/checkpoint update).
- **Git state at implementation checkpoint:** branch `milestone/100-ecosystem-20261023`; PR #80 remains open, draft, unmerged, base `main`. No merge or direct-main write was performed.
- **E3 — Android AArch64 runtime:** strengthened `crates/one-core/tests/native_backend_android_aarch64.rs` to assert ELF magic, ELFCLASS64, little-endian encoding, and `EM_AARCH64` before pushing the artifact to Android and executing it. Added a dedicated Android API 35 arm64 emulator job to canonical CI and the independent verification mirror. The job explicitly runs the previously ignored build-and-execute acceptance test; a build-only pass does not count. **Runtime result on this SHA: NOT YET VERIFIED.**
- **E4 — filesystem race-sensitive resolution:** added direct `libc` dependency and Linux/Android descriptor-relative directory traversal using `openat`, `O_DIRECTORY`, and `O_NOFOLLOW` for each parent component and final file opens. Read/write targets are checked as regular files after opening. Added regression tests for symlinked parent directories, final-component read symlinks, and final-component write symlinks preserving their referent. Updated `docs/SECURITY.md` to state the guarantee and boundaries accurately.
- **E4 boundary remains open:** Windows/macOS do not yet have equivalent descriptor-relative race-resistant opens; the capability contract still lacks a root confinement directory. Do not claim a complete cross-platform filesystem sandbox.
- **E2 regression:** added a CLI test ensuring unsupported dependency declarations fail closed instead of being silently ignored. This does **not** implement external registry resolution, a committed Cargo lockfile, signed publisher identity, or authenticated provenance; E2 remains partial.
- **Verification:** canonical source Actions on prior source SHA `6361461a81718594c78921abb3106b1fdf86e1c5` failed before any steps started; all inspected jobs returned empty step lists. This is a runner provisioning failure, not a source-test result. The independent mirror selector must be pinned to the final checkpoint SHA after this documentation update. No new code verification points are awarded yet.
- **Score reconciliation:** E4 implementation increases from 3/6 to 4/6 because descriptor-relative no-follow opens are now connected to the Linux/Android file adapters and regression tests exist in source. **Implementation: 85/100 provisional. Verified: 60/100.** E1 0/10, E2 0/8, E3 0/8, E4 0/6, E5 0/4, E6 0/4 verified points remain unchanged under full milestone-100 acceptance.
- **Remaining E1–E6:** full standard-library contract breadth; external dependency/registry resolution and authenticated package provenance; actual Android AArch64 ELF execution; Windows/macOS race-resistant filesystem handling and defined confinement root; broader resource quotas/threat review; release/compatibility/tooling acceptance; final exact-SHA forensic audit.
- **GATE: NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — committed Cargo lockfile and locked verification

- **Code-bearing source SHA before this documentation update:** d9196cf97c4f42de8a8dbf8e1de93ced5b8db4dd.
- **Lockfile generation evidence:** independent workflow [ONE Cargo Lockfile Bootstrap Evidence](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40052223860) successfully ran Cargo lockfile generation against canonical source SHA cd4aa9f9be450ecc43a0419d057867967d23e469. Cargo.lock SHA-256: 84b2111b5d1ab0c942c6fc68040285bccc3065a5f4e4150814e202c0fe15bc21; 113 package entries.
- Added the generated root Cargo.lock to the canonical repository and removed Cargo.lock from .gitignore. Updated canonical CI, Linux distro conformance, focused standard-library verification, and local verification scripts to invoke Cargo build/test/run/Clippy with --locked.
- Added the lockfile policy to docs/verification/human-vnext-exact-sha-verification-contract.md. The policy explicitly distinguishes Rust workspace dependency locking from One-language registry resolution and publisher authentication.
- **E2 remaining:** no One-language external dependency graph/registry resolver, version-conflict resolution, publisher trust store, digital signature verification, or authenticated provenance envelope. The lockfile change closes only Cargo workspace dependency determinism.
- **Verification on the current lockfile-bearing source SHA:** NOT YET VERIFIED. The mirror must pin the final checkpoint SHA and confirm --locked succeeds on Ubuntu and Windows and the distro/stdlib jobs complete.
- **Score reconciliation:** E2 implementation increases 5/8 → 6/8 because the Cargo lockfile is committed and the repository's verification commands now enforce it. Implementation subtotal: **86/100 provisional**. Verified subtotal remains **60/100**; no verification credit is assigned for the lockfile until exact-SHA checks pass.
- **GATE: NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — toolchain reproducibility and workflow pin

- Root rust-toolchain.toml now pins Rust 1.99.0 with rustfmt and Clippy. This version was observed in the successful lockfile-generation log; the previous moving stable channel resolved to 1.99.1 in a verification job, so relying on stable alone was not deterministic.
- Canonical CI, Linux distro conformance, and focused stdlib workflows now explicitly request Rust 1.99.0 from dtolnay/rust-toolchain. The independent mirror's Android, distro, and focused stdlib workflows now do the same. The mirror cross-platform workflow uses the source root rust-toolchain.toml and now invokes Cargo commands with --locked.
- Source commits containing the lockfile and toolchain pin are followed by workflow changes; the current source branch head must be re-read and mirrored before any release-gate credit.
- **E5 implementation score:** 1/4 → 2/4 because the toolchain pin and documented reproducibility contract are now present alongside the committed lockfile policy. Full compatibility, release packaging and developer workflow review remain incomplete.
- **Current implementation subtotal: 87/100 provisional. Verified subtotal: 60/100.**
- **GATE: NOT PASSED — exact-SHA verification for the lockfile/toolchain workflow revisions is still required.**

## 2026-10-24 continuation — exact-SHA lock/toolchain verification and workflow repair

- **Current code/workflow source SHA before this ledger update:** 34f2162918eaff316f68659b51d5666333160a29.
- **Cargo.lock:** committed root lockfile has 113 package entries and SHA-256 84b2111b5d1ab0c942c6fc68040285bccc3065a5f4e4150814e202c0fe15bc21. The independent generator workflow succeeded against source SHA cd4aa9f9be450ecc43a0419d057867967d23e469.
- **Toolchain drift was found and fixed:** an exact-source focused run initially resolved moving stable to Rust 1.99.1. Root rust-toolchain.toml and explicit workflow inputs now pin Rust 1.99.0. The updated independent focused stdlib/database workflow passed on source SHA c740982ef40b680ac8ae8682e738272ffad05331; its log reports rustc 1.99.0, and 28 Cargo commands include --locked. This result is valid for that SHA, not automatically for the current source SHA.
- **Filesystem regression:** on an earlier exact SHA, workspace tests exposed malformed NUL-terminated root literals in the new absolute-path resolver. This was corrected, then strict Clippy exposed manual C-string literals; those were corrected to Rust C-string literals. The focused lint/test workflow passed after the path correction on the source snapshot before the lockfile/toolchain revisions. The final snapshot still requires a fresh full test run.
- **Mirror workflow repair:** explicit Rust toolchain configuration initially introduced duplicate YAML with blocks in the Linux distro workflow. The malformed workflow runs had no jobs; the duplicate blocks were merged and a new run was queued. Do not credit those parse failures as source-test failures or passes.
- **Latest exact-source mirror jobs:** source SHA c740982ef40b680ac8ae8682e738272ffad05331 is undergoing Ubuntu/Debian and Windows/Linux cross-platform verification; the focused job passed. Android arm64 runtime job is still installing its SDK/NDK/system image and has not reached ELF execution.
- **Canonical Actions:** inspected jobs on source SHA f93a960635d648d6b9390bef42ed42328bf2ddec failed before any steps started (empty step lists). This remains a runner-provisioning failure, not source-test evidence.
- **E3:** the dedicated Android workflow now explicitly runs the ignored acceptance test and asserts the emulator ABI is arm64-v8a. The workflow has not yet produced an ELF execution result; E3 verified points remain zero.
- **Score:** implementation remains **87/100 provisional** and verified remains **60/100**. No verified points were added by the current work. E1/E2/E3/E4/E5/E6 acceptance is not complete.
- **GATE: NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — full Linux acceptance and exact-SHA script closure

- **Code/workflow source SHA before this ledger update:** 9bf679bb52810031a533025cbc5a18e97450d63b.
- The release-check workflow and tools/verify_exact_sha.sh were also updated to use Cargo --locked. A recursive audit of all workflow/shell/PowerShell files found no remaining unlocked Cargo build/test/run/Clippy command; the sole non-locked match is cargo clippy --version, which does not resolve dependencies.
- **Exact-SHA focused standard-library/database job passed** on source SHA 102741eddac431cfe4f1b4b24bec3d7a2032428b. Log reports rustc 1.99.0 and 28 --locked Cargo invocations.
- **Exact-SHA Linux distro job passed** on source SHA 102741eddac431cfe4f1b4b24bec3d7a2032428b on both Ubuntu 24.04 and Debian 12. Each job reported Rust 1.99.0, seven --locked Cargo commands, and 64 successful test-result groups; the logs include successful filesystem no-follow regression tests and the fail-closed unsupported-dependency test.
- These passes are evidence for SHA 102741, not for the newer source SHA 9bf679 containing the release-check/exact-SHA script updates. A final mirror run must target the post-checkpoint SHA.
- **Current Windows cross-platform job:** prior run on SHA 102741 was cancelled by a later mirror-workflow push after the Windows test steps had completed and before cleanup finished. It is not credited as a complete run; rerun required.
- **Android runtime:** exact-SHA Android arm64 emulator job has installed the SDK/NDK/system image but has not reached the ELF execution step. The old unbounded boot step is still running; a newer workflow revision now bounds adb wait/boot and uploads emulator logs on failure.
- **Score remains 87/100 provisional implementation, 60/100 verified.** No new verified points are awarded from the partial/cancelled matrix.
- **GATE: NOT PASSED — exact-SHA cross-platform and Android runtime results remain pending.**

## 2026-10-24 continuation — bounded package-provenance inputs

- Added bounded file reads to package signing and verification: artifact <=64 MiB, private-key seed file <=256 bytes, provenance sidecar <=16 KiB, and publisher trust store <=1 MiB. The bounded-read regression rejects inputs above the configured limit.
- Implementation score: E4 4/6 -> 5/6 (+1) because the new bounds are integrated into the signing/verification path with a regression test. Overall implementation subtotal 87/100 -> 88/100. Verified subtotal remains 60/100; the full workspace test matrix for the latest source SHA is still running.
- Exact source SHA 0a0f250dfc2e556bca40f2201752ee5d06f56854: focused standard-library/database verification passed using Rust 1.99.0 and 28 --locked Cargo commands. Linux distro verification passed on Ubuntu 24.04 and Debian 12; each reports Rust 1.99.0, seven --locked commands, and 64 successful test-result groups.
- Current exact-SHA Windows cross-platform and Android arm64 emulator runs are still in progress. Do not credit E3 verified points until Windows and the actual Android ELF execution acceptance both complete.
- E2 correction: artifact signing/trusted publisher verification is implemented, but the external One package registry, dependency graph/version conflict resolution, automatic package signing, and complete build provenance remain unimplemented.
- GATE: NOT PASSED — mandatory E1-E6 acceptance is incomplete.


## 2026-10-24 continuation — Android SDK license handling and latest-SHA evidence audit

- **Code/workflow SHA:** `4291a820e07b99820f1789f344bc2ab427f7f9d5` ([commit](https://github.com/panglimanetizen01/one/commit/4291a820e07b99820f1789f344bc2ab427f7f9d5)).
- Corrected the Android emulator job's SDK license pipeline. The previous `yes | sdkmanager --licenses || true` masked genuine SDK-manager failures. The replacement tolerates the expected SIGPIPE from `yes` while explicitly checking `sdkmanager`'s exit status and failing closed when it is nonzero.
- Exact-SHA GitHub Actions runs inspected for this commit: One CI [40060363429](https://github.com/panglimanetizen01/one/actions/runs/40060363429), focused standard-library verification [40060363449](https://github.com/panglimanetizen01/one/actions/runs/40060363449), and Linux distribution conformance [40060363463](https://github.com/panglimanetizen01/one/actions/runs/40060363463). All three report `failure`.
- The GitHub job-details API returned empty step arrays for all six jobs; log retrieval returned `BlobNotFound` and there were no workflow artifacts. Consequently, these runs do **not** provide executable build/test logs or establish whether test steps ran. Do not infer a source-code failure from this alone, and do not infer a pass.
- The Android acceptance test source explicitly checks ELF magic, ELFCLASS64, little-endian encoding and `EM_AARCH64`, then pushes and executes the artifact through ADB and asserts exact output. This source-level inspection is not a substitute for successful emulator execution evidence.
- PR #80 remains open, draft and unmerged. The verified score remains **60/100**; implementation remains **88/100 provisional**. No new verified points are awarded.
- **GATE: NOT PASSED.** E1–E6 acceptance and usable exact-SHA verification evidence are still incomplete.


### CI infrastructure evidence — run attempt 2 (2026-10-24)

- Follow-up inspection of the raw GitHub Actions job payloads for run `40060644366` and `40060644370` confirms every failed job has `runner_id: 0`, an empty `steps: []` array, and a runtime of only a few seconds. The One CI check runs on the same source SHA `1c450863cdfcb824dcf7cef6b64e86e58bc25b44` also completed in approximately two seconds with failures before any step was recorded.
- This is evidence of a job/runner provisioning failure before workflow execution, not evidence that the source tests themselves failed. The API exposes no annotation details through the available connector and log downloads return `BlobNotFound`; therefore the exact underlying service/configuration cause remains unproven.
- Retried the failed focused stdlib and Linux distro jobs. The retry again reported `runner_id: 0` and no steps/logs. Do not repeat retries indefinitely as if they were source-level tests.
- The verification mirror's `verification` branch selector was advanced to the exact source SHA `1c450863cdfcb824dcf7cef6b64e86e58bc25b44`, which should trigger its push workflow. The mirror workflow requires `CANONICAL_READ_TOKEN` to fetch the private source. No successful mirror run has yet been independently observed for this selector; treat its execution and secret configuration as unverified until a concrete run and logs are available.
- No verified points are added. **GATE remains NOT PASSED.**


### Subsequent branch-head reconciliation

- A later branch commit was observed at `6826e3a1b41525bbfa5d3f02dda100c7e59a7004` (`docs(security): document signed artifact format binding`). This commit updates the provenance documentation to bind the validated package format alongside the artifact digest.
- Exact-SHA runs for this later commit: One CI [40062110808](https://github.com/panglimanetizen01/one/actions/runs/40062110808), focused stdlib [40062110699](https://github.com/panglimanetizen01/one/actions/runs/40062110699), and Linux distro [40062110726](https://github.com/panglimanetizen01/one/actions/runs/40062110726) all failed before recorded steps; each job reports `runner_id: 0`, empty steps, and a 2–3 second runtime.
- The verification mirror selector has independently advanced to `6826e3a1b41525bbfa5d3f02dda100c7e59a7004`; its runner result is not confirmed. No points are added and the gate remains **NOT PASSED**.


## 2026-10-24 — E4 ledger reconciliation and exact-SHA evidence audit

- **E4 documentation correction:** the current source snapshot already contains Unix descriptor-relative opens with `openat`/`O_NOFOLLOW` (the Unix implementation includes macOS) and Windows reparse-point plus opened-handle validation. The current `docs/SECURITY.md` describes these controls and their limits; `crates/one-core/src/adapter.rs` contains the implementation and regression tests. Earlier checkpoint notes stating that Windows/macOS race controls were still absent describe an older revision and are superseded for current-state reporting.
- **E4 remains partial:** the adapter contract still authorizes explicit target paths rather than a separately configured project root; broader runtime-wide quotas and the complete threat review remain open. Keep E4 implementation at 5/6 provisional and verified at 0/6 until final-revision acceptance is complete.
- On canonical source SHA `e745b6b46ec436e7f7d005a95cb24333a4128533`, independent exact-SHA cross-platform verification passed on Ubuntu and Windows in [run 40065575079](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40065575079); Ubuntu 24.04 and Debian 12 conformance passed in [run 40065353608](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40065353608); focused standard-library/database regressions passed in [run 40064936650](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40064936650). All three logs explicitly checked out and verified that exact source SHA.
- Linux and Windows release bundles for that SHA were created with SHA-256 digests and remain available from [run 40065575079](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40065575079). These results do not satisfy the Android runtime gate.
- Android attempts on the same source SHA: x86-host ARM64 TCG failed with `Timeout waiting for emulator to boot` ([run 40066472833](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066472833)); Linux ARM64 runner setup failed before emulator launch because the Android SDK emulator package was unavailable ([run 40066577198](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066577198)). The independent workflow was moved to a macOS ARM64 host for a native ARM64 Android emulator; [run 40066663909](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066663909) was still in progress when this ledger correction was prepared.
- This is a documentation-only ledger correction. Per the exact-revision rule, the resulting commit SHA must still receive fresh exact-SHA verification; historical successful runs for the prior source SHA are not promoted to final-head proof.
- **Scores unchanged:** implementation 88/100 provisional; verified 60/100. **GATE: NOT PASSED.** E1, E2, E3, E4, E5, and E6 acceptance is not complete.


## 2026-10-24 — E2 scope reconciliation against normative package contract

- The current `docs/spec/one-v0.1-package-build.md` explicitly states that the executable v0.1 package contract supports an empty dependency set only. External One package registry, dependency graph and version-conflict resolution are outside that executable v0.1 subset; unsupported dependency declarations must fail closed. Do not list the absence of a registry as a v0.1 conformance defect or use it alone to block the milestone.
- The same normative contract documents the milestone-100 Ed25519 V2 sidecar binding the validated artifact format/digest to source-tree digest, clean Git revision, CLI version, builder OS/architecture, Rust/Cargo versions, and the empty-v0.1 dependency model. Verification checks the signature and a separately configured trusted-publisher mapping; optional source-root verification recomputes source digest/revision.
- **Actual E2 remaining gate:** release process integration and evidence that the final artifact is signed after the final build and verified against a trust store delivered through an independently trusted channel, plus exact-SHA acceptance. Artifact-only V1 provenance is not enough. The ledger row has been corrected to match this contract; the E2 implementation score remains 6/8 provisional and verified remains 0/8 until acceptance evidence passes.
- This corrects the scope wording without relaxing any approved contract or changing point weights. Overall totals remain **88/100 provisional implementation, 60/100 verified**. **GATE: NOT PASSED.**


## 2026-10-24 — E2 release-process integration added; final acceptance pending

- Added `.github/workflows/package-provenance-release.yml` as a protected manual release path. It builds a committed project with `one package`, signs the final `.onepkg` artifact using Ed25519 V2 source-bound provenance, verifies it against the configured trusted-publisher store and the same source tree, and uploads the artifact, sidecar, and SHA-256 digests.
- The release workflow requires the protected `one-release` environment and the externally provisioned `ONE_RELEASE_SIGNING_SEED` and `ONE_TRUSTED_PUBLISHERS` environment secrets. The trust store must be provisioned independently; the workflow does not synthesize a trust store from the signing key.
- Added `docs/RELEASE_PROVENANCE.md` with setup and operational acceptance requirements. Expanded the CLI integration regression to build the package through `one package` before signing and verification, retaining untrusted-publisher, artifact-tampering, and source-mismatch checks.
- The first exact-SHA build exposed a compile error in the new regression because the original artifact bytes were not retained. Commit `435fefdabc2b90d52883e08ef6edd9dc82b9173f` retains the built artifact bytes; the follow-up exact-SHA matrix must confirm this correction and all release workflow changes.
- This is implementation evidence, not release acceptance. The protected environment secrets/review rules have not been exercised by an actual release dispatch in this session, and no verified milestone points are awarded until exact-SHA tests pass and the trust-store provisioning is demonstrated.
- **Scores remain 88/100 provisional implementation and 60/100 verified. GATE: NOT PASSED.**


- Follow-up: added the committed release fixture at `examples/package-release/{one.toml,src/main.one}` and documented it in `docs/RELEASE_PROVENANCE.md`. It gives the protected manual release workflow a reproducible project input. This does not count as a release run; the workflow still requires the independently provisioned environment secrets and approval controls.


- Release workflow hardening follow-up: validated the publisher ID and 32-byte seed format before use, bounded the project path to the checked-out repository, required an exact publisher entry in the protected trust store, and changed artifact upload to use the exact package path emitted by the build step. The workflow uploads only the signed package, its provenance sidecar, and checksums.
- The workflow and committed fixture are implementation artifacts; they do not substitute for a protected-environment release dispatch. Keep E2 verified points at 0 until the final exact-SHA test matrix passes and the actual protected signing/trust-store process is exercised.


## 2026-10-24 — E4 explicit filesystem root policy added

- Added `ScopeRule::Root` for filesystem `Target` requests. The rule requires path-component containment, rejects parent-directory components, and rejects absolute/relative scope mismatches.
- Added CLI `--root <directory>` for `filesystem.read` and `filesystem.write`, preserving the prior exact-target behavior when `--root` is omitted. Added policy regressions for allowed descendants, prefix-sibling denial, parent-traversal denial, and a CLI integration test for inside-root versus outside-root reads.
- Documented the option in `docs/SECURITY.md` and explicitly limited the claim: this is a capability-policy boundary backed by existing adapter no-follow/reparse-point protections, not an operating-system sandbox or a substitute for general memory/open-resource quotas and full threat review.
- These changes are not credited as verified until the latest exact-SHA build, Clippy, workspace, network, distro, and Android checks complete successfully. E4 verified points remain 0/6 pending full acceptance.


## 2026-10-24 — E5 documentation coverage added; acceptance pending

- Added `docs/panduan-pengguna/referensi-standard-library.md` documenting registered source-level built-ins, parameter/return types, deterministic behavior, limits, and known unsupported domains.
- Added `docs/panduan-pengguna/alur-pengembangan.md` for the end-to-end project → local verification → exact-SHA evidence → release gate workflow; expanded `kesalahan-dan-debugging.md` with diagnostic families and capability/scope troubleshooting; updated CLI documentation for `--root`, linked the new pages from the documentation indexes, and recorded the current candidate boundary in the feature-status page.
- These documentation artifacts still require example/link review and exact-SHA contract/build/test evidence. No E5 verified points are awarded from documentation presence alone. Keep the ledger at **88/100 provisional implementation, 60/100 verified** until a fixed-weight forensic reconciliation is completed.
- The latest source revision after this documentation update must be pinned in the independent verification selector. The release gate remains **NOT PASSED**.


- Documentation index review found and repaired literal `\\n` sequences that had collapsed several verification links into single lines in `docs/README.md`. The final doc candidate now includes the standard-library reference, developer workflow, expanded debugging guidance, CLI root-scope instructions, and release provenance operations. The examples and links still require final review on the tested revision.


- Acceptance-boundary correction: `docs/spec/one-v0.1-effect-capability.md` now records that the new `--root` policy is defense-in-depth but **not full E4 root-confinement acceptance**. Root-relative descriptor anchoring, race-resistant resolution, mount behavior, and cross-platform case/path semantics remain open; E4 verified points remain 0/6.


## 2026-10-24 — E5 user-facing release guidance and documentation gate

- Added `docs/panduan-pengguna/provenance-rilis.md`, linked it from both documentation indexes, and changed the end-to-end guide to keep its relative links within the user-documentation tree. The new page explains that the manual release workflow becomes dispatchable after it is present on the default branch, and documents protected environment, key/trust-store separation, and the acceptance boundary.
- The independent verification mirror now runs `tools/check_user_docs.py` and its regression tests alongside contract checks on both Linux and Windows. Canonical `.github/workflows/ci.yml` now also runs the user-doc consistency checker and its tests on both platforms.
- Exact-SHA runs started after these documentation/CI changes; the final ledger/checkpoint commits will change the source SHA again, so the selector must be repinned and the full matrix rerun after those commits. Do not reuse results from earlier source SHAs as final-release evidence.
- Android emulator diagnostics identified `qemu-system-aarch64: failed to initialize HVF: Invalid argument` on the hosted macOS ARM64 runner. The independent Android workflow now uses `-accel off` to avoid unavailable host acceleration. Actual Android ELF execution remains unverified until a run completes successfully.
- **Scores unchanged: 88/100 provisional implementation, 60/100 verified.** E1–E6 remain incomplete; **GATE: NOT PASSED**. This entry awards no points from documentation or workflow presence alone.


## 2026-10-24 — provisional implementation reconciliation after E2/E5 integration

- **E2 implementation:** 6/8 → 7/8. The protected release workflow now builds, signs, verifies, and uploads the final source-bound V2 package path; actual protected-environment dispatch and trusted-store provisioning have not been demonstrated.
- **E5 implementation:** 2/4 → 3/4. Added a user-facing release-provenance guide, explicit CLI documentation for signing/verifying, clearer unsigned-package boundaries, and enforced documentation consistency checks in canonical CI plus the independent Linux/Windows mirror.
- These are provisional implementation points only. **Implementation: 90/100; verified completion: 60/100; gate NOT PASSED.** The final candidate SHA changed with this ledger update and must be pinned and tested exactly. E3 earns no verified points until the Android AArch64 ELF is actually executed on the current source SHA.


## 2026-10-24 — final acceptance checklist and candidate freeze preparation

- Added [`docs/verification/DEFINITION_OF_DONE.md`](DEFINITION_OF_DONE.md) as the auditable fixed-weight E1–E6 release checklist. It distinguishes implementation artifacts from verified acceptance and explicitly prohibits credit for a running workflow, prior-SHA result, checksum-only provenance, or infrastructure-only CI failure.
- Reconciled the provisional score: the latest E2/E5 adjustment is authoritative — **90/100 implementation, 60/100 verified, release gate NOT PASSED**. Earlier 88/100 statements in historical entries are superseded, not additional active score.
- Candidate source was advanced by the Definition of Done document. The independent verification selector must pin the resulting commit SHA and rerun cross-platform, Linux distro, focused standard-library/database/provenance, lockfile, and Android workflows. Results from earlier source revisions are historical only.
- Do not make further source changes after the next candidate SHA is selected unless a required test reveals a real defect. Any defect fix creates a new candidate and requires the exact-SHA matrix to be rerun.
- Outstanding acceptance is unchanged: E1–E6 row-level verification is not complete; protected signing/trust-store dispatch has not been exercised; Android AArch64 ELF runtime has not passed; E4 full threat/root-confinement review is open. **No release authorization.**


## 2026-10-24 — SQLite final-symlink hardening follow-up

- Security review found that the SQLite adapter performed a path precheck and then reopened the path through SQLite, leaving a final-component symlink time-of-check/time-of-use window.
- File-backed SQLite connections now combine `SQLITE_OPEN_READ_ONLY` with `SQLITE_OPEN_NOFOLLOW`; a Unix regression checks that SQLite rejects a final symlink and that the adapter denies the target.
- This is defense in depth only: it does not eliminate parent-component races or establish root confinement across mount points/platforms. E4 verified credit remains **0/6** until the complete scoped security review and exact-SHA matrix pass.
- The reusable Definition of Done no longer embeds an old candidate SHA. The next source commit is the freeze candidate for the independent matrix. **Implementation 90/100; verified 60/100; gate NOT PASSED.**


## 2026-10-24 — E5 user-workflow review and documentation reconciliation

- Reviewed the user-facing start guide, CLI command reference, test/build/package guide, standard-library reference, feature-status page, development workflow, and package-provenance instructions against the current CLI contract and executable tests.
- Corrected stale claims in `docs/panduan-pengguna/perintah-cli.md` and `docs/panduan-pengguna/status-fitur.md`: filesystem write, SQLite query, and HTTP GET now reference the exact-SHA Ubuntu/Windows tests and retain their explicit experimental scope.
- Independent review artifact: [USER_WORKFLOW_ACCEPTANCE.md](https://github.com/panglimanetizen01/one-windows-verification/blob/verification/USER_WORKFLOW_ACCEPTANCE.md). The review passes for the tested workflow and documentation consistency, while final exact-SHA reruns remain mandatory after the documentation commits.
- **E5 implementation score:** 3/4 → 4/4 because the documentation gaps are corrected and the user workflow has been reviewed. **E5 verified remains 0/4** until the final candidate's cross-platform, focused, distro, and lockfile evidence passes.
- Current source revision is a documentation/ledger reconciliation candidate; the independent selector must be pinned to the resulting commit SHA and all mandatory exact-SHA checks rerun. No other score or fixed weight changes.


## 2026-10-24 — fixed-weight arithmetic reconciliation

- Forensic arithmetic check of the fixed ledger rows: F1–F6 = 30 verified points and C1–C5 = 30 verified points; E1–E6 implementation rows currently total 31 provisional points (E1 7/10 + E2 7/8 + E3 8/8 + E4 5/6 + E5 4/4 + E6 0/4). Therefore the consistent implementation subtotal is **91/100**, not 90/100. Verified completion remains **60/100** because E1–E6 have 0 verified points.
- This is a correction of arithmetic/reporting only; weights and row scores are unchanged. It does not award verified points or pass the release gate.
- Candidate SHA before this ledger-only correction: `f46b95ebdd0e26c5ba17f6acb953538f32401aeb`. The ledger commit changes the candidate SHA, so all final-SHA evidence must be rerun against the resulting branch head.
- **Gate: NOT PASSED.**
