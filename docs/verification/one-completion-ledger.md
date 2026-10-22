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
| F1 | Versioned grammar contract, lexical/name rules, block and newline rules — **8** | Reviewed formal grammar; Unicode, identifier, ambiguity and versioning rules; positive/negative fixtures | 5/8 | 0/8 | IN PROGRESS |
| F2 | Deterministic parser and semantic rejection — **7** | Exact text-block tests, CRLF, blank lines, indentation, Unicode, duplicate names, unresolved references, stable diagnostics | 5/7 | 0/7 | IMPLEMENTED, NOT VERIFIED |
| F3 | Explicit CLI integration isolated from v0.1 — **5** | CLI command invokes vNext parser and supported runtime; v0.1 regression tests pass | 4/5 | 0/5 | IMPLEMENTED, NOT VERIFIED |
| F4 | Task-list app actions — **5** | Source example supports add/list/complete with input validation through public CLI | 4/5 | 0/5 | IMPLEMENTED, NOT VERIFIED |
| F5 | Durable task persistence — **3** | Data survives a new process; corrupt storage and write failures are reported | 2/3 | 0/3 | IMPLEMENTED, NOT VERIFIED |
| F6 | Beginner docs, example, exact-SHA build/CI evidence and checkpoint — **2** | Reproducible commands, docs agree with implementation, build/test/Clippy/CI green at exact SHA | 1/2 | 0/2 | IN PROGRESS |
| C1 | Canonical vNext AST/IR integration — **8** | Human frontend lowers into canonical IR with golden semantic tests | 0/8 | 0/8 | NOT IMPLEMENTED |
| C2 | General frontend-to-runtime execution pipeline — **8** | More than the task-list special-case; ordinary programs run through canonical compiler/interpreter | 0/8 | 0/8 | NOT IMPLEMENTED |
| C3 | Expanded semantic/type/scope model and diagnostics — **6** | Defined scope/type rules and positive/negative fixture suite | 0/6 | 0/6 | NOT IMPLEMENTED |
| C4 | Reusable persistence abstraction and recovery policy — **4** | Versioned storage contract, corruption handling, restart and atomicity tests | 0/4 | 0/4 | NOT IMPLEMENTED |
| C5 | Additional real application vertical slices — **4** | At least two additional source programs integrated through the same compiler/runtime | 0/4 | 0/4 | NOT IMPLEMENTED |
| C6 | Milestone 60 evidence and checkpoint — **0** | Documentation-only reporting; earns no points by itself | 0/0 | 0/0 | NOT APPLICABLE |
| E1 | Required standard library and reusable APIs — **10** | Scope-defined API contracts and executable tests | 0/10 | 0/10 | NOT IMPLEMENTED |
| E2 | Packaging, dependency and project/build tooling for vNext — **8** | Deterministic artifacts, version metadata, supported project workflow | 0/8 | 0/8 | NOT IMPLEMENTED |
| E3 | Conformance, compatibility and cross-platform suite — **8** | Exact-SHA Linux/Windows plus other declared targets where applicable | 0/8 | 0/8 | NOT IMPLEMENTED |
| E4 | Security, resource bounds, diagnostics and failure recovery — **6** | Threat review and tests for unsafe/invalid inputs and runtime failures | 0/6 | 0/6 | NOT IMPLEMENTED |
| E5 | Documentation, examples and developer tooling — **4** | Reviewed reference docs and tested user workflows | 0/4 | 0/4 | NOT IMPLEMENTED |
| E6 | Final forensic audit, release evidence and checkpoint — **4** | Definition-of-Done audit at release SHA; no unresolved required gates | 0/4 | 0/4 | NOT IMPLEMENTED |
|  | **TOTAL** |  | **21/100** | **0/100** | **30% GATE NOT PASSED** |

## Baseline and evidence

- This ledger is the first weighted completion ledger located for the vNext workstream; no prior canonical weighted formula was found during repository discovery.
- Current source branch: `spec/human-instruction-grammar-vnext-20261022`.
- Current code includes the experimental parser, the dedicated task-list runner, an explicit CLI command, a runnable example, and regression tests. These changes are not credited as verified until they execute successfully.
- The GitHub Actions runs inspected for earlier commits failed before any steps started; the jobs reported an empty step list and logs were unavailable. This is an infrastructure/runner blocker, not evidence of a Rust source failure or a pass.
- The current source SHA and exact-SHA CI state must be refreshed when this ledger/checkpoint is next updated.

## Milestone gate definitions

### 30% — Foundation

PASS only when F1–F6 are fully implemented, relevant checks pass on the exact SHA, v0.1 behavior remains compatible, and all six rows earn their full **30 verified points**. An open PR or committed source alone is insufficient.

### 60% — Core language/runtime

PASS only when all F1–F6 and C1–C5 are verified (60 points total) and the checkpoint is updated.

### 100% — Ecosystem and release

PASS only when every weighted row earns its full verified points, all required CI/release gates pass on the recorded release SHA, and the final forensic audit finds no outstanding mandatory Definition-of-Done requirement.

## Current result

- **Implementation progress:** 21/100 points (provisional, unverified work is shown separately).
- **Verified completion:** 0/100 points for this newly introduced vNext tranche on the current head; tests/build/Clippy/CI have not completed successfully on the exact current SHA.
- **Milestone target:** 30%.
- **Gate:** **NOT PASSED**.
