# One v0.1 Linux/Windows Verification Evidence — 2026-10-15

## Canonical source

Repository: `panglimanetizen01/one`

Implementation commit independently verified:

`d17f2829187ec28031a4b4a94dd0e0af282f66b8`

This source commit contains the parser/name-boundary fixes, explicit action enforcement, multi-word loop/function coverage, and structural-word boundary tests. Documentation-only commits follow this verified source commit.

## Linux

Independent public-runner verification mirror: `panglimanetizen01/one-windows-verification`

Workflow run: `38753437740`
Job: `verify (ubuntu-latest)` (`116485529987`)

The job checked out the canonical implementation commit directly and completed successfully.

Successful stages:

- canonical commit checkout and SHA verification;
- Rust/Cargo/Python toolchain detection;
- contract-artifact consistency check;
- `cargo build --workspace --all-targets`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace --all-targets`;
- `tools/verify_offline.sh`.

## Windows

Independent public-runner verification mirror: `panglimanetizen01/one-windows-verification`

Workflow run: `38753437740`
Job: `verify (windows-2025)` (`116485530381`)
Runner: Windows Server 2025 (`windows-2025`)

The job checked out the canonical implementation commit directly and completed successfully.

Successful stages:

- canonical commit checkout and SHA verification;
- Rust/Cargo/Python toolchain detection;
- contract-artifact consistency check;
- `cargo build --workspace --all-targets`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace --all-targets`;
- `tools/verify_offline.ps1`.

## Regression coverage added in this audit

The verified source includes regression tests for:

- arbitrary first-word/implicit actions being rejected;
- the explicit `sapa` action remaining valid and aligned with the grammar;
- multi-word loop variables;
- multi-word function parameters;
- structural words such as `ke` remaining valid name content outside their active production boundary;
- `dan` remaining valid name content outside function-call argument context;
- list names containing `dan`;
- deterministic invalid-fixture error codes.

## Interpretation

These are independent GitHub-hosted executions in a temporary public verification repository, not executions of the private canonical repository's hosted workflow. The canonical repository remains the source of truth.

The private canonical GitHub Actions environment remains separately recorded as blocked because its jobs failed before usable step/log evidence. No paid GitHub upgrade is required or used for this verification path.

## Result

For implementation commit `d17f2829187ec28031a4b4a94dd0e0af282f66b8`, Linux and Windows execution evidence is green for build, Clippy, tests, contract artifacts, and the platform-specific offline verification scripts.
