# One v0.1 — Formal Grammar Design

**Status:** Draft specification for review
**Role:** Next language-design gate after Language Invariants and prior-art research
**Surface language:** Indonesian-first
**Semantic principle:** Natural on the surface; formal underneath.

## 1. Scope

This document defines the proposed concrete grammar for the first Indonesian One language surface. It is a design contract, not yet an implementation patch.

The grammar is intentionally **controlled natural programming**, not unrestricted Indonesian prose.

A source line is accepted only when it matches a defined production. A phrase that merely sounds reasonable to a human is not automatically valid source.

## 2. Design goals

The grammar MUST:

1. minimize punctuation in ordinary code;
2. use newline and indentation as structural syntax;
3. let beginners describe data and actions in familiar Indonesian phrasing;
4. distinguish data, actions, and control flow at the semantic level;
5. support deterministic parsing without an LLM;
6. reject unresolved ambiguity instead of guessing;
7. permit explicit multi-word names without treating arbitrary prose as names;
8. map all accepted surface constructs into a common semantic model;
9. support later localization without duplicating semantic rules;
10. scale to functions and modules without requiring symbol-heavy syntax.

## 3. Fundamental principle: phrases, not free prose

One uses **defined phrases** rather than unrestricted sentences.

Examples of defined phrase families:

```text
nama adalah Budi
umur menjadi 26
tampilkan nama
hapus telur dari daftar belanja
tambahkan apel ke daftar buah
kalau umur sudah 17 tahun
untuk setiap nama di daftar teman
buat pekerjaan hitung total
```

The words surrounding a user-defined name have grammar roles. They are not interpreted by semantic similarity.

For example, `tambahkan apel ke daftar buah` is recognized because it matches the `add-to-target` production. A sentence such as `tolong mungkin tambahkan apel entah ke mana` is not automatically accepted.

## 4. Lexical interface assumed by this grammar

The grammar operates over the following abstract token classes:

```text
WORD       = a valid user word
NUMBER     = an ASCII decimal integer
TEXT       = a quoted text literal
NEWLINE    = logical line terminator
INDENT     = one structural indentation increase
DEDENT     = one structural indentation decrease
```

Reserved words and phrase keywords are identified by the concrete grammar in context. The lexical specification must define how reserved words coexist with user-defined multi-word names without making source interpretation probabilistic.

This point is an explicit dependency between the lexical and grammar gates and must be resolved before parser implementation is changed.

## 5. Name model

### 5.1 Name phrase

A user-facing name MAY contain multiple words.

```text
name = name_word, { name_word } ;
```

A `name_word` is a lexical word eligible for a name in the current grammar position.

Examples:

```text
nama
umur
nama pengguna
daftar belanja
nama produk
total harga setelah pajak
```

### 5.2 Name boundaries

Multi-word names are **not** recognized by consuming all remaining words on a line.

A grammar production determines where a name starts and ends.

Examples:

```text
nama pengguna adalah Budi
```

has the form:

```text
name + ADALAH + expression
```

while:

```text
tampilkan nama pengguna
```

has the form:

```text
TAMPILKAN + name
```

The parser MUST use production boundaries, not a heuristic such as "take the longest phrase that sounds like a name."

### 5.3 Ambiguous name boundaries

If a line can be parsed into more than one complete valid production with different semantic meanings, the parser MUST reject it.

It MUST NOT select a candidate merely because it is longer, more frequent, or more similar to a known name unless a later approved grammar rule explicitly establishes that precedence.

## 6. Program

```text
program = { line } ;
line = blank_line
      | comment_line
      | statement, NEWLINE
      | block_header, NEWLINE, INDENT, block, DEDENT ;

block = { statement, NEWLINE }
      | { block_header, NEWLINE, INDENT, block, DEDENT } ;
```

The exact parser representation of physical blank/comment lines may be normalized, but their handling MUST preserve source locations.

## 7. Data

### 7.1 Declaration

```text
declaration = name, ADALAH, expression ;
```

Examples:

```text
nama adalah Budi
umur adalah 25
kota adalah Jakarta
nama pengguna adalah Budi
```

Semantic form:

```text
Data.Declare
  name
  value
```

### 7.2 Assignment/update

```text
assignment = name, MENJADI, expression ;
```

Example:

```text
umur menjadi 26
```

Semantic form:

```text
Data.Assign
  target
  value
```

### 7.3 Data versus action

`adalah` and `menjadi` in these productions create/update data. They MUST NOT be treated as generic English-like filler words.

## 8. Values and expressions

### 8.1 Primary values

```text
expression = comparison_expression ;
comparison_expression = additive_expression, [ comparison_operator, additive_expression ] ;
additive_expression = multiplicative_expression, { additive_operator, multiplicative_expression } ;
multiplicative_expression = primary, { multiplicative_operator, primary } ;
primary = TEXT
        | NUMBER
        | BOOLEAN
        | NOTHING
        | name
        | call_expression
        | parenthesized_expression ;
```

Parentheses are reserved for an advanced/explicit syntax extension and are not required by the beginner surface. If parentheses are excluded from v0.1, `parenthesized_expression` is omitted from the accepted grammar rather than silently interpreted.

