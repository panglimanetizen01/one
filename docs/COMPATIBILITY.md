# One Compatibility Contract

## Compatibility principle

One is designed as a portable language/toolchain. Compatibility targets are hosts, not semantic forks.

## Operating systems

### First-class

- Linux
- Windows

### Planned/targeted expansion

- macOS
- additional Unix-like systems
- other platforms supported by future backends

## Linux distributions

The compatibility program must include build/run paths for major distributions, including:

- Ubuntu
- Debian
- Kali Linux
- Alpine Linux
- Arch Linux
- Fedora
- openSUSE
- other distributions using compatible architectures/toolchains

Alpine's musl environment and other libc/toolchain differences must be handled by the platform/build layer rather than by changing One semantics.

## Android/Linux environments

- Termux: native Android host integration target.
- UserLAnd: Linux userspace integration target.
- Pydroid: application-hosted integration target when its execution model permits invoking One binaries/tooling.
- Other Linux/container environments: supported according to their ABI, architecture, filesystem, process, and toolchain capabilities.

Compatibility claims require executable evidence for the relevant revision and environment.

## Terminals and shells

Target terminal environments include:

- Bash
- Zsh
- Fish
- POSIX-style `sh`
- BusyBox-style environments
- SSH sessions
- CI terminals
- container terminals
- desktop Linux terminals
- Windows terminal hosts
- Android terminal hosts

The shell is an invocation host, not One's semantic runtime.

## Editors and IDEs

Integration targets include:

- VS Code
- Acode
- SuperCode
- terminal editors
- future IDEs/editors

Integration should use CLI, LSP, formatter, diagnostics, debugger, project metadata, and build/test interfaces as applicable.

## CPU architectures

The architecture should avoid hard-coding one CPU family. Practical initial targets should include x86-64 and ARM64, with ARM and other architectures added as backend/toolchain evidence permits.

## Containers and remote environments

One should work in containers, VMs, remote shells, SSH sessions, and CI systems when the required runtime/toolchain dependencies are available.

## Support-status discipline

A platform is not Supported merely because code appears portable. Record:

```text
platform
OS/distribution
architecture
host environment
One revision
compiler/toolchain
command
result
known limitations
```

## Compatibility objective

The long-term objective is broad practical reach without fragmenting the language:

```text
One language semantics
        ↓
Portable IR/toolchain/runtime
        ↓
Platform abstraction
        ↓
OS / distro / architecture
        ↓
CLI / terminal / IDE / container / device
```


## Filesystem project-root confinement status — 2026-11-18

Filesystem root confinement is an experimental capability under Issue #104, not a general platform-support claim.

- Linux candidate: reads use `openat2` with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`. Candidate atomic writes are limited to direct children of the retained host root; nested write paths fail closed.
- Windows candidate: handle-relative traversal and temporary-sibling atomic replacement are implemented in the open PR candidate and require exact-SHA Windows verification for the latest change set.
- Other platforms: fail closed / NOT IMPLEMENTED for this capability.
- Canonical `main` does not yet contain the unmerged PR #113 candidate. Do not treat the candidate as a released or broadly SUPPORTED feature.

Compatibility status must be recorded per exact source revision and tested environment. Passing one Windows Server 2025 runner does not establish support for all Windows versions or filesystems.


### CLI filesystem root selection

The CLI's filesystem authority root is selected exclusively from the host process current working directory (`std::env::current_dir()`), independently of source-file location and requested target. The source path and `--target` cannot select or widen this root. Relative and absolute targets outside it are rejected for both reads and writes. This host integration remains a draft PR #113 candidate pending exact-SHA verification.


## Filesystem adapter platform metadata and host selection — 2026-11-20

The Issue #104 candidate uses `PlatformKind::current_host()` for orchestrator selection. Linux, Windows, Android, and macOS builds now advertise their respective direct-child filesystem backend. Android requires `statx(STATX_MNT_ID)`; macOS verifies `st_dev`; both reject symlink/hardlink leaves and use root-relative atomic replacement. Nested Android/macOS paths and Linux nested writes remain fail-closed. The new Android/macOS code is NOT BUILT/TESTED.

Exact candidate evidence: source `c7f460a4e38be61de635ad5af9f7ff987a941f01`; E4 run #44585270928 passed the metadata regression on Linux and Windows; Linux distro run #44585282690 passed Ubuntu 24.04 and Debian 12 workspace tests. CircleCI on this source remained PENDING at this checkpoint. Issue #104 remains OPEN / NOT ACCEPTED and this candidate is not in `main`.

## Host-owned filesystem root implementation checkpoint — 2026-11-20

PR #113 implementation now derives filesystem authority exclusively from the host process working directory. Linux root acquisition uses `O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC` and checks device/inode identity against the retained handle. Windows preserves the final root component, opens with native no-reparse flags, and checks volume/file identity. CLI positive/negative root-selection fixtures plus Linux symlink-root and Windows junction-root regressions have been added.

Historical candidate `ea3765...` is superseded. Latest source candidate is `4be1b8fd579092d8195c3fb775b7c765f6221814` plus later Issue #105 source `0a9685bded788ae46596227bd61457d10db1179c`; both are NOT BUILT/TESTED. Android/macOS direct-child adapters are implemented as candidates; nested paths remain fail-closed. Issue #104 remains OPEN / NOT ACCEPTED.

## Android syscall-availability boundary — 2026-11-20

Do not enable the Linux `openat2` backend for Android by changing `cfg(target_os = "linux")` to include Android. A recent native Termux report on Android 16 documents Android seccomp delivering `SIGSYS` for `openat2`, which terminates the process instead of returning `ENOSYS`/`EPERM` for a safe fallback ([upstream report](https://github.com/oven-sh/bun/issues/39060)). This is a specific environment report, not a claim that every Android version/device blocks the syscall.

Android filesystem support therefore remains fail-closed until a separate resolver is designed around primitives that preserve the required root, symlink, mount-identity, and race guarantees, and then exercised on actual Android/Termux targets. An errno-only fallback is insufficient if the syscall can terminate the process.

## Android host identity — 2026-11-20

- `crates/one-core/src/platform.rs` now exposes `Platform::Android` and returns it for `target_os = "android"` instead of collapsing Android into `Platform::Other` (`f8e46856803c9f45f54402e71d7e19853bdf46bc`).
- This is host identity only; it does NOT enable Android filesystem operations. The filesystem adapter remains fail-closed on Android until a safe resolver is available that avoids process-killing seccomp behavior and preserves root/symlink/mount/race guarantees.
- The platform unit test was updated but has NOT been run.

## Explicit desktop/mobile host identity — 2026-11-20

- `Platform` now identifies Windows, Linux, Android, and macOS explicitly rather than collapsing Android/macOS into `Other`; the platform unit test covers all four target cfgs (`5846eb70f2a462331873cc0023d1f1ab1106f86a`).
- This is platform identification only. It does not imply filesystem adapter support on Android/macOS; those adapters remain fail-closed pending platform-safe resolver implementations.
- The platform test has NOT been run.

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
