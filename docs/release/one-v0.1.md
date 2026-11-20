# One v0.1 Release Contract

## Integrated P0 candidate — release gate unchanged — 2026-11-17

PR #108 now integrates the four independently passing fixes from PR #98–#101 in source and regression tests. Integration source commit: `e8cd41ec212c3224bdbecf85eaf18c5850096e16`; normative specification updates follow in later commits. The combined candidate has no exact-SHA execution evidence yet and is NOT RELEASE-ACCEPTED. No E1–E6 acceptance, DoD checkbox, release readiness, merge authorization, or distribution claim is promoted. Issue #104 filesystem root confinement, Issue #105 escaped Unix descendant containment, and Issue #82 real Android AArch64 execution remain blockers.


Status: **EXACT-SHA CROSS-PLATFORM VERIFIED BASELINE WITH EXPERIMENTAL ORCHESTRATION; RELEASE APPROVAL REMAINS A SEPARATE GATE**.

## Current forensic release gate — 2026-11-17

This checkpoint supersedes earlier historical candidate/head statements below where they conflict with live PR metadata. It does not authorize merge or release.

- Canonical `main`: `d35db95153f6ddd79885494c72f5aee16c899751`; repository visibility is PUBLIC. `main` still contains GitHub Actions workflows and has no `.circleci/config.yml`.
- PR #98–#101 each have exact-SHA GitHub Actions Ubuntu + Windows Server 2025 PASS on their own candidate SHAs: #98 `a83321bc41b3d0d28378841e29a6169abed8ac15` run #2018; #99 `8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9` run #2019; #100 `6375356c9c275ace5fd35cf4a81799a9546e1e1d` run #2020; #101 `cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f` run #2138. These fixes remain unmerged and their results do not combine into a single integrated source revision.
- PR #102 source `6c72d8e548bbbc28d85be39b8de427614989a038`: GitHub Actions run #2229 / `44086429416` PASS on Ubuntu and Windows Server 2025.
- PR #107 source `21e393157e52728d9510699ac6c9120987e3852c`: GitHub Actions run #2230 / `44088328030` PASS on Ubuntu and Windows Server 2025. It adds bounded Unix reader shutdown; it does not terminate escaped Unix descendants in production.
- PR #103 ledger source `041b359b8504551c2c1705093050d9c467368bac`: GitHub Actions run #2231 / `44089449162` PASS on Ubuntu and Windows Server 2025.
- PR #108 is the CircleCI migration candidate, based on PR #102 and carrying PR #107's verified bounded-reader implementation. It adds CircleCI config, updates contract checking/portable offline scripts for `--locked`, and removes the three repository GitHub Actions workflows on the candidate branch only. Its CircleCI schema/job execution has NOT been verified; no CircleCI status context is currently available.
- CircleCI contexts #669/#671/#673 are errors on branches without `.circleci/config.yml`; they are not source-test failures. Pipeline #643 on old PR #106 explicitly reported missing config. Do not generalize that exact error to other pipelines without their logs.
- Open security gates: Issue #104 (race-resistant filesystem root confinement), Issue #105 (strong Unix escaped-descendant containment), and Issue #82 (Android AArch64 runtime execution). Issue #81 remains open pending explicit reconciliation with the newer subprocess work.
- The Definition of Done remains 3 checked / 10 unchecked / 13 total. E6 remains **NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED**. No 100% completion, merge, or release claim is made.


## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

## Verified source revision and exact-SHA evidence boundary

The last code-bearing canonical source revision `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` passed independent exact-SHA verification run [`39554661451`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451) on Ubuntu and Windows Server 2025.

The matrix passed contract-artifact checks, build, AST contract regression tests, Clippy, workspace tests, real-network runtime tests, and platform-appropriate offline verification. It also provisioned the Windows/Android cross-toolchains and Linux cross-compilers required by the native CLI integration coverage.

