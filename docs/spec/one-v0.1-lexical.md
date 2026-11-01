# One v0.1 Lexical Specification

Status: **normative** for v0.1 implementation.

## 1. Scope

This document defines the lexical layer only. It does not decide the semantic meaning of a source statement and does not recognize user-defined multi-word names as lexical tokens.

The lexical layer MUST be deterministic. It MUST NOT use fuzzy matching, frequency, editor history, filesystem state, execution history, machine learning, or an LLM to classify source text.

The concrete grammar consumes the lexical stream and determines the boundaries of grammar-level names and reserved phrases.

## 2. Source encoding

- Source is UTF-8.
- A source file is a sequence of Unicode scalar values.
- Newline is normalized to LF before lexical analysis.
- Tabs are rejected in indentation in v0.1 to prevent visual ambiguity.
- Four spaces are the canonical indentation unit.
- The v0.1 lexer rejects source files larger than 4 MiB (UTF-8 bytes), token streams larger than 250,000 tokens, and indentation deeper than 256 levels. These deterministic limits prevent unbounded token and indentation allocation.

## 3. Line structure

- Blank lines are ignored for statement/block structure.
- A line containing only spaces is blank.
- A comment begins with `#` outside a string and continues to end of line.
- Newline terminates a statement; v0.1 avoids implicit multiline expressions.
- The lexer preserves source locations for every emitted token.

## 4. Indentation

- Indentation is measured in groups of exactly four spaces.
- The first non-blank, non-comment line establishes indentation level zero.
- A block is introduced by a construct whose grammar requires a nested body.
- The following non-blank line must increase indentation by exactly one level.
- Dedentation may return only to a previously observed indentation level.
- Inconsistent indentation is a syntax error.
- Indentation on blank/comment-only lines does not alter block structure.

## 5. Token classes

The abstract lexical interface exposed to the concrete grammar is:

```text
WORD      = one eligible lexical word
NUMBER    = ASCII decimal integer
TEXT      = quoted UTF-8 text literal
NEWLINE   = logical line terminator
INDENT    = structural indentation increase
DEDENT    = structural indentation decrease
```

The lexer may retain additional source metadata internally, but those implementation details do not change the normative token contract.

## 6. WORD tokens and user-defined names

A `WORD` is a lexical word consisting of a Unicode alphabetic character or `_`, followed by zero or more Unicode alphanumeric characters or `_`.

The lexer emits adjacent words separately. For example:

```text
nama pengguna adalah Budi
```

produces the conceptual sequence:

```text
WORD("nama")
WORD("pengguna")
WORD("adalah")
WORD("Budi")
```

The lexer MUST NOT emit:

```text
NAME("nama pengguna")
```

or otherwise decide that adjacent words form a user-defined name.

A user-facing multi-word name is a **grammar-level construct**. Its start, eligible words, and terminating boundary are determined by the active concrete-grammar production.

This means the lexical layer remains language-neutral with respect to name span while the grammar remains responsible for syntactic boundaries.

## 7. Grammar-level name boundary contract

A `name_word` is a `WORD` that is legal as a user-defined name component at the current grammar position.

A structural keyword is not consumed as a name component when the active production requires that keyword as its boundary. This is context supplied by the concrete grammar, not a lexical longest-match rule.

Examples:

```text
nama pengguna adalah Budi
saldo rekening menjadi 100000
tambahkan apel ke daftar buah
hapus telur dari daftar belanja
```

The lexical layer merely emits words. The grammar decides that `adalah`, `menjadi`, `ke`, and `dari` are structural boundaries in those productions.

The same spelling is not globally forbidden as a name word. A word becomes structurally reserved only where the active grammar production assigns it that role.

## 8. Reserved phrases

Multi-word reserved phrases are sequences of lexical tokens, not opaque lexical identifiers.

Examples include:

```text
kalau tidak
untuk setiap
buat pekerjaan
hasilnya adalah
```

