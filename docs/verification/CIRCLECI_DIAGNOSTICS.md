# CircleCI targeted diagnostics for Project ONE

## Purpose

Use the CircleCI API to identify the failed job and retrieve only the failed step's stdout/stderr. The default mode matches the current local Git HEAD to the pipeline revision before selecting jobs. Direct `--job-id` mode bypasses that discovery and now prints `SOURCE SHA: UNVERIFIED` in the report until the workflow/pipeline revision is independently checked.

## Prerequisites

- Python 3 (standard library only; no pip install required).
- A CircleCI personal API token available locally as CIRCLECI_TOKEN (or CIRCLE_TOKEN).
- The token must have access to the panglimanetizen01/one CircleCI project.

Do not paste the token into chat, save it in the repository, or print it in terminal output.

## Usage

From the root of the ONE repository:

```bash
python3 tools/ci/diagnose_circleci.py
```

The default mode reads the current branch and exact git rev-parse HEAD, finds the CircleCI pipeline for that exact revision, discovers failed/timed-out jobs, and prints only their failed-step logs.

Inspect a known job UUID directly:

```bash
python3 tools/ci/diagnose_circleci.py --job-id 9f3acc50-11b9-47e5-b841-6a943300be94
```

**Important:** `--job-id` bypasses exact-HEAD pipeline discovery. Before treating its output as evidence for a particular source revision, independently verify the job's workflow/pipeline source SHA in the CircleCI UI and record that SHA beside the report. The utility currently does not assert an expected SHA in direct-ID mode.

Save a redacted report:

```bash
python3 tools/ci/diagnose_circleci.py --output /tmp/one-circleci-diagnostic.txt
```

Override the exact revision or branch when diagnosing a historical run:

```bash
python3 tools/ci/diagnose_circleci.py --sha COMMIT_SHA --branch BRANCH_NAME
```

## API behavior

- The Linux job installs the exact Rust release from rust-toolchain.toml in a dedicated step. Rustup bootstrap and toolchain installation have explicit wall-clock limits, separate from the cross-toolchain package provisioning step; the no-output timeout is 12 minutes, deliberately longer than the 10-minute install deadline so the command timeout can fire and report its exit before CircleCI's inactivity guard.
- CircleCI API v2 is currently documented by CircleCI as legacy but supported; the pipeline/workflow/job discovery endpoints used here are not part of the announced February 1, 2027 retirement, which targets nine API v1/v1.1 trigger and rerun endpoints only. Do not perform a speculative v2-to-v3 migration based on that v1/v1.1 deadline. Reassess v3 endpoint coverage for exact project + revision discovery separately; use v3 where the required semantics are documented and tested, and retain v2 where v3 does not yet provide an equivalent.
- CircleCI API v3 is current and recommended for new integrations; it is Cloud-only, uses UUID resource IDs and opaque cursor pagination. API v2 remains the supported fallback for existing integrations and CircleCI Server.
- CircleCI API v3 is used for job details and per-step stdout/stderr.
- API v2 uses the Circle-Token header; API v3 uses Authorization: Bearer.
- The script prints step numbers, execution indexes, outcome, and a bounded tail of each failed step's logs.
- Secret-like values are redacted before display and before writing an optional report.
- A missing stderr stream is normal; CircleCI returns no content when a step has no stderr.
- Pagination follows CircleCI API v2 `next_page_token` values until the API returns no next token. Previously seen continuation tokens are rejected to prevent an infinite loop; valid result sets are not truncated by an arbitrary page-count cap.
- Pagination responses are schema-checked: `items` must be an array of objects and `next_page_token` must be null or a non-empty string. Malformed responses stop discovery as API/shape errors rather than being treated as an empty result.
- If no pipeline exists for the exact local SHA, the script stops rather than selecting a nearby or newer commit.
- If an exact-SHA pipeline exists but no failed/timed-out job is listed, the utility reports that state as inconclusive (running, successful, or no-job workflow); it does not label it a diagnostic/API error or claim PASS. Confirm workflow status in CircleCI.

## Tests

Run the standard-library unit tests:

```bash
python3 -m unittest discover -s tools/ci -p 'test_*.py' -v
```

The tests cover failed-step extraction (including parallel executions), timeout/compiler/test classification, secret redaction, complete and malformed multi-page discovery (including 12+ pages and repeated-token rejection), exact-SHA pipeline discovery with no failed jobs, and source-identity reporting for direct job-ID versus exact-SHA lookup. The current suite contains 14 test methods.

## Limitations

The classification is a log-based hint, not a guaranteed root-cause analysis. The utility does not rerun, cancel, approve, or modify CircleCI jobs. It only reads pipeline/job metadata and logs.

## Known Project ONE job IDs — 2026-11-19

The following CircleCI UUIDs are already resolved. Use `--job-id` to inspect a job directly without pipeline/branch discovery. Direct-ID mode does not validate the source SHA; cross-check the job's workflow/pipeline revision in CircleCI before attributing logs to a commit.

| Source SHA | Workflow UUID | Job UUID | Last observed status |
| --- | --- | --- | --- |
| `722d662e7a81711ba3607d26571f4048a292b3ed` | `9fb2893b-4373-4ef1-ad97-60b804ebedda` | `9f3acc50-11b9-47e5-b841-6a943300be94` | FAILURE |
| `e8dab3a0ac2de6b09b374b692d740b9fb5630f00` | `e55fc480-62fb-469f-8351-deff706595ac` | `69d31382-c676-47f8-92b3-98c0dd8cbf1e` | FAILURE |
| `424f7bbc73901c5e4342eb04444eaabb526b711a` | `141f741c-74b9-4b36-96e4-9f81d76143cb` | `09b422cb-8c1a-47c5-a252-3974c649022c` | PENDING at last readback (historical) |
| `bbe5d696ddfe5358932cd4351d5cd0af6070b4a1` | `dc55d982-a016-44a9-8f74-116a8e2c77ff` | `954d8930-d569-4b1e-9eab-87945b8b50c6` | PENDING at historical readback (2026-11-19) |
| `9965ce53f816662d52074d81c9fcab4ba70e7d5f` | `72f890ca-d9c2-4441-b743-49b7ad24cd90` | `cd298634-331a-4032-9540-a05e688245b2` | PENDING at latest readback (2026-11-19) |
| `f92a4795fc96b0d7780bda150249c5005d98505d` | `bf515c38-2b62-46e3-bf7e-cc2f07093f5f` | `c6e87d6e-5f3c-4f61-8ab2-4e7b9b58b1f9` | PENDING at latest exact-HEAD readback (2026-11-19) |

Run the two known failed jobs directly (after setting `CIRCLECI_TOKEN` locally):

```bash
python3 tools/ci/diagnose_circleci.py --job-id 9f3acc50-11b9-47e5-b841-6a943300be94 --output /tmp/one-circleci-722d662.txt
python3 tools/ci/diagnose_circleci.py --job-id 69d31382-c676-47f8-92b3-98c0dd8cbf1e --output /tmp/one-circleci-e8dab3a.txt
```

The latest pending job can be inspected directly once it reaches a terminal state:

```bash
python3 tools/ci/diagnose_circleci.py --job-id cd298634-331a-4032-9540-a05e688245b2 --output /tmp/one-circleci-current-head.txt
```

Use the step number from the job metadata and inspect only the first failed step before interpreting downstream failures. Do not enable blanket reruns or add retries until the logs establish a transient failure mode. A successful rerun of the same exact SHA without code/config changes is evidence of a potentially transient failure; a repeatable failure requires a targeted fix.

## Local validation commands

Run these from the repository root on the exact checked-out SHA:

