# ONE Toolchain Execution Boundary

**Status:** Approved design, implementation in progress
**Date:** 2026-10-20

## Goal

Add a deterministic execution boundary between ONE's existing `ToolchainSpec` model and platform compiler/linker processes, without changing the Semantic IR, target model, or existing native backend behavior.

## Problem

The target/toolchain foundation already models target identity, linker strategy, artifact format, and build/execute capability. Existing native backends still own process invocation and executable discovery directly. This couples semantic target validation to host-specific process behavior and makes future cross-compilation, firmware, and embedded adapters harder to isolate and test.

## Architecture

```text
One Semantic IR
      |
      v
 TargetSpec
      |
      v
 ToolchainSpec
      |
      v
 Toolchain execution boundary
   |              |
resolver       invoker
   |              |
compiler/linker process
      |
      v
 artifact validation
```

The execution boundary is deliberately below deterministic target/toolchain validation and above platform process details.

## Design Rules

1. `ToolchainSpec::validate_for(...)` remains deterministic and must not inspect the host filesystem, environment variables, PATH, installed compilers, or linker executables.
2. Toolchain resolution is an execution concern. It may inspect an explicitly supplied configuration and the host environment only at the point where a build is requested.
3. Process invocation must use structured argument vectors (`Command`/equivalent), never shell interpolation or shell command strings.
4. Host-toolchain identity and target-toolchain identity must remain distinct. A host executable used to perform a cross-build is not itself the target identity.
5. Resolver output must be explicit enough to reproduce the selected compiler/linker command: executable path/name plus target/toolchain metadata.
6. Invocation failures must be structured and distinguish resolution failure, process-start failure, non-zero compiler/linker exit, and artifact validation failure.
7. Tests must be able to exercise resolution and invocation with fixture executables without requiring an externally installed cross compiler.
8. The first real integration proof remains one hosted/cross-compilation slice; this design does not add GPIO, HAL, MCU SDKs, RTOS APIs, firmware flashing, or target-specific Semantic IR.
9. Existing native backends remain behavior-compatible. Refactoring them behind the boundary is allowed only when tests prove no contract change.
10. No automatic package installation, SDK download, network discovery, or mutation of the developer machine is permitted.

## Proposed Interfaces

The implementation should introduce a small execution module, preferably `crates/one-core/src/toolchain.rs` or an equivalent focused module following existing source organization.

Conceptual interfaces:

```rust
pub struct ResolvedToolchain {
    pub compiler: PathBuf,
    pub linker: Option<PathBuf>,
}

pub trait ToolchainResolver {
    fn resolve(&self, spec: &ToolchainSpec) -> Result<ResolvedToolchain, ToolchainExecutionError>;
}

pub struct ToolchainCommand {
    pub executable: PathBuf,
    pub args: Vec<OsString>,
}

pub trait ToolchainInvoker {
    fn invoke(&self, command: &ToolchainCommand) -> Result<InvocationOutput, ToolchainExecutionError>;
}
```

Exact names may follow the repository's established naming conventions during implementation; the boundary must preserve these responsibilities even if signatures differ.

## Error Contract

The execution layer must provide stable machine-readable categories for at least:

- toolchain not found / unresolved
- compiler or linker process could not start
- compiler or linker exited unsuccessfully
- artifact missing or invalid after successful process execution

Errors must retain actionable human-readable context without exposing shell commands as executable strings.

## Compatibility

- Existing `TargetDefinition`, `TargetSpec`, `ToolchainSpec`, `TargetBackend`, and `NativeBackend` contracts remain valid.
- Existing Linux/Windows native backend tests remain authoritative.
- Existing historical cross-linker resolver behavior is reference evidence, not a reason to reimplement that resolver.
- The Android AArch64 backend remains explicitly bounded by its current toolchain availability and execution-test policy.

## Non-Goals

- No new MCU/vendor SDK integration.
- No HAL implementation.
- No GPIO/I2C/SPI/UART/CAN device APIs.
- No RTOS scheduler/runtime.
- No firmware flashing.
- No automatic toolchain installation/discovery service.
- No target-specific Semantic IR fork.
- No replacement of existing linker discovery already merged in native backends.
