# Gate 1 — Toolchain Execution Boundary Verification

## Scope and source boundary

- Canonical base: `main` at `34725aa393ca31759eb88c96688063f6358ce54a`.
- Gate 1-only branch: `fix/gate1-toolchain-execution-boundary`.
- Gate 1 is isolated from native compiler generalization / Gate 2 files.
- No parser, analyzer, One Semantic IR, interpreter, or target semantic changes are included.

## Implemented contract

- `ToolchainCommand` stores an executable path and ordered `OsString` arguments.
- `ToolchainInvocation` retains process status, stdout, and stderr.
- `ToolchainExecutionError` distinguishes toolchain-not-found, process-start, non-zero-exit, artifact-missing, and validation failures.
- `PathToolchainResolver` resolves an explicit executable path.
- `CommandToolchainInvoker` uses `std::process::Command` directly; no shell parsing or interpolated shell command is used.
- `execute_toolchain_build` validates target/toolchain compatibility before resolution, checks invocation status even for custom invokers, and requires a fresh regular artifact.
- A pre-existing output path is rejected before resolver/invoker execution. This prevents stale-artifact false success and preserves the existing file, but means callers must select a fresh output path or deliberately relocate/remove an old artifact before calling the API. When invocation fails or exits non-zero, any regular partial output file is best-effort removed.
- Fake-toolchain fixture and regression tests cover argument preservation, missing executable, process-start failure, non-zero status, target mismatch, missing artifact, successful artifact creation, custom-invoker non-zero status, stale-artifact rejection, partial-artifact cleanup, and high-level composition.

## RED evidence status

The focused regression tests are present, but the required pre-fix RED run has not been executed. The current environment cannot run Rust, and the hosted runner has not started any job steps. Therefore no RED or GREEN test result is claimed.

## Repository policy note

The current `.gitignore` explicitly ignores `Cargo.lock`. Gate 1 commands therefore do not use `--locked`; this avoids claiming lockfile-based reproducibility that the repository does not currently provide. The lockfile policy is not changed by this Gate and remains a separate workspace-level decision.

## Verification evidence

The source, tests, workflow, and documentation have been read back from GitHub after updates. This is repository-state evidence, not executable test evidence.

The available container has no `rustc`, `cargo`, `rustfmt`, or `clippy-driver`; network DNS resolution to GitHub and Rust distribution hosts also fails. Therefore this environment cannot execute the Rust verification commands.

The clean Gate 1 branch run [`39543528736`](https://github.com/panglimanetizen01/one/actions/runs/39543528736) targeted source/test revision `215c41b325161da6bee82208e3ccbad0bf114ba1`. Both matrix jobs failed with empty `steps: []`, empty runner names, and no downloadable job logs. Multiple earlier runs and a retry of a prior exact-branch run showed the same pre-step failure. This is infrastructure/verification-insufficiency evidence, not a source-code test result.

The branch has a focused Gate 1 CI step and push-triggered CI for branches, but repeated workflow attempts have failed before checkout/toolchain steps. No Rust command has been shown to pass or fail.

The public [GitHub Status page](https://www.githubstatus.com/) was checked on 2026-10-21 and reported all systems operational with no incident listed for that date. Therefore the exact cause of the zero-step jobs is **unknown**; this record does not attribute it to a confirmed GitHub-wide outage or infer account billing/quota state. Subsequent documentation-only commits do not alter the source/test blobs; their own workflow runs also fail before executable steps.

## Required commands not yet evidenced

- `cargo test -p one-core --test toolchain_execution_boundary`
- `cargo fmt --check`
- `cargo test -p one-core`
- `cargo check --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `python tools/check_contract.py`
- `git diff --check`

## Status

`IN PROGRESS / BLOCKED BY EXECUTABLE VERIFICATION`

Gate 1 is not eligible for `GREEN LOCKED 🔒 FINAL` until the focused test, full `one-core` suite, formatting, workspace check, Clippy, contract check, diff check, and final-diff review have fresh evidence for the final source revision.
