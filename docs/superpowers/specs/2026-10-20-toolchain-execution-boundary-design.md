# Gate 1 — Toolchain Execution Boundary Design

## Goal

Add a host-process boundary between One target/toolchain metadata and an external compiler/linker without changing parser, analyzer, One Semantic IR, or target semantics.

## Architectural boundary

```text
One Source
 -> Parser
 -> Analyzer
 -> One Semantic IR
 -> TargetSpec
 -> ToolchainSpec
 -> ToolchainResolver
 -> ToolchainCommand
 -> ToolchainInvoker
 -> Compiler + Linker process
 -> ToolchainInvocation
 -> Artifact validation
 -> NativeArtifact
```

The execution layer owns process resolution, deterministic command construction, process invocation, structured execution errors, and artifact existence checks. Existing target/toolchain compatibility validation remains authoritative and is performed before process execution.

## Interfaces

- `ToolchainCommand`: executable path plus ordered `OsString` arguments. It is passed directly to `std::process::Command`; no shell command string is constructed or parsed.
- `ToolchainInvocation`: process `ExitStatus`, captured stdout, and captured stderr for a completed process.
- `ToolchainResolver`: resolves a compatible `ToolchainSpec` to a `ToolchainCommand`.
- `PathToolchainResolver`: resolves a configured executable path for the requested toolchain.
- `ToolchainInvoker`: executes a `ToolchainCommand` and returns `ToolchainInvocation` or a structured `ToolchainExecutionError`.
- `CommandToolchainInvoker`: invokes through `std::process::Command` with arguments preserved as discrete values.
- `execute_toolchain_build`: validates target/toolchain compatibility, resolves the command, invokes it, validates the expected artifact exists, and returns `NativeArtifact`.

## Error boundary

`ToolchainExecutionError` must distinguish:

1. `ToolchainNotFound` — executable could not be resolved by the configured path resolver or the OS reported `NotFound` while starting it.
2. `ProcessStart` — executable resolution succeeded but process creation failed for another OS error.
3. `NonZeroExit` — process started and completed unsuccessfully; exit status/stdout/stderr are retained.
4. `ArtifactMissing` — process completed successfully but the expected artifact does not exist.
5. `Validation` — target/toolchain compatibility or artifact validation failed before/after execution.

## Compatibility rule

`ToolchainSpec::validate_for` remains the gate. A mismatch is rejected before the resolver or invoker can execute an external process.

## Command construction

Command construction is deterministic: the resolved executable is fixed, the target triple is emitted as a discrete `--target=<triple>` argument, source and output paths are discrete arguments, and output uses `-o <artifact>`. No shell quoting or interpolation is used.

## Test strategy

A fake toolchain is supplied by fixture shell-free executable test programs where the test platform permits it. The core boundary tests additionally use a platform-neutral Rust helper executable path and temporary files so they can prove:

- executable and argument preservation;
- missing executable classification;
- non-zero exit classification;
- target mismatch before execution;
- successful execution with missing artifact;
- successful execution with artifact present;
- high-level execution passes through resolver, invoker, and artifact validation;
- the high-level boundary rejects a non-zero `ToolchainInvocation` even when a custom invoker returns it as `Ok`;
- a pre-existing output path is rejected before resolution/execution so a stale artifact cannot satisfy post-build validation.

## Scope exclusions

- No parser/analyzer/IR changes.
- No new per-target Semantic IR.
- No GPIO/HAL/RTOS/firmware flashing.
- No toolchain installation, package management, network discovery, or implicit filesystem probing.
- No shell execution semantics.
- No unsupported platform claims beyond verified tests.
