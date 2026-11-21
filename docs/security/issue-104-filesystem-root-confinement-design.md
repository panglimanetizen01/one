# Issue #104 — Filesystem Root-Confinement Implementation Design

**Status:** Candidate implementation PARTIAL; protected post-merge E4 verification CONFIRMED; Issue #104 acceptance remains OPEN / NOT ACCEPTED  
**Latest code-bearing source at 2026-11-21 checkpoint:** `0f5859426273a61174d6d1b08ed8771a6f4b136a` (NOT BUILT/TESTED)  
**Latest CircleCI-confirmed PR #113 HEAD:** `26a34efb6d0908bd02738a7b05090f96051a6dc7` (`ci/circleci: one-linux` SUCCESS; workflow `a7016ead-ed2d-467f-9f1b-4d20c60a966f`, job `6cc9778c-307c-48fc-9111-39c97d288393`). Later documentation-only commits are not credited with this result.  
**Branch:** security/issue-104-root-confinement-20261118  
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


## 15. Follow-up: expanded E4 verifier logs retrieved — 2026-11-18

The expanded GitHub Actions logs for verifier run `43998206030` were subsequently retrieved (the earlier log-access limitation is superseded for this run):

- Unix job `133755415781`, job name `Unix no-follow and SQLite resource bounds`, fails at the first filtered Cargo invocation before any test executes. Exact Cargo error: `cannot create the lock file .../source/Cargo.lock because --locked was passed to prevent this`; process exit code `101`.
- Windows job `133755416047`, job name `Windows reparse-point and handle checks`, fails at the first expected test invocation for the same missing `source/Cargo.lock` under `--locked`; Cargo exit code `101`, PowerShell job exit code `1`.
- The workflow had already checked out the exact verifier branch commit `9cf1bb3071c56497118ad9c9e532abfee3fd60ee` and source pin `e50748a1b25922d9090eba1f5fb90548d7c7d003`. The source checkout does not contain `Cargo.lock`.

Classification: verifier input/precondition failure. No named filesystem test ran, so this run is neither a filesystem test failure nor a security PASS. Do not remove `--locked`, generate an ephemeral lockfile, or weaken nonzero-test assertions to make this run green. Keep the historical pin unchanged until a source candidate both contains the committed lockfile and implements every required exact-match adversarial test. Then advance the pin and run the full Unix/Windows matrix against that exact source SHA.


## 16. Current checkpoint metadata — 2026-11-18

