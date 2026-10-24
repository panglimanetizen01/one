# Milestone 1 — E1 Standard Library Test Lab

**Scope:** E1 only. This lab does not award completion points by itself and does not authorize merge or release.

## Hosted lab

Workflow: [ONE Milestone 1 — E1 Standard Library Test Lab](../../actions/workflows/milestone-1-e1-lab.yml)

- Run automatically on relevant changes to `milestone/100-ecosystem-20261023`.
- Can also be started with GitHub Actions → workflow → **Run workflow**.
- Prints the exact source SHA and toolchain into the run summary.
- Runs formatting, the complete `one-core` stdlib integration test file, core library unit tests, all workspace targets, Clippy with warnings denied, and the repository contract checker.

## Local lab

From a clean clone of the candidate branch:

```sh
git switch milestone/100-ecosystem-20261023
git pull --ff-only
bash tools/milestone1-e1-lab.sh
```

Prerequisites: a Rust toolchain compatible with `rust-toolchain.toml`, Cargo, Python 3, Git, and the native build prerequisites required by this repository. The script fails immediately on the first failed command and prints the exact tested revision.

## Evidence rules

1. Record the full commit SHA, OS/architecture, Rust/Cargo versions, command exit codes, and complete logs.
2. A local PASS is independent evidence, not a GitHub-hosted CI PASS.
3. Do not mark E1 verified if any command is skipped, fails, or cannot start.
4. A hosted job that finishes without an assigned runner or executed steps is **BLOCKED/INFRASTRUCTURE**, not a source-test result.
5. Only after all required checks pass on the same exact SHA may the E1 ledger be reconsidered. This lab does not change the completion ledger automatically.
6. E2, E3, E4, E6, merge, and release are outside this lab's scope.