```bash
python3 -m unittest discover -s tools/ci -p 'test_*.py' -v
circleci config validate .circleci/config.yml
```

Record `git rev-parse HEAD` alongside both command outputs. Do not report either validation as passed unless the commands actually execute and exit successfully.


## Direct job source-identity output hardening — 2026-11-19

- Static review found that direct `--job-id` mode correctly bypasses exact-SHA discovery but did not surface that evidence limitation in the generated report itself.
- Updated `tools/ci/diagnose_circleci.py` to print `SOURCE SHA: UNVERIFIED` for direct-ID inspection, with an explicit instruction to verify the workflow/pipeline revision before attributing logs. Exact-SHA discovery mode prints the resolved commit SHA.
- Added two regression tests for the direct-ID and exact-SHA report identity lines. Test count is now 13 methods; execution remains NOT VERIFIED until run on the exact candidate SHA.
- Updated this guide's purpose, known-job ledger, and current pending job command. Latest pre-change source SHA was `9965ce53f816662d52074d81c9fcab4ba70e7d5f`; the new commit will require its own CircleCI status.


## Latest exact-HEAD refresh — 2026-11-19

- PR #113 HEAD `f92a4795fc96b0d7780bda150249c5005d98505d` remains OPEN / DRAFT / UNMERGED. Its base SHA remains `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Exact-HEAD status `ci/circleci: one-linux` = PENDING; workflow `bf515c38-2b62-46e3-bf7e-cc2f07093f5f`; job `c6e87d6e-5f3c-4f61-8ab2-4e7b9b58b1f9`: https://app.circleci.com/workflow/bf515c38-2b62-46e3-bf7e-cc2f07093f5f/job/c6e87d6e-5f3c-4f61-8ab2-4e7b9b58b1f9.
- The previous `9965ce53...` status is historical and must not be transferred to this SHA.
- Unit tests and CircleCI config validation remain NOT VERIFIED; CircleCI provider logs remain inaccessible and failure root cause unknown.

## Pagination completeness fix — 2026-11-19

- Exact-SHA pipeline discovery now follows CircleCI API v2 `next_page_token` values until the API returns no next token; the former arbitrary 10-page stop has been removed.
- The paginator tracks previously used continuation tokens and fails closed if CircleCI repeats a token, preventing an infinite loop without silently truncating a valid result set.
- Regression coverage now includes a 12-page response and a repeated-token cycle.
- Validate on the exact branch HEAD with:

```bash
python3 -m unittest discover -s tools/ci -p 'test_*.py' -v
python3 tools/ci/diagnose_circleci.py \
  --sha e7014092a01caebf201e43ca9a7ef3bf63e8270f \
  --branch security/issue-104-root-confinement-20261118 \
  --output "$HOME/one-circleci-exact-sha.txt"
```

The SHA above is the diagnostic target that exposed the pagination limit, not the new code HEAD. After this fix is committed, use the new exact HEAD SHA for any CI attribution. Store reports under `$HOME` on Termux; `/tmp` may be unavailable or read-only.


## Linux cross-toolchain provisioning timeout — 2026-11-19

- Exact source SHA: `72bd068e33045c95d042c225c0bdfd27ea4df57f`.
- Failed job: `one-linux`, job number `730`, UUID `a693b817-a36a-45fa-b7da-1fd0a31b95ac`; failed step `#105`, “Install Linux and Windows-target cross-toolchains (Linux host)”.
- The failed step's stderr ends with `Exited with code exit status 124`. Its command used `timeout 15m` for `apt-get install`, while CircleCI's inactivity guard was `20m`. Exit 124 is consistent with the explicit GNU `timeout` deadline expiring.
- The supplied log shows 66.1 MB fetched successfully in 3 seconds and package unpack/setup progressing through man-db and libc-bin trigger processing. This does not indicate a package download or dependency-resolution failure; apt did not return successfully before its command deadline.
- Remediation committed on the existing PR #113 branch: raise the bounded install deadline from 15m to 25m and the step's no-output timeout from 20m to 30m. The package selection and Linux-host Windows cross-compilation capability are unchanged.
- This is an evidence-based timeout adjustment, not proof that the next execution will pass. The new exact SHA requires its own CI result and log inspection.


## Linux cross-toolchain timeout follow-up — 2026-11-19

- Exact source SHA: `bb35f1e4777a1a2f078c4f88ad7f16d3dbad9dc2`; CircleCI job `one-linux` #734, UUID `9c6733a3-98ee-4c97-a5be-52022440ca6d`, workflow `408d665d-ebcc-4f8b-9c93-20ede933e2d3`.
- Exact-SHA diagnostic confirmed failed step #105, `Install Linux and Windows-target cross-toolchains (Linux host)`, and confirmed source SHA equality.
- The failed command was `timeout 25m sudo apt-get ... install`; stderr ends with `Exited with code exit status 124`. The last 100 stdout lines show clang, MinGW x86_64 POSIX GCC, binutils, and dependencies unpacked/configured, followed by man-db and libc-bin trigger processing. No package-resolution or download error appears in the captured tail.
- Evidence-based classification: the explicit 25-minute GNU timeout expired. This is not evidence of a compiler/test failure or CircleCI no-output timeout; the step's configured no-output guard was 30 minutes.
- Remediation on the PR branch: increase the apt install command deadline from 25m to 40m and the step no-output guard from 30m to 45m. Keep the package set and `--no-install-recommends` unchanged. This preserves Linux-host Windows cross-compilation capability while allowing more time for dpkg setup/triggers.
- The 14 diagnostic unit tests passed on the pre-remediation exact SHA `bb35f1e...`; they do not verify the changed CircleCI configuration. The local CircleCI CLI validator remains unusable on Termux due its previously observed SIGSYS/faccessat2 crash; config validation is NOT VERIFIED.
- Next gate: sync the new config/docs HEAD in Termux, run the 14 tests on that exact HEAD, then inspect its own CircleCI status and logs. Do not transfer results from `bb35f1e...`.
- PR #113 remains OPEN / DRAFT / UNMERGED; PR #44 remains OPEN / DRAFT / UNMERGED; Issue #104 remains OPEN / NOT ACCEPTED; protected verifier pin remains unchanged. No merge, main modification, force-push, or protected-pin change.


## Cross-toolchain timeout diagnosis and instrumentation — 2026-11-19

- Failed exact source SHA: `69e66846a67f5d79044329ccf407838f454d24bf`; job `one-linux` #740 / `5c42627c-b911-45d1-a32a-7cb72826af53`; workflow `one-ci`; failed step #105.
- The command was `timeout 40m sudo apt-get ... install`; CircleCI no-output guard was 45m. The stderr ended with exit status 124. The supplied output showed package downloads and setup/trigger processing, without a dependency-resolution error. The explicit 40-minute command deadline is the high-confidence cause; the exact package hook/process is not yet identified.
- New config commit `84028a01fff08325db365a131a26a34c32690d62` splits Clang/Linux-binutils and x86_64 POSIX MinGW into separate 25-minute installs, with a 30-minute step no-output guard. It sets `DEBIAN_FRONTEND=noninteractive`, emits UTC timestamps and elapsed seconds, and captures dpkg audit, apt/dpkg/debconf processes, and dpkg log tail on failure.
- New exact-SHA config validation and diagnostic test execution are NOT VERIFIED. Read the resulting CircleCI log to identify the package group/process; do not transfer failure or pending status from parent SHA `69e668...`.
- Detailed evidence: [CIRCLECI_FAILURE_69E668_2026-11-19.md](CIRCLECI_FAILURE_69E668_2026-11-19.md).


## Exact-SHA live APT diagnosis — 2026-11-19

