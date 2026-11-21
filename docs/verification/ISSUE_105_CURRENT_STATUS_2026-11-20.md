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


## Process adapter smoke-fixture correction — 2026-11-21

Latest source commit: `b4b4b3ee82e013eb3c001606820deaa76d50a9df` (`crates/one-core/src/adapter.rs`).

- Replaced the no-argument `rustc` smoke target (which can exit nonzero when no input file is supplied) with `whoami`, a no-argument command that emits output on Linux, Android and Windows.
- Cfg-gated the smoke test to the platforms where ProcessAdapter advertises native containment, and included Android in the process-platform metadata regression.
- Test source only; NOT RUN. No build/CI/verifier evidence is claimed. Issue #105 remains OPEN / NOT ACCEPTED.

## Process-containment primitive review and adversarial source update — 2026-11-21

Native implementation boundary reviewed in `crates/one-core/src/interop.rs`:

- Linux/Android: `CommandExt::process_group(0)` creates an invocation process group; child `pre_exec` installs `PR_SET_NO_NEW_PRIVS` plus inherited seccomp before `exec`. The filter denies `setsid`, `setpgid`, `unshare`, `setns`, namespace-bearing `clone` flags, and x32 ABI entry; `clone3` returns `ENOSYS` so compatible runtimes may fall back to `clone`, whose namespace flags are filtered. Cleanup uses `waitid(WNOWAIT)` to retain the direct-child PID, sends SIGKILL to the invocation group on normal exit, timeout, or output overflow, then reaps the direct child.
- Windows: `PROC_THREAD_ATTRIBUTE_JOB_LIST` assigns the job during `CreateProcessW` before child execution; `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` is configured; `IsProcessInJob` verifies assignment; `TerminateJobObject` terminates remaining members and cleanup waits for active process count to reach zero. Job close remains the final safety net.
- macOS: no equivalent non-escapable descendant-containment backend is implemented; process execution remains fail-closed and must not be advertised as supported.

Source/test commit `5e79f351b887f3dee7f77ae3a2e8dd86e5d108ea` isolates each Linux/Android escaped-descendant test in a unique working directory so parallel test cases cannot share or overwrite one PID fixture. Commit `18bd42f1eecb7ca40dcf88a5b8d6456d15361f2e` adds direct `setpgid(0, 0)` escape probes for normal completion, timeout, and output overflow, complementing the existing `setsid` cases. Commit `477537c3e6ceabc79ff56ef04a3983cd3bae29ea` corrects test-attribute placement after source review.

All these changes are source-only and NOT BUILT / NOT TESTED. No process containment PASS or Issue #105 acceptance is claimed. Next evidence must run the exact-SHA adversarial matrix on supported Linux architectures and real Android/Termux, and re-run Windows Job Object descendants tests on the same candidate. Keep macOS fail-closed unless a separately reviewed native primitive can provide equivalent guarantees.

## 2026-11-21 — macOS process containment primitive research

Seatbelt/Sandbox Policy Language exposes a `process-fork` operation that can deny `fork`/`vfork`. This is a plausible way to prevent descendants rather than trying to kill an escaped process tree. It is NOT yet a production implementation: the current evidence does not prove whether `posix_spawn`, `posix_spawnp`, Foundation `NSTask`/Process, shell helpers, or other process-creation paths are denied by the same rule, nor does it prove the exact wrapper/profile inheritance behavior on supported macOS versions.

Do not enable macOS process execution on this basis. A native proof must launch an untrusted child under a Seatbelt profile, verify the initial `exec` still works, then attempt fork, vfork, posix_spawn, posix_spawnp, system/popen, Python/Node subprocess APIs, setsid/setpgid, and descendant cleanup under normal exit, timeout, and output overflow. The host must verify no child or grandchild survives. Until that exact native matrix passes, macOS remains fail-closed and Issue #105 remains OPEN / NOT ACCEPTED.

