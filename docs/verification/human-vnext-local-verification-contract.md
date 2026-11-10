# Human-vNext Local Verification Contract (Review Required)

**Status:** HISTORICAL / SUPERSEDED as a hosted-CI replacement. This proposal must not be used as a substitute for CircleCI or as permission to implement in Termux.  
**Scope:** The isolated human-instruction vNext foundation on `spec/human-instruction-grammar-vnext-20261022`.  
**Purpose:** Provide reproducible, exact-commit evidence when GitHub Actions jobs fail before executing any steps.

> **Current operational rule (2026-11-11):** GitHub is the source of truth and CircleCI is canonical CI for the One source repository. Termux may only perform read-only independent verification after the matching GitHub SHA has passed CircleCI. Do not edit, repair, commit, or push One changes from Termux. This older proposal is retained only as historical context; any local check is supplementary and cannot replace CircleCI.

## What this contract can establish

A successful run establishes that the named GitHub commit passed the checks executed in the specified Debian 12 environment. It does **not** establish that GitHub-hosted CI passed, that an unavailable Android native execution test passed, or that other operating systems were tested.

## Required environment

- Debian 12 with Git, Python 3, Rust toolchain, and Cargo installed.
- The verification checkout is fetched from GitHub, not copied from an uncommitted development tree.
- The worktree is clean and detached at the exact branch commit under test.
- For the current test suite, `ONE_ANDROID_CLANG` may point to a documented wrapper used by the repository's Android-target checks. This does not count as executing the ignored Android AArch64 native-run test.

## Required procedure

From the repository's bound working directory, fetch the target branch and detach the verification worktree at its fetched remote-tracking ref. Record the full commit SHA printed by Git, then run these commands in order:

```sh
python3 tools/check_contract.py
git diff --check
cargo build --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
ONE_ANDROID_CLANG=/tmp/one-android-clang-wrapper cargo test --workspace --all-targets
```

The wrapper path is environment-specific. If it does not exist or is not valid, stop and report the failure; do not silently omit the environment variable or claim equivalent coverage.

## Pass criteria

All of the following are required:

1. The contract checker exits zero and reports the required artifact and fixture inventory.
2. `git diff --check` exits zero.
3. Workspace build exits zero.
4. Clippy exits zero with warnings denied.
5. Every executed test suite exits zero; any ignored tests and their reasons are recorded.
6. The exact full commit SHA, OS/environment, command list, outputs, ignored tests, and date are recorded in the completion ledger.
7. No changes are made to `main` or committed from the local verification checkout.

Any nonzero exit, incomplete output, dirty verification checkout, or unrecorded test skip makes the run inconclusive or failed rather than passed.

## Evidence handling and limitations

- Evidence applies only to the exact recorded commit; later commits require another run.
- A local run must never be labeled GitHub Actions/hosted CI.
- The Android native execution test requiring Android AArch64 Clang remains unverified when ignored.
- This procedure does not independently verify Windows/macOS behavior, hosted runner configuration, or release artifacts.
- A human reviewer must explicitly approve this contract and decide whether its scope is an acceptable substitute for the hosted-CI portion of the 30% gate. Until that approval and the remaining F1–F6 acceptance evidence are recorded, verified points must not be increased and the 30% gate remains NOT PASSED.


## Supersession notice

The procedure above is not an approved fallback CI route under the current operating contract. Do not use it to replace failed/unavailable CircleCI jobs. If an independent local check is explicitly requested, perform it only after CircleCI has proven the same GitHub SHA, in an isolated read-only checkout, and report it as independent local evidence.
