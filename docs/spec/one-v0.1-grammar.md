# One v0.1 Concrete Grammar

Status: **normative** for the initial Indonesian implementation.

This grammar defines a bounded programming language rather than accepting arbitrary Indonesian prose. The implementation MUST reject incomplete, unsupported, or semantically ambiguous source instead of guessing.

## 1. Notation

EBNF-like notation is used:

- `=` defines a production.
- `|` means alternative.
- `?` means optional.
- `*` means zero or more.
- Quoted text is a literal word or phrase component.
- `INDENT` and `DEDENT` are lexical structure tokens.
- `NEWLINE` terminates a physical statement.

The grammar is defined over lexical `WORD`, `NUMBER`, `TEXT`, `NEWLINE`, `INDENT`, and `DEDENT` tokens. A multi-word user-defined name is never a lexical token; it is constructed by a production below.

## 2. Program

```text
program = { statement | blank_line } ;
statement = declaration
          | assignment
          | action
          | conditional
          | loop
          | function
          | event ;
```

A physical source line is terminated by `NEWLINE`. Blank lines and comment-only lines do not create statements.

## 3. Names

### 3.1 General name model

```text
name = name_word, { name_word } ;
name_word = WORD eligible at the current production position ;
```

`name` is a grammar-level construct, not a lexer token.

The parser MUST determine a name boundary from the enclosing production. It MUST NOT consume all remaining words merely because they could form a plausible name.

A structural word such as `adalah`, `menjadi`, `ke`, or `dari` terminates a name when the active production requires that word as its boundary.

A structural spelling is not globally prohibited inside every possible name. Its interpretation is determined by the active grammar production.

### 3.2 Name boundary rule

For each production, the parser MUST identify the complete syntactic role containing `name` and its terminator.

If exactly one complete semantic parse exists, it is accepted.

If no complete parse exists, parsing fails.

If multiple complete parses remain and their semantic structures differ, parsing fails with `EAMBxxx`.

The parser MUST NOT resolve this ambiguity using longest-match heuristics, frequency, editor history, filesystem state, execution history, machine learning, or an LLM.

Equivalent parses that normalize to the same semantic structure MAY be collapsed before ambiguity reporting.

## 4. Data

```text
declaration = name, "adalah", expression, NEWLINE ;
assignment = name, "menjadi", expression, NEWLINE ;
```

Examples:

```text
nama adalah "Budi"
nama pengguna adalah "Budi"
saldo rekening menjadi 100000
```

For `nama pengguna adalah "Budi"`, the declaration name is `nama pengguna`; `adalah` is the structural boundary and is not part of the name.

## 5. Action

Core v0.1 actions are explicit productions. There is **no generic action production** that turns an arbitrary identifier into an executable command.

```text
action = display_action
        | greet_action
        | create_list_action
        | add_list_action
        | remove_list_action
        | dns_discovery_action
        | filesystem_read_action
        | filesystem_write_action
        | process_execute_action
        | database_query_action
        | http_get_action ;

display_action = "tampilkan", expression, NEWLINE ;
greet_action = "sapa", expression, NEWLINE ;
create_list_action = "buat", "daftar", name, NEWLINE ;
add_list_action = "tambahkan", expression, "ke", name, NEWLINE ;
remove_list_action = "hapus", expression, "dari", name, NEWLINE ;
dns_discovery_action = "cari", "alamat", expression, NEWLINE ;
filesystem_read_action = "baca", "berkas", expression, NEWLINE ;
filesystem_write_action = "tulis", "ke", "berkas", expression, "dengan", expression, NEWLINE ;
process_execute_action = "jalankan", "proses", expression, NEWLINE ;
database_query_action = "kueri", "database", expression, "dengan", expression,
                        { "dan", expression }, NEWLINE ;
http_get_action = "ambil", "dari", expression, NEWLINE ;
```

Examples:

```text
buat daftar belanja
tambahkan apel ke daftar buah
hapus telur dari daftar belanja
tampilkan nama pengguna
sapa teman
cari alamat "127.0.0.1"
baca berkas "data/contoh.txt"
tulis ke berkas "data/hasil.txt" dengan "halo dari One"
jalankan proses "rustc"
kueri database ":memory:" dengan "SELECT ?1" dan "42"
ambil dari "http://127.0.0.1:8080/"
```

`cari alamat`, `baca berkas`, `tulis ke berkas`, `jalankan proses`, `kueri database`, and `ambil dari` are source-level orchestration productions. Their effectful execution is governed by explicit runtime capability policy; grammar recognition alone does not grant permission.

The database-query production is intentionally bounded to a database resource, one SQL expression, and zero or more explicit parameter expressions. The current implementation boundary accepts read-only parameterized SQLite `SELECT` statements; broader database mutation/query semantics are not implied by this grammar entry.

