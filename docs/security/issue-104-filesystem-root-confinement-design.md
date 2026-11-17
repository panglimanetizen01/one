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
3. Verify each opened directory against the trusted root's mount identity as well as device identity. On Linux, use statx with STATX_MNT_ID (or a stronger available mount identifier); st_dev alone is insufficient because bind mounts can share a device number. On other Unix targets, use an equivalent mount-identity primitive or fail closed where mount-boundary enforcement cannot be proven.
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

## 11. Host-owned API and CLI integration contract

Use one shared authority object so read and write adapters cannot drift to different roots or limits:

- `FilesystemLimits` is immutable and carries `max_read_bytes` and `max_write_bytes`; construction rejects zero or unsupported platform values and uses checked conversions before allocation.
- `ProjectFilesystem::open(root: &Path, limits: FilesystemLimits)` is the only operation that establishes root authority. It opens the root itself without following a final symlink/reparse point, verifies it is a directory, retains its handle, and records its file identity and mount/volume identity.
- `FileReadAdapter::new(filesystem: Arc<ProjectFilesystem>)` and `FileWriteAdapter::new(filesystem: Arc<ProjectFilesystem>)` share that same immutable authority. There is no parameterless production constructor that silently adopts the process working directory.
- Add an explicit interpreter host constructor, conceptually `Interpreter::with_orchestration_policy_and_filesystem(policy, filesystem)`. Existing non-filesystem interpreter construction remains available. Existing `with_orchestration_policy(policy)` must not infer a filesystem root; filesystem adapter invocation without an injected authority fails closed.
- The CLI is the trusted host boundary. For a filesystem capability invocation it obtains `std::env::current_dir()` independently of the user-supplied `--target`, opens that directory as the project root, and injects the authority. A target is never used to select or widen the root.
- CLI callers that need to access a file outside the current project directory must explicitly change the trusted host root through a separately designed host configuration; the language source and `--target` cannot do so.
- Tests create a temporary project directory, explicitly open it as the trusted root, and place all positive fixtures inside it. They must not make the target's parent directory the root as a shortcut.
- Preserve the existing capability-policy evaluation order and inherited deny/approval semantics. Root-handle acquisition occurs at host setup; per-operation policy authorization still precedes adapter invocation and filesystem access.

No root may be derived from the target, source text, ambient environment override, or a path supplied by the program. A root configuration failure is a host setup error; an operation with no filesystem authority is denied/fails closed without touching the filesystem.

## 12. Historical PR #80 implementation review — reuse boundaries

PR #80 at exact head `d75b2846465ba53281198693750f2e6ba39e2e33` was inspected as reference material, not accepted as an implementation to copy.

Reusable Unix techniques, subject to current-contract adaptation and tests:
- descriptor-relative parent traversal with `openat`, `O_DIRECTORY`, `O_NOFOLLOW`, `O_CLOEXEC`, and `O_NONBLOCK`;
- leaf-handle validation for regular-file type and link count;
- Linux `statx` mount identity plus device identity;
- race-test structure for parent-directory symlink substitution and side-effect assertions.

Rejected as-is:
- root authority is recalculated from `std::env::current_dir()` for each operation rather than injected and retained by the host;
- the Unix root walk reopens a path chain instead of deriving operations from one retained trusted root handle;
- non-Linux Unix mount identity falls back to device number only, which cannot prove bind-mount separation;
- the write path is not a same-directory exclusive-temp + atomic rename transaction;
- Windows opens by path, then uses canonicalization/final-path comparison; this is check/open and path-based, not a handle-relative traversal guarantee;
- the API has no shared immutable root authority or host-supplied per-operation byte limits.

The current implementation must be a fresh, narrowly scoped design that preserves only the reviewed Unix primitives and closes the listed gaps. Do not copy PR #80's adapter, path resolution, or Windows implementation wholesale.

## 13. Independent verifier execution diagnosis — 2026-11-18

Inspected GitHub Actions run `43998206030` in `panglimanetizen01/one-windows-verification`:

- Unix job `133755415781` failed before running any filtered test because Cargo reported it could not create `source/Cargo.lock` while `--locked` was set.
- Windows job `133755416047` failed for the same reason before the first named test.
- The verifier's `CANONICAL_COMMIT.txt` at branch `verification` pointed to `e50748a1b25922d9090eba1f5fb90548d7c7d003`, and GitHub file readback confirms that exact source SHA has no `Cargo.lock`.
- The current PR #108 base candidate `c4bb4b690e698a83e4ddaf279596479d3dc282ca` does contain `Cargo.lock` (content blob `3fd2cb2be6aa30afa54602c733b2ab192cc91d2b`). PR #110 inherits it.

