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

## Static review finding and repair

A source review found that `command_preserves_executable_and_arguments_without_shell_parsing` passed the relative output path `artifact with spaces` to the fake toolchain. The fixture writes the requested `-o` output, so this could leave a generated artifact in the test working directory. The test now directs that artifact to a process-specific path under the system temporary directory, asserts the requested output path, confirms the artifact exists, and removes it.

This is a test-isolation repair committed at `763d39b392000603353556bd801b418cb4ea35a7`. It has been read back from GitHub, but it is **not yet executable-verified**: the available hosted run `39454526520` failed with zero steps, and this session's local environment has no `rustc` executable.
## Required commands not yet evidenced

- `cargo test -p one-core --test toolchain_execution_boundary --locked`
- `cargo fmt --check`
- `cargo test -p one-core --locked`
- `cargo check --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets --locked -- -D warnings`
- `python tools/check_contract.py`
- `git diff --check`

## Status

`IN PROGRESS / BLOCKED BY VERIFICATION INFRASTRUCTURE`

Gate 1 is not eligible for `GREEN LOCKED 🔒 FINAL` until fresh executable test, format, check, clippy, contract, and diff evidence exists for the final revision.
