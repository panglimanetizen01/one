# Verification artifacts and CI workflows

GitHub metadata currently reports the canonical ONE repository as **PUBLIC**. No visibility change was made during this documentation work. Treat public source and build logs as a disclosure boundary; never place credentials or private data in tracked files or logs.

## Current CI ownership split — 2026-11-16

- Canonical source: `panglimanetizen01/one`; candidate PR #95 branch `ci/circleci-migration-20261113`, exact SHA `07d8cbf7422b8340f8efda3c2f9d4425f85b7537`.
- Canonical Linux product CI: CircleCI `one-linux`, terminal SUCCESS on this exact SHA (workflow `0e03e28b-dfb9-4198-b897-3ce5d16dc2d8`, job `b2f536a1-6c87-49b5-9a3f-2db2f2908595`, status ID `—`).
- Independent Windows host verification: [run #28](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43865524749), terminal SUCCESS on Windows Server 2025 AMD64; 32/32 steps, native exit 0, output `Budi`; artifact and manifest hashes independently checked.
- Both results bind to the same exact canonical source SHA. Linux-host MinGW cross-compilation is not Windows-host execution evidence.
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