Classification: the historical E4 run is a verifier-input/lockfile precondition failure, not a filesystem test failure and not a security PASS. The current source candidate still lacks the required Issue #104 test implementations, so do not simply change the verifier canonical SHA to the current PR #108/PR #110 candidate: the filters would then execute zero tests. Update the verifier's canonical candidate only after a source implementation has landed on a candidate SHA containing all required test names and the lockfile. Then run the workflow and inspect both job logs.

No verifier branch file was changed as part of this diagnosis.
## 14. Current-source integration map — exact PR #108 base candidate

Static readback was performed on `c4bb4b690e698a83e4ddaf279596479d3dc282ca`. This is a source integration map, not implementation or test evidence.

### Production call sites

- `crates/one-core/src/adapter.rs`: `FileReadAdapter` and `FileWriteAdapter` each currently hold only `AdapterMetadata`; their public `new()` constructors take no authority. Read calls `std::fs::read(path)`; write calls `std::fs::write(path, content)`. Read rejects all adapter arguments; write accepts exactly one `AdapterArgument::Text`. Both extract the path from `ExecutionScope::Target`. These are the primary unsafe operations to replace.
- `crates/one-core/src/interpreter.rs`: `Interpreter::with_orchestration_policy` is the single registration point and currently registers both filesystem adapters with parameterless constructors. Add a dedicated constructor taking the shared filesystem authority and register both adapters with clones of that same authority. Preserve policy-first request evaluation in `Orchestrator`; do not create a second policy path.
- `crates/one-cli/src/main.rs`: the `run` command parses `--target` into `ExecutionScope::Target` and selects the interpreter constructor in one dispatch expression. It currently does not obtain a trusted current-directory handle for this operation. Capture the host working directory independently of `--target` and inject it only for filesystem capability dispatch. The target string must never choose the root. Non-filesystem capabilities retain their existing construction path.
- `crates/one-core/src/lib.rs`: adapter types are publicly re-exported. Export the new filesystem authority and limits deliberately; do not expose internal platform handle types.
- `crates/one-core/Cargo.toml`: `libc = "0.2"` is already present for Unix; `windows-sys = "0.61"` is already present with Foundation, Security, Storage/FileSystem, and process/job features. No new dependency is justified until a specific API gap is demonstrated and checked against Cargo.lock.

### Existing tests that need migration or replacement

- `crates/one-core/src/adapter.rs` contains unit tests using `FileReadAdapter::new()` and `FileWriteAdapter::new()`, including a write test using a fixed temp-directory filename. Convert these to explicitly injected temporary roots and unique fixtures; never infer root from the target.
- `crates/one-core/tests/filesystem_vertical_slice.rs` directly constructs adapters and tests policy/scope behavior. Preserve those policy assertions while injecting a trusted root. Its current-directory target fixtures are not a confinement test.
- `crates/one-core/tests/filesystem_write_runtime.rs` must be reviewed and migrated to explicit root injection; retain the existing runtime semantics.
- Existing direct CLI file operations for build/package/project management are separate host operations. Do not claim this adapter remediation automatically confines all CLI project-management file operations.

### Safe implementation order once a test runner is available

1. Add `FilesystemLimits` and immutable `ProjectFilesystem` ownership/constructors; establish root-handle lifetime and fail-closed behavior first.
2. Add platform-specific handle-relative resolution primitives with no path-based fallback. Validate root, every parent, leaf type, link count, mount/volume identity, and supported path syntax.
3. Change adapters to require `Arc<ProjectFilesystem>`; enforce read/write byte limits and use atomic same-directory temporary writes.
4. Add the dedicated interpreter constructor; preserve existing no-authority construction so filesystem adapter dispatch fails closed, while non-filesystem adapters continue working.
5. Inject host-owned cwd from CLI independently of `--target`; preserve all current language and CLI vocabulary.
6. Migrate tests, add adversarial side-effect/race tests with exact independent verifier names, then run compile + full relevant test suites on Unix and Windows at one exact SHA.

### Execution gate

This session has GitHub repository read/write access but no authenticated CircleCI tool, no GitHub Actions workflow-dispatch action, and no working local clone/network route to compile the Rust workspace. Therefore source implementation cannot currently be validated. Do not write unsafe platform code into the branch without a working compile/test path. The static source map above is complete enough to start implementation immediately when a runner becomes available; it is not a security fix.
