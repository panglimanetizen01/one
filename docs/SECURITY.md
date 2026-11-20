# One Security Contract

## Security position

One is general-purpose and can be used for legitimate security engineering, defensive automation, vulnerability research, authorized penetration testing, system hardening, forensic tooling, and network analysis.

Security capability does not require a separate language. It requires explicit runtime governance.

## Authority model

```text
human-readable source
 -> formal semantics
 -> typed capability request
 -> scope
 -> policy
 -> adapter
 -> execution
 -> audit
```

Source wording alone never grants privilege.

## Required controls for sensitive operations

Where applicable, a capability contract must define:

- semantic identity;
- runtime capability;
- effect class;
- target/resource scope;
- privilege requirements;
- platform/backend availability;
- adapter identity;
- input/output schema;
- audit requirements;
- determinism/side-effect expectations;
- verification boundary.

## No privilege escalation through syntax

The following must not bypass security controls:

- localized wording;
- aliases;
- macros;
- generated source;
- libraries;
- adapters;
- external tool substitution;
- AI-generated code.

## Security tooling interoperability

One may interoperate with operating-system APIs, network libraries, native libraries, external security tools, and analysis frameworks through explicit interfaces. Such integration must preserve scope and authorization boundaries.

## Auditability

Security-sensitive execution should record observed runtime facts such as capability, target/resource, adapter, policy decision, execution result, and relevant diagnostics without inventing unverifiable facts.

## Safe project principle

The project should not artificially cripple the language merely because it can express security tooling. Instead, it should make authorization, scope, and execution boundaries explicit and testable.


## Project-root filesystem boundary — Issue #104

Filesystem effects are not a security sandbox merely because they pass through a capability adapter. Before any release can claim project-root confinement, the host must inject an immutable trusted root authority, all accepted operations must remain handle-relative, and symlink/reparse, mount, hard-link, traversal, race, side-effect, and size-limit tests must pass on the exact source revision.

The current implementation candidate is tracked in [Issue #104](https://github.com/panglimanetizen01/one/issues/104) and its [design record](security/issue-104-filesystem-root-confinement-design.md). PR #113 remains unmerged. Linux candidate writes are limited to direct children of the retained root; nested writes fail closed. Windows native atomic replacement is under exact-SHA verification. Other platform backends remain unsupported.

Do not advertise or rely on this capability as a released project sandbox until Issue #104 is ACCEPTED and the release contract is satisfied.


## CLI authority-root selection

The CLI must derive the filesystem authority root exclusively from the host process current working directory (`std::env::current_dir()`), independently of both the source-file location and the requested target. It opens and retains that root authority before filesystem adapter invocation. The source path and `--target` never select or widen the root. Absolute and relative targets are independently resolved against the retained root; targets outside it are rejected for both read and write operations.

Regression tests cover outside-root read/write attempts and verify that outside data is neither disclosed nor modified. This remains a draft PR #113 candidate pending exact-SHA verification.


## Process execution containment boundary — Issue #105 — 20 November 2026

The current process adapter enforces a 10-second direct-child deadline and 1 MiB capture limit per output stream. On Unix, stdout/stderr readers are nonblocking and have a 100 ms shutdown grace (20 ms idle grace); this prevents an escaped pipe writer from keeping the caller blocked indefinitely. However, cleanup still kills only the original process group. A descendant that calls `setsid()` or creates a new process group can survive after the invocation returns. Bounded reader shutdown is **not** process-tree containment or a sandbox.

Windows uses a Job Object assigned during process creation and terminates remaining job members during cleanup. The cross-platform guarantee is therefore not identical.

Evidence boundary: SEC-05 run #44566187162 passed on its pinned source `722d662...` for the bounded timeout/output contract, but does not prove escaped-session cleanup. On candidate source `c7f460a4e38be61de635ad5af9f7ff987a941f01`, the Windows SEC-05 job #135776430245 is SUCCESS while the Linux job #135776429279 remains IN_PROGRESS at native cross-compiler installation. Issue #105 remains OPEN; no process-tree containment claim is made.


## Issue #105 — containment feasibility boundary — 2026-11-20

The default Ubuntu 24.04 GitHub-hosted runner has cgroup v2 and `cgroup.kill`, but the job's current service cgroup is not delegated. A temporary child cgroup could be created and its control files chowned, but moving a child process into `cgroup.procs` failed. Unprivileged PID namespace creation also failed with `Operation not permitted`. See [Issue #105 current status](verification/ISSUE_105_CURRENT_STATUS_2026-11-20.md) for exact run IDs and results.

Do not implement or claim a cgroup/PID-namespace containment backend based only on kernel feature presence. Integration verification requires a host/self-hosted runner that explicitly delegates source-cgroup membership and a writable subtree, or permits user/PID namespaces. Until then, Unix process-group cleanup remains best-effort and is not process-tree containment or a sandbox.