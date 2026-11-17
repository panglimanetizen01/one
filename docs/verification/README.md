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


## Completion ledger and release gate — 2026-11-17

The fixed-weight completion ledger and Definition of Done are restored in the canonical documentation surface by [PR #103](https://github.com/panglimanetizen01/one/pull/103):

- [Fixed-weight completion ledger](https://github.com/panglimanetizen01/one/blob/docs/restore-completion-ledger-dod-20261117/docs/verification/one-completion-ledger.md) — F1–F6, C1–C6, and E1–E6; weights remain fixed at 100 points.
- [Definition of Done](https://github.com/panglimanetizen01/one/blob/docs/restore-completion-ledger-dod-20261117/docs/verification/DEFINITION_OF_DONE.md) — release acceptance checklist.

Current disposition is deliberately fail-closed:
- Historical implementation inventory: 92/100 provisional; current exact-SHA verified points: 0/100.
- Historical DoD state: 3 checked / 10 unchecked; the checked items require revalidation against a frozen final source SHA.
- Lockfile candidate `a10a69afdf8675e703f38f1620abc935d8361897` passed GitHub Actions Ubuntu+Windows workspace CI and independent Windows native PE32+ runtime (exit 0, output `Budi`). This is prior-SHA evidence and does not verify the newer process-hardening HEAD.
- PR #102 current HEAD: `47596f8e3f6c2b7a2c58abb12e2a52d673f90ece`; exact-SHA GitHub Actions run #44002036948 is in progress, including the newly added Windows descendant-cleanup regression. Prior run #44000173122 completed SUCCESS on parent SHA `d6f7230...`; it passed bounded timeout/output tests but did not include the descendant test. Run #44000805140 on `c498fc4...` failed only in the descendant test because PowerShell `Start-Process` delegated through shell launch behavior; the test now uses direct `[System.Diagnostics.Process]::Start`. Do not attribute parent-SHA results to the current HEAD.
- CircleCI migration PR #95 remains unmerged; `.circleci/config.yml` is absent from `main`. CircleCI ERROR contexts on product-fix branches are not proof that source tests failed.
- No final candidate SHA is frozen. Release gate remains **NOT PASSED**. No merge or release is authorized by this checkpoint.

