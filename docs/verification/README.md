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
- PR #110 is OPEN / DRAFT / UNMERGED at current exact HEAD `26d29c61944e51919161f5360bccfef4a09d370d`; GitHub commit status `ci/circleci: one-linux` is SUCCESS, job [c153ba39-4073-4548-aaa6-6370e6bed6bf](https://app.circleci.com/workflow/a15e7c3f-0492-4ac6-a025-d8d2fe31f054/job/c153ba39-4073-4548-aaa6-6370e6bed6bf), status updated 2026-11-18T04:42:11Z. Its parent code-bearing SHA is `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b`; source findings below refer to that parent and remain applicable because the child commit changes verification documentation only.
- The CircleCI status context is verified from GitHub. Expanded CircleCI job logs were not retrieved in this session; do not claim step-level log inspection.
- Issue #104 remains OPEN. At PR #110 HEAD, `crates/one-core/src/adapter.rs` still implements filesystem read/write through `std::fs::read(path)` and `std::fs::write(path, ...)`; production root-handle confinement is NOT IMPLEMENTED. A green CircleCI status is not Issue #104 acceptance.
- The independent verifier repository's current `main` is `3d915528e7771d9443ccf0da7e4d7c9e25231537`; its current main tree lists three workflows (Linux distro, stdlib-focused, Windows). This is only the verifier main snapshot; do not infer other branches have the same workflow set.
- Full docs inventory at PR #110 HEAD: 206 tracked files total, 103 documentation files including README. This is an inventory count, not proof that all 103 documents have been semantically reviewed.
- The architecture PNG was not visually inspected in this session because the available GitHub file fetch returns text and cannot deliver the binary image to the visual inspection path. Do not claim visual inspection complete.

[Detailed live checkpoint](live-checkpoint-2026-11-18.md).




## Live CircleCI diagnostics checkpoint — 2026-11-19

- PR #113 is OPEN / DRAFT / UNMERGED at exact HEAD `e8dab3a0ac2de6b09b374b692d740b9fb5630f00`, base branch `fix/windows-process-verifier-timeouts-20261118` at base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Exact-HEAD GitHub commit status: `ci/circleci: one-linux` PENDING. Current job: [CircleCI exact-HEAD job](https://app.circleci.com/workflow/e55fc480-62fb-469f-8351-deff706595ac/job/69d31382-c676-47f8-92b3-98c0dd8cbf1e). No job logs were retrievable through the available connectors; do not infer a failure cause or PASS.
- CircleCI diagnostics utility now fails closed on pagination truncation and malformed pagination shapes, distinguishes an exact-SHA pipeline with no failed jobs from an API error, and has 11 unit-test methods. Test execution and `circleci config validate` are NOT VERIFIED.
- Rust toolchain provisioning has explicit command deadlines (5m bootstrap / 10m install) and a 12m CircleCI no-output guard.
- Independent Windows verifier PR #44 remains OPEN; its protected canonical pin remains `e50748a1b25922d9090eba1f5fb90548d7c7d003`. This is a separate evidence boundary from CircleCI.
- Issue #104 remains OPEN / NOT ACCEPTED. Do not merge PR #113, modify `main`, or change the protected verifier pin.

## Canonical records

- [CircleCI targeted diagnostics tool and usage](CIRCLECI_DIAGNOSTICS.md)

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

## Live verification checkpoint — 2026-11-18 (Linux + Windows baseline PASS)

- Independent verifier Linux-only run [44267886845](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44267886845), job `134676844501`, passed on exact source SHA `a20676be53e5f78c465832379d86c3675a2ed7c8`: 239 passed, 0 failed, 1 ignored across 27 test-result summaries.
- Independent verifier Windows job [44263674485](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44263674485/job/134662858839) passed on the same exact source SHA: 232 passed, 0 failed, 1 ignored across 27 test-result summaries.
- This closes the Linux runner bootstrap blocker for the independent baseline candidate. The original matrix run's Linux job remains stale/in-progress at the original MinGW step; use the separate Linux-only run as the passing Linux evidence.
- PR #110 remains OPEN/DRAFT/UNMERGED at `2ab82ea9cccb1c9aa0fef25c3591a1310ab84b5c`; its CircleCI `one-linux` status is independently SUCCESS. This is not the same SHA as the independent Windows/Linux candidate.
- Issue #104 remains OPEN. Production `FileReadAdapter` and `FileWriteAdapter` still use ambient `std::fs::read/write`; root-handle confinement, resource limits, and adversarial tests remain unimplemented. E4, DoD, release and security acceptance remain unchanged.
- Next: preserve this exact-SHA evidence; proceed with a compilable, narrowly scoped Issue #104 implementation using an available validated Rust runner. Do not copy the historical PR #80 Windows path-based approach or commit uncompiled OS-handle code.
