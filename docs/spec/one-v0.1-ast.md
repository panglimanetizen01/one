# One v0.1 — AST Contract

**Status:** Accepted for the v0.1 implementation baseline  
**Depends on:** Approved language invariants, lexical specification, concrete grammar  
**Scope:** Parser output consumed by semantic analysis and IR lowering

## 1. Purpose and boundary

The Abstract Syntax Tree (AST) is the structural boundary between parsing and semantic analysis. It records the parse selected by the deterministic v0.1 grammar. It does not execute source, resolve names, infer types, authorize external effects, or call an LLM/network service.

This contract deliberately describes the **v0.1 AST shape that the implementation exposes**. It does not promise a richer source-preserving tree than the current public types provide. Future AST expansion must be proposed as a separately reviewed compatibility change rather than assumed from aspirational diagrams.

## 2. Core invariants

1. Accepted source is parsed deterministically according to the v0.1 lexical and concrete grammar.
2. Unsupported statements and malformed expressions are rejected; they are not converted to generic executable intent.
3. Multi-word names are one logical name in the AST, represented by a normalized string formed by joining the grammar-selected word tokens with one ASCII space.
4. The normalized name string is semantic identity for v0.1. Source spelling and component-level name spans are not retained by the AST.
5. Every statement variant carries a `Span` anchor inherited from its introducer token. This is a deterministic diagnostic anchor, **not** a full statement-range span.
6. Expression nodes do not currently carry their own spans.
7. `Program` does not currently carry a span; it contains an ordered vector of statements.
8. AST construction does not perform symbol-table lookup, type inference, capability checks, or execution.
9. The parser must not guess between distinct meanings. Unsupported or malformed input returns a diagnostic and no successful `Program`.

## 3. Source locations

The current `Span` type contains:

```rust
Span {
    start: usize,
    end: usize,
    line: usize,
    column: usize,
}
```

`start` and `end` are byte offsets; `line` and `column` identify the token's starting location. In v0.1, statement spans are anchors based on the statement introducer token. They do not cover the complete statement or each component of a multi-word name.

Diagnostics that need a location must use the available statement/token anchor. Consumers MUST NOT interpret a statement span as a complete statement range.

Full node ranges, expression spans, and per-word name spans are intentionally outside this accepted v0.1 AST contract. Adding them requires tests and a reviewed compatibility change.

## 4. Program and statement schema

The v0.1 public schema is:

```rust
Program {
    statements: Vec<Statement>,
}

Statement =
    Declaration { name: String, value: Expr, span: Span }
  | Assignment  { name: String, value: Expr, span: Span }
  | Action      { name: String, args: Vec<Expr>, span: Span }
  | Conditional {
        condition: Expr,
        then_block: Vec<Statement>,
        else_block: Vec<Statement>,
        span: Span,
    }
  | Loop { kind: LoopKind, body: Vec<Statement>, span: Span }
  | Function {
        name: String,
        params: Vec<String>,
        result: Expr,
        body: Vec<Statement>,
        span: Span,
    }
  | Event { trigger: Vec<String>, body: Vec<Statement>, span: Span }

LoopKind =
    ForEach { variable: String, iterable: Expr }
  | While { condition: Expr }
```

The enum/field names above are the public v0.1 AST contract. The AST is not required to use a different algebraic model such as separate `Declare`, `Condition`, or `EventTrigger` structs.

## 5. Data and name representation

Declarations and assignments use a normalized `String` name:

```text
nama pengguna adalah "Budi"
saldo rekening menjadi 100000
```

The first example creates a declaration whose `name` is `"nama pengguna"`; the second creates an assignment whose `name` is `"saldo rekening"`.

Name words are selected by the concrete grammar. Structural separators such as `adalah` and `menjadi` are excluded from the name. A name is not an arbitrary slice of prose: the parser must apply the defined grammar boundary and reject unsupported forms.

References use `Expr::Identifier(String)`. Function names and parameters, loop variables and iterable references, and event triggers use the corresponding string fields shown in the schema. Event triggers are retained as an ordered `Vec<String>` of token spellings.

Because v0.1 normalizes a multi-word name to a string, component word spans and exact original whitespace are not available in the AST. Localization or tooling code must not assume otherwise.

## 6. Action schema

The implementation uses an explicit, grammar-validated action name and argument list:

```rust
Action { name: String, args: Vec<Expr>, span: Span }
```

Examples of normalized action names include `tampilkan`, `sapa`, `cari alamat`, `baca berkas`, `tulis ke berkas`, `ambil dari`, `jalankan proses`, `kueri database`, `buat daftar`, `tambahkan`, and `hapus`.

The generic AST variant does **not** mean arbitrary action text is accepted. The parser must recognize an explicit production and reject unsupported action heads with a stable diagnostic. Semantic analysis and IR lowering validate the action's argument contract.

## 7. Expression schema

The v0.1 expression schema is:

```rust
Expr =
    Text(String)
  | Number(i64)
  | Boolean(bool)
  | Nothing
  | Identifier(String)
  | Binary { left: Box<Expr>, op: BinaryOp, right: Box<Expr> }
  | Call { name: String, args: Vec<Expr> }

BinaryOp =
    Add | Subtract | Multiply | Divide
  | Equal | Greater | Less | AtLeast | AtMost
```

Operator matching, precedence/associativity, and legal expression boundaries are defined by the concrete grammar and parser. Type validity belongs to semantic analysis. Integer values use `i64` in the v0.1 implementation; changing the integer model requires a type-system decision and compatibility tests.

## 8. Blocks and control flow

A conditional stores its condition expression and separate `then_block` / `else_block` statement vectors. A loop stores a `LoopKind` and its body. A function stores its normalized name, normalized parameter names, result expression, and body. An event stores its ordered trigger tokens and body.

The parser enforces indentation and block structure. A successful AST must not contain an unresolved parse choice. Empty-block handling follows the concrete grammar and parser diagnostics; the AST schema itself does not reinterpret empty blocks.

## 9. Error and ambiguity behavior

The parser returns either a complete `Program` or a diagnostic bundle. It must not expose a partial public AST for malformed input.

- No valid grammar parse: reject with a parser diagnostic.
- Exactly one grammar-defined parse: return its AST.
- Unsupported action or expression token: reject; do not create a generic fallback node.
- Distinct semantic interpretations: reject rather than guess.

Stable diagnostic codes and their documented meanings must remain deterministic. A change to error codes or parse boundaries requires regression tests and an explicit compatibility review.

## 10. Conformance tests

The v0.1 AST test suite must cover:

1. Declaration and assignment with one-word and multi-word names.
2. Normalized names exclude structural separators and preserve word order.
3. Every statement variant has the expected enum and fields.
4. Expression variants and binary operators match the concrete grammar.
5. Action heads outside the grammar are rejected.
6. Function, loop, conditional, and event structures preserve their parsed order and nesting.
7. Statement span anchors identify the introducer token and are not mistaken for full statement ranges.
8. Invalid/incomplete statements return diagnostics rather than a partial `Program`.
9. Repeated parsing of identical source yields structurally identical AST output.
10. Existing valid, invalid, golden IR, semantic, and CLI fixtures remain deterministic.

## 11. Explicit v0.1 limitations

The following are not guarantees of this AST version:

- component-level source spans for multi-word names;
- full statement or expression ranges;
- a distinct `Name` type in the public AST;
- separate AST variants for every action;
- a source-preserving event-trigger node;
- an implementation-neutral integer literal type.

These limitations are documented rather than hidden. A future expansion should add the missing representation with fail-first tests and a coordinated migration across parser, semantic analysis, IR lowering, diagnostics, and tooling. It must not silently change v0.1 meaning.

## 12. Decision

For v0.1, the accepted AST is the concrete public schema implemented by `crates/one-core/src/ast.rs`, with grammar-defined normalized string names, explicit statement variants, generic-but-validated action nodes, expression enums, and statement-introducer span anchors.

This decision closes the discrepancy between the previous aspirational draft and the implementation baseline. It does **not** claim that full source-preserving AST tooling, multi-locale conformance, or release verification is complete.
