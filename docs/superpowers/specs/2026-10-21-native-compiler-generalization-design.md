# Gate 2 — Native Compiler Generalization Design

## Goal

Generalize native compilation from the current declaration/assignment/output subset to the six primary ONE construct families: DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT, while preserving one target-independent Semantic IR and the Gate 1 toolchain boundary.

## Binding Architecture

```text
ONE SOURCE
  -> PARSER
  -> ANALYZER
  -> ONE SEMANTIC IR
  -> SHARED NATIVE C GENERATOR
  -> TARGET BACKEND
  -> GATE 1 TOOLCHAIN
  -> NATIVE ARTIFACT
```

The native compiler uses one shared semantic-to-C lowering/emission implementation. Target backends provide target metadata and toolchain configuration; they do not define separate semantic IRs or construct semantics.

## Construct Contract

### DATA

Declarations and assignments remain represented by `IrStatement::Declaration` and `IrStatement::Assignment`. Native emission must preserve the IR type information required by the supported scalar literals and references and reject invalid/unsafe C mappings deterministically.

### ACTION

Native emission must provide a real path for the supported action family represented by `IrStatement::Action`. Existing special orchestration actions remain governed by their existing orchestration IR/runtime contract; native compilation must not silently reinterpret them as unrelated native operations. Unsupported action names or invalid arity must produce structured deterministic diagnostics.

### CONDITION

`IrStatement::Conditional` emits native conditional control flow. Conditions use the existing `IrExpr` semantics and must reject expressions/types that cannot be emitted safely.

### LOOP

`IrStatement::Loop` emits native control flow for the existing `IrLoopKind::ForEach` and `IrLoopKind::While` variants. The implementation must preserve variable/reference semantics and reject unsupported iterable/type combinations deterministically.

### FUNCTION

`IrStatement::Function` emits a real native function boundary and `IrExpr::Call` emits a native call. Parameter, result, local-variable, and return semantics must remain target-independent. Function declarations must not be emitted as nested C functions.

### EVENT

`IrStatement::Event` emits a native event handler plus deterministic dispatch metadata. The initial event model is handler + dispatcher: declaring an event does not implicitly execute it. Event execution occurs through an explicit native dispatcher path corresponding to the existing semantic event model.

## Type and Expression Contract

The existing `IrExpr` remains authoritative. Gate 2 may extend native type mapping only where required by the existing Semantic IR; it must not introduce a native-only semantic representation. Literal/reference/binary/call emission must be deterministic. Invalid type combinations, undefined references, unsupported expressions, and unsafe identifiers produce structured `NativeBackendError` diagnostics.

## Target Contract

The same semantic/native generator is reused for Linux AArch64, Linux x86_64, Windows x86_64, and Android AArch64 backends. Target-specific behavior is restricted to `TargetSpec`, toolchain specification, compiler target arguments, linker resolution, and artifact validation.

## Artifact Contract

Each construct-family integration test must prove that compilation reaches a native artifact. Tests may inspect generated C/source where execution is unavailable, but artifact existence and artifact validation remain mandatory for build-success claims. Platform execution claims require real platform/toolchain evidence.

## Negative Contract

The following must fail deterministically with structured diagnostics:

- unsupported native construct/action;
- invalid type mapping or incompatible assignment/condition/operation;
- target/backend mismatch;
- invalid IR lowering input;
- missing/invalid native artifact.

## Determinism

For identical Semantic IR and target specification, generated native source and command construction must be byte-for-byte/deterministically equivalent. No target-specific semantic ordering or hidden filesystem discovery may influence generated semantics.

## Regression Boundary

Gate 2 must preserve existing parser, analyzer, Semantic IR, interpreter, orchestration, target-model, and Gate 1 toolchain behavior. No Gate 2 change may create a second semantic model for a target.

## Verification

Required evidence:

- RED tests for each construct family before implementation;
- DATA, ACTION, CONDITION, LOOP, FUNCTION, EVENT each reaching native compilation/artifact validation;
- negative tests for unsupported construct, invalid type, invalid target/backend, invalid lowering, invalid artifact;
- deterministic native output;
- `cargo fmt --check` or scoped formatter verification when repository-wide formatting contains unrelated pre-existing failures;
- relevant tests;
- `cargo test -p one-core`;
- `cargo clippy -p one-core --all-targets --offline -- -D warnings`;
- native artifact verification;
- `git diff --check`;
- Gate 1 focused regression remains green;
- documentation and final diff audit.

## Explicit Non-Goals

- no new Semantic IR per target;
- no replacement of the existing Semantic IR with backend-specific semantics;
- no firmware flashing, GPIO/HAL, RTOS, or Gate 3 work;
- no interpreter semantic rewrite;
- no unsupported platform execution claims.