Canonical hosted `One CI` runs `39372356906` and `39381125667` failed before executable steps (`steps: []`). These are infrastructure/runner failures and do not establish a source failure; the independent exact-SHA matrix is the positive source-revision evidence.

The matrix verifies the recorded source revision on its Ubuntu and Windows hosts. Subsequent evidence-sync commits change documentation only, not source or tests. It does not automatically imply that every cross-compiled target artifact was executed on its own target architecture; target-specific execution claims remain bounded to their recorded evidence.

## Verified baseline

- DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT core language constructs.
- Indonesian lexical and grammar contracts.
- Language-independent AST and Semantic IR.
- Reference interpreter and structured diagnostics.
- Capability-aware process boundary and cross-platform process API shape.
- CLI commands `new`, `check`, `run`, `test`, `build`, `package`, `version`, and `ast`, with Indonesian conversational primary aliases.
- Deterministic `.oneir` and `.onepkg` textual artifacts.
- Strict v0.1 project manifest validation.
- Conformance fixtures and golden-output tests.
- Verified native network runtime slices: DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit runtime limits.
- General-purpose orchestration runtime foundation: typed capability requests, deterministic governance, governed adapters, normalized results/evidence, audit records, localized semantic-resolution fixtures, and `network.dns.resolve`.
- Experimental source-level DNS orchestration through `cari alamat`.
- Experimental source-level filesystem-read orchestration through `baca berkas`.
- Experimental source-level process-execution orchestration through `jalankan proses`, limited to one explicitly targeted program without arguments and backed by the recorded cross-platform evidence.
- Experimental source-level filesystem-write orchestration through `tulis ke berkas`, limited to one text payload and one explicitly targeted file, with `filesystem.write` permission; source/core/CLI tests passed in the exact-SHA Ubuntu/Windows matrix `39554661451` for `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`, alongside historical Termux evidence.
- Experimental source-level database-query and HTTP-GET slices with historical Termux evidence and source/core/CLI tests passing in the exact-SHA Ubuntu/Windows matrix `39554661451` for `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`.

## Native backend development boundary — PR #64

The native backend contract is:

```text
One Source
→ Parser
→ Analyzer
→ One Semantic IR
→ Target Backend
→ Toolchain
→ Native Executable
```

Semantic IR remains target-independent.

### Linux AArch64

Target: `aarch64-unknown-linux-gnu`

Status on implementation SHA: **VERIFIED — AArch64/Linux host boundary**. A native ELF AArch64 artifact was built and executed successfully.

### Linux x86_64

Target: `x86_64-unknown-linux-gnu`

Status on implementation SHA: **BUILD TERBUKTI — execution pending**. The backend produces a real ELF x86-64 artifact using the verified cross-linker selection in the recorded environment. No x86_64 executor was available.

### Windows x86_64

Target: `x86_64-pc-windows-gnu`

Status on implementation SHA: **BUILD TERBUKTI — execution pending**. A PE32+ x86-64 artifact is proven. No Windows/Wine executor was available for the recorded environment.

The last code-bearing source revision has passed the exact-SHA build/test matrix. Individual native target execution claims remain bounded to the exact artifact and target executor recorded in the native-backend verification evidence.

## Explicit v0.1 limits

