# ONE E4 Threat Review — 2026-10-26

## Scope and review target

This is a scoped threat review of the filesystem adapters, capability-denial path, SQLite resource limits, and bounded subprocess execution used by ONE v0.1. The code reviewed for this checkpoint is candidate source SHA `2a7670c5166efee54426196f28a8152cbfe758b3`. This document is a review artifact, not a claim that ONE is a complete sandbox. The final candidate SHA must be revalidated after this document is committed.

## Assets and security objectives

- Prevent filesystem read/write requests from escaping the process working-directory project root.
- Reject parent traversal, symlink/reparse traversal, non-regular files, and cross-mount targets where the platform-specific implementation can identify the boundary.
- Avoid destructive side effects before validating the opened target and its mount identity.
- Deny capability-prohibited operations before invoking an adapter or causing external side effects.
- Bound active resource operations, SQLite query work, subprocess wall-clock duration, captured stdout/stderr, and aggregate interpreter output.
- Ensure subprocess descendants do not outlive timeout/output-limit cleanup on supported Unix and Windows implementations.
- Preserve diagnosable failure behavior without exposing secret values or silently accepting partial verification.

## Threat model

Attacker-controlled inputs include requested filesystem paths, symlink/reparse-point layouts, concurrent directory renames, SQLite queries, process arguments, child-process output, and requests lacking required capabilities. The attacker may race filesystem path resolution or attempt to consume resources with long-running processes, inherited pipes, large output, or repeated operations.

The review assumes the OS kernel and runner are functioning correctly and that the process's current working directory is the intended project root. It does not assume that string canonicalization alone prevents TOCTOU races. It does not claim protection against a compromised kernel, privileged host administrator, malicious compiler/toolchain, or all resource consumption inside the process heap.

## Controls observed in source

- Project filesystem requests are resolved against the canonical current working directory; absolute targets outside that root and parent-directory traversal are rejected.
- Unix filesystem access walks parent components using descriptor-relative `openat` with `O_NOFOLLOW`; the final component is opened without following symlinks. Linux mount identity checks reject a crossed mount boundary before a write truncates the target.
- Windows file access checks the final opened handle/path, rejects reparse points, validates the parent before creation, and checks the target remains under the project root.
- Unsupported non-Unix/non-Windows filesystem adapters fail closed.
- Capability denial is checked before adapter invocation.
- A shared resource budget caps active operations and SQLite queries; SQLite query limits and the interpreter output cap are enforced.
- Subprocess execution bounds wall-clock time and stdout/stderr/aggregate capture. Unix process-group cleanup and Windows Job Object association at process creation are used to terminate descendants on timeout/output-limit failure.

## Exact-SHA test evidence to retain

The independent lab currently has scoped runs on the candidate pin `2a7670c5166efee54426196f28a8152cbfe758b3`:

- [E4 adversarial filesystem/resource-bound run 40321345484](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40321345484) — Unix no-follow/mount-boundary/race tests and Windows reparse/handle/absolute-path regressions completed successfully.
- [E4 subprocess resource-guard run 40321345794](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40321345794) — scoped resource-guard audit passed.
- [Cross-platform run 40321345494](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40321345494) — scoped Linux/Windows verification passed.
- [Fixed-weight gate 40324100070](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40324100070) — **NOT PASSED** overall. A passing E4 workflow is not a substitute for the full E4 row's acceptance.

These run links are historical evidence for the source SHA explicitly checked out by their jobs. If any code-bearing candidate change is made, the relevant tests must be rerun against the new frozen SHA before acceptance.

## Residual risks and explicit limitations

1. Linux `STATX_MNT_ID` can distinguish bind mounts that share a device ID, but this depends on kernel/filesystem support and the implementation's successful identity query.
2. On other Unix platforms, `st_dev` is only a best-effort mount identity and may not distinguish bind-mount-like boundaries.
3. Windows handle/path validation and reparse-point rejection reduce escape risk but are not a proof of every possible filesystem-provider race or malicious kernel behavior.
4. Resource quotas cover the named operations and captured output. They do not establish a universal cap on all transient allocations, caller-retained buffers, compiler/runtime heap use, or every external tool's resource consumption.
5. Subprocess containment is platform-specific. The tests establish only the adversarial cases they execute; they do not prove all possible descendant-process, inherited-handle, or kernel scheduling races.
6. This review does not claim a complete sandbox, OS-level isolation, privilege separation, or resistance to a compromised host.
7. Android API 35 AArch64 runtime acceptance and the protected package-provenance release gate are separate acceptance requirements and remain open until their own exact-SHA evidence passes.