Linux/Android inherited seccomp + process-group cleanup and Windows Job Object containment remain the only implemented production backends. Recent setpgid adversarial source tests remain NOT BUILT / NOT TESTED.

## 2026-11-21 — macOS Seatbelt process-creation guard source candidate

Source commits: `ff60c7bd8a058f0950a0d67bb596d94feb8e6803` (runtime wrapper) and `ab799dd5faa5c50b5c75069c1d0b91a43511df93` plus `3bad0196e9352871d09cb9b966cf6b393a866603` (native regression probes and failure cleanup). Latest code-bearing process candidate: `3bad0196e9352871d09cb9b966cf6b393a866603` (NOT BUILT / NOT TESTED).

- macOS `run_process_macos` now launches `/usr/bin/sandbox-exec -p "(version 1) (allow default) (deny process-fork)" -- <program> ...`. The wrapper enters Seatbelt and execs the target. The profile intentionally denies direct process creation while allowing `exec` replacement; timeout and output caps remain enforced by the host wrapper. Missing `sandbox-exec` fails closed.
- Native XNU source supports the mechanism: `fork1()` invokes `mac_proc_check_fork()` before process creation and handles both `PROC_CREATE_FORK` and `PROC_CREATE_SPAWN`; the `posix_spawn` path calls `fork1(..., PROC_CREATE_SPAWN, ...)`. Thus direct `fork`, `vfork`, `posix_spawn`, and `posix_spawnp` should be denied by the same policy. Sources: `https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/kern_fork.c` and `https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/kern_exec.c`. `sandbox-exec(1)` documents that it enters the selected profile and execs the command; the tool is deprecated, so the backend must remain fail-closed if removed or rejected.
- Added macOS-only ignored helper probes for `fork`, `vfork`, `posix_spawn`, `posix_spawnp`, plus a positive `exec`-replacement probe, deadline termination, and stdout overflow. These tests are source-only and NOT RUN.
- **Important limitation:** this is a single-process execution mode, not a descendant-tree backend. It denies the target's direct child creation rather than permitting children and later killing a tree. Brokered process launches through launchd or other Mach services are not yet proven blocked. `ProcessAdapter` metadata intentionally continues to exclude `PlatformKind::Macos`; do not advertise macOS process capability until native tests cover direct and brokered process creation and product semantics accept the narrowed mode.
- Issue #105 remains OPEN / NOT ACCEPTED. Linux/Android inherited seccomp + process-group cleanup and Windows Job Objects remain the only advertised process-containment backends. No macOS native build/test/CI result is claimed.

## 2026-11-21 — latest macOS Seatbelt source SHA

- Latest combined source/test candidate is `79593d2f225284eff841e67c8eb9a1e005611981`. This commit only clarifies the direct fork/posix_spawn boundary and that brokered launchd/Mach-service launches remain unproven.
- Runtime implementation and probes remain source-only / NOT BUILT / NOT TESTED. `ProcessAdapter` remains unadvertised on macOS. Issue #105 remains OPEN / NOT ACCEPTED.

## 2026-11-21 — XNU process-group escape policy boundary

Inspection of XNU `bsd/kern/kern_prot.c` shows `setsid()` and `setpgid()` call `setsid_internal()` / `enterpgrp()` directly; the corresponding paths do not invoke a `mac_proc_check_setpgid` or `mac_proc_check_setsid` hook. `bsd/kern/kern_proc.c::enterpgrp` also performs the process-group transition without such a MAC policy hook. Therefore a Seatbelt profile cannot be assumed to prevent a child from escaping its process group while still allowing child creation. This is why the macOS candidate denies `process-fork` entirely instead of pretending to provide tree containment.

The resulting guarantee is deliberately narrower: one target process may execute and replace itself, but it cannot create direct fork/vfork/posix_spawn children. Brokered launchd/Mach-service creation remains a separate unresolved route. `ProcessAdapter` remains unavailable on macOS. Sources: `https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/kern_prot.c` and `https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/kern_proc.c`.
