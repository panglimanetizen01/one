# ONE Human Instruction Surface vNext — Grammar Draft

**Status:** PROPOSED DESIGN DRAFT — EXPERIMENTAL PARSER PROTOTYPE EXISTS; NOT NORMATIVE  
**Language identifier:** one-human-vnext (provisional; must be ratified before implementation)  
**Locale:** Indonesian canonical surface, id  
**Compatibility:** Separate from normative ONE v0.1. No v0.1 source is reinterpreted by this draft.

## 1. Purpose and conformance rule

This document narrows the product contract into a candidate grammar that can be implemented and tested. ONE remains a programming language with a learned, finite instruction vocabulary and deterministic semantics. It is not arbitrary prose interpreted by an AI model.

The ordinary source form must not require conventional programming punctuation to express program structure. This includes braces, parentheses, semicolons, commas, colons, symbolic operators, and quote marks. Newlines and indentation are structural whitespace. Ordinary words and fixed phrase patterns carry instruction meaning.

A program conforms only if every nonblank instruction line matches exactly one production in this version, every referenced name resolves, and every block boundary is valid. If there is no parse or more than one meaning-preserving parse, compilation fails with a diagnostic. No fuzzy fallback is permitted.

## 2. Source layout

- Encoding is UTF-8.
- LF and CRLF line endings are accepted and normalized before parsing.
- A blank line separates visual groups but has no semantic effect.
- Top-level instructions begin at column zero.
- A nested body is indented by exactly four spaces per level.
- Tabs are forbidden in indentation.
- Indentation is legal only after a production that requires a body.
- A block ends when indentation returns to its parent level.
- The file must end at a complete instruction boundary.
- No punctuation token is required to end an instruction. A newline ends a simple instruction; indentation delimits a body.

These rules intentionally retain a whitespace-based block model, not a punctuation-based one.

## 3. Lexical and name model

The lexer recognizes Unicode words, decimal integers, line boundaries, indentation transitions, and text-block boundaries. It does not treat punctuation as an instruction delimiter. Punctuation appearing in a text block is data.

Names are sequences of words in a grammar-defined name slot. Each production must identify where a name ends. The parser must not greedily absorb reserved phrase markers when they belong to the production. If a name and marker can be interpreted in two different ways, the source is ambiguous and rejected.

The initial implementation should keep names case-sensitive after Unicode normalization policy is decided. The implementation must not silently fold two distinct names into one. The normalization algorithm remains an explicit pre-implementation decision.

## 4. Text values without mandatory quotes

### 4.1 Chosen candidate: introduced indented text block

This draft chooses an explicit text-introduction phrase followed by an indented body for arbitrary text values. The introducing instruction is:

    Isi teks berikut
        Ini adalah isi teks
        Tanda baca seperti titik koma kurung dan tanda kutip
        tetap menjadi data bukan sintaks ONE

The block content is data, not executable ONE instructions. In the current starter prototype, exactly the first eight leading ASCII spaces are structural and removed; additional leading spaces are preserved as payload indentation. Blank lines inside the block are represented as empty payload lines, and a trailing source newline after a nonempty payload is preserved as a terminal LF. The parser records this value as text data, distinct from an executable statement block.

This candidate avoids requiring quote marks and permits punctuation and command-like words inside data. It does mean a multi-line text literal is visually a block. The implementation must test empty text, trailing newline, leading spaces, Unicode normalization, CRLF normalization, and round-trip fidelity before this rule can be promoted to normative status.

### 4.2 Inline text

Inline text without quotes is not yet standardized in this draft. An arbitrary sequence of words cannot safely be distinguished from a variable reference or instruction marker using guesswork. The first vertical slice should use named values or introduced text blocks for text data. A later reviewed production may introduce inline text using an unambiguous phrase and boundary rule.

The vNext compiler must never infer whether words are text or a variable based only on whether a name happens to exist.

## 5. Canonical starter productions

The following are candidate forms for the first prototype. They are intentionally a small subset, not a claim that the whole language is specified.

### 5.1 Program declaration

    Buat aplikasi daftar tugas

This declares a project artifact of kind application with project name daftar tugas. The phrase Buat aplikasi is reserved in this production; the remaining name is the project identifier. The first prototype should require exactly one project declaration at the start of a project file.

### 5.2 User action declarations

    Pengguna bisa menambahkan tugas
        Terima judul tugas
        Simpan tugas baru

    Pengguna bisa melihat semua tugas
        Tampilkan semua tugas

    Pengguna bisa menandai tugas sebagai selesai
        Terima tugas yang dipilih
        Tandai tugas sebagai selesai

These declare named user actions. The words after Pengguna bisa are the action name; body instructions specify the action behavior. The initial vertical slice must implement only the action patterns it can prove end-to-end. Unsupported body instructions are compile errors, not placeholders.

### 5.3 Persistence requirement

    Simpan semua tugas agar tidak hilang saat aplikasi ditutup

