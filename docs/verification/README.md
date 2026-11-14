# Verification artifacts and CI workflows

## Current CI ownership split — 2026-11-14

- Canonical source: private `panglimanetizen01/one`.
- Canonical product CI: CircleCI Linux-only, configured at `.circleci/config.yml`.
- Independent Windows host verification: `panglimanetizen01/one-windows-verification`, workflow `.github/workflows/windows-native-backend-runtime.yml`.
- The canonical CircleCI configuration no longer defines `one-windows`, `release-windows`, or `windows-native-verification` jobs. Linux-host MinGW cross-compilation remains a regression check, not Windows-host evidence.
- PR #95 remains the CircleCI migration review branch; PR #42 in the independent verifier contains the Windows exact-SHA workflow and remains draft/unmerged.
- Latest post-split Windows run [#11](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43592922789) completed SUCCESS for exact source SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858`; manifest verifier checkout SHA `4435dbac04103c140b2ec14abc97d816ade28be9`. Its downloaded ZIP digest, manifest SHA, executable SHA-256, and PE32+ AMD64 header were independently checked; see [the Windows verification contract](windows-native-verification.md).
- Run #10 remains a separate terminal SUCCESS for predecessor source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642`; run #9 remains the earlier pre-split baseline. Neither result transfers to another SHA.
- CircleCI on the same current source SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858` reports `ci/circleci: one-linux` FAILURE (workflow `ae7233e9-6146-417e-bef9-17b236a93716`, job `a8c73406-8277-4c6b-83d1-5d0ed85670a2`). The first failing step remains UNKNOWN without the authenticated CircleCI log; therefore the overall cross-platform candidate is not PASS.

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
