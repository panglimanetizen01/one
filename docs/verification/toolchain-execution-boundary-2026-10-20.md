# Gate 1 — Toolchain Execution Boundary Verification

Revision under verification: `feature/gate1-toolchain-execution-boundary`
Base: `34725aa393ca31759eb88c96688063f6358ce54a`

## Implemented boundary

- `ToolchainCommand`: executable plus ordered arguments.
- `ToolchainInvocation`: exit status, stdout, stderr.
- `ToolchainExecutionError`: toolchain-not-found, process-start, non-zero-exit, artifact-missing, and validation failures.
- `ToolchainResolver` and `PathToolchainResolver`.
- `ToolchainInvoker` and `CommandToolchainInvoker` using direct `std::process::Command` argument passing.
- Deterministic build command construction from target triple, source, and output.
- `execute_toolchain_build`: compatibility validation → resolution → invocation → artifact existence validation.
- Cargo fake-toolchain binary fixture and integration regression suite.

## RED evidence

The Gate 1 integration test suite was added before the production boundary implementation. A local RED run could not be executed in this environment because the repository could not be cloned locally: the container reported `Could not resolve host: github.com`.

The hosted verification branch was temporarily enabled for the existing CI workflow. Run `39454526520` (`One CI`) was created for commit `f815771f451e25ccb4f883003e38d215338310e4`, but both matrix jobs completed with `steps: null`; this is the repository's known zero-step CI infrastructure failure mode and does not provide source compilation/test evidence. The temporary CI trigger was restored immediately afterward.

Therefore no claim is made here that the Gate 1 test suite has executed successfully.

## Static review findings and repairs

A source review found that `command_preserves_executable_and_arguments_without_shell_parsing` passed the relative output path `artifact with spaces` to the fake toolchain. The fixture writes the requested `-o` output, so this could leave a generated artifact in the test working directory. The test now directs that artifact to a process-specific path under the system temporary directory, asserts the requested output path, confirms the artifact exists, and removes it. Other fixed artifact fixture names were also made process-specific to reduce collisions between concurrent test processes.

A second contract gap was identified: `execute_toolchain_build` previously trusted any `Ok(ToolchainInvocation)` from a custom invoker without checking `invocation.success()`. It could therefore accept a non-zero status if an artifact happened to exist. The high-level boundary now converts a completed non-zero invocation to structured `NonZeroExit`, preserving status/stdout/stderr.

A third false-success path was identified: a pre-existing artifact could satisfy `artifact.is_file()` even if the current toolchain invocation did not produce it. The high-level boundary now rejects an already-existing output path before resolver/invoker execution, preserving the existing file and returning structured `Validation`. The documented limitation is that callers must choose a fresh output path or deliberately relocate/remove an old artifact before invoking this API.

The original test-isolation repair was committed at `763d39b392000603353556bd801b418cb4ea35a7`; the later status and stale-artifact fixes are in the current branch history. These changes have been read back from GitHub, but **none has executable test evidence yet**.

## Latest hosted verification attempt

The branch-push workflow was configured to run on branch pushes and to include a focused Gate 1 test step. Run [`39542394202`](https://github.com/panglimanetizen01/one/actions/runs/39542394202) targets source revision `934439ce07e8f356cd3731fdf3fb7070f564a99a`. Both matrix jobs failed with empty `steps: []`, empty runner names, and no downloadable job log. This is evidence that hosted execution did not reach checkout or any executable test step; it is not source-code pass/fail evidence.
## Required commands not yet evidenced

- `cargo test -p one-core --test toolchain_execution_boundary`
- `cargo fmt --check`
- `cargo test -p one-core --locked`
- `cargo check --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets --locked -- -D warnings`
- `python tools/check_contract.py`
- `git diff --check`

## Status

`IN PROGRESS / BLOCKED BY VERIFICATION INFRASTRUCTURE`

Gate 1 is not eligible for `GREEN LOCKED 🔒 FINAL` until fresh executable test, format, check, clippy, contract, and diff evidence exists for the final revision.
