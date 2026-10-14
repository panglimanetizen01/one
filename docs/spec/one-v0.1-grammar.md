# One v0.1 Concrete Grammar

Status: normative for the initial Indonesian implementation.

This grammar intentionally defines a bounded programming language rather than accepting arbitrary Indonesian prose.

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

## Data

```text
declaration = identifier, "adalah", expression ;
assignment = identifier, "menjadi", expression ;
```

Examples:

```text
nama adalah "Budi"
umur adalah 25
umur menjadi 26
```

## Action

```text
action = "tampilkan", expression ;
```

## Condition

```text
conditional = "kalau", condition, NEWLINE, INDENT, block, DEDENT,
              [ "kalau tidak", NEWLINE, INDENT, block, DEDENT ] ;
```

The condition expression is a comparison or boolean expression. v0.1 comparisons are expressed as deterministic phrases:

```text
lebih besar dari
lebih kecil dari
sama dengan
sudah
belum
```

The implementation must reject a phrase that can be parsed in more than one way.

## Loop

```text
loop = for_each | while_loop ;
for_each = "untuk setiap", identifier, "di", expression, NEWLINE, INDENT, block, DEDENT ;
while_loop = "selama", condition, NEWLINE, INDENT, block, DEDENT ;
```

## Function

```text
function = "buat pekerjaan", identifier, NEWLINE, INDENT,
           { parameter_decl }, result_decl, block, DEDENT ;
parameter_decl = "terima", identifier, NEWLINE ;
result_decl = "hasilnya adalah", expression, NEWLINE ;
```

The v0.1 implementation represents function bodies explicitly; `hasilnya adalah` defines the returned expression.

## Event

```text
event = "ketika", event_trigger, NEWLINE, INDENT, block, DEDENT ;
event_trigger = identifier, { identifier | literal | phrase } ;
```

Event trigger matching is explicit. Free-form event descriptions are not accepted.

## Expressions

```text
expression = literal
           | identifier
           | list_expression
           | binary_expression
           | call_expression ;
```

Core arithmetic phrases:

```text
ditambah

dikurangi

dikali

dibagi
```

Precedence is fixed and documented in the parser contract. Parentheses are not required for core examples and are reserved for a future explicit expression extension.

## Lists

The v0.1 surface provides list construction through a dedicated action family in the standard library rather than punctuation-heavy literal syntax. This keeps the beginner surface consistent.

## Block rules

A block begins after a block-introducing statement and must be indented exactly one level. A block cannot be empty.

## Ambiguity rules

1. A phrase is recognized only when it matches a complete grammar production.
2. Longest defined phrase wins only when the grammar makes the alternative unambiguous.
3. If two complete productions remain possible, parsing fails with an ambiguity diagnostic.
4. The parser never invokes an LLM or fuzzy matching to choose an interpretation.

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
