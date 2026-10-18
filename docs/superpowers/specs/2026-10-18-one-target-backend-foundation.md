# One Target Backend Foundation

**Status:** Implemented foundation specification

## Purpose

Establish the first target-independent compiler/backend boundary required for One to scale from hosted software to constrained, embedded, firmware, robotics, and programmable-device targets without changing One's language semantics.

## Architectural invariant

```text
One Source
    -> One Semantic IR
    -> Target-independent backend contract
    -> Target description / validation
    -> Target backend
    -> Runtime / firmware / platform
    -> Hardware / device
```

The target layer MUST NOT introduce a semantic fork for a CPU, board, RTOS, robot, appliance, or vendor.

## Scope of this slice

This slice establishes only the foundational target model and validation boundary.

It includes:

- target architecture identity;
- execution profile identity;
- target environment metadata;
- runtime class metadata;
- backend identity and declared target support;
- deterministic target validation;
- explicit unsupported-target diagnostics;
- tests proving the same semantic model can be validated independently of target selection.

It does NOT implement GPIO, I2C, SPI, UART, CAN, interrupts, RTOS integration, firmware flashing, a microcontroller SDK, or a concrete hardware target.

## Target model

A target description distinguishes at least:

- architecture (`x86_64`, `aarch64`, `arm`, and future architectures);
- execution profile (`hosted`, `constrained`, `firmware`);
- operating environment (`windows`, `linux`, `rtos`, `bare-metal`, or future environments);
- runtime class (`hosted`, `rtos`, `bare-metal`);
- backend identity.

The representation is extensible without changing One Semantic IR semantics. Runtime class is derived deterministically from the target environment in this foundation slice so the target model cannot contain an unvalidated runtime/environment pairing.

## Backend contract

A backend is represented by the `TargetBackend` trait. Each implementation declares:

- stable backend identity;
- supported target profiles;
- supported architectures;
- supported environments;
- required runtime class;
- deterministic validation of a target description;
- a boundary at which platform-specific implementation begins.

The reference implementation is `ReferenceTargetBackend`. It declares support for x86_64, AArch64, and ARM architectures; the hosted execution profile; Windows and Linux environments; and the hosted runtime class.

A backend MUST reject a target outside its declared support rather than silently selecting a fallback with different semantics.

## Diagnostics

Unsupported or malformed target descriptions produce deterministic, machine-testable diagnostics. Diagnostics identify the rejected target dimension and do not imply hardware support merely because a target name exists in metadata.

## Evidence states

Adding a target descriptor is not evidence that the concrete target is supported. Concrete support remains subject to executable build/runtime/device evidence as defined by `docs/COMPATIBILITY.md` and `docs/ONE_CONSTITUTION.md`.

## Testing contract

The implementation includes tests for:

1. valid hosted target acceptance;
2. valid constrained/firmware target description construction;
3. backend contract identity and required runtime declaration;
4. runtime class distinction in the target model;
5. unsupported architecture rejection;
6. unsupported execution profile rejection;
7. unsupported environment rejection;
8. deterministic backend mismatch and rejection diagnostics;
9. no semantic-IR mutation caused by target selection.

## Non-goals

- no hardware driver implementation;
- no device I/O API;
- no compiler machine-code backend;
- no cross-compilation toolchain installation;
- no claim of ESP32, ARM MCU, robot, TV, AC, refrigerator, or appliance support.
