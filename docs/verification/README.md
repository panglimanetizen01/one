# Verification and Evidence Index

## Live integration checkpoint — 2026-11-17

- PR #108 now integrates PR #98–#101 source/test changes; integration source commit `e8cd41ec212c3224bdbecf85eaf18c5850096e16`. Normative specifications are synchronized in subsequent commits.
- Combined exact-SHA Ubuntu/Windows verification is NOT RUN. CircleCI job-level evidence is NOT AVAILABLE in this session. Historical parent-SHA PASS results are not transferable.
- PR #108 remains OPEN/DRAFT; main remains unchanged. DoD remains 3/13 and E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED.


This directory contains exact-SHA verification records, release gates, incident analysis, and templates. It is part of the canonical public ONE repository; large native binaries and verifier workflow artifacts may remain in GitHub Actions artifacts or the independent Windows verifier repository.

## Current candidate state — 2026-11-17

- Canonical GitHub `main`: `d35db95153f6ddd79885494c72f5aee16c899751`. It still contains GitHub Actions workflows and does not contain `.circleci/config.yml`.
- PR #102 source `6c72d8e548bbbc28d85be39b8de427614989a038`: GitHub Actions run #2229 / `44086429416` PASS on Ubuntu and Windows Server 2025.
- PR #107 source `21e393157e52728d9510699ac6c9120987e3852c`: GitHub Actions run #2230 / `44088328030` PASS on Ubuntu and Windows Server 2025. This verifies bounded Unix reader shutdown, not production containment of escaped Unix descendants.
- PR #108 is the current CircleCI migration candidate. Its current exact HEAD must be read from live PR metadata; it is based on the latest PR #107 candidate and includes CircleCI configuration, lockfile enforcement, and removal of legacy repository GitHub Actions workflows. CircleCI execution remains NOT VERIFIED until the pipeline reaches terminal state and job logs are inspected.
- PR #103 previous ledger SHA `041b359b8504551c2c1705093050d9c467368bac`: run #2231 / `44089449162` PASS on Ubuntu and Windows Server 2025. Current PR #103 HEAD `fde2b81b2f63743b990e9fbbade47ec8076e2151` has run #2232 / `44092982606` IN_PROGRESS; do not claim terminal success yet. Definition of Done remains 3 checked / 10 unchecked; E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED.
- Issue #104: race-resistant filesystem project-root confinement remains OPEN.
- Issue #105: strong Unix escaped-descendant containment remains OPEN. Bounded reader shutdown is not process-tree containment.
- Issue #82: Android AArch64 runtime execution acceptance remains OPEN.

## Live checkpoint — 2026-11-18 (PR #110 exact-SHA reconciliation)

- Canonical `main` remains unchanged at `d35db95153f6ddd79885494c72f5aee16c899751`.
- PR #108 is OPEN / DRAFT / UNMERGED at exact HEAD `286a0fd563e10611a3465cb6457ab069a0df8a52`; GitHub commit status `ci/circleci: one-linux` is SUCCESS, job [99216205-62aa-4e98-af42-d4b742a5feaa](https://app.circleci.com/workflow/a0e85852-848f-4c6b-8681-53438dea6bea/job/99216205-62aa-4e98-af42-d4b742a5feaa).
- PR #110 is OPEN / DRAFT / UNMERGED at exact HEAD `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b`; GitHub commit status `ci/circleci: one-linux` is SUCCESS, job [c9916871-9746-4c65-b8fb-615d9f96dc78](https://app.circleci.com/workflow/476d8936-1898-4406-a0b5-0fc1a61a27c9/job/c9916871-9746-4c65-b8fb-615d9f96dc78), status updated 2026-11-18T03:40:55Z.
- The CircleCI status context is verified from GitHub. Expanded CircleCI job logs were not retrieved in this session; do not claim step-level log inspection.
- Issue #104 remains OPEN. At PR #110 HEAD, `crates/one-core/src/adapter.rs` still implements filesystem read/write through `std::fs::read(path)` and `std::fs::write(path, ...)`; production root-handle confinement is NOT IMPLEMENTED. A green CircleCI status is not Issue #104 acceptance.
- The independent verifier repository's current `main` is `3d915528e7771d9443ccf0da7e4d7c9e25231537`; its current main tree lists three workflows (Linux distro, stdlib-focused, Windows). This is only the verifier main snapshot; do not infer other branches have the same workflow set.
- Full docs inventory at PR #110 HEAD: 206 tracked files total, 103 documentation files including README. This is an inventory count, not proof that all 103 documents have been semantically reviewed.
- The architecture PNG was not visually inspected in this session because the available GitHub file fetch returns text and cannot deliver the binary image to the visual inspection path. Do not claim visual inspection complete.

[Detailed live checkpoint](live-checkpoint-2026-11-18.md).


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
