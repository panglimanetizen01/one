# CircleCI Infrastructure Incident Record — 2026-11-16

**Scope:** forensic record for Project ONE hosted-CI infrastructure. This record does not change ONE language behavior, grammar, semantics, compiler, runtime, CLI, or user experience.

## Executive classification

| Evidence | Classification | What it proves |
|---|---|---|
| Historical CircleCI job #254, one-windows, pipeline #201 | **FAILED — MSYS2 first-run/keyring initialization** | The Windows job started and failed during MSYS2 package database synchronization before ONE source verification. |
| CircleCI job #426, pipeline #290 | **PLAN-CREDIT-BLOCKED — explicit provider message** | The v1.1 job response explicitly reports `free-plan-no-credits-available`; no runner execution or source/test result is demonstrated. |
| CircleCI job #409, pipeline #278 | **PLAN-CREDIT-BLOCKED — explicit provider message** | The v1.1 job response for UUID `dde778de-e8d9-48ff-b2ac-18b9aa2aa04f` explicitly reports `free-plan-no-credits-available`; no runner execution or source/test result is demonstrated. |
| Jobs #407–#408 | **BLOCKED — pre-run/runner not started; cause unknown** | Available records show no executed steps; their individual root causes remain unconfirmed. Do not infer their cause from the separate no-credit responses for #409, #424, or #426. |
| Job #424 | **PLAN-CREDIT-BLOCKED — explicit provider message** | CircleCI explicitly reported that no plan credits were available for this job; classify it independently. |
| Independent Windows verifier run #21 | **SUCCESS — exact-SHA and platform-bounded** | Windows Server 2025 verification passed for ONE source SHA f85375e279257dcbb7d4ded9632800fcd1b78cc9; it does not verify later SHAs. |

## 1. Historical Windows job #254 — confirmed failure cause

- Repository: panglimanetizen01/one
- Pipeline: #201; workflow: one-ci
- Job: #254, one-windows
- Job UUID: 5a0d780e-0521-4a90-80da-9293718266de
- Exact source revision: e44d16b2b5f4ef50c33674ccf711bd11b410effa
- Commit subject: Record official CircleCI executor schema correction
- Execution interval: 2026-11-13 11:20:07.713Z–11:21:42.608Z (94.894 seconds)
- Environment: Windows Server 2025 GUI image windows-server-2025-gui:2026.10.1
- Rust stable installation succeeded (rustc 1.100.0, cargo 1.100.0).
- Failure occurred in “Install MSYS2 and MinGW-w64”, exit code 1.

The job extracted the MSYS2 SFX archive to C:\msys64, then invoked pacman -Syu --noconfirm without first initializing the extracted MSYS2 environment through its login shell. The log reported a missing public keyring, unknown signing key 5F944B027F7FE2091985AA2EFA11531AA0AA7F57, a non-writable keyring, and PGP signature/database synchronization failure.

**Root cause:** MSYS2 first-run/keyring initialization sequence was incomplete. This is a Windows toolchain bootstrap failure, not a Rust compilation failure and not a ONE compiler, grammar, runtime, CLI, or product-test failure.

