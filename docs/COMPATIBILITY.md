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

## Robots, embedded systems, and electronic devices

The compatibility program must also be capable of targeting programmable physical devices through appropriate compiler backends, runtimes, SDK integrations, or generated artifacts. Target categories include:

- robots and robotic controllers;
- microcontrollers and embedded computers;
- industrial/device controllers where technically appropriate;
- smart televisions and media appliances;
- refrigerators and refrigeration controllers;
- air conditioners and HVAC controllers;
- electric fans and ventilation controllers;
- air coolers and other cooling equipment;
- other programmable household and consumer electronic appliances.

This section defines compatibility direction, not current support status. A specific device, board, chipset, RTOS, firmware environment, or appliance model must not be called Supported without executable evidence for that target and revision.

Hardware, firmware, real-time, memory, power, peripheral, boot, and vendor-SDK differences belong in the platform/backend layer and must not create incompatible One language semantics.

## Containers and remote environments

One should work in containers, VMs, remote shells, SSH sessions, and CI systems when the required runtime/toolchain dependencies are available.

## Support-status discipline

A platform is not Supported merely because code appears portable. Record:

```text
platform
OS/distribution
architecture
host environment
device/board/firmware target when applicable
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
OS / distro / architecture / device
        ↓
CLI / terminal / IDE / container / robot / embedded device / appliance
```