This is a requirement-level phrase, not a runtime implementation by itself. The compiler must lower it to an explicit persistence requirement node and the runtime/backend must bind that node to a documented storage adapter. The first prototype must choose a concrete storage location, schema, write atomicity, restart behavior, and error policy. No persistence claim is valid until a test starts the program twice and observes the same stored tasks.

### 5.4 Data declaration and text block

    Buat data pesan
        Isi teks berikut
            Halo dunia

The semantic meaning is a variable named pesan whose value is the exact text block Halo dunia. This is a candidate production. It must be tested against nested statement blocks so a text block cannot be confused with executable instructions.

### 5.5 Display

    Tampilkan pesan

This resolves pesan as a value and emits its display representation. If pesan is unresolved, compilation fails. It is not treated as the literal text pesan.

## 6. Versioned prototype grammar

**Grammar revision:** `one-human-vnext/0.1-draft`  
**Status:** deterministic prototype contract; not yet normative.

The grammar below describes the accepted source subset. It does not claim that unsupported general-purpose language features exist.

```ebnf
program              = project-declaration, newline, { top-level-item }, EOF ;
top-level-item       = blank-line | data-declaration | user-action | persistence-requirement ;
project-declaration  = "Buat aplikasi ", name ;
data-declaration     = "Buat data ", name, newline,
                       indent-4, "Isi teks berikut", newline, text-block ;
user-action          = "Pengguna bisa ", name, newline,
                       indent-4, action-body ;
action-body          = action-instruction, { newline, action-instruction } ;
action-instruction   = display-instruction
                     | "Terima judul tugas"
                     | "Simpan tugas baru"
                     | "Tampilkan semua tugas"
                     | "Terima tugas yang dipilih"
                     | "Tandai tugas sebagai selesai" ;
display-instruction  = "Tampilkan ", name ;
persistence-requirement
                     = "Simpan semua ", name,
                       " agar tidak hilang saat aplikasi ditutup" ;
name                 = name-character, { name-character | " ", name-character } ;
name-character       = unicode-alphanumeric | "_" | "-" ;
indent-4             = "    " ;
indent-8             = "        " ;
newline              = LF | CRLF ;
blank-line           = zero-or-more-spaces, newline ;
text-block           = { text-payload-line } ;
text-payload-line    = indent-8, source-text, newline ;
```

The EBNF is paired with these boundary rules because a text block carries raw lines rather than parsed instructions:

1. The first nonblank line must be the single project declaration. A second project declaration is rejected as an unknown top-level instruction.
2. Names are case-sensitive UTF-8 strings composed of Unicode alphanumeric characters, underscores, hyphens and single ASCII spaces between characters/words. Leading/trailing or repeated spaces, punctuation, tabs and control characters are rejected in names. The implementation does not normalize Unicode; canonically equivalent spellings remain distinct. Combining marks that are not classified as alphanumeric by Rust's standard library are currently outside the name grammar.
3. Every top-level declaration starts at column zero. The action body starts at four spaces. Text payload lines use at least eight leading spaces; exactly eight are structural and any additional spaces are retained as payload indentation.
4. LF and CRLF are accepted. CRLF is normalized to LF in text values. UTF-8 payload content is preserved without Unicode normalization.
5. Blank lines within a text payload are data. A blank line after the final nonblank payload line and before the next top-level declaration is also payload, so it adds a trailing newline to that value. Blank lines outside text blocks are ignored. This removes guessing at the cost of requiring authors to understand that blank lines following a text block belong to the value.
6. An empty text block produces the empty string. A terminal newline in a nonempty text block is preserved as a terminal LF. Text payload is opaque and is never parsed as executable ONE source.
7. Duplicate data/action names and duplicate persistence declarations are rejected. A display reference must resolve to a declared data value. Unknown instructions are rejected; there is no AI or fuzzy fallback.
8. The task-list runtime accepts only the supported action-body sequences described in section 13. Parsing another action phrase does not imply that the runtime can execute it.

This is a formal contract for the current starter subset, not a complete grammar for the future general-purpose language. Unsupported loops, conditions, types, functions, modules, arbitrary input/output, and general persistence declarations remain unimplemented and must fail rather than silently degrade.

## 7. AST contract for the prototype

The versioned AST should preserve intent rather than source-language phrasing:

- ProjectDeclaration { name, artifact_kind }
- UserActionDeclaration { name, parameters, body }
- DataDeclaration { name, value }
- TextBlock { exact_text }
- Display { value_reference }
- PersistenceRequirement { collection, durability_scope }

Each node also carries a source span for diagnostics. Locale-specific words lower to the same node shape. The exact Rust type layout is an implementation choice, but golden tests must lock the observable semantic shape.

## 8. Semantic rules

