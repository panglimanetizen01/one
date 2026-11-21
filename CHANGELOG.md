# Changelog

## Unreleased — security and filesystem integration

- Bind filesystem authority to the host process working directory instead of the `.one` source location or requested target.
- Open Linux project roots with directory/no-follow/close-on-exec flags and validate path identity against the retained root handle.
- Open Windows project roots without following a final reparse point; validate volume/file identity against the retained native handle.
- Add a CLI regression fixture for a source file outside the working directory accessing an in-root target.
- Add the experimental `hapus berkas` language/CLI path with an explicit `filesystem.delete` grant and exact target scope; native handle-bound deletion is Windows-only, while Linux/Android/macOS remain fail-closed.
- Add Linux/Android process-containment adversarial source coverage for `setsid` and `setpgid` escape attempts across normal exit, timeout, and output overflow, with isolated per-test working directories. These new regressions are not yet verified on the latest candidate.


## Unreleased — One v0.1

- Added formal lexical and concrete grammar contracts.
- Added Rust 2024 workspace and Linux/Windows CI matrix.
- Added deterministic lexer, AST, parser, semantic analysis, Semantic IR, reference interpreter, diagnostics, and capability model.
- Added cross-platform process interoperability and core standard-library primitives.
- Added deterministic project lifecycle, conversational Indonesian CLI commands plus technical aliases, manifest validation, `.oneir`/`.onepkg` artifacts, conformance fixtures, and golden outputs.
- Added Linux/Windows offline verification scripts and deterministic artifact checks so verification does not depend on purchasing hosted CI capacity.
- Added the One Network semantic/runtime foundation and verified native runtime slices for DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit limits.
- Added the general-purpose human-language orchestration foundation: typed semantic capability requests, scoped deterministic governance, typed adapters, deterministic orchestration, normalized evidence, audit records, Indonesian/English conformance fixtures, and the first real `network.dns.resolve` adapter.
- Added the first experimental language-level orchestration slice: `cari alamat` reaches the governed DNS adapter through typed IR and returns a normalized CLI result when `network.discover` is explicitly granted for an exact target.
- Added the second experimental language-level orchestration slice: `baca berkas` reaches the governed `filesystem.read` adapter through typed IR and returns normalized UTF-8 text when `filesystem.read` is explicitly granted for an exact target.
- Added policy-denial, out-of-scope, backend-failure, and unavailable-adapter coverage for the filesystem source-level orchestration path.
- Independently verified the filesystem source-level orchestration/CLI revision on Ubuntu and Windows Server 2025 through exact-SHA checkout, contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, offline verification, and deterministic artifact verification at canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` via mirror run `38903950612`.
- Refreshed the implementation-status, release, CLI contract, grammar, and user-guide documentation to distinguish the verified filesystem vertical slice from the still-in-development orchestration catalog.
- Added the language-level process-execution slice: `jalankan proses` → parser → semantic validation → typed orchestration IR → target-scoped governance → `process.execute` adapter → normalized output → CLI, with TDD coverage for success, denial, and backend failure.
- Independently verified the process-execution source/runtime slice at canonical source `9038ac75f197753ff1ab6af7478e63e4831903a2` via mirror run `38929072800` on Ubuntu and Windows Server 2025. The verified boundary is one explicitly targeted program without additional arguments and explicit `process.execute` permission.
- Added the filesystem-write source-level slice: `tulis ke berkas` → parser → semantic validation → typed orchestration IR → target-scoped governance → `filesystem.write` adapter → CLI.
- Added a CLI regression test proving `filesystem.write` permission reaches the adapter and writes the expected text to the exact target. The filesystem-write CLI verification was first recorded on Termux/Android and remains platform-bound to its recorded exact SHA.
- Added the database-query source/runtime slice: `kueri database` → parser → semantic validation → resource-scoped `database.query` capability → `database.sqlite.query` adapter → parameterized SQLite `SELECT` → deterministic UTF-8 normalization → CLI, with executable evidence recorded for its current Termux/Android boundary.
- Added the HTTP GET source/runtime slice: `ambil dari` → parser → semantic validation → target-scoped `network.connect` capability → `network.http.get` adapter → bounded HTTP/1.1 GET → deterministic response-body normalization → CLI, with executable evidence recorded for its current Termux/Android boundary.
- Added the native hosted backend contract and implementation slice for Linux AArch64, Linux x86_64, and Windows x86_64 on PR #64. Semantic IR remains target-independent and native code generation uses the existing deterministic backend/toolchain boundary.
- Native Linux AArch64 is verified on the exact PR #64 SHA `797c6fe75bd3fa9081329f73dd59e34278f27ddd` for an AArch64/Linux host boundary: ELF AArch64 artifact built and executed successfully.
- Native Linux x86_64 is build-proven on the same PR SHA: ELF x86-64 artifact built successfully using the verified cross-linker selection and the dedicated regression test passes. Target execution remains pending because no x86_64 executor was available.
- Native Windows x86_64 is build-proven on the same PR SHA: PE32+ x86-64 artifact proven. Target execution remains pending because no Windows/Wine executor was available.
- PR #64 was merged into canonical `main` at `ec844aa652a4d35c3891fc810d344c9c9db2f5cf`. The pre-merge native evidence remains bounded to its recorded implementation SHA until exact-SHA post-merge verification is completed.
- The first post-merge `One CI` run `39371290853` failed on both matrix jobs before any executable step (`steps: []`); this is recorded as an infrastructure/runner failure, not as source-code failure evidence.
- Updated the native backend contract map, implementation-status inventory, evidence matrix, release boundary, user feature-status guide, and verification record to reflect the merged state and preserve exact-SHA evidence boundaries.

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution.

## Release execution controls — 2026-11-20

- Added one linked release blocker register and a compact cross-session master prompt, connected to the existing roadmap, evidence matrix, Mermaid architecture source, release contract, and documentation index.
- Recorded the exact CircleCI pre-execution mapping for PR #113 pipelines #1199 and #1198. Both jobs have `started_at=null`; root cause remains UNKNOWN and is not labeled a test timeout or source failure.

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

## Issue #105 — Linux process-group containment implementation candidate — 2026-11-20

- Latest code-bearing candidate: `0a9685bded788ae46596227bd61457d10db1179c` (NOT BUILT OR TESTED).
- Linux subprocess startup now installs an inherited seccomp filter in the child before `exec`. It denies `setsid`, `setpgid`, `unshare`, `setns`, clone namespace flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` so compatible runtimes can fall back to `clone`. The existing dedicated process group is therefore intended to remain non-escapable for descendants; group cleanup still runs on normal return, timeout, and output overflow.
- If the kernel refuses `PR_SET_NO_NEW_PRIVS` or filter installation, process spawn fails closed. Windows Job Object containment remains unchanged.
- Process execution is now explicitly unavailable on Android and macOS until equivalent non-escapable containment exists; `ProcessAdapter` advertises only Linux and Windows.
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

