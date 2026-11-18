# One v0.1 — Common Semantic IR Contract

**Status:** Draft normative contract for review

## 1. Purpose

The Common Semantic IR (CSIR) is the platform-neutral representation between validated language semantics and execution backends.

Its purpose is to ensure that the reference interpreter, compiler backends, platform runtimes, and localized frontends consume one semantic meaning rather than independently interpreting source syntax.

## 2. Pipeline position

```text
Localized Source
  -> Lexer
  -> Parser
  -> AST
  -> Name Resolution
  -> Type Analysis
  -> Semantic Analysis
  -> CSIR
  -> Interpreter / Compiler Backend
```

Only semantically validated programs may enter CSIR.

## 3. Core invariants

1. CSIR is language-semantic, not platform-specific.
2. Every CSIR node has a source span or an explicit generated-node provenance.
3. CSIR contains resolved declaration identities, not unresolved source names.
4. CSIR contains semantic types where required by the operation.
5. CSIR contains no localization-dependent keyword spelling.
6. CSIR contains no parser ambiguity.
7. CSIR construction is deterministic.
8. CSIR construction performs no execution.
9. CSIR does not require an LLM, network, or filesystem state.
10. Backends must not reinterpret CSIR into different language semantics.

## 4. Program structure

Conceptual top-level form:

```text
Program {
  declarations: [Declaration]
  body: [Operation]
  events: [Event]
}
```

The exact serialization format is implementation-defined until a stable interchange format is required. The semantic structure is normative.

## 5. Declaration IR

A declaration has:

```text
Declaration {
  id: DeclarationId
  name: LogicalName
  type: Type
  scope: ScopeId
  initializer: ValueExpression?
  span: Span
}
```

`DeclarationId` is stable only within one CSIR program instance. It is not a source-level identifier.

Function declarations additionally contain parameter declarations, result type, and body operations.

## 6. Value expressions

Minimum CSIR expression forms:

```text
Const(value, type)
Load(declaration_id, type)
Binary(operator, left, right, type)
Call(function_id, arguments, result_type)
```

Expressions must be typed and resolved before emission.

## 7. Operations

Minimum operation set:

```text
Declare
Assign
Display
CreateList
AddToList
RemoveFromList
Condition
ForEach
While
FunctionCall
Return
EventRegister
```

Each operation carries source provenance.

## 8. Control flow

CSIR may represent structured control flow directly or use basic blocks/terminators internally. The normative meaning is:

- `Condition` selects exactly one branch based on a Boolean value;
- `ForEach` processes list elements in semantic iteration order;
- `While` reevaluates its Boolean condition before each iteration;
- `Return` exits the current function and supplies the declared result where applicable.

A later backend representation may lower structured control flow to basic blocks, but the lowering must preserve these semantics.

## 9. Evaluation order

CSIR preserves left-to-right evaluation order for expression operands and function arguments unless a future optimization contract proves an equivalent transformation.

The reference interpreter must execute CSIR in this semantic order.

## 10. Types

CSIR uses the language type identities defined by the v0.1 type-system contract:

```text
Nothing
Boolean
Integer
Number
Text
List<T>
Function([...], R)
```

Backend-specific representations must not redefine these language types.

## 11. References

A CSIR reference is a resolved `DeclarationId` or function identity.

There is no unresolved identifier node in valid CSIR.

This makes name-resolution failures impossible to defer into backend execution.

## 12. Side effects and capabilities

CSIR distinguishes pure value operations from effectful operations.

Effectful operations carry the semantic effect category required by the future capability contract.

No backend may acquire a stronger capability than the operation requires merely because the host platform exposes it.

The v0.1 effect categories are intentionally minimal and are finalized by the separate effect/capability contract.

## 13. Events

An event registration contains:

```text
EventRegister {
  event_id
  trigger
  handler
  span
}
```

The trigger remains semantic data. Platform-specific event APIs belong below CSIR.

## 14. Errors and invalid IR

A compiler/interpreter must reject construction of invalid CSIR.

Invalid CSIR includes:

- unresolved declaration references;
- missing required type information;
- incompatible operation operands;
- invalid control-flow structure;
- duplicate declaration identity within one scope;
- malformed event registration;
- unsupported operation.

A backend must not attempt to repair invalid CSIR heuristically.

## 15. Source provenance

Every user-visible diagnostic originating after lowering must be able to map back to source provenance where a source construct exists.

Generated helper nodes may carry provenance to the source operation that caused their creation.

The IR must preserve enough provenance to support debugging, diagnostics, and conformance failures.

## 16. Serialization boundary

v0.1 does not require CSIR to be a public serialized file format.

If serialization is later exposed, it must define:

- versioning;
- schema compatibility;
- type encoding;
- declaration identity encoding;
- source-span encoding;
- unknown-node behavior;
- deterministic ordering.

No backend may depend on an undocumented in-memory layout as the language-level contract.

## 17. Backend independence

A backend may choose any internal representation after consuming CSIR, including bytecode, native code, a VM graph, or direct interpretation.

The backend must preserve:

- observable values;
- operation ordering;
- control-flow semantics;
- function results;
- list behavior;
- event registration semantics;
- capability boundaries;
- diagnostics/provenance where required.

Platform-specific behavior must remain below the common semantic boundary.

## 18. Reference interpreter role

The reference interpreter executes CSIR without changing its semantics.

It is the executable oracle for conformance tests.

Compiler and platform backends must be tested against the same CSIR fixtures and observable results.

The reference interpreter is not permitted to implement source-language parsing as a separate semantic path.

## 19. Determinism

Given identical validated semantics, the same language version, and the same explicit inputs, CSIR generation must be byte-for-byte or structurally deterministic according to the chosen representation.

No randomness, network state, filesystem enumeration, environment state, machine locale, execution history, or LLM output may affect CSIR meaning.

## 20. Acceptance tests

The conformance suite must cover at least:

1. source-to-CSIR structure for each core construct;
2. multi-word name resolution represented by one declaration identity;
3. literal and reference typing;
4. arithmetic operation lowering;
5. condition control flow;
6. for-each and while control flow;
7. function definition/call/return;
8. list operations;
9. event registration;
10. left-to-right evaluation preservation;
11. source-span provenance;
12. rejection of unresolved/ill-typed input before CSIR;
13. deterministic repeated CSIR generation;
14. equivalent localized source producing equivalent CSIR;
15. backend-independent conformance fixtures.

## 21. Implementation gate

CSIR implementation may begin only after the semantic model, type system, name resolution, AST, grammar, and localization contracts are stable enough to serve as inputs.

The reference interpreter is the first consumer. Compiler backends must be implemented against CSIR rather than directly against localized AST syntax.

The runtime model and effect/capability contracts are now separately documented in docs/spec/one-v0.1-runtime-model.md and docs/spec/one-v0.1-effect-capability.md. This original dependency note is historical; consult docs/spec/README.md and the current implementation-status record for live gate state.
