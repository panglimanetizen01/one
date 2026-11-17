# One v0.1 — Semantic Model

**Status:** Draft normative contract for review

## 1. Purpose

This document defines the semantic meaning of the v0.1 AST after parsing, name resolution, and type analysis. It is the contract that precedes the common semantic IR and reference interpreter.

## 2. Semantic pipeline

The normative semantic pipeline is:

```text
Source
  -> localized lexer
  -> parser
  -> AST
  -> name resolution
  -> type analysis
  -> semantic analysis
  -> validated semantic program
  -> semantic IR
```

Each stage consumes the previous stage's contract and must not silently reinterpret rejected input.

## 3. Semantic invariants

1. A program that has not passed parsing has no semantic program.
2. A program with unresolved or duplicate names has no validated semantic program.
3. A program with type errors has no validated semantic program.
4. Semantic analysis is deterministic.
5. Semantic analysis does not execute user code.
6. Semantic analysis does not call an LLM for meaning.
7. Semantic analysis does not read arbitrary external state to decide program meaning.
8. Every validated semantic operation has an explicit semantic definition.
9. Unsupported surface constructs are rejected rather than assigned guessed meaning.
10. The semantic model is independent of localization spelling.

## 4. Program semantics

A program is an ordered sequence of statements. Statement order is semantically significant.

Execution begins at the program entry sequence defined by the runtime contract and proceeds in source order unless a control-flow construct changes the path.

A declaration statement establishes a binding according to the name-resolution contract. Its initializer expression is evaluated according to expression semantics before the binding becomes available to subsequent statements.

## 5. Values and expressions

Expressions evaluate to semantic values with types defined by the type-system contract.

Core expression forms:

- text literal;
- integer literal;
- number literal;
- Boolean literal;
- Nothing;
- reference;
- function call;
- binary arithmetic/comparison expression.

Expression evaluation is strict in v0.1: an expression's required operands are evaluated before the operation is applied, subject to the explicit control-flow rules for conditions and loops.

No implicit execution of arbitrary text is permitted.

## 6. Declaration semantics

A declaration evaluates its value expression and associates the resulting value with its resolved declaration identity.

A declaration does not create a hidden global variable when placed in a nested lexical scope.

The declaration's type must satisfy the declared/inferred type rules.

## 7. Assignment semantics

Assignment evaluates the right-hand expression and updates the existing resolved binding.

Assignment does not create a missing binding. An unresolved assignment target is a name-resolution error.

The resulting value must be assignment-compatible with the target type.

Assignment is a statement and therefore has result type `Nothing`.

## 8. Display action

The display action evaluates its expression and sends the resulting value to the runtime output facility.

Display does not mutate the value being displayed.

Formatting of semantic values is a runtime/standard-library concern and must not change the value's semantic type.

## 9. List creation and mutation

List creation produces a `List<T>` value according to the list type contract.

Adding an element:

1. resolves the target list;
2. evaluates the element expression;
3. checks element compatibility with `T`;
4. appends the element.

Removing an element:

1. resolves the target list;
2. evaluates the removal expression;
3. checks compatibility with `T`;
4. removes according to the list removal policy defined by the runtime contract.

The semantic model does not silently convert incompatible elements.

## 10. Conditions

A condition evaluates its test expression.

The test must have type `Boolean`.

If the test is true, the then-block executes. Otherwise, if an else-block exists, the else-block executes.

Exactly one branch executes for each condition evaluation.

A condition statement has result type `Nothing`.

## 11. For-each loop

A `ForEach` evaluates its source expression and requires a list value.

For each element in iteration order:

1. the loop binding receives the element value;
2. the loop body executes in the binding's lexical scope;
3. the next element is processed.

The loop binding is not visible after the loop.

Mutation of the source list during iteration is outside the v0.1 semantic guarantee unless the runtime contract explicitly defines the behavior. The implementation must not silently choose a mutation policy and present it as language semantics.

## 12. While loop

A `While` evaluates its Boolean test before each iteration.

If true, the body executes and the test is evaluated again.

If false, the loop terminates.

The semantics do not guarantee termination.

A `While` statement has result type `Nothing`.

## 13. Functions

A function declaration introduces a named callable binding in its containing scope.

Calling a function:

1. resolves the function declaration;
2. evaluates arguments according to the call evaluation order;
3. creates a new function scope;
4. binds parameters to argument values;
5. executes the function body;
6. produces the declared result value or `Nothing`.

The v0.1 call evaluation order is left-to-right by source order.

