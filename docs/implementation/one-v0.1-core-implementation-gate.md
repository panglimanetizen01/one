# One v0.1 — Core Implementation Gate

**Status:** Normative implementation gate

## Purpose

This gate converts the approved v0.1 contracts into an implementation sequence without changing their semantics.

## Canonical pipeline

`source → lexer → parser → AST → name/scope analysis → type/semantic analysis → CSIR → reference interpreter`

## Rules

1. Each stage consumes only the contract immediately before it.
2. No stage reparses source after parsing.
3. No stage uses an LLM, network service, filesystem ordering, or execution history for semantic decisions.
4. Invalid input must stop at the earliest applicable stage with a stable diagnostic.
5. CSIR is the boundary between language semantics and execution.
6. The reference interpreter is the executable semantic oracle.
7. Platform adapters and future compiler backends must conform to the interpreter and conformance suite.
8. Existing implementation behavior must not silently redefine normative contracts; discrepancies become implementation fixes or explicit contract revisions.

## Implementation order

1. Lexer/token invariants and source spans.
2. Parser and AST construction.
3. Name resolution and lexical scopes.
4. Type checking and semantic validation.
5. CSIR lowering.
6. Capability/effect enforcement.
7. Reference interpreter.
8. Layered conformance fixtures.
9. CLI integration.
10. Platform adapters and packaging.

## Required evidence

Every implementation stage must have deterministic unit tests and at least one end-to-end fixture through the complete pipeline. No stage is considered complete solely because it compiles.

## Compatibility rule

The implementation may add internal data structures, optimizations, diagnostics helpers, or adapters, but observable behavior must remain within the approved v0.1 contracts.