## Disposition

The reviewed controls and listed regression coverage support recording that a scoped threat review now exists. They do **not** by themselves grant E4 verified points or close the overall milestone. Keep E4 verified at 0/6 until the fixed-weight gate's full E4 acceptance criteria and final-SHA evidence are accepted. Record any new findings or code changes in a subsequent dated review rather than rewriting historical run results.


## Historical candidate reconciliation addendum — 2026-10-28 (superseded by the exact-SHA addendum below)

This addendum supersedes the historical target SHA for purposes of identifying the current candidate, but it does not rewrite or re-date the historical runs above. Historical candidate SHA at the time of this addendum: `1a84d522a2953044c250ebe43c22989d6ec07dfc`. The links and disposition in this section are retained as historical evidence only.

### Current candidate-scoped evidence

The independent verifier checked out that exact source SHA for these scoped workflows:

- [E4 adversarial filesystem/resource regression, run 40634065806](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40634065806) — passed the workflow's focused adversarial regression scope.
- [SEC-05 subprocess timeout/output regression, run 40634065919](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40634065919) — passed its focused timeout/output regression scope.
- [Subprocess resource guard, run 40634065925](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40634065925) — passed its focused resource-guard scope.
- [Cross-platform verification, run 40634065845](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40634065845) — passed the configured Linux/Windows verification matrix.
- [Linux distribution verification, run 40634065997](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40634065997) — passed the configured Linux distribution matrix.

These results supersede older run links only for the same explicitly scoped tests; they do not constitute a complete E4 acceptance suite. The gate run [40638256965](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40638256965) still rejects the milestone, and E4 remains 0/6 verified.

### Remaining E4 acceptance work

1. Re-run the required security matrix after the final candidate SHA is frozen, so every accepted run checks out the same SHA.
2. Reconcile each threat claim against the implementation and test assertions, including path traversal, symlink/reparse handling, mount-boundary detection, TOCTOU races, capability denial before side effects, SQLite/query limits, shared operation quotas, subprocess descendants, timeout/output bounds, and recovery after quota release.
3. Explicitly record platform-specific gaps and the limits of test coverage; do not describe these controls as a complete sandbox or universal heap limit.
4. Have the fixed-weight acceptance gate consume the current-SHA evidence and review artifact before any E4 points are credited.

**Disposition:** the threat review is updated with current-candidate evidence pointers, but E4 remains open pending final-SHA revalidation and full row acceptance. No release or merge is authorized by this review.


## Exact-SHA E4 reconciliation — 2026-10-28

**Current source candidate at review time:** `6d492006f9fa873fbd7d8ad61fa5833df707b1f6`.

The independent verifier checked out this exact source SHA for the current scoped E4 evidence:

- [E4 adversarial filesystem/resource regression, run 40664235672](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40664235672) — PASS for its executed adversarial filesystem and resource-bound tests.
- [SEC-05 subprocess timeout/output regression, run 40664235569](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40664235569) — PASS for its bounded timeout/output scope.
- [Subprocess resource guard, run 40664235630](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40664235630) — PASS for its scoped resource-guard assertions.
- [Cross-platform Linux/Windows verification, run 40664235567](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40664235567) — PASS for its configured matrix.

These exact-SHA results replace the older candidate as current scoped evidence; they do not retroactively change historical runs. The threat model still has explicit limits: non-Linux Unix mount identity is best-effort, Windows path/reparse defenses are not a proof against every filesystem-provider race, and named operation/output quotas are not a universal heap limit. E4 remains **0/6 verified** until the complete E4 row and final frozen-SHA acceptance are satisfied. Canonical source-repository CI continues to fail before executable steps and is classified as infrastructure-blocked, not PASS. No merge or production release is authorized.


## Scoped E4 acceptance record — candidate `28d1384588662cc93f405dc9dd5f3012183d249f`

The E4 acceptance review is complete for the current declared support boundary, with residuals explicitly retained rather than overstated.

**Evidence on the exact candidate SHA `28d1384588662cc93f405dc9dd5f3012183d249f`:**
- [E4 adversarial filesystem/resource-bound regression 40669813282](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40669813282) — PASS on Linux and Windows scoped adversarial tests.
- [SEC-05 subprocess timeout/output regression 40669813337](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40669813337) — PASS; Linux and Windows suites completed, including timeout/output bounds and process containment checks.
- [Subprocess resource guard 40669813307](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40669813307) — PASS for static guard and resource-enforcement checks.
- [Cross-platform Linux/Windows verification 40669813367](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40669813367) — PASS, including workspace regression suites on both OSes.

