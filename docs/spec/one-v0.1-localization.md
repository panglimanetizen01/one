# One v0.1 Localization Architecture Overview

**Status:** Non-normative architecture overview. The detailed requirements are in [the Localization Contract](./one-v0.1-localization-contract.md), which remains a draft for review until explicitly approved.

## Goal

A human-language surface is a locale-specific grammar that maps to the same language-independent AST and Semantic IR. Localization is a cross-cutting architectural requirement and must be considered alongside lexical rules and grammar design, not postponed until the end of the implementation roadmap.

## Locale boundary

A locale may define:

- keywords and multi-word phrases;
- phrase order;
- lexical conventions;
- literal spellings such as booleans;
- diagnostics language;
- examples and documentation terminology.

A locale may not redefine the meaning of an existing semantic construct without an explicit language-version change.

## Initial locale

`id-ID` / Indonesian is the first implementation locale.

## Future locales

English is the first future conformance target. Additional locales should be added through locale-specific grammar modules rather than by scattering translated keyword checks through semantic/runtime layers. This is an architectural target, not a claim that multiple locale implementations are currently verified.

## Architecture rule

```text
Locale Grammar -> Common AST -> Semantic Analysis -> One IR -> Runtime/Compiler
```

No runtime component may inspect Indonesian keywords to determine program meaning.

## Conformance boundary

The detailed localization contract defines required equivalence and test categories. A locale is not considered conformant until it passes lexical, grammar/ambiguity, AST normalization, semantic-equivalence, diagnostic-localization, and cross-locale IR-equivalence tests.
