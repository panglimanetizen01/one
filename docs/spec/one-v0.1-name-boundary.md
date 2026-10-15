# One v0.1 — Name Boundary Contract

**Status:** Normative cross-gate contract for v0.1 review
**Scope:** Lexical ↔ concrete-grammar boundary for user-defined names
**Surface language:** Indonesian-first
**Semantic principle:** Natural on the surface; formal underneath.

## 1. Purpose

This specification resolves the v0.1 conflict between the older single-word identifier assumption and the approved requirement that One support explicit multi-word user-defined names without accepting arbitrary prose.

The lexer continues to emit individual `WORD` tokens. Multi-word names are parser-level constructs whose boundaries are determined by the active grammar production.

No fuzzy matching, frequency heuristic, editor history, filesystem order, execution history, machine learning, or LLM may determine a name boundary.

This contract does not define runtime semantics. It defines the boundary that the lexical and concrete-grammar specifications must implement consistently.

## 2. Alternatives considered

### A. Single-word names only

Rejected for the approved beginner-oriented surface because names such as `nama pengguna` and `daftar belanja` would require unnatural punctuation or renaming.

### B. Longest-match multi-word names

Rejected. Consuming the longest plausible sequence can swallow grammar keywords, arguments, or later statement structure.

### C. Quoted names

Rejected as the primary v0.1 surface. Requiring quotation for ordinary multi-word names adds punctuation and works against the beginner-oriented design. A quoted-name escape hatch may be considered later.

### D. Grammar-bounded names

**Selected.** Each production determines where a name occurs, which words may belong to it, and what terminates that syntactic role. If more than one complete semantic parse remains, the source is rejected.

## 3. Core model

The lexical layer emits words separately:

```text
WORD("nama")
WORD("pengguna")
WORD("adalah")
WORD("Budi")
```

The concrete grammar may construct:

```text
Name(words = ["nama", "pengguna"])
```

when `adalah` is reached as the declaration boundary.

The lexer MUST NOT emit `NAME("nama pengguna")` merely because two adjacent words occur.

## 4. Name eligibility

A `name_word` is a `WORD` legal as a user-defined name component at the current grammar position.

A word required as a structural keyword by the active production is not consumed as part of the name when doing so would prevent completion of that production.

Important structural boundaries include, according to production context:

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

This list is not a global ban on those spellings in every context. It is a production-specific boundary rule.

## 5. Production-bounded names

### 5.1 Declaration

```text
declaration = name, ADALAH, expression ;
```

Therefore:

```text
nama pengguna adalah Budi
```

has `nama pengguna` as the declaration name and `adalah` as the boundary.

### 5.2 Assignment

```text
assignment = name, MENJADI, expression ;
```

Example:

```text
saldo rekening menjadi 100000
```

The name is `saldo rekening`.

### 5.3 Create list

```text
create_list_action = "buat", "daftar", name, NEWLINE ;
```

Example:

```text
buat daftar belanja
```

The fixed phrase `buat daftar` establishes the start of the target name, and `NEWLINE` terminates it.

### 5.4 Add action

```text
add_list_action = "tambahkan", expression, "ke", name, NEWLINE ;
```

Example:

```text
tambahkan apel ke daftar buah
```

The parser completes the value expression at `ke`, then constructs `daftar buah` as the target name.

`ke` MUST NOT become part of either name.

### 5.5 Remove action

```text
remove_list_action = "hapus", expression, "dari", name, NEWLINE ;
```

Example:

```text
hapus telur dari daftar belanja
```

The target name begins after `dari` and ends at the statement boundary.

### 5.6 Display/reference

```text
display_action = "tampilkan", expression, NEWLINE ;
name_reference = name ;
```

Therefore:

```text
tampilkan nama pengguna
```

may construct `Name("nama pengguna")` because the expression grammar explicitly permits a reference name.

If another complete production produces a different semantic interpretation, the source is ambiguous and MUST be rejected.

