# Milestone 1 — E2 Package and Provenance Test Lab

**Scope:** E2 deterministic packaging, module-lock behavior, and provenance verification tests. This lab does not authorize a release and does not award E2 completion points automatically.

## Hosted lab

Workflow: [ONE Milestone 1 — E2 Package and Provenance Test Lab](../../actions/workflows/milestone-1-e2-lab.yml)

- Runs on relevant package/CLI/runtime changes on `milestone/100-ecosystem-20261023`, on pull requests targeting `main`, and by manual dispatch.
- Prints the exact checked-out source SHA and Rust/Cargo versions.
- Runs the provenance cryptographic unit tests, CLI package/lock/provenance integration tests, full workspace tests, and Clippy with warnings denied.
- Uses synthetic test keys created inside test fixtures. It does not read or print the protected release environment's signing seed or trusted-publisher secret.

## Local lab

From a clean checkout of the candidate branch:

```sh
git switch milestone/100-ecosystem-20261023
git pull --ff-only
cargo --locked test -p one-cli --bin one package_provenance::tests
cargo --locked test -p one-cli --test cli
cargo --locked test --workspace --all-targets
cargo --locked clippy --workspace --all-targets -- -D warnings
```

Prerequisites: Rust/Cargo matching `rust-toolchain.toml`, Git, and the repository's native build prerequisites.

## Evidence and security boundaries

1. Record exact source SHA, OS/architecture, toolchain versions, every command's exit code, and complete logs.
2. Verify both positive and negative cases: deterministic package/lock bytes, stale-lock rejection, trusted and untrusted publishers, altered package digest, tampered provenance metadata, changed source tree, malformed dependency declarations, and bounded reads.
3. A test using synthetic keys is not evidence that the protected `one-release` environment is configured correctly.
4. A workflow with no runner or no executed steps is **BLOCKED/INFRASTRUCTURE**, not a pass or source-test failure diagnosis.
5. Release acceptance additionally requires a protected release run and independent verification using an independently authenticated publisher trust store. Never paste signing secrets into chat, source control, logs, or artifacts.
6. Do not mark E2 verified until all acceptance criteria in `docs/verification/DEFINITION_OF_DONE.md` pass on the same frozen SHA. This lab does not change the completion ledger automatically.
