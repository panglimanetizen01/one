# Issue #105 — Unix subprocess descendant containment status

**Status:** OPEN / NOT IMPLEMENTED / NOT ACCEPTED

**Current code source reviewed:** `722d662e7a81711ba3607d26571f4048a292b3ed`; relevant `interop.rs` behavior is unchanged in the Issue #104 candidate `c7f460a4e38be61de635ad5af9f7ff987a941f01`.

## Current implementation

- Direct-child execution deadline: 10 seconds.
- Stdout/stderr capture: 1 MiB per stream.
- Unix readers are nonblocking. Reader shutdown uses `READER_IDLE_GRACE = 20 ms` and `READER_SHUTDOWN_GRACE = 100 ms`, then records an explicit incomplete-shutdown error instead of blocking the caller indefinitely.
- Unix cleanup kills the original process group only. A descendant that calls `setsid()` or creates another process group can survive after the invocation returns.
- The test `unix_reader_shutdown_does_not_wait_for_escaped_session_descendant` proves bounded caller return, but the test manually kills the escaped descendant afterward. It does not prove production cleanup kills it.
- Windows uses a Job Object assigned during process creation and terminates remaining job members during cleanup.

## Exact feasibility probes

| Probe | Result | Evidence |
|---|---|---|
| cgroup v2 availability | Present; `cgroup.kill` exists | verifier run [#44587279712](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44587279712) |
| Unprivileged child cgroup creation under current runner cgroup | Denied: `CGROUP_SUBTREE_NOT_DELEGATED` | run #44587279712 |
| Unprivileged PID namespace | Unavailable: `/proc/self/uid_map: Operation not permitted` | verifier run [#44587435053](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44587435053) |
| Host-admin cgroup subtree | Temporary subtree/control files could be created/chowned, but moving a child into `cgroup.procs` failed: `DELEGATED_CGROUP_PROCS_WRITE_FAILED` | verifier run [#44587792952](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44587792952) |

The exact kernel denial for the final migration failure was not captured. Do not infer a more specific cause. These results show that the default Ubuntu 24.04 hosted runner cannot currently validate either cgroup-v2 or unprivileged PID-namespace containment.

## Required next step

Use a host/self-hosted Linux runner that explicitly delegates source-cgroup membership and a writable child subtree, or permits user/PID namespaces. Implement containment through a host-owned authority and fail closed when it is unavailable. Add real adversarial tests proving no escaped descendant remains after normal return, timeout, and output overflow. Preserve the bounded reader shutdown and Windows Job Object path.

If a robust provider is not feasible on a platform, narrow and document the platform guarantee and do not claim process-tree containment or sandboxing. Keep the issue OPEN until the acceptance evidence exists.

## Evidence boundary

SEC-05 run [#44566187162](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44566187162) passed on its pinned source for bounded timeout/output behavior; it does not prove escaped-session cleanup. On candidate source `c7f460a...`, the Windows SEC-05 job passed while the Linux job remained in cross-compiler installation at the last read.


## IMPLEMENTATION-FIRST CONTINUATION — 2026-11-21

This checkpoint follows the owner's explicit instruction to finish the in-scope implementation before starting broad tests, verification, or release-gate reruns. It does not change any acceptance criteria or protected state.

### Exact candidate identity
- ONE PR #113 remains OPEN / DRAFT / UNMERGED; branch `security/issue-104-root-confinement-20261118`; base `fix/windows-process-verifier-timeouts-20261118` at `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Latest code-bearing implementation source: `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` (Issue #105 Linux/Android child-PID retention using `waitid(WNOWAIT)`).
- Test-fixture correction commits: `46c777e70f396cfee6d796d0667769dabe2d7208` and `2600ec18d4e3fb05558ca50d7cf466638e1ab015`; both affect only `crates/one-cli/tests/filesystem_write.rs`.
- PR #113 HEAD after those commits: `2600ec18d4e3fb05558ca50d7cf466638e1ab015`. No source implementation or CI config changed in those two commits.

### Implementation/source inventory — presence is not execution evidence
- Issue #104 source contains Linux retained-root `openat2` reads, atomic direct-child writes, guarded detached-mount nested writes with fail-closed fallback; Windows native handle-relative/reparse-point backend; Android direct-child read/write with no-follow and `statx(STATX_MNT_ID)`; macOS direct-child read/write with no-follow and filesystem identity checks; exclusive temporary files, sync/rename paths, and hard-link rejection.
- Issue #105 source contains inherited Linux/Android seccomp process-group containment, Windows Job Object containment, macOS process execution fail-closed, and Linux/Android `waitid(WNOWAIT)` so the direct child PID is retained until process-group cleanup.
- Source contains three direct-`fork` + `setsid` adversarial regressions for normal completion, timeout, and output overflow, with escaped-PID recording/cleanup. These tests have NOT been run on source `72eae...`.
- Android/macOS nested filesystem paths and macOS process execution remain fail-closed. Do not claim broader support.

### Test-fixture implementation corrections
- Replaced the incorrect “nearest manifest selects root” expectation with the contract-compliant host working-directory root test.
- Added an explicit CLI test runner working-directory helper and used it where the trusted host root must be controlled by the fixture.
- Updated Linux nested-write integration expectations: success is accepted only when the detached-mount backend works; otherwise only `UnsupportedPlatform` with unchanged target bytes and no leaked temp file is accepted.
- Restricted the unsupported-platform fixture to targets without native filesystem backends; Android and macOS are no longer incorrectly classified as unsupported.
- Commits use `[skip ci]` by design. No tests/builds/verifier workflows were run as part of this implementation-first checkpoint.

### CI and acceptance boundary
- Exact PR HEAD `64cea5895d930b8621a7e31209676057d275fdb3` had GitHub status `ci/circleci: one-linux = FAILURE` (workflow `b7b0bbab-06dc-4cbc-88cf-f54624594d8f`, job `6e1d368b-8055-4cfe-a64e-858f9bd2fcdd`). Provider pipeline revision, job membership, logs, and root cause were not obtained; no cause is inferred.
- The newer commits `46c777e...` and `2600ec1...` intentionally skip CI and have no status contexts. No PASS is claimed for the new HEAD.
- Issue #104 and Issue #105 remain OPEN / NOT ACCEPTED until the required exact-SHA evidence is produced. This checkpoint records implementation/source state only; it does not promote ledger points, DoD, verifier pins, or release status.

### Next implementation work
Continue contract-to-source integration review of the existing v0.1 implementation scope and close any remaining production-code gaps before the broad test matrix. Do not rerun B-001 diagnostics, existing exact-SHA verifier suites, or historical gates without a relevant input change. Preserve fail-closed behavior and all protected state.
