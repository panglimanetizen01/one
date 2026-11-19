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


## Live CircleCI checkpoint — 2026-11-19 (status refresh)

- PR #113 remains OPEN / DRAFT / UNMERGED. Exact PR HEAD at this readback: `6f1c2fe41fcd101349f31275340ad58a7a7a78e1`; base SHA: `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- The preceding code/config SHA `e8dab3a0ac2de6b09b374b692d740b9fb5630f00` has `ci/circleci: one-linux` = PENDING, workflow `e55fc480-62fb-469f-8351-deff706595ac`, job `69d31382-c676-47f8-92b3-98c0dd8cbf1e`: [CircleCI job](https://app.circleci.com/workflow/e55fc480-62fb-469f-8351-deff706595ac/job/69d31382-c676-47f8-92b3-98c0dd8cbf1e).
- The exact PR HEAD `6f1c2fe41fcd101349f31275340ad58a7a7a78e1` independently has `ci/circleci: one-linux` = PENDING, workflow `67615aef-2786-409c-a60e-b95f8d3a5360`, job `4eae19a5-f0bc-47e5-95a6-0e24f1074558`: [CircleCI job](https://app.circleci.com/workflow/67615aef-2786-409c-a60e-b95f8d3a5360/job/4eae19a5-f0bc-47e5-95a6-0e24f1074558).
- These are separate exact-SHA executions. Do not transfer status between code/config and documentation commits.
- Current connected tools expose no CircleCI-specific connector, CLI action, or provider job-log reader. The actual job logs remain inaccessible; no failure cause is inferred.
- The 11 diagnostic unit tests and `circleci config validate` remain NOT VERIFIED. Do not report PASS until both commands execute against an identified exact revision with captured output.
- PR #44 remains OPEN / DRAFT / UNMERGED; protected verifier pin `e50748a1b25922d9090eba1f5fb90548d7c7d003` unchanged. Issue #104 remains OPEN / NOT ACCEPTED. Do not merge, modify `main`, or change the protected pin.
- This section supersedes older active-status statements in this index; earlier entries remain historical records.


## Independent verifier evidence reconciliation — 2026-11-19

### Diagnostic pin vs protected pin (do not conflate)

- PR #44 uses the separate verifier branch `probe/windows-obj-dont-reparse-20261118`, whose `CANONICAL_COMMIT.txt` is `722d662e7a81711ba3607d26571f4048a292b3ed`.
- The protected verifier branch `verification` still has `CANONICAL_COMMIT.txt` = `e50748a1b25922d9090eba1f5fb90548d7c7d003`. It was not changed. PR #44 remains OPEN / DRAFT / UNMERGED.

### Exact-source evidence on diagnostic candidate 722d662...

- E4 adversarial workflow [44335797344](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797344): success. Unix job [134912035794](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797344/job/134912035794) and Windows job [134912036561](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797344/job/134912036561) both checked out and printed exact `source_sha=722d662e7a81711ba3607d26571f4048a292b3ed`; each completed successfully.
- Windows native API probe job [134912035140](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797344/job/134912035140) succeeded on Windows Server 2025 Datacenter build 26100 / PowerShell 7.6.9. Ordinary file opens with both flags; direct junction and intermediate-junction opens with `OBJ_DONT_REPARSE` returned `0xC000050B`; outside sentinel remained unchanged; terminal marker `OBJ_DONT_REPARSE_ROOT_DIRECTORY_JUNCTION_DENIAL_OK` was emitted.
- Cross-platform workflow [44335797200](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797200) succeeded on Ubuntu and Windows jobs; distro workflow [44335797294](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797294) succeeded on Debian 12 and Ubuntu 24.04; Android AArch64 build [44335797320](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797320) and API 35 ARM64 runtime [44335797240](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797240) succeeded; SEC-05 workflow [44335797365](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797365) succeeded.
- Candidate `722d662...` is an ancestor of current PR #113 HEAD. Compare reports 53 commits from `722d662...` to `c25d191...`; intervening changed paths are CI configuration, verification/docs, and CircleCI diagnostic tooling only, with no intervening runtime implementation path changes. This supports continued relevance of the scoped runtime evidence, but does NOT make later SHAs tested or grant them PASS status.

### CircleCI exact-SHA reconciliation

- Candidate `722d662...`: `ci/circleci: one-linux` FAILURE [job](https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94).
- Code/config SHA `e8dab3a0ac2de6b09b374b692d740b9fb5630f00`: `ci/circleci: one-linux` FAILURE [job](https://app.circleci.com/workflow/e55fc480-62fb-469f-8351-deff706595ac/job/69d31382-c676-47f8-92b3-98c0dd8cbf1e). Provider logs are still inaccessible; cause unknown.
- PR #113 HEAD at the time of this readback `c25d191148091e7f62566d0290a6b71004fa798a`: `ci/circleci: one-linux` PENDING [job](https://app.circleci.com/workflow/35bda6e3-acf8-4232-9437-608467816602/job/90ae8601-d89f-4314-a346-d37089540363).
- The 11 diagnostic unit tests and `circleci config validate` remain NOT VERIFIED. Do not infer the CircleCI failure cause or transfer status between SHAs.


### Post-reconciliation documentation HEAD status — 2026-11-19

- After the evidence-reconciliation documentation commits, PR #113 HEAD `2cfd73f9e54dac283fb5c07bb85f5992e88ef506` received `ci/circleci: one-linux` = PENDING; workflow `e2e1cab5-28cf-4829-aafc-a1d385abace9`; job `7e872036-ca77-4f15-a66f-e97e5a058788`: [exact-HEAD job](https://app.circleci.com/workflow/e2e1cab5-28cf-4829-aafc-a1d385abace9/job/7e872036-ca77-4f15-a66f-e97e5a058788).
- This status belongs only to exact SHA `2cfd73f...`; subsequent documentation commits require a new status read. CircleCI provider logs remain inaccessible.

## Direct CircleCI job-log retrieval — 2026-11-19

`tools/ci/diagnose_circleci.py` already supports direct API v3 inspection by CircleCI job UUID. The exact known failed jobs are documented in `CIRCLECI_DIAGNOSTICS.md`; the direct commands are recorded there to avoid re-discovery. Local `CIRCLECI_TOKEN` is required and must never be pasted into chat or committed.

- `722d662...` failed job UUID: `9f3acc50-11b9-47e5-b841-6a943300be94`.
- `e8dab3a...` failed job UUID: `69d31382-c676-47f8-92b3-98c0dd8cbf1e`.
- PR #113 latest docs HEAD at this read: `5bfdd712fdbc6cb688908786758d9d262d190b40`; combined status returned no contexts yet. This is inconclusive, not PASS/FAIL.
- Current validation state: 11 diagnostic unit tests NOT VERIFIED; `circleci config validate` NOT VERIFIED; provider failure logs not yet retrieved.


## Direct CircleCI job lookup safety correction — 2026-11-19

The direct `--job-id` mode of `tools/ci/diagnose_circleci.py` bypasses exact-HEAD pipeline discovery and does not itself assert the job's source SHA. Before attributing direct-ID logs to a commit, verify the workflow/pipeline revision in CircleCI and record that SHA with the report. Copy-ready UUIDs, the latest pending job, and validation commands are maintained in `CIRCLECI_DIAGNOSTICS.md`.

- Latest previously observed PR #113 HEAD `bbe5d696ddfe5358932cd4351d5cd0af6070b4a1`: `ci/circleci: one-linux` PENDING, workflow `dc55d982-a016-44a9-8f74-116a8e2c77ff`, job `954d8930-d569-4b1e-9eab-87945b8b50c6`.
- The diagnostic documentation code fences were corrected to render as actual Markdown code blocks.
- Do not infer the latest status for a newer SHA from this prior readback. Poll the exact current PR HEAD.
- Unit tests and CircleCI config validation remain NOT VERIFIED; no failure root cause is claimed without logs.


## Live continuation refresh — 2026-11-19 (direct job identity guard)

- PR #113 pre-change HEAD `9965ce53f816662d52074d81c9fcab4ba70e7d5f`: exact-HEAD CircleCI `ci/circleci: one-linux` PENDING; workflow `72f890ca-d9c2-4441-b743-49b7ad24cd90`; job `cd298634-331a-4032-9540-a05e688245b2`. This status does not transfer to subsequent commits.
- The diagnostic tool now prints `SOURCE SHA: UNVERIFIED` when `--job-id` is used, because that path bypasses exact-SHA pipeline discovery. The normal exact-SHA path prints its resolved source SHA. Two regression tests were added; suite now contains 13 test methods.
- Test execution and `circleci config validate .circleci/config.yml` remain NOT VERIFIED. CircleCI provider logs remain inaccessible; root cause unknown.
- PR #113 and PR #44 remain OPEN / DRAFT / UNMERGED; Issue #104 OPEN / NOT ACCEPTED; protected verifier pin remains `e50748a1b25922d9090eba1f5fb90548d7c7d003`. No merge, main change, force-push, or pin change.

## Latest exact-HEAD refresh — 2026-11-19

- Previously observed PR #113 HEAD `f92a4795fc96b0d7780bda150249c5005d98505d` had `ci/circleci: one-linux` PENDING (workflow `bf515c38-2b62-46e3-bf7e-cc2f07093f5f`, job `c6e87d6e-5f3c-4f61-8ab2-4e7b9b58b1f9`). This is historical after the documentation synchronization commit; never transfer this status to another SHA.
- Diagnostic suite now contains 13 test methods, including direct-job source identity warnings and exact-SHA identity output. Execution remains NOT VERIFIED.
- CircleCI provider logs remain inaccessible; root cause unknown. `circleci config validate .circleci/config.yml` remains NOT VERIFIED.
- PR #113 remains OPEN / DRAFT / UNMERGED; PR #44 remains OPEN / DRAFT / UNMERGED; Issue #104 remains OPEN / NOT ACCEPTED; protected verifier pin remains unchanged.

## CircleCI pagination hardening — 2026-11-19

- Removed the arbitrary 10-page cap from exact-SHA pipeline discovery. The diagnostic now follows all API v2 continuation tokens until the endpoint ends pagination and detects repeated tokens to prevent infinite loops.
- Added regression coverage for a 12-page result and repeated-token cycle; diagnostic suite now has 14 test methods. The updated suite has not yet been executed on the new HEAD.
- See `CIRCLECI_DIAGNOSTICS.md` for the implementation details and exact-SHA validation command. Previous CI/test evidence does not transfer to the new commits.


## Linux cross-toolchain timeout remediation — 2026-11-19

- Exact-SHA diagnostics on `72bd068e33045c95d042c225c0bdfd27ea4df57f` identified job `one-linux` #730, failed step #105, “Install Linux and Windows-target cross-toolchains (Linux host)”.
- The step exited 124 while running `apt-get install` under an explicit 15-minute deadline. The log showed successful 66.1 MB download and package setup/trigger processing; package download failure was not indicated.
- On PR #113, the bounded apt install deadline was increased to 25 minutes and its CircleCI no-output guard to 30 minutes. Toolchain package selection and platform coverage were not reduced.
- The remediation is pending a fresh exact-SHA CircleCI execution. Do not transfer prior statuses or claim PASS until the new run reaches a terminal state and its failed/successful steps are checked.

## Linux cross-toolchain timeout follow-up — 2026-11-19

- Exact source SHA: `bb35f1e4777a1a2f078c4f88ad7f16d3dbad9dc2`; CircleCI job `one-linux` #734, UUID `9c6733a3-98ee-4c97-a5be-52022440ca6d`, workflow `408d665d-ebcc-4f8b-9c93-20ede933e2d3`.
- Exact-SHA diagnostic confirmed failed step #105, `Install Linux and Windows-target cross-toolchains (Linux host)`, and confirmed source SHA equality.
- The failed command was `timeout 25m sudo apt-get ... install`; stderr ends with `Exited with code exit status 124`. The last 100 stdout lines show clang, MinGW x86_64 POSIX GCC, binutils, and dependencies unpacked/configured, followed by man-db and libc-bin trigger processing. No package-resolution or download error appears in the captured tail.
- Evidence-based classification: the explicit 25-minute GNU timeout expired. This is not evidence of a compiler/test failure or CircleCI no-output timeout; the step's configured no-output guard was 30 minutes.
- Remediation on the PR branch: increase the apt install command deadline from 25m to 40m and the step no-output guard from 30m to 45m. Keep the package set and `--no-install-recommends` unchanged. This preserves Linux-host Windows cross-compilation capability while allowing more time for dpkg setup/triggers.
- The 14 diagnostic unit tests passed on the pre-remediation exact SHA `bb35f1e...`; they do not verify the changed CircleCI configuration. The local CircleCI CLI validator remains unusable on Termux due its previously observed SIGSYS/faccessat2 crash; config validation is NOT VERIFIED.
- Next gate: sync the new config/docs HEAD in Termux, run the 14 tests on that exact HEAD, then inspect its own CircleCI status and logs. Do not transfer results from `bb35f1e...`.
- PR #113 remains OPEN / DRAFT / UNMERGED; PR #44 remains OPEN / DRAFT / UNMERGED; Issue #104 remains OPEN / NOT ACCEPTED; protected verifier pin remains unchanged. No merge, main modification, force-push, or protected-pin change.


## CircleCI provisioning incident — 2026-11-19

- Exact failed source SHA `69e66846a67f5d79044329ccf407838f454d24bf`: CircleCI job `one-linux` #740, step #105, cross-toolchain provisioning; explicit `timeout 40m` ended with exit 124.
- APT downloads and package setup output completed, but the parent install command did not return before its deadline. The precise blocking package hook/process remains unknown.
- Instrumented split-install config was committed as `84028a01fff08325db365a131a26a34c32690d62`; it separates Clang and MinGW installs and captures dpkg/process diagnostics on failure. Validation is pending on the new exact SHA.
- Full incident evidence and follow-up: [CIRCLECI_FAILURE_69E668_2026-11-19.md](CIRCLECI_FAILURE_69E668_2026-11-19.md) and [CircleCI config review](CIRCLECI_CONFIG_REVIEW_2026-11-19.md).


## Clang package-group timeout — exact SHA 26fcd96 — 2026-11-19

- CircleCI job #747 / `9666c56b-20cc-4be8-b08c-6547c4a467ab`, source SHA `26fcd96d53e19b4218e5b1e5926020a8326fb09b`, failed step #105.
- The Clang/binutils APT group ran for exactly 1,500 seconds and exited 124. Package configuration and man-db/libc-bin triggers completed within seven seconds of the group start; the parent apt command then remained alive until the 25-minute deadline. MinGW did not start.
- The exact blocking wait point remains UNKNOWN. The post-timeout process listing is not evidence of the pre-timeout process state.
- Follow-up instrumentation commit `60a8a20ca92dd4d55ce062f08708250eb1cb6317` captures live process state every 120 seconds and APT terminal/history logs while preserving all compiler packages. It is not yet validated; inspect the new exact-SHA CircleCI run before claiming resolution.
- See [the detailed failure report](CIRCLECI_FAILURE_69E668_2026-11-19.md).


- Instrumentation refinement commit: `5c0b45a9dd8cbab4814f7935eacd08cecea17c88`. The live process snapshot now filters on the process `comm` field (avoids matching the enclosing shell's embedded script text) and emits a real newline after each captured command line. This is a logging-quality correction only; no package, timeout, or compiler behavior changed.


## Latest Issue #104 evidence reconciliation — 2026-11-19

- Current PR #113 exact HEAD `70ae27cd9dab4429c528d6f01a42377f474c725c`: CircleCI `ci/circleci: one-linux` SUCCESS (workflow `44163dc3-9fe7-41d1-a199-f1327b7d4055`, job `5a6562fb-9501-40fa-a49c-05966cf2a66b`).
- Independent E4 and platform runs on code-bearing source `722d662e7a81711ba3607d26571f4048a292b3ed` are terminal SUCCESS. GitHub compare confirms the 103-commit range to `70ae27c...` changed only CircleCI configuration, docs, and diagnostic tooling—not runtime implementation or runtime tests. Exact run IDs, platform limits, and acceptance boundaries are recorded in [Issue #104 current evidence reconciliation](ISSUE_104_CURRENT_STATUS_2026-11-19.md).
- The APT timeout was not reproduced in the successful run; its root cause remains undetermined because the success path deletes the monitor log. Do not claim a root-cause fix.
- Issue #104 remains OPEN / NOT ACCEPTED; PR #113 and PR #44 remain OPEN / DRAFT / UNMERGED; protected verifier pin remains unchanged. Linux nested writes and unsupported platforms remain fail-closed.