- The existing `build` command emits a deterministic `.oneir` IR/debug artifact; the native backend slice does not replace it with a general-purpose native compiler claim.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- The language-level orchestration catalog is not complete.
- Filesystem read/write remains experimental and limited to one explicitly scoped target and one text payload. The current Issue #104 candidate exposes filesystem adapters on Linux, Windows, Android, and macOS: Linux supports direct-child operations plus a detached-mount nested-write path when required kernel/filesystem primitives are available (otherwise it fails closed); Windows uses its native handle-relative backend; Android and macOS currently support direct-child operations while nested paths remain fail-closed because POSIX directory handles do not prevent an opened parent directory from being renamed outside the retained root. These source capabilities are not platform acceptance or a `SUPPORTED` claim until the fixed Issue #104 adversarial acceptance evidence exists. Historical Termux evidence applies only to its recorded older source revision and must not be used to claim current Android acceptance.
- Process execution is experimental only within the explicitly targeted single-program/no-arguments boundary.
- Database query remains a read-only parameterized SQLite SELECT slice; its tested source-level path has exact-SHA Ubuntu/Windows evidence at `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` and historical Termux evidence.
- HTTP GET remains limited to explicit `http://` HTTP/1.1 GET, exact target scope, and response-body normalization; its tested source-level path has exact-SHA Ubuntu/Windows evidence at `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` and historical Termux evidence.
- Native Linux x86_64 and Windows x86_64 execution are not yet verified for the merged `main` revision.
- Native C/C++ FFI, Python/Node/JVM native bindings, package registry, IDE/editor, GUI framework integration, additional locales, and additional protocol/capability inventory beyond the verified boundary remain future work.

## Release labels

- **IMPLEMENTATION-COMPLETE:** in-scope source/docs/tests are present.
- **VERIFIED:** exact source revision has executable evidence for the stated platform boundary.
- **EXPERIMENTAL:** the declared vertical slice is verified but intentionally limited in coverage or platform boundary.
- **BUILD TERBUKTI:** a native artifact was produced and structurally validated, but target execution evidence is missing.
- **SUPPORTED:** reserved for a stronger platform/support claim.

## Verification infrastructure rule

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution. **Do not purchase an upgrade** merely to obtain hosted runner execution; use the independent verification path when the canonical hosted runner is unavailable.

## Issue #104 implementation continuation — Android + macOS adapters — 2026-11-20

- Latest code-bearing candidate: `4be1b8fd579092d8195c3fb775b7c765f6221814`; current PR #113 HEAD before this documentation update: `0bed293163aee8d398e1894811ef1ece17b78ff5`. PR remains OPEN / DRAFT / UNMERGED; no source build/test/device evidence is claimed.
- Added Android direct-child filesystem support using retained root handles, no-follow opens, mandatory `statx(STATX_MNT_ID)` checks, hard-link rejection, and atomic temp-file replacement. Android nested paths remain fail-closed because directory descriptors alone do not prevent parent rename-out.
- Added macOS direct-child filesystem support using retained root handles, no-follow opens, device-identity checks, hard-link rejection, and atomic temp-file replacement. macOS nested paths remain fail-closed because directory descriptors alone do not prevent parent rename-out.
- Added three target-gated regression tests per platform. They are implementation coverage only until compiled and executed on the respective targets.
- Linux nested writes use the detached-mount/root-relative backend when the kernel and backing filesystem provide the required operations; the implementation fails closed with `UnsupportedPlatform` when they do not. Windows retains the native handle-relative backend. Android and macOS currently provide direct-child filesystem operations; nested paths remain fail-closed until a native anti-move primitive is implemented.
- These changes advance platform implementation but do NOT close Issue #104. Remaining work includes adversarial review of parent/root movement, platform-specific execution evidence, and any acceptance gaps in the fixed Issue #104 contract. Do not run the broad release matrix until implementation scope is complete.

## Issue #105 — Linux process-group containment implementation candidate — 2026-11-20

