# CircleCI targeted diagnostics for Project ONE

## Purpose

Use the CircleCI API to identify the failed job and retrieve only the failed step's stdout/stderr. The utility deliberately matches the current local Git HEAD to the pipeline revision before selecting jobs, so it will not silently diagnose a different commit.

## Prerequisites

- Python 3 (standard library only; no pip install required).
- A CircleCI personal API token available locally as CIRCLECI_TOKEN (or CIRCLE_TOKEN).
- The token must have access to the panglimanetizen01/one CircleCI project.

Do not paste the token into chat, save it in the repository, or print it in terminal output.

## Usage

From the root of the ONE repository:

\`\`\`bash
python3 tools/ci/diagnose_circleci.py
\`\`\`

The default mode reads the current branch and exact git rev-parse HEAD, finds the CircleCI pipeline for that exact revision, discovers failed/timed-out jobs, and prints only their failed-step logs.

Inspect a known job UUID directly:

\`\`\`bash
python3 tools/ci/diagnose_circleci.py --job-id 94f53464-44ae-40e1-8768-65e6a5e7014c
\`\`\`

Save a redacted report:

\`\`\`bash
python3 tools/ci/diagnose_circleci.py --output /tmp/one-circleci-diagnostic.txt
\`\`\`

Override the exact revision or branch when diagnosing a historical run:

\`\`\`bash
python3 tools/ci/diagnose_circleci.py --sha COMMIT_SHA --branch BRANCH_NAME
\`\`\`

## API behavior

- The Linux job installs the exact Rust release from rust-toolchain.toml in a dedicated step. Rustup bootstrap and toolchain installation have explicit wall-clock limits, separate from the cross-toolchain package provisioning step; the no-output timeout is 12 minutes, deliberately longer than the 10-minute install deadline so the command timeout can fire and report its exit before CircleCI's inactivity guard.
- CircleCI API v2 is currently documented by CircleCI as legacy but supported; the pipeline/workflow/job discovery endpoints used here are not part of the announced February 1, 2027 retirement, which targets nine API v1/v1.1 trigger and rerun endpoints only. Do not perform a speculative v2-to-v3 migration based on that v1/v1.1 deadline. Reassess v3 endpoint coverage for exact project + revision discovery separately; use v3 where the required semantics are documented and tested, and retain v2 where v3 does not yet provide an equivalent.
- CircleCI API v3 is current and recommended for new integrations; it is Cloud-only, uses UUID resource IDs and opaque cursor pagination. API v2 remains the supported fallback for existing integrations and CircleCI Server.
- CircleCI API v3 is used for job details and per-step stdout/stderr.
- API v2 uses the Circle-Token header; API v3 uses Authorization: Bearer.
- The script prints step numbers, execution indexes, outcome, and a bounded tail of each failed step's logs.
- Secret-like values are redacted before display and before writing an optional report.
- A missing stderr stream is normal; CircleCI returns no content when a step has no stderr.
- Pagination is limited to ten pages as a safety boundary. If CircleCI advertises another page after the tenth, the utility raises a diagnostic error and marks discovery incomplete; it never silently treats truncated results as complete.
- Pagination responses are schema-checked: `items` must be an array of objects and `next_page_token` must be null or a non-empty string. Malformed responses stop discovery as API/shape errors rather than being treated as an empty result.
- If no pipeline exists for the exact local SHA, the script stops rather than selecting a nearby or newer commit.
- If an exact-SHA pipeline exists but no failed/timed-out job is listed, the utility reports that state as inconclusive (running, successful, or no-job workflow); it does not label it a diagnostic/API error or claim PASS. Confirm workflow status in CircleCI.

## Tests

Run the standard-library unit tests:

\`\`\`bash
python3 -m unittest discover -s tools/ci -p 'test_*.py' -v
\`\`\`

The tests cover failed-step extraction (including parallel executions), timeout/compiler/test classification, secret redaction, complete multi-page discovery, and explicit failure when the ten-page safety limit would truncate results.

## Limitations

The classification is a log-based hint, not a guaranteed root-cause analysis. The utility does not rerun, cancel, approve, or modify CircleCI jobs. It only reads pipeline/job metadata and logs.

## Known Project ONE job IDs — 2026-11-19

The following CircleCI UUIDs are already resolved. Use `--job-id` to inspect the exact job directly; this avoids pipeline/branch discovery and cannot silently select a different commit.

| Source SHA | Workflow UUID | Job UUID | Last observed status |
| --- | --- | --- | --- |
| `722d662e7a81711ba3607d26571f4048a292b3ed` | `9fb2893b-4373-4ef1-ad97-60b804ebedda` | `9f3acc50-11b9-47e5-b841-6a943300be94` | FAILURE |
| `e8dab3a0ac2de6b09b374b692d740b9fb5630f00` | `e55fc480-62fb-469f-8351-deff706595ac` | `69d31382-c676-47f8-92b3-98c0dd8cbf1e` | FAILURE |
| `424f7bbc73901c5e4342eb04444eaabb526b711a` | `141f741c-74b9-4b36-96e4-9f81d76143cb` | `09b422cb-8c1a-47c5-a252-3974c649022c` | PENDING at last readback; inspect after terminal status |

Run the two known failed jobs directly (after setting `CIRCLECI_TOKEN` locally):

```bash
python3 tools/ci/diagnose_circleci.py --job-id 9f3acc50-11b9-47e5-b841-6a943300be94 --output /tmp/one-circleci-722d662.txt
python3 tools/ci/diagnose_circleci.py --job-id 69d31382-c676-47f8-92b3-98c0dd8cbf1e --output /tmp/one-circleci-e8dab3a.txt
```

The latest pending job can be inspected directly once it reaches a terminal state:

```bash
python3 tools/ci/diagnose_circleci.py --job-id 09b422cb-8c1a-47c5-a252-3974c649022c --output /tmp/one-circleci-current-head.txt
```

If the CLI is installed and authenticated, the equivalent low-level inspection path is:

```bash
circleci job get JOB_UUID --json
circleci job output get JOB_UUID --step-num STEP_NUMBER
```

Use the step number from the job metadata and inspect only the first failed step before interpreting downstream failures. Do not enable blanket reruns or add retries until the logs establish a transient failure mode. A successful rerun of the same exact SHA without code/config changes is evidence of a potentially transient failure; a repeatable failure requires a targeted fix.

## Local validation commands

Run these from the repository root on the exact checked-out SHA:

```bash
python3 -m unittest discover -s tools/ci -p 'test_*.py' -v
circleci config validate .circleci/config.yml
```

Record `git rev-parse HEAD` alongside both command outputs. Do not report either validation as passed unless the commands actually execute and exit successfully.
