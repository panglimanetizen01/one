# One Target Toolchain Foundation Specification

**Status:** Implemented on `docs/sync-target-backend-status`; executable verification remains required before merge.

## 1. Goal

Establish a target-independent toolchain abstraction that can describe how a `TargetSpec` is built or executed without coupling One Semantic IR to a compiler vendor, SDK, operating system, MCU, RTOS, or hardware platform.

The foundation must make toolchain identity, target-platform identity, artifact expectations, linker strategy, and execution/build capability explicit and deterministically validated against the existing target/backend model.

## 2. Architectural Boundary

The intended dependency direction is:

```text
Localized Source
    ↓
One Semantic IR
    ↓
TargetSpec
    ↓
ToolchainSpec
    ↓
TargetBackend
    ↓
Platform / Runtime / Firmware
```

`One Semantic IR` remains target-independent. Toolchain selection is metadata and validation, not a new semantic representation.

A toolchain abstraction must not become an alternative semantic or compiler pipeline. A backend may consume a compatible `ToolchainSpec`, but the toolchain model itself does not define language semantics.

## 3. Core Model

Introduce a deterministic `ToolchainSpec` model in the target/backend foundation layer.

The model must identify, at minimum:

- stable toolchain identity;
- target triple/platform identity;
- linker strategy;
- artifact format;
- execution/build capability.

The exact Rust representation must remain small, typed, deterministic, and independent of a particular external toolchain implementation.

### 3.1 Toolchain Identity

Toolchain identity is a stable descriptive identifier. It is not a filesystem path, executable discovery result, installation state, or mutable environment variable.

### 3.2 Target Triple / Platform Identity

The model must represent the platform identity expected by the toolchain independently from host execution. It must be possible to compare that identity deterministically with the target architecture/environment model.

### 3.3 Linker Strategy

The model must distinguish linker strategy as explicit metadata rather than assuming a single host linker. This is descriptive only; this slice does not execute a linker.

### 3.4 Artifact Format

The model must describe the expected output artifact format. This is descriptive only; this slice does not generate machine code or artifacts.

### 3.5 Execution / Build Capability

The model must explicitly distinguish whether a toolchain description represents build capability, execution capability, both, or neither, using a typed representation rather than implicit booleans scattered across callers.

## 4. Compatibility Contract

`TargetSpec` and `ToolchainSpec` compatibility must be deterministic.

Validation must reject at least:

1. toolchain target identity incompatible with the requested target architecture/environment;
2. toolchain execution/build capability incompatible with the requested operation;
3. backend/toolchain combination that is outside the backend's declared contract.

Diagnostics must identify the failed compatibility dimension deterministically. No validation may silently select a different target, runtime, backend, or toolchain.

The existing `TargetBackend` contract remains authoritative for backend identity and declared target support. This slice extends the boundary with toolchain compatibility; it does not weaken or bypass existing target validation. Runtime compatibility remains governed by the existing target/backend contract rather than introducing a second runtime rule.

## 5. Determinism

Given identical `TargetSpec`, `ToolchainSpec`, backend declaration, and requested operation, validation must produce the same result and diagnostic ordering on every run.

No environment probing, executable discovery, network lookup, filesystem scan, or host-specific fallback is permitted in the foundation validation path.

## 6. Semantic IR Invariance

Selecting or validating a toolchain must not mutate, fork, specialize, or otherwise alter One Semantic IR.

Tests must prove that the same semantic IR remains equivalent before and after target/toolchain selection metadata is applied.

## 7. Reference Toolchain

Provide a minimal reference toolchain representation for tests and contract validation. It is a model fixture, not a claim that a real compiler, linker, SDK, or cross-compilation environment is installed or supported.

The reference representation must use stable deterministic identifiers and must be sufficient to exercise both compatible and incompatible validation paths.

## 8. Explicit Non-Goals

This slice does **not** implement or claim:

- installation or discovery of external toolchains;
- automatic invocation of compilers or linkers;
- machine-code generation;
- real cross-compilation support;
- firmware flashing or packaging;
- hardware drivers;
- GPIO, I2C, SPI, UART, CAN, or other device I/O;
- concrete MCU/vendor SDK support;
- concrete RTOS integration;
- host-specific environment probing;
- a target-specific semantic IR.

Concrete target support requires separate evidence and follows the project's support-status rules.

## 9. Testing Contract

The implementation must include focused tests covering:

1. deterministic construction of `ToolchainSpec`;
2. stable toolchain identity;
3. stable target-platform identity;
4. explicit linker strategy and artifact format;
5. explicit build/execution capability;
6. compatible `TargetSpec` + `ToolchainSpec` validation;
7. deterministic target/toolchain incompatibility;
8. deterministic requested-operation capability mismatch;
9. backend/toolchain contract mismatch;
10. deterministic diagnostics and validation ordering;
11. semantic IR invariance across toolchain selection metadata;
12. reference fixture behavior without making real toolchain support claims.

The existing workspace contract, vertical tests, clippy policy, deterministic-artifact checks, and exact-SHA verification remain required for completion.

## 10. Documentation Requirements

Implementation documentation must state clearly that this is a toolchain abstraction foundation, not cross-compilation support. Any future concrete toolchain integration must add independent evidence for the specific toolchain, target, host, artifact, and execution path.

The next downstream work may evaluate actual cross-compilation/toolchain adapters only after this foundation is independently verified.
