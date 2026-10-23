# ONE Human-Instruction vNext Starter Grammar

**Status:** Reviewed and accepted for the implementation-bounded experimental subset; not normative for ONE generally. See [the review record](../verification/human-vnext-grammar-review.md).  
**Version:** human-vnext-starter/0.1  
**Implementation entry point:** one_core::human_vnext::parse, selected only by the explicit jalankan-human-vnext CLI command.  
**Compatibility:** The default ONE v0.1 lexer, parser, AST, and CLI behavior are unchanged.

This contract describes only the deterministic subset implemented by the current experimental parser and task-list runner. It is not a claim that arbitrary Indonesian instructions are executable.

## 1. Design invariants

1. Parsing is deterministic and does not call an LLM or infer omitted intent.
2. Unsupported or ambiguous syntax is rejected; it is never silently reinterpreted as another construct.
3. The first non-blank line is the application declaration.
4. Top-level declarations start at column zero. Nested instruction lines use four spaces per indentation level; tabs are invalid.
5. Line endings may be LF or CRLF. Source text in an indented text block is data, not executable instructions.
6. Names are compared exactly and case-sensitively. No Unicode normalization or locale-dependent case folding is performed.
7. This version's identifiers allow Unicode alphanumeric characters, underscore, hyphen, and single ASCII spaces between non-space characters. Leading/trailing/repeated spaces and punctuation are rejected.
8. A declaration or instruction outside this grammar is an error. A successful parse is not by itself proof that a program can execute: the current runner accepts only the task-list vertical slice.

## 2. Versioning and compatibility rules

- The contract identifier is `human-vnext-starter/0.1`. It is a versioned experimental grammar, not the version of the ONE language as a whole.
- A source file has no implicit grammar-version negotiation. It is parsed only when the caller explicitly selects the human-vNext entry point; the ordinary v0.1 CLI commands never auto-detect or fall back to this grammar.
- A future incompatible change to keywords, identifier validity, indentation, text-block boundaries, declaration binding, or diagnostic codes must use a new contract version and add fixtures. It must not silently change the meaning of a file accepted by this version.
- This contract treats source as UTF-8 text supplied to the parser. It preserves Unicode text as-is and does not normalize it. Names are exact, case-sensitive strings.
- Duplicate data/action names share one declaration namespace and are rejected with `H0017`. A display reference resolves only to a data declaration; references to action names and missing names are rejected with `H0018`. Repeated persistence declarations for the same collection are rejected with `H0019`.
- Diagnostic codes are part of the experimental conformance surface. Tests assert codes for stable rejection classes; line numbers identify the declaration that caused a semantic conflict or unresolved reference.

## 3. Lexical conventions

- NL is LF or CRLF.
- SP is one ASCII space.
- INDENT is exactly four ASCII spaces for each nesting level.
- NAME is a non-empty sequence of Unicode alphanumeric characters, underscore, or hyphen, optionally separated by one ASCII space; it may not begin/end with a space.
- Blank lines are ignored for declaration parsing. Inside a text block, blank lines and relative indentation are preserved as text.
- Keywords are literal, case-sensitive Indonesian strings shown below. No synonym expansion is implied.

## 4. Starter grammar (EBNF)

```ebnf
program              = blank*, application, (blank* declaration)*, blank* ;
application          = "Buat aplikasi ", NAME, NL ;
declaration          = data_declaration
                     | action_declaration
                     | persistence_declaration ;

data_declaration     = "Buat data ", NAME, NL, text_block_header, text_block_payload ;
text_block_header    = INDENT, "Isi teks berikut", NL ;
text_block_payload   = { blank | text_payload_line } ;
text_payload_line    = INDENT, INDENT, TEXT, NL ;
action_declaration   = "Pengguna bisa ", NAME, NL, instruction_block ;
instruction_block    = instruction, (NL, instruction)* ;
instruction          = INDENT, (display_instruction
                     | accept_title
                     | add_task
                     | display_all_tasks
                     | accept_task_id
                     | complete_task) ;
display_instruction  = "Tampilkan ", NAME ;
accept_title         = "Terima judul tugas" ;
add_task             = "Simpan tugas baru" ;
display_all_tasks    = "Tampilkan semua tugas" ;
accept_task_id       = "Terima tugas yang dipilih" ;
complete_task        = "Tandai tugas sebagai selesai" ;
persistence_declaration
                     = "Simpan semua ", NAME,
                       " agar tidak hilang saat aplikasi ditutup", NL ;
blank                = NL ;
```

This EBNF is a structural summary, not a standalone scanner specification: the parser first identifies nonblank structural lines and then collects raw source lines for text blocks. For a text block, a nonblank payload line must have at least eight leading ASCII spaces. Exactly the first eight spaces are removed from the stored payload; any additional spaces or tabs after that prefix are retained as payload content, with CRLF line endings normalized to LF. Payload contents are not subject to structural indentation or tab validation. A tab before the eight-space prefix does not qualify as payload indentation and is rejected by structural validation. Blank lines between payload lines are preserved as empty payload lines. Trailing blank lines before the next nonblank structural line are also preserved when the block contains at least one payload line. If there is no payload line, the value is the empty string. A final source newline alone does not create an extra newline in an otherwise empty block. A text block ends at the next nonblank line with fewer than eight leading spaces, or at end-of-file.

