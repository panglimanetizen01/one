# Verification artifacts and CI workflows

GitHub metadata currently reports the canonical ONE repository as **PUBLIC**. No visibility change was made during this documentation work. Treat public source and build logs as a disclosure boundary; never place credentials or private data in tracked files or logs.

## Historical CI ownership snapshot — 2026-11-16

- Canonical source: `panglimanetizen01/one`; candidate PR #95 branch `ci/circleci-migration-20261113`, exact SHA `07d8cbf7422b8340f8efda3c2f9d4425f85b7537`.
- Canonical Linux product CI: CircleCI `one-linux`, terminal SUCCESS on this exact SHA (workflow `0e03e28b-dfb9-4198-b897-3ce5d16dc2d8`, job `b2f536a1-6c87-49b5-9a3f-2db2f2908595`, status ID `—`).
- Independent Windows host verification: [run #28](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43865524749), terminal SUCCESS on Windows Server 2025 AMD64; 32/32 steps, native exit 0, output `Budi`; artifact and manifest hashes independently checked.
- Both results bind to the same exact canonical source SHA. Linux-host MinGW cross-compilation is not Windows-host execution evidence.
- PR #95 remains OPEN / DRAFT / UNMERGED; canonical `main` remains unchanged at `d35db95153f6ddd79885494c72f5aee16c899751`. The migration is not live on main until separately authorized merge.
- The opt-in Linux release-check has not run. No merge, tag, or release is authorized by this CI checkpoint.




## Current PR #95 migration candidate — 2026-11-17

- PR #95 remains OPEN / DRAFT / UNMERGED on branch `ci/circleci-migration-20261113`; latest source HEAD is `e50748a1b25922d9090eba1f5fb90548d7c7d003`. The historical same-SHA Linux + Windows evidence for `07d8cbf...` remains valid only for that older source SHA.
- CircleCI Linux reports SUCCESS for source SHA `e50748a...` (workflow `dc6f3660-328f-4016-8c01-72c3565d0321`, job `64a34e41-9759-4a5c-8e50-b77296730a54`).
- Independent Windows native runtime run #32 passed on source SHA `a10a69afdf8675e703f38f1620abc935d8361897`, which is the PR #102 lockfile candidate, not PR #95 HEAD. Do not combine these results into same-SHA cross-platform parity.
- The current PR #102 process-containment candidate is separate: source HEAD `5d92a17fcc34178c419e6817030058ee17cafc6c`; GitHub Actions run #44006414909 has Ubuntu SUCCESS and Windows still running at the last check. It is not release evidence yet.
- CircleCI is not active on `main` while PR #95 remains unmerged. The release-check remains unproven; no merge/release is authorized.


## Independent verifier security hardening — 2026-11-17

PR #39, #40 and #42 now remove the unnecessary `CANONICAL_READ_TOKEN` secret from exact-source checkout steps because the canonical ONE repository is public. All harness and source `actions/checkout` steps set `persist-credentials: false`, preventing checkout credentials from remaining in the worktree while build/test code runs. The PRs remain OPEN / DRAFT / UNMERGED; these security changes still require exact workflow execution and review.

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
- PR #102 latest HEAD: `54a68a7673247c0fe23aba24c5e86f7f77372c60`; no GitHub Actions run is visible yet. Current HEAD is NOT VERIFIED. The previous SHA `385d8e0...` passed Ubuntu and Windows build but failed Clippy before tests on an unused `PROCESS_TERMINATE` import; latest source uses that access right in the `OpenProcess` mask.
- Previous diagnostic SHA `5d92a17fcc34178c419e6817030058ee17cafc6c`, run #44006414909: Ubuntu SUCCESS; Windows build/Clippy SUCCESS; Windows tests 119 passed / 1 failed due `ParseBoolError` parsing PowerShell's capitalized `True`. Direct child was confirmed in the intended Job Object; descendant membership remains NOT VERIFIED.
- Earlier source SHA `a472b3bd9e410f1d42f37f31cfcb4e5c11f399a8`, run #44002493159: Ubuntu SUCCESS; Windows FAILURE (119 passed / 1 failed), descendant PID `8864` survived (`WAIT_TIMEOUT=258`).
- CircleCI migration PR #95 remains unmerged; `.circleci/config.yml` is absent from `main`. CircleCI ERROR contexts on product-fix branches are not proof that source tests failed.
- CircleCI status on PR #95 exact HEAD `e50748a...` is SUCCESS (`ci/circleci: one-linux`, workflow `dc6f3660-328f-4016-8c01-72c3565d0321`, job `64a34e41-9759-4a5c-8e50-b77296730a54`). `.circleci/config.yml` is absent from `main` and PR #102 because PR #95 remains unmerged; generic CircleCI Pipeline ERROR contexts on those refs are consistent with missing migration config, not product-test failure. PR #102 adds `Cargo.lock` and expects `--locked`; the PR #95 config does not yet pass `--locked`. After authorized integration, update all CircleCI Cargo commands and make the contract checker require lockfile enforcement.
- No final candidate SHA is frozen. Release gate remains **NOT PASSED**. No merge or release is authorized by this checkpoint.

