# One v0.1 Concrete Grammar

Status: **normative** for the initial Indonesian implementation.

This grammar intentionally defines a bounded programming language rather than accepting arbitrary Indonesian prose. The implementation must reject incomplete, ambiguous, or unsupported phrases instead of guessing.

## Notation

EBNF-like notation is used:

- `=` defines a production.
- `|` means alternative.
- `?` means optional.
- `*` means zero or more.
- Quoted text is a literal phrase.
- `INDENT` and `DEDENT` are lexical structure tokens.

## Program

```text
program = { statement | blank_line } ;
statement = declaration | assignment | action | conditional | loop | function | event ;
```

A physical source line is terminated by `NEWLINE`. Blank lines and comment-only lines do not create statements.

## Data

```text
declaration = identifier, "adalah", expression, NEWLINE ;
assignment = identifier, "menjadi", expression, NEWLINE ;
```

v0.1 identifiers are **single lexical words** (`nama`, `umur`, `belanja`). Multi-word user-facing names such as `daftar belanja` are a future grammar extension; v0.1 uses a single identifier for the collection name.

Examples:

```text
nama adalah "Budi"
umur adalah 25
umur menjadi 26
```

## Action

```text
action = display_action
        | create_list_action
        | add_list_action
        | remove_list_action
        | generic_action ;

display_action = "tampilkan", expression ;
create_list_action = "buat", "daftar", expression ;
add_list_action = "tambahkan", expression, "ke", expression ;
remove_list_action = "hapus", expression, "dari", expression ;
generic_action = identifier, [ expression ] ;
```

Core examples:

```text
buat daftar belanja
tambahkan beras ke belanja
hapus telur dari belanja
tampilkan belanja
```

The first line above is represented semantically as an action named `buat daftar` whose argument is `belanja`. The words `beras` and `telur` in the examples are identifiers; if literal text is intended, quote it explicitly.

## Condition

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

The optional trailing word `tahun` is accepted only for `sudah`/`belum` age-style comparisons. The parser does not infer other omitted units.

## Loop

```text
loop = for_each | while_loop ;
for_each = "untuk", "setiap", identifier, "di", expression,
           NEWLINE, INDENT, block, DEDENT ;
while_loop = "selama", condition, NEWLINE, INDENT, block, DEDENT ;
```

`selama` in v0.1 uses the same explicit comparison grammar as `kalau`. The more conversational form `selama masih ada barang di daftar belanja` is reserved for a future loop-condition extension and is not silently accepted by v0.1.

## Function

```text
function = "buat", "pekerjaan", identifier, NEWLINE, INDENT,
           { parameter_decl }, result_decl, { body_statement }, DEDENT ;
parameter_decl = "terima", identifier, NEWLINE ;
result_decl = "hasilnya", "adalah", expression, NEWLINE ;
```

A function may end immediately after its result declaration, or may contain additional body statements. The result expression is explicit and deterministic.

Example:

```text
buat pekerjaan hitung
    terima harga
    terima pajak
    hasilnya adalah harga ditambah pajak
```

Calls use the explicit `dengan` / `dan` form:

```text
total adalah hitung dengan harga dan pajak
```

## Event

```text
event = "ketika", event_trigger, NEWLINE, INDENT, block, DEDENT ;
event_trigger = identifier, { identifier | literal } ;
```

Event trigger matching is exact after token normalization. Free-form event descriptions are not accepted as hidden semantics.

Example:

```text
ketika pengguna menekan tombol beli
    tampilkan "dibeli"
```

## Expressions

```text
expression = primary, { binary_operator, primary } ;
primary = literal | identifier | call_expression ;
call_expression = identifier, "dengan", primary, { "dan", primary } ;
```

Core arithmetic operators:

```text
ditambah
dikurangi
dikali
dibagi
```

Comparison operators are listed above. v0.1 evaluation is left-associative for repeated binary operators; parentheses are not part of the v0.1 grammar.

## Literals

```text
literal = text | integer | boolean | nothing ;
text = quoted UTF-8 string with the supported escapes;
integer = one or more ASCII decimal digits;
boolean = "benar" | "salah";
nothing = the explicit absence value used by runtime semantics;
```

## Lists

v0.1 list construction and mutation use dedicated action forms rather than punctuation-heavy list literals. This keeps the beginner surface consistent. A list name is a normal single-word identifier.

## Block rules

A block begins after a block-introducing statement and must be indented exactly one level relative to its parent. One indentation level is exactly four spaces. Tabs are rejected. A block cannot be empty.

## Comments and blank lines

- `#` begins a comment outside a quoted string.
- A comment-only line is ignored.
- Blank lines are ignored.
- Indentation on a comment-only or blank line does not alter block structure.

## Ambiguity rules

1. A phrase is recognized only when it matches a complete grammar production.
2. Longest defined phrase wins only when the grammar makes the alternative unambiguous.
3. If two complete productions remain possible, parsing fails with an ambiguity diagnostic.
4. The parser never invokes an LLM or fuzzy matching to choose an interpretation.
5. Unsupported conversational variants are rejected rather than treated as aliases.

## Invalid examples

```text
kalau umur 17
    tampilkan "..."
```

Invalid because the comparison operator phrase is missing.

```text
kalau umur sudah 17 tahun
  tampilkan "..."
```

Invalid because indentation is not four spaces.

```text
nama menjadi
```

Invalid because the assignment has no expression.

```text
selama masih ada barang di belanja
    tampilkan barang
```

Invalid in v0.1 because this conversational loop condition is reserved for a future grammar extension.