- PR #110 current branch HEAD: `73c705f3581cf26414b4e4d69616fee9886396e4` (documentation-only).
- CircleCI context `ci/circleci: one-linux` for this exact SHA remains PENDING at last refresh; job [8b851d97-57f9-4043-af0e-49c5b357413e](https://app.circleci.com/workflow/7b17fb55-4deb-4256-b820-a9adca6cf606/job/8b851d97-57f9-4043-af0e-49c5b357413e), status timestamp `2026-11-18T04:52:08Z`.
- The most recently reviewed code-bearing source SHA is `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b`; the intervening PR #110 commits are documentation-only. The implementation status remains unchanged: path-based `std::fs::read` / `std::fs::write` are present and Issue #104 acceptance is not met.
- This checkpoint updates source/branch metadata only. It adds no implementation or test evidence.


## 17. Adapter boundary source audit — exact SHA `a566fbc54349af9eaf0589939b0837a6ea08c02f`

Direct readback of `crates/one-core/src/adapter.rs` at the exact PR #110 HEAD confirms the following implementation facts:

- `FileReadAdapter` and `FileWriteAdapter` each retain only `AdapterMetadata`; neither retains a root directory handle or a `ProjectFilesystem` authority.
- Both public constructors are parameterless (`new()`), and `Default` delegates to those constructors. There is no trusted-root injection boundary in these adapters.
- `FileReadAdapter::invoke` passes the requested target directly to `std::fs::read(path)`.
- `FileWriteAdapter::invoke` passes the requested target directly to `std::fs::write(path, ...)`.
- Existing unit tests exercise a repository-relative read, a missing-path error, and a temporary-directory write. They do not establish root confinement, symlink/reparse-point rejection, mount/volume confinement, race resistance, bounded file size, or atomic replacement.
- The metadata currently labels both adapters `AdapterStatus::Implemented`; that status is not evidence that the Issue #104 security contract is implemented. Do not change the public status semantics as part of a narrow filesystem patch without auditing all consumers and documentation.
- `crates/one-core/Cargo.toml` already has target-specific `libc` on Unix and `windows-sys` features for Foundation, Security, Storage/FileSystem, and Threading. The Windows dependency feature list must be checked against the exact native APIs selected; do not assume required NT declarations are exposed by the current feature set.

### Required call-site migration before implementation can be considered complete

The code change must inventory every `FileReadAdapter::new`, `FileWriteAdapter::new`, `Default` construction, and interpreter/adapter registry construction. A safe API migration must ensure no production call site can construct a filesystem adapter without an explicit trusted `ProjectFilesystem` authority. A compatibility constructor that silently grants an inferred root, or derives the root from the request target, is prohibited. If source compatibility is required, preserve it only as a fail-closed constructor that cannot perform filesystem access without authority.

### Test gaps that must become executable acceptance cases

1. Relative in-root read/write succeeds; absolute in-root read/write succeeds only when the root-relative mapping is unambiguous.
2. Outside-root absolute targets, `..`, prefix-confusion siblings, empty/ambiguous components, device paths and unsupported UNC paths fail closed.
3. Symlink/reparse-point traversal at every parent depth and at the leaf is rejected.
4. Mount/bind-mount or volume crossing is rejected, including same-device bind-mount cases on Linux where mount identity differs.
5. Hard-link policy is enforced on read and write.
6. Non-regular leaves (directory, FIFO, socket, device) are rejected without blocking.
7. Oversized reads and writes stop at configured limits without unbounded allocation or partial unsafe mutation.
8. Concurrent parent replacement, symlink swap, leaf replacement, and rename races never read or modify an outside sentinel.
9. Failed writes clean temporary entries; successful writes publish atomically and preserve the documented durability semantics.
10. Tests run on Unix and Windows from a committed lockfile and assert named test execution rather than treating a successful compile or empty test filter as acceptance.

This audit is source inspection only; no compiler, unit test, adversarial test, or platform verification was run for this checkpoint.


## 18. Latest exact-HEAD refresh — 2026-11-18

- Live PR #110 HEAD is `88e36dfef55cc11972b9bbc68a1883385d9f52e8`; PR remains OPEN / DRAFT / UNMERGED.
- CircleCI `ci/circleci: one-linux` for this exact SHA was PENDING at last status refresh (`2026-11-18T04:59:48Z`), job `cfcaee95-6a29-4254-ad62-11c2c997238d`, workflow `e6d1cd32-da3c-404b-9ef9-a7ef209a4af9`. The preceding documentation commit `a566fbc54349af9eaf0589939b0837a6ea08c02f` later reached SUCCESS, but that result does not transfer to this newer SHA.
- Re-read `crates/one-core/src/adapter.rs`, `crates/one-core/src/interpreter.rs`, and `crates/one-cli/src/main.rs` at exact SHA `88e36df...`. Adapter still calls `std::fs::read(path)` and `std::fs::write(path, ...)`; constructors remain parameterless; interpreter still constructs both with `new()`; CLI source does not establish/inject a trusted filesystem root. Issue #104 remains NOT IMPLEMENTED.
- Commit `88e36df...` is documentation-only (`docs/security/issue-104-filesystem-root-confinement-design.md`); no production filesystem code or test evidence was added by that commit.
- This refresh supersedes earlier statements in sections 16–17 that described `a566fbc...` as the latest HEAD. Those sections remain historical audit checkpoints, not current branch metadata.


## 19. Exact source re-audit — 2026-11-18 — HEAD 871a399

- Read `crates/one-core/src/adapter.rs` at exact PR #110 HEAD `871a39967db315c695a642f527c41c7172922201` (blob `c5abaf9178eae81bea7c80402990f2adae130e2d`). `FileReadAdapter::invoke` still calls `std::fs::read(path)`; `FileWriteAdapter::invoke` still calls `std::fs::write(path, &content)`. Both constructors remain parameterless `new()`; this is a direct source confirmation that root-handle authority is not implemented.
- Read `crates/one-core/src/interpreter.rs` at the same source SHA (blob `db1f421133e2c4f866c05e91c55b2ce2b5016369`); it still constructs `FileReadAdapter::new()` and `FileWriteAdapter::new()`.
- Read `crates/one-cli/src/main.rs` at the same SHA (blob `6ff60189eabb19886d8d4636645aea17cf9997e2`); the CLI does not directly inject a trusted project filesystem authority into those adapters.
- PR #110 remains OPEN / DRAFT / UNMERGED. CircleCI `ci/circleci: one-linux` for this exact HEAD was PENDING at `2026-11-18T05:01:55Z`, job `07d035f3-cac0-4a75-a4f8-0eaaa3f19a4b`; no CircleCI log inspection or terminal result is claimed.
- This is source inspection only. No production source or test was changed; Issue #104 remains NOT IMPLEMENTED / NOT ACCEPTED.


## 20. Recheck and verifier-ledger boundary — 2026-11-18

- Rechecked `crates/one-core/src/adapter.rs` at PR #110 SHA `4a1dbdf08e9235bb070e970dfb59c33cc9a6c549`. `FileReadAdapter::invoke` still calls `std::fs::read(path)` and `FileWriteAdapter::invoke` still calls `std::fs::write(path, &content)`; both constructors remain parameterless. Issue #104 remains unimplemented in production source.
- PR #110 SHA `4a1dbdf...` has CircleCI `ci/circleci: one-linux` PENDING at last refresh; job `54d76c6c-e85d-4a91-984f-20dec0a0f701`, workflow `524e7b1e-de00-49fd-9a25-69266a2c5fd5`. This is not a test result yet.
- The independent verifier's `ONE Fixed-Weight Ledger Gate` currently fails closed because the pinned source SHA `e50748a1b25922d9090eba1f5fb90548d7c7d003` does not contain `docs/verification/one-completion-ledger.md` or `docs/verification/DEFINITION_OF_DONE.md`. Its unit tests pass (15/15) on run `44241595784` and `44241822346`; the subsequent live-input step fails with `FileNotFoundError` for `one-source/docs/verification/one-completion-ledger.md`. Historical copies exist at `2ead9dba45db4ce4105deaa81720aacc8fcadd6c`, but must not be silently reused as current acceptance inputs. The verifier branch's `LEDGER_GATE.md` explicitly records this root cause and calls for a versioned current-source ledger/DoD plus separation of the ledger documentation pin from the E4 security pin.
- No ledger scores, DoD checks, source pins, or security acceptance were changed in this checkpoint. The evidence matrix and implementation-status documents are not format-equivalent substitutes for the fixed-weight ledger.


## 21. Remote verification route and native Windows API availability — 2026-11-18

This section supersedes the older execution-gate statements above where they say no remote compile/test route exists, and supersedes section 20's statement that the independent ledger gate still fails on missing input files.

### Current exact candidate and remote CI path

- The source snapshot rechecked immediately before this documentation update was PR #110 HEAD `005476dd8cf810fff2989cc03f22b59b9f26f644` (documentation-only); CircleCI `ci/circleci: one-linux` was PENDING for that exact SHA at job https://app.circleci.com/workflow/2f9eccab-8721-4cc6-981b-3ad50213eb8b/job/94ba22d0-cb36-49da-8445-37ca6891f30d. This section was committed in documentation-only commit `4152b3aa9d2d4acffbb72c1caf4ddeb8a60dc091`; the new commit's CircleCI status had not yet been returned at this checkpoint. No terminal result or expanded CircleCI logs are available here.
- The current PR #110 tree contains the committed `Cargo.lock` (blob `3fd2cb2be6aa30afa54602c733b2ab192cc91d2b`). The historical E4 pin `e50748a1b25922d9090eba1f5fb90548d7c7d003` does not contain `Cargo.lock`; historical E4 run `43998206030` failed before any named test because Cargo `--locked` could not find it.
- A remote validation route therefore exists: PR #110 CircleCI `one-linux` for Linux build/test; the independent verifier's E4 workflow can compile and execute the named Unix and Windows adversarial tests once a candidate SHA contains the committed lockfile, implementation, and exact test inventory. The E4 pin MUST remain unchanged until those code and tests are committed. There is no local Rust compiler/clone in this session, so CI must be the actual execution evidence.

### Ledger gate input repair is complete; acceptance remains blocked

- The independent verifier now keeps `CANONICAL_COMMIT.txt` (E4 candidate pin) separate from `LEDGER_SOURCE_COMMIT.txt` (ledger/DoD documentation pin). The latter points to exact PR #103 docs commit `d6c382a245c49a028bc60809f22ca956c7f894cc`.
- Verifier run `44242751457` on verifier SHA `074e9f06b96f8569662b957c1b95f884a89eaebe` successfully checked out both source revisions and passed all 16 Python unit tests. It then correctly returned NOT PASSED from the actual acceptance gate: 18 ledger rows, fixed weights 100, provisional implementation 92/100, verified 0/100, 10 unchecked DoD items, required exact-SHA workflow failures on historical candidate `e50748a...`, and no protected package-provenance release run for that SHA.
- This resolves the verifier's missing-file precondition; it does not establish E1–E6 acceptance, alter any score, or validate PR #110 product code.

### Windows native handle-relative API feasibility finding

- The pinned dependency is `windows-sys 0.61.2`. Its documented `NtCreateFile` binding is exported under `windows_sys::Wdk::Storage::FileSystem`; the signature accepts `OBJECT_ATTRIBUTES`, enabling a `RootDirectory` handle-relative open. `OBJECT_ATTRIBUTES` is exposed under `windows_sys::Wdk::Foundation` and contains `RootDirectory` and `ObjectName` fields.
- Official API references inspected: [windows-sys 0.61.2 NtCreateFile](https://docs.rs/windows-sys/0.61.2/windows_sys/Wdk/Storage/FileSystem/fn.NtCreateFile.html) and [OBJECT_ATTRIBUTES](https://docs.rs/windows-sys/0.61.2/windows_sys/Wdk/Foundation/struct.OBJECT_ATTRIBUTES.html).
- Current `crates/one-core/Cargo.toml` enables Win32 Foundation/Security/Storage/FileSystem and process features, but does not yet enable the required WDK feature modules. Before Windows implementation, add only the exact feature gates required for `Wdk::Foundation`, `Wdk::Storage::FileSystem`, and the relevant I/O status structure, then validate compile on the independent Windows runner. Do not substitute path canonicalization followed by a second open.
- This API discovery is feasibility research only. No production source or test was changed; Issue #104 remains OPEN / NOT IMPLEMENTED.

## 15. Independent Linux + Windows baseline verification — 2026-11-18

The verifier input issue was bypassed safely with a dedicated Linux-only workflow; the successful evidence below is bound to the pinned source candidate, not to the verifier workflow commit:

- Candidate source SHA: `a20676be53e5f78c465832379d86c3675a2ed7c8` (PR #108 head at verification time; also recorded in `CANDIDATE_BUILD_COMMIT.txt`).
- Linux-only run: [44267886845](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44267886845), job `134676844501`, Ubuntu 24.04.5, SUCCESS. The job validated the candidate pin, checked out the exact source SHA, installed Rust 1.100.0 and MinGW, then completed `cargo test --locked -p one-core -- --nocapture`. Decoded job log contains 27 test-result summaries: 239 passed, 0 failed, 1 ignored.
- Windows job: run [44263674485](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44263674485), job `134662858839`, SUCCESS. It checked out the same exact candidate SHA and completed the full `one-core` test suite. Decoded job log contains 27 test-result summaries: 232 passed, 0 failed, 1 ignored.
- These are matching cross-platform baseline results for the same exact source candidate. The platform-specific test totals differ by seven due to target-specific compilation/test selection; both report zero failures.
- PR #110 HEAD `2ab82ea9cccb1c9aa0fef25c3591a1310ab84b5c` is a separate stacked candidate and has its own CircleCI `ci/circleci: one-linux` SUCCESS. Do not conflate that status with the independent Windows run, which is pinned to `a20676be...`.
- The old full matrix run `44263674485` still has its Linux job stuck at the original MinGW install step; the Windows job from that run is the success cited above. The separate Linux-only run is the successful Linux evidence.
- Issue #104 remains OPEN / NOT IMPLEMENTED. This baseline suite contains no required root-handle confinement implementation or adversarial acceptance tests; it does not satisfy E4 or authorize a score/DoD promotion.


## 22. Implementation branch and native API compile checkpoint — 2026-11-18

### Branch and PR

- Created isolated implementation branch `security/issue-104-root-confinement-20261118` from exact source SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`, the candidate that passed the independent Linux/Windows baseline matrix.
- Opened draft PR #113: https://github.com/panglimanetizen01/one/pull/113.
- PR #113 is stacked directly on PR #112 branch `fix/windows-process-verifier-timeouts-20261118`, whose base SHA is the same `181ad350...`. This keeps the Issue #104 work isolated from the Windows process-test remediation.
- Current PR #113 head: `8b92038afb205e8f36353b34ce808dba1ce1d389`.
- Canonical `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`; no merge or main update.

### Changes in this first spike

- `crates/one-core/Cargo.toml`: enabled `windows-sys 0.61` features `Wdk_Foundation` and `Wdk_Storage_FileSystem`.
- Added `crates/one-core/src/filesystem.rs` with a compile-time-only Windows binding probe that references `Wdk::Foundation::OBJECT_ATTRIBUTES` and `Wdk::Storage::FileSystem::NtCreateFile`. The latter accepts an `OBJECT_ATTRIBUTES` structure with a `RootDirectory` handle, which is the native API surface needed for handle-relative opens.
- Registered `pub mod filesystem` in `crates/one-core/src/lib.rs`.
- No filesystem read/write operation, authority object, or path fallback was implemented by this spike.

### Validation state

- CircleCI `ci/circleci: one-linux` for exact head `8b92038...` is PENDING, job https://app.circleci.com/workflow/30362ce7-e263-4558-b8c8-fa0366b4052b/job/8e7fa151-00a3-4b77-a3a7-18868ed259d6. No compile PASS is claimed yet.
- The prior independent baseline matrix run [44285138516](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44285138516) remains valid only for its exact source SHA `181ad350...`; it does not transfer to PR #113.
- The next checkpoint is to compile this probe on Windows and Linux, then build the immutable host-owned root authority and platform-specific primitives only after compile evidence.

### Security boundary remains unchanged

Issue #104 remains OPEN / NOT IMPLEMENTED / NOT ACCEPTED. Outstanding work includes:
- retained root handle and host injection path;
- safe absolute in-root mapping and traversal validation;
- Unix descriptor-relative no-follow traversal and mount identity;
- Windows NtCreateFile RootDirectory traversal, reparse-point rejection, and volume/file identity;
- hard-link and regular-file validation;
- bounded read/write I/O and atomic same-directory replacement;
- policy-first and fail-closed no-authority tests;
- required nonzero Unix/Windows adversarial test names from the independent verifier;
- race tests and exact-SHA E4 verification.

This is an API-surface feasibility spike, not an E4 implementation or security certification.


## 23. Platform backend decision and latest CI checkpoint — 2026-11-18

### Exact candidate validation

- PR #113 current source candidate: `14d81a93a60ea2bba1ebfd2ddf7d85baf0893e24`.
- Independent verifier run [44286988623](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44286988623) completed SUCCESS on that exact source SHA:
  - Ubuntu 24.04 job `134742079698`: 239 passed, 0 failed, 1 ignored.
  - Windows Server 2025 job `134742079233`: 232 passed, 0 failed, 1 ignored.
- CircleCI `ci/circleci: one-linux` for exact source SHA `14d81a93...` is now SUCCESS (was pending at the previous checkpoint).
- These are compile/test baseline results only. The filesystem module remains a compile-time API probe and does not perform reads or writes.

### Capability-library evaluation: cap-std 4.0.3

Reviewed the current `cap-std 4.0.3` API and its upstream documentation/source before considering adoption.

- Positive: `cap_std::fs::Dir` is backed by an open directory handle; `Dir::open` and `Dir::open_with` resolve relative to that handle. Its documentation explicitly requires a Windows root file handle opened without `FILE_SHARE_DELETE` to prevent rename races.
- Blocking mismatch: the documented `Dir::open` model permits symlink traversal when the resolved destination remains beneath the capability root. Issue #104's selected strict policy requires rejecting symlink/reparse-point components, not merely preventing escape through them.
- The public `OpenOptions` API does not expose a portable no-follow-all-components contract. Adding a separate metadata precheck would reintroduce a TOCTOU race and is rejected.
- The public API evidence inspected does not establish the required no-cross-mount / bind-mount policy and mount identity check for this project's strict contract.
- Security hygiene: do not use cap-std/cap-primitives versions `<=4.0.2`; upstream advisory GHSA-hp8f-xmx4-4qrg documents a critical trailing-slash symlink-follow bypass, patched in 4.0.3. The existence of the patched version does not resolve the policy/API mismatch above.

Decision: do NOT add cap-std as the Issue #104 enforcement boundary. Do not wrap it with path metadata prechecks. Keep the native implementation explicit and platform-specific.

### Native implementation sequence

1. Introduce an immutable host-owned `FilesystemAuthority` that retains an opened root handle and explicit read/write byte limits. No authority supplied means fail closed.
2. Linux/Android: use a single root-handle-relative `openat2` operation with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`; unsupported kernel/syscall or unsupported path form fails closed. Verify leaf type, mount identity, link count and bounded size on the opened handle.
3. Windows: use `NtCreateFile` with `OBJECT_ATTRIBUTES.RootDirectory`, no-follow/reparse-point open semantics, and root/parent handles opened without delete sharing while resolving. Validate volume/file identity, leaf type and link count. A compile-only binding probe is not sufficient.
4. macOS and other Unix targets: remain unavailable/fail closed until a race-resistant primitive meeting the same strict contract is implemented and independently tested. Do not silently substitute ordinary `openat` component loops where rename-out-of-root races are unresolved.
5. Integrate the authority into both direct adapters and the interpreter/CLI host boundary. Preserve policy-first evaluation and approval/explicit-deny precedence.
6. Add exact-name nonzero adversarial tests to the independent E4 filters; test actual outside-file side effects, symlink/reparse-point races, parent rename, mount crossing, hard links, regular-file checks, bounded I/O and atomic replacement.

### Threat-boundary note

A retained directory handle alone does not prove that a descendant remains in the original namespace if an attacker can rename an opened parent outside the root while traversal continues. Linux/Android implementation therefore uses one kernel-enforced beneath/no-symlink/no-cross-mount resolution operation rather than a sequence of individually safe-looking `openat` calls. Windows must hold relevant directory handles without delete sharing during traversal and verify the opened object identities. If a platform cannot establish these invariants, its adapter remains unavailable.

Issue #104 remains OPEN / NOT IMPLEMENTED / NOT ACCEPTED. No E4 or ledger acceptance pin is changed by this design decision.


## 24. Independent E4 adversarial test-name contract — 2026-11-18

The verifier workflow `.github/workflows/e4-adversarial-regression.yml` is the authoritative test-name contract. The current source candidate does not yet contain these tests; a successful generic `cargo test -p one-core` run is not a substitute.

### Unix filters (must execute nonzero exact counts)

| Verifier filter | Required passing matches | Required evidence |
|---|---:|---|
| `descriptor_relative_read_rejects_` | 2 | Two distinct read traversal/symlink rejection cases with no outside read |
| `descriptor_relative_write_rejects_` | 1 | Write through hostile path rejected with no outside create/modify |
| `descriptor_relative_walk_rejects_` | 1 | Parent traversal rejected before any external side effect |
| `adapter::filesystem_nofollow_tests::descriptor_relative_read_race_never_follows_swapped_parent_symlink` (`--exact`) | 1 | Concurrent parent-component swap cannot redirect read outside root |
| `adapter::filesystem_nofollow_tests::final_component_mount_boundary_denial_does_not_truncate_file` (`--exact`) | 1 | Mount-boundary denial leaves pre-existing outside/target file bytes unchanged |

### Windows filters (each must execute exactly once)

- `filesystem_reader_rejects_directory_junction_escape`
- `filesystem_adapter_reads_absolute_target_inside_project_root`
- `filesystem_adapters_confine_absolute_targets_to_project_root_without_side_effects`
- `filesystem_write_adapter_writes_text_target`
- `filesystem_write_adapter_rejects_oversized_content`
- `filesystem_adapters_reject_parent_directory_traversal`

### Test construction rules

- Positive absolute-path control must prove an absolute target inside the configured project root still works; do not implement blanket absolute-path rejection.
- Every negative case must assert actual outside-file bytes and directory entries before/after, not only an error value.
- Race test must coordinate concurrent parent-component replacement and assert no external target was opened/read.
- Mount test must assert denial happens before truncation and compare the exact original bytes.
- Windows junction test must create a real directory junction/reparse point and assert the outside sentinel remains untouched.
- Oversized write test must assert the existing destination remains byte-identical and no temp artifact leaks.
- Tests must use the same public adapter/host injection path as production. A helper-only unit test that bypasses adapter construction is insufficient.
- Keep the verifier's expected counts/filter names unchanged unless both verifier and source contract are reviewed together.

Current inventory status: **0 of 5 Unix filter groups and 0 of 6 Windows named tests are implemented/accepted on this candidate**. The existing generic filesystem smoke tests do not count toward these filters.


## 25. First host-authority implementation checkpoint — 2026-11-18

### Exact candidate

- Latest implementation candidate: `955e1841adf79ae400395feef86186482276818d`.
- PR #113 remains OPEN / DRAFT / UNMERGED. Canonical `main` remains unchanged.
- Independent verifier run [44292290308](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44292290308) is IN_PROGRESS for this exact source SHA. Do not claim Linux or Windows PASS until the run reaches a terminal result and both jobs are inspected.
- CircleCI `ci/circleci: one-linux` is PENDING for this exact source SHA: [job](https://app.circleci.com/workflow/b0b95c25-c8d8-4afb-b4c3-6d0b18866a78/job/4ed659a7-2816-4c3f-a5bf-af018431f051).

### Implemented slice

- Added `FilesystemAuthority`, created only from host-selected root configuration. It retains the root directory file handle and explicit read/write byte limits.
- Linux file reads use one `openat2` call relative to the retained root descriptor with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`. There is no ordinary path fallback.
- Read validation requires a regular file, rejects link counts other than one, and enforces the configured maximum while reading.
- Requested relative paths reject parent traversal. Absolute targets are accepted only when lexically beneath the canonical host root, then converted to relative paths before descriptor-relative resolution.
- Direct file adapters require injected authority and no longer call ambient `std::fs::read(target)` or `std::fs::write(target)`.
- Interpreter exposes `with_orchestration_policy_and_filesystem_authority`; the Linux source-to-runtime read integration test now injects the authority.
- Policy denial remains prior to adapter invocation. An allowed write request currently reaches the adapter and fails closed; its runtime test asserts no destination file is created.

### Explicit limitations

- Write/create/replace remains unavailable. No atomic write, pre-mutation size enforcement, or race-resistant write boundary is implemented.
- Windows, macOS, and Android authority backends remain unavailable/fail-closed. The current `openat2` implementation is compiled only for Linux; Android syscall availability is not assumed.
- CLI/application host-root configuration and full integration-test migration remain outstanding.
- This implementation has not yet proven resistance to all parent-directory rename-out-of-root races. The exact E4 race and mount tests remain mandatory before acceptance.
- E4 inventory remains 0/5 Unix filter groups and 0/6 Windows named tests implemented/accepted. The initial authority unit tests are not substitutes for the exact E4 filters.

### Validation corrections retained as evidence

- Candidate `a077dcc27512669d4d8b5cff7602b459c86d01e2` failed two Linux integration tests because the old runtime test still constructed the interpreter without host authority and expected the old adapter failure classification.
- Candidate `d20bac50f25e4660d662d69d704daff9a4dc95ee` fixed those read integration tests, then the Linux matrix exposed one remaining legacy write runtime test that expected a successful ambient write.
- Candidate `955e1841adf79ae400395feef86186482276818d` changes that write test to assert fail-closed behavior and no file creation. Its new exact-SHA matrix is still in progress; no pass is claimed here.

Issue #104 remains OPEN / NOT IMPLEMENTED / NOT ACCEPTED. Keep PR #113 draft and do not modify E4 or ledger acceptance pins.


## 26. Exact candidate matrix completed; CircleCI failure remains unresolved — 2026-11-18

### Independent verifier result

- Exact ONE code candidate tested: `955e1841adf79ae400395feef86186482276818d`.
- Independent run [44292290308](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44292290308) completed SUCCESS.
- Linux job `134760262324`: SUCCESS; full `one-core` suite passed. Main test binary reported 137 passed, 0 failed; additional integration binaries also passed, including 5/5, 6/6, 9/9, 1/1, 7/7, and remaining suites with no failures. One test remains ignored in a separate suite.
- Windows job `134760262885`: SUCCESS; full `one-core` suite passed. Main test binary reported 126 passed, 0 failed; all remaining integration binaries reported zero failures, with one ignored test in a separate suite.
- This verifies the exact candidate code SHA on the independent Linux and Windows matrix. It does not verify Android/macOS and does not constitute E4 security acceptance.

### CircleCI result on the same code candidate

- GitHub commit status `ci/circleci: one-linux` is FAILURE for exact SHA `955e1841adf79ae400395feef86186482276818d`.
- Status description: `Job failed. To see why, run the CircleCI CLI: circleci run get cb880159-7100-4893-b863-3bfe2f717b0b --failure-report`.
- CircleCI job URL: [one-linux job](https://app.circleci.com/workflow/b0b95c25-c8d8-4afb-b4c3-6d0b18866a78/job/4ed659a7-2816-4c3f-a5bf-af018431f051).
- CircleCI's job log is not accessible through the connected GitHub Actions interface in this session; no root cause is asserted. Do not modify the CircleCI config or add retries based on a guess. Obtain the CircleCI failure report/logs through an authenticated CircleCI CLI/API session, then classify and fix the first failing step.
- Thus candidate has independent Linux/Windows PASS, but overall CI is NOT GREEN until CircleCI is diagnosed and passes.

### Acceptance boundary unchanged

- E4 remains 0/5 Unix filter groups and 0/6 Windows named tests implemented/accepted.
- Writes remain fail-closed; Windows/macOS/Android authority backends remain unavailable.
- Issue #104 remains OPEN / NOT IMPLEMENTED / NOT ACCEPTED. Keep PR #113 draft/unmerged and do not change acceptance pins.


## 27. CircleCI Linux cross-toolchain footprint reduction — 2026-11-18

### Change

- Updated `.circleci/config.yml` to replace the broad Ubuntu `mingw-w64` meta-package with the targeted `gcc-mingw-w64-x86-64-posix` package required by the x86_64 Windows GNU artifact test.
- Retained Clang and `binutils-x86-64-linux-gnu`; the latter supports the Linux x86_64 linker test.
- Updated the compiler probe to invoke the package's explicit `x86_64-w64-mingw32-gcc-posix` executable.
- Added bounded apt update/install timeouts, HTTP(S) timeouts, retry policy, and dpkg lock timeout.
- Rationale is grounded in `crates/one-core/tests/native_backend_linux_windows.rs`: Linux CI runs `NativeWindowsX86_64Backend::build` and requires a Windows GNU x86_64 toolchain. It does not require every MinGW architecture and threading variant included by the broad meta-package.

### Validation boundary

- This is a CI footprint/reliability optimization, not a proven root-cause fix for the earlier CircleCI failures. CircleCI job logs are still required to identify those failures.
- Exact ONE source candidate with the config change: `2c3d9e75ad94cff25728c783b675d8e492499d2d`.
- Independent candidate matrix run [44295933072](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44295933072) was in progress at checkpoint: Windows toolchain/test steps had not yet completed; Linux was at the bounded minimal-toolchain install. No matrix result claimed.
- Earlier candidate `62c1a19a0c8bedcaf9e87be23ee082cc793c78c3` had Windows PASS (126 main tests, zero failures), but Linux remained at its MinGW installation step. That Windows result does not transfer to this new exact source SHA.
- Issue #104 remains OPEN / NOT IMPLEMENTED / NOT ACCEPTED. Keep E4 and ledger pins unchanged; PR #113 remains draft/unmerged; main remains untouched.


## 28. Remove successful-run duplicate test execution in CircleCI — 2026-11-18

- The full `cargo test --locked --workspace --all-targets` step already executes the deterministic CLI artifact, DNS, orchestration, database, and HTTP vertical-slice tests.
- The six immediately following focused commands repeated subsets of those same targets on every successful run; they did not add a distinct test configuration or coverage boundary.
- Consolidated them into one CircleCI `run` step with `when: on_fail`. The focused commands remain available as diagnostic reruns after a prior step fails, but no longer consume time on a fully successful pipeline.
- This is based on command/target overlap in the checked-in config and Rust integration-test layout, not a measured duration claim. CircleCI duration/test insights are not available in this session.
- The previous toolchain optimization is retained: use the x86_64 POSIX MinGW package instead of the all-architecture meta-package, with bounded apt/network/lock timeouts.
- Exact source candidate with both CircleCI optimizations: `a2cc2fdaac4fb21220953aa67d98c7f80aa70a9c`.
- Candidate matrix and CircleCI validation are pending for this exact SHA. No pass is claimed here.
- Rollback is limited to reverting the two isolated `.circleci/config.yml` commits if CircleCI's validator or runtime rejects the step syntax/command sequence; no product code behavior is changed by these CI edits.


## 29. Exact candidate matrix after CircleCI config optimizations — 2026-11-18

- Exact ONE source SHA: `59f856fbe695fb514a4d894c880840bea3f57866`.
- Independent verifier run [44297294679](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44297294679) completed SUCCESS on that exact SHA.
- Linux job `134777549091`: SUCCESS; main `one-core` binary 142 passed / 0 failed. All integration binaries had zero failures; one existing ignored test remains in a separate suite. The four new Linux adapter-path rejection tests and the parent-symlink swap race test all passed.
- Windows job `134777548731`: SUCCESS; main `one-core` binary 126 passed / 0 failed. All additional test binaries reported zero failures; one existing ignored test remains in a separate suite.
- This is exact-SHA Linux/Windows baseline verification, not E4 acceptance, not Android/macOS verification, and not a release gate.
- CircleCI `ci/circleci: one-linux` for this SHA was still PENDING at checkpoint: [job](https://app.circleci.com/workflow/badb07eb-447c-43d0-bc1b-93415ffc077b/job/886fcdd8-3bef-4302-b869-3cfe63893a03). CircleCI job page is not accessible through the current web/connector session. Do not claim CircleCI success or infer failure cause.
- The CircleCI config now avoids successful-run duplicate focused tests while retaining them under `when: on_fail`, and uses a targeted x86_64 POSIX MinGW package with bounded APT operations. No measured runtime delta is claimed.
- Issue #104 remains OPEN / NOT IMPLEMENTED / NOT ACCEPTED: mount-boundary no-truncate test and all six Windows E4 tests are outstanding; safe write remains fail-closed; Windows/macOS/Android filesystem backends remain unavailable. Keep E4 and ledger pins unchanged; PR #113 draft/unmerged; main unchanged.


## 30. Windows reparse and handle-relative API research — 2026-11-18

### API facts confirmed from Microsoft documentation

- `OBJECT_ATTRIBUTES.RootDirectory` may reference a filesystem directory handle. When non-NULL, `ObjectName` is resolved relative to that handle: https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-_object_attributes
- The same `OBJECT_ATTRIBUTES` reference documents `OBJ_DONT_REPARSE`: if a reparse point is encountered while parsing the object name, the operation fails with `STATUS_REPARSE_POINT_ENCOUNTERED`. This is the strongest documented candidate guard for rejecting reparse traversal during handle-relative opens.
- `NtCreateFile` documents RootDirectory-relative opens and `FILE_OPEN_REPARSE_POINT`. The latter opens the reparse point itself rather than following it; the resulting handle still requires attribute/tag inspection and rejection before data I/O: https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntcreatefile
- `NtCreateFile` documents `FILE_SHARE_DELETE` as allowing other opens for delete access. Omitting delete sharing on retained directory handles is therefore part of the intended parent-rename defense while descendants are resolved. The handles must remain alive throughout the complete operation.
- `FILE_ID_INFO` supplies both `VolumeSerialNumber` and a 128-bit `FileId`; compare volume identity across opened components to reject volume/mount transitions and compare object identity where needed: https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_id_info
- `FILE_ATTRIBUTE_TAG_INFO` supplies file attributes and reparse tag; `GetFileInformationByHandleEx` supports FileAttributeTagInfo and FileIdInfo: https://learn.microsoft.com/en-us/windows/win32/api/winbase/ns-winbase-file_attribute_tag_info and https://learn.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-getfileinformationbyhandleex

### Documentation discrepancy and verification requirement

- The general `OBJECT_ATTRIBUTES` page documents `OBJ_DONT_REPARSE`, while the user-mode `NtCreateFile` page describes its Attributes argument more narrowly as zero or `OBJ_CASE_INSENSITIVE`. Microsoft Q&A discussion dated 2026-10-19 reports that the user-mode SDK header lists `OBJ_DONT_REPARSE` among valid attributes, but explicitly notes that the function page does not confirm its support/scope. Treat this as an unresolved documentation discrepancy, not as a platform-wide guarantee: https://learn.microsoft.com/en-us/questions/6042974/is-obj-dont-reparse-supported-in-object-attributes
- A separate Microsoft Q&A runtime report says `OBJ_DONT_REPARSE` was honored for a RootDirectory-relative single-component `NtCreateFile` open on Windows 11 build 26200. This is useful observed behavior, but does not replace execution on the project's Windows Server 2025 independent runner: https://learn.microsoft.com/answers/a/12957558

### Implementation direction (proposed; not yet implemented)

1. Acquire the initial project root only from trusted host configuration. The root-acquisition boundary and its ancestor path remain a separate trust decision.
2. Open each descendant as a single component relative to the already-open parent handle, with `OBJ_DONT_REPARSE`; retain each parent handle for the full operation and omit `FILE_SHARE_DELETE` so concurrent delete/rename opens are denied while resolution is in progress.
3. Use `FILE_OPEN_REPARSE_POINT` where needed to inspect a leaf object itself; query and reject reparse attributes/tags before reading or writing. If the native call returns a reparse-related or unsupported status, fail closed.
4. Query volume serial/file ID, link count, directory/regular-file type, and size from the opened handle—not from a pre-open path lookup. Reject volume changes and hard-linked leaves under the selected policy.
5. Validate `OBJ_DONT_REPARSE` behavior and status mapping with real junction/reparse-point tests on the exact Windows runner before enabling the backend. Do not silently fall back to Win32 path-based open calls.

### Current dependency and status

- The current `one-core` Cargo target dependency already enables `Wdk_Foundation`, `Wdk_Storage_FileSystem`, `Win32_Foundation`, and `Win32_Storage_FileSystem`; this research alone does not justify another feature change.
- No production code was changed by this research. No Windows backend, junction denial, parent-rename protection, volume identity check, or write operation is claimed implemented or verified.
- The six required Windows E4 test names remain outstanding. Issue #104 remains OPEN / NOT IMPLEMENTED / NOT ACCEPTED; do not advance the E4 verifier pin until the exact required tests exist and pass.

## 31. Rust windows-sys implementation surface and compile constraints — 2026-11-18

### Existing generated bindings in the pinned dependency

- `windows-sys` is already pinned by the existing dependency resolution at the manifest's `0.61` requirement; `Cargo.toml` enables `Wdk_Foundation`, `Wdk_Storage_FileSystem`, `Win32_Foundation`, and `Win32_Storage_FileSystem`.
- The generated `NtCreateFile` binding is `windows_sys::Wdk::Storage::FileSystem::NtCreateFile`; its signature accepts a `HANDLE`, `OBJECT_ATTRIBUTES`, `IO_STATUS_BLOCK`, access/share/disposition/options flags, and returns `NTSTATUS`: https://docs.rs/windows-sys/0.61.2/windows_sys/Wdk/Storage/FileSystem/fn.NtCreateFile.html
- `OBJECT_ATTRIBUTES` is exposed under `windows_sys::Wdk::Foundation`; its generated layout includes `Length`, `RootDirectory`, `ObjectName`, `Attributes`, `SecurityDescriptor`, and `SecurityQualityOfService`: https://docs.rs/windows-sys/0.61.2/windows_sys/Wdk/Foundation/struct.OBJECT_ATTRIBUTES.html
- The `IO_STATUS_BLOCK` used by this binding resolves to `windows_sys::Win32::System::IO::IO_STATUS_BLOCK`, already enabled by `Win32_System_IO`: https://docs.rs/windows-sys/0.61.2/windows_sys/Win32/System/IO/struct.IO_STATUS_BLOCK.html
- Root handle acquisition is a separate operation from descendant resolution. The intended trusted-root opener can use `CreateFileW` with directory backup semantics, then transfer the owned HANDLE into `std::fs::File` only after validating it is a directory. Descendants must be opened as one path component at a time relative to retained parent handles; do not pass a multi-component untrusted path or use ambient `std::fs::File::open` as a fallback.

### Metadata and stable-Rust constraint

- `std::os::windows::fs::MetadataExt` exposes `file_attributes`, `file_size`, `volume_serial_number`, `number_of_links`, and `file_index`, but the latter three are documented as nightly-only experimental APIs in the current standard-library reference. Do not rely on them under this repository's pinned stable toolchain: https://doc.rust-lang.org/stable/std/os/windows/fs/trait.MetadataExt.html
- Use stable handle-based Win32 `GetFileInformationByHandleEx` with `FileAttributeTagInfo`, `FileStandardInfo`, and `FileIdInfo` to obtain reparse attributes/tags, directory/link-count/size information, and volume/file identity. Reject missing/unsupported metadata rather than silently weakening the checks.

### Required implementation shape before enabling Windows

1. `FilesystemAuthority::open` on Windows must open the host-trusted canonical root as a directory handle (backup semantics), validate it, and store the owned handle. The existing non-Linux early return currently prevents this.
2. `open_relative` must reject non-normal components and open each component relative to the retained parent handle using `NtCreateFile`, with `OBJ_DONT_REPARSE` and a directory-only option for intermediate components. Keep parent handles alive for the full walk and omit delete sharing while those handles protect the walk.
3. Inspect each opened component from its handle; reject reparse attributes, unexpected object type, hard-linked leaf files, and volume identity changes. Use `FILE_OPEN_REPARSE_POINT` only when inspecting the leaf itself, never as permission to read/write through a reparse object.
4. Read from the final owned handle only after all checks pass and enforce the byte limit while streaming, not only from a pre-read size.
5. Implement writes as a separate design: no truncating existing target before all validation, no path-based fallback, and atomic handle-relative replacement or create-new semantics. The mount-boundary no-truncate and Windows tests must demonstrate no side effects on denial.
6. Add compile-time Windows build coverage and run the six exact verifier tests on Windows Server 2025. A successful Linux build does not compile or validate this cfg(windows) implementation.

### Explicit status

- This section is an implementation map, not production code. No Windows implementation was added or compiled in this checkpoint.
- The current stable toolchain is `1.100.0`; the proposed design deliberately avoids nightly-only `MetadataExt` accessors.
- Keep Windows filesystem adapters fail-closed until the full handle walk, metadata checks, positive read/write cases, and six named Windows adversarial tests pass on the independent runner.

## 32. User-mode NtCreateFile reparse contract: unresolved security gate — 2026-11-18

### Authoritative documentation reconciliation

- Microsoft Learn documents that `NtCreateFile` accepts a non-NULL `OBJECT_ATTRIBUTES.RootDirectory` and interprets `ObjectName` relative to that directory handle: https://learn.microsoft.com/windows/win32/api/winternl/nf-winternl-ntcreatefile
- The general `OBJECT_ATTRIBUTES` reference defines `OBJ_DONT_REPARSE` and says parsing fails with `STATUS_REPARSE_POINT_ENCOUNTERED` if a reparse is encountered: https://learn.microsoft.com/windows/win32/api/ntdef/ns-ntdef-_object_attributes
- However, the user-mode `NtCreateFile` page's function-specific `Attributes` description says the value can be zero or `OBJ_CASE_INSENSITIVE`; it does not list `OBJ_DONT_REPARSE`. Microsoft Q&A moderator responses dated 2026-10-19 and 2026-11-02 explicitly distinguish these two published contracts and conclude that the exact user-mode combination `NtCreateFile + filesystem RootDirectory + OBJ_DONT_REPARSE` is not established as a fully documented, version-stable security contract: https://learn.microsoft.com/en-us/questions/6042974/is-obj-dont-reparse-supported-in-object-attributes and https://learn.microsoft.com/en-us/answers/questions/6056645/does-user-mode-ntcreatefile-support-obj-dont-repar
- The latest Microsoft Q&A security analysis further states that the public documentation does not define an atomic object/effect boundary for a final-component reparse point inserted before or during a `FILE_CREATE` directory request; it therefore does not establish a guarantee that failure always precedes any foreign mutation: https://learn.microsoft.com/en-us/answers/a/13042905

### Binding implementation consequence

- `OBJ_DONT_REPARSE` remains a candidate defense and must be tested, but it MUST NOT be the sole documented security premise for enabling Windows filesystem reads or writes. The previous section 31 implementation map is a proposed direction only; this section adds a hard verification gate and supersedes any interpretation that the exact user-mode contract is already guaranteed.
- `FILE_OPEN_REPARSE_POINT` opens the reparse-point object itself and bypasses normal reparse processing for that open. This is useful for inspecting a leaf handle and rejecting it before data I/O; it does not by itself prove that intermediate components were not followed: https://learn.microsoft.com/windows/win32/api/winternl/nf-winternl-ntcreatefile
- Windows backend remains fail-closed until tests on the exact Windows Server 2025 runner demonstrate intermediate junction/symlink rejection, final-component reparse rejection, concurrent reparse insertion/replacement behavior, parent rename behavior, and no side effects on every denied write. Tests must verify sentinel bytes and directory contents after each denial.
- If the exact contract cannot be established on the supported Windows/filesystem matrix, do not ship the backend as supported. Keep `UnsupportedPlatform` and report Windows filesystem authority unavailable rather than weakening the boundary or falling back to `CreateFileW` on an untrusted multi-component path.

### Required adversarial validation additions

1. Test intermediate directory junction and symlink escape with a target sentinel outside the root.
2. Test final-component junction/symlink and assert the outside sentinel remains byte-identical.
3. Race creation/replacement of an intermediate reparse point while resolving one-component relative opens; no successful result may expose outside bytes.
4. Race parent directory rename/replacement during resolution and assert no outside directory/file is read or modified.
5. For every denied write, assert the target's original bytes, length, timestamps where stable, and relevant directory entries remain unchanged.
6. Run the exact tests on Windows Server 2025 and record OS build, filesystem type, toolchain, exact candidate SHA, and test output. Passing one Windows build is evidence for that environment, not a universal guarantee.

### Status

- No production Windows code was changed or compiled in this checkpoint.
- The Windows backend is still NOT IMPLEMENTED / NOT ACCEPTED. The six current E4 Windows test names remain required; this section adds race and side-effect expectations needed to make their assertions meaningful.
- Do not advance `CANONICAL_COMMIT.txt` until the six exact Windows tests, the Unix mount-boundary no-truncate test, and all other nonzero-count E4 checks pass on one exact candidate SHA.


## 33. Implementation status correction — 2026-11-18 (PR #113 candidate)

This dated addendum supersedes earlier status statements in sections 31–32 that said no Windows implementation had been added. Those statements remain historical descriptions of their original checkpoints.

At candidate source SHA `5c5bfb53e086222f89dc8178426a8c6b65b50a04`, the Windows implementation now includes:

- Root and intermediate directory handles retained during component traversal.
- Handle-relative component opens using `NtCreateFile`, with reparse-point and volume checks.
- Existing final leaf inspection without following the leaf reparse point.
- A unique sibling temporary file created relative to the retained parent handle.
- Bounded content supplied by the caller, write and flush through the temporary file handle, followed by a handle-relative `SetFileInformationByHandle(FileRenameInfo)` replacement.
- A temporary-file guard that requests deletion on failure.
- A Windows regression test for successful replacement and a denied-replacement test asserting original bytes and no leftover temporary sibling.

Linux continues to use `openat2` for reads and fails closed for writes. Other unsupported platform backends remain fail-closed.

### Verification status for this implementation update

The E4, native Windows reparse probe, full Ubuntu/Windows, SEC-05, distro, and Android artifact results previously recorded apply to source SHA `4c3bccaf59d965ce8b446e0b84cb84c00ff2eed6`. They do **not** transfer to `5c5bfb53e086222f89dc8178426a8c6b65b50a04` or later documentation commits.

The code shape now implements a temporary-file/rename strategy, but the following remain required before acceptance:

1. Compile and run the updated Windows code and all relevant E4 tests on the exact new candidate SHA.
2. Add or demonstrate failure-injection coverage for write/flush failure before rename, and verify original target bytes plus absence of temporary siblings.
3. Validate replacement behavior for a missing target, existing target, read-only target, target reparse point, and concurrent target replacement.
4. Review the rename information structure, information class, parent-handle semantics, and cleanup behavior against supported Windows/filesystem versions; preserve the unresolved published-contract caveat in section 32.
5. Run exact-SHA Linux and Windows regression suites; rerun only the necessary suites affected by the source change.
6. Complete the real Linux mount-boundary no-truncate test and the required race/side-effect checks.
7. Obtain terminal CircleCI evidence on the final exact SHA and complete Android API 35 runtime evidence against the candidate SHA actually being accepted.
8. Keep Issue #104 OPEN and the feature experimental until the entire acceptance set is met.

**Classification:** Windows atomic replacement is now **IMPLEMENTED in the PR candidate**, but it is **NOT VERIFIED on the updated candidate SHA**, not yet SUPPORTED as a cross-platform feature, and not ACCEPTED. No claim is made that atomic rename alone resolves every race or the documented `NtCreateFile` contract limitation.


## 34. Rename implementation correction and exact-SHA revalidation — 2026-11-18

The initial Windows atomic replacement implementation at `5c5bfb53e086222f89dc8178426a8c6b65b50a04` failed its positive write test. The Windows job log for E4 run [#44329034647](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44329034647) showed `filesystem_write_adapter_writes_text_target` returning `BackendFailure` during the Win32 `SetFileInformationByHandle(FileRenameInfo)` operation. The native cause recorded in the PR discussion was `ERROR_INVALID_PARAMETER (87)` for the non-null relative `RootDirectory` argument.

The implementation was changed to call native `NtSetInformationFile(FileRenameInformation)` (information class 10). On exact source SHA `20ede9fb1d3e97c04f81d4fe8f79f642a8cc8e78`, independent E4 run [#44329905887](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44329905887) passed the Windows reparse/handle job, all seven named Windows filesystem tests (including positive atomic write and denied replacement preservation), Unix no-follow/resource-bound tests, and the Windows Server 2025 native `OBJ_DONT_REPARSE` probe.

A subsequent code-bearing change at current candidate SHA `f6acb23a2a0038281c613fb6fd187bc04d2f3dc9` changes the rename structure's `RootDirectory` to NULL and supplies only the target leaf name, describing the operation as a same-directory rename from the source handle. This change is NOT yet verified by the prior PASS run: the prior run checked out `20ede9...`, not `f6acb...`.

The diagnostic verifier branch's temporary `CANONICAL_COMMIT.txt` now pins `f6acb...`; fresh E4, cross-platform, SEC-05, distro, Android artifact, and Android runtime workflows are queued. CircleCI `ci/circleci: one-linux` is PENDING for the latest candidate. Keep the feature NOT VERIFIED at `f6acb...` until those workflows complete and their source pins/logs are checked.

The original Win32 rename failure is retained as historical evidence. Do not remove or weaken the positive write test, the denied replacement preservation test, or the exact-SHA verifier. Issue #104 remains OPEN and NOT ACCEPTED.


## 35. Windows atomic-denial test cleanup correction — 2026-11-18

The first cross-platform run for `f6acb23a2a0038281c613fb6fd187bc04d2f3dc9` reached Windows Clippy and failed on test-only use of `Permissions::set_readonly(false)` (`clippy::permissions_set_readonly_false`). This was not a production backend error.

The test `filesystem_write_adapter_preserves_target_when_atomic_replace_is_denied` now induces replacement denial using a real open destination handle created with Windows `OpenOptionsExt::share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)`, deliberately omitting `FILE_SHARE_DELETE`. It verifies the original bytes and that no temporary sibling remains, then drops the blocker before deleting the test directory. It no longer resets the read-only attribute.

This correction is committed at `a79665056149bf1ff302de79a7b86a346e0353d8`. The independent verifier diagnostic pin now targets that exact source SHA. Fresh E4 run [#44331243475](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44331243475) is QUEUED; CircleCI is PENDING. The earlier E4 PASS on `f6acb...` remains valid for that SHA but does not verify this new test change. Keep the current candidate NOT VERIFIED until the new exact-SHA runs complete.


## 36. Linux direct-child atomic writes and nested-parent fail-closed boundary — 2026-11-18

The Linux backend previously failed closed for all writes. Candidate source `cc54597b471493f4350d0dc0f3a2896babb33ea4` now implements atomic writes for **direct children of the retained trusted root only**:

1. The requested target is normalized relative to the host-injected root.
2. The implementation rejects nested parent paths with `UnsupportedPlatform`. This is deliberate: a nested directory can be renamed outside the root after its directory handle is opened; a later dirfd-relative create/rename would then operate on that moved directory.
3. For direct children, the retained root directory descriptor is the parent authority. The existing leaf is opened with `openat2` and `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`, then checked for regular-file type, same device, and single link.
4. An exclusive temporary sibling is created relative to the root descriptor with `O_CREAT | O_EXCL | O_NOFOLLOW`, written, and `sync_all`ed.
5. `renameat(root_fd, temp_name, root_fd, leaf_name)` atomically replaces the directory entry. It does not follow a raced destination symlink. On failure, the temporary sibling is removed.

Regression coverage added on the Linux adapter includes: create + replace an in-root file (relative and absolute target forms), reject a symlink leaf without modifying its outside target, reject oversized content without side effects, fail closed for nested parent paths, and a real `/dev/shm` mount-boundary read denial. The existing mount-boundary write test still proves no truncation and no side effect for a nested mount target; the raw `openat2` probe checks `RESOLVE_NO_XDEV` behavior.

**Important limitation:** nested Linux writes are NOT IMPLEMENTED / fail-closed. This is a security boundary, not a temporary test workaround. Do not remove it or replace it with path-based fallback. A stronger parent-movement defense and adversarial write race proof are still required before nested writes can be supported.

The independent verifier E4 contract was expanded to require five Linux `descriptor_relative_write_*` tests and three Linux `descriptor_relative_read_rejects_*` tests. The diagnostic verifier pin is updated to source SHA `cc54597b471493f4350d0dc0f3a2896babb33ea4`; fresh exact-source runs are pending. This implementation is NOT VERIFIED until those jobs complete.


## 37. Linux writer root-descriptor compile correction — 2026-11-18

The first Linux direct-child writer candidate `cc54597b471493f4350d0dc0f3a2896babb33ea4` failed the Android AArch64 artifact build at compile time with four `E0425` errors: stale references to `parent_handle` remained in temp creation, cleanup guard setup, and rename calls after the implementation was restricted to the retained root descriptor. The failure occurred before any runtime test; no artifact was uploaded.

The correction replaces those references with the retained `root.as_raw_fd()` value in source SHA `090c84dbab02e48af0a493faeb8d4fe0ff80a876`. The verifier diagnostic pin now targets this SHA. New E4 and cross-platform verification are queued/pending; CircleCI is PENDING. Do not treat the earlier compile failure as a runtime security failure, and do not treat the correction as verified until the new exact-SHA build/tests finish.


## 38. Android artifact build evidence on corrected source — 2026-11-18

The Android AArch64 artifact build now passes on exact source SHA `090c84dbab02e48af0a493faeb8d4fe0ff80a876`: [run #44332808717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808717). The job verified the exact source pin, built and inspected an ELF64 AArch64 executable with Android linker `/system/bin/linker64`, and uploaded artifact ID `14438938784` (SHA-256 `5971ef900ead08e6f571327594625ba2856a1917d59196d095c785bdf51a0ea4`).

This is only artifact/build evidence. Android API 35 runtime execution remains queued and is not VERIFIED. E4 and full cross-platform verification for the current candidate remain queued/pending; CircleCI remains pending. Do not promote Android to SUPPORTED or Issue #104 to ACCEPTED from an ELF build alone.


## 39. Strict Clippy correction and diagram source — 2026-11-18

The Linux direct-child writer candidate `090c84dbab02e48af0a493faeb8d4fe0ff80a876` compiled and passed Android AArch64 artifact build, but Ubuntu distro Clippy failed on a redundant field initializer in the temporary-file cleanup guard (`clippy::redundant_field_names`). The initializer now uses Rust field shorthand in source SHA `3d8e0d0e3af54310837eb06d442d30a02df821d0`.

The Android artifact PASS at `090c84d...` does not transfer to `3d8e0d...`; exact-SHA artifact and platform checks must be rerun. The verifier pin now targets `3d8e0d...`.

A focused Mermaid source/rendered Markdown diagram was added at `docs/architecture/filesystem-root-confinement.md` and linked from `docs/ARCHITECTURE.md` and `docs/README.md`. It maps the host authority, Linux direct-child write boundary, Windows atomic replacement, fail-closed paths, and cleanup behavior to inspected source files. The separate full architecture overview image in PR #97 remains uninspected because the binary cannot be retrieved through the current connector; do not claim that image has been reviewed.


## 40. CLI test alignment for Linux direct-child writes — 2026-11-18

The Debian 12 workspace suite on source `3d8e0d0...` exposed a stale CLI integration test named `run_command_rejects_filesystem_write_when_backend_is_unsupported`. It was guarded by `not(windows)` and therefore incorrectly asserted that Linux direct-child writes remain unsupported.

The test now splits platform expectations:

- Linux absolute direct-child target: the CLI must successfully write under the target's host-injected parent root.
- Linux nested relative target: the CLI must fail closed with `UnsupportedPlatform`, preserve the original target bytes, and leave no extra directory entry.
- Other non-Windows/non-Linux platforms: the unsupported-backend test remains.

The update is source SHA `2fa1436daa102d3cf32125da6c440ae19cc83ba7`. This aligns integration expectations with the deliberately partial Linux writer; it does not expand nested write support. Fresh exact-SHA cross-platform and distro workflows are pending.


## 41. Host-root selection must be independent of requested target — 2026-11-18

The CLI host previously chose the authority root from the requested absolute target's parent. This is not acceptable for project-root confinement: an absolute target could effectively choose its own root. The host now selects the root from source/project context:

1. Canonicalize the source parent.
2. Prefer the nearest ancestor containing `one.toml`.
3. If none is found and the source is beneath the process working directory, use the canonical working directory.
4. Otherwise use the canonical source parent for a standalone source outside the working directory.
5. Construct `FilesystemAuthority` from that root independently of the requested target. The authority then rejects absolute targets outside it.

The CLI applies the same host-root selection to filesystem.read and filesystem.write. Added integration tests verify outside-root absolute write rejection with the outside sentinel unchanged, outside-root read rejection without disclosure, and relative-source root resolution.

Source implementation and tests: `79a98861080fed2ceaa0a7a2f56e07d6fbc19fca`; focused Mermaid diagram updated at `d4687325f4b1236509bfe484cae46b72649d839f`. Verifier pin is updated to the source SHA; exact-SHA workflows are pending.


## 42. Manifest-root selection regression coverage — 2026-11-18

Added `run_command_uses_nearest_one_manifest_as_filesystem_root` to the CLI integration suite. It creates a temporary project with `one.toml`, places the source under `src/main.one`, and verifies a write to a direct child of the project root. This proves the host selects the nearest manifest ancestor rather than the source subdirectory or requested target parent.

Test source SHA: `1b7c3bb1dbc29a33c086b4df21a9996025c1e075`; verifier pin updated to the same exact SHA. Verification pending.


## 43. Ancestor manifest search correction — 2026-11-18

The CLI source-root resolver no longer stops manifest discovery at the current working directory. It now walks all ancestors of the canonical source parent and chooses the nearest `one.toml`, then falls back to the canonical working directory when the source is beneath it, or to the canonical source parent otherwise. This covers source files located directly under a subdirectory while the project manifest is above the working directory.

Correction source SHA: `722d662e7a81711ba3607d26571f4048a292b3ed`; diagnostic verifier pin updated. Exact-SHA verification pending.


## Exact-SHA checkpoint refresh — 18 November 2026

The latest code-bearing candidate remains `722d662e7a81711ba3607d26571f4048a292b3ed`. The current PR #113 HEAD is a documentation-only descendant `1eebed322c696fe4f958578a3c7947abce75e173`; the focused architecture diagram now explicitly shows retained Windows root/parent handles opened without `FILE_SHARE_DELETE` to prevent parent-directory rename races during the handle-relative walk and atomic replacement.

The independent diagnostic verifier pin is confirmed as `722d662...`. Fresh exact-source verification is still incomplete: Linux distro run #44335797294 completed SUCCESS: Debian 12 and Ubuntu 24.04 jobs both passed; E4 #44335797344 is queued; cross-platform #44335797200 is pending; Android artifact #44335797320, Android runtime #44335797240, SEC-05 #44335797365, and resource guard #44335797191 are queued. CircleCI `ci/circleci: one-linux` is PENDING on source `722d662...`: https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94.

Classification remains IMPLEMENTED / NOT VERIFIED for the CLI host-root resolver and Linux direct-child writer; Windows handle-relative atomic replacement has historical E4 evidence only on earlier exact source SHA and needs the current exact-SHA run. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 OPEN / DRAFT / UNMERGED; canonical main and protected verifier pin remain unchanged.


## Exact-SHA verification refresh — 2026-11-18

Code candidate: 722d662e7a81711ba3607d26571f4048a292b3ed. The independent diagnostic verifier pin matches this exact source.

- E4 Unix + Windows filesystem adversarial matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797344.
- Windows Server 2025 build 26100 native probe: ordinary file open with OBJ_DONT_REPARSE succeeded; direct and intermediate junctions were denied with STATUS_REPARSE_POINT_ENCOUNTERED (0xC000050B); outside sentinel remained unchanged.
- Seven Windows filesystem regressions passed, including in-root absolute read, outside-root confinement without side effects, normal atomic write, oversized-content rejection, denied atomic replacement preserving original bytes and cleaning the temp sibling, junction escape rejection, and parent traversal rejection.
- Ubuntu + Windows Server 2025 cross-platform matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797200.
- Debian 12 + Ubuntu 24.04 distro matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797294.
- Android AArch64 artifact build: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797320; artifact ID 14442665831; archive SHA-256 3bc37c20a92087464abb087be1fe174fac314dbddd635116656e34ad1cee4123. Android API 35 ARM64 runtime, run #44335797240, remains IN PROGRESS.
- SEC-05 Linux + Windows: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797365. The targeted interop.rs formatting check passed; the separate whole-workspace formatting diagnostic reports diffs but is configured non-blocking.
- Subprocess resource guard: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797191.

CircleCI one-linux on source SHA 722d662 reports FAILURE: https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94. The prior documentation HEAD b94e85db5d83c9a4bec582609405d9c65e4db841 separately reports PENDING: https://app.circleci.com/workflow/a09cd902-a88f-4884-a212-148ba0c6b80c/job/125775a6-33ba-4bac-b5f5-74249b8e0a19. CircleCI logs are unavailable through the current tools; failure cause is unknown and is not inferred.

Classification: host-root resolver, Linux direct-child writer, Windows adapter regressions, E4, cross-platform, distro, SEC-05, and resource guard are VERIFIED on the exact source SHA and stated environments. Linux nested writes remain fail-closed. Windows native probe evidence is limited to Windows Server 2025 build 26100. Android artifact is BUILD-VERIFIED only; Android runtime is not yet verified. CircleCI remains unresolved. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 remains OPEN / DRAFT / UNMERGED; main and the protected verifier pin remain unchanged.


## CircleCI status correction — 2026-11-18

A fresh combined-status read confirms that CircleCI `ci/circleci: one-linux` is FAILURE on the code-bearing source `722d662e7a81711ba3607d26571f4048a292b3ed` (job https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94). The previously recorded documentation HEAD `b94e85db5d83c9a4bec582609405d9c65e4db841` is PENDING, and its descendant documentation HEAD `add130412cf42088ca9171fa2660295f7c30ae65` is also PENDING (job https://app.circleci.com/workflow/bfe4df8d-85a8-49bc-b7f2-21910a4692cd/job/d2aa7f1d-0c7a-4e7b-8ba4-ecaf595674c2). The failure cause on the code SHA remains UNKNOWN because CircleCI logs are inaccessible through the current tools. This correction does not change the exact-source independent verification results or Issue #104 acceptance classification.


## AUTHORITATIVE STATUS OVERRIDE — 2026-11-19 (protected E4 verification)

This dated addendum supersedes earlier status statements in this design that describe PR #110, the pre-implementation candidate, the old protected verifier pin, or E4 as still queued. Earlier entries remain historical records.

- Current implementation candidate code-bearing SHA: `722d662e7a81711ba3607d26571f4048a292b3ed`. Subsequent PR #113 commits through `67d3c14c70fc43c1eae26c7f21fea8e7a4918b07` are documentation/CI-diagnostic descendants; do not transfer a CircleCI result across exact SHAs.
- PR #44 was explicitly authorized and merged into protected verifier branch `verification`; merge commit `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`; protected `CANONICAL_COMMIT.txt` now pins `722d662...`.
- Protected post-merge E4 run #44519577693 is terminal SUCCESS: Unix no-follow/resource bounds, Windows reparse/handle checks, and Windows native `NtCreateFile + OBJ_DONT_REPARSE` probe all passed. Native probe scope remains Windows Server 2025 build 26100 only.
- Linux nested writes remain deliberately fail-closed. macOS and other unsupported platforms remain fail-closed. Android runtime evidence is API 35 ARM64 only.
- Same-push SEC-05 Linux failed at the native cross-compiler installation step while its Windows job passed; this is separate from E4. Its failing command/log has not yet been retrieved, so root cause is unclassified. Other E1/stdlib/provenance/health/artifact workflows also failed separately; do not describe the whole verifier push as green.
- PR #113 remains OPEN / DRAFT / UNMERGED. Issue #104 remains OPEN / NOT ACCEPTED pending remaining acceptance/ledger gates. CircleCI on latest recorded PR HEAD `67d3c14...` was PENDING; verify the exact current SHA before any CI claim.


## Live exact-SHA refresh — 20 November 2026

- ONE PR #113 is OPEN / DRAFT / UNMERGED at exact HEAD `b0d854124191587882705c832e1825ad2a46cc8f`; base remains `181ad350d8f0db53cda654c4d378425fbdd6ab24`. ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI `ci/circleci: one-linux` is terminal SUCCESS on this exact PR HEAD: workflow `ecd5d877-e5f5-429c-884c-426196fc7e78`, job `cc77e7cb-8daf-4d9c-845d-df991e0f68bf`. This updates the prior PENDING observation for the same SHA. Only the combined-status/job result is verified here; no provider step-log or test-count claim is made.
- The last code-bearing candidate remains `722d662e7a81711ba3607d26571f4048a292b3ed`. GitHub's net tree comparison from that source pin to PR #113 HEAD shows no changed source or test paths; the net changes are CI configuration, diagnostics tooling, and documentation. Keep E4 and other independent results attributed to the exact source pin `722d662...`, not relabeled as runs on `b0d854124191587882705c832e1825ad2a46cc8f`.
- Protected verifier branch `verification` is `b14abf0020472dc45842be30854e0a09317d6cb4`; `CANONICAL_COMMIT.txt` remains pinned to `722d662...`, and `LEDGER_SOURCE_COMMIT.txt` remains `d6c382a245c49a028bc60809f22ca956c7f894cc`.
- Protected E4 run [#44519577693](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44519577693) is SUCCESS for its pinned source. The historical fixed-weight gate [#44519577312](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44519577312) remains FAILURE; it is not overwritten by E4 PASS.
- Verifier PR #50 remains OPEN / DRAFT at `0c0d8563d28e1f70734ecf4d906be91f912472f3`, with no status contexts/check runs reported. PR #49 remains OPEN / DRAFT for Android runtime metadata. ONE PR #103 remains OPEN / DRAFT at `5f616ae08e303081666a5e0d6a4b8b63b9d69c16`; its CircleCI status is a separate config-fetch error and does not transfer to PR #113.
- Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes and unsupported platform adapters remain fail-closed. No main update, merge, protected pin change, ledger promotion, or release action occurred.


## Exact-SHA continuation update — 20 November 2026

- Latest CircleCI-confirmed PR #113 HEAD at this checkpoint: `051c7815cc5cf3dbca840ac8cbb38e3e663da6b7`; `ci/circleci: one-linux` terminal SUCCESS, workflow `3b89115a-ff07-41ac-940e-fb78baccd7ac`, job `241f1d78-0aaf-4f47-8ea2-57fbd3b35a1c`. This is job-level evidence; provider step logs were not separately inspected.
- PR #113 remains OPEN / DRAFT / UNMERGED; ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`. This successful candidate result does not mean CircleCI has been integrated into `main`.
- Android API 35 AArch64 runtime metadata evidence is now complete: verifier run #44565956781 SUCCESS on PR #49 exact head `fa0b5cffe52bf31fedef81ed945d6cc3a6dc2e19`; build job `135707610069` and runtime job `135708475823` SUCCESS. Inspected artifact #14553867982 records source pin `722d662e7a81711ba3607d26571f4048a292b3ed`, API 35, ABI `arm64-v8a`, host `aarch64`, exact ADB invocation, exit 0 and `ONE_NATIVE_BACKEND_OK`. ELF SHA-256 matched the metadata. ONE Issue #82 is CLOSED / COMPLETED for this runtime-evidence scope only.
- Protected verifier branch `verification` remains `b14abf0020472dc45842be30854e0a09317d6cb4`; canonical source pin remains `722d662...`; protected ledger source pin remains `d6c382a245c49a028bc60809f22ca956c7f894cc`. No protected pin or ledger weight changed.
- The fixed-weight gate #44519577312 remains historical FAILURE. Issue #104 remains OPEN / NOT ACCEPTED; Linux nested writes and unsupported platform filesystem adapters remain fail-closed. Issue #105 and the broader E1/E2/E4/E5/E6 release work remain unresolved.
- This documentation synchronization itself creates a new PR #113 HEAD. Do not transfer the SUCCESS above to the new SHA; inspect the exact resulting commit status before any subsequent CircleCI claim.


## CircleCI exact-HEAD terminal confirmation — 20 November 2026

- Current PR #113 HEAD at this checkpoint: `26a34efb6d0908bd02738a7b05090f96051a6dc7`.
- GitHub combined status: `ci/circleci: one-linux` = terminal SUCCESS.
- Workflow: `a7016ead-ed2d-467f-9f1b-4d20c60a966f`; job: `6cc9778c-307c-48fc-9111-39c97d288393`.
- This is a direct exact-SHA status read for `26a34efb6d0908bd02738a7b05090f96051a6dc7`; it is not transferred from parent commits. Provider step logs were not inspected, so no per-step or test-count claim is made.
- PR #113 remains OPEN / DRAFT / UNMERGED; ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`. The CircleCI candidate has not been integrated into `main`.
- Issue #82 is CLOSED / COMPLETED for Android API 35 AArch64 runtime evidence; artifact #14553867982 was downloaded and its metadata/ELF digest inspected. This does not establish Android filesystem adapter support.
- Issue #104 remains OPEN / NOT ACCEPTED; fixed-weight gate #44519577312 remains historical FAILURE. Linux nested writes and unsupported platform filesystem adapters remain fail-closed.
- This documentation-only synchronization creates a new PR #113 HEAD; check that resulting SHA independently before transferring any CI status.


## Platform metadata and host selection — 2026-11-20

The filesystem adapters no longer advertise `PlatformKind::Any`. Their metadata is compiled as Linux-only on Linux, Windows-only on Windows, and empty on Android/macOS/other unsupported targets. The interpreter now supplies `PlatformKind::current_host()` to orchestration. This aligns adapter selection with the implemented backend and ensures unsupported filesystem platforms are rejected before invocation.

Candidate source `c7f460a4e38be61de635ad5af9f7ff987a941f01`: E4 run #44585270928 passed the metadata regression on Linux and Windows; Linux distro run #44585282690 passed the corrected filesystem CLI integration tests on Ubuntu 24.04 and Debian 12. CircleCI on this source remains PENDING; Windows full-workspace and remaining aggregate verification were still in progress at this checkpoint. Issue #104 remains OPEN / NOT ACCEPTED.

## LIVE EXACT-SHA CHECKPOINT — 2026-11-20

This checkpoint supersedes older live-status statements above where they conflict. Historical run outcomes remain preserved.

### Exact source and repository state

- ONE PR #113 remains OPEN / DRAFT / UNMERGED on `security/issue-104-root-confinement-20261118`.
- Exact source HEAD: `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`; it was not modified.
- Independent verifier PR #51 remains OPEN / DRAFT / UNMERGED; current verifier HEAD: `27207b85ed205d2cf12e6fd66b918193a215d8fd`.
- Verifier PR #51 branch-local `CANONICAL_COMMIT.txt` is `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. Protected `verification` branch and its canonical source pin `722d662e7a81711ba3607d26571f4048a292b3ed` remain unchanged.

### Exact-source verification

- CircleCI `ci/circleci: one-linux`: SUCCESS on source `fb5c8bc...`; workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`. This is status-context evidence; provider step logs were not inspected.
- E4 adversarial filesystem run [#44651796986](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796986): SUCCESS on verifier HEAD `27207b85...`, pinned source `fb5c8bc...`. Windows job #136008224371 explicitly passed the seven named E4 Windows filesystem regressions, the atomic-replacement failure-preservation regression, the hard-link regression, and the platform-metadata regression (9 selected tests, 0 failed). Unix job #136008224711 passed the no-follow, mount-boundary, traversal, race, direct-child write, hard-link, platform metadata, policy, and related resource-bound selected tests.
- Linux distro run [#44651796952](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796952): SUCCESS on Ubuntu 24.04 and Debian 12, including contract checks, build, Clippy, workspace tests, and real-network tests.
- Windows native backend runtime [#44651796852](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796852): SUCCESS.
- E1 Windows acceptance [#44651797000](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651797000): SUCCESS.
- Focused standard-library/database [#44651796941](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796941): SUCCESS.
- SEC-05 subprocess timeout/output-limit [#44651796634](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796634): SUCCESS.
- E4 subprocess resource guard [#44651796650](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796650): SUCCESS.
- Artifact integrity [#44651796712](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796712): SUCCESS. Linux artifact #14563276002 SHA-256 `59aa04887ffa5c2cb66d65e4ef7936622c6e0a4f6164ea8ea458457d10b8a940`; Windows artifact #14561664789 SHA-256 `7d3c51123f73c398032ef0c636c018c12ea501340b822b8e3b4edc9777645a3a`. Embedded manifests both match source SHA `fb5c8bc...`; ELF64 x86-64 and PE32+ x86-64 checks passed.
- Android AArch64 build artifact [#44651796635](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796635): SUCCESS. Android API 35 ARM64 runtime [#44651796976](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796976) remained IN_PROGRESS at this checkpoint.
- Cross-platform aggregate [#44651796654](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796654) remained PENDING; E1 Milestone 1 aggregate [#44651796950](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796950) remained IN_PROGRESS. Do not infer completion from individual platform jobs.

### Artifact-integrity historical failure reconciliation

- Earlier artifact-integrity run [#44589012615](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44589012615) FAILED while waiting for a successful cross-platform bundle run for source `fb5c8bc...`; its log did not identify archive corruption.
- Later exact-source run #44651796712 successfully discovered both artifacts, checked GitHub SHA-256 digests, verified embedded source manifests and toolchain metadata, and validated ELF/PE executable formats. Preserve the old failure as historical evidence; the later success is a separate terminal verification, not a deletion or rewrite of that failure.

### Acceptance boundary

- The hard-link test closes a previously missing regression-evidence gap: an in-root file with an outside hard link is rejected for both read and write, and both names retain their original bytes.
- Linux mount-boundary, symlink, traversal, race, and direct-child write controls are exercised by the exact-source Unix E4 run. Linux nested writes remain fail-closed.
- Windows junction/reparse-point and handle-relative checks are exercised by the exact-source Windows E4 run and native backend runtime.
- Filesystem adapter metadata remains Linux-only on Linux and Windows-only on Windows; Android/macOS filesystem adapters remain unavailable. Android native runtime evidence does not imply Android filesystem adapter support.
- Issue #104 remains OPEN / NOT ACCEPTED. Do not promote protected pins, ledger weights, or DoD items; do not merge PR #113 or PR #51; do not change `main` or protected `verification`.


## Exact-SHA Windows parent-rename regression — 2026-11-20

- New code-bearing test SHA: `c4f3800bc12b70a4d59d72fbc5a80029cb053cea`; the only source change relative to the prior candidate is a Windows-only test in `crates/one-core/src/filesystem.rs`, not production behavior.
- Verifier PR #51 exact HEAD: `7c3ded0d811fe71bd14034880b24b0f8e396349e`; branch-local canonical source pin: `c4f3800...`. Protected verifier branch and canonical pin remain unchanged.
- New test: `filesystem::windows_native::tests::retained_parent_handle_blocks_directory_rename`. It opens a nested parent with the same `FILE_SHARE_READ | FILE_SHARE_WRITE` share flags used by `write_relative` (no `FILE_SHARE_DELETE`), proves rename fails while the handle is retained, then proves rename succeeds after handle close.
- E4 push run #44760172674 is terminal SUCCESS on exact source `c4f3800...` / verifier `7c3ded0...`: Unix job #136370280796 SUCCESS; Windows reparse/handle job #136370280317 SUCCESS; Windows native probe job #136370280733 SUCCESS. Windows log shows the new test executed exactly once and passed (1 passed, 0 failed).
- Duplicate E4 pull-request run #44760180858 is also terminal SUCCESS on the same exact source/verifier pair; Unix job #136370312121, Windows reparse/handle job #136370312012, and native probe job #136370311602 all SUCCESS. Count the duplicate event as confirmation, not a second acceptance point.
- Other exact-verifier-SHA PR workflows: SUCCESS — E4 resource guard #44760181208, focused stdlib/database #44760180912, Android AArch64 artifact build #44760180867, Linux distro #44760180826, E1 Windows #44760181124. IN_PROGRESS — Windows native runtime #44760181204, candidate artifact integrity #44760180923, SEC-05 #44760180951, E1 Milestone 1 #44760180919 (Ubuntu job #136370312623 SUCCESS; Windows job #136370313083 IN_PROGRESS), cross-platform #44760181947, Android API35 runtime #44760180953. Do not call the complete verifier matrix green yet.
- ONE source PR #113 GitHub Actions run #44760138732 is IN_PROGRESS on exact source `c4f3800...`: Ubuntu job #136370164396 SUCCESS; Windows job #136370165192 IN_PROGRESS. CircleCI `ci/circleci: one-linux` is PENDING on this exact SHA, workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Pipeline UUID/number, `vcs.revision`, and job→workflow→pipeline membership remain unverified.
- B-001 remains PARTIAL / NOT CLOSED pending CircleCI pipeline mapping/provider logs. Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes fail-closed; Android/macOS filesystem adapters unavailable. This test verifies one Windows parent-handle exclusion primitive, not complete E4/Issue #104 acceptance.
- No production runtime code, CI configuration, main, protected verifier branch/pin, ledger/DoD, acceptance criteria, merge, or release changed.


## Exact-SHA verification refresh — 2026-11-20 15:14Z

### Candidate identity
- Code-bearing source: `c4f3800bc12b70a4d59d72fbc5a80029cb053cea` (only source delta from `fb5c8bc...` is the Windows-only parent-handle rename regression test).
- Verifier PR #51 HEAD: `7c3ded0d811fe71bd14034880b24b0f8e396349e`; branch-local source pin: `c4f3800...`.
- ONE PR #113 current docs-only HEAD before this sync: `043390cf0673247524ffb44c444cbbe4745be91f`. This sync changes existing docs only and uses `[skip ci]`; no CI result transfers to the resulting docs HEAD.

### Terminal results on exact source/verifier pair
- ONE GitHub Actions run #44760138732 on source `c4f3800...`: Ubuntu job #136370164396 SUCCESS; Windows Server 2025 job #136370165192 SUCCESS. Windows log explicitly ran and passed `interop::tests::subprocess_execution_is_bounded_by_a_deadline`, `subprocess_stdout_capture_has_a_hard_limit`, `subprocess_stderr_capture_has_a_hard_limit`, and `windows_job_object_terminates_descendants_when_parent_exits`.
- Verifier PR matrix on verifier SHA `7c3ded0...` / source pin `c4f3800...`: 11 of 12 named workflows are terminal SUCCESS:
  - E4 adversarial #44760180858: Unix #136370312121, Windows #136370312012, native probe #136370311602 — SUCCESS. New `retained_parent_handle_blocks_directory_rename` test executed once and passed.
  - E4 subprocess resource guard #44760181208 / job #136370313217 — SUCCESS.
  - Focused stdlib/database #44760180912 / job #136370312551 — SUCCESS.
  - Android AArch64 artifact build #44760180867 / job #136370311618 — SUCCESS.
  - Linux distro #44760180826: Debian #136370312102 and Ubuntu #136370312874 — SUCCESS.
  - Windows native runtime #44760181204 / job #136370312887 — SUCCESS; Windows Server 2025, Rust 1.100.0, runtime exit 0, output `Budi`.
  - SEC-05 #44760180951: Linux #136370311990 and Windows #136370312357 — SUCCESS. Source PR Windows GHA also proves the dynamic deadline/stdout/stderr/Job Object tests executed and passed.
  - E1 Windows #44760181124 / job #136370313677 — SUCCESS.
  - E1 Milestone 1 #44760180919: Ubuntu #136370312623 and Windows #136370313083 — SUCCESS.
  - Cross-platform #44760181947: Windows #136370329101 and Ubuntu #136370329521 — SUCCESS.
  - Candidate artifact integrity #44760180923 / job #136370311819 — SUCCESS.
- E4 push event #44760172674 is also SUCCESS on the same exact source/verifier pair (Unix #136370280796, Windows #136370280317, native probe #136370280733); it is a duplicate trigger and is not a second acceptance point.

### Artifact provenance and remaining runtime
- Artifact integrity selected cross-platform run #44760181947 at verifier SHA `7c3ded0...` for exact source `c4f3800...`.
- Linux artifact #14640868829, SHA-256 `7e8e443e2ae7c736f18243043f355840801afebdddca46a5efe41c159bac5732`; metadata target `x86_64-unknown-linux-gnu`, Rust/Cargo 1.100.0, ONE CLI 0.1.0, ELF64 little-endian EM_X86_64 PASS.
- Windows artifact #14641018247, SHA-256 `f68c7325ff39ae33b2cfadf0b896896c0ba4c3eeb581b033f8e1a75974900846`; metadata target `x86_64-pc-windows-msvc`, Rust/Cargo 1.100.0, ONE CLI 0.1.0, PE32+ x86-64 PASS.
- The only verifier workflow not terminal is Android API 35 ARM64 runtime #44760180953. ELF build job #136370311867 is SUCCESS; runtime job #136371143664 is still executing the “Boot Android API 35 AArch64 on ARM64 host” step. Workflow timeout is 150 minutes; no timeout/failure is claimed.
- Do not claim Android filesystem adapter support from this native-backend runtime. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

### CircleCI and release gate
- CircleCI `ci/circleci: one-linux` on source `c4f3800...` remains PENDING; workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Pipeline ID/`vcs.revision` mapping and provider logs remain unavailable. B-001 remains PARTIAL / NOT CLOSED.
- Protected verifier branch/pin, protected ledger pin `d6c382a...`, fixed weights, DoD, main, acceptance criteria, merge, and release state remain unchanged. Historical fixed-weight gate #44519577312 remains FAILURE.

## Android syscall-availability boundary — 2026-11-20

Do not enable the Linux `openat2` backend for Android by changing `cfg(target_os = "linux")` to include Android. A recent native Termux report on Android 16 documents Android seccomp delivering `SIGSYS` for `openat2`, which terminates the process instead of returning `ENOSYS`/`EPERM` for a safe fallback ([upstream report](https://github.com/oven-sh/bun/issues/39060)). This is a specific environment report, not a claim that every Android version/device blocks the syscall.

Android filesystem support therefore remains fail-closed until a separate resolver is designed around primitives that preserve the required root, symlink, mount-identity, and race guarantees, and then exercised on actual Android/Termux targets. An errno-only fallback is insufficient if the syscall can terminate the process.

## Rejected Linux nested-write workaround: Landlock-only worker

Do not treat a Landlock ruleset in a helper process as sufficient proof for nested writes. The Linux `landlock(7)` contract states that file descriptors opened before sandboxing are not subject to the restrictions, and `LANDLOCK_ACCESS_FS_WRITE_FILE` is checked when opening a file, not on subsequent `write(2)` calls. A parent directory can therefore be moved after a temporary file is opened; the already-open descriptor may still be writable. See the [Linux Landlock manual](https://man7.org/linux/man-pages/man7/Landlock.7.html).

Until a different kernel-enforced primitive or threat-model change is proven on real Linux and Windows runners, keep nested Linux writes fail-closed. Do not replace this limitation with a Landlock-only helper, post-open ancestry check, advisory lock, or userspace watcher.

## Candidate for safe Linux nested writes — NOT APPROVED FOR PRODUCTION

Research identified a possible kernel-enforced path-resolution approach that must be proven before production changes:

1. Clone the retained root mount as a detached mount tree with `open_tree(root_fd, "", AT_EMPTY_PATH | OPEN_TREE_CLONE | AT_RECURSIVE | OPEN_TREE_CLOEXEC)`.
2. Apply `MOUNT_ATTR_NOSYMFOLLOW` recursively with `mount_setattr(mount_fd, "", AT_EMPTY_PATH | AT_RECURSIVE, ...)`.
3. Use the detached mount fd as the `dirfd` for `*at()` operations (the `open_tree(2)` contract explicitly permits this). Create the temporary file at the clone root, then commit with a full relative destination path rooted at the same mount fd. Do not retain a nested parent fd for the final rename.
4. If a parent is moved out, the destination path should no longer resolve from the cloned root; if an intermediate symlink is substituted, the mount's no-symlink attribute should reject traversal; cross-mount rename should fail with `EXDEV`.

Hard limitation: `OPEN_TREE_CLONE` requires `CAP_SYS_ADMIN`; unsupported kernels/hosts must remain fail-closed. References: [open_tree(2)](https://man7.org/linux/man-pages/man2/open_tree.2.html), [mount_setattr(2)](https://man7.org/linux/man-pages/man2/mount_setattr.2.html).

Status: design hypothesis only. No production code changed for this approach. Before adoption, a dedicated adversarial prototype must demonstrate parent move-out, symlink substitution, mount-boundary, temp cleanup, and outside-sentinel preservation on the actual Linux runner; Windows behavior must remain green. Do not claim this solves the gap until those results exist.

## Android filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `7dd7b0049a7f5d71ac8a3d59570c2975fed835ec` (NOT BUILT OR TESTED).
- Android now has a native `FilesystemAuthority` root open and a handle-relative direct-child read/atomic-replace backend. Root and leaf are opened without following final symlinks; `statx(STATX_MNT_ID)` is required to reject mount crossings; reads reject non-regular and hard-linked files; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The backend deliberately avoids `openat2` because Android seccomp may terminate the process with `SIGSYS` for that syscall. It requires kernel `statx` mount-ID support; if unavailable it fails closed.
- Scope is intentionally limited to direct children of the trusted root. Nested Android paths return `UnsupportedPlatform`; no claim of full Android filesystem support or Issue #104 acceptance is made.
- Three Android-targeted regression tests cover direct-child read/write, leaf-symlink and nested-path denial, and final-component root-symlink rejection. They have NOT been run. Actual Termux/device execution is still required.
- Linux nested writes remain fail-closed; macOS filesystem adapter remains unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

## macOS filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `4be1b8fd579092d8195c3fb775b7c765f6221814` (NOT BUILT OR TESTED).
- macOS now has a native retained-root open and direct-child read/atomic-replace backend. Root and leaf operations use directory/file descriptors with `O_NOFOLLOW`; file and root device identity are compared; hard-linked and non-regular targets are rejected; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The macOS adapter is deliberately limited to direct children of the trusted root. Nested paths return `UnsupportedPlatform`. This implementation has no target-host execution evidence yet; its mount/device and race behavior still require platform-specific review.
- Three macOS-only regression tests cover direct-child read/write, symlink/nested-path denial, and final-component root-symlink rejection. They have NOT been run.
- Android direct-child backend remains statx-mount-ID dependent; Linux nested writes remain fail-closed. Issue #104 remains OPEN / NOT ACCEPTED.


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


## Android/macOS filesystem helper repair and nested-path security boundary — 2026-11-21

Latest code-bearing source: `5f98db9f70d28a0d5a206a3e6e37f986efc3574b` (`crates/one-core/src/filesystem.rs`).

- Removed the duplicate Android `create_temp` definition and supplied the missing macOS `create_temp` helper. These were source-visible compile blockers; no build was run.
- Android/macOS direct-child read/write backends remain. Their root-relative direct-child operations retain no-follow checks, regular-file/single-link checks, Android `statx(STATX_MNT_ID)` or macOS `fstatfs` identity checks, and same-directory atomic replacement.
- Nested Android/macOS paths remain fail-closed. A proposed generic openat parent walk was deliberately not enabled: ordinary POSIX directory handles do not pin an opened parent beneath the retained root, so an attacker may rename that directory outside the root before the leaf operation. The current resolver rejects multi-component paths with `UnsupportedPlatform` until an anti-move primitive is implemented.
- Android/macOS regression fixtures now cover nested-path fail-closed behavior, leaf symlinks, and an outside sentinel. These tests are NOT RUN.
- This is source implementation hardening and a precise capability boundary, not acceptance. Issue #104 remains OPEN / NOT ACCEPTED. No build/test/CI/verifier/platform matrix was run and no status was promoted.


## 2026-11-21 — Android/macOS filesystem source repair and confinement boundary

Latest code-bearing source: `0f5859426273a61174d6d1b08ed8771a6f4b136a` (`crates/one-core/src/filesystem.rs`).

- Android's duplicate `create_temp` definition has been removed; macOS now has its missing exclusive `create_temp` helper. Static source inventory shows one helper definition in each native module. No compiler was run.
- Android/macOS direct-child operations retain root-handle-relative no-follow reads and same-directory exclusive-temp atomic replacement; Android requires `statx(STATX_MNT_ID)`, while macOS compares `f_fsid` and mount point with `fstatfs`. Regular-file and hard-link checks remain.
- Nested paths on both POSIX backends explicitly return `UnsupportedPlatform`. Ordinary POSIX directory descriptors do not prevent a parent directory from being renamed outside the retained root, so a multi-component openat walk would not satisfy the root-confinement race contract. No such unsafe nested support is enabled.
- Android/macOS source regressions cover direct-child operation, leaf symlink rejection, nested-path fail-closed behavior, root symlink rejection, and hard-link side-effect preservation. These tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED. This source correction is not a build/test/verifier result; no platform support or release approval is promoted.


## 2026-11-21 — Android nested-read implementation with kernel-enforced confinement

Latest source candidate: `52fcfb2706fcb48e9599d75f9086578279f1bbac` (`crates/one-core/src/filesystem.rs`, NOT BUILT/TESTED).

- Static source inventory at this candidate finds exactly one `create_temp` definition in `android_native` and exactly one in `macos_native`. The earlier duplicate Android helper was already removed; macOS's missing helper was added. This is source inspection only, not Android/macOS compile evidence.
- Android nested reads now use Linux/Android `openat2` with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`. The kernel performs the complete path resolution relative to the retained root descriptor and rejects symlink, mount-crossing, escape, and unsafe rename-race outcomes. The opened leaf is still checked for regular-file type, single hard-link, device identity, and `STATX_MNT_ID` equality.
- Android nested writes remain fail-closed (`UnsupportedPlatform`) until an anti-move atomic replacement backend is implemented. Direct-child atomic writes remain unchanged.
- macOS nested reads and writes remain fail-closed. Ordinary `openat` directory-descriptor walks cannot prevent a parent directory from being renamed outside the root; no unsafe traversal has been enabled. Direct-child operations remain unchanged.
- Added Android source regression for a valid nested read and a symlinked-parent escape with outside-sentinel preservation. Existing Android expectations now distinguish missing nested files (`NotFound`) from symlink-parent denial (`ResolutionFailed`). Tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED. No build, test, CI, verifier, or platform matrix ran; no platform support or release approval is promoted.