- Latest code-bearing candidate: `0a9685bded788ae46596227bd61457d10db1179c` (NOT BUILT OR TESTED).
- Linux subprocess startup now installs an inherited seccomp filter in the child before `exec`. It denies `setsid`, `setpgid`, `unshare`, `setns`, clone namespace flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` so compatible runtimes can fall back to `clone`. The existing dedicated process group is therefore intended to remain non-escapable for descendants; group cleanup still runs on normal return, timeout, and output overflow.
- If the kernel refuses `PR_SET_NO_NEW_PRIVS` or filter installation, process spawn fails closed. Windows Job Object containment remains unchanged.
- At this checkpoint, process execution was unavailable on Android and macOS pending equivalent containment. The subsequent Android process-containment extension below supersedes the Android portion: the current source advertises Linux, Android, and Windows, while macOS remains fail-closed.
- Replaced the old escaped-session reader-only probes with three Linux adversarial regressions for attempted `setsid` on normal completion, timeout, and output overflow. These tests have NOT been run.
- This is an implementation candidate, not Issue #105 acceptance. Exact-SHA build/execution and adversarial evidence are still required. Preserve bounded reader shutdown and all existing process limits.

## Android process containment extension — 2026-11-20

- Code candidate `58ae7831c7d17e60e53fa6a933dc2f170196c585` enables the same inherited Linux seccomp process-group filter on Android as on Linux and advertises `ProcessAdapter` on Android. It denies session/group changes and namespace escapes; filter setup failure is fail-closed.
- macOS process execution remains unavailable until equivalent containment exists. Windows Job Object behavior is unchanged.
- This Android extension is NOT BUILT OR TESTED. Android seccomp policy compatibility and the three adversarial process tests still require execution on an actual Android/Termux target. Issue #105 remains OPEN / NOT ACCEPTED.

## Android process containment regression coverage — 2026-11-20

- Latest code-bearing candidate: `418d9642778d975440b81df2bdb5e1bf9aa1d8a5` (NOT BUILT/TESTED).
- The inherited seccomp pre-exec filter is now installed on both Linux and Android. ProcessAdapter advertises Linux, Android, and Windows; macOS remains fail-closed. Android uses the same `setsid`/`setpgid`/namespace restrictions and fails closed if the child cannot install the filter.
- The three adversarial `setsid` regression tests and bounded-reader test are now target-gated for Linux and Android, so they can run in Termux as well as Linux CI. No Android/Termux execution evidence is claimed.
- Issue #105 remains OPEN / NOT ACCEPTED pending exact-SHA build and adversarial execution on Linux and a real Android/Termux target, plus Windows Job Object evidence reconciliation.

## IMPLEMENTATION CHECKPOINT — 2026-11-21 (supersedes older platform-status bullets)

Code-bearing source/test candidate: `4c46a41fe45baa7627557a2a25b9c71812840479`. PR #113 was OPEN / DRAFT / UNMERGED at this exact SHA before documentation synchronization. This candidate has NOT been built or tested; no previous exact-SHA result transfers.

### Issue #104 — filesystem implementation
- Linux: retained root handle, `openat2` no-follow/beneath/no-cross-device reads; atomic direct-child replacement; nested writes use the detached-mount + `mount_setattr(MOUNT_ATTR_NOSYMFOLLOW)` + `openat2` path and fail closed when the kernel/security profile cannot provide those primitives.
- Windows: existing native handle-relative/reparse-point backend and retained-parent rename protection remain unchanged.
- Android: direct-child reads and atomic replacement use retained root `dirfd`, `O_NOFOLLOW`, mandatory `statx(STATX_MNT_ID)` equality, regular-file/single-link checks, exclusive temporary sibling, file sync, root-directory sync, and `renameat`. Nested paths remain fail-closed.
- macOS: direct-child reads and atomic replacement use retained root `dirfd`, `O_NOFOLLOW`, regular-file/single-link checks, `fstatfs` filesystem-ID plus mount-point comparison, exclusive temporary sibling, file sync, root-directory sync, and `renameat`. Nested paths remain fail-closed.
- Android/macOS hard-link side-effect regressions were added. Existing Android/macOS direct-child, symlink, and root-symlink regressions remain in source. These tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED until the fixed acceptance criteria have exact-SHA Unix and Windows execution evidence, including race/side-effect proof.

### Issue #105 — process containment implementation
- Linux and Android subprocess startup install an inherited seccomp filter before `exec`; it denies `setsid`, `setpgid`, `unshare`, `setns`, namespace-bearing `clone` flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` for compatible fallback. Filter setup failure is fail-closed.
- Windows Job Object containment is unchanged. macOS process execution remains fail-closed until equivalent non-escapable containment exists.
- Three adversarial `setsid` regressions cover normal completion, timeout, and output overflow on Linux/Android; they are NOT RUN. Issue #105 remains OPEN / NOT ACCEPTED pending Linux + real Android/Termux execution and reconciliation of Windows Job Object evidence.

