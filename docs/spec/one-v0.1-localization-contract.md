# One v0.1 — Localization Contract

**Status:** Draft normative contract for review

## 1. Purpose

Localization changes the human-language surface of One without changing its semantic model. All approved localizations must lower to the same language-level AST/semantic/CSIR meaning.

## 2. Separation of concerns

A localization defines:

- lexical vocabulary;
- grammar productions;
- structural phrase forms;
- diagnostic templates;
- examples/documentation wording;
- locale-specific literal conventions where explicitly supported.

It does not redefine core semantic operations or runtime behavior.

## 3. Semantic identity

Two localized programs are semantically equivalent when their validated AST/semantic forms represent the same language meaning under the same versioned contract. Keyword translation alone is insufficient; grammar may vary by language.

## 4. Determinism

Localization parsing must be deterministic and must reject ambiguity. No translation model, fuzzy matching, external network, or user history may decide syntax.

## 5. Names

User-defined names remain source data and are not translated automatically. Name-boundary rules from the v0.1 name contract apply within every localization grammar.

## 6. Structural vocabulary

Each localization must explicitly map the six semantic foundations: DATA, ACTION, CONDITION, LOOP, FUNCTION, EVENT. A localization may use natural grammatical forms, but every accepted form must map to a defined production.

## 7. Diagnostics

Diagnostic codes, structured parameters, and source-span semantics are shared. Human-facing text is localized. A localization must not change the error's meaning to make it sound more natural.

## 8. Numbers and literals

Locale-specific numeric or literal syntax is accepted only when specified by the lexical contract. Parsing must produce the same semantic literal value where forms are equivalent.

## 9. Versioning

A localization is versioned independently from the runtime but declares the One language version it targets. A change that alters accepted grammar or semantic mapping requires a new contract version or explicit compatibility rule.

## 10. Conformance

Each localization must pass:

- lexical fixtures;
- grammar/ambiguity fixtures;
- AST normalization fixtures;
- semantic equivalence fixtures;
- diagnostic localization fixtures;
- cross-localization CSIR equivalence fixtures.

## 11. Acceptance criteria

- Indonesian remains the initial reference localization;
- English and future localizations can be added without duplicating semantic/runtime rules;
- no localization requires an LLM for compilation or execution;
- equivalent meaning lowers to equivalent CSIR;
- ambiguity is rejected rather than guessed.