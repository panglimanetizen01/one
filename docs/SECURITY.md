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

The current implementation candidate is tracked in [Issue #104](https://github.com/panglimanetizen01/one/issues/104) and its [design record](security/issue-104-filesystem-root-confinement-design.md). PR #113 remains unmerged. Linux supports direct-child atomic writes and a guarded detached-mount nested-write path that fails closed when required kernel primitives are unavailable. Windows uses a native handle-relative backend; Android and macOS have direct-child native candidates, while nested Android/macOS paths remain fail-closed. None of this promotes Issue #104 to ACCEPTED or makes the capability part of `main`.

Do not advertise or rely on this capability as a released project sandbox until Issue #104 is ACCEPTED and the release contract is satisfied.


## CLI authority-root selection

The CLI must derive the filesystem authority root exclusively from the host process current working directory (`std::env::current_dir()`), independently of both the source-file location and the requested target. It opens and retains that root authority before filesystem adapter invocation. The source path and `--target` never select or widen the root. Absolute and relative targets are independently resolved against the retained root; targets outside it are rejected for both read and write operations.

Regression tests cover outside-root read/write attempts and verify that outside data is neither disclosed nor modified. This remains a draft PR #113 candidate pending exact-SHA verification.


## Process execution containment boundary — Issue #105 — 20 November 2026

The current process implementation candidate enforces a 10-second direct-child deadline, 1 MiB stdout and stderr limits independently, and a 2 MiB aggregate capture ceiling. Linux/Android create a dedicated process group and install an inherited seccomp filter before exec to deny `setsid`, `setpgid`, namespace creation, and related escape paths; cleanup targets the retained process group on normal completion, timeout, and output overflow. Windows uses a Job Object assigned during process creation and terminates remaining job members during cleanup. macOS process execution remains fail-closed. The Linux/Android containment implementation is source-present but NOT BUILT/TESTED on the current candidate; do not describe it as a verified process-tree guarantee or sandbox.

Windows uses a Job Object assigned during process creation and terminates remaining job members during cleanup. The cross-platform guarantee is therefore not identical.

Historical evidence boundary: SEC-05 run #44566187162 passed on its pinned source `722d662...` for the bounded timeout/output contract, but does not prove later seccomp containment. The current code-bearing candidate `cd95e5ec2234a2909ef79bf8b25fe65296fab5ed` adds explicit aggregate capture enforcement and is NOT BUILT/TESTED. Issue #81 and Issue #105 remain OPEN; no current-candidate process-tree PASS is claimed.


## Issue #105 — containment feasibility boundary — 2026-11-20

The default Ubuntu 24.04 GitHub-hosted runner has cgroup v2 and `cgroup.kill`, but the job's current service cgroup is not delegated. A temporary child cgroup could be created and its control files chowned, but moving a child process into `cgroup.procs` failed. Unprivileged PID namespace creation also failed with `Operation not permitted`. See [Issue #105 current status](verification/ISSUE_105_CURRENT_STATUS_2026-11-20.md) for exact run IDs and results.

Do not implement or claim a cgroup/PID-namespace containment backend based only on kernel feature presence. Integration verification requires a host/self-hosted runner that explicitly delegates source-cgroup membership and a writable subtree, or permits user/PID namespaces. Until then, Unix process-group cleanup remains best-effort and is not process-tree containment or a sandbox.
## Host-owned filesystem root implementation checkpoint — 2026-11-20

PR #113 implementation now derives filesystem authority exclusively from the host process working directory. Linux root acquisition uses `O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC` and checks device/inode identity against the retained handle. Windows preserves the final root component, opens with native no-reparse flags, and checks volume/file identity. CLI positive/negative root-selection fixtures plus Linux symlink-root and Windows junction-root regressions have been added.

Latest source/test candidate: `ea376531e99f37b874d3d3a8edd4e8b4d656095f`; current PR #113 HEAD at last readback: `6f7d86842c80a81135227d8d62404e848ab5d8c3`. These changes have NOT been built or tested; no prior exact-SHA evidence transfers. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.


## Implementation-first source refresh — 2026-11-21

The latest code-bearing Issue #104/#105 candidate is `72eae103bc3304b0961c66a0cf61f716ddfa9ac4`. The CLI trusted root is the host process current working directory only; source-file path, manifest discovery, and requested target do not select or widen it. The Linux nested writer uses a detached mount and fails closed if `open_tree`, `mount_setattr(MOUNT_ATTR_NOSYMFOLLOW)`, or `openat2` cannot provide the required boundary. Android/macOS nested paths remain fail-closed. Source presence is not acceptance: the candidate has not been built or tested, and Issue #104 remains OPEN / NOT ACCEPTED.


## Linux special-file read hardening — 2026-11-21

Code change: `50923b545f2c804b24aa773ea5bb60129d1e0ff2` updates the Linux `openat2` read flags to include `O_NONBLOCK` before the opened object is checked with `fstat` for regular-file type. This prevents opening a FIFO with blocking semantics while retaining the existing handle-relative `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV` policy. A Linux regression `authority_rejects_fifo_without_blocking` creates a FIFO with no writer and requires `FilesystemError::NotRegularFile`. The regression has NOT RUN; this is source/test presence only, not execution evidence or Issue #104 acceptance.


## Filesystem read-allocation and resource-limit hardening — 2026-11-21

Code commit: `8792f7a1f707f444d00026d916fa8c18b7f3e7f1`.

- Replaced the unchecked `u64 as usize` conversion used to preallocate filesystem read buffers with `usize::try_from` and `Vec::try_reserve_exact`. Oversized capacities now fail with a defined filesystem error instead of truncating on 32-bit hosts or panicking on reservation failure.
- Added focused Linux regression `authority_enforces_read_and_write_limits_without_partial_replacement`; it verifies an oversized read is rejected, an oversized write is rejected, and the existing target bytes remain unchanged.
- This regression is NOT RUN. No build, test, CI, verifier, or platform matrix is claimed. Issue #104 remains OPEN / NOT ACCEPTED.
