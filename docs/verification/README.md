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


## Protected verifier promotion completed — 2026-11-19

- With explicit user authorization, PR #44 was merged into the protected `verification` branch. Merge commit: `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`.
- Protected `CANONICAL_COMMIT.txt` now pins `722d662e7a81711ba3607d26571f4048a292b3ed`; the updated E4 workflow and Windows native contract probe are present on the protected branch.
- Pre-merge diagnostic workflow runs are green; post-merge protected push-triggered runs still require independent confirmation. Do not claim those runs green yet.
- ONE PR #113 remains OPEN / DRAFT / UNMERGED. Its exact HEAD `d6b4c4e781a8d1ace85b2657626d4e02295005c8` has CircleCI PENDING (workflow `08114a30-dac4-4ee5-9d4d-dc075ef9adff`, job `ae78409c-7e8f-4063-b0f8-faf0c7eee3d7`). Issue #104 remains OPEN / NOT ACCEPTED.
- Full evidence and limits: [Issue #104 current evidence reconciliation](ISSUE_104_CURRENT_STATUS_2026-11-19.md).


## Protected post-merge E4 verification — 2026-11-19

- Protected branch `verification` received merge commit `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`; canonical source pin is `722d662e7a81711ba3607d26571f4048a292b3ed`.
- Post-merge E4 run #44519577693 is terminal SUCCESS: Unix no-follow/resource bounds, Windows reparse/handle checks, and Windows native `NtCreateFile + OBJ_DONT_REPARSE` probe all passed.
- Separate SEC-05 Linux cross-compiler-install job failed; its Windows job passed. Do not confuse this with the E4 result or claim the full verifier repository push was green.
- PR #113 remains OPEN / DRAFT / UNMERGED; Issue #104 remains OPEN / NOT ACCEPTED pending remaining acceptance-gate reconciliation.
- Detailed exact-SHA evidence: [Issue #104 current evidence reconciliation](ISSUE_104_CURRENT_STATUS_2026-11-19.md).


## Fixed-weight release ledger gate — 2026-11-19

- Protected verifier run #44519577312 remains IN_PROGRESS, checking the fixed-weight ledger against source SHA `722d662...` and ledger/DoD document SHA `d6c382a...`.
- The pinned ledger records 92/100 provisional implementation, 0/100 verified, E6 0/4; 10 Definition of Done items remain unchecked. The gate also requires exact-SHA mirror workflows and a protected package-provenance release run.
- This is a broader final-release gate, not the E4 result. Do not claim final release acceptance while it is in progress.
- Detailed evidence: [Issue #104 current evidence reconciliation](ISSUE_104_CURRENT_STATUS_2026-11-19.md).


## SEC-05 Linux package-install failure classified — 2026-11-19

- SEC-05 run #44519577571, Linux job 135544437757, failed at `Install Linux native cross-compilers`, not in Rust or SEC-05 tests.
- APT update succeeded, but package installation downloaded 194 MB over 11m 9s then failed to fetch remaining packages because the Azure Ubuntu mirror backend `azure.archive.ubuntu.com` (`52.252.75.106:80`) repeatedly timed out/unreachable; APT exit code 100.
- Windows SEC-05 job passed. E4 protected run #44519577693 also passed independently.
- Failed Linux job rerun attempt 2 is in progress as a transient-network confirmation; job 135581237093. Do not claim it passed until terminal.
- Detailed timestamps, exact evidence, and safe remediation boundary are in [Issue #104 current evidence reconciliation](ISSUE_104_CURRENT_STATUS_2026-11-19.md).


## Live continuation refresh — 2026-11-19 13:20Z

- PR #113 exact HEAD `bc0070cbe6ea23b97ff814c145d68c24585a83c5` is CircleCI `ci/circleci: one-linux` SUCCESS (workflow `3d609b9a-fadc-4451-97fd-daf371ae0d4f`, job `8973d4dd-f38f-4bd1-9aa5-f098cfa3c7b0`). This is exact-SHA evidence only.
- SEC-05 rerun attempt 2 Linux job `135581237093` is still IN_PROGRESS at `Install Linux native cross-compilers`; Windows job `135581239271` is SUCCESS. Linux logs are unavailable while the job is active. No SEC-05 Linux pass/fix is established.
- Continue by polling run `44519577571` attempt 2. If it fails, retrieve the terminal logs and determine whether the Azure Ubuntu mirror failure recurred. If it succeeds, record a successful retry on the same pinned candidate, not a permanent remediation.
- Keep PR #113 OPEN / DRAFT / UNMERGED, Issue #104 OPEN / NOT ACCEPTED, and `main` untouched. Do not change protected verifier branch without explicit authorization.


## Protected verifier multi-workflow failure reconciliation — 2026-11-19 13:30Z

### PR #113 exact-SHA CircleCI

- PR #113 remains OPEN / DRAFT / UNMERGED; exact HEAD at this checkpoint: `d3924152f2536492a3117e288f60572f3e97814a`; base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Exact-HEAD CircleCI `ci/circleci: one-linux` = SUCCESS (workflow `21bd7a8f-d992-4b62-8250-a46295ca2ea6`, job `328ed531-0e4a-4160-a0bf-3fb50bbf2478`). This success applies only to `d3924152...`.

### SEC-05 Linux rerun attempt 2 — failure reproduced

- Protected verifier run #44519577571 attempt 2; Linux job `135581237093` FAILED at step 5 `Install Linux native cross-compilers`; Windows job `135581239271` SUCCESS.
- Attempt 2 downloaded 277 MB in 15m19s, then received HTTP 503 Service Unavailable from `security.ubuntu.com` for two remaining MinGW packages. APT ended with exit code 100. This differs from attempt 1's connection/timeouts to Azure backend `azure.archive.ubuntu.com` / `52.252.75.106:80`, but both are package-fetch failures before Rust setup or Linux tests.
- Classification remains hosted Ubuntu mirror/package-service availability failure, not a SEC-05 source/test failure. Two attempts now reproduce the package-fetch problem across distinct endpoints; no Linux test result exists for either attempt.
- Draft PR #46: [SEC-05 bounded APT retry mitigation](https://github.com/panglimanetizen01/one-windows-verification/pull/46), head `8a880ad00225ed8a78a3fd58adfbc329f0df5b11`, base protected merge `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`. It records configured APT sources, applies bounded `Acquire::Retries=3` and 30-second HTTP/HTTPS timeouts to update/install, records package policy, and raises Linux job timeout 30→45 minutes. Full compiler set remains unchanged. No protected branch modification or merge.
- PR #46 pull-request workflow run #44533999931 is IN_PROGRESS: Linux job `135594520678` at bounded APT installation step 5; Windows job `135594521270` at Windows toolchain installation step 2. Do not claim validation until terminal results and logs are read.

### Package provenance verifier — false-positive probe corrected in PR #45

- Original run #44519577562 / job `135544438096`: exact-SHA provenance/CLI tests passed 19/19, then the stale-lockfile probe failed because its `sed -i '/^sha2 = /a regex = "1"'` insertion matched no line in the pinned `crates/one-cli/Cargo.toml` (the manifest has only `one-core` under dependencies). Cargo therefore correctly accepted the unchanged lockfile; the probe had not mutated the manifest.
- Draft PR #45: [fix package provenance and artifact verifier assumptions](https://github.com/panglimanetizen01/one-windows-verification/pull/45), head `dabccd46bb6549e36761a4fa3b02da5d7c84a42e`. The probe now inserts `regex = "1"` after the `[dependencies]` header and asserts that the mutation occurred. Workflow execution is NOT RUN; the workflow is push-triggered on protected `verification`, so PR remains draft pending review/authorized promotion.

### Artifact integrity verifier — stale Rust version expectation corrected in PR #45

- Original run #44519577533 / job `135544437263`: artifact discovery and archive digest verification passed, then embedded metadata validation rejected Rust/Cargo `1.100.0` because the verifier hardcoded `1.99.0`.
- Exact pinned ONE source SHA `722d662e7a81711ba3607d26571f4048a292b3ed` has `rust-toolchain.toml` channel `1.100.0`; actual artifact metadata reports Rust/Cargo `1.100.0`. PR #45 updates the expectation to match the exact source toolchain. Workflow execution remains NOT RUN until authorized promotion to protected branch.

### Health scanner workflow — implementation absent, not a scanner defect proven

- Run #44519577289: Linux job `135544437333` and Windows job `135544437821` both failed at final outcome aggregation.
- Each of three unittest discovery commands reported `Ran 0 tests` / `NO TESTS RAN` (exit code 5). The self-scan command could not open `one-source/tools/one_health.py`; recursive source-tree inspection at the pinned SHA found no health/scanner/collector/quality implementation paths.
- The verifier repository's `docs/CONTINUOUS_REPOSITORY_HEALTH_AND_REPAIR_SYSTEM.md` explicitly labels the system a proposal / implementation plan, not an implemented tool. Do not fabricate implementation, change the workflow to green, or claim scanner tests passed. This is an unimplemented-scope / workflow-fixture alignment blocker requiring a separate design and implementation decision.

### Final release gate

- Fixed-weight ledger gate run #44519577312 / job `135544436703` remains IN_PROGRESS at the exact-SHA ledger/DoD enforcement step. It is not PASS or FAIL. Preserve ledger scores and unchecked DoD items; do not manufacture acceptance.
- Protected E4 run #44519577693 remains independently SUCCESS. Do not call the entire protected verifier push green; E1/stdlib, SEC-05 Linux, provenance, artifact, health scanner, and release gate are distinct workflows/findings.
- Issue #104 remains OPEN / NOT ACCEPTED. PR #113 remains OPEN / DRAFT / UNMERGED. Protected branch `verification` and `main` remain unchanged absent explicit authorization.


## Exact-SHA continuation refresh — 2026-11-19 13:40Z



### PR #113 and CircleCI



- PR #113 remains OPEN / DRAFT / UNMERGED. Exact HEAD before this documentation refresh: 09f043d44099c77ed2cb2d3496190571ab47795e; base 181ad350d8f0db53cda654c4d378425fbdd6ab24.

- CircleCI ci/circleci: one-linux = SUCCESS for exact SHA 09f043d; workflow cffa57d9-b339-432c-8d3f-61a592fc531c; job 25e62c16-b0e2-4282-ae75-eb1f6810c234. This does not transfer to the next docs SHA.



### Draft verifier remediation PRs



- PR #45 — https://github.com/panglimanetizen01/one-windows-verification/pull/45, head dabccd46bb6549e36761a4fa3b02da5d7c84a42e, OPEN / DRAFT. Static diff/readback only; workflow execution NOT RUN. Corrects stale-lock probe that previously inserted after nonexistent sha2 dependency line, and aligns artifact Rust/Cargo expectation from 1.99.0 to exact source pin 1.100.0.

- PR #46 — https://github.com/panglimanetizen01/one-windows-verification/pull/46, head 8a880ad00225ed8a78a3fd58adfbc329f0df5b11, OPEN / DRAFT. Pull-request run #44533999931 COMPLETED / SUCCESS on exact PR head. Linux job 135594520678 SUCCESS; Windows job 135594521270 SUCCESS. Linux APT source diagnostics, bounded retries, complete compiler set, subprocess contract tests, full one-core suite, and static SEC-05 guards passed. Windows toolchain, exact source checkout, Rust 1.100.0, 10 subprocess contract tests, full one-core suite, and Windows descendant guard passed. This validates the mitigation on that runner run only; no permanent upstream mirror guarantee. Whole-repo fmt remains a non-blocking diagnostic with pre-existing diffs.

- PR #47 — https://github.com/panglimanetizen01/one-windows-verification/pull/47, head bd100d10c12c7b0849f01e9481111b0b0c859dd1, OPEN / DRAFT. Pull-request run #44535174043 COMPLETED / SUCCESS. Windows Server 2025 job 135598544938 checked out exact source SHA 722d662e7a81711ba3607d26571f4048a292b3ed, read Rust 1.100.0 from source rust-toolchain.toml, installed MinGW clang, passed PE compiler probe, built and inspected PE32+ AMD64, executed with exit code 0 / output Budi, and wrote exact-SHA manifest. PR branch run is not yet protected-branch evidence.

- All three PRs target protected branch verification but remain draft/unmerged. No protected branch mutation or merge was performed.



### E1 / standard-library failure classification



- E1 Windows run #44519577311, job 135544437394: failed at cargo --locked test -p one-core --test stdlib_builtins; Cargo reports no test target named stdlib_builtins and lists actual targets (conformance, core_pipeline, database_vertical_slice, etc.).

- E1 cross-platform run #44519577315: Linux job 135544437616 and Windows job 135544437995 failed at the same nonexistent stdlib_builtins test target.

- Focused standard-library/database run #44519577640, job 135544438572: several filtered --lib invocations returned success with 0 passed / 148 filtered out (no matching tests), then the UTC integration step failed because stdlib_builtins does not exist.

- Exact source tree at 722d662 has no crates/one-core/tests/stdlib_builtins.rs target. crates/one-core/src/stdlib.rs currently implements only display/list append/list remove; expected UTC/JSON/text/integer standard-library tests and features are not present in that file/tree.

- Do not change these workflows to --lib merely to make them green; that would mask missing acceptance coverage. The correct next step is to implement the required E1 APIs and tests in a separately scoped ONE source change, then update verifier tests to target real, named tests. Until then E1 is NOT accepted.



### Windows native runtime exact-SHA gate blocker



- Fixed-weight ledger gate run #44519577312 remains IN_PROGRESS at job 135544436703, timeout budget 180 minutes; its live evidence poll waits up to 10,200 seconds (170 minutes).

- The required workflow ONE Windows Native Backend Runtime — Windows Server 2025 had no run for canonical source SHA 722d662: its old workflow read WINDOWS_SOURCE_COMMIT.txt = a10a69afdf8675e703f38f1620abc935d8361897 and triggered only on that file or its own workflow. The gate resolves evidence through CANONICAL_COMMIT.txt = 722d662, so this workflow could not satisfy the exact-SHA requirement and remained pending.

- PR #47 changes the workflow to trigger/read CANONICAL_COMMIT.txt, adds PR validation, and derives Rust toolchain from the exact checked-out source. Its PR-run SUCCESS validates the fix but is not included in the protected branch's branch=verification run list. After authorized merge, the protected push should generate the exact-SHA run; then the gate must be rerun/evaluated on terminal evidence.

- Other required exact-SHA mirror workflows on verifier commit a44c32e are terminal: E1 Windows #44519577311 FAILURE, E1 cross-platform #44519577315 FAILURE, focused stdlib #44519577640 FAILURE, artifact #44519577533 FAILURE, SEC-05 #44519577571 FAILURE; Linux distro #44519577570, Cargo lockfile #44519577555, subprocess resource guard #44519577526, E4 #44519577693, cross-platform #44519577496, and native ARM64 runtime #44519577666 SUCCESS. Thus even after the missing Windows native run becomes terminal, the gate cannot pass while the above failures and other acceptance requirements remain unresolved.



### Unchanged acceptance boundaries



- Protected E4 run #44519577693 remains SUCCESS, scoped to its stated threat boundary.

- Issue #104 remains OPEN / NOT ACCEPTED. PR #113 remains OPEN / DRAFT / UNMERGED. Main and protected verifier branch remain unchanged absent explicit authorization.

- Preserve exact-SHA claims, fixed ledger weights, DoD unchecked items, fail-closed platform behavior, and full compiler architecture coverage. Do not transfer PR-branch success into protected-branch acceptance.


## Correction and final scope reconciliation — 2026-11-19 13:52Z

### Corrected E1 classification

- The prior note that expected UTC/JSON/text/integer standard-library APIs were missing required E1 features is superseded. The normative docs/spec/one-v0.1-stdlib.md explicitly scopes v0.1 to display, greeting, empty-list creation, list append/remove, arithmetic, and comparison phrases; it warns against adding operations based only on helper signatures. UTC/JSON/text-prefix/suffix/checked-remainder are not required v0.1 APIs.
- E1 verifier failures were stale test assumptions: stdlib_builtins target does not exist; focused --lib filters returned 0 tests matched. Correct action is align workflows to documented v0.1 behavior and existing test targets, not implement unapproved language features.
- Draft PR #48 https://github.com/panglimanetizen01/one-windows-verification/pull/48 head 0fcbbd60e7c97a64cdcb1931b50e16c7cc00ff79. Focused Linux run #44536641377 SUCCESS: stdlib 6/6, core pipeline 6/6, conformance 5/5, DB unit 1/1, DB vertical slice 9/9, full one-core lib 148/148, contract checker PASS. E1 Windows run #44536641371 SUCCESS: stdlib 6/6, core pipeline 6/6, conformance 5/5, full one-core lib 133/133, contract checker and Clippy PASS. Cross-platform run #44536641375: Windows job 135603697996 SUCCESS; Ubuntu job 135603697243 still IN_PROGRESS at native cross-compiler installation. Keep PR draft until Linux terminal result.

### Corrected PR #45 scope

- PR #45 https://github.com/panglimanetizen01/one-windows-verification/pull/45 head 9a72e04b7e2992d655f8e3fc4e478892e91b7ae5 is artifact-integrity-only; current diff changes only candidate-artifact-inspection.yml. Exact-head run #44538012071 SUCCESS; job 135608513172 verified source SHA 722d662, Linux/Windows archive digests, embedded Rust/Cargo 1.100.0, ELF64 x86-64 and PE32+ x86-64.
- The package-provenance signing lab is explicitly excluded from PR #45. The normative v0.1 package/build spec states the full provenance envelope is not implemented. The lab references nonexistent examples/package-release fixture and unsupported sign-package/verify-package commands; it is a future/experimental workflow and not an Issue #104 gate. The attempted stale-lock probe modification was reverted from PR #45. Do not claim package signing/provenance is implemented or validated.

### PR #46 and PR #47 validation

- PR #46 https://github.com/panglimanetizen01/one-windows-verification/pull/46 head 8a880ad00225ed8a78a3fd58adfbc329f0df5b11: run #44533999931 SUCCESS on Linux and Windows; bounded APT retries, full compiler set, subprocess contract tests, full one-core suites, and static guards passed. Draft/unmerged.
- PR #47 https://github.com/panglimanetizen01/one-windows-verification/pull/47 head bd100d10c12c7b0849f01e9481111b0b0c859dd1: run #44535174043 SUCCESS; exact canonical source SHA 722d662, Rust 1.100.0, MinGW clang, PE32+ AMD64, native Windows runtime exit 0/output Budi. Draft/unmerged; PR-branch run is not protected-branch evidence.

### PR #113 and ledger gate

- PR #113 remains OPEN / DRAFT / UNMERGED at pre-refresh HEAD f68246eefed45a45f78f0c43a29d9a55ca7dce99; CircleCI was PENDING immediately after that docs SHA and must be polled again after this documentation update.
- Fixed-weight gate #44519577312 remains IN_PROGRESS waiting for a protected exact-SHA Windows Native Backend run. PR #47 fixes the trigger/source-pin mismatch, but its PR-branch SUCCESS is not visible to the gate's branch=verification query until authorized protected promotion. Other required workflows still include the SEC-05 and artifact/E1 failures addressed by PRs #45/#46/#48.
- Issue #104 remains OPEN / NOT ACCEPTED. Protected E4 #44519577693 is independently SUCCESS; do not claim the entire verifier push is green.

### Current safe boundary

- PRs #45–#48 remain OPEN / DRAFT / UNMERGED. No protected verification branch or main change has been made. No merge authorization inferred.
- Preserve exact-SHA evidence, fixed ledger weights, unchecked DoD, fail-closed behavior, and full compiler target coverage.
