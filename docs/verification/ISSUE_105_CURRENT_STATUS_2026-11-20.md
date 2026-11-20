# Issue #105 — Unix subprocess descendant containment status

**Status:** OPEN / PARTIAL IMPLEMENTATION / NOT ACCEPTED

**Latest Issue #105 source correction:** `0cceeb447fbe2ae59b8e3da78dee3ce8d9959830` (`crates/one-core/src/interop.rs`); it removes a duplicate `cleanup_test_escaped_group` test helper. The process-containment implementation is partial and still NOT ACCEPTED.

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


## Implementation-first source update — 2026-11-21

Latest code-bearing source `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` now includes Linux/Android inherited seccomp rules that prevent descendants from leaving the invocation process group or creating namespaces, plus `waitid(WNOWAIT)` observation so the direct child PID is retained until process-group SIGKILL cleanup. Windows Job Object containment remains the native Windows path; macOS process execution remains fail-closed. The source contains three direct-`fork` + `setsid` adversarial regressions for normal completion, timeout, and output overflow, including escaped-PID recording and cleanup. These tests have NOT RUN on this source SHA. This is an implementation/source update only; Issue #105 remains OPEN / NOT ACCEPTED until exact-SHA adversarial execution proves no escaped descendant survives each path. Do not infer success from source presence.


## Subprocess resource-bound follow-up — 2026-11-21

The code-bearing candidate advanced to `cd95e5ec2234a2909ef79bf8b25fe65296fab5ed`. In addition to the Linux/Android seccomp + `waitid(WNOWAIT)` containment source and Windows Job Object backend recorded above, the process implementation now explicitly names and checks a 2 MiB aggregate stdout/stderr capture ceiling (1 MiB per stream), alongside the 10-second deadline. Regression tests exist for stdout overflow, stderr overflow, timeout, normal exit/status/stream preservation, and the three direct-fork + `setsid` adversarial cases. None has been run on this candidate. Issue #105 remains OPEN / NOT ACCEPTED; related Issue #81 remains OPEN pending exact-SHA execution and cleanup evidence.


## Duplicate subprocess-test helper compile blocker removed — 2026-11-21

- Source commit: `0cceeb447fbe2ae59b8e3da78dee3ce8d9959830` (`crates/one-core/src/interop.rs`).
- Removed the second identical `cleanup_test_escaped_group` definition from the `interop.rs` test module. The duplicate would produce a Rust duplicate-definition error when compiling the test module; only one definition remains in source.
- This is a source-level compile-blocker correction, not a compile/test result. No build, test, CI, verifier, or platform matrix was run. Issue #105 remains OPEN / NOT ACCEPTED.


## Current implementation status — 2026-11-21

The earlier "Current implementation" description above is a historical pre-containment snapshot and is superseded by the implementation-first updates below.

- Linux and Android source now installs an inherited seccomp filter before exec to prevent descendants from leaving the invocation process group or creating namespaces; cleanup observes direct-child exit with `waitid(WNOWAIT)`, kills the retained group, then reaps the child.
- Windows uses the existing Job Object assigned during process creation and terminates remaining job members during cleanup. macOS remains fail-closed.
- The source contains adversarial fork + `setsid` regressions for normal completion, timeout, and output overflow, plus stdout/stderr and deadline bounds. The duplicate test helper was removed in `0cceeb447fbe2ae59b8e3da78dee3ce8d9959830`.
- No exact-SHA build or adversarial test run has been performed on this candidate. Issue #105 remains OPEN / NOT ACCEPTED; source presence is not process-tree acceptance.
