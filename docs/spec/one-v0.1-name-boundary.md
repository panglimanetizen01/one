# One v0.1 — Name Boundary Contract

**Status:** Design specification for review
**Scope:** Lexical ↔ concrete-grammar boundary for user-defined names
**Surface language:** Indonesian-first
**Semantic principle:** Natural on the surface; formal underneath.

## 1. Purpose

This specification resolves the v0.1 conflict between single-word lexical identifiers and the approved requirement that One support explicit multi-word user-defined names without accepting arbitrary prose.

The lexer continues to emit individual `WORD` tokens. Multi-word names are a parser-level construct whose boundaries are determined by the active grammar production. No fuzzy matching, frequency heuristic, editor history, filesystem order, execution history, machine learning, or LLM may determine a name boundary.

This specification does not change runtime semantics and does not authorize implementation changes by itself. It defines the contract that the lexical and concrete-grammar specifications must implement consistently.

## 2. Design alternatives considered

### A. Single-word names only

Rejected for the approved beginner-oriented surface. It prevents natural names such as `nama pengguna` and `daftar belanja` from being represented directly.

### B. Longest-match multi-word names

Rejected. A parser that consumes the longest sequence that looks like a name creates context-sensitive guessing and can swallow grammar keywords or arguments.

### C. Quoted names

Rejected as the primary v0.1 surface. Quoting every multi-word identifier increases punctuation and moves the language away from the intended beginner experience. Quoted identifiers may be considered as a future escape hatch.

### D. Grammar-bounded multi-word names

**Selected.** Each production declares where a name may occur and which tokens terminate it. The parser considers only complete grammar parses. If multiple semantic parses remain, the source is rejected.

## 3. Core model

The lexical layer emits words individually:

```text
WORD("nama")
WORD("pengguna")
WORD("adalah")
WORD("Budi")
```

The grammar may then construct:

```text
Name(words = ["nama", "pengguna"])
```

when `ADALAH` is reached as the production boundary.

The lexer MUST NOT emit `NAME("nama pengguna")` merely because two adjacent words occur.

## 4. Name eligibility

A `name_word` is a `WORD` that is legal as a user-defined name component at the current grammar position.

A word that is required as a structural keyword in the active production is not consumed as part of the name when doing so would prevent completion of that production.

Core structural boundaries include, according to production context:

```text
adalah
menjadi
ke
dari
di
kalau
kalau tidak
selama
terima
hasilnya adalah
dengan
dan
```

The list is not a global ban on these spellings in every context. It is a grammar-boundary rule: a keyword token has structural meaning when the active production requires it.

## 5. Production-bounded names

### 5.1 Declaration

```text
declaration = name, ADALAH, expression ;
```

Therefore:

```text
nama pengguna adalah Budi
```

has one name boundary immediately before `adalah`.

The parser may construct:

```text
Name("nama pengguna")
```

and then parse `Budi` as the value expression.

### 5.2 Assignment

```text
assignment = name, MENJADI, expression ;
```

Example:

```text
saldo rekening menjadi 100000
```

The boundary is immediately before `menjadi`.

### 5.3 Add action

```text
add_action = TAMBAHKAN, expression, KE, name ;
```

Example:

```text
tambahkan apel ke daftar buah
```

The parser must first complete the value expression at `ke`, then construct `Name("daftar buah")` from the remaining eligible words.

It MUST NOT consume `ke` as part of either name.

### 5.4 Remove action

```text
remove_action = HAPUS, expression, DARI, name ;
```

Example:

```text
hapus telur dari daftar belanja
```

The target name begins after `dari` and ends at the statement boundary.

### 5.5 Display

The v0.1 display production must distinguish an expression from a name without consuming arbitrary prose. For a simple reference:

```text
tampilkan nama pengguna
```

may construct `Name("nama pengguna")` only if the expression grammar admits a reference name in that position and no competing complete parse with a different semantic meaning exists.

If the expression grammar permits a different interpretation, ambiguity is an error rather than a longest-match decision.

## 6. End-of-line boundary

When a name is the final argument of a production, `NEWLINE`, `DEDENT`, or end-of-file may terminate the name according to the enclosing production.

Example:

```text
buat daftar belanja
```

The name following the fixed phrase `buat daftar` is `belanja`.

For a production that permits a multi-word target, all remaining eligible `WORD` tokens up to the production's explicit terminator may form the name. This is not the rejected heuristic "take the longest phrase that sounds like a name" because the production itself defines the complete syntactic role and terminator.

## 7. Expressions versus names

A name may appear as an expression only where the expression grammar explicitly permits a reference.

