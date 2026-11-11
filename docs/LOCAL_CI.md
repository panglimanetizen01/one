## Cargo lockfile policy

The Rust workspace commits `Cargo.lock` so all CI and local Cargo commands using `--locked` resolve the same dependency graph. Do not remove `--locked` to work around a missing lockfile. When dependencies or manifests change, regenerate with `cargo generate-lockfile` using the repository toolchain, review the full lockfile diff and registry checksums, then verify with `cargo metadata --locked --format-version 1` and the complete Local CI runner. Never hand-edit the generated lockfile or copy one from another branch/source SHA without proving the workspace manifests and toolchain context are identical.

# Local CI with Python

The Python runner at `tools/ci_local.py` runs the repository's portable CI checks on the current machine. It is useful for repeatable local verification and produces a machine-readable JSON report for review or archiving.

## Requirements

- Python 3.10 or newer (the runner uses standard-library modules only).
- Rust toolchain with `rustc`, `cargo`, and Clippy available.
- Repository dependencies available locally, or network access for Cargo to download them.

## Run

From the repository root:

```sh
python tools/ci_local.py
```

By default, the runner writes `ci-local-report.json` in the repository root. The schema-version 2 report includes host and Python versions, the exact pre-run Git source revision, a separate post-check worktree snapshot with bounded dirty-entry details, start/end timestamps, redacted per-step commands, status, elapsed seconds, errors/exit codes, a summary, and an explicit evidence boundary. Report output is written atomically. Child-process stdout/stderr and command arguments are redacted against credential-named environment variables before being printed; raw secret values are not stored in the report. The JSON `source_revision` object records the commit SHA, branch (or detached HEAD), and worktree state before checks. `worktree_after` records the state after checks but before report output, so the report file itself cannot make the recorded source tree appear dirty. Both snapshots include up to 100 dirty status entries for diagnosis. If the runner is executed outside a Git checkout, `source_revision.status` is `UNAVAILABLE`; the run may still execute, but the report cannot independently prove an exact source revision. In hosted CI, compare `source_revision.commit_sha` with the workflow's pinned source SHA.

To choose a different report path and per-command timeout:

```sh
python tools/ci_local.py --report artifacts/local-ci.json --timeout 3600
```

The timeout applies separately to each command (default: 1800 seconds). Use `--timeout 0` to disable command timeouts.

To require Cargo's offline mode:

```sh
python tools/ci_local.py --offline
```

Offline mode intentionally fails if required dependencies are not cached. It does not silently switch back to network access.

## Checks performed

The runner checks `rustc --version` and `cargo --version`, then executes every Cargo build/test/clippy command with `--locked` so the committed `Cargo.lock` cannot be silently rewritten. It then executes:

1. `tools/check_contract.py`
2. `cargo --locked build --workspace --all-targets`
3. `cargo clippy --locked --workspace --all-targets -- -D warnings`
4. `cargo --locked test --workspace --all-targets`
5. Deterministic `.oneir`/`.onepkg` artifact test
6. DNS adapter tests
7. Orchestration tests
8. Database vertical-slice tests
9. HTTP GET vertical-slice tests
10. HTTP GET CLI integration tests

It prints every command, each step result, elapsed time, and a final summary. The process exits with code 0 only when all configured checks pass. Missing tools, failed commands, and timed-out commands are not reported as passes. A non-zero exit is also returned if the requested report cannot be written.

## Evidence boundary

This runner is an orchestration tool; it does not replace the Rust compiler, Cargo, Clippy, or the test suite. It runs on the current host only. A successful Linux/Codespaces run does not prove Windows execution or Android cross-compilation/runtime behavior. Those claims still require evidence from the corresponding target environment.

The GitHub Actions workflows remain a separate infrastructure path and are not repaired or replaced by this script. CircleCI is also a distinct hosted CI path. The Local CI runner is an additional reproducible check, not a hosted CI service; only evidence tied to the same source SHA may be compared across these systems.

## Unix test portability

The Linux linker-discovery unit tests use symlinks to the host's true and false utilities instead of executable shell scripts created under the temporary directory. This keeps the tests meaningful on CI runners whose temporary filesystem may be mounted with noexec; it does not skip or weaken the linker compatibility checks.

