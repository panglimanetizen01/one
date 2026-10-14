# One v0.1 Lexical Specification

Status: normative for v0.1 implementation.

## Source encoding

- Source is UTF-8.
- A source file is a sequence of Unicode scalar values.
- Newline is normalized to LF before lexical analysis.
- Tabs are rejected in indentation in v0.1 to prevent visual ambiguity. Four spaces are the canonical indentation unit.

## Line structure

- Blank lines are ignored for statement/block structure.
- A line containing only spaces is blank.
- A comment begins with `#` outside a string and continues to end of line.
- Newline terminates a statement unless the grammar explicitly requires a continuation phrase; v0.1 avoids implicit multiline expressions.

## Indentation

- Indentation is measured in groups of exactly four spaces.
- The first non-blank, non-comment line establishes indentation level zero.
- A block is introduced by a construct whose grammar requires a nested body.
- The following non-blank line must increase indentation by exactly one level.
- Dedentation may return only to a previously observed indentation level.
- Inconsistent indentation is a syntax error.

## Identifiers

Identifiers use Unicode letters followed by zero or more Unicode letters, decimal digits, or `_`. Identifiers may contain spaces only when they are represented by an explicitly defined multi-word semantic phrase; arbitrary whitespace-separated names are not identifiers.

The v0.1 implementation initially accepts ASCII identifiers plus Unicode letters/digits where the Rust Unicode character predicates provide deterministic behavior. Reserved phrases are recognized contextually by the parser rather than by allowing arbitrary natural-language prose.

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

These are not intended to make the language arbitrary; each phrase has a defined production in the concrete grammar.

## Error principle

Unknown words are not automatically mapped to keywords by similarity. The lexer/parser must preserve enough source information to issue deterministic diagnostics.