- Source SHA `26fcd96d53e19b4218e5b1e5926020a8326fb09b`, job #747, step #105: Clang/binutils install began at 08:47:02 UTC and the package/trigger log ended at 08:47:09 UTC. The wrapper timed out at 09:12:02 UTC with exit 124 after exactly 1,500 seconds.
- MinGW was never reached. The post-timeout dpkg audit showed no pending package output; the process snapshot was after timeout cleanup and therefore cannot identify what APT was waiting on.
- New config commit `60a8a20ca92dd4d55ce062f08708250eb1cb6317` adds a background process-state recorder every 120 seconds (process IDs, parent IDs, state, wait channel, command) and captures `/var/log/apt/term.log` and `history.log` on failure. Package selection and timeout values are unchanged.
- This is instrumentation only. CircleCI execution, config validation, and the 14 diagnostic unit tests on the new commit remain NOT VERIFIED.


- Instrumentation refinement commit: `5c0b45a9dd8cbab4814f7935eacd08cecea17c88`. The live process snapshot now filters on the process `comm` field (avoids matching the enclosing shell's embedded script text) and emits a real newline after each captured command line. This is a logging-quality correction only; no package, timeout, or compiler behavior changed.


## Current exact-HEAD checkpoint — 2026-11-19 16:14Z

- PR #113 HEAD `519f0f6856e0ae6242746581b9579cce33fca8cd`: `ci/circleci: one-linux` SUCCESS; workflow `a3e98250-83bd-4c4a-a72c-cc15b596267b`; job `8ef8576f-999c-4b09-954f-b52aa82f0dd3`.
- The status is verified from GitHub's exact-commit status context. CircleCI provider stdout/stderr is not available through the connected session; do not claim provider-log inspection.
- The configured job includes `python3 -m unittest discover -s tools/ci -p 'test_*.py' -v`. Job-level SUCCESS means no reported job failure, but the exact discovered test count/output is not independently visible.
- Separate `circleci config validate .circleci/config.yml` remains NOT VERIFIED. Do not infer it from the job status.

## Live exact-SHA status refresh — 2026-11-20

This is a later GitHub status readback than earlier dated entries in this document. Historical checkpoints remain unchanged.

- ONE PR #113 is OPEN / DRAFT / UNMERGED. Exact HEAD: `5daf3337b4bcc93fa3ee6072f9633915d5fea85b`; base: `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Code-bearing source: `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. The 16 commits from this source to current PR HEAD are documentation-only; no runtime implementation or test paths changed in that range.
- CircleCI `ci/circleci: one-linux` on exact code source `fb5c8bc...`: SUCCESS (workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`). This is GitHub status-context evidence; provider step logs were not inspected.
- CircleCI `ci/circleci: one-linux` on exact PR HEAD `5daf333...`: FAILURE (workflow `5ed6d9e7-976e-4322-b7ba-6deea915e1a9`, job `8d2d756e-9297-4a00-9a7d-a0c9bd734cba`, job #833). Available diagnostic: step #101 CANCELED, `Task information unavailable`, stdout/stderr empty. Root cause UNKNOWN; do not infer a timeout, infrastructure fault, or source/config regression without provider logs.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`. Its separate `CircleCI Pipeline` status is ERROR at pipeline #399 (`https://app.circleci.com/pipelines/gh/panglimanetizen01/one/399`). Provider logs and cause are UNKNOWN; this status must not be conflated with PR #113's `ci/circleci: one-linux` context.
- Verifier PR #51 remains OPEN / DRAFT / UNMERGED at `6c0979e62eb01e4b929c48b4224a3d4f742bc8af`. Its branch-local source pin is `fb5c8bc...`; 12 associated PR-triggered workflows are terminal SUCCESS. Protected `verification` pin remains `722d662e7a81711ba3607d26571f4048a292b3ed` and is unchanged.
- Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes and unavailable Android/macOS filesystem adapters remain fail-closed. Verifier successes do not constitute Issue #104 acceptance.
- Provider-side CircleCI logs are not accessible through the connected session. Collect job #833 logs by direct job UUID and pipeline #399 logs using the local diagnostic script; verify each run's source revision before attribution. Never expose `CIRCLECI_TOKEN`.
- No merge, force-push, `main` update, protected pin promotion, ledger-weight/DoD change, or blind retry is authorized by this status refresh.



## Exact-SHA pre-execution investigation — pipeline #1201 — 2026-11-20

- PR #113 HEAD: `509b7b197115adb1ba80d730342e9f6fbf981645`.
- Pipeline #1201 UUID `41becc6d-b56e-45e8-b273-629ba08c4390`; workflow `9a87f37e-ebc9-4574-a966-14dc1300e7db`; job #836 UUID `9dbc05b4-a601-4fbc-b6a4-61f64ecc8e9a`; pipeline revision exactly matches the PR HEAD.
- Job `started_at=null`, stopped at the workflow's ~0.5s failure. API v3 step #101 is CANCELED / `Task information unavailable`, stdout/stderr empty. Do not label this a test timeout.
- Source/compiled config were present. SHA-256 source `b7827cd95925851482b833a5f45033e1b26ef0f731857b29e2df0821e08fd155`; compiled `9f3f77d48351a10c3222a4a1b79634d30de29038c562c9c0e548046026b85aee`. Compiled job uses `android:default`, resource class `large`.
- Project Advanced UI shows Free and Open Source ON and Block all new work OFF. Plan screenshot reports 110,089 / 30,000 regular credits used and states OS contributions are not subject to that 30,000 limit; the separate 400,000 OSS pool balance is not visible. Earlier `free-plan-no-credits-available` remains an unresolved provider error, not yet reconciled with the UI evidence.
- Project settings API is HTTP 403 despite Personal API Token `/api/v2/me` and project metadata returning HTTP 200. Do not infer OSS is OFF from the settings endpoint denial.
- A local diagnostic false-positive was identified: the report's explanatory phrase `failed/timed_out` is matched by the classifier, causing a `TIMEOUT / NO-OUTPUT` label. The actual evidence remains canceled step/no job start. Do not treat the label as a confirmed timeout.
- Root cause remains UNKNOWN. Next: provider-side job failure detail / OSS allocation eligibility. No speculative CircleCI config patch or blind rerun.

## GitHub Actions bootstrap workflow correction — 2026-11-20

- PR #115 initial run #2243 failed on both Ubuntu and Windows at Contract artifacts because `tools/check_contract.py` in the PR #113 candidate correctly rejects `.github/workflows/ci.yml`. Build, Clippy, and tests were skipped.
- Corrective commit `3b684acd1f3e68f1b40dd6d600fad0e013044495` updates only the bootstrap workflow: normal PRs retain merge-result checkout; PRs targeting `fix/windows-process-verifier-timeouts-20261118` checkout exact PR head SHA; only the bootstrap PR branch skips the contract checker, while PR #113 retains the full check.
- Run #2244 (`44748865610`) is IN_PROGRESS on this exact commit; do not record PASS until both OS jobs and all non-skipped steps complete successfully.
- PR #115 remains OPEN / UNMERGED. PR #113 remains OPEN / DRAFT / UNMERGED. CircleCI status for PR #115 corrective commit is PENDING at workflow `e6e6a97c-2cda-4a7a-a5ff-3b0c9129d9ec`, job `667d82b1-c817-41f3-91f2-5bd09b55e879`; CircleCI provider logs are not accessible in this session.


## Terminal GitHub Actions bootstrap result — 2026-11-20

- PR #115 corrective workflow commit `3b684acd1f3e68f1b40dd6d600fad0e013044495` completed GitHub Actions run #2244 / `44748865610` with terminal SUCCESS on Ubuntu and Windows. Build, Clippy, workspace tests, and all focused tests passed on both. The bootstrap-only Contract artifacts step was intentionally skipped; no contract-check PASS is claimed for the bootstrap branch.
- CircleCI status on this same SHA is FAILURE at workflow `e6e6a97c-2cda-4a7a-a5ff-3b0c9129d9ec`, job `667d82b1-c817-41f3-91f2-5bd09b55e879`. The CircleCI job page is inaccessible through the connected browser and there is no CircleCI CLI/MCP action in this session; root cause remains UNKNOWN.
- PR #113 docs HEAD `2cbf87ed05277fc6d66088a645f2aea8d0d8ce16` has CircleCI PENDING at last readback, workflow `d924efb4-6927-43e7-85c7-ad38cab7f425`, job `192e4737-7e01-426d-b2f4-80ee80fa48c3`. PR #113 has no GitHub Actions run yet because bootstrap PR #115 remains unmerged.


## Bootstrap PR merge and subsequent CI observation — 2026-11-20

- PR #115 merged successfully to `fix/windows-process-verifier-timeouts-20261118` with merge commit `3adace165890bb69de346880e3c612c5cd6c2ff8`; expected head was `3b684acd1f3e68f1b40dd6d600fad0e013044495`.
- Merge diff adds only `.github/workflows/ci.yml` (87 lines). GitHub Actions PR #115 run #2244 had terminal SUCCESS on Ubuntu and Windows before merge; CircleCI for that PR commit was FAILURE but provider logs were unavailable.
- After merge, CircleCI status on merge commit is PENDING at workflow `509828d0-3d7d-434a-8f00-a58645eabe9e`, job `83e5ce02-f0ee-4d30-a0df-5dbfe5c77fc6`. This is a new exact merge-commit status context, not evidence of a successful job start.
- PR #113 remained OPEN / DRAFT / UNMERGED. The newly installed GitHub Actions workflow had not yet yielded a PR #113 run in the immediate readback. Follow-up documentation-only commit to PR #113 should create a `pull_request` run; verify that exact SHA and all full-contract/build/test steps.
- B-001 remains open. Do not classify pending CircleCI as pass, and do not assume the prior OSS credit error is resolved.


## PR #113 GitHub Actions run #2247 — 2026-11-20

- Exact candidate HEAD: `6ef875291b599ef144dba5333e9a40c455f3e013`. GitHub Actions run #2247 / `44750302610` is QUEUED then RUNNING on this exact SHA; [run #2247](https://github.com/panglimanetizen01/one/actions/runs/44750302610).
- Ubuntu job `136336114373`: runner started, correct exact-head checkout succeeded, Android SDK setup succeeded; currently in `Install cross-compilers (Ubuntu)`. Contract artifacts, Build, Clippy, workspace tests, and focused tests have not yet started.
- Windows 2025 job `136336114808`: runner started, correct exact-head checkout succeeded, Android SDK setup succeeded; currently in `Install cross-compilers (Windows)`. Contract artifacts, Build, Clippy, workspace tests, and focused tests have not yet started.
- Both jobs are still IN_PROGRESS with no failed steps at latest readback. Do not claim any test pass yet.
- CircleCI on this same PR #113 SHA remains PENDING: workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. B-001 remains open until exact CircleCI job start/outcome and cause are established.
- PR #113 remains OPEN / DRAFT / UNMERGED. PR #115 remains merged at `3adace165890bb69de346880e3c612c5cd6c2ff8`.


## Run #2247 progress refresh — 2026-11-20 (latest readback)

- PR #113 remains OPEN / DRAFT / UNMERGED. Current PR HEAD: `796831e9636c4aa322b4d30c4e96796f62bb45ff`. The exact source SHA being executed by GitHub Actions run #2247 / `44750302610` is `6ef875291b599ef144dba5333e9a40c455f3e013`; comparison confirms the three commits between these SHAs modify only the three existing verification documents (10 insertions each), no source or workflow files.
- Ubuntu job `136336114373`: exact checkout and Android SDK setup PASS; `Install cross-compilers (Ubuntu)` remains IN_PROGRESS. Contract artifacts, Build, Clippy, and Tests have not started on Ubuntu.
- Windows 2025 job `136336114808`: exact checkout, Android SDK setup, Windows cross-compilers, Android NDK, and Toolchain PASS; `Contract artifacts` PASS; Build IN_PROGRESS. Clippy and Tests are pending. No failed steps at latest readback.
- Run #2247 remains IN_PROGRESS; do not claim overall PASS. [Run #2247](https://github.com/panglimanetizen01/one/actions/runs/44750302610).
- CircleCI on exact run SHA `6ef875291b599ef144dba5333e9a40c455f3e013` remains PENDING at workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. CircleCI job page is inaccessible via current browser session; no job start, result, or root cause can be inferred. B-001 remains OPEN / ROOT CAUSE UNKNOWN.
- PR #115 is MERGED at `3adace165890bb69de346880e3c612c5cd6c2ff8`. The merge added only `.github/workflows/ci.yml` to the verifier base branch.
- Prior historical snapshots above are not current status; this progress refresh supersedes them. No PR #113 merge, main change, verifier pin change, or acceptance/ledger/DoD change.


## Run #2247 progress update — Windows build/clippy passed — 2026-11-20

- Run #2247 / `44750302610` remains IN_PROGRESS on source SHA `6ef875291b599ef144dba5333e9a40c455f3e013`.
- Windows 2025 job `136336114808`: checkout, Android SDK, Windows cross-compilers, Android NDK, Toolchain, Contract artifacts, Build, and Clippy are SUCCESS; Tests are IN_PROGRESS. All focused tests are pending. No failed step.
- Ubuntu job `136336114373`: checkout and Android SDK are SUCCESS; `Install cross-compilers (Ubuntu)` remains IN_PROGRESS. Contract artifacts, Build, Clippy, and Tests are pending. No failed step.
- The job log-download endpoint returned BlobNotFound/404 for both running jobs; this is a log retrieval limitation, not evidence that the jobs failed. Step summaries remain available and show the states above.
- Exact current PR #113 HEAD is `07745db32bb1051cc02e602c64f27bc14649238b`. Comparing from tested SHA `6ef875291b599ef144dba5333e9a40c455f3e013` shows six commits ahead, with only the three verification documentation files modified (21 insertions each; no source/workflow changes). The run therefore covers the same source/workflow state as the current PR head.
- CircleCI on tested SHA remains PENDING: workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. B-001 remains OPEN / ROOT CAUSE UNKNOWN.
- PR #115 remains MERGED; PR #113 remains OPEN / DRAFT / UNMERGED. No source change, main change, verifier pin change, or acceptance/ledger/DoD change.


## Run #2247 — Windows matrix leg terminal PASS — 2026-11-20

- Windows Server 2025 job `136336114808` is terminal SUCCESS. All workflow steps succeeded: checkout, Android SDK, Windows cross-compilers, Android NDK, Toolchain, Contract artifacts, Build, Clippy, workspace Tests, deterministic .oneir/.onepkg, DNS classification, orchestration policy, database vertical slice, HTTP GET vertical slice, HTTP GET CLI vertical slice, and cleanup. Job logs were retrieved and show the focused database tests (9/9), HTTP GET vertical slice tests (3/3), and HTTP GET CLI tests (3/3) passing.
- Ubuntu job `136336114373` remains IN_PROGRESS in `Install cross-compilers (Ubuntu)`; all later stages are pending. No failure is recorded. Its log download currently returns BlobNotFound/404 while the job is active, so no cause for the long-running step is inferred.
- Run #2247 / `44750302610` remains IN_PROGRESS overall until Ubuntu completes. [Run #2247](https://github.com/panglimanetizen01/one/actions/runs/44750302610).
- Tested source SHA remains `6ef875291b599ef144dba5333e9a40c455f3e013`; latest PR #113 HEAD `70409c742d2707b1f277b5185787e2e7bdb7184f` differs only in the three existing verification docs (21 insertions each, no source/workflow changes).
- CircleCI on tested SHA remains PENDING at workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. B-001 remains OPEN / ROOT CAUSE UNKNOWN.
- PR #115 MERGED; PR #113 OPEN / DRAFT / UNMERGED. No main/source/verifier pin/acceptance/ledger/DoD change.


## CircleCI status correction — exact tested SHA success — 2026-11-20 21:17 WIB

- Fresh GitHub combined-status readback for exact source SHA `6ef875291b599ef144dba5333e9a40c455f3e013` now reports context `ci/circleci: one-linux` = `success`, target workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. This supersedes the earlier PENDING observation for that exact SHA. The CircleCI status check is successful; detailed CircleCI pipeline/job metadata and underlying log were not independently retrieved in this session, so do not infer details beyond the successful check.
- Current PR #113 HEAD at readback: `441df4590ba6d356671408371a5fc5b87739f4a7`. Comparing tested SHA to current HEAD shows 12 documentation-only commits touching only the existing `ONE_RELEASE_BLOCKER_REGISTER.md`, `live-checkpoint-2026-11-20.md`, and `CIRCLECI_DIAGNOSTICS.md` (42 insertions each; no runtime source or workflow files). GitHub combined-status query for current docs HEAD returned no status entries. Therefore CircleCI success is proven for tested source SHA, not a new CircleCI check on latest docs-only HEAD.
- GitHub Actions run #2247 / `44750302610`: Windows job `136336114808` terminal SUCCESS; Ubuntu job `136336114373` remains IN_PROGRESS at `Install cross-compilers (Ubuntu)`; all following Ubuntu steps pending. Run remains IN_PROGRESS overall, with no recorded failure. Do not claim full matrix success.
- PR #113 remains OPEN / DRAFT / UNMERGED. PR #115 remains MERGED at `3adace165890bb69de346880e3c612c5cd6c2ff8`. No merge, main change, source/workflow modification, verifier pin change, or acceptance/ledger/DoD change.


## B-001 operational resolution — 2026-11-20

- Historical CircleCI pipeline #1201 on SHA `509b7b197115adb1ba80d730342e9f6fbf981645` remains a confirmed pre-execution termination; provider root cause remains unknown.
- Exact SHA `6ef875291b599ef144dba5333e9a40c455f3e013` subsequently received `ci/circleci: one-linux = success`, workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. Compare from original failed SHA to tested SHA shows only the three existing verification docs changed; runtime source and CI workflow are unchanged.
- B-001 is RESOLVED OPERATIONALLY for this candidate code/workflow state; historical provider root cause UNKNOWN. Latest PR #113 HEAD `7a9ac7ae720eba3b8f64c1bfa46feda3927dd410` adds docs-only commits beyond tested SHA and has no CircleCI status entry.
- GitHub Actions run #2247: Windows SUCCESS; Ubuntu IN_PROGRESS at Ubuntu cross-compiler installation, later steps pending. Overall matrix is not yet PASS.
- Free and Open Source is ON per user-confirmed CircleCI UI. Settings API 403 is only a permission limitation.


## Live exact-SHA refresh — 2026-11-20 (B-001 / B-002 / release-gate reconciliation)

This is the newest readback. Preserve all earlier snapshots as history; this block supersedes only conflicting live-state claims.

### Repository and pull requests
- ONE PR #113: OPEN / DRAFT / UNMERGED; exact HEAD `7296c573cbd7eab4a1d7ff08bced2b397a5c555f`; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. Compare from code-bearing source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` shows 48 commits and documentation-only changes across 18 existing docs paths; no runtime source, test, or CI workflow path changed.
- PR #115 is MERGED at `3adace165890bb69de346880e3c612c5cd6c2ff8`.
- Verifier PR #51: OPEN / DRAFT / UNMERGED; HEAD `6c0979e62eb01e4b929c48b4224a3d4f742bc8af`; branch-local `CANONICAL_COMMIT.txt` pins ONE source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. Compare from verifier HEAD `226360d00bc26f858128d3127591fce13ecd52d9` is three documentation-only commits (CANDIDATE_VERIFICATION.md, MIRROR_STATUS.md, README.md, Android evidence note); no workflow/source implementation change.
- Protected verifier branch remains `verification` at `b14abf0020472dc45842be30854e0a09317d6cb4`; protected canonical source pin remains `722d662e7a81711ba3607d26571f4048a292b3ed`. Protected `LEDGER_SOURCE_COMMIT.txt` remains `d6c382a245c49a028bc60809f22ca956c7f894cc`. No protected pin was changed.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`; no main mutation.

### B-001 — CircleCI pre-execution termination
- Historical pipeline #1201 on `509b7b197115adb1ba80d730342e9f6fbf981645` remains a confirmed pre-execution termination; provider root cause UNKNOWN.
- Code-bearing ONE source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` has GitHub combined status `ci/circleci: one-linux = SUCCESS`, workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`. Provider step logs were not inspected; claim only exact-SHA status success.
- Current PR #113 HEAD `7296c573...` has no combined-status entries. Its predecessor docs SHA `a0d94bc9a26cf9effd4b202f29af50f81c8e4ee5` had FAILURE at workflow `916983e9-dbde-4db2-a967-438a666ebb19`, job `a130c5d9-c078-4c18-8ae5-571470f46d60`; available step #101 was CANCELED with unavailable task information and empty logs. Do not call it a test timeout or source failure.
- Exact main SHA `d35db951...` separately shows context `CircleCI Pipeline = ERROR`, target pipeline #1202. Pipeline-to-workflow/job/revision attribution and provider logs remain unavailable. Do not conflate this with the PR context or the source-SHA SUCCESS.
- B-001 is operationally resolved only for the tested code-bearing source/workflow state; provider root cause remains UNKNOWN and the latest docs HEAD has no new CircleCI result.

### B-002 — Issue #104 exact-source verification and remaining product gap
- Twelve PR-triggered verifier workflows associated with verifier HEAD `6c0979e62eb01e4b929c48b4224a3d4f742bc8af` are terminal SUCCESS, all using branch-local source pin `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`:
  - E4 adversarial #44658361321: Unix job #136030099861; Windows reparse/handle job #136030099812; Windows native probe #136030099584 — all SUCCESS.
  - Linux distro #44658361778: Ubuntu 24.04 job #136030102055; Debian 12 job #136030102321 — SUCCESS.
  - Windows native runtime #44658361296: job #136030100371 — SUCCESS.
  - E1 Windows #44658361571: job #136030101715 — SUCCESS.
  - E1 Milestone 1 #44658361518: Windows job #136030101394 and Ubuntu job #136030101588 — SUCCESS.
  - SEC-05 #44658361512: Linux job #136030100674 and Windows job #136030100987 — SUCCESS.
  - Cross-platform #44658361514: Windows job #136030101518 and Ubuntu job #136030101784 — SUCCESS.
  - Artifact integrity #44658361320: job #136030099580 — SUCCESS.
  - Resource guard #44658361457: job #136030100840 — SUCCESS.
  - Android AArch64 build #44658361365: job #136030100171 — SUCCESS.
  - Focused stdlib/database #44658361557: job #136030101561 — SUCCESS.
  - Android API 35 ARM64 runtime #44658361350: build job #136030099698 and runtime job #136030558684 — SUCCESS.
- E4 job logs were inspected, not just run summaries. Linux executed 31 named tests (3 read, 5 write, 1 walk, race, mount-boundary, hard-link, database, 11 governance, 5 network, policy-denial, platform metadata), 0 failed. Windows executed 9 named regressions, 0 failed. The Linux write suite explicitly includes `descriptor_relative_write_fails_closed_for_nested_parent`; this is proof of fail-closed behavior, not positive nested-write support.
- Android API 35 ARM64 runtime is native-backend execution evidence only; it does not establish an Android filesystem adapter.
- Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Do not promote full filesystem support from the scoped E4 pass.

### Release gate / B-003 boundary
- The protected ledger document pin remains `d6c382a245c49a028bc60809f22ca956c7f894cc`; the protected ledger gate #44519577312 remains historical FAILURE (92/100 provisional implementation, 0/100 verified at that gate's ledger source, 18 rows checked, 10 DoD items unchecked, no protected exact-source ONE Package Provenance Release).
- ONE PR #103 remains OPEN / DRAFT / UNMERGED at HEAD `5f616ae08e303081666a5e0d6a4b8b63b9d69c16`. Its branch does not replace the protected ledger source pin. Keep fixed weights, verified points, DoD checkboxes, canonical pins, and release criteria unchanged. No new gate evaluation is justified until relevant acceptance/evidence inputs change.
- No source code, CI configuration, main branch, protected verifier branch/pin, ledger weights, DoD checkbox, merge, or release was changed in this refresh.

### Next executable work
1. Continue B-002 from the remaining Linux nested-write acceptance gap. Keep the current fail-closed behavior; do not weaken it. Any implementation proposal must prove parent-directory move/race confinement with real Linux and Windows runners before changing security-sensitive code.
2. Keep unsupported Android/macOS filesystem adapters explicitly unavailable unless native backends and their own adversarial matrices are implemented and verified.
3. Continue B-003 only after the exact release-candidate SHA, complete E1–E6 evidence, source-bound package provenance, and ledger/DoD reconciliation are genuinely available. Do not rerun the unchanged historical gate or promote scores.


## B-003 exact-head refresh — PR #103 ledger draft (2026-11-20)

- ONE PR #103 remains OPEN / DRAFT / UNMERGED at HEAD `5f616ae08e303081666a5e0d6a4b8b63b9d69c16`. It is 7 commits ahead of protected ledger source pin `d6c382a245c49a028bc60809f22ca956c7f894cc`, and 2 commits ahead of prior CI-verified docs commit `e053b5ffae46a7e1d696bea5eb5a13999c722ee0`. The 2-commit delta changes only `docs/verification/one-completion-ledger.md` and `docs/verification/DEFINITION_OF_DONE.md`.
- GitHub Actions run #44573696031 is COMPLETED / SUCCESS on exact PR #103 HEAD `5f616ae...`: Ubuntu job #135735331182 and Windows Server 2025 job #135735332010 both terminal SUCCESS. This is documentation-branch CI evidence, not a release gate or E1–E6 acceptance promotion.
- CircleCI combined status for exact PR #103 HEAD is `CircleCI Pipeline = ERROR`, target pipeline #1168. Provider logs and pipeline-to-workflow/job/revision mapping are not available; classify the CircleCI cause as UNKNOWN, not a source/test failure.
- The unmerged PR #103 draft ledger records 92/100 IMPLEMENTED, 8/100 VERIFIED (E3 8/8), and DoD 4 checked / 9 unchecked / 13. These are draft values only. The protected verifier ledger source pin remains `d6c382a...`; the protected snapshot at that pin remains 92/100 implementation, 0/100 verified, DoD 3 checked / 10 unchecked / 13.
- Historical protected fixed-weight gate #44519577312 remains FAILURE. Do not treat PR #103's GHA success as a protected ledger-gate pass, and do not merge PR #103, move the ledger source pin, change weights, or check DoD boxes without explicit authorization and full acceptance evidence.
- No product source, CI configuration, main branch, protected verifier branch/pin, ledger weights, or acceptance criteria changed in this refresh.


## B-001 classification correction — 2026-11-20 (pipeline mapping not verified)

This correction supersedes any earlier wording in this document that called B-001 fully or operationally resolved. Do not delete the historical notes.

- Exact ONE source SHA `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` has GitHub combined status `ci/circleci: one-linux = success`. The status target URL identifies workflow `3a9c0613-a1bc-43ae-b2d6-319841061048` and job `c4faf162-80e9-4f17-a83a-a72634bb592f`.
- The CircleCI pipeline UUID/number for that job, pipeline `vcs.revision`, and independent proof that the job belongs to that pipeline/workflow were not retrieved. The GitHub status is exact-SHA context evidence, but it does not complete the required job → workflow → pipeline → `vcs.revision` chain.
- The historical #1201 pre-execution termination remains documented; its provider root cause remains UNKNOWN. Current PR #113 docs HEAD `abfae2b95c179650b362426a001a47e164f7ac20` has no CircleCI status context. Main pipeline #1202 and PR #103 pipeline #1168 show separate ERROR contexts, but their `vcs.revision`, job mapping, and provider logs are unavailable.
- CircleCI UI pipeline pages #1202, #1168, and #1199 were not accessible through the available web session. No authenticated CircleCI API/CLI provider-log access was available. Do not infer timeout, test failure, source regression, or root cause.
- **B-001 status: PARTIAL / NOT CLOSED.** The exact-source GitHub status is SUCCESS, but pipeline mapping and provider diagnostics are still required before claiming full operational resolution. Do not trigger blind retries or change CI configuration to hide the gap.


## Exact-SHA Windows parent-rename regression — 2026-11-20

- New code-bearing test SHA: `c4f3800bc12b70a4d59d72fbc5a80029cb053cea`; the only source change relative to the prior candidate is a Windows-only test in `crates/one-core/src/filesystem.rs`, not production behavior.
- Verifier PR #51 exact HEAD: `7c3ded0d811fe71bd14034880b24b0f8e396349e`; branch-local canonical source pin: `c4f3800...`. Protected verifier branch and canonical pin remain unchanged.
- New test: `filesystem::windows_native::tests::retained_parent_handle_blocks_directory_rename`. It opens a nested parent with the same `FILE_SHARE_READ | FILE_SHARE_WRITE` share flags used by `write_relative` (no `FILE_SHARE_DELETE`), proves rename fails while the handle is retained, then proves rename succeeds after handle close.
- E4 push run #44760172674 is terminal SUCCESS on exact source `c4f3800...` / verifier `7c3ded0...`: Unix job #136370280796 SUCCESS; Windows reparse/handle job #136370280317 SUCCESS; Windows native probe job #136370280733 SUCCESS. Windows log shows the new test executed exactly once and passed (1 passed, 0 failed).
- Duplicate E4 pull-request run #44760180858 is also terminal SUCCESS on the same exact source/verifier pair; Unix job #136370312121, Windows reparse/handle job #136370312012, and native probe job #136370311602 all SUCCESS. Count the duplicate event as confirmation, not a second acceptance point.
- Other exact-verifier-SHA PR workflows: SUCCESS — E4 resource guard #44760181208, focused stdlib/database #44760180912, Android AArch64 artifact build #44760180867, Linux distro #44760180826, E1 Windows #44760181124. IN_PROGRESS — Windows native runtime #44760181204, candidate artifact integrity #44760180923, SEC-05 #44760180951, E1 Milestone 1 #44760180919 (Ubuntu job #136370312623 SUCCESS; Windows job #136370313083 IN_PROGRESS), cross-platform #44760181947, Android API35 runtime #44760180953. Do not call the complete verifier matrix green yet.
- ONE source PR #113 GitHub Actions run #44760138732 is IN_PROGRESS on exact source `c4f3800...`: Ubuntu job #136370164396 SUCCESS; Windows job #136370165192 IN_PROGRESS. CircleCI `ci/circleci: one-linux` is PENDING on this exact SHA, workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Pipeline UUID/number, `vcs.revision`, and job→workflow→pipeline membership remain unverified.
- B-001 remains PARTIAL / NOT CLOSED pending CircleCI pipeline mapping/provider logs. Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes fail-closed; Android/macOS filesystem adapters unavailable. This test verifies one Windows parent-handle exclusion primitive, not complete E4/Issue #104 acceptance.
- No production runtime code, CI configuration, main, protected verifier branch/pin, ledger/DoD, acceptance criteria, merge, or release changed.


## Exact-SHA verification refresh — 2026-11-20 15:14Z

### Candidate identity
- Code-bearing source: `c4f3800bc12b70a4d59d72fbc5a80029cb053cea` (only source delta from `fb5c8bc...` is the Windows-only parent-handle rename regression test).
- Verifier PR #51 HEAD: `7c3ded0d811fe71bd14034880b24b0f8e396349e`; branch-local source pin: `c4f3800...`.
- ONE PR #113 current docs-only HEAD before this sync: `043390cf0673247524ffb44c444cbbe4745be91f`. This sync changes existing docs only and uses `[skip ci]`; no CI result transfers to the resulting docs HEAD.

### Terminal results on exact source/verifier pair
- ONE GitHub Actions run #44760138732 on source `c4f3800...`: Ubuntu job #136370164396 SUCCESS; Windows Server 2025 job #136370165192 SUCCESS. Windows log explicitly ran and passed `interop::tests::subprocess_execution_is_bounded_by_a_deadline`, `subprocess_stdout_capture_has_a_hard_limit`, `subprocess_stderr_capture_has_a_hard_limit`, and `windows_job_object_terminates_descendants_when_parent_exits`.
- Verifier PR matrix on verifier SHA `7c3ded0...` / source pin `c4f3800...`: 11 of 12 named workflows are terminal SUCCESS:
  - E4 adversarial #44760180858: Unix #136370312121, Windows #136370312012, native probe #136370311602 — SUCCESS. New `retained_parent_handle_blocks_directory_rename` test executed once and passed.
  - E4 subprocess resource guard #44760181208 / job #136370313217 — SUCCESS.
  - Focused stdlib/database #44760180912 / job #136370312551 — SUCCESS.
  - Android AArch64 artifact build #44760180867 / job #136370311618 — SUCCESS.
  - Linux distro #44760180826: Debian #136370312102 and Ubuntu #136370312874 — SUCCESS.
  - Windows native runtime #44760181204 / job #136370312887 — SUCCESS; Windows Server 2025, Rust 1.100.0, runtime exit 0, output `Budi`.
  - SEC-05 #44760180951: Linux #136370311990 and Windows #136370312357 — SUCCESS. Source PR Windows GHA also proves the dynamic deadline/stdout/stderr/Job Object tests executed and passed.
  - E1 Windows #44760181124 / job #136370313677 — SUCCESS.
  - E1 Milestone 1 #44760180919: Ubuntu #136370312623 and Windows #136370313083 — SUCCESS.
  - Cross-platform #44760181947: Windows #136370329101 and Ubuntu #136370329521 — SUCCESS.
  - Candidate artifact integrity #44760180923 / job #136370311819 — SUCCESS.
- E4 push event #44760172674 is also SUCCESS on the same exact source/verifier pair (Unix #136370280796, Windows #136370280317, native probe #136370280733); it is a duplicate trigger and is not a second acceptance point.

### Artifact provenance and remaining runtime
- Artifact integrity selected cross-platform run #44760181947 at verifier SHA `7c3ded0...` for exact source `c4f3800...`.
- Linux artifact #14640868829, SHA-256 `7e8e443e2ae7c736f18243043f355840801afebdddca46a5efe41c159bac5732`; metadata target `x86_64-unknown-linux-gnu`, Rust/Cargo 1.100.0, ONE CLI 0.1.0, ELF64 little-endian EM_X86_64 PASS.
- Windows artifact #14641018247, SHA-256 `f68c7325ff39ae33b2cfadf0b896896c0ba4c3eeb581b033f8e1a75974900846`; metadata target `x86_64-pc-windows-msvc`, Rust/Cargo 1.100.0, ONE CLI 0.1.0, PE32+ x86-64 PASS.
- The only verifier workflow not terminal is Android API 35 ARM64 runtime #44760180953. ELF build job #136370311867 is SUCCESS; runtime job #136371143664 is still executing the “Boot Android API 35 AArch64 on ARM64 host” step. Workflow timeout is 150 minutes; no timeout/failure is claimed.
- Do not claim Android filesystem adapter support from this native-backend runtime. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

### CircleCI and release gate
- CircleCI `ci/circleci: one-linux` on source `c4f3800...` remains PENDING; workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Pipeline ID/`vcs.revision` mapping and provider logs remain unavailable. B-001 remains PARTIAL / NOT CLOSED.
- Protected verifier branch/pin, protected ledger pin `d6c382a...`, fixed weights, DoD, main, acceptance criteria, merge, and release state remain unchanged. Historical fixed-weight gate #44519577312 remains FAILURE.


## CircleCI diagnostic utility correction — 2026-11-20 (exact source SHA refresh)

This entry supersedes earlier live diagnostic-utility conclusions where they conflict; retain prior incident records as history.

- Exact ONE PR #113 HEAD: `36c0abff77d20016dc0afc11e8428a6c3fb1248d` (OPEN / DRAFT / UNMERGED; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`). The diagnostic code/test delta adds no product runtime or CircleCI configuration changes.
- CircleCI combined status for this exact SHA: `ci/circleci: one-linux = PENDING`; status target workflow UUID `376028bf-901b-42b1-bd3e-0d93b64350ce`, job UUID `553f3bfc-8bd6-457d-8656-6a65837b1725`. This GitHub status does not establish that the provider job has started. Pipeline UUID/number, pipeline `vcs.revision`, workflow membership, provider start timestamp, and provider logs remain unverified. Do not label this a timeout or failure.
- Earlier exact source `c4f3800bc12b70a4d59d72fbc5a80029cb053cea` separately had CircleCI `ci/circleci: one-linux = SUCCESS`, target workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Do not transfer that result to the new diagnostic SHA.
- Static review found a false-positive classifier path: the utility passed its entire rendered report to `classify_failure()`; its own explanatory text `failed/timed_out` could therefore turn a canceled/no-output step into a false TIMEOUT label. Corrected to classify captured stdout/stderr only, while using explicit provider step outcomes separately. A timed-out outcome remains TIMEOUT; explicitly null job start plus empty logs becomes `PRE-EXECUTION TERMINATION / ROOT CAUSE UNKNOWN`; missing start-time metadata remains inconclusive; a started job with no logs becomes `NO-OUTPUT / ROOT CAUSE UNKNOWN`.
- Static review also found exact-SHA discovery selected only the newest matching pipeline, potentially hiding an earlier failed pipeline for the same commit. It now inspects every pipeline whose `vcs.revision` equals the requested SHA and reports pipeline number/UUID/status/revision, workflow UUID/status, job UUID/number, and job start metadata. Direct `--job-id` mode continues to mark source identity UNVERIFIED.
- Added six unit tests (suite count 14 → 20) for classifier self-text false positives, canceled pre-start vs started/no-output vs missing-start metadata, explicit timed-out outcomes, and multiple pipelines for one exact SHA. **These tests have not yet been observed executing on this exact SHA**; CircleCI currently reports PENDING. Do not claim test PASS until the exact job reaches a terminal result and its step log proves the test step ran.
- Historical pipeline #1201 remains confirmed pre-execution termination (`started_at=null`, canceled step, empty logs) with provider root cause UNKNOWN. Historical account-credit/provider eligibility ambiguity remains unresolved; no credit purchase, plan change, settings toggle, retry, or trigger was performed.
- **B-001 remains PARTIAL / NOT CLOSED.** No root cause has been established for #1201, and the new diagnostic candidate has not yet produced a terminal CircleCI result or complete pipeline → workflow → job → revision provenance.


## CircleCI diagnostic follow-up — 2026-11-20 (job visibility and current provider status)

This is the latest diagnostic checkpoint; earlier incident history remains intact.

- Latest diagnostic source/test SHA: `3db852f68d8bb12fb655524c7133c4cfa4002ca2`; PR #113 remains OPEN / DRAFT / UNMERGED. Changes are limited to `tools/ci/diagnose_circleci.py` and `tools/ci/test_diagnose_circleci.py`; `.circleci/config.yml` is unchanged.
- The diagnostic now discovers **all jobs from every pipeline whose `vcs.revision` exactly matches the requested SHA**, not only failed jobs from the newest matching pipeline. This keeps pending/running jobs visible and reports pipeline number/UUID/status/revision, workflow UUID/status, job number/UUID/status, and start-time metadata. Direct `--job-id` mode continues to mark source SHA as UNVERIFIED.
- The classifier now uses captured provider stdout/stderr and explicit step outcomes, never its own explanatory report text. It distinguishes explicit timed-out outcomes, explicit null start time with no output, known-start/no-output, and missing start-time metadata without inventing a timeout/root cause.
- Diagnostic unit suite now contains 20 test methods (14 baseline + 6 targeted regression tests). These tests have **not yet been verified as executed successfully on SHA `3db852f...`**.
- Exact-SHA CircleCI status readbacks:
  - `36c0abff77d20016dc0afc11e8428a6c3fb1248d`: `ci/circleci: one-linux = FAILURE`; workflow `376028bf-901b-42b1-bd3e-0d93b64350ce`, job `553f3bfc-8bd6-457d-8656-6a65837b1725`.
  - `2bcf397474019cd4415f6506d8dac83a3c5c4010`: `ci/circleci: one-linux = FAILURE`; workflow `fa1500e3-c2e2-42c7-a8c1-dc932bd439ab`, job `3fd1b528-f2a4-42f5-85e3-1d1716d8ad72`.
  - Latest code/test SHA `3db852f...`: `ci/circleci: one-linux = PENDING`; workflow `92fd294f-c8d5-480c-af6f-9ddbb7ffe479`, job `5666c0ea-e06e-475b-8967-5ccb24ccb383`.
  - Earlier source SHA `c4f3800bc12b70a4d59d72fbc5a80029cb053cea` remains a separate historical `SUCCESS` (workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd)); it does not validate the diagnostic changes.
