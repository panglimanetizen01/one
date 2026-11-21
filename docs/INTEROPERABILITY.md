# One Interoperability Contract

## Principle

One must integrate with the existing technical ecosystem instead of becoming an isolated language island.

## Target ecosystems

The interoperability roadmap includes:

- C and C++
- Python
- Java/JVM
- JavaScript/Node.js
- HTML/CSS/web tooling
- operating-system APIs
- native libraries
- external processes and CLI tools
- databases
- network services
- cloud/platform APIs

## Boundary types

Interop may use:

- FFI/ABI;
- process execution;
- generated source/artifacts;
- stable APIs;
- protocol clients;
- shared libraries;
- platform APIs;
- package/module interfaces.

The selected mechanism must be explicit and documented.

## Semantic rule

Interop is an implementation boundary, not a license to change One's semantics. External systems must be represented through typed contracts where practical.

## Security rule

Interop must respect capability, scope, policy, and audit requirements. Calling a native library or external command must not bypass governance.

## Toolchain rule

The One compiler/runtime should expose stable interfaces so existing build systems, CI, package systems, editors, and IDEs can invoke One without embedding One-specific assumptions into the host.

## Progressive adoption

Users should be able to introduce One into an existing project incrementally rather than requiring an all-at-once rewrite.


## Process execution containment boundary — Issue #105 — 20 November 2026

The current process adapter enforces a 10-second direct-child deadline and 1 MiB capture limit per output stream. On Unix, stdout/stderr readers are nonblocking and have a 100 ms shutdown grace (20 ms idle grace); this prevents an escaped pipe writer from keeping the caller blocked indefinitely. However, cleanup still kills only the original process group. A descendant that calls `setsid()` or creates a new process group can survive after the invocation returns. Bounded reader shutdown is **not** process-tree containment or a sandbox.

Windows uses a Job Object assigned during process creation and terminates remaining job members during cleanup. The cross-platform guarantee is therefore not identical.

Evidence boundary: SEC-05 run #44566187162 passed on its pinned source `722d662...` for the bounded timeout/output contract, but does not prove escaped-session cleanup. On candidate source `c7f460a4e38be61de635ad5af9f7ff987a941f01`, the Windows SEC-05 job #135776430245 is SUCCESS while the Linux job #135776429279 remains IN_PROGRESS at native cross-compiler installation. Issue #105 remains OPEN; no process-tree containment claim is made.


## Issue #105 — containment feasibility boundary — 2026-11-20

The default Ubuntu 24.04 GitHub-hosted runner has cgroup v2 and `cgroup.kill`, but the job's current service cgroup is not delegated. A temporary child cgroup could be created and its control files chowned, but moving a child process into `cgroup.procs` failed. Unprivileged PID namespace creation also failed with `Operation not permitted`. See [Issue #105 current status](verification/ISSUE_105_CURRENT_STATUS_2026-11-20.md) for exact run IDs and results.

Do not implement or claim a cgroup/PID-namespace containment backend based only on kernel feature presence. Integration verification requires a host/self-hosted runner that explicitly delegates source-cgroup membership and a writable subtree, or permits user/PID namespaces. Until then, Unix process-group cleanup remains best-effort and is not process-tree containment or a sandbox.