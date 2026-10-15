# One v0.1 — AST Contract

**Status:** Draft normative contract for review
**Depends on:** Language Invariants, lexical specification, concrete grammar, name-boundary contract
**Scope:** Parser output and source-preserving structural representation

## 1. Purpose

The Abstract Syntax Tree (AST) is the stable structural boundary between parsing and semantic analysis.

The AST MUST represent the structure established by the concrete grammar without retaining unresolved grammar ambiguity. It MUST preserve enough source information for deterministic diagnostics and later tooling.

The AST is not the runtime IR. It is a source-oriented structural representation. Semantic analysis resolves names, types, capabilities, and other meaning after the AST is constructed.

## 2. Core invariants

The v0.1 AST MUST satisfy all of the following:

1. Every accepted source statement maps to exactly one normalized AST structure.
2. No AST node may encode an unresolved choice between different semantic parses.
3. A grammar-level multi-word name is represented as one logical `Name`, not as unrelated expression nodes.
4. The complete source span of a multi-word name is preserved.
5. Lexical spelling is preserved sufficiently for diagnostics and source tooling.
6. AST construction does not perform symbol-table lookup or type inference.
7. AST construction does not execute code.
8. AST construction does not call an LLM or network service.
9. Unsupported conversational prose does not receive an AST node.
10. Equivalent grammar parses that normalize to the same semantic structure produce the same AST shape.

## 3. Source locations

Every externally visible AST node MUST have a source span.

A span is half-open:

```text
Span = [start, end)
```

Each endpoint MUST identify at least:

```text
line
column
byte offset
```

The implementation MAY additionally retain UTF-8 character offsets or token indices.

For:

```text
nama pengguna adalah Budi
```

the `Name("nama pengguna")` span MUST cover both `nama` and `pengguna`, but MUST NOT include `adalah`.

## 4. Name node

The canonical source-level name node is:

```text
Name {
    words: [NameWord, ...],
    span: Span
}

NameWord {
    text: String,
    span: Span
}
```

A name MUST contain at least one word.

The AST MUST NOT represent a multi-word name by concatenating arbitrary source text and losing its component spans.

Example:

```text
nama pengguna
```

normalizes to one logical name:

```text
Name {
    words: ["nama", "pengguna"]
}
```

not a binary expression and not an unresolved text blob.

## 5. Program and statement nodes

Minimum structure:

```text
Program {
    statements: [Statement],
    span: Span
}
```

`Statement` is one of:

```text
Declare
Assign
Action
Condition
Loop
Function
Event
```

No statement node may represent unsupported or unresolved source.

## 6. Data nodes

### Declaration

```text
Declare {
    name: Name,
    value: Expression,
    span: Span
}
```

Source:

```text
nama pengguna adalah "Budi"
```

produces one `Declare` with `Name(["nama", "pengguna"])` and `Text("Budi")`.

The word `adalah` is grammar syntax and is not part of the `Name` node.

### Assignment

```text
Assign {
    name: Name,
    value: Expression,
    span: Span
}
```

Source:

```text
saldo rekening menjadi 100000
```

produces one `Assign` with `Name(["saldo", "rekening"])` and an integer expression.

## 7. Action nodes

The v0.1 core contains explicit actions only:

```text
Action = Display
       | CreateList
       | AddToList
       | RemoveFromList
```

### Display

```text
Display {
    value: Expression,
    span: Span
}
```

### CreateList

```text
CreateList {
    name: Name,
    span: Span
}
```

### AddToList

```text
AddToList {
    value: Expression,
    target: Name,
    span: Span
}
```

### RemoveFromList

```text
RemoveFromList {
    value: Expression,
    target: Name,
    span: Span
}
```

There is deliberately no `GenericAction` node in v0.1. An arbitrary first word cannot become an executable action merely because a matching identifier exists.

## 8. Condition nodes

```text
Condition {
    test: ConditionExpression,
    then_branch: Block,
    else_branch: Option<Block>,
    span: Span
}

Comparison {
    operator: ComparisonOperator,
    left: Expression,
    right: Expression,
    span: Span
}
```

`ComparisonOperator` contains only operators admitted by the concrete grammar, including `sudah`, `belum`, `sama dengan`, `lebih besar dari`, and `lebih kecil dari`.

The AST MUST preserve operator identity. It MUST NOT store only the original phrase and defer interpretation to runtime.

## 9. Loop nodes

```text
Loop = ForEach | While
```

### ForEach

```text
ForEach {
    binding: Name,
    source: Name,
    body: Block,
    span: Span
}
```

### While

```text
While {
    test: ConditionExpression,
    body: Block,
    span: Span
}
```

The conversational phrase `selama masih ada ...` is not represented because it is outside v0.1.

## 10. Function nodes

```text
Function {
    name: Name,
    parameters: [Parameter],
    result: Expression,
    body: [Statement],
    span: Span
}

Parameter {
    name: Name,
    span: Span
}
```

The parser records syntactic function structure only. It does not decide whether a function name is defined, duplicated, or callable.

A function call is an expression:

```text
Call {
    function: Name,
    arguments: [Expression],
    span: Span
}
```

The `dengan` / `dan` words are grammar syntax and are not executable operation nodes.

## 11. Event nodes

```text
Event {
    trigger: EventTrigger,
    body: Block,
    span: Span
}
```