### 8.2 Operators

```text
additive_operator = DITAMBAH | DIKURANGI ;
multiplicative_operator = DIKALI | DIBAGI ;
comparison_operator = SUDAH | BELUM
                    | SAMA DENGAN
                    | LEBIH BESAR DARI
                    | LEBIH KECIL DARI ;
```

Examples:

```text
total adalah harga ditambah pajak
sisa adalah jumlah dikurangi terpakai
kalau saldo lebih besar dari harga
```

The semantic analyzer determines whether the operands support the requested operation. The parser does not infer types from the words alone.

## 9. Action statements

An action has a formally defined verb/phrase and explicitly defined arguments.

```text
action = display_action
        | add_action
        | remove_action
        | create_list_action
        | generic_named_action ;
```

### 9.1 Display

```text
display_action = TAMPILKAN, expression ;
```

Examples:

```text
tampilkan nama
tampilkan umur
tampilkan isi daftar belanja
```

### 9.2 Add to a collection

```text
add_action = TAMBAHKAN, expression, KE, name ;
```

Example:

```text
tambahkan apel ke daftar buah
tambahkan telur ke daftar belanja
```

Semantic form:

```text
Action.Add
  value
  target_collection
```

### 9.3 Remove from a collection

```text
remove_action = HAPUS, expression, DARI, name ;
```

Example:

```text
hapus telur dari daftar belanja
```

### 9.4 Create list

```text
create_list_action = BUAT, DAFTAR, name ;
```

Example:

```text
buat daftar belanja
buat daftar teman
```

### 9.5 Generic actions are constrained

The grammar MUST NOT accept an arbitrary first word as a generic executable action merely because an identifier exists.

A future extensibility mechanism may register explicit action declarations, but an undeclared phrase cannot acquire execution semantics by lexical coincidence.

This rule prevents a line such as:

```text
apel
```

from becoming an implicit action merely because `apel` is a valid name.

## 10. Collection access

Collection access is a distinct semantic operation from collection mutation.

The beginner surface MAY use:

```text
tampilkan isi daftar belanja
```

The grammar should normalize this into a structured access form such as:

```text
Collection.ReadAll
  target = daftar belanja
```

The phrase `isi` is therefore a defined grammar component in this context, not an instruction to perform fuzzy semantic search.

## 11. Condition

```text
conditional = KALAU, condition, NEWLINE, INDENT, block, DEDENT,
              [ KALAU TIDAK, NEWLINE, INDENT, block, DEDENT ] ;

condition = expression, comparison_operator, expression ;
```

Examples:

```text
kalau umur sudah 17 tahun
    tampilkan "Boleh masuk"
```

and:

```text
kalau umur sudah 17 tahun
    tampilkan "Boleh masuk"

kalau tidak
    tampilkan "Belum boleh masuk"
```

Semantic form:

```text
Condition
  condition
  then
  otherwise?
```

`kalau tidak` is a defined branch marker. It is not an arbitrary natural-language sentence.

## 12. Loop

### 12.1 For-each

```text
for_each = UNTUK SETIAP, name, DI, name,
           NEWLINE, INDENT, block, DEDENT ;
```

Example:

```text
untuk setiap nama di daftar teman
    sapa nama
```

The loop variable and collection are explicit semantic fields.

### 12.2 While-condition

```text
while_loop = SELAMA, condition,
             NEWLINE, INDENT, block, DEDENT ;
```

Example:

```text
selama jumlah lebih besar dari 0
    kurangi jumlah
```

The more conversational form:

```text
selama masih ada barang di daftar belanja
    ambil barang berikutnya
```

is **not accepted merely because it sounds natural**. It requires an explicit grammar production and semantic definition before it becomes part of v0.1.

This is a deliberate boundary between naturalness and formal determinism.

## 13. Function definition

### 13.1 Definition

```text
function = BUAT PEKERJAAN, name,
           NEWLINE, INDENT,
           { parameter },
           { statement, NEWLINE },
           result_clause,
           DEDENT ;

parameter = TERIMA, name, NEWLINE ;
result_clause = HASILNYA ADALAH, expression, NEWLINE ;
```

Example:

```text
buat pekerjaan hitung total
    terima harga
    terima pajak
    hasilnya adalah harga ditambah pajak
```

### 13.2 Function return

`hasilnya adalah` is a formal result clause and maps to a function return value.

A function with no return value is a separate semantic form and MUST NOT be inferred solely from the absence of `hasilnya adalah` if the definition grammar requires an explicit result policy.

### 13.3 Function call

```text
call_expression = name, DENGAN, expression,
                  { DAN, expression } ;
```

Example:

```text
total adalah hitung total dengan harga dan pajak
```

Named-argument call blocks MAY be added later:

```text
total adalah hitung total
    harga 100000
    pajak 11000
```

Such a form must have its own production and must not be implemented by heuristic line grouping.

## 14. Action-oriented functions

A named function can be invoked as an action only if its declaration explicitly exposes an action-call form.

Example target design:

```text
buat pekerjaan kirim pesan
    terima pesan
    terima penerima
    kirim pesan kepada penerima
```

Call form:

```text
kirim pesan
    pesan "Halo Budi"
    penerima Budi
```

The relationship between function declarations and action-call syntax is a semantic/type-system rule, not a lexical guess.

## 15. Modules and large programs

Modules are organizational constructs, not a new semantic execution category.

Proposed surface:

```text
aplikasi toko

    di bagian produk

        buat pekerjaan cari produk
        buat pekerjaan tambah produk
        buat pekerjaan hapus produk

    di bagian pembayaran

        buat pekerjaan bayar pesanan
        buat pekerjaan batalkan pembayaran
```

This is **not yet accepted as v0.1 grammar**. It is a design candidate requiring a separate module specification.

The semantic model should eventually represent:

```text
Module
  name
  declarations
  functions
  dependencies
```

The surface syntax must not require beginners to understand `import`, `export`, or package paths merely to organize a large project.

## 16. Ambiguity policy

One adopts the following deterministic policy:

1. A source line must match a complete grammar production.
2. Grammar alternatives may overlap only if their precedence and resulting AST are formally identical.
3. If two valid parses produce different semantic structures, the source is rejected as ambiguous.
4. Semantic name resolution may report multiple candidates, but it MUST NOT silently choose one.
5. Suggestions are diagnostics only and never mutate or reinterpret source.
6. Frequency, machine learning, LLM output, editor history, filesystem order, or execution history MUST NOT resolve language ambiguity.

Example semantic ambiguity:

```text
tambahkan apel
```

If the language context contains multiple eligible collections and no target is specified, this is not executable One source. A diagnostic may list candidates:

```text
EAMB001 — Tujuan penambahan belum jelas.
Ditemukan beberapa daftar yang cocok:
    daftar belanja
    daftar buah
    daftar makanan
Sebutkan daftar tujuan secara lengkap.
```

The diagnostic is not an interactive compiler request for an LLM. It is deterministic semantic feedback.

## 17. Error classes

The grammar stage reserves distinct error categories:

- `ELEXxxx` — lexical failure;
- `EPARSExxx` — no grammar production matches;
- `EAMBxxx` — more than one semantic interpretation remains;
- `ENAMExxx` — name resolution failure;
- `ETYPExxx` — type incompatibility;
- `ESEMxxx` — semantic rule violation;
- `ECAPxxx` — capability/permission violation.

Exact diagnostic wording belongs to the diagnostics specification.

## 18. Invalid examples

### Missing value

```text
nama adalah
```

Rejected: declaration requires an expression.

### Missing target

```text
tambahkan apel
```

Rejected in the strict v0.1 grammar because `add_action` requires `KE name`.

### Ambiguous target

```text
tambahkan apel ke daftar
```

Rejected if `daftar` alone does not uniquely identify a collection under the current name-resolution rules.

### Missing comparison operator

```text
kalau umur 17
    tampilkan "..."
```

Rejected: `condition` requires a comparison operator.

### Invalid indentation

```text
kalau umur sudah 17 tahun
  tampilkan "Boleh masuk"
```

Rejected by the lexical/structural whitespace rules.

### Unbounded natural language

```text
kayaknya besok tampilkan sesuatu yang bagus kalau user mungkin suka
```

Rejected: no production accepts this as a One statement.

## 19. AST mapping requirements

The concrete grammar must map into a small semantic vocabulary rather than preserve surface phrases as opaque strings.

Minimum semantic categories required by this grammar:

```text
Program
Data.Declare
Data.Assign
Action.Display
Action.Add
Action.Remove
Action.CreateList
Condition
Loop.ForEach
Loop.While
Function.Define
Function.Call
Module (future gate)
Value.Text
Value.Number
Value.Boolean
Value.Nothing
Reference
```

The exact AST node schema is deferred to the AST Contract stage.

## 20. Acceptance criteria

This grammar design can become the normative v0.1 grammar only when:

1. every production has valid examples;
2. every important invalid form has a rejection test;
3. ambiguous examples have deterministic rejection tests;
4. multi-word name boundaries are formally resolved with the lexical specification;
5. all six core constructs are represented without adding an unapproved seventh core construct;
6. grammar alternatives have no unresolved semantic overlap;
7. the grammar can represent nested conditions and loops;
8. function definition/call semantics are explicit;
9. module syntax is either explicitly included or explicitly deferred;
10. the grammar maps cleanly to the future AST contract;
11. no production requires an LLM, network call, or runtime state to parse;
12. the grammar remains consistent with the Language Invariants;
13. prior-art findings are reflected in the ambiguity and controlled-naturalness rules.

## 21. Design status

**Decision:** The project adopts the grammar direction **controlled natural programming**, not unrestricted natural-language programming.

**Not yet locked:**

- exact multi-word-name lexical boundary;
- complete module grammar;
- complete action registry/extensibility model;
- full type grammar;
- complete function action-call syntax;
- exact AST schema;
- exact precedence/associativity contract for all future operators.

These items are deliberately deferred rather than guessed.
