# One Localization Contract

## Goal

One starts with Bahasa Indonesia and is designed from the beginning to support additional human languages without creating semantic forks.

## Separation of concerns

```text
Localized vocabulary/grammar
        ↓
Localized AST construction
        ↓
Common semantic model
        ↓
Common One IR
        ↓
Common runtime/compiler semantics
```

## Required invariants

Different locales must preserve:

- construct identity;
- name/scope semantics;
- type semantics;
- effects;
- capability identity;
- target/resource scope;
- policy decisions;
- result types;
- diagnostics meaning;
- deterministic execution semantics.

## Initial locale

Bahasa Indonesia is the initial language and the canonical first localization surface.

## Future locales

The architecture must permit languages such as English, Japanese, Chinese, Spanish, Arabic, and other languages without copying or independently evolving the semantic core.

## Ambiguity

Localization must not convert ambiguous wording into guessed execution. Each localized grammar needs deterministic acceptance/rejection rules and conformance tests against shared semantic expectations.

## Tooling

Language tooling should allow locale-aware lexing, parsing, diagnostics, formatting, documentation, and editor support while keeping the core semantic model shared.
