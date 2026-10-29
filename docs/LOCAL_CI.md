# Local CI with Python

The Python runner at `tools/ci_local.py` provides one command to run the repository's portable CI checks on the current machine. It is useful when hosted GitHub Actions cannot allocate a runner or when a contributor wants a local result without a paid CI service.

## Requirements

- Python 3.10 or newer (the runner uses standard-library modules only).
- Rust toolchain with `rustc`, `cargo`, and Clippy available.
- Repository dependencies available locally, or network access for Cargo to download them.

## Run

From the repository root:

```sh
python tools/ci_local.py
```

To require Cargo's offline mode:

```sh
python tools/ci_local.py --offline
```

Offline mode intentionally fails if required dependencies are not cached. It does not silently switch back to network access.

## Checks performed

The runner checks toolchain availability, then executes:

1. `tools/check_contract.py`
2. `cargo build --workspace --all-targets`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo test --workspace --all-targets`
5. Deterministic `.oneir`/`.onepkg` artifact test
6. DNS adapter tests
7. Orchestration tests
8. Database vertical-slice tests
9. HTTP GET vertical-slice tests
10. HTTP GET CLI integration tests

It prints every command, each step result, elapsed time, and a final summary. The process exits with code 0 only when all configured checks pass. A missing tool or failed command is not reported as a pass.

## Evidence boundary

This runner is an orchestration tool; it does not replace the Rust compiler, Cargo, Clippy, or the test suite. It runs on the current host only. A successful Linux/Codespaces run does not prove Windows execution or Android cross-compilation/runtime behavior. Those claims still require evidence from the corresponding target environment.

The existing hosted workflows remain unchanged. This tool is an additional local verification path, not proof that GitHub Actions infrastructure has been repaired and not a hosted CI service.
