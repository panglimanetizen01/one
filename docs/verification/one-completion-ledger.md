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
| C1 | Canonical vNext AST/IR integration — **8** | Human frontend lowers into canonical IR with golden semantic tests | 8/8 | 0/8 | IMPLEMENTED — EXACT-SHA VERIFICATION PENDING |
| C2 | General frontend-to-runtime execution pipeline — **8** | More than the task-list special-case; ordinary programs run through canonical compiler/interpreter | 8/8 | 0/8 | IMPLEMENTED — EXACT-SHA VERIFICATION PENDING |
| C3 | Expanded semantic/type/scope model and diagnostics — **6** | Defined scope/type rules and positive/negative fixture suite | 6/6 | 0/6 | IMPLEMENTED — EXACT-SHA VERIFICATION PENDING |
| C4 | Reusable persistence abstraction and recovery policy — **4** | Versioned storage contract, corruption handling, restart and atomicity tests | 4/4 | 0/4 | IMPLEMENTED — EXACT-SHA VERIFICATION PENDING |
| C5 | Additional real application vertical slices — **4** | At least two additional source programs integrated through the same compiler/runtime | 4/4 | 0/4 | IMPLEMENTED — EXACT-SHA VERIFICATION PENDING |
| C6 | Milestone 60 evidence and checkpoint — **0** | Documentation-only reporting; earns no points by itself | 0/0 | 0/0 | NOT APPLICABLE |
| E1 | Required standard library and reusable APIs — **10** | Scope-defined API contracts and executable tests | 0/10 | 0/10 | NOT IMPLEMENTED |
| E2 | Packaging, dependency and project/build tooling for vNext — **8** | Deterministic artifacts, version metadata, supported project workflow | 0/8 | 0/8 | NOT IMPLEMENTED |
| E3 | Conformance, compatibility and cross-platform suite — **8** | Exact-SHA Linux/Windows plus other declared targets where applicable | 0/8 | 0/8 | NOT IMPLEMENTED |
| E4 | Security, resource bounds, diagnostics and failure recovery — **6** | Threat review and tests for unsafe/invalid inputs and runtime failures | 0/6 | 0/6 | NOT IMPLEMENTED |
| E5 | Documentation, examples and developer tooling — **4** | Reviewed reference docs and tested user workflows | 0/4 | 0/4 | NOT IMPLEMENTED |
| E6 | Final forensic audit, release evidence and checkpoint — **4** | Definition-of-Done audit at release SHA; no unresolved required gates | 0/4 | 0/4 | NOT IMPLEMENTED |
|  | **TOTAL** |  | **60/100** | **30/100** | **60% IMPLEMENTATION PRESENT; 30% VERIFIED; 60% GATE NOT PASSED** |

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

## Milestone 60 implementation candidate — 2026-10-23

- Feature PR: [#79](https://github.com/panglimanetizen01/one/pull/78), branch `spec/human-instruction-grammar-vnext-20261022`.
- Implementation now present for C1–C5: human program header adapter to canonical AST/semantic/IR; explicit CLI execution through the canonical interpreter; tests for values, functions, conditions, lists, loops, scope and semantic rejection; relative multi-file module loading with duplicate/cycle/path-traversal rejection; a versioned bounded key/value persistence API with corruption rejection, restart, rollback and bulk replacement tests; calculator and score-tracker programs.
- The existing vNext task-list CLI and persistence path are left intact. The new key/value API is reusable but has not yet replaced the task-list app's legacy storage format.
- The canonical repository Actions jobs continued to fail before executing steps on earlier commits. Those runs are not evidence of source failure or success.
- Independent hosted verification is being run through the documented mirror fallback. Only a completed exact-SHA run with both Linux and Windows jobs green may award C1–C5 verified points.
- **Implementation subtotal: 60/100. Verified completion: 30/100. Milestone 60 gate: NOT PASSED until exact-SHA acceptance checks succeed.**
- Do not call this a complete vNext language: map values, broader native vNext grammar beyond the canonical-language adapter, and further ecosystem work remain outside this milestone candidate.

## Current result

- **Implementation progress:** 60/100 points present (Foundation plus the current Core/Runtime candidate); C1–C5 points remain provisional until exact-SHA acceptance evidence passes.
- **Verified completion:** 30/100 points (30%); the current Core/Runtime candidate has 0 additional verified points pending exact-SHA Linux/Windows checks.
- **Milestone target:** 60%.
- **Gate:** **NOT PASSED** for milestone 60 until the current candidate passes exact-SHA checks. The prior 30-point Foundation scope remains separately locked only at its recorded implementation SHA.

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

**Gate decision:** award all 30 Foundation points. The 30% target is reached without changing weights or counting ignored tests as passed. Later C/E milestones remain unimplemented and uncredited.
