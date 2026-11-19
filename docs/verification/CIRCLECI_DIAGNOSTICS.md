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
