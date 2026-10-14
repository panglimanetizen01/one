# One v0.1 Localization Contract

## Goal

A human-language surface is a locale-specific grammar that maps to the same language-independent AST and Semantic IR.

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

English is the first future conformance target. Additional locales must be added through locale-specific grammar modules rather than by scattering translated keyword checks through the semantic/runtime layers.

## Architecture rule

```text
Locale Grammar -> Common AST -> Semantic Analysis -> One IR -> Runtime/Compiler
```

No runtime component may inspect Indonesian keywords to determine program meaning.