- The CircleCI app job pages remain inaccessible in this connected session, and no authenticated CircleCI CLI/MCP action is available. Pipeline UUID/number, `vcs.revision` readback, job membership readback, step logs, and terminal provider outcomes are not available for the new jobs. GitHub status FAILURE alone does not reveal whether the cause is the new test, configuration, provider setup, or another condition.
- **B-001 remains PARTIAL / NOT CLOSED.** Do not infer test failure, timeout, or provider root cause from the status labels alone. No blind retry, configuration edit, plan purchase, or settings change was performed. Required next evidence: a redacted exact-SHA diagnostic report/provider step log for the failed workflow/job above, plus the latest pending job's terminal status and full pipeline → workflow → job → `vcs.revision` mapping.


## CircleCI terminal-status correction — 2026-11-20

A fresh exact-SHA status readback supersedes the PENDING observation above for diagnostic SHA `3db852f68d8bb12fb655524c7133c4cfa4002ca2`:

- `ci/circleci: one-linux = FAILURE`; workflow UUID `92fd294f-c8d5-480c-af6f-9ddbb7ffe479`; job UUID `5666c0ea-e06e-475b-8967-5ccb24ccb383`.
- The earlier diagnostic SHAs remain FAILURE: `36c0abf...` (workflow `376028bf-901b-42b1-bd3e-0d93b64350ce`, job `553f3bfc-8bd6-457d-8656-6a65837b1725`) and `2bcf397...` (workflow `fa1500e3-c2e2-42c7-a8c1-dc932bd439ab`, job `3fd1b528-f2a4-42f5-85e3-1d1716d8ad72`).
- The status target URLs still expose workflow/job UUIDs only. The CircleCI app pages remain inaccessible here; no provider step output, pipeline UUID/number, `vcs.revision`, job membership, or `started_at` has been independently read. Therefore the three FAILURE labels do **not** establish a test failure, timeout, or root cause.
- The diagnostic tests on SHA `3db852f...` must be treated as **NOT VERIFIED** until the actual CircleCI step log is obtained. B-001 remains PARTIAL / NOT CLOSED.
- Next safe evidence collection: run the repository's updated exact-SHA diagnostic locally for `3db852f...` using an already-configured local `CIRCLECI_TOKEN`; do not paste the token. Share only the redacted report. The utility now discovers every matching pipeline and lists all jobs, including pending/running, while preserving exact revision and job-start evidence.


