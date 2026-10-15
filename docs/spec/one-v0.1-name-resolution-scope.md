# One v0.1 — Name Resolution and Scope Model

**Status:** Draft normative contract for review

## 1. Purpose

This document defines how identifiers represented by the v0.1 AST are declared, resolved, scoped, shadowed, and diagnosed.

It is the contract between the AST layer and semantic analysis. It does not define runtime storage, code generation, or IR lowering.

## 2. Normative principles

1. Resolution is deterministic.
2. A name is resolved only against declarations visible in its lexical scope.
3. The resolver never guesses between competing declarations.
4. Multi-word names are resolved as the single logical `Name` represented by the AST.
5. Declaration order is significant: a declaration is not visible before its declaration point unless a later language contract explicitly introduces hoisting.
6. Duplicate declarations in the same scope are errors.
7. An inner declaration may shadow an outer declaration only when the language construct explicitly permits it; v0.1 permits lexical shadowing.
8. A reference with no visible declaration is an error.
9. Resolution does not perform type inference, capability checks, execution, filesystem access, network access, or LLM calls.
10. Diagnostics must identify the reference/declaration spans involved when available.

## 3. Scope model

v0.1 uses lexical scopes.

A scope is created by:

- the program/module root;
- each function body;
- each nested block that introduces declarations or bindings;
- each loop body for its loop binding;
- each event body for event-local bindings, when such bindings are defined by the event contract.

A nested scope has exactly one lexical parent.

Resolution searches the current scope first, then its parent, continuing until the root. The first matching declaration wins because lexical scope depth is deterministic.

## 4. Declaration visibility

A declaration becomes visible immediately after its declaration statement.

Example:

```text
nama adalah Budi
sapa nama
```

The reference to `nama` resolves to the preceding declaration.

This is invalid:

```text
sapa nama
nama adalah Budi
```

because `nama` is not visible at the reference point.

No forward-reference exception exists in v0.1.

## 5. Declaration identity

The resolver compares logical names, not raw token fragments.

```text
nama depan adalah Budi
```

creates one declaration whose logical name consists of the words `nama` and `depan`.

A later reference using the same grammar-bounded name resolves to that declaration:

```text
tampilkan nama depan
```

The resolver must not split `nama depan` into two unrelated identifiers.

Name spelling comparison is exact for v0.1. Case-folding, Unicode normalization, aliases, and locale-specific equivalence require an explicit later contract.

## 6. Declaration categories

The following AST constructs introduce names:

- `Declare.name`
- `Function.name`
- `Parameter.name`
- `ForEach.binding`
- future explicitly specified bindings

A declaration category may impose additional semantic restrictions, but all declarations enter the same lexical-resolution framework.

## 7. Function scope

A function declaration creates a function name in the containing scope.

A function body creates a child scope containing its parameters and body-local declarations.

Parameters are visible throughout the function body after parameter binding is established.

Example:

```text
buat pekerjaan hitung total
    terima harga
    terima pajak
    hasilnya adalah harga ditambah pajak
```

`harga` and `pajak` resolve to parameters inside the function body.

A parameter name duplicated within the same parameter list is an error.

## 8. Block scope

Nested blocks have lexical scope. A declaration inside a nested block is not visible after leaving that block.

Example shape:

```text
kalau umur sudah 17 tahun
    status adalah dewasa
    tampilkan status

tampilkan status
```

The final `status` reference is unresolved unless another visible declaration exists outside the condition block.

The exact set of statement forms that introduce a block is determined by the AST contract and future grammar contracts; this document does not invent new surface syntax.

## 9. Loop binding scope

A `ForEach.binding` is visible inside its loop body and is not visible after the loop.

Example:

```text
untuk setiap teman di daftar teman
    tampilkan teman

tampilkan teman
```

The final reference to `teman` is unresolved unless another declaration is visible.

The loop binding does not retroactively become visible in the collection expression.

## 10. Shadowing

v0.1 permits lexical shadowing in nested scopes.

Example:

```text
nama adalah Budi

kalau benar
    nama adalah Siti
    tampilkan nama

tampilkan nama
```

Inside the nested scope, `nama` resolves to `Siti`. Outside that scope, `nama` resolves to `Budi`.

A declaration that shadows an outer declaration is not a duplicate declaration because the declarations belong to different scopes.

The implementation must preserve enough source information for diagnostics and tooling to distinguish the declarations.

