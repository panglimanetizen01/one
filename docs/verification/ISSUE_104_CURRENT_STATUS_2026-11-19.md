# Issue #104 current evidence reconciliation — 2026-11-19

## Exact branch and CircleCI state

- Repository: `panglimanetizen01/one`.
- PR #113: OPEN / DRAFT / UNMERGED.
- Exact PR HEAD: `70ae27cd9dab4429c528d6f01a42377f474c725c`.
- Base branch: `fix/windows-process-verifier-timeouts-20261118`; exact base SHA: `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- GitHub combined status on the exact PR HEAD: `ci/circleci: one-linux` = SUCCESS.
- CircleCI workflow: `44163dc3-9fe7-41d1-a199-f1327b7d4055`.
- CircleCI job: `5a6562fb-9501-40fa-a49c-05966cf2a66b`.
- Job URL: https://app.circleci.com/workflow/44163dc3-9fe7-41d1-a199-f1327b7d4055/job/5a6562fb-9501-40fa-a49c-05966cf2a66b
- User-supplied job summary shows successful progression through Android SDK licenses, Android NDK 27.2.12479018, Rust, contract artifacts, user-documentation consistency, documentation-checker tests, build, Clippy, workspace tests, release CLI build, release CLI smoke test, and cache saving.
- The local CircleCI diagnostic unit suite was run by the user on this exact SHA: 14/14 passed; generated `tools/ci/__pycache__/` was removed and the worktree was reported clean.
- This verifies the current CI candidate; it does not prove that the earlier APT hang's root cause has been identified. The successful install branch deletes its live-monitor log, so no live process history survives a successful install. Record the timeout as NOT REPRODUCED; root cause remains UNDETERMINED.

## Runtime-source identity and independent E4 evidence

- The independent diagnostic verifier branch `probe/windows-obj-dont-reparse-20261118` pins code-bearing source SHA `722d662e7a81711ba3607d26571f4048a292b3ed` in its branch-local `CANONICAL_COMMIT.txt`.
- GitHub compare `722d662...` → `70ae27c...` reports 103 commits ahead, 0 behind. The complete changed-path list contains only `.circleci/config.yml`, documentation files, and `tools/ci/diagnose_circleci.py` plus its tests. No runtime implementation or runtime test file changed in that range. Therefore the filesystem runtime/test source at PR HEAD remains byte-for-byte the code-bearing source pinned by the diagnostic verifier.
- E4 Unix + Windows adversarial matrix: all three jobs SUCCESS — [run #44335797344](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797344). Jobs: Unix no-follow/resource bounds; Windows reparse/handle checks; Windows user-mode `NtCreateFile + OBJ_DONT_REPARSE` probe.
- Cross-platform Ubuntu + Windows Server 2025 matrix: both jobs SUCCESS — [run #44335797200](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797200).
- Debian 12 + Ubuntu 24.04: both jobs SUCCESS — [run #44335797294](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797294).
- Android AArch64 artifact build: SUCCESS — [run #44335797320](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797320); artifact `14442665831`, SHA-256 `3bc37c20a92087464abb087be1fe174fac314dbddd635116656e34ad1cee4123`.
- Android API 35 ARM64 runtime: both build and runtime jobs SUCCESS — [run #44335797240](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797240).
- SEC-05 Linux + Windows: both jobs SUCCESS — [run #44335797365](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797365).
- Subprocess resource guard: SUCCESS — [run #44335797191](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797191).
- Windows native probe evidence is limited to Windows Server 2025 build 26100. Do not generalize it to every Windows version.

## E4 filter inventory readback

The current PR HEAD still contains the exact named tests required by the diagnostic verifier workflow:
- Unix: three `descriptor_relative_read_rejects_*` cases, five `descriptor_relative_write_*` cases, one `descriptor_relative_walk_rejects_*` case, the exact swapped-parent-symlink race test, and the exact final-component mount-boundary no-truncate test.
- Windows: all seven exact tests in the verifier workflow are present, including directory-junction escape, absolute in-root read, outside-root no-side-effect confinement, successful atomic write, oversized write rejection, denied atomic replacement preserving bytes/cleaning the temp sibling, and parent traversal rejection.
- The verifier jobs were executed on the pinned source SHA and their GitHub job records are terminal SUCCESS, not filtered-only or pending.

## Remaining boundaries — do not promote prematurely

- Issue #104 remains OPEN / NOT ACCEPTED. The evidence above materially advances implementation and verification, but does not authorize changing the protected verifier pin or closing the issue.
- PR #44 remains OPEN / DRAFT / UNMERGED. Its diagnostic branch pin is separate from the protected `verification/CANONICAL_COMMIT.txt`.
- Protected verifier pin remains exactly `e50748a1b25922d9090eba1f5fb90548d7c7d003`; it must not be changed without explicit authorization.
- Linux nested writes remain deliberately fail-closed; do not add a path-based fallback.
- macOS and other unsupported platforms remain fail-closed. Android runtime evidence is API 35 ARM64 only.
- Keep PR #113 OPEN / DRAFT / UNMERGED. Do not merge, modify `main`, force-push, delete branches, or change verifier pins.
- A successful CircleCI run is not a substitute for the independent E4 checks; conversely, the independent verifier runs do not replace the CircleCI exact-HEAD status.

## Next safe gate

1. Preserve the exact-SHA evidence above and reconcile any remaining stale status statements in the Issue #104 design, evidence matrix, implementation-status ledger, and user feature status.
2. Review the acceptance/ledger procedure for how a candidate may be proposed for protected-verifier promotion. Do not perform the promotion or close Issue #104 without explicit authorization.
3. Keep unsupported cases fail-closed and continue only with changes supported by a concrete test or documented acceptance gap.
