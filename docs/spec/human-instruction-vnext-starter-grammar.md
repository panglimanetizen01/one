# ONE Human-Instruction vNext Starter Grammar

**Status:** Proposed, implementation-bounded starter contract; not normative for ONE generally.  
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

## 2. Lexical conventions

- NL is LF or CRLF.
- SP is one ASCII space.
- INDENT is exactly four ASCII spaces for each nesting level.
- NAME is a non-empty sequence of Unicode alphanumeric characters, underscore, or hyphen, optionally separated by one ASCII space; it may not begin/end with a space.
- Blank lines are ignored for declaration parsing. Inside a text block, blank lines and relative indentation are preserved as text.
- Keywords are literal, case-sensitive Indonesian strings shown below. No synonym expansion is implied.

## 3. Starter grammar (EBNF)

```ebnf
program              = blank*, application, (blank* declaration)*, blank* ;
application          = "Buat aplikasi ", NAME, NL ;
declaration          = data_declaration
                     | action_declaration
                     | persistence_declaration ;

data_declaration     = "Buat data ", NAME, NL, text_block ;
text_block           = INDENT, "Isi teks berikut", NL, text_line,
                       (NL, text_line)* ;
text_line            = INDENT, TEXT ;
action_declaration   = "Pengguna bisa ", NAME, NL, instruction_block ;
instruction_block    = instruction, (NL, INDENT, instruction)* ;
instruction          = display_instruction
                     | accept_title
                     | add_task
                     | display_all_tasks
                     | accept_task_id
                     | complete_task ;
display_instruction  = INDENT, "Tampilkan ", NAME ;
accept_title         = INDENT, "Terima judul tugas" ;
add_task             = INDENT, "Simpan tugas baru" ;
display_all_tasks    = INDENT, "Tampilkan semua tugas" ;
accept_task_id       = INDENT, "Terima tugas yang dipilih" ;
complete_task        = INDENT, "Tandai tugas sebagai selesai" ;
persistence_declaration
                     = "Simpan semua ", NAME,
                       " agar tidak hilang saat aplikasi ditutup", NL ;
blank                = NL ;
```

TEXT is the remaining source content on a line after the text-block indentation. Text-block collection continues until the next non-blank line at indentation zero or until end-of-file. The implementation may preserve empty lines and indentation relative to the block's content indentation. An empty text block is currently accepted and produces an empty text value; the required conformance matrix must preserve this behavior unless a versioned contract change deliberately changes it.

**Important implementation boundary:** the EBNF above names the currently supported forms, but text-block boundaries and indentation are governed by the prose rules and parser tests; this starter contract does not yet define a general-purpose expression language.

## 4. Name binding and rejection rules

- The application declaration must be first (ignoring blank lines) and must not be indented.
- Duplicate data/action names are rejected by the parser's current declaration namespace.
- Duplicate persistence declarations are rejected.
- A display instruction must resolve to a declared data name; an action name is not a data value.
- Unknown action-body instructions, missing text blocks, empty/invalid names, tabs, invalid indentation, and orphaned nested lines are rejected.
- The current prototype's persistence runner requires the collection name tugas and the supported add/list/complete instruction shapes. Other validly parsed declarations are not automatically executable by that runner.
- Diagnostic codes are stable within this starter version only; changing a code or its meaning requires updating tests and this contract together.

## 5. Versioning and compatibility

- This contract identifier is human-vnext-starter/0.1; it is separate from ONE's normative v0.1 language.
- New keywords, synonyms, indentation rules, identifier characters, name binding, text-block behavior, or diagnostic semantics require a versioned contract change and positive/negative regression fixtures.
- A syntax change must not alter the default v0.1 parser. Any future shared AST/IR lowering is a separate milestone and must be specified and tested independently.
- The starter grammar may be narrowed to match executable evidence. It must not be broadened merely by adding prose examples.

## 6. Required conformance matrix

| Area | Positive evidence | Negative evidence |
|---|---|---|
| Structure | application + supported declarations | empty source, indented application, orphan instruction |
| Indentation | four-space nested body | tab, non-multiple-of-four indentation |
| Text blocks | Unicode and multiline content, blank lines, CRLF, empty block | missing text-block header |
| Names | Indonesian/Unicode alphanumeric names, underscore, hyphen | repeated/edge spaces, punctuation |
| Binding | display of declared data | unresolved data name, action name used as data |
| Persistence | one tugas declaration | duplicate persistence declarations |
| Compatibility | explicit vNext CLI path | default v0.1 regression suite must remain green |

Executable fixture coverage is stored under `tests/human-vnext/valid/` and `tests/human-vnext/invalid/`. The Rust unit tests parse each positive fixture and require every negative fixture to be rejected; `tools/check_contract.py` also fails if the minimum fixture inventory is missing. The fixtures cover representative structure, Unicode names, task-list declarations, text-block boundaries, empty source, indented application declarations, orphan instructions, unknown instructions, missing text blocks, duplicate data/persistence declarations, action-name-as-data binding, repeated spaces, and unresolved display references. The fixture inventory gate currently requires at least four valid and ten invalid human-vNext source files. This remains representative rather than exhaustive coverage of every row above.

The completion ledger remains authoritative for milestone credit; adding fixtures alone does not earn verified points until the exact commit passes the required verification contract.