A function cannot access a local declaration from a caller's scope by lexical name lookup. It may access declarations visible through its own lexical parent according to the language's function-declaration placement rules.

Recursion is permitted unless a later resource/capability contract explicitly restricts it.

## 14. Function result

A function result is produced by the explicit result construct defined by the grammar/AST contract.

If execution reaches the end of a function that declares a non-`Nothing` result, semantic validation must reject the function unless the control-flow analysis proves that every path produces the required result. The exact path-completeness analysis is a later correctness contract.

A `Nothing` function cannot be used as a value expression.

## 15. Events

An event declaration registers a handler associated with a trigger description.

Event registration itself does not execute the handler.

When the runtime receives the matching event, it creates the handler execution context and evaluates the handler body according to the event contract.

Event delivery, ordering, concurrency, cancellation, and external event sources are runtime/effect concerns and are not guessed by semantic analysis.

## 16. Evaluation order

Unless a construct explicitly defines another order, v0.1 evaluates expressions from left to right in source order.

This rule applies to function arguments and binary expression operands.

The evaluator must not reorder observable operations merely as an optimization unless the later optimization contract proves semantic equivalence.

## 17. Errors

Semantic errors prevent creation of a validated semantic program.

Runtime errors occur only after a syntactically and semantically valid program begins execution. Runtime errors must not be disguised as compile-time semantic success.

Examples of semantic errors include:

- unresolved name;
- duplicate declaration;
- invalid operand types;
- invalid function argument count/type;
- non-Boolean condition;
- invalid list element type;
- value expression using a `Nothing` result;
- unsupported construct.

Examples of runtime concerns include:

- division by zero;
- output failure;
- resource exhaustion;
- external I/O failure.

The boundary between these classes must remain explicit.

## 18. Side effects

Core pure expression evaluation does not perform hidden external I/O.

Explicit side-effecting actions are represented as semantic operations and later checked against the effect/capability model.

A semantic operation must not acquire an undeclared filesystem, process, network, native, or other external capability merely because an implementation happens to have access to it.

## 19. Localization independence

The semantic model is expressed in language-neutral operations and types.

For example, Indonesian surface text equivalent to display and an English localized surface form must lower to the same semantic operation when their grammar productions are defined as equivalent.

Localized spelling must not alter:

- type identity;
- scope rules;
- evaluation order;
- arithmetic semantics;
- ambiguity policy;
- capability boundaries.

## 20. Determinism

For identical AST, resolved declarations, type results, language version, and explicit program inputs, semantic analysis must produce identical validated semantic output.

Semantic analysis must not depend on network state, filesystem enumeration order, machine locale, random values, timing, editor state, execution history, or LLM output.

## 21. Semantic operation vocabulary

The minimum v0.1 semantic vocabulary is:

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
FunctionDefine
FunctionCall
EventDefine
Reference
Literal
BinaryOperation
Return
```

The list is semantic, not necessarily a one-to-one AST node list. Lowering may combine or split internal nodes as long as the observable semantics remain equivalent.

## 22. Semantic program boundary

A validated semantic program contains:

- normalized AST structure;
- resolved declaration identities;
- checked semantic types;
- explicit semantic operations;
- source spans sufficient for diagnostics;
- no unresolved ambiguity.

It does not yet contain:

- target-machine instructions;
- platform-specific ABI details;
- runtime memory addresses;
- optimized machine code;
- package downloads;
- external process execution.

Those belong to later IR/runtime/compiler contracts.

## 23. Acceptance tests

The conformance suite must cover at least:

1. declaration and assignment semantics;
2. display evaluation;
3. list creation/add/remove semantics;
4. Boolean condition branching;
5. for-each iteration and binding scope;
6. while-loop condition evaluation;
7. function declaration/call/result behavior;
8. left-to-right argument evaluation;
9. recursion;
10. event registration without immediate execution;
11. semantic rejection of unresolved names;
12. semantic rejection of type-invalid operations;
13. semantic rejection of Nothing in value position;
14. side-effect boundary preservation;
15. deterministic semantic output;
16. localization-independent semantic equivalence;
17. source-span preservation through semantic diagnostics.

## 24. Implementation gate

The reference interpreter and future compiler must consume this semantic contract rather than independently reinterpreting the surface grammar.

The common semantic IR is now separately specified in docs/spec/one-v0.1-semantic-ir.md. The IR must preserve every observable semantic rule defined here and must not introduce platform-specific meaning into the language core. This section records the original dependency order, not a pending gate.