### Execution priority
- Continue production implementation against the existing v0.1 contract and release blockers. Add regression coverage with each code change, but do not launch broad matrix/audit cycles until the implementation scope is complete.
- Preserve B-001's prior exact-SHA success; do not repeat its investigation without a new relevant provider failure.
- Preserve `main`, protected verifier pins, fixed ledger weights, DoD, acceptance criteria, and release authorization boundaries.


## Implementation continuation refresh — 2026-11-21

Latest code-bearing candidate and PR #113 HEAD at this checkpoint: `fd1ccc5ca56f48a306ac8f8c551189f46b8fe44c` (OPEN / DRAFT / UNMERGED; NOT BUILT/TESTED). This supersedes the previous 2026-11-21 candidate pointer.

Issue #105 regression coverage now uses ignored helper tests in the same Rust test binary. The helper forks a descendant and calls `setsid(2)` directly, avoiding dependence on an external `setsid` utility on Android/Termux. The parent tests cover normal completion, timeout, and output overflow, record any escaped PID, and clean up the escaped group before reporting failure. The tests are NOT RUN.

Issue #104 implementation changes from the previous checkpoint remain: Linux guarded detached-mount nested writer; Android direct-child `statx(STATX_MNT_ID)` backend; macOS direct-child `fstatfs` filesystem-ID/mount-point backend; exclusive temp replacement with file and directory sync; hard-link regressions. Windows native backend remains unchanged. Nested Android/macOS operations fail closed.

Issues #104 and #105 remain OPEN / NOT ACCEPTED. No exact-SHA build/test or broad verification is claimed. Continue implementation-first and preserve all release/security gates.

## Process-group cleanup hardening — 2026-11-21

Latest code-bearing source/test candidate: `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` (NOT BUILT/TESTED). Linux/Android process supervision now observes direct-child exit using `waitid(P_PID, WEXITED | WNOHANG | WNOWAIT)` rather than reaping via `try_wait()` before process-group cleanup. The child remains a zombie (its PID/PGID cannot be recycled) while the wrapper sends SIGKILL to the invocation process group; only then does `Child::wait()` reap it. The redundant post-reap group kill was removed. All timeout, output-overflow, wait-error and normal-completion paths terminate the group before reaping.

Process escape regressions use in-binary ignored helpers with direct `fork` + `setsid(2)` syscalls, record any escaped PID and clean up before failing. Three paths: normal completion, timeout, output overflow. NOT RUN. Issue #105 remains OPEN / NOT ACCEPTED.

## Scope reconciliation — fixed milestone ledger vs v0.1 release — 2026-11-21

The protected verifier's `LEDGER_SOURCE_COMMIT.txt` pins documentation SHA `d6c382a245c49a028bc60809f22ca956c7f894cc`, which contains the fixed 100-point milestone ledger and Definition of Done. Its 92/100 implementation subtotal (E1 7/10, E2 7/8, E3 8/8, E4 6/6, E5 4/4, E6 0/4) is PROVISIONAL and belongs to the separate milestone-100 workstream; it is not a completion score for PR #113's current v0.1 candidate. The current PR #113 source branch is `security/issue-104-root-confinement-20261118`, with code-bearing candidate `72eae103bc3304b0961c66a0cf61f716ddfa9ac4`. The separate `milestone/100-ecosystem-20261023` branch is not safe to merge as-is and is not the current PR #113 integration base.

