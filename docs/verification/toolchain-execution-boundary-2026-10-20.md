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
- A pre-existing output path is rejected before resolver/invoker execution. This prevents stale-artifact false success and preserves the existing file, but means callers must select a fresh output path or deliberately relocate/remove an old artifact before calling the API.
- Fake-toolchain fixture and regression tests cover argument preservation, missing executable, process-start failure, non-zero status, target mismatch, missing artifact, successful artifact creation, custom-invoker non-zero status, stale-artifact rejection, and high-level composition.

## Repository policy note

The current `.gitignore` explicitly ignores `Cargo.lock`. Gate 1 commands therefore do not use `--locked`; this avoids claiming lockfile-based reproducibility that the repository does not currently provide. The lockfile policy is not changed by this Gate and remains a separate workspace-level decision.

## Verification evidence

The source, tests, workflow, and documentation have been read back from GitHub after updates. This is repository-state evidence, not executable test evidence.

The available container has no `rustc`, `cargo`, `rustfmt`, or `clippy-driver`; network DNS resolution to GitHub and Rust distribution hosts also fails. Therefore this environment cannot execute the Rust verification commands.

A previous branch-push attempt, run [`39542767735`](https://github.com/panglimanetizen01/one/actions/runs/39542767735), targeted a related working branch whose Gate 1 source/test blobs match this implementation, but it is **not promoted as evidence for this clean Gate 1 branch**. Both jobs had empty `steps: []`, no runner name, and no downloadable job log. It proves only that hosted execution failed before an executable step on that attempt.

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
