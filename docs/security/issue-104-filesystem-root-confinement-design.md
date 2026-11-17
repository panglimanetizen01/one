# Issue #104 — Filesystem Root-Confinement Implementation Design

**Status:** Design checkpoint only — implementation and security acceptance NOT VERIFIED  
**Parent source SHA:** c4bb4b690e698a83e4ddaf279596479d3dc282ca  
**Branch:** security/issue-104-root-handle-design-20261118  
**Issue:** https://github.com/panglimanetizen01/one/issues/104

## 1. Security objective

A filesystem adapter may access only a regular file reached beneath an explicitly configured, trusted project root. The authority comes from the runtime/project configuration and a retained root directory handle, never from the requested path. Existing capability-policy evaluation remains the first gate; this adapter boundary is additional defense in depth.

The implementation must preserve currently supported relative paths and absolute paths that identify a location inside the configured root. It must reject traversal and any path resolution that cannot be proven to remain beneath the retained root handle.

## 2. Proposed API boundary

Introduce an immutable, cloneable project-filesystem authority created by a trusted host boundary, conceptually:

- ProjectFilesystem::open(root_config, limits) opens and validates the configured root once.
- The object retains the platform root handle and root identity (device/volume identity).
- FileReadAdapter and FileWriteAdapter receive this object explicitly; they must not construct authority from ExecutionScope::Target.
- Interpreter construction gains an explicit filesystem-root configuration path. Existing constructors must not silently infer the root from the requested path. If no trusted root is configured, filesystem adapter invocation fails closed.
- Tests can inject a temporary trusted root through the same public host configuration boundary.

The exact public type and constructor names are provisional until API compatibility and call-site inventory are complete.

## 3. Path normalization and compatibility

- Keep the original capability request and scope unchanged for policy evaluation.
- Relative targets resolve from the retained root.
- Supported absolute in-root targets are mapped to a root-relative component sequence only after proving that the lexical target is beneath the configured root; actual access still proceeds relative to retained handles.
- Reject parent components, empty components where ambiguous, device/UNC namespaces not explicitly supported, and paths outside the configured root.
- Do not use string-prefix containment as the security decision.
- Do not follow symbolic links, junctions, or other reparse points in any traversed component.
- Do not cross mount/volume boundaries.
- Restrict the leaf to a regular file. Reject hard-linked files under the initial strict policy unless a later reviewed contract provides a safe, testable rule.

Lexical normalization is only input validation. It is not the security boundary; handle-relative traversal is.

## 4. Unix implementation requirements

Use directory descriptors and descriptor-relative operations:

1. Open the trusted root as a directory with close-on-exec and no-follow semantics; retain its device/inode identity.
2. Walk each parent component with openat using O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC.
3. Verify each opened directory is on the same device as the trusted root; fail closed on mount crossing.
4. Open the leaf relative to the retained parent descriptor with O_NOFOLLOW | O_CLOEXEC, then validate with fstat that it is a regular file and satisfies the link-count policy.
5. Reads use a bounded loop and stop before exceeding the configured maximum.
6. Writes use a bounded input and a same-directory exclusive temporary file, flush/sync as required by the durability contract, then atomically rename relative to the parent descriptor. Cleanup the temporary entry on every failure path.
7. Never fall back to std::fs::read, std::fs::write, canonicalize-then-open, or check-then-open path operations.

Race tests must replace path components and leaf entries concurrently while the operation runs; assertions must verify no outside file was read, created, or modified.

## 5. Windows implementation requirements

The Windows path must provide equivalent handle-relative guarantees, not merely canonicalize/check/reopen:

1. Open and retain the configured root directory handle with backup-semantics and reparse-point inspection; record volume/file identity.
2. Traverse components relative to the retained parent handle using a supported native handle-relative open primitive (for example, NtCreateFile with OBJECT_ATTRIBUTES.RootDirectory), with reparse-point-safe flags and explicit disposition/share semantics.
3. Inspect each opened handle for reparse-point attributes and verify volume identity to reject junctions/reparse traversal and volume crossing.
4. Validate the leaf handle as a regular disk file and enforce the hard-link policy using handle metadata.
5. Perform bounded handle-based reads and same-directory temporary-file + handle-relative rename writes.
6. If a race-resistant native primitive cannot be implemented and independently validated, the Windows adapter must fail closed and the feature must remain experimental; a path-based fallback is prohibited.

