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

- CircleCI API v2 is used only for pipeline/workflow/job discovery because the project-scoped pipeline listing is available there.
- CircleCI API v3 is used for job details and per-step stdout/stderr.
- API v2 uses the Circle-Token header; API v3 uses Authorization: Bearer.
- The script prints step numbers, execution indexes, outcome, and a bounded tail of each failed step's logs.
- Secret-like values are redacted before display and before writing an optional report.
- A missing stderr stream is normal; CircleCI returns no content when a step has no stderr.
- Pagination is limited to ten pages as a safety boundary. If CircleCI advertises another page after the tenth, the utility raises a diagnostic error and marks discovery incomplete; it never silently treats truncated results as complete.
- If no pipeline exists for the exact local SHA, the script stops rather than selecting a nearby or newer commit.

## Tests

Run the standard-library unit tests:

\`\`\`bash
python3 -m unittest discover -s tools/ci -p 'test_*.py' -v
\`\`\`

The tests cover failed-step extraction (including parallel executions), timeout/compiler/test classification, secret redaction, complete multi-page discovery, and explicit failure when the ten-page safety limit would truncate results.

## Limitations

The classification is a log-based hint, not a guaranteed root-cause analysis. The utility does not rerun, cancel, approve, or modify CircleCI jobs. It only reads pipeline/job metadata and logs.