This distinction prevents arbitrary sentence tails from becoming names.

For example:

```text
nama pengguna adalah Budi
```

is a declaration because `adalah` provides a complete declaration boundary.

But:

```text
nama pengguna mungkin besok
```

does not become a declaration or a name merely because `nama pengguna` resembles a known identifier. It must match an actual production or be rejected.

## 8. Reserved phrases and longest phrase behavior

Multi-word reserved phrases such as:

```text
untuk setiap
kalau tidak
hasilnya adalah
buat pekerjaan
```

are recognized as grammar sequences, not as opaque lexical identifiers.

A multi-word reserved phrase may take precedence over a shorter keyword only when the concrete grammar defines that sequence at the current position. This is grammar precedence, not general longest-match guessing.

## 9. Ambiguity algorithm contract

For every source statement:

1. Tokenize deterministically into lexical tokens.
2. Parse only productions legal at the current syntactic position.
3. Construct every complete parse permitted by those productions, without fuzzy name inference.
4. Normalize equivalent parses that produce the same semantic structure.
5. If exactly one semantic parse remains, accept it.
6. If zero remain, emit a parse error.
7. If more than one remain and their semantic structures differ, emit `EAMBxxx`.
8. Never resolve ambiguity using frequency, source history, filesystem state, execution history, ML, or LLM output.

An implementation may use deterministic parser precedence to avoid constructing equivalent alternatives, provided the resulting semantics are exactly those specified by the grammar.

## 10. Invalid boundary cases

The following MUST NOT be accepted by name-boundary heuristics:

```text
tambahkan apel entah ke mana
```

No production authorizes `entah ke mana` as a target collection.

```text
kalau nama pengguna mungkin benar
    tampilkan nama pengguna
```

No comparison production authorizes `mungkin` as an implicit operator.

```text
apel ke daftar buah
```

This does not become an action simply because it contains a phrase resembling an add target.

## 11. Required grammar changes

The existing normative grammar must be revised as follows:

1. Replace single-word `identifier` assumptions with the grammar-level `name` construct where v0.1 explicitly permits user-facing multi-word names.
2. Remove `generic_action = identifier, [ expression ]` from the core v0.1 grammar.
3. Define explicit action productions or an explicit future action registry boundary.
4. Keep `WORD` lexical and deterministic; do not introduce a lexer-level `NAME` token.
5. Align list creation, collection mutation, declarations, assignments, references, function names, and parameter names with their respective production-specific name boundaries.
6. Preserve exact error behavior for incomplete and ambiguous forms.

## 12. Required lexical changes

The normative lexical specification must be revised to state:

- identifiers are lexical words;
- user-facing names may be grammar-level sequences of eligible lexical words;
- the lexer never guesses multi-word names;
- structural keyword boundaries are determined by the concrete grammar;
- no implicit punctuation or quoting is required for ordinary multi-word names;
- source locations cover the complete span of a grammar-level name.

## 13. Acceptance examples

These forms are intended to be valid after the normative grammar is aligned with this contract:

```text
nama pengguna adalah Budi
saldo rekening menjadi 100000
buat daftar belanja
tambahkan apel ke daftar buah
hapus telur dari daftar belanja
tampilkan nama pengguna
buat pekerjaan hitung total
```

These forms remain invalid unless a separate production is explicitly added:

```text
selama masih ada barang di daftar belanja
apel
saya ingin kamu menampilkan nama pengguna
```

## 14. Test requirements

The eventual lexer/parser test suite MUST cover at minimum:

- single-word names;
- two-word names;
- three-or-more-word names where a production permits them;
- names adjacent to `adalah`, `menjadi`, `ke`, `dari`, and `di` boundaries;
- names containing words that are not structural keywords;
- reserved multi-word phrases;
- ambiguous parses that must produce `EAMBxxx`;
- incomplete productions that must produce `EPARSExxx`;
- proof that arbitrary prose is not accepted as a name;
- proof that `apel` alone is not an implicit action;
- source-span correctness for multi-word names;
- deterministic repeated parsing of the same source.

## 15. Compatibility rule

This contract supersedes the older v0.1 assumption that all user-facing identifiers are single lexical words, once the lexical and concrete grammar documents are updated together.

No parser or lexer implementation change may claim conformance to the new multi-word-name behavior until those normative documents are updated and their tests exist.

## 16. Design decision

**Approved direction:** grammar-bounded multi-word names with lexical `WORD` tokens, deterministic production boundaries, explicit ambiguity rejection, and no generic implicit actions.

This design preserves the beginner-facing surface while maintaining the language invariant that accepted source has a formal, deterministic interpretation.
