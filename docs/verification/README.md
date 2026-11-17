# Verification artifacts and CI workflows

## Current CI ownership split — 2026-11-17

- Canonical source: `panglimanetizen01/one`; candidate PR #95 branch `ci/circleci-migration-20261113`, exact SHA `07d8cbf7422b8340f8efda3c2f9d4425f85b7537`.
- Canonical Linux product CI: CircleCI `one-linux`, terminal SUCCESS on this exact SHA (workflow `0e03e28b-dfb9-4198-b897-3ce5d16dc2d8`, job `b2f536a1-6c87-49b5-9a3f-2db2f2908595`).
- Independent Windows host verification: [run #28](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43865524749), terminal SUCCESS on Windows Server 2025 AMD64; 32/32 steps, native exit 0, output `Budi`; artifact and manifest hashes independently checked.
- Both results bind to the same exact canonical source SHA. Linux-host MinGW cross-compilation is not Windows-host execution evidence.
- GitHub metadata reports the canonical repository as PUBLIC. Keep credentials and private data out of tracked files and public build logs.
- PR #95 remains OPEN / DRAFT / UNMERGED; canonical `main` remains unchanged at `d35db95153f6ddd79885494c72f5aee16c899751`. The migration is not live on main until separately authorized merge.
- The opt-in Linux release-check has not run. No merge, tag, or release is authorized by this CI checkpoint.

## Canonical evidence locations

- CircleCI job logs/artifacts: Linux product and Linux release jobs for the exact ONE source SHA.
- Independent Windows logs/artifacts: exact source pin, verifier SHA, native executable, and runtime manifest in `one-windows-verification`.
- Historical cross-platform evidence remains recorded in `one-v0.1-verification.md`, `one-v0.1-evidence-matrix.md`, and `native-backends-2026-10-19.md`.

## Evidence rules

- Source presence and workflow configuration are not execution evidence.
- A failed or unstarted job does not establish a source-code failure without executable steps.
- Every platform result is bound to its exact canonical source SHA and stated host boundary.
- Windows PASS requires terminal-success Windows workflow plus independent manifest and binary-hash verification.
- A CircleCI Linux PASS does not imply Windows PASS; a Windows verifier PASS does not imply CircleCI Linux PASS.


## Current CircleCI migration candidate — exact-source reconciliation — 2026-11-17 19:23 WIB

- PR #95 current HEAD: `993f76351c7a119bea2e66f7d9e5bca07f392fbc`, OPEN / DRAFT / UNMERGED. Config blob: `9d05f65badffae75e7e22bd1f0962e056b4960bd`.
- The branch contains `Cargo.lock` blob `3fd2cb2be6aa30afa54602c733b2ab192cc91d2b`, identical to PR #102 branch HEAD `32fc9b77bac14ecfc06a6fddd946a5419298e997`. Root `Cargo.toml`, `crates/one-core/Cargo.toml`, and `crates/one-cli/Cargo.toml` blobs also match across PR #95 and PR #102.
- All inspected Cargo build, Clippy, test, vertical-slice, real-network, and release build commands in `.circleci/config.yml` use `--locked`. `tools/check_contract.py` requires the tracked lockfile and checks inline and multiline Cargo build/Clippy/test/run invocations, including offline scripts, for `--locked`.
- Independent Windows verifier run #43 / `44049102193` is PASS for exact ONE source SHA `993f763...` (verifier SHA `56271be82645357c2feaa1134ae2dc1d2ceae734`, Windows job `133922119577`, artifact `14313416141`).
- No CircleCI status context is visible on current PR #95 HEAD `993f763...`; the default `one-linux` workflow is defined without branch filters, but the session has no authenticated CircleCI job-log/API/CLI access to determine why a current status has not appeared. Optional `release-check` with `run_release_checks=true` remains NOT VERIFIED. Static configuration alignment is not CircleCI execution PASS.
- PR #95 remains unmerged, so CircleCI migration is proposed but not active on `main`. Do not infer that the CircleCI error context on dependent PR #106 pipeline #643 is a failure of this PR #95 candidate; the two refs have different config availability. No merge or release.
