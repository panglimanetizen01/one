# One

**One** is a human-oriented, deterministic, general-purpose programming language designed to let ordinary users build software from small scripts to large projects.

## Foundation

- Human-oriented conversational surface syntax
- Formal deterministic grammar
- Minimal required punctuation
- Indonesian as the initial language with localization designed in from the start
- Common semantic model / IR shared across locales
- Core constructs: DATA, ACTION, CONDITION, LOOP, FUNCTION, EVENT
- Ambiguity is rejected rather than guessed
- Human-readable diagnostics
- Windows and Linux as first-class platforms
- Complete project lifecycle is an architectural goal
- Deep interoperability with existing ecosystems is an architectural goal
- LLM-independent compiler/runtime semantics
- Progressive complexity from beginner programs to large systems

## Current v0.1 implementation

The canonical repository now contains the first executable language core:

```text
Localized source
    -> Lexer
    -> AST parser
    -> Semantic analyzer
    -> One Semantic IR
    -> Reference interpreter
    -> CLI
```

The v0.1 CLI currently provides:

```text
one check <file.one>
one run <file.one>
one ast <file.one>
```

The implementation includes the six core constructs, structured diagnostics, a capability model, cross-platform process-interoperability primitives, and a Linux/Windows CI matrix. Native FFI, a full optimizing compiler backend, package registry, IDE tooling, and additional human-language grammars remain follow-on milestones and are not represented as complete until their contracts and tests exist.

## Example

```text
nama adalah "Budi"
umur adalah 18

kalau umur sudah 17 tahun
    tampilkan "Boleh masuk"
kalau tidak
    tampilkan "Belum boleh masuk"
```

## Repository status

One v0.1 is under active implementation and verification. The dependency-ordered plan is the execution contract; unsupported features must not be claimed as complete.

See:

- `docs/superpowers/specs/2026-10-15-one-language-foundation-design.md`
- `docs/superpowers/plans/2026-10-15-one-v0-1-implementation.md`
- `docs/spec/one-v0.1-lexical.md`
- `docs/spec/one-v0.1-grammar.md`

## Canonical source

This GitHub repository is the canonical source of truth for One. Local copies are verification/validation environments and are not the primary source of changes.