**Acceptance boundary:** Linux and Windows are first-class targets; Android/Linux is the declared integration target. The independent exact-SHA matrix also separately passed Android API 35 `arm64-v8a` ELF runtime execution on candidate SHA `28d1384588662cc93f405dc9dd5f3012183d249f` ([run 40669813305](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40669813305), exit code 0, output `ONE_NATIVE_BACKEND_OK`). macOS and additional Unix-like systems remain planned expansion, not current first-class acceptance targets.

**Residual risk statements retained:** no claim of a universal sandbox, protection against a compromised kernel/administrator, universal heap cap, complete Windows filesystem-provider race proof, or strict bind-mount identity on non-Linux Unix. The tested current target controls cover root anchoring, Linux mount identity, Unix no-follow behavior, Windows handle/reparse confinement, capability denial before side effects, bounded subprocess execution/output, shared active-operation/SQLite budgets, output cap, and recovery assertions.

**Disposition:** E4 is accepted provisionally as 6/6 for this candidate scope. This acceptance record itself is a source-document change, so the candidate SHA advances; the independent verifier must pin the resulting SHA and rerun the relevant matrix. Overall release gate remains NOT PASSED while E1, E2, E6 and final-freeze conditions remain open. No merge or production release is authorized.


## Hard-link confinement correction — 2026-11-01

A follow-up source review identified a filesystem confinement case not covered by the earlier symlink/reparse and mount-boundary checks: a regular file hard-linked into the project root can still share an inode/file identity with a name outside that root. Descriptor-relative no-follow traversal and same-mount checks alone do not reject that alias.

The candidate implementation now rejects file targets whose opened-handle hard-link count is not exactly one:
- Unix: inspect the opened file descriptor metadata before reading or truncating.
- Windows: query `BY_HANDLE_FILE_INFORMATION.nNumberOfLinks` through the stable Win32 `GetFileInformationByHandle` API; fail closed if the query fails.
- Regression tests cover both read and write attempts through an in-project hard link to a sibling/outside file and assert the outside contents remain unchanged.

This is a source correction and test addition, **not yet verification evidence**. The new exact candidate SHA must pass the independent Linux/Windows matrix, including the hard-link regressions, before this control is marked verified. The restriction is intentionally conservative: project filesystem read/write adapters reject files with multiple hard links rather than attempting to infer which alias is authoritative.


## CLI artifact-output confinement correction — 2026-11-01

A second path review found that the language-level filesystem adapters were not the only write surface. The CLI's build/package commands previously used ordinary `create_dir_all` and `fs::write` for `target/*.oneir`, `target/*.onepkg`, and module-lock files. A project-controlled `target` symlink/junction or pre-existing hard-linked artifact could therefore redirect or alias those writes outside the intended project tree.

The candidate now routes CLI build, package, and module-lock output through the shared root-confined writer:
- create the `target` directory only as a direct project child; an existing symlink/reparse point is rejected by descriptor/handle-based parent traversal;
- open final artifacts without following symlinks/reparse points;
- reject final files with more than one hard link before truncation;
- reject unsafe artifact filename components before constructing output paths.

Regression tests cover Unix symlinked `target`, Windows target junctions, hard-linked package artifact targets, and build output through a symlinked `target`. These are code and test additions only; they are **not verification evidence** until the new exact candidate SHA completes Linux/Windows tests, Clippy, and the dedicated E4 adversarial regression workflow.


## Human-core imported-module hard-link correction — 2026-11-01

The same hard-link alias applies to build-time human-core module loading: descriptor-relative no-follow traversal rejects symlinks, but an in-root hard link can still reference an inode with a name outside the project. The Unix and Windows module readers now reuse the opened-handle single-link check before reading source bytes. A regression creates an outside source file, hard-links it into the project, and requires the module reader to reject it. This addition is not verification evidence until the exact candidate SHA passes the compiler, Linux/Windows, and E4 regression suites.


## Human-core root-source symlink correction — 2026-11-01

The human-core compiler previously canonicalized the root source file before opening it, which erased evidence that the caller-supplied final component was a symlink. The compiler now canonicalizes only the selected parent directory and opens the original filename through the no-follow module reader. A regression requires a symlinked root source file to fail with the bounded HV1010 diagnostic. This code/test change is not verification evidence until the exact candidate SHA passes the Linux/Windows compiler and full workspace suites.