The approved v0.1 package/build contract explicitly scopes the executable subset to an empty external dependency set and deterministic debug/intermediate `.oneir` / `.onepkg` artifacts; it does not claim a full external package registry or complete signed provenance envelope. Do not import future vNext/milestone-100 scope into v0.1 merely to chase ledger points, and do not alter the protected ledger, fixed weights, DoD, or release acceptance. Finish implementation against the approved v0.1 contract, then perform the required exact-SHA verification and gate reconciliation.


## Subprocess resource-bound implementation refresh — 2026-11-21

The latest code-bearing source for Issue #81/#105 is `cd95e5ec2234a2909ef79bf8b25fe65296fab5ed`. It explicitly enforces a 10-second deadline, 1 MiB stdout cap, 1 MiB stderr cap, and 2 MiB aggregate output ceiling before returning a process result. Source tests cover stdout overflow, stderr overflow, timeout, normal exit/status/stream preservation, and Linux/Android direct-fork + `setsid` escape attempts for normal completion, timeout, and output overflow. No build or test was run on this source. Issues #81 and #105 remain OPEN; no security or release acceptance is claimed.


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


## Duplicate subprocess-test helper compile blocker removed — 2026-11-21

- Source commit: `0cceeb447fbe2ae59b8e3da78dee3ce8d9959830` (`crates/one-core/src/interop.rs`).
- Removed the second identical `cleanup_test_escaped_group` definition from the `interop.rs` test module. The duplicate would produce a Rust duplicate-definition error when compiling the test module; only one definition remains in source.
- This is a source-level compile-blocker correction, not a compile/test result. No build, test, CI, verifier, or platform matrix was run. Issue #105 remains OPEN / NOT ACCEPTED.


## Current platform capability reconciliation — 2026-11-21

This section supersedes earlier broad platform summaries where they conflict with the latest source inventory; it does not promote any platform to accepted or supported status.

- Filesystem adapter metadata currently includes Linux, Windows, Android, and macOS. Linux has a detached-mount nested-write backend with explicit fail-closed fallback; Android/macOS nested filesystem paths remain fail-closed until native anti-move primitives are implemented. Windows retains its handle-relative backend. Direct-child implementation presence is not Issue #104 acceptance.
- Process adapter metadata currently includes Linux, Android, and Windows. Linux/Android use the inherited seccomp process-group filter; Windows uses Job Objects. macOS process execution remains fail-closed. Issue #105 remains OPEN / NOT ACCEPTED.
- The current PR #113 source also had a duplicate subprocess-test helper removed in commit `0cceeb447fbe2ae59b8e3da78dee3ce8d9959830`. This is a source compile-blocker correction, not build/test evidence.
- No broad build, test, CI, verifier, or platform matrix has been run on the latest candidate. Do not claim a verified/accepted/supported release state.


## Current Issue #104 platform-boundary correction — 2026-11-21

Latest source commit: `5f98db9f70d28a0d5a206a3e6e37f986efc3574b` (`crates/one-core/src/filesystem.rs`).

- Removed a duplicate Android `create_temp` definition and added the missing macOS `create_temp` implementation. The duplicate Android definition and missing macOS helper were source-visible compile blockers; no build was run.
- Android and macOS retain direct-child no-follow read and atomic replacement paths. Nested paths remain explicitly fail-closed. A first attempt to generalize traversal using ordinary POSIX directory handles was not retained as supported: an opened parent directory can be renamed outside the trusted root while its descriptor remains usable. Mount identity and `O_NOFOLLOW` checks alone do not close that parent-move race.
- The Android/macOS parent resolver now rejects any multi-component target with `UnsupportedPlatform` before opening a nested parent. Direct-child operations continue to use the retained root handle and existing mount/filesystem identity checks.
- Android/macOS regression fixtures assert nested-path denial and preserve an outside sentinel. They are NOT RUN.
- This is implementation hardening and capability-boundary correction only. Issue #104 remains OPEN / NOT ACCEPTED; no platform is promoted to accepted or supported. Do not run broad verification until implementation scope is complete.
