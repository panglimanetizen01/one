# One v0.1 — Reference Interpreter Contract

**Status:** Draft normative contract for review

## 1. Purpose

The reference interpreter is the executable semantic oracle for One v0.1. It executes validated CSIR directly enough to make language behavior independently testable before compiler backends exist.

## 2. Required pipeline

```text
source
 → lexer/parser
 → AST
 → name resolution
 → type/semantic analysis
 → CSIR validation
 → reference interpreter
```

The interpreter must not bypass validation or reparse source text during execution.

## 3. Oracle rule

For every conforming program, the interpreter defines the expected observable result, structured error, and defined effect request sequence used by the v0.1 conformance suite.

Compiler backends must match these results rather than redefine semantics.

## 4. Inputs

The interpreter accepts validated CSIR, runtime configuration, capability policy, and explicitly supplied external inputs. It must reject malformed or semantically invalid CSIR before execution.

## 5. Outputs

Execution produces a structured result containing:

- completion state;
- return/value result where applicable;
- ordered observable outputs;
- structured runtime error when execution fails;
- effect requests/results visible through the test harness;
- provenance information when defined by CSIR.

The harness must distinguish semantic output from diagnostic/log output.

## 6. Deterministic mode

Conformance execution uses controlled clock, randomness, filesystem fixtures, network fixtures, and process stubs. Ambient external state is forbidden in deterministic tests.

## 7. Effect adapter interface

The interpreter does not directly embed arbitrary OS behavior into semantic execution. Effects are dispatched to explicit adapters after capability approval. Test adapters may return deterministic fixtures or failures.

## 8. State isolation

Each test execution starts from a clean runtime state unless a test explicitly defines retained state. A failed test must not contaminate later tests. Nested lexical scopes must be restored on both successful and failed block execution; a failed event dispatch must not leak local bindings into subsequent event dispatches.

## 9. Error fidelity

The interpreter must preserve stable error codes and source/provenance spans where the upstream contracts provide them. Host exceptions must be translated into defined One runtime errors rather than leaking implementation-specific exception types.

## 10. Trace mode

The implementation should support a structured execution trace for conformance/debugging. Trace records may include operation, source/provenance, effect request, and state transition identifiers, but must not expose secrets by default.

Trace output is diagnostic and is not semantic input.

## 11. Backend comparison

A backend comparison test runs identical validated CSIR through the reference interpreter and the backend, then compares:

1. completion/error category;
2. returned semantic value;
3. observable output;
4. defined effect sequence;
5. externally visible state changes defined by the test fixture.

Implementation-specific timing, memory addresses, stack layout, and allocation strategy are excluded.

## 12. Unsupported behavior

The interpreter must fail explicitly for CSIR operations outside the v0.1 contract. It must not silently approximate future features.

## 13. Acceptance criteria

- executes validated v0.1 CSIR;
- rejects invalid CSIR before execution;
- uses the capability gate for effects;
- supports deterministic test adapters;
- emits stable structured results/errors;
- provides enough trace data to diagnose semantic mismatches;
- serves as the oracle for compiler/backend conformance.