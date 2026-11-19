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
- PR #44 was subsequently marked ready and merged with explicit user authorization; the exact merge commit and post-merge pin are recorded below.
- At the time of the initial evidence snapshot, protected verifier pin was `e50748a1b25922d9090eba1f5fb90548d7c7d003`; it was subsequently promoted through the explicitly authorized PR #44 merge. Current protected pin is recorded in the promotion section below.
- Linux nested writes remain deliberately fail-closed; do not add a path-based fallback.
- macOS and other unsupported platforms remain fail-closed. Android runtime evidence is API 35 ARM64 only.
- Keep PR #113 OPEN / DRAFT / UNMERGED. Do not merge, modify `main`, force-push, delete branches, or change verifier pins.
- A successful CircleCI run is not a substitute for the independent E4 checks; conversely, the independent verifier runs do not replace the CircleCI exact-HEAD status.

## Next safe gate

1. Preserve the exact-SHA evidence above and reconcile any remaining stale status statements in the Issue #104 design, evidence matrix, implementation-status ledger, and user feature status.
2. Review the acceptance/ledger procedure for how a candidate may be proposed for protected-verifier promotion. Do not perform the promotion or close Issue #104 without explicit authorization.
3. Keep unsupported cases fail-closed and continue only with changes supported by a concrete test or documented acceptance gap.


## Protected-verifier promotion gap — readback 2026-11-19

The diagnostic PR #44 and protected `verification` branch are intentionally different:

- Protected branch pin `CANONICAL_COMMIT.txt` remains `e50748a1b25922d9090eba1f5fb90548d7c7d003`.
- Protected branch E4 workflow still has the older inventory: 2 Linux read-rejection matches, 1 Linux write-rejection match, 1 walk-rejection match, and older database/governance/network test counts. It does not yet contain the added Windows atomic-replacement-denial test or the separate native `OBJ_DONT_REPARSE` contract-probe job.
- PR #44's three-file diff updates the diagnostic branch's E4 workflow to the current source inventory (3 read-rejection matches, 5 write matches, 1 walk match, exact race + mount no-truncate tests, current supporting test counts), adds the Windows denied-atomic-replacement regression, adds the Windows native contract-probe job, enables the diagnostic `probe/**` push trigger, and updates only the diagnostic branch's `CANONICAL_COMMIT.txt` to `722d662...`.
- PR #44 remains OPEN / DRAFT / UNMERGED. Its diagnostic branch runs have all completed successfully, but none of this changes the protected `verification` branch.
- This is the remaining verifier-governance/acceptance gate, not an unverified runtime code claim. Do not merge PR #44, alter the protected pin, or close Issue #104 without explicit authorization.
- PR #44 conversation comment ID: `7008358258`; PR #113 conversation comment ID: `7008342294`.


## AUTHORIZED PROTECTED VERIFIER PROMOTION — 2026-11-19

- User explicitly authorized promotion of PR #44 to protected branch `verification`.
- PR #44 was marked ready for review and merged successfully. Merge commit: `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`.
- PR #44 head immediately before merge: `75eddb0489947f473a8130d246a3f5d79711d693`; base branch: `verification`; merge used expected-head SHA guard. PR URL: https://github.com/panglimanetizen01/one-windows-verification/pull/44.
- Post-merge readback from protected branch `verification`: `CANONICAL_COMMIT.txt` = `722d662e7a81711ba3607d26571f4048a292b3ed`. The protected workflow contains the new Windows `NtCreateFile + OBJ_DONT_REPARSE` probe, the Windows denied-atomic-replacement regression, and the updated Linux read/write E4 test inventory.
- The diagnostic branch's seven relevant workflow runs were all terminal SUCCESS before merge: #44335797294 (Debian 12 + Ubuntu 24.04), #44335797191 (resource guard), #44335797320 (Android AArch64 artifact build), #44335797200 (Ubuntu + Windows Server 2025 cross-platform), #44335797344 (three E4 jobs: Unix, Windows reparse/handle, Windows native OBJ_DONT_REPARSE), #44335797365 (SEC-05 Linux + Windows), and #44335797240 (Android API 35 ARM64 build + runtime).
- Important evidence boundary: these are successful diagnostic-branch runs on the candidate. The post-merge push-triggered protected-branch runs are not yet visible in the available workflow-run readback; do not call the protected post-merge execution green until those exact runs are confirmed. The workflow's push trigger includes branch `verification` and path filters for the canonical pin and E4 workflow.
- The PR body records CircleCI failure on source SHA `722d662...`; cause remains unknown from that historical run. The later CircleCI `one-linux` run on `70ae27c...` was SUCCESS, and compare confirms runtime/test paths unchanged between those source SHAs; only CircleCI config, docs, and diagnostics changed. Current ONE PR #113 HEAD `d6b4c4e781a8d1ace85b2657626d4e02295005c8` has CircleCI status PENDING (workflow `08114a30-dac4-4ee5-9d4d-dc075ef9adff`, job `ae78409c-7e8f-4063-b0f8-faf0c7eee3d7`).
- PR #113 remains OPEN / DRAFT / UNMERGED; base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`. Issue #104 remains OPEN / NOT ACCEPTED. No change was made to ONE `main` or the PR #113 state.