## 11. Duplicate declarations

Two declarations of the same logical name in one scope are rejected.

Example:

```text
nama adalah Budi
nama adalah Siti
```

The resolver must report a duplicate declaration error rather than silently replacing the first declaration.

A later declaration in the same scope does not override an earlier declaration.

## 12. Unresolved references

A reference is invalid when no declaration with the same logical name is visible at the reference point.

Example:

```text
tampilkan nama yang belum ada
```

If the grammar produces a reference to `nama yang belum ada`, and no matching declaration exists, semantic analysis reports an unresolved-name error.

The resolver must not create an implicit declaration to make the program valid.

## 13. Ambiguous resolution

If the semantic model ever permits multiple visible declarations with the same logical name at the same resolution depth, resolution must fail with an ambiguity diagnostic rather than choosing by insertion order, source order, popularity, fuzzy similarity, or external state.

The v0.1 scope rules are designed so ordinary lexical scopes should not produce this state; the explicit rule exists as a safety invariant for future extensions.

## 14. Resolution result

Conceptually, resolution produces one of:

```text
Resolved(declaration_id)
Unresolved(reference_span, logical_name)
Duplicate(declaration_span, previous_declaration_span, logical_name)
Ambiguous(reference_span, candidate_declarations)
```

`declaration_id` is an internal semantic-analysis identity. It is not a source-level identifier and is not part of the surface language.

The resolver must not mutate the source AST spelling to resolve a name.

## 15. Diagnostics

Recommended v0.1 diagnostic categories:

- `ENAME001` — name not found / unresolved reference
- `ENAME002` — duplicate declaration in the same scope
- `ENAME003` — ambiguous name resolution
- `ENAME004` — invalid declaration/binding position

Diagnostics should include:

1. stable error code;
2. primary source span;
3. concise explanation;
4. related declaration span when relevant;
5. a concrete valid-resolution hint when one exists.

Diagnostics must not invent a correction that changes the program's intended semantics.

## 16. Determinism

For identical source text and identical language version/localization contract, name resolution must produce identical results.

Resolution must not depend on:

- hash-map iteration order;
- filesystem ordering;
- environment variables;
- process timing;
- network state;
- execution history;
- editor state;
- machine-local history;
- ML/LLM output.

## 17. Semantic boundary

Name resolution answers only: **which declaration, if any, does this reference denote?**

It does not answer:

- whether the value has the correct type;
- whether an operation is allowed by capability policy;
- whether a function call has valid argument types;
- how a value is represented at runtime;
- how the program is lowered to semantic IR;
- whether an external module/package exists, except insofar as a later module-resolution contract explicitly supplies declarations to the lexical resolver.

## 18. Acceptance tests

The conformance suite must cover at least:

1. single-word declaration and reference;
2. multi-word declaration and reference;
3. declaration-before-use;
4. use-before-declaration rejection;
5. nested lexical lookup;
6. inner shadowing;
7. outer visibility after nested scope ends;
8. duplicate declaration rejection in one scope;
9. duplicate parameter rejection;
10. function parameter visibility;
11. loop-binding visibility;
12. loop-binding non-visibility outside the loop;
13. unresolved-name diagnostics;
14. deterministic repeated resolution;
15. exact spelling behavior;
16. no implicit declarations;
17. ambiguity rejection if a future extension can create it.

## 19. Examples

### Valid

```text
nama lengkap adalah Budi Santoso
tampilkan nama lengkap
```

```text
nama adalah Budi
kalau benar
    nama adalah Siti
    tampilkan nama

tampilkan nama
```

### Invalid

```text
tampilkan nama
nama adalah Budi
```

```text
nama adalah Budi
nama adalah Siti
```

```text
buat pekerjaan hitung
    terima nilai
    terima nilai
    hasilnya adalah nilai
```

## 20. Implementation gate

Parser implementation must consume the existing AST contract and must not embed semantic symbol-table decisions.

Semantic-analysis implementation may add declaration identities, scope objects, symbol tables, and resolution maps, but those are downstream of the AST.

The implementation is accepted only when:

- all normative rules above have executable conformance tests;
- diagnostics preserve source spans;
- multi-word names follow the name-boundary contract;
- no implicit or heuristic resolution exists;
- repeated runs are deterministic;
- the reference interpreter/compiler consumes the same semantic resolution contract.
