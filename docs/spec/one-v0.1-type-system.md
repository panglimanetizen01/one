# One v0.1 — Type System

**Status:** Draft normative contract for review

## 1. Purpose

This document defines the v0.1 static type model used by semantic analysis. It builds on the AST and name-resolution contracts and precedes the full semantic specification and semantic IR.

## 2. Principles

1. Type checking is deterministic.
2. Types are semantic properties, not surface-language keywords alone.
3. No implicit type-changing guess is permitted.
4. Invalid operations are rejected before execution.
5. Type inference may derive a type only from explicit language rules.
6. Runtime values must preserve the semantic type guarantees established by analysis.
7. The type system has no dependency on an LLM, network, filesystem state, or execution history.

## 3. v0.1 primitive types

The initial semantic type universe is:

```text
Nothing
Boolean
Integer
Number
Text
```

`Nothing` represents the absence of a value produced by a statement/action that has no result.

`Boolean` represents logical truth values.

`Integer` represents whole-number values.

`Number` represents numeric values that may contain a fractional component.

`Text` represents Unicode text values.

## 4. Collection types

v0.1 defines a homogeneous list type:

```text
List<T>
```

where every element has the same semantic type `T`.

A list may be empty. An empty list declaration requires an explicit element type from a later declaration/type annotation rule; the analyzer must not infer an arbitrary element type from absence of elements.

Examples of semantic types:

```text
List<Text>
List<Integer>
List<Number>
List<Boolean>
```

Nested lists are structurally valid where the grammar and semantic operation permit them, for example `List<List<Text>>`.

## 5. Function types

A function has a parameter type sequence and a result type:

```text
Function([T1, T2, ...], R)
```

A function that produces no value has result type `Nothing`.

The surface syntax for declaring and calling functions is governed by the grammar contract; this document defines only the semantic type relation.

## 6. Event types

Events are not ordinary values in v0.1. An event declaration establishes an event handler with a trigger and a body.

The exact trigger/value model is deferred to the event semantic contract. An event handler must not be treated as a normal `Function` unless a later contract explicitly defines that conversion.

## 7. Type identity

Primitive types are nominally distinct:

```text
Integer != Number
Text != Boolean
Boolean != Integer
```

Parameterized collections are invariant in v0.1:

```text
List<Integer> != List<Number>
```

No inheritance, subtyping, union types, nullable types, generics beyond list parameterization, or user-defined nominal types are introduced by this contract.

## 8. Literal typing

AST literals have fixed types:

```text
Text literal    -> Text
Integer literal -> Integer
Boolean literal -> Boolean
Nothing literal -> Nothing
```

The concrete syntax for a `Number` literal is a grammar concern; when parsed as a fractional numeric literal it has type `Number`.

## 9. References

A reference expression receives the type of its resolved declaration.

If resolution fails, type checking does not invent a type. The diagnostic from name resolution remains authoritative for that failure.

## 10. Assignment compatibility

An assignment is valid only when the assigned expression type is compatible with the declared target type.

For v0.1, compatibility is exact except for the explicit numeric promotion:

```text
Integer -> Number
```

No reverse conversion is implicit.

Examples:

```text
Number x
x = 10              // valid: Integer promotes to Number
Integer y
 y = 10.5            // invalid: Number does not implicitly narrow to Integer
```

The examples above describe semantic behavior; exact One surface syntax for explicit type declarations is deferred to the grammar/type-annotation contract.

## 11. Numeric operations

The verbal arithmetic operators defined by the grammar operate according to these rules:

```text
Integer + Integer -> Integer
Integer - Integer -> Integer
Integer * Integer -> Integer
Integer / Integer -> Number

Number + Number -> Number
Number - Number -> Number
Number * Number -> Number
Number / Number -> Number
```

Mixed `Integer`/`Number` operands are promoted to `Number` before arithmetic.

Text concatenation is not an arithmetic operation. A later explicit concatenation operator/function must define it; v0.1 must not reinterpret `ditambah` as text concatenation merely because one operand is `Text`.

Division by zero is a semantic/runtime validity concern and must have an explicit diagnostic/runtime rule; it must not alter the type result.

## 12. Comparisons

Comparison expressions produce `Boolean`.

Equality is defined only for compatible types under the explicit equality relation.