`EventTrigger` MUST retain its parsed components and source spans. It MUST NOT contain a free-form semantic intent generated from prose.

The final event-trigger semantic model is a later dependency. This AST contract therefore requires structured source representation and exact span preservation; runtime dispatch semantics belong to later semantic/runtime specifications.

## 12. Expression nodes

Minimum expression model:

```text
Expression = Text
           | Integer
           | Boolean
           | Nothing
           | Reference
           | Call
           | Binary
```

### Text

```text
Text {
    value: String,
    span: Span
}
```

### Integer

```text
Integer {
    value: implementation-neutral integer literal representation,
    span: Span
}
```

Exact integer width is a later type-system decision and MUST NOT be silently assumed here.

### Boolean

```text
Boolean {
    value: true | false,
    span: Span
}
```

### Nothing

```text
Nothing {
    span: Span
}
```

### Reference

```text
Reference {
    name: Name,
    span: Span
}
```

A `Reference` is syntactic. It does not prove that the referenced name exists.

### Binary

```text
Binary {
    operator: BinaryOperator,
    left: Expression,
    right: Expression,
    span: Span
}
```

`BinaryOperator` is an explicit enum for the grammar's arithmetic operators. Precedence and associativity are parser contracts; type validity belongs to semantic/type analysis.

## 13. Blocks

```text
Block {
    statements: [Statement],
    span: Span
}
```

A block MUST contain at least one statement under the current concrete grammar.

Indentation is structural syntax and is represented by block nesting rather than executable indentation nodes.

## 14. AST normalization

The parser MUST normalize syntactic variants that are explicitly equivalent under the grammar into one semantic AST shape.

The parser MUST NOT normalize distinct semantic meanings into one AST shape merely because their surface text looks similar.

For example, `nama pengguna` and `nama` remain distinct names when both are valid references.

The complete `Name.words` sequence MUST be preserved.

## 15. Ambiguity handling

The AST layer receives input only after concrete parsing has established a unique semantic parse.

Therefore:

- zero parses → parser diagnostic; no AST;
- one semantic parse → construct one AST;
- multiple equivalent parses → normalize to one AST;
- multiple different semantic parses → `EAMBxxx`; no AST.

An AST MUST NEVER contain an `Ambiguous` node as a substitute for rejecting unresolved source in v0.1.

## 16. Invalid AST cases

The following source forms MUST produce no AST:

```text
apel
```

because no core action or statement production accepts it.

```text
nama menjadi
```

because the assignment has no expression.

```text
kalau umur 17
    tampilkan "..."
```

because the comparison operator is missing.

```text
selama masih ada barang di daftar belanja
    tampilkan barang
```

because the conversational loop-condition extension is outside v0.1.

```text
tambahkan apel entah ke mana
```

because the target does not satisfy an explicit production boundary.

A parser MUST NOT create a partial AST and let later semantic analysis guess missing structure.

## 17. Source-span acceptance tests

The test suite MUST verify:

1. single-word name spans cover exactly one word;
2. multi-word name spans cover every component word;
3. structural keywords are excluded from name spans;
4. declaration, assignment, action, condition, loop, function, event, and expression nodes have enclosing spans;
5. nested block spans are stable;
6. literal spans follow the implementation's documented delimiter convention;
7. repeated parsing preserves identical spans;
8. malformed input does not produce a partial public AST.

## 18. Structural acceptance tests

The conformance suite MUST establish at minimum:

### Valid

```text
nama pengguna adalah "Budi"
```

→ one `Declare` with a two-word `Name` and `Text` value.

```text
saldo rekening menjadi 100000
```

→ one `Assign` with a two-word `Name` and integer value.

```text
tambahkan apel ke daftar buah
```

→ one `AddToList` with an expression value and two-word target `Name`.

```text
untuk setiap teman di daftar teman
    tampilkan teman
```

→ one `ForEach` with a two-word source `Name` and nested `Block`.

### Invalid

The suite MUST reject all invalid forms in section 16 and verify that no ambiguous source can be represented as a valid AST.

## 19. Semantic boundary

The AST does NOT decide:

- whether a name is defined;
- whether two names collide;
- whether an expression has a valid type;
- whether an operation requires a capability;
- whether a function call is valid for its target function;
- whether an event can be registered;
- how a value is represented at runtime;
- how the AST lowers to common semantic IR.

Those decisions belong to subsequent semantic, type, capability, and IR contracts.

## 20. Determinism requirement

For identical normalized source and identical grammar version, parsing MUST produce structurally identical AST output, including node kinds, name-word ordering, and source spans.

AST output MUST NOT depend on machine learning, LLM output, network access, filesystem enumeration order, process timing, prior execution history, or editor state.

## 21. Compatibility and implementation gate

This AST contract becomes normative after review acceptance.

Before parser implementation is updated to claim conformance:

1. lexical and concrete grammar gates must agree;
2. implementation types must represent the AST schema;
3. parser tests must cover valid, invalid, boundary, ambiguity, and span cases;
4. conformance fixtures must assert AST structure, not only successful parsing;
5. downstream semantic analysis must consume this AST rather than reparsing source text.

## 22. Design decision

**Proposed v0.1 AST:** deterministic, source-oriented structure with explicit `Name` nodes, complete source spans, explicit action nodes, structured control-flow nodes, and no unresolved ambiguity node.

Principle:

> Parse once into a stable structure; perform semantic meaning after the structure is unambiguous.
