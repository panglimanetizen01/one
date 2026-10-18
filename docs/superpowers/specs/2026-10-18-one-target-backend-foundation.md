# One Target Backend Foundation

**Status:** Proposed implementation specification

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
- backend identity and declared target support;
- deterministic target validation;
- explicit unsupported-target diagnostics;
- tests proving the same semantic model can be validated independently of target selection.

It does NOT implement GPIO, I2C, SPI, UART, CAN, interrupts, RTOS integration, firmware flashing, a microcontroller SDK, or a concrete hardware target.

## Target model

A target description must distinguish at least:

- architecture (`x86_64`, `aarch64`, and future architectures);
- execution profile (`hosted`, `constrained`, `firmware`);
- operating environment (`windows`, `linux`, `rtos`, `bare-metal`, or future environments);
- backend identity;
- runtime requirements.

The representation must be extensible without changing One Semantic IR semantics.

## Backend contract

A backend declares:

- stable backend identity;
- supported target profiles/architectures;
- required runtime class;
- deterministic validation of a target description;
- a boundary at which platform-specific implementation begins.

A backend MUST reject a target outside its declared support rather than silently selecting a fallback with different semantics.

## Diagnostics

Unsupported or malformed target descriptions must produce deterministic, machine-testable diagnostics. Diagnostics must identify the rejected target dimension and must not imply hardware support merely because a target name exists in metadata.

## Evidence states

Adding a target descriptor is not evidence that the concrete target is supported. Concrete support remains subject to executable build/runtime/device evidence as defined by `docs/COMPATIBILITY.md` and `docs/ONE_CONSTITUTION.md`.

## Testing contract

The implementation must include tests for:

1. valid hosted target acceptance;
2. valid constrained/firmware target description construction where the model permits it;
3. unsupported architecture rejection;
4. unsupported execution profile rejection;
5. deterministic diagnostics;
6. backend declaration matching target validation;
7. no semantic-IR mutation caused by target selection.

## Non-goals

- no hardware driver implementation;
- no device I/O API;
- no compiler machine-code backend;
- no cross-compilation toolchain installation;
- no claim of ESP32, ARM MCU, robot, TV, AC, refrigerator, or appliance support.
