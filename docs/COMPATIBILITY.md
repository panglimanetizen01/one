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

The Issue #104 candidate now uses `PlatformKind::current_host()` for orchestrator selection. `FileReadAdapter` and `FileWriteAdapter` advertise only the compiled native backend: Linux builds advertise Linux; Windows builds advertise Windows; Android, macOS, and other unsupported targets advertise no filesystem backend and fail closed. This prevents a generic `PlatformKind::Any` declaration from overstating filesystem support.

Exact candidate evidence: source `c7f460a4e38be61de635ad5af9f7ff987a941f01`; E4 run #44585270928 passed the metadata regression on Linux and Windows; Linux distro run #44585282690 passed Ubuntu 24.04 and Debian 12 workspace tests. CircleCI on this source remained PENDING at this checkpoint. Issue #104 remains OPEN / NOT ACCEPTED and this candidate is not in `main`.

## Host-owned filesystem root implementation checkpoint — 2026-11-20

PR #113 implementation now derives filesystem authority exclusively from the host process working directory. Linux root acquisition uses `O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC` and checks device/inode identity against the retained handle. Windows preserves the final root component, opens with native no-reparse flags, and checks volume/file identity. CLI positive/negative root-selection fixtures plus Linux symlink-root and Windows junction-root regressions have been added.

Latest source/test candidate: `ea376531e99f37b874d3d3a8edd4e8b4d656095f`; current PR #113 HEAD at last readback: `6f7d86842c80a81135227d8d62404e848ab5d8c3`. These changes have NOT been built or tested; no prior exact-SHA evidence transfers. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

## Android syscall-availability boundary — 2026-11-20

Do not enable the Linux `openat2` backend for Android by changing `cfg(target_os = "linux")` to include Android. A recent native Termux report on Android 16 documents Android seccomp delivering `SIGSYS` for `openat2`, which terminates the process instead of returning `ENOSYS`/`EPERM` for a safe fallback ([upstream report](https://github.com/oven-sh/bun/issues/39060)). This is a specific environment report, not a claim that every Android version/device blocks the syscall.

Android filesystem support therefore remains fail-closed until a separate resolver is designed around primitives that preserve the required root, symlink, mount-identity, and race guarantees, and then exercised on actual Android/Termux targets. An errno-only fallback is insufficient if the syscall can terminate the process.