For structural lines, the parser rejects tabs and indentation widths that are not multiples of four. Application, data, action, and persistence declarations must begin at column zero; the text-block header is indented four spaces; action instructions are indented four spaces; text payload lines are indented at least eight spaces. Blank source lines are ignored while finding structural lines but are interpreted as text data while collecting a text block according to the rules above.

**Important implementation boundary:** this grammar defines only the currently supported forms and exact text-block behavior. It does not define a general-purpose expression language, and a successful parse does not imply that every parsed program is executable by the current task-list runner.

## 5. Name binding and rejection rules

- The application declaration must be first (ignoring blank lines) and must not be indented.
- Duplicate data/action names are rejected by the parser's current declaration namespace.
- Duplicate persistence declarations are rejected.
- A display instruction must resolve to a declared data name; an action name is not a data value.
- Unknown action-body instructions, missing text blocks, empty/invalid names, tabs, invalid indentation, and orphaned nested lines are rejected.
- The current prototype's persistence runner requires exactly one collection named `tugas`, no data declarations, and these exact action names and bodies: `menambahkan tugas` with `Terima judul tugas` then `Simpan tugas baru`; `melihat semua tugas` with `Tampilkan semua tugas`; and `menandai tugas sebagai selesai` with `Terima tugas yang dipilih` then `Tandai tugas sebagai selesai`. Extra persistence collections and any unsupported action name/body are rejected before storage is opened. Other validly parsed declarations are not automatically executable by that runner.
- Diagnostic codes are stable within this starter version only; changing a code or its meaning requires updating tests and this contract together.

## 6. Stable diagnostic classes

| Code | Rejection class |
|---|---|
| H0001 | Tab used in structural indentation |
| H0002 | Noncanonical structural indentation or invalid text-block prefix |
| H0003 | Empty source or missing application declaration |
| H0004 | Invalid application name |
| H0005 | Indented application declaration |
| H0006 | Orphaned nested instruction |
| H0007 | Invalid data name |
| H0008 | Missing or malformed text-block header |
| H0010 | Invalid action name |
| H0011 | Action body indentation is not exactly one four-space level |
| H0012 | Invalid name in a display instruction |
| H0013 | Unsupported action-body instruction |
| H0014 | Empty action body |
| H0015 | Invalid persistence collection name |
| H0016 | Unknown top-level instruction |
| H0017 | Duplicate data/action declaration name |
| H0018 | Display reference does not resolve to a data declaration |
| H0019 | Duplicate persistence declaration |

H0009 is unused and has no meaning in this version. Codes and rejection classes are versioned behavior; changes require updated fixtures and an explicit compatibility decision. For semantic conflicts and unresolved references, the diagnostic line identifies the declaration whose semantics cannot be validated.

## 7. Required conformance matrix

| Area | Positive evidence | Negative evidence |
|---|---|---|
| Structure | application + supported declarations | empty source, indented application, orphan instruction |
| Indentation | four-space nested body | tab, non-multiple-of-four indentation |
| Text blocks | Unicode and multiline content, blank lines, CRLF, empty block, instruction-like text remains literal, extra spaces and tabs after the eight-space prefix remain data | missing text-block header |
| Names | Indonesian/Unicode alphanumeric names, underscore, hyphen | repeated/edge spaces, punctuation |
| Binding | display of declared data | unresolved data name, action name used as data, duplicate name shared by data and action |
| Persistence | one tugas declaration | duplicate persistence declarations |
| Compatibility | explicit vNext CLI path | default v0.1 regression suite must remain green |

Executable fixture coverage is stored under `tests/human-vnext/valid/` and `tests/human-vnext/invalid/`. The Rust unit tests parse each positive fixture and assert the expected diagnostic code and source line for every negative fixture; `tools/check_contract.py` also fails if the minimum fixture inventory is missing. The fixtures cover representative structure, Unicode names, task-list declarations, text-block boundaries and opaque payload whitespace, empty source, indented application declarations, orphan instructions, unknown instructions, missing text blocks, duplicate data/persistence declarations, cross-kind data/action name collisions, action-name-as-data binding, repeated spaces, unresolved display references, second application declarations, empty action bodies, tab indentation, noncanonical indentation widths, and invalid action identifiers, and combining marks outside the Unicode-alphanumeric identifier class. The fixture inventory gate currently requires at least six valid and seventeen invalid human-vNext source files. This remains representative rather than exhaustive coverage of every row above.

The completion ledger remains authoritative for milestone credit; adding fixtures alone does not earn verified points until the exact commit passes the required verification contract.
