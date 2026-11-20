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


## Linux nested-write fallback and verifier-inventory alignment — 2026-11-21

Code commits:
- `0fd434ad46e6b8e93c3b9ed887447baf10d0f697`: classify Linux `EOPNOTSUPP` from required detached-mount primitives as `FilesystemError::UnsupportedPlatform`, preserving fail-closed behavior when the backing filesystem/kernel cannot provide the required operation.
- `858a5da09fe2ea74ebd17b0798f13fac848c7a28`: replace the stale test that required every Linux nested write to fail with `nested_write_uses_native_backend_or_fails_closed_without_side_effects`. It accepts a successful native detached-mount write or an explicit `UnsupportedPlatform` fallback, and checks target integrity and temp-file cleanup.

The independent verifier's expected source test-filter inventory is now structurally matched in `adapter.rs`: 3 `descriptor_relative_read_rejects_`, 5 `descriptor_relative_write_`, and 1 `descriptor_relative_walk_rejects_` tests. This is a source inventory check only; no tests were run and no verifier PASS is claimed. Issue #104 remains OPEN / NOT ACCEPTED.


## 2026-11-21 — Android/macOS filesystem source repair and confinement boundary

Latest code-bearing source: `0f5859426273a61174d6d1b08ed8771a6f4b136a` (`crates/one-core/src/filesystem.rs`).

- Android's duplicate `create_temp` definition has been removed; macOS now has its missing exclusive `create_temp` helper. Static source inventory shows one helper definition in each native module. No compiler was run.
- Android/macOS direct-child operations retain root-handle-relative no-follow reads and same-directory exclusive-temp atomic replacement; Android requires `statx(STATX_MNT_ID)`, while macOS compares `f_fsid` and mount point with `fstatfs`. Regular-file and hard-link checks remain.
- Nested paths on both POSIX backends explicitly return `UnsupportedPlatform`. Ordinary POSIX directory descriptors do not prevent a parent directory from being renamed outside the retained root, so a multi-component openat walk would not satisfy the root-confinement race contract. No such unsafe nested support is enabled.
- Android/macOS source regressions cover direct-child operation, leaf symlink rejection, nested-path fail-closed behavior, root symlink rejection, and hard-link side-effect preservation. These tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED. This source correction is not a build/test/verifier result; no platform support or release approval is promoted.


## 2026-11-21 — bounded HTTP response ingestion and filesystem payload allocation

Latest relevant source commits:
- `d32bb207f8ea6f24740560f374a4f698c687e172` — `FileWriteAdapter` now passes a borrowed text slice to `FilesystemAuthority::write`, so the configured write-byte limit is enforced before an owned output copy is made. The bounded echo copy happens only after a successful write. Added `filesystem_write_adapter_rejects_oversized_content_without_side_effects` (NOT RUN).
- `ce39630fc9abdc9ba7c6accdf8db161a5cb002fa`, `8c219882aa37ee271874f58de172072531d80ddc`, `b636f0bc1e22008fc62dafe46a378cc2fb2b599b`, and `5fe958cad798b3d05c3ceb21f875f1f90074cb35` — HTTP/1.1 response ingestion now enforces a 64 KiB header ceiling and 32 MiB wire-body ceiling while reading, with fallible buffer growth; decoded response bodies remain capped at 16 MiB. The parser rejects duplicate Content-Length, unsupported/duplicate Transfer-Encoding, and simultaneous Content-Length + chunked framing; chunk-size arithmetic is checked and chunk allocation is fallible.
- Added source regressions for oversized headers, ambiguous framing, duplicate lengths, chunk-size overflow, and bounded socket reads. NOT RUN.

These are implementation/resource-boundary changes only. No build, test, CI, verifier, or platform matrix was run. No prior exact-SHA result transfers to this candidate.


## HTTP request serialization resource bounds — 2026-11-21

Latest source commit: `198fbf4ccdbb7e219efbe3e296e98462a385c384` (`crates/one-core/src/network_runtime.rs`).

- HTTP request bodies are capped at 16 MiB before serialization; request line + headers are capped at 64 KiB with checked size arithmetic and fallible preallocation.
- Explicit Content-Length must parse and exactly match the body size; duplicate Content-Length and request Transfer-Encoding are rejected (chunked request serialization is not implemented).
- Added source regressions for oversized request bodies, mismatched Content-Length, and unsupported Transfer-Encoding. NOT RUN.
- No build, test, CI, verifier, or platform matrix was run. This remains an implementation candidate, not verification evidence.


## HTTP request syntax validation — 2026-11-21

Latest source commit: `b0a3379a1fb49feb18dcf61a2521446f199a8e2b` (`crates/one-core/src/network_runtime.rs`).

- Request method and header names must be valid HTTP tokens.
- Authority rejects whitespace/control bytes, user-info separators and fragments; request-target must use origin-form or `*`, and rejects whitespace/control bytes and fragments.
- Header values reject control bytes other than horizontal tab.
- These checks complement the 16 MiB request-body / 64 KiB header caps, Content-Length consistency checks, and response framing limits. Source-only change; no build or tests run.


## HTTP exact framing and trailer validation — 2026-11-21

Latest source commit: `eea3115ba5e70ff83f8ed78a01dc70d95e012ff0` (`crates/one-core/src/network_runtime.rs`).

- Content-Length response bodies must now match the declared length exactly; trailing bytes are rejected.
- Chunked responses bound chunk-size lines, require valid terminating CRLF, validate trailer syntax, reject framing/Host fields in trailers, and reject trailing bytes after the trailer terminator.
- Added regressions for valid trailers, forbidden Content-Length trailers, and extra bytes after Content-Length. NOT RUN.
- No build/test/CI/verifier/platform matrix was run; source-only candidate, no acceptance promoted.


## HTTP header token/control validation regressions — 2026-11-21

Latest source commit: `11bf5172ed060818dbd55d2ecee3cddf3cd99875` (`crates/one-core/src/network_runtime.rs`).

- Response header names are validated as HTTP tokens and values reject control bytes except HTAB. Request header names/values and request-target syntax have equivalent validation.
- Added regressions for invalid response header names/values and invalid request header names/targets. NOT RUN.
- No build/test/CI/verifier/platform matrix was run; this remains source-only implementation work.