Do not claim Windows parity based only on path canonicalization, GetFinalPathNameByHandle, or metadata checks followed by a second path open.

## 6. Resource and atomicity contract

- Define explicit maximum read bytes and maximum write bytes in trusted runtime configuration.
- Reject over-limit operations before unbounded allocation or file mutation.
- Use bounded buffers and checked arithmetic.
- Write through an exclusive temporary file in the already-open parent directory; never create the temporary file through an ambient absolute path.
- On failure, remove only the temporary entry created by this operation; do not delete or truncate an unrelated target.
- Specify flush/sync behavior and whether atomic rename guarantees visibility only or durability as well.
- Preserve existing adapter error mapping without leaking host paths or secrets.

## 7. Required adversarial test matrix

For both Unix and Windows, with real nonzero verifier matches:

- positive relative in-root read and write;
- positive supported absolute in-root read and write;
- reject parent traversal and sibling-prefix confusion;
- reject symlink/reparse-point parent and leaf;
- reject mount/volume crossing;
- reject hard-link leaf under the strict initial policy;
- reject directory/device/special-file leaf;
- reject oversized reads and writes without partial outside effects;
- race parent replacement, symlink substitution, and leaf replacement during repeated operations;
- verify atomic write behavior and temporary-file cleanup after injected failures;
- verify an outside sentinel file remains byte-identical and no outside file is created;
- verify policy denial and approval precedence remain unchanged and occur before adapter invocation.

The test harness must assert side effects, not merely error codes or metadata snapshots. Test names must match the independent verifier's exact filters, and the verifier must report a nonzero matched-test count.

## 8. Integration constraints

- Keep the orchestrator's policy-first ordering and PR #101 inherited deny/approval precedence unchanged.
- Do not broaden capabilities or derive root authority from user-controlled source values, environment variables, or requested targets.
- Update adapter tests to inject a trusted temporary root rather than relying on repository working-directory side effects.
- Update interpreter construction, CLI/runtime host setup, normative specs, threat model, developer docs, user feature status, evidence matrix, verification record, and DoD only when supported by implementation evidence.
- Keep filesystem effects documented as experimental until Unix and Windows adversarial tests pass on the same exact source SHA and independent verification is reviewed.

## 9. Explicit non-acceptance

This design document is not an implementation, a test result, or a security certification. No source code has been changed by this checkpoint. Issue #104 remains OPEN. Do not close it, promote DoD/E4 evidence, or claim the filesystem sandbox is secure based on this document.

## 10. Independent verifier's current exact test filters

The existing independent verifier workflow at panglimanetizen01/one-windows-verification, branch verification, currently requires these nonzero test matches on the candidate source:

Unix:
- 2 passing tests matching `descriptor_relative_read_rejects_`
- 1 passing test matching `descriptor_relative_write_rejects_`
- 1 passing test matching `descriptor_relative_walk_rejects_`
- exact test `adapter::filesystem_nofollow_tests::descriptor_relative_read_race_never_follows_swapped_parent_symlink`
- exact test `adapter::filesystem_nofollow_tests::final_component_mount_boundary_denial_does_not_truncate_file`

Windows:
- exact test `filesystem_reader_rejects_directory_junction_escape`
- exact test `filesystem_adapter_reads_absolute_target_inside_project_root`
- exact test `filesystem_adapters_confine_absolute_targets_to_project_root_without_side_effects`
- exact test `filesystem_write_adapter_writes_text_target`
- exact test `filesystem_write_adapter_rejects_oversized_content`
- exact test `filesystem_adapters_reject_parent_directory_traversal`

The verifier checks that the expected tests actually execute and pass; filtered-only output is explicitly rejected. The implementation must either preserve these exact test names/counts or update the independent verifier filters in a separately reviewed change. Do not weaken nonzero-match assertions. This inventory is from the currently checked-in verifier workflow and must be refreshed before the final acceptance run.