The HTTP GET production is intentionally bounded to an explicit `http://` URL. The current implementation uses the existing bounded HTTP/1.1 runtime, performs GET only, returns the UTF-8-normalized response body, and requires explicit `network.connect` permission scoped to the exact URL. HTTPS, arbitrary methods, request headers/bodies, redirects, cookies, and API-specific authentication are outside this source-level slice.

The fixed words `buat daftar`, `ke`, and `dari` are structural boundaries for their respective productions. The fixed two-word prefixes `cari alamat`, `baca berkas`, `jalankan proses`, `kueri database`, and `ambil dari`, and the fixed three-word prefix `tulis ke berkas`, likewise identify distinct semantic operations; arbitrary prose is not mapped to these operations.

The source line:

```text
apel ke daftar buah
```

is not an action merely because it contains an `ke` phrase. No core action production begins with `apel`.

## 6. Condition

```text
conditional = "kalau", condition, NEWLINE, INDENT, block, DEDENT,
              [ "kalau", "tidak", NEWLINE, INDENT, block, DEDENT ] ;

condition = expression, comparison_operator, expression ;

comparison_operator = "sudah"
                    | "belum"
                    | "sama", "dengan"
                    | "lebih", "besar", "dari"
                    | "lebih", "kecil", "dari" ;
```

Examples:

```text
kalau umur sudah 17 tahun
    tampilkan "Boleh masuk"
kalau tidak
    tampilkan "Belum boleh masuk"
```

The optional trailing word `tahun` is accepted only for the approved `sudah`/`belum` age-style comparison form. The parser does not infer other omitted units.

Unsupported conversational modifiers remain invalid.

For example:

```text
kalau nama pengguna mungkin benar
    tampilkan nama pengguna
```

is rejected because `mungkin` is not a comparison operator.

## 7. Loop

```text
loop = for_each | while_loop ;

for_each = "untuk", "setiap", name, "di", name,
           NEWLINE, INDENT, block, DEDENT ;

while_loop = "selama", condition,
             NEWLINE, INDENT, block, DEDENT ;
```

Examples:

```text
untuk setiap teman di daftar teman
    tampilkan teman
```

The form:

```text
selama masih ada barang di daftar belanja
    tampilkan barang
```

remains invalid in v0.1 because no explicit production assigns semantics to `masih ada ... di ...`. It is a future loop-condition extension and MUST NOT be inferred.

## 8. Function

```text
function = "buat", "pekerjaan", name, NEWLINE, INDENT,
           { parameter_decl }, result_decl, { body_statement }, DEDENT ;

parameter_decl = "terima", name, NEWLINE ;
result_decl = "hasilnya", "adalah", expression, NEWLINE ;
```

Examples:

```text
buat pekerjaan hitung total
    terima harga barang
    terima pajak
    hasilnya adalah harga barang ditambah pajak
```

The fixed phrase `buat pekerjaan` ends the function header prefix; the following eligible words form the function name until the production's `NEWLINE` boundary.

Function calls use the explicit `dengan` / `dan` form:

```text
total adalah hitung total dengan harga dan pajak
```

Call syntax is explicit. The parser does not treat an arbitrary sentence as a function call.

## 9. Event

```text
event = "ketika", event_trigger, NEWLINE, INDENT, block, DEDENT ;

event_trigger = name, { event_word | literal } ;
```

Event trigger matching is exact after token normalization. Free-form event descriptions are not accepted as hidden semantics.

Example:

```text
ketika pengguna menekan tombol beli
    tampilkan "dibeli"
```

The event grammar still requires an explicit production and semantic event representation. It must not be implemented as arbitrary natural-language intent recognition.

## 10. Expressions

```text
expression = primary, { binary_operator, primary } ;

primary = literal
        | name_reference
        | call_expression ;

name_reference = name ;

call_expression = name, "dengan", primary,
                  { "dan", primary } ;

binary_operator = "ditambah"
                | "dikurangi"
                | "dikali"
                | "dibagi" ;
```

Comparison operators are defined separately by `comparison_operator`.

v0.1 evaluation is left-associative for repeated binary operators. Parentheses are not part of the core v0.1 grammar.

A `name` may appear as an expression only because `name_reference` explicitly permits it. This rule prevents arbitrary sentence tails from silently becoming identifiers.

## 11. Literals

```text
literal = text | integer | boolean | nothing ;
text = quoted UTF-8 string with the supported escapes ;
integer = one or more ASCII decimal digits ;
boolean = "benar" | "salah" ;
nothing = "tidak", "ada" ;
```

The lexical layer emits the component words of `tidak ada`; the grammar gives the phrase its value-level meaning.

## 12. Lists

v0.1 list construction and mutation use dedicated action forms rather than punctuation-heavy list literals.

A collection target is represented by `name` where the action production explicitly requires a target name.

Examples:

```text
buat daftar belanja
tambahkan beras ke daftar belanja
hapus telur dari daftar belanja
tampilkan isi daftar belanja
```

`isi` is a defined part of the collection-read surface when the corresponding expression/action production is added to the normative collection grammar. It is not a semantic search term.

## 13. Block rules

A block begins after a block-introducing statement and must be indented exactly one level relative to its parent. One indentation level is exactly four spaces. Tabs are rejected.

A block cannot be empty.

`kalau tidak` is a structural two-word phrase inside the conditional production, not a lexical identifier.

## 14. Comments and blank lines

- `#` begins a comment outside a quoted string.
- A comment-only line is ignored.
- Blank lines are ignored.
- Indentation on a comment-only or blank line does not alter block structure.

## 15. Ambiguity rules

The following rules are normative:

1. A phrase is accepted only when it matches a complete grammar production.
2. Multi-word names are bounded by the production that contains them.
3. A structural keyword required by the active production terminates the corresponding name role.
4. No general longest-name or longest-phrase heuristic is allowed.
5. If two complete parses produce the same semantic structure, they may be normalized to one result.
6. If two complete parses produce different semantic structures, parsing fails with `EAMBxxx`.
7. If zero complete parses exist, parsing fails with the appropriate parse diagnostic.
8. The parser never invokes an LLM, fuzzy matching, probability, editor history, filesystem order, or execution history to select a meaning.
9. Unsupported conversational variants are rejected rather than treated as aliases.
10. Arbitrary leading words never become executable actions without an explicit action production.

## 16. Invalid examples

Missing comparison operator:

```text
kalau umur 17
    tampilkan "..."
```

Invalid indentation:

```text
kalau umur sudah 17 tahun
  tampilkan "..."
```

Missing assignment expression:

```text
nama menjadi
```

Unsupported conversational loop condition:

```text
selama masih ada barang di daftar belanja
    tampilkan barang
```

Arbitrary prose mistaken for a name/action:

```text
saya ingin kamu menampilkan nama pengguna
```

Implicit action:

```text
apel
```

Unresolved target wording:

```text
tambahkan apel entah ke mana
```

Ambiguous name boundary, where two complete semantic parses are possible under an extension, MUST produce `EAMBxxx` rather than silently selecting the longer or shorter name.

## 17. Required grammar acceptance tests

The conformance suite MUST include, at minimum:

### Valid

1. single-word declaration names;
2. two-word declaration names;
3. three-or-more-word declaration names;
4. multi-word assignment names;
5. multi-word collection targets after `ke`;
6. multi-word collection targets after `dari`;
7. multi-word references after `tampilkan`;
8. multi-word function names;
9. multi-word parameter names;
10. nested blocks containing multi-word references;
11. reserved multi-word phrases such as `kalau tidak` and `buat pekerjaan`;
12. explicit `sapa` actions;
13. explicit `cari alamat` source orchestration;
14. explicit `baca berkas` source orchestration;
15. explicit `tulis ke berkas` source orchestration;
16. explicit `jalankan proses` source orchestration;
17. explicit `kueri database` source orchestration with a database resource, SQL, and parameter list;
18. explicit `ambil dari` HTTP source orchestration;
19. repeated parsing of identical input with identical semantic output.

### Invalid

20. tabs in indentation;
21. incomplete declaration/assignment;
22. unsupported natural-language filler;
23. arbitrary first-word actions;
24. unsupported loop-condition prose;
25. malformed reserved phrases;
26. ambiguous multi-word boundaries that yield different semantics;
27. names swallowing structural keywords;
28. missing action targets;
29. source spans that cannot cover the complete multi-word name;
30. orchestration source without the required explicit operation prefix;
31. database query source without a database resource or SQL expression;
32. HTTP source using an unsupported scheme instead of explicit `http://`.

## 18. Compatibility and implementation gate

This document supersedes the previous normative assumption that v0.1 identifiers are only single lexical words.

The implementation MUST NOT claim conformance to grammar-bounded multi-word names until:

1. this grammar and the lexical specification agree;
2. the AST contract defines the representation of `Name`/name references;
3. parser tests cover the required boundary cases;
4. ambiguity and diagnostic tests exist;
5. the reference semantic representation preserves the complete name span.

The `cari alamat`, `baca berkas`, `tulis ke berkas`, `jalankan proses`, `kueri database`, and `ambil dari` productions are source-level orchestration boundaries. Their grammar recognition does not imply execution permission; runtime governance remains mandatory.

No parser or lexer implementation change is authorized merely by this document. Implementation follows only after the specification gate is accepted.

## 19. Design decision

**Approved direction:** controlled Indonesian surface syntax with lexical `WORD` tokens, grammar-bounded multi-word names, explicit action productions including the currently verified orchestration forms, deterministic ambiguity rejection, and no implicit natural-language intent inference.

Principle:

> Natural on the surface; formal underneath.