## CircleCI integration

The source branch includes `.circleci/config.yml`. The configured CircleCI job runs the same Python runner and shared report/manifest validator on Linux. A CircleCI result counts as evidence only when its status is attached to the exact source SHA, its job conclusion is successful, and the report/manifest verify. A config file or a failed/pending status alone is not proof of a successful CircleCI run. The runner's local use remains host-only evidence.


## Report integrity and secret handling

The runner writes the JSON report using a temporary file in the destination directory, flushes it, and atomically replaces the destination. The runner redacts values from environment variables whose names indicate tokens, secrets, passwords, credentials, private keys, API keys, or authorization before printing command arguments and captured child-process output. Captured output is not copied into the JSON report.

The hosted verifier validates schema version, exact source SHA, clean pre-run and post-check worktree snapshots, and the complete 12/12 Local CI summary before accepting a report. It creates a separate manifest containing the report SHA-256 and source/verifier execution identity. Missing reports or manifests are failures, not warnings. The report and manifest are distinct artifacts and their digests must be checked after download.


## Clean Python test execution

The Local CI runner sets `PYTHONDONTWRITEBYTECODE=1` for its child checks. The repository also ignores Python-generated `__pycache__/` and `*.py[cod]` files so running the independent unit-test step before Local CI does not falsely mark an otherwise clean source checkout as modified. These are generated interpreter artifacts only; source files remain visible to the worktree audit.


## Report/manifest verification contract

`tools/ci_local_evidence.py` validates the report's schema, exact source SHA, pre-run and post-check worktree snapshots, the canonical ordered set of all 12 Local CI step names and commands, each step's PASS status and zero exit code, and summary consistency. The canonical step plan is defined once in the evidence validator and its count is derived from that plan, so a report cannot substitute arbitrary successful-looking steps or omit a required command while retaining a 12/12 summary. It creates an atomic manifest with the report's byte length and SHA-256 plus verifier/run/OS identity. Its `verify` command re-reads the report and manifest and rejects missing files, malformed JSON, source/verifier SHA mismatch, noncanonical step names/commands, missing or nonzero step exit codes, summary inconsistency, and report digest/size mismatch. The independent verifier and CircleCI use the same helper instead of maintaining separate inline validation logic.


## Pull request evidence review procedure

For each PR candidate, reviewers must use the live PR head SHA, not a historical SHA copied from an earlier report.

1. Record the exact PR head SHA and confirm the independent verifier's `LOCAL_CI_CANONICAL_COMMIT.txt` contains the same full 40-character SHA. Do not confuse it with the separate `CANONICAL_COMMIT.txt` pin used for another candidate.
2. Confirm the verifier workflow actually checked out that SHA. Review the checkout log and `git rev-parse HEAD` output in each OS job. A pin file by itself is not checkout evidence.
3. Require terminal PASS for every required OS job. Review the report's source SHA, pre-run and post-check worktree snapshots, all 12 individual step statuses, summary, exit code, and the verifier/run/OS identity in the manifest.
4. Download both report and manifest. Compare the downloaded archive SHA-256 with the artifact digest published by GitHub. Then verify the extracted files:
   ```sh
   python tools/ci_local_evidence.py verify \
     --report artifacts/ci-local-report.json \
     --manifest artifacts/ci-local-manifest.json \
     --expected-source-sha "$SOURCE_SHA" \
     --expected-verifier-sha "$VERIFIER_SHA"
   ```
5. Check the CircleCI status context for the same source SHA. A CircleCI FAILURE remains FAIL even if the independent verifier passes; obtain its job logs and resolve the root cause before claiming CircleCI acceptance.
6. Check whether the repository actually enforces required status checks. If classic branch protection or rulesets do not enforce them, treat CI evidence as a manual review gate; do not describe it as an automatic merge blocker.

The current Local CI parity workflow does not execute an Android emulator or Android runtime test. Android SDK/NDK installation and cross-compiler discovery are setup evidence only. Android runtime remains NOT RUN unless a separate device/emulator execution is recorded against the same source SHA.
