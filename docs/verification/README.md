# Verification artifacts and CI workflows

## Current CI ownership split — 2026-11-14

- Canonical source: private `panglimanetizen01/one`.
- Canonical product CI: CircleCI Linux-only, configured at `.circleci/config.yml`.
- Independent Windows host verification: `panglimanetizen01/one-windows-verification`, workflow `.github/workflows/windows-native-backend-runtime.yml`.
- The canonical CircleCI configuration no longer defines `one-windows`, `release-windows`, or `windows-native-verification` jobs. Linux-host MinGW cross-compilation remains a regression check, not Windows-host evidence.
- PR #95 remains the CircleCI migration review branch; PR #42 in the independent verifier contains the Windows exact-SHA workflow and remains draft/unmerged.
- Windows baseline run [#9](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43589646641) passed on source SHA `195755054bb733f44ff30acf8d3281b4efc820bf`; see [the Windows verification contract](windows-native-verification.md) for manifest/artifact hashes and the strict evidence boundary.
- Run #9 is pre-split baseline evidence only. Do not transfer it to a later source SHA; consult the exact-source pin and manifest for each candidate.

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