### Issue #104 — macOS nested-write candidate (2026-11-21)
- Add a source-only macOS nested atomic-write candidate using XNU `renameatx_np(RENAME_RESOLVE_BENEATH | RENAME_NOFOLLOW_ANY)`, root-mount staging, XNU beneath/no-symlink/unique checks, and `fstatfs` parent validation. Add source regressions for nested replacement, symlink parent, hard link, and parent moved outside root before publication.
- Correct platform status: current Android source is direct-child-only; multi-component read/write paths fail closed because the observed Termux seccomp profile kills `openat2` with SIGSYS. Earlier notes describing Android nested `openat2`/detached-mount support are not the current source state.
- macOS deletion remains fail-closed because XNU `unlinkat` lacks an atomic no-cross-mount constraint; Linux/Android deletion remain fail-closed. macOS process containment remains fail-closed; Seatbelt `process-fork` is research only.
- Candidate `c7a2d45a0e218c7e26a3f4b2dad0a206e024307e` is NOT BUILT / NOT TESTED; no support or acceptance claim is made.

### Issue #105 — macOS Seatbelt process-creation guard candidate (2026-11-21)
- Add a macOS process wrapper candidate using `/usr/bin/sandbox-exec` and SBPL `deny process-fork`, with host-enforced timeout and output caps. Source probes cover fork, vfork, posix_spawn, posix_spawnp, exec replacement, timeout, and output overflow.
- XNU source inspection shows `fork1()` invokes `mac_proc_check_fork()` and the spawn path uses `PROC_CREATE_SPAWN`; this supports the direct process-creation denial hypothesis but is not native test evidence.
- This is single-process mode, not descendant-tree termination. Brokered launchd/Mach-service launches remain unproven, and `ProcessAdapter` continues to exclude macOS. `sandbox-exec` is deprecated; absence or profile failure must fail closed.
- Candidate `3bad0196e9352871d09cb9b966cf6b393a866603` is NOT BUILT / NOT TESTED. Issue #105 remains OPEN / NOT ACCEPTED.

- Extend the ProcessAdapter platform-metadata regression to macOS so the experimental Seatbelt single-process runner remains unadvertised until brokered process creation and native acceptance are resolved. Source/test candidate `0408b92f6eafb12c18371d0079c6e73915e8e87c` is NOT BUILT / NOT TESTED.

### Build blocker correction — Project ONE (2026-11-21)
- Fix Rust parser errors in the process escape probe by parenthesizing unsafe `setpgid` and `setsid` call expressions before comparisons.
- Commit: `502a888da46e3d4be4ac80b3a2ffc3233dbabc12`.
- The pre-fix GitHub Actions Ubuntu and Windows Build jobs failed on exact SHA `168ff06e7a8d09e575e6ac71513399c9f1f4f994`; the fixed SHA has not yet been rebuilt. No verification or release status is promoted.