Official guidance:
- [MSYS2 — Using MSYS2 in CI](https://www.msys2.org/docs/ci/): after SFX extraction, initialize with C:\msys64\usr\bin\bash -lc ' ', then perform the documented update sequence (including the required repeated pacman --noconfirm -Syuu update pass).
- [MSYS2 — Installer](https://www.msys2.org/docs/installer/): extracted installations require first-run shell initialization.
- [MSYS2 — Updating](https://www.msys2.org/docs/updating/): follow the official keyring/update recovery guidance if signature errors persist; do not disable signature verification.

**Disposition:** historical finding closed as diagnosed; no fix is to be applied to current ONE CI because the current canonical CircleCI configuration no longer defines this Windows-host/MSYS2 job. Windows host verification was separated into panglimanetizen01/one-windows-verification. Do not reintroduce MSYS2 into the ONE CircleCI workflow merely to repair this superseded job.

## 2. Credit-blocked and pre-run jobs

### Job #426 — explicit no-credit block

- Job UUID: `16aedf05-149f-4c3e-9d25-888d1af8fe10`
- Pipeline #290 / UUID `76ef6e84-7795-4519-bb09-0a38d74e8ece`
- Branch: `ci/circleci-migration-20261113`
- Exact SHA: `e92d3a3003b68283d5f0fe1315dcb41dcc33bd62`
- Job: `one-linux`
- The newly supplied CircleCI v1.1 response contains the explicit message `This job has been blocked because no credits are available on your plan` and reason `free-plan-no-credits-available`. `start_time` is null and the only synthetic step is `Error: Task information unavailable`; no source/test command ran.

Classification: **PLAN-CREDIT-BLOCKED**. This is now confirmed for #426; the earlier “root cause unknown” wording is superseded.

### Job #409 — explicit no-credit block

- Job UUID: `dde778de-e8d9-48ff-b2ac-18b9aa2aa04f`
- Pipeline #278 / UUID `3b065bab-76cf-4fa2-8a81-de878e7d5993`
- The newly supplied CircleCI v1.1 response for pipeline #278 (the one-linux run whose preceding same-job build was #407) reports reason `free-plan-no-credits-available`. No executable build/test step ran.

Classification: **PLAN-CREDIT-BLOCKED**. This is job-level evidence and is not inferred from #424 or #426.

### Jobs #407–#408 — cause remains unconfirmed

Usage-export rows for #407 and #408 show zero elapsed seconds, zero credits, and NULL executor/resource class; v2 details expose no steps. No explicit no-credit message has been supplied for these two jobs. Keep them **RUNNER-NOT-STARTED / BLOCKED — cause unknown** until their own provider responses identify a cause.

### Job #424 — separate explicit no-credit block

Job #424 independently reported `free-plan-no-credits-available`. Its message applies to #424 only and is not used as evidence for other jobs.

## 3. Exact-SHA Windows success boundary

Independent Windows verifier run #21 (43667599852) succeeded on Windows Server 2025 AMD64 for verifier revision 9c40fffdaca6125561320a16475515e82923e9e8, with its manifest pinning ONE source revision f85375e279257dcbb7d4ded9632800fcd1b78cc9. The native executable exited 0 and produced the recorded output Budi; artifact digests were checked.

This success is valid only for the pinned source SHA and recorded environment. It must not be transferred to the later CircleCI migration branch SHA or to a different source revision.

## 4. CircleCI usage and trigger observations

- Project ID: b89bb4bd-9c27-4f83-85f1-9e9f1bbb41c1; organization ID: b513967e-8024-435a-ba08-ea0f98e88401.
- The 2026-11-09 through 2026-11-15 usage export contained both ONE and Axion Studio rows; only records matching the ONE project were used for ONE conclusions.
- One executed failed Windows job (#254) consumed 64 credits over 95 seconds on windows.medium; it is materially different from zero-second/zero-credit pre-run records.
- Pipelines #268–#278 were 11 distinct webhook-triggered commits in approximately 94 seconds. Commit inspection found separate documentation-only changes; no duplicate webhook loop was established.
- Insights aggregates and usage exports do not prove the current remaining plan balance. CircleCI Plan → Plan Usage is the authoritative UI for the available credit balance.

## 5. Required interpretation and future handling

1. Classify every CI outcome using job-level evidence: executed source/test failure, toolchain/bootstrap failure, runner-not-started blocker, or plan-credit blocker.
2. A failed status alone is not evidence of a source-code defect.
3. Zero elapsed time, zero credits, NULL executor, and no steps indicate no demonstrated execution; preserve the root cause as unknown unless CircleCI supplies explicit evidence.
4. Never infer the cause of one job from another job's message.
5. Bind every verification claim to the exact source SHA, verifier SHA, platform, and run identifier.
6. Do not change ONE product semantics or user-facing behavior to accommodate CI infrastructure.
7. Do not purchase or upgrade a paid plan without the user's explicit decision. Use the independent verification repository where applicable.
8. If MSYS2 is ever intentionally used by a future Windows toolchain job, follow the official MSYS2 CI initialization/update sequence and retain signature verification.

## 6. Source-of-truth links

- [ONE repository](https://github.com/panglimanetizen01/one)
- [Independent Windows verification repository](https://github.com/panglimanetizen01/one-windows-verification)
- [MSYS2 CI documentation](https://www.msys2.org/docs/ci/)
- [MSYS2 installer documentation](https://www.msys2.org/docs/installer/)
- [MSYS2 updating documentation](https://www.msys2.org/docs/updating/)

## 7. Follow-up diagnostic route — official CircleCI CLI

The official CircleCI CLI supports consolidated failed-step output for completed failed runs. It reads the CIRCLE_TOKEN environment variable when set; do not paste tokens into chat or print them.

Run-level reports to retrieve:

- Pipeline #278: 3b065bab-76cf-4fa2-8a81-de878e7d5993 (jobs #408 and #409)
- Pipeline #277: 37b9eaf1-5259-4aaa-9f32-f6ff4b2f3481 (job #407)
- Pipeline #290: 76ef6e84-7795-4519-bb09-0a38d74e8ece (job #426)

```sh
circleci run get 3b065bab-76cf-4fa2-8a81-de878e7d5993 --failure-report
circleci run get 37b9eaf1-5259-4aaa-9f32-f6ff4b2f3481 --failure-report
circleci run get 76ef6e84-7795-4519-bb09-0a38d74e8ece --failure-report
```

For direct job metadata and step output, use the job UUID returned by the live CircleCI details response for the specific job number. A previously copied job-number-to-UUID list in this record was internally inconsistent with later v1.1 responses and is withdrawn; do not use that stale mapping for lookups.

```sh
circleci job get <JOB_UUID> --json
circleci job output list <JOB_UUID> --json
```

The CLI must be installed and authenticated; the existing CIRCLE_TOKEN environment variable is supported. If a failure report returns no output, inspect run status and job details first. The command only returns failed-step context for completed failed runs; it cannot manufacture logs for jobs with no recorded steps. Preserve output/errors exactly, redact secrets, and do not rerun while credits are unavailable or before a plausible transient cause is established.

CircleCI's Cloud job-status documentation defines Failed as a job that failed or terminated for unknown reasons. This reinforces that a failed status alone does not prove a source-code failure. See [Jobs and steps](https://circleci.com/docs/guides/orchestrate/jobs-steps/), [failure-report guide](https://circleci.com/docs/guides/toolkit/diagnose-failed-runs-with-failure-report/), and [CLI reference](https://cli.circleci.com/reference/).


## 8. Correction from additional CircleCI v1.1 responses — 2026-11-16

Additional CircleCI v1.1 responses supplied after the earlier incident record was written establish the following classifications and supersede the earlier uncertainty where explicitly stated:

- **#409** (`dde778de-e8d9-48ff-b2ac-18b9aa2aa04f`, pipeline #278): explicit `free-plan-no-credits-available` message.
- **#426** (`16aedf05-149f-4c3e-9d25-888d1af8fe10`, pipeline #290): explicit `free-plan-no-credits-available` message.
- **#424**: remains independently and explicitly credit-blocked.
- **#407 and #408**: remain pre-run / runner-not-started with cause unknown; do not infer their cause from the other jobs.

The #426 response's `circle_yml` is the current Linux-only workflow: it defines `one-linux` and `release-linux`; workflow `one-ci` runs only `one-linux`. It contains no `one-windows`, `release-windows`, or MSYS2 installation step. This confirms that #426 did not execute the historical Windows/MSYS2 job. The current config blob on `ci/circleci-migration-20261113` is `35eed93039992108041e0982f233eac138735df2`.

No CI tests ran in #409 or #426. These are plan-credit blockers, not source-code failures. No rerun, paid upgrade, billing change, merge, or product-code modification is authorized or performed. CircleCI has no connected execution tool in this session; its hosted dashboard/API cannot be operated directly here. The repository evidence and Skill checkpoint are being synchronized without exposing credentials or presigned output URLs.


## 9. Free-plan recovery research — 2026-11-16

### Verified account and repository observations

- GitHub repository metadata checked during the current continuation (2026-11-16) reports `panglimanetizen01/one` visibility as **public**. This is a read-only observation; this work did not change repository visibility. The earlier private-visibility statement in this incident record is superseded.
- Historical CircleCI job metadata for the blocked runs reports `oss: false`. That value applies to those job records and does not establish the current CircleCI project's open-source eligibility, activation state, or available credits.
- CircleCI's published Free Plan has separate private/personal and open-source credit allocations. Public visibility alone must not be treated as proof that the CircleCI project is currently enrolled in or receiving the open-source allocation.
- CircleCI warns that public projects/build logs are public. Treat this as an ongoing disclosure boundary; never print secrets in build logs or enable secrets for forked PRs without a deliberate security review.
- Before making any current credit/billing recommendation, inspect the CircleCI project's present classification and Plan Usage directly. Historical job metadata and repository visibility are not substitutes for that live account state.

Official sources:
- https://circleci.com/docs/guides/plans-pricing/credits/
- https://circleci.com/pricing/
- https://circleci.com/docs/guides/integration/oss/

### Free recovery options, ordered by preserving repository privacy

1. **Owner-authorized private-repository option — wait for the next Free Plan monthly credit refill.** GitHub currently reports ONE as PUBLIC. If the owner separately decides to change it to PRIVATE, the Free Plan personal/private allocation may provide a hosted CircleCI path after the actual monthly refill; this would be a privacy decision requiring explicit owner authorization, not a routine CI fix. The next actual refill date must be read from CircleCI Plan → Plan Usage/Plan Overview; do not assume the calendar-month boundary if the account UI shows a different billing cycle.
2. **Owner-authorized private-repository option — self-hosted runner after at least one credit is available.** This also requires an explicit owner decision to make the repository PRIVATE if private-build behavior is desired. CircleCI says runner compute itself does not consume credits, but the organization must have at least one credit and storage/network usage may consume credits. The owner must supply and maintain supported hardware. Supported machine runner platforms include Windows Server 2019+ x86_64 and Linux x86_64/ARM64; Android/Termux is not a documented supported platform. This is a possible future option only if the owner has an always-on supported Windows/Linux host.
3. **Opt into CircleCI open-source allocation only if the owner deliberately approves public visibility.** This is the strongest no-cost hosted CircleCI option for Linux, but it requires the repository to be public and means project/build logs are public. This is a privacy decision, not an automatic CI fix. No visibility change has been made.
4. **Use local/independent test evidence while waiting.** Local builds or the independent Windows verifier can establish evidence for their exact tested SHA, but cannot turn a CircleCI job that never ran into CircleCI PASS. Do not forge statuses or relabel BLOCKED as PASS.

### Cost and trigger optimization

- Current canonical config on `ci/circleci-migration-20261113` uses `machine.image: android:default` and `resource_class: large` for `one-linux`. CircleCI documents Android machine images and recommends `large` or above for Android-related workloads. The current configuration uses the Android SDK/NDK but does not launch an emulator in the shown product CI steps.
- CircleCI's price list reports Linux VM Gen1 `large` at 20 credits/minute and `medium` at 10 credits/minute; Docker x86 `small` is 5 credits/minute. Do not change the Android job to a smaller class blindly: first measure peak RAM/CPU and run a controlled exact-SHA trial when credits are available. If a standard Linux VM replaces the Android image, explicitly provision and pin Android command-line tools/NDK; do not assume the Android SDK remains preinstalled.
- The usage export showed 11 distinct documentation-only commits triggering 11 pipelines in about 94 seconds on the migration branch. This was not a duplicate webhook loop, but it is avoidable CI spend. Keep documentation-only work on the dedicated docs branch/PR #97 rather than pushing it to the CircleCI migration branch. For future docs-only commits on a CI-enabled branch, use a supported CI-skip commit marker only when no product/CI source has changed, or introduce carefully tested path-based workflow filtering.
- Avoid duplicate full-workflow triggers, opt-in release jobs, repeated cache misses, and unnecessary artifact retention. Keep release checks opt-in.

### Self-hosted runner constraints

CircleCI's self-hosted runner docs state that the organization must have at least one credit; runner execution itself does not use compute credits, but storage/network may. Public projects cannot use self-hosted runners while forked-PR builds are enabled. Sources:
- https://circleci.com/docs/guides/execute/run-jobs-on-self-hosted-runners/
- https://circleci.com/docs/guides/execution-runner/runner-overview/
- https://circleci.com/pricing/price-list/

### Explicit non-actions

No repository visibility change, paid plan/credit purchase, billing mutation, fake status, source-code change, CircleCI rerun, or merge was performed. No CircleCI PASS can be claimed for #407, #408, #409, #424, or #426: those jobs remain historical blocked/pre-run records. A new, eligible run on an exact SHA is required to produce a real PASS.