1. A project file has exactly one project declaration, before all other top-level declarations.
2. Each action name is unique within a project.
3. Each data name is unique in its scope.
4. Each reference must resolve at compile time unless the production explicitly declares a runtime-bound value.
5. A display instruction reads a value; it does not create or mutate one.
6. A text block is opaque data and is never recursively parsed as ONE code.
7. A persistence requirement must bind to an available persistence implementation or fail before execution.
8. Unsupported action body instructions are rejected.
9. No AI or network access is used to decide parsing, name resolution, or semantics.
10. Every side effect must pass the existing capability and execution-policy boundary.

## 9. Negative grammar fixtures

The prototype must reject, with stable diagnostic categories:

- a missing project declaration;
- two project declarations;
- an action declaration with no body;
- an instruction body at the wrong indentation level;
- tabs used for indentation;
- an unknown instruction phrase;
- a reference to an undeclared value;
- duplicate action or data names;
- a persistence requirement when no compatible adapter exists;
- a line that has two plausible parses;
- text-block content accidentally parsed as executable instructions;
- an unterminated or malformed text block;
- unsupported inline free-form text where a value reference is required.

## 10. Positive fixtures

At minimum, the parser fixture set must include:

- one project declaration;
- multiple actions with deterministic body boundaries;
- a text block containing punctuation and quote characters as data;
- a text block containing words that resemble ONE instructions;
- Unicode text and mixed whitespace inside text data;
- blank lines between declarations;
- nested text and instruction blocks with different indentation depths;
- a complete task-list source fixture;
- exact expected AST output for every accepted fixture.

Runtime fixtures must separately verify add, list, complete, persistence across restart, and storage failure behavior. A parser fixture alone cannot claim application behavior works.

## 11. Versioning and CLI isolation

The current prototype is explicitly isolated and invoked with:

    one jalankan-human-vnext examples/human-vnext/daftar-tugas.one
    one jalankan-human-vnext examples/human-vnext/daftar-tugas.one --data ./data/tugas.db

The default `one jalankan`, `one periksa`, build, and package commands continue to use the normative v0.1 parser. There is no automatic fallback between the two source languages. The vNext command currently implements only the task-list vertical slice; it is not a general-purpose vNext compiler and must be labelled experimental.

The provisional language identifier is `one-human-vnext`. Names are case-sensitive and preserved as UTF-8 without Unicode normalization. Therefore canonically equivalent Unicode spellings are not currently folded together. This is an explicit limitation, not an implicit normalization policy.

Text-block rules implemented by the prototype:
- LF and CRLF are accepted; CRLF line endings are normalized.
- Eight leading spaces identify text payload lines. Additional indentation beyond those eight spaces is preserved as data.
- Blank lines inside the text payload are preserved.
- A terminal source newline inside the payload is preserved as a terminal newline in the value.
- Tabs are rejected in structural indentation. Control characters in task titles are rejected by the current line-based storage format.
- Unknown instructions and unresolved display references fail with stable prototype diagnostic codes. These codes are experimental and not yet a public compatibility promise.

Before release, the repository must decide the stable language identifier, project manifest declaration, migration policy, Unicode normalization policy, and how formatters and editor tooling identify the language version.

## 12. Prototype implementation gate

Do not promote this draft to normative until all of the following are true:

- grammar decisions left open above are resolved;
- text blocks round-trip exactly under a documented newline/indentation policy;
- parser positive and negative fixtures pass;
- AST and semantic lowering golden tests pass;
- the task-list vertical slice runs through the public toolchain;
- persistence survives process restart and reports failures accurately;
- capability checks are enforced;
- Linux and Windows verification is recorded for the exact commit;
- beginner documentation uses only tested syntax;
- review confirms v0.1 behavior is unchanged.

## 13. Current status

This remains a proposed, non-normative grammar draft. The parser and task-list runner live in `crates/one-core/src/human_vnext.rs`; the explicit CLI entry point is `one jalankan-human-vnext`; the runnable source fixture is `examples/human-vnext/daftar-tugas.one`.

The current vertical slice recognizes only the exact task app action names and add/list/complete instruction sequences, rejects data declarations and persistence collections it cannot execute, validates that exactly one supported persistence requirement is declared, stores records in a tab-delimited UTF-8 file through an exclusive temporary file and replacement rename, and reloads the file at startup. It does not yet lower the human-vNext AST into ONE's canonical v0.1 IR, and the task runner is a dedicated experimental interpreter for this narrow app shape. Therefore the general human-vNext compiler/execution pipeline requirement remains **NOT IMPLEMENTED**.

Regression tests were added for text-block fidelity, CRLF/relative indentation, task persistence across sessions, completion persistence, missing persistence declarations, and corrupted storage. They are committed but remain **NOT VERIFIED** until the Rust tests and exact-SHA Linux/Windows CI execute successfully. The CI run currently fails before any job steps start, so this is not evidence of either a code failure or a code pass.

The task-list slice is implemented in source but must not be described as verified or production-ready until tests, CI, error-path review, and v0.1 regression checks pass.