Ordering comparisons (`lebih besar dari`, `lebih kecil dari`) are valid for numeric operands in v0.1.

Text ordering is not implicitly introduced by this contract.

The analyzer must reject incompatible comparisons rather than converting values by guesswork.

## 13. Logical conditions

Condition positions require a `Boolean` expression.

Therefore:

```text
kalau <Boolean>
```

is valid, while a raw `Integer`, `Number`, `Text`, `List<T>`, or `Nothing` is not implicitly converted to Boolean.

There is no truthiness rule in v0.1.

## 14. Function calls

A function call is valid when:

1. the resolved declaration is callable;
2. argument count equals parameter count;
3. each argument type is compatible with the corresponding parameter type;
4. no argument requires an unsupported implicit conversion.

The result type is the function's declared result type.

A function with result type `Nothing` cannot be used where a value expression is required.

## 15. Lists

For `List<T>`:

- `create list` establishes a list value;
- adding an element requires the element type to be compatible with `T`;
- removing an element requires the removal expression to be compatible with `T`;
- reading the list produces `List<T>`;
- iteration binds each element as type `T`.

The exact surface syntax for list typing/creation is governed by grammar and standard-library contracts.

## 16. Nothing

`Nothing` is not a wildcard and is not compatible with arbitrary types.

It may be used only where the surrounding semantic construct accepts an operation with no value result, such as a statement position.

A `Nothing` expression cannot be silently replaced by a default value.

## 17. Type inference boundary

v0.1 permits local inference only where the source construct supplies an unambiguous type:

- literal expressions;
- resolved references;
- function calls with known result type;
- arithmetic expressions under the operator rules;
- list element operations with a known `T`.

Inference must never use variable names, comments, natural-language similarity, external metadata, runtime observations, or LLM output.

## 18. Explicit conversions

Implicit conversion is intentionally minimal.

v0.1 recognizes only numeric widening from `Integer` to `Number` where the surrounding operation explicitly permits it.

All other conversions require an explicit future conversion operation/type rule.

For example, text such as `"10"` must not automatically become integer `10` because the text looks numeric.

## 19. Type errors

Recommended stable diagnostics:

- `ETYPE001` — incompatible types
- `ETYPE002` — condition requires Boolean
- `ETYPE003` — invalid function argument type
- `ETYPE004` — wrong function argument count
- `ETYPE005` — invalid list element type
- `ETYPE006` — invalid operation for operand types
- `ETYPE007` — unsupported implicit conversion
- `ETYPE008` — value required but expression has Nothing type

Diagnostics should report the expression/operator span and, where useful, the expected and actual semantic types.

## 20. Determinism

For identical AST, resolved declarations, and language-version contract, type analysis must produce identical results.

It must not depend on declaration insertion order beyond the already-defined lexical resolution result, hash-map ordering, machine locale, environment variables, filesystem order, network state, timing, execution history, or ML/LLM output.

## 21. Semantic boundary

This type contract does not define:

- concrete type-annotation grammar;
- user-defined structs/classes;
- generic types beyond `List<T>`;
- ownership/borrowing;
- concurrency types;
- effect/capability types;
- module/package type exports;
- runtime memory representation;
- ABI/FFI representation;
- semantic IR encoding.

Those contracts must build on this type model rather than redefining its primitive identities.

## 22. Acceptance tests

The conformance suite must cover at least:

1. literal type classification;
2. exact primitive type identity;
3. Integer arithmetic;
4. mixed Integer/Number arithmetic;
5. division result type;
6. invalid numeric/text operation rejection;
7. Boolean condition acceptance;
8. non-Boolean condition rejection;
9. exact equality compatibility;
10. numeric ordering comparison;
11. incompatible comparison rejection;
12. function result typing;
13. function argument count/type checking;
14. homogeneous list element checking;
15. loop binding type from list element type;
16. Nothing not accepted as a value type;
17. Integer-to-Number widening;
18. rejection of unsupported implicit conversions;
19. deterministic repeated type analysis.

## 23. Implementation gate

Type analysis may run only after AST construction and name resolution succeed for the relevant node.

The implementation must represent semantic types explicitly and must not encode type identity solely through localized surface words.

The reference interpreter, future compiler, standard library, and interoperability layers must consume the same semantic type definitions.
