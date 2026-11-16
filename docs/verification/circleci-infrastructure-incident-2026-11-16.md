# CircleCI Infrastructure Incident Record — 2026-11-16

**Scope:** forensic record for Project ONE hosted-CI infrastructure. This record does not change ONE language behavior, grammar, semantics, compiler, runtime, CLI, or user experience.

## Executive classification

| Evidence | Classification | What it proves |
|---|---|---|
| Historical CircleCI job #254, one-windows, pipeline #201 | **FAILED — MSYS2 first-run/keyring initialization** | The Windows job started and failed during MSYS2 package database synchronization before ONE source verification. |
| CircleCI job #426, pipeline #290 | **BLOCKED / INCONCLUSIVE — runner/task did not start** | Zero build time, same start/stop timestamp, no executable command, no steps, and no source/test result. Root cause is not established. |
| Jobs #407–#409 (pipelines #277–#278) | **BLOCKED — pre-run/runner not started; cause unknown** | Usage records show 0 seconds, 0 credits, NULL executor/resource class; job API exposed no steps. Do not infer a specific cause. |
| Job #424 | **BLOCKED — explicit no-credit message** | CircleCI explicitly reported that no plan credits were available for that job. This does not prove the cause of #426 or #407–#409. |
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

## 2. Runner-not-started jobs #426 and #407–#409

### Job #426

- Job UUID: 16aedf05-149f-4c3e-9d25-888d1af8fe10
- Workflow UUID: 7e8e12ac-11be-4503-bcfc-c22a50f2874b
- Pipeline #290 / UUID 76ef6e84-7795-4519-bb09-0a38d74e8ece
- Branch: ci/circleci-migration-20261113
- Exact SHA: e92d3a3003b68283d5f0fe1315dcb41dcc33bd62
- Job: one-linux
- Lifecycle finished with failed status, but build time was 0 ms; start and stop timestamps were identical; messages were empty; no command/exit code or executable steps were available. The temporary output endpoint returned 404.

Classification is **runner/task did not start — root cause unknown**. The evidence is insufficient to call this a source-code failure or to claim that the job failed because of credits.

### Jobs #407–#409

Usage-export rows for these jobs show zero elapsed seconds, zero credits, and NULL executor/resource class. The v2 job details returned no steps. They are pre-run records, not executed test failures. A no-credit message on job #424 is separate evidence and must not be generalized to these jobs without a matching job-level message or authoritative account evidence.

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
