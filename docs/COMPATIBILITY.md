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

The CLI's filesystem authority root is selected from source/project context, not the requested target. It prefers the nearest `one.toml` ancestor; otherwise it uses the working directory for sources beneath it or the canonical source parent for standalone sources outside it. Absolute targets outside this root are rejected for both reads and writes. This host integration is still a draft PR #113 candidate pending exact-SHA verification.
