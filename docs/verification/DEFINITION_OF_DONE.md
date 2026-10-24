# ONE v0.1 — Definition of Done and Release Gate

**Status:** In progress — not a release authorization  
**Candidate branch:** `milestone/100-ecosystem-20261023`  
**Candidate source SHA:** see the synchronized source pin and evidence index; this reusable checklist intentionally does not embed a self-referential commit SHA.  
**Fixed-weight ledger:** [one-completion-ledger.md](one-completion-ledger.md)  
**Independent evidence index:** [CANDIDATE_VERIFICATION.md](https://github.com/panglimanetizen01/one-windows-verification/blob/verification/CANDIDATE_VERIFICATION.md)  
**PR:** [#80](https://github.com/panglimanetizen01/one/pull/80)

This checklist is an acceptance control, not a claim that the release is complete. A checked item requires direct evidence tied to the exact source SHA. Do not adjust ledger weights or award credit because a workflow was started, a bundle exists, or a test on another SHA passed.

## 1. Immutable evidence rules

- [x] Candidate source SHA is recorded explicitly in the PR and independent verification selector; the ledger records the freeze rule without embedding a self-referential commit SHA.
- [x] Independent Linux/Windows build and test evidence is tied to the candidate source SHA.
- [x] Candidate Linux and Windows bundles were inspected; embedded checksum manifests verify and build metadata pins the source SHA.
- [ ] Final candidate SHA is frozen after all remaining source changes; all evidence is rerun against that exact SHA.
- [ ] All evidence links, ledger rows, PR body, workflow pins, and artifact metadata agree on the final SHA.
- [ ] Any failed workflow is classified from logs/job metadata; no infrastructure failure is described as a passing test.
- [ ] No hosted-CI gate is silently replaced by a local or independent run. Any approved alternate evidence must execute equivalent checks on the exact SHA and state the remaining hosted-CI limitation.

## 2. Fixed-weight ledger acceptance

Keep the existing weights unchanged. Mark a row verified only after its acceptance criteria are demonstrated by executable tests and/or review artifacts.

- [ ] **E1 — standard library (10 points):** complete the scope-defined public reusable API contracts; tests cover valid/invalid inputs, deterministic behavior, diagnostics, and cross-platform behavior. Existing focused UTC/JSON/text/list/map/integer tests are partial evidence, not automatic full-row credit.
- [ ] **E2 — packaging and provenance (8 points):** deterministic package/lock behavior and stale-lock rejection pass; v2 source-bound provenance signature verifies against an independently configured trusted-publisher store; source tree is rechecked; protected `one-release` release path is exercised without exposing signing seeds or trust secrets.
- [ ] **E3 — conformance and platforms (8 points):** exact-SHA Linux and Windows suites pass; supported Linux distribution jobs pass; Android AArch64 ELF is built, header-checked, pushed to API 35 arm64-v8a Android, executed, and emits the asserted output. A queued/running emulator is not evidence of runtime acceptance.
- [ ] **E4 — security, resource bounds, recovery (6 points):** capability denial/no-side-effect behavior, Unix no-follow descriptor-relative opens, Windows reparse/handle checks, SQLite limits, and failure paths are tested; scope and race limitations are reviewed and documented. Do not claim a complete sandbox or project-root confinement where the implementation does not provide one.
- [x] **E5 — docs and developer tooling (4 points):** contract checker, documentation consistency checks, examples, CLI create/check/run/build/test/package smoke tests, and developer workflow instructions pass on source SHA `e8156e7d6c0a0f999f24ad00bf670e46f21dc128` in [cross-platform run 40091247192](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40091247192), [focused run 40091247190](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40091247190), [Linux distro run 40091247128](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40091247128), and [lockfile run 40091247177](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40091247177); the final-SHA rerun remains mandatory after ledger reconciliation.
- [ ] **E6 — final forensic audit (4 points):** E1–E5 acceptance is reconciled; required docs, specs, plans, completion ledger, checkpoints, CI evidence, PR metadata, and release provenance are mutually consistent; no mandatory gate remains open.

## 3. Mandatory release gate

The release gate is **NOT PASSED** while any required checklist item or fixed-weight ledger acceptance row remains unchecked, while the final SHA is not frozen, or while a required platform/provenance/security gate lacks direct evidence.

Before authorizing a release:

1. Run the complete required verification matrix against the final SHA.
2. Verify artifact checksums and signed provenance independently.
3. Verify the trusted-publisher configuration and protected release environment without disclosing secrets.
4. Review the final diff and security limitations; ensure no unrelated changes were included.
5. Reconcile the completion ledger and PR description to the same evidence.
6. Record the exact final SHA and the URLs of every passing required workflow.
7. Only then may the ledger and release report say **PASS GREEN LOCKED — FINAL RELEASE**.

## 4. Current known open items

- Canonical repository Actions attempts failed before executable steps (`runner_id: 0`, `steps: [])); those attempts do not prove source tests passed.
- The independent Android AArch64 emulator runtime attempt has not yet provided successful ELF execution evidence. A diagnostic workflow now logs host architecture, emulator boot polls, and emulator log tails to make the next failure actionable.
- The complete security/root-confinement review and full E1–E6 ledger acceptance have not been completed.
- This document does not authorize merging PR #80 or publishing a release.
