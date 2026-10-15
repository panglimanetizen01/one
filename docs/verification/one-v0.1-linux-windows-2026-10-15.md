# One v0.1 Linux/Windows Verification Evidence — 2026-10-15

## Canonical source

Repository: `panglimanetizen01/one`

Canonical commit verified:

`69c4918e6845fdbbd79495b3fa2c59798831bce0`

The commit contains the standard Windows runner change only; the source implementation and test artifacts are inherited from the previously independently verified Linux commit lineage.

## Linux

Independent public-runner verification mirror: `panglimanetizen01/one-windows-verification`

Workflow run: `38747320152`
Job: `verify (ubuntu-latest)` (`116464264565`)

The job verified the canonical commit directly using the repository read token and completed successfully.

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

Workflow run: `38744988440`
Job: `windows-verification` (`116460500173`)
Runner: Windows Server 2025 (`windows-2025`)

The job verified the canonical commit directly and completed successfully.

Successful stages:

- canonical commit checkout and SHA verification;
- Rust/Cargo/Python toolchain detection;
- contract-artifact consistency check;
- `cargo build --workspace --all-targets`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace --all-targets`;
- `tools/verify_offline.ps1`.

## Interpretation

These are independent GitHub-hosted executions in a temporary public verification repository, not executions of the private canonical repository's hosted workflow. The canonical repository remains the source of truth.

The private canonical GitHub Actions environment remains recorded as blocked because its jobs failed before producing usable workflow-step evidence. No paid GitHub upgrade is required or used for this verification path.

## Result

For canonical commit `69c4918e6845fdbbd79495b3fa2c59798831bce0`, Linux and Windows execution evidence is green for build, clippy, tests, contract artifacts, and the platform-specific offline verification scripts.
