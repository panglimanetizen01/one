# One v0.1 Lexical Specification

Status: **normative** for v0.1 implementation.

## Source encoding

- Source is UTF-8.
- A source file is a sequence of Unicode scalar values.
- Newline is normalized to LF before lexical analysis.
- Tabs are rejected in indentation in v0.1 to prevent visual ambiguity. Four spaces are the canonical indentation unit.

## Line structure

- Blank lines are ignored for statement/block structure.
- A line containing only spaces is blank.
- A comment begins with `#` outside a string and continues to end of line.
- Newline terminates a statement; v0.1 avoids implicit multiline expressions.

## Indentation

- Indentation is measured in groups of exactly four spaces.
- The first non-blank, non-comment line establishes indentation level zero.
- A block is introduced by a construct whose grammar requires a nested body.
- The following non-blank line must increase indentation by exactly one level.
- Dedentation may return only to a previously observed indentation level.
- Inconsistent indentation is a syntax error.

## Identifiers

Identifiers are single lexical words consisting of a Unicode alphabetic character or `_`, followed by zero or more Unicode alphanumeric characters or `_`.

v0.1 does **not** treat arbitrary whitespace-separated words as a multi-word identifier. Multi-word semantic phrases such as `buat pekerjaan`, `untuk setiap`, and `hasilnya adalah` are grammar phrases composed from separate tokens. Multi-word user-defined names are reserved for a future explicit name grammar and must not be guessed by the implementation.

The v0.1 lexer uses deterministic character predicates supplied by Rust; numeric literals remain ASCII decimal digits.

## Literals

### Text

Double-quoted strings are text literals. A string may contain escaped `\"`, `\\`, `\n`, and `\t`.

### Number

Numbers are base-10 integers in v0.1. Decimal floating point is reserved for a later type-system extension.

### Boolean

The canonical Indonesian literals are `benar` and `salah`.

### Nothing

`tidak ada` is the canonical empty/nothing value phrase where a value is required.

## Punctuation

Punctuation is minimized. The following are syntax punctuation rather than required statement terminators:

- `"` for text delimiters;
- `#` for comments;
- `(` and `)` are not required by core v0.1 constructs;
- `{}`, `;`, `=>`, and comma-separated statement syntax are not part of the core surface grammar.

## Reserved core phrases

The initial Indonesian grammar recognizes these semantic phrases:

- `buat`
- `adalah`
- `menjadi`
- `tampilkan`
- `kalau`
- `kalau tidak`
- `untuk setiap`
- `di`
- `selama`
- `buat pekerjaan`
- `terima`
- `hasilnya adalah`
- `ketika`

These are not arbitrary natural-language aliases; each phrase has a defined production in the concrete grammar.

## Error principle

Unknown words are not automatically mapped to keywords by similarity. The lexer/parser must preserve enough source information to issue deterministic diagnostics.