## 6. End-of-line boundary

When a name is the final argument of a production, `NEWLINE`, `DEDENT`, or end-of-file may terminate the name according to the enclosing production.

This is not the rejected heuristic "take the longest phrase that sounds like a name" because the production has already assigned the complete syntactic role to the remaining eligible words.

## 7. Reserved phrases

Multi-word reserved phrases are grammar sequences, not opaque lexical identifiers.

Examples:

```text
untuk setiap
kalau tidak
hasilnya adalah
buat pekerjaan
```

A phrase sequence may take precedence over a shorter keyword only when the active grammar production defines that sequence. This is grammar precedence, not general longest-match guessing.

## 8. Expressions versus names

A name can be parsed as an expression only where the expression grammar explicitly allows a name reference.

For example:

```text
nama pengguna adalah Budi
```

is a declaration because `adalah` completes the declaration production.

But:

```text
nama pengguna mungkin besok
```

does not become a declaration or a name merely because `nama pengguna` resembles a known identifier. It must match an actual production or be rejected.

## 9. Ambiguity algorithm

For every source statement:

1. Tokenize deterministically.
2. Consider only productions legal at the current syntactic position.
3. Construct complete parses permitted by those productions without fuzzy name inference.
4. Normalize parses that have identical semantic structure.
5. If exactly one semantic parse remains, accept it.
6. If zero remain, emit a parse error.
7. If more than one remains and their semantic structures differ, emit `EAMBxxx`.
8. Never resolve ambiguity using frequency, probability, source history, filesystem state, execution history, ML, or LLM output.

A deterministic parser may use grammar precedence to avoid constructing equivalent alternatives, provided the resulting semantics are exactly those specified by this contract.

## 10. Invalid boundary cases

The following MUST NOT be accepted through name-boundary heuristics:

```text
tambahkan apel entah ke mana
```

No core production authorizes `entah ke mana` as an implicit target.

```text
kalau nama pengguna mungkin benar
    tampilkan nama pengguna
```

No comparison production authorizes `mungkin` as an implicit operator.

```text
apel ke daftar buah
```

This does not become an action merely because it contains an `ke` phrase.

```text
saya ingin kamu menampilkan nama pengguna
```

This is ordinary Indonesian prose and has no matching core production.

## 11. Required normative changes

The normative lexical and grammar specifications MUST:

1. replace single-word identifier assumptions with grammar-level `name` where v0.1 permits user-facing names;
2. keep `WORD` as the lexical unit and avoid a lexer-level `NAME` token;
3. remove `generic_action = identifier, [ expression ]` from the core v0.1 grammar;
4. define explicit action productions or a future explicit action registry boundary;
5. align declarations, assignments, list creation, collection mutation, references, function names, and parameter names with their production-specific boundaries;
6. preserve deterministic diagnostics for incomplete and ambiguous forms;
7. preserve source spans over the complete grammar-level name.

## 12. Required tests

The conformance suite MUST cover:

- single-word names;
- two-word names;
- three-or-more-word names where a production permits them;
- names adjacent to `adalah`, `menjadi`, `ke`, `dari`, and `di` boundaries;
- names containing ordinary non-structural words;
- reserved multi-word phrases;
- ambiguous parses producing `EAMBxxx`;
- incomplete productions producing the appropriate parse error;
- proof that arbitrary prose is not accepted as a name;
- proof that `apel` alone is not an implicit action;
- source-span correctness for multi-word names;
- deterministic repeated parsing of identical source.

## 13. Compatibility rule

This contract supersedes the older v0.1 assumption that every user-facing identifier is one lexical word.

No parser or lexer implementation change may claim conformance to the new multi-word-name behavior until the normative lexical and grammar documents agree and the required tests exist.

## 14. Design decision

**Approved direction:** grammar-bounded multi-word names with lexical `WORD` tokens, explicit production boundaries, deterministic ambiguity rejection, and no generic implicit actions.