## Termux worktree map and continuation entrypoint

For Termux continuation, read [TERMUX_WORKTREE_MAP.md](TERMUX_WORKTREE_MAP.md) first. The PR #113 branch `security/issue-104-root-confinement-20261118` is already checked out in `$HOME/one-pr113`; do not try to switch to it from `$HOME/one` or create a duplicate worktree. The map records the confirmed directory/branch layout, safe fast-forward-only synchronization procedure, and current B-001 handoff. Always re-check status and exact SHAs before syncing; preserve all local and untracked evidence.

## 2026-11-21 — exact build failure diagnosis and source fix

- PR #113 pre-fix exact HEAD: `168ff06e7a8d09e575e6ac71513399c9f1f4f994`.
- CircleCI status context on that SHA: `ci/circleci: one-linux = FAILURE`; target workflow `16c4f795-1bcb-4a24-9935-ffd0e553f30c`, job `b7e21b09-8205-4d29-a9c7-a2f555e5e913`. This session could not retrieve CircleCI pipeline number/UUID, `vcs.revision`, workflow/job membership, `started_at`, or provider step logs. CircleCI root cause therefore remains UNKNOWN.
- Independently, GitHub Actions `One CI` run #44898144880 on the same exact source failed deterministically in Build on Ubuntu job #136828104281 and Windows 2025 job #136828104468. Both logs show Rust parser errors in `crates/one-core/src/interop.rs` lines 1207 and 1209: unsafe block expressions were compared directly using `== 0` and `>= 0` without parentheses. Clippy and Tests were skipped.
- Minimal fix committed on the PR branch as `502a888da46e3d4be4ac80b3a2ffc3233dbabc12`: `(unsafe { libc::setpgid(0, 0) }) == 0` and `(unsafe { libc::setsid() }) >= 0`.
- Post-fix exact-SHA status readback: no GitHub status contexts and no GitHub Actions workflow run yet associated with `502a888...`. This is NOT PASS and does not prove CircleCI triggered. The source fix is awaiting a fresh build.
- Keep B-001 PARTIAL / NOT CLOSED. Do not attribute the CircleCI failure to the Rust syntax error until provider-side pipeline → workflow → job → revision/log evidence independently establishes that same failure.
