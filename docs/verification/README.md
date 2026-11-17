# Verification and Evidence Index

This directory contains exact-SHA verification records, release gates, incident analysis, and templates. It is part of the canonical public ONE repository; large native binaries and verifier workflow artifacts may remain in GitHub Actions artifacts or the independent Windows verifier repository.

## Current candidate state — 2026-11-17

- Canonical GitHub `main`: `d35db95153f6ddd79885494c72f5aee16c899751`. It still contains GitHub Actions workflows and does not contain `.circleci/config.yml`.
- PR #102 source `6c72d8e548bbbc28d85be39b8de427614989a038`: GitHub Actions run #2229 / `44086429416` PASS on Ubuntu and Windows Server 2025.
- PR #107 source `21e393157e52728d9510699ac6c9120987e3852c`: GitHub Actions run #2230 / `44088328030` PASS on Ubuntu and Windows Server 2025. This verifies bounded Unix reader shutdown, not production containment of escaped Unix descendants.
- PR #108 is the current CircleCI migration candidate. Its current exact HEAD must be read from live PR metadata; it is based on the latest PR #107 candidate and includes CircleCI configuration, lockfile enforcement, and removal of legacy repository GitHub Actions workflows. CircleCI execution remains NOT VERIFIED until the pipeline reaches terminal state and job logs are inspected.
- PR #103 ledger branch exact SHA `041b359b8504551c2c1705093050d9c467368bac`: GitHub Actions run #2231 / `44089449162` PASS on Ubuntu and Windows Server 2025. Definition of Done remains 3 checked / 10 unchecked; E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED.
- Issue #104: race-resistant filesystem project-root confinement remains OPEN.
- Issue #105: strong Unix escaped-descendant containment remains OPEN. Bounded reader shutdown is not process-tree containment.
- Issue #82: Android AArch64 runtime execution acceptance remains OPEN.

## Canonical records

- [Forensic repository audit — 2026-11-17](forensic-audit-2026-11-17.md)
- [Master continuation prompt — 2026-11-17](MASTER_CONTINUATION_PROMPT_2026-11-17.md)

- [CI and verification evidence contract](../spec/one-v0.1-ci-verification-contract.md)
- [Release contract](../release/one-v0.1.md)
- [Implementation status](one-v0.1-implementation-status.md)
- [Evidence matrix](one-v0.1-evidence-matrix.md)
- [Verification record](one-v0.1-verification.md)
- [Offline evidence template](one-v0.1-offline-evidence-template.md)
- [Historical forensic audit — 2026-10-21](forensic-audit-2026-10-21.md)

The fixed-weight completion ledger and Definition of Done are maintained in [PR #103](https://github.com/panglimanetizen01/one/tree/docs/restore-completion-ledger-dod-20261117/docs/verification).

## Evidence rules

1. Every PASS belongs to the exact source SHA, provider run, executor, and tested boundary recorded.
2. GitHub Actions PASS is not CircleCI PASS; an independent Windows verifier PASS is not a CircleCI Windows job.
3. A CircleCI pipeline error without job logs is NOT VERIFIED as a source failure; distinguish config discovery, runner startup, toolchain bootstrap, and executed tests.
4. A later docs-only commit does not inherit test results unless the verification contract explicitly allows it and the exact source/code diff proves no relevant behavior changed.
5. Never promote DoD checkboxes, E1–E6 points, or release approval based only on a scoped PASS.
