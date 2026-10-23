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
| E1 | Required standard library and reusable APIs — **10** | Scope-defined API contracts and executable tests | 6/10 | 0/10 | IMPLEMENTED — PARTIAL; EXACT-SHA VERIFICATION PENDING |
| E2 | Packaging, dependency and project/build tooling for vNext — **8** | Deterministic artifacts, version metadata, supported project workflow | 5/8 | 0/8 | IMPLEMENTED — PARTIAL; EXTERNAL DEPENDENCY RESOLUTION PENDING |
| E3 | Conformance, compatibility and cross-platform suite — **8** | Exact-SHA Linux/Windows plus other declared targets where applicable | 1/8 | 0/8 | IMPLEMENTED — PARTIAL; FULL MATRIX PENDING |
| E4 | Security, resource bounds, diagnostics and failure recovery — **6** | Threat review and tests for unsafe/invalid inputs and runtime failures | 2/6 | 0/6 | IMPLEMENTED — PARTIAL; FULL THREAT REVIEW PENDING |
| E5 | Documentation, examples and developer tooling — **4** | Reviewed reference docs and tested user workflows | 1/4 | 0/4 | IMPLEMENTED — PARTIAL; FULL DOCS/TOOLING REVIEW PENDING |
| E6 | Final forensic audit, release evidence and checkpoint — **4** | Definition-of-Done audit at release SHA; no unresolved required gates | 0/4 | 0/4 | NOT IMPLEMENTED |
|  | **TOTAL** |  | **75/100** | **60/100** | **MILESTONE 100% IN PROGRESS — GATE NOT PASSED** |

## Baseline and evidence

- This ledger is the first weighted completion ledger located for the vNext workstream; no prior canonical weighted formula was found during repository discovery.
- Current source branch: `spec/human-instruction-grammar-vnext-20261022`.
- Last code-changing SHA at the current repository checkpoint: `26199c0114ea0da5f9784ea4d000ab185799e4fc`. Later commits in this checkpoint sequence only update verification documentation/ledger evidence; the tested source snapshot is unchanged.
- Latest checkpoint update commit: `d936b96283162eea3eff7b4bccec7c6abbcb5f2b`; this records the verified Foundation decision, while later documentation-only commits may advance the branch head.
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


- **Implementation progress:** 75/100 provisional points (60 previously verified points plus 6 provisional E1, 5 provisional E2, 1 provisional E3, 2 provisional E4, and 1 provisional E5 implementation points; no new points are verified).
- **Verified completion:** 60/100 points.
- **Milestone target:** 100%.
- **Gate:** **NOT PASSED** for milestone 100%. The prior 60% gate remains scope-limited to code SHA `3027e43b9e2481d73bdb67dbbbbb2e27c3419639`.
- **E1 partial work:** public deterministic text/list/map/integer helper APIs, bounded deterministic JSON serialization/parsing, Unicode case/prefix/suffix operations, checked list joining/summing, and source-level built-ins are implemented. JSON floating-point/exponent parsing remains unsupported because ONE has no float value type. Filesystem, HTTP/networking, time/date, concurrency, logging/configuration, and complete persistence integration remain incomplete. E1 verified points remain 0/10 pending exact-SHA acceptance evidence.
- **E2 partial work:** `one build`, `one test`, and `one package` process human-core projects through the canonical compiler path; package emits a deterministic versioned artifact and module lock v2 with SHA-256 content digests. Build/test reject stale locks with `ETOOL006`. The lock is not signed and does not authenticate publisher identity or provenance. External dependency declarations/registry resolution, signed package identity, authenticated provenance, target-specific artifacts, and full provenance remain incomplete; E2 verified points remain 0/8.
- **E3 partial work:** valid/invalid human-core conformance fixtures are dispatched through the matching compiler path. Full compatibility coverage and a latest exact-SHA Linux/Windows pass remain pending; E3 verified points remain 0/8.
- **E4 partial work:** checked integer arithmetic, recursion-depth bounds, and error-path tests are implemented. Comprehensive threat review, general memory/open-resource quotas, and broader failure recovery remain incomplete; E4 verified points remain 0/6.
- **E5 partial work:** standard-library/runtime/package contracts, runnable examples, and CLI integration tests exist. Complete tutorials, API reference coverage, debugging workflow, and end-to-end developer workflow review remain incomplete; E5 verified points remain 0/4.
- **E6:** final forensic audit and release evidence have not been completed; 0/4.
- **Current verification:** exact-SHA mirror run [39958882035](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39958882035) targets source SHA `d9d696fa87683027b78fdcbbec504696904055aa`. Ubuntu build, AST contract tests, strict Clippy, workspace tests, real-network tests, and offline verification passed; Windows is still running. Until the required platform matrix completes successfully on the canonical source SHA, no additional verified points are awarded.
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
