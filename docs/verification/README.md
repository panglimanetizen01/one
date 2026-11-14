# Verification artifacts and CI workflows

## Current CI ownership split — 2026-11-14

- Canonical source: private `panglimanetizen01/one`.
- Canonical product CI: CircleCI Linux-only, configured at `.circleci/config.yml`.
- Independent Windows host verification: `panglimanetizen01/one-windows-verification`, workflow `.github/workflows/windows-native-backend-runtime.yml`.
- The canonical CircleCI configuration no longer defines `one-windows`, `release-windows`, or `windows-native-verification` jobs. Linux-host MinGW cross-compilation remains a regression check, not Windows-host evidence.
- PR #95 remains the CircleCI migration review branch; PR #42 in the independent verifier contains the Windows exact-SHA workflow and remains draft/unmerged.
- Current post-split Windows run [#10](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43591754838) completed SUCCESS for exact source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642`; its manifest verifier checkout SHA is `07cd54d440c4f22910e4ed8417f4e6515339fb8d`. The downloaded ZIP digest and executable SHA-256 were independently checked; see [the Windows verification contract](windows-native-verification.md).
- Historical run #9 remains pre-split baseline evidence for source SHA `195755054bb733f44ff30acf8d3281b4efc820bf` only. Do not transfer it to a later source SHA.
- CircleCI on the same current candidate SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642` reports `ci/circleci: one-linux` FAILURE (workflow `bb9174a8-ac7f-42e1-84ed-1ce816267d64`, job `301edf07-748d-4adb-83b0-ba4dc8594ac9`). The first failing step remains UNKNOWN without the authenticated CircleCI log; therefore the overall cross-platform candidate is not PASS.

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
