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
