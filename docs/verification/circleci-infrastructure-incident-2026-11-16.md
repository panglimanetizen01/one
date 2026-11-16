# CircleCI Infrastructure Incident Record — 2026-11-16

**Scope:** forensic record for Project ONE hosted-CI infrastructure. This record does not change ONE language behavior, grammar, semantics, compiler, runtime, CLI, or user experience.

## Executive classification

| Evidence | Classification | What it proves |
|---|---|---|
| Historical CircleCI job #254, one-windows, pipeline #201 | **FAILED — MSYS2 first-run/keyring initialization** | The Windows job started and failed during MSYS2 package database synchronization before ONE source verification. |
| CircleCI job #426, pipeline #290 | **PLAN-CREDIT-BLOCKED — explicit provider message** | The v1.1 job response explicitly reports `free-plan-no-credits-available`; no runner execution or source/test result is demonstrated. |
| CircleCI job #407, pipeline #277 | **PLAN-CREDIT-BLOCKED — explicit provider message** | The v1.1 job response for UUID `dde778de-e8d9-48ff-b2ac-18b9aa2aa04f` explicitly reports `free-plan-no-credits-available`; no runner execution or source/test result is demonstrated. |
| Jobs #408–#409 (pipeline #278) | **BLOCKED — pre-run/runner not started; cause unknown** | Usage records show 0 seconds, 0 credits, NULL executor/resource class; no explicit no-credit message has been supplied for these two jobs. |
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

### Job #407 — explicit no-credit block

- Job UUID: `dde778de-e8d9-48ff-b2ac-18b9aa2aa04f`
- Pipeline #277 / UUID `37b9eaf1-5259-4aaa-9f32-f6ff4b2f3481`
- The newly supplied CircleCI v1.1 response likewise reports reason `free-plan-no-credits-available`. No executable build/test step ran.

Classification: **PLAN-CREDIT-BLOCKED**. This is job-level evidence and is not inferred from #424 or #426.

### Jobs #408–#409 — cause remains unconfirmed

Usage-export rows for #408 and #409 show zero elapsed seconds, zero credits, and NULL executor/resource class; v2 details expose no steps. No explicit no-credit message has been supplied for these two jobs. Keep them **RUNNER-NOT-STARTED / BLOCKED — cause unknown** until their own provider responses identify a cause.

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

For direct job metadata and step output, use:

- #407: dde778de-e8d9-48ff-b2ac-18b9aa2a04f
- #408: c70d0a4a-b681-49a7-a510-e185a6e0f92e
- #409: a55423d0-6ec9-4047-b3e9-2812dd31c4e4
- #426: 16aedf05-149f-4c3e-9d25-888d1af8fe10

```sh
circleci job get <JOB_UUID> --json
circleci job output list <JOB_UUID> --json
```

The CLI must be installed and authenticated; the existing CIRCLE_TOKEN environment variable is supported. If a failure report returns no output, inspect run status and job details first. The command only returns failed-step context for completed failed runs; it cannot manufacture logs for jobs with no recorded steps. Preserve output/errors exactly, redact secrets, and do not rerun while credits are unavailable or before a plausible transient cause is established.

CircleCI's Cloud job-status documentation defines Failed as a job that failed or terminated for unknown reasons. This reinforces that a failed status alone does not prove a source-code failure. See [Jobs and steps](https://circleci.com/docs/guides/orchestrate/jobs-steps/), [failure-report guide](https://circleci.com/docs/guides/toolkit/diagnose-failed-runs-with-failure-report/), and [CLI reference](https://cli.circleci.com/reference/).


## 8. Correction from additional CircleCI v1.1 responses — 2026-11-16

The user supplied the complete job metadata response for #407 and #426 after the earlier incident record was written. These responses supersede the earlier statement that their root causes were unknown:

- **#407** (`dde778de-e8d9-48ff-b2ac-18b9aa2aa04f`, pipeline #277): explicit `free-plan-no-credits-available` message.
- **#426** (`16aedf05-149f-4c3e-9d25-888d1af8fe10`, pipeline #290): explicit `free-plan-no-credits-available` message.
- **#424**: remains independently and explicitly credit-blocked.
- **#408 and #409**: remain pre-run / runner-not-started with cause unknown; do not infer their cause from the other jobs.

The #426 response's `circle_yml` is the current Linux-only workflow: it defines `one-linux` and `release-linux`; workflow `one-ci` runs only `one-linux`. It contains no `one-windows`, `release-windows`, or MSYS2 installation step. This confirms that #426 did not execute the historical Windows/MSYS2 job. The current config blob on `ci/circleci-migration-20261113` is `35eed93039992108041e0982f233eac138735df2`.

No CI tests ran in #407 or #426. These are plan-credit blockers, not source-code failures. No rerun, paid upgrade, billing change, merge, or product-code modification is authorized or performed. CircleCI has no connected execution tool in this session; its hosted dashboard/API cannot be operated directly here. The repository evidence and Skill checkpoint are being synchronized without exposing credentials or presigned output URLs.