The lexer emits their component `WORD` tokens. The concrete grammar recognizes the sequence only where the corresponding production is legal.

The lexer MUST NOT implement general-purpose phrase matching that changes tokenization based on semantic context.

## 9. Literals

### Text

Double-quoted strings are text literals. A string may contain escaped `\"`, `\\`, `\n`, and `\t`.

### Number

Numbers are base-10 integers in v0.1. Decimal floating point is reserved for a later type-system extension.

### Boolean

The canonical Indonesian literals are `benar` and `salah`.

### Nothing

`tidak ada` is the canonical empty/nothing value phrase where a value is required. Its phrase-level interpretation belongs to the concrete grammar; the lexical layer still emits separate words.

## 10. Punctuation

Punctuation is minimized. The following are syntax punctuation rather than required statement terminators:

- `"` for text delimiters;
- `#` for comments;
- `(` and `)` are not required by core v0.1 constructs;
- `{}`, `;`, `=>`, and comma-separated statement syntax are not part of the core surface grammar.

## 11. Core grammar phrases

The initial Indonesian grammar recognizes these semantic phrases through concrete productions:

```text
buat
adalah
menjadi
tampilkan
kalau
kalau tidak
untuk setiap
di
selama
buat daftar
buat pekerjaan
terima
hasilnya adalah
ketika
tambahkan
ke
hapus
dari
dengan
```

These are not arbitrary natural-language aliases. The grammar assigns each phrase a defined syntactic role.

The presence of a word in this list does not make it globally unusable inside every user-defined name. Boundary behavior is production-specific.

## 12. Error principle

Unknown words are not automatically mapped to keywords by similarity.

A lexical error MUST identify a deterministic source location and stable lexical error category where applicable.

The lexer MUST preserve deterministic token-level source locations for each emitted `WORD`. The v0.1 AST stores normalized multi-word names and statement-introducer anchor spans; it does not promise a complete name span after parsing. Consumers MUST NOT infer a full multi-word name range from the AST. A future AST/source-map extension that exposes such ranges requires a separately reviewed compatibility change.

## 13. Non-goals

The lexical specification does not:

- infer names from natural-language meaning;
- resolve semantic identifiers against a symbol table;
- choose among ambiguous grammar parses;
- create executable actions from arbitrary words;
- normalize synonyms by similarity;
- invoke an LLM or external network service.

## 14. Required lexical acceptance tests

At minimum, tests MUST establish that:

1. `nama pengguna adalah Budi` emits separate `WORD` tokens for `nama` and `pengguna`.
2. `daftar belanja` is not emitted as a lexical `NAME` token.
3. reserved multi-word phrases remain component tokens.
4. tabs in indentation are rejected with the stable lexical diagnostic.
5. token-level source spans are preserved for every component word; AST consumers do not assume those spans survive name normalization.
6. repeated lexing of identical input produces an identical token stream.
7. punctuation outside the permitted literal/comment forms is rejected deterministically.
8. quoted text remains a single `TEXT` token even when it contains spaces.

## 15. Compatibility rule

This specification supersedes the former v0.1 statement that user-facing identifiers are necessarily single lexical words.

The replacement rule is:

> lexical words remain single tokens; user-facing multi-word names are grammar-level sequences whose boundaries are defined by concrete productions.

No lexer implementation may claim conformance to multi-word names by introducing a lexical `NAME` token.

## 16. Design decision

**Approved:** lexical `WORD` tokens plus grammar-bounded multi-word names.

This preserves deterministic lexing while allowing the beginner-oriented surface to use names such as `nama pengguna`, `daftar belanja`, and `saldo rekening` without requiring punctuation-heavy quoting.

## 9. Deterministic implementation resource bounds

The reference lexer enforces a maximum source size of 4 MiB, a maximum of 250,000 emitted tokens (including structural tokens), and a maximum indentation depth of 256 levels. Exceeding a limit returns a stable diagnostic; it must not panic, truncate input, or silently change tokenization.
