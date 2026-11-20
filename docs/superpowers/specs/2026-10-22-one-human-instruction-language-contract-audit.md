# ONE Human Instruction Language Contract — Forensic Audit and Next Gate

**Status:** PROPOSED DESIGN / AUDIT ARTIFACT — NOT A NORMATIVE v0.1 GRAMMAR CHANGE  
**Purpose:** Record the gap between ONE's accepted human-oriented philosophy and the currently implemented source language, then define the evidence required before changing the language surface.

## 1. Binding product intent

ONE is a programming language that users learn. It is not a prompt wrapper, a free-form natural-language interface, or a system whose semantics are supplied by an LLM.

The user-facing source language should let a person express program behavior as direct human instructions, in the manner of a superior giving clear orders to a subordinate. Users learn ONE's instruction vocabulary, composition rules, control structures, data model, and semantics.

The distinguishing design constraint is that source authors must not be required to use conventional programming punctuation such as braces, parentheses, semicolons, or symbolic operators to express program structure and behavior.

Illustrative target direction:

```text
Buat aplikasi daftar tugas

Pengguna bisa menambahkan tugas

Pengguna bisa melihat semua tugas

Pengguna bisa menandai tugas sebagai selesai

Simpan semua tugas agar tidak hilang saat aplikasi ditutup
```

This is a target-design example only. It is not a claim that the current v0.1 parser accepts or executes this whole application description.

## 2. What the current repository establishes

The accepted v0.1 foundation and implementation plan already require a human-oriented surface, deterministic formal grammar, minimal required punctuation, explicit ambiguity rejection, and independence from LLMs for parsing and execution.

The current implementation is nevertheless a bounded language subset. The lexer recognizes words, integers, quoted text, newlines, indentation, and comments. It rejects unknown characters. The parser recognizes specific productions such as declarations and assignments, display actions, list operations, conditions, loops, functions, events, and selected capability operations. Unsupported sentence variants are rejected rather than inferred.

The current beginner examples use quoted text, for example `nama adalah "Budi"`. The current lexical contract uses `#` for comments and quoted strings for text literals. These are concrete facts about the current v0.1 surface, not a reason to make conventional punctuation the long-term identity of ONE.

Relevant implementation and contract references:

- `docs/foundation/one-v0.1-foundation.md`
- `docs/spec/one-v0.1-lexical.md`
- `docs/spec/one-v0.1-grammar.md`
- `docs/spec/one-v0.1-ast.md`
- `docs/spec/one-v0.1-language-invariants.md`
- `crates/one-core/src/lexer.rs`
- `crates/one-core/src/parser.rs`
- `crates/one-core/src/ast.rs`
- `crates/one-core/src/semantic.rs`
- `crates/one-core/src/ir.rs`
- `crates/one-core/src/interpreter.rs`
- `docs/panduan-pengguna/program-pertama.md`
- `docs/verification/one-v0.1-implementation-status.md`

## 3. Forensic findings

### F1 — Philosophy and implementation are not the same completion state

The foundation's design direction is compatible with human instruction syntax. However, the current implementation does not establish that an ordinary user can express an entire application using the target instruction style. Existing evidence supports only the productions and capabilities explicitly covered by grammar, tests, and exact-source verification.

### F2 — “Minimal punctuation” is weaker than the intended design constraint

The existing contract says required punctuation is minimized. The product intent recorded here is stronger: conventional programming punctuation must not be required as the structural mechanism of the user-facing language.

This difference must be resolved through an explicit approved design decision. Do not silently reinterpret the v0.1 normative grammar or claim it already meets the stronger requirement.

### F3 — Literal text and programming syntax must be separated conceptually

Removing mandatory punctuation from program structure does not mean ordinary punctuation can never appear in data. A text value may need to contain a question mark, exclamation mark, comma, colon, or other character. The language needs a consistent, teachable way to express text data without making conventional punctuation the control syntax. Candidate literal forms require design and tests before selection.

### F4 — Instruction-like source still needs a real grammar

ONE must not execute arbitrary Indonesian prose by guessing intent. Every accepted instruction must map deterministically to a defined construct, AST, semantic representation, and runtime behavior. Unknown, incomplete, conflicting, or ambiguous instructions must fail with actionable diagnostics.

### F5 — AI may assist authoring but cannot define language meaning

An optional AI assistant may suggest or translate instructions into valid ONE source. The canonical lexer, parser, semantic analyzer, conformance suite, and runtime must work without AI and must remain the authority on what source means. AI output is ordinary source and must pass the same deterministic validation.

### F6 — The application-level instruction example is an unimplemented acceptance target until proven

The example in Section 1 describes multiple requirements and persistent state. Before claiming support, the repository needs an explicit grammar/semantic model for application declarations, user actions, persistence, and how requirements compose into an executable project, along with end-to-end tests.

## 4. Required architecture

The target path remains:

```text
Human instruction source
    → deterministic lexical and grammatical analysis
    → language-independent AST
    → semantic validation and name resolution
    → semantic IR
    → governed runtime and capability adapters
    → observable program behavior
```

The compiler pipeline may be technically sophisticated. That complexity is not exposed as punctuation-heavy source syntax to the learner.

The grammar should distinguish at least:

- declarations of data and application concepts;
- direct actions and their explicit targets;
- conditions and alternatives;
- repetition;
- reusable procedures and parameters;
- events and user interactions;
- input, output, and persistent state;
- errors and recovery;
- capability/effect declarations and security policy.

These are design categories, not a claim that every category is fully implemented today.

## 5. Acceptance gates before a language-surface redesign is considered complete

1. **Design contract:** define the intended source style and explicitly list structural punctuation that users must not need to write.
2. **Instruction grammar:** specify bounded, composable productions in the initial language locale, with examples for valid, invalid, incomplete, and ambiguous source.
3. **Data representation:** define how text, numbers, booleans, names, and collections are written without relying on conventional syntax punctuation as structural delimiters.
4. **Deterministic parsing:** each accepted source maps to one defined structure; ambiguity is rejected.
5. **Semantic contract:** every instruction has a documented meaning, type/effect rules, and runtime behavior.
6. **Conformance fixtures:** each production has positive and negative fixtures, expected AST/IR, diagnostics, and runtime behavior where applicable.
7. **Application vertical slice:** prove a small but meaningful app can be expressed, validated, run, and persist data using the intended user-facing syntax.
8. **AI-independence test:** the canonical pipeline must pass with no model, network service, or probabilistic intent inference.
9. **Documentation alignment:** beginner documentation teaches the target language without presenting temporary v0.1 syntax as the final design.
10. **Versioning and migration:** decide whether the stronger language surface is a new version or a compatible extension. Do not silently break the locked v0.1 contract.
11. **Evidence:** report exact commit SHA, test commands, results, platform boundaries, and any unverified claims.

## 6. Non-goals and anti-patterns

- Do not turn ONE into “write any prompt and hope the AI understands.”
- Do not add fuzzy matching or model-based guessing to the canonical parser.
- Do not claim that the application example works before end-to-end tests prove it.
- Do not equate a natural-looking keyword with a fully human-oriented language.
- Do not change lexer, grammar, AST, and runtime independently without a coordinated contract and conformance update.
- Do not remove every punctuation character from literal data merely to satisfy a slogan; distinguish data content from syntax.
- Do not change the normative v0.1 contract without explicit versioning and compatibility decisions.

## 7. Recommended next implementation milestone

Create a dedicated **ONE Human Instruction Surface vNext** specification and a small vertical slice before broad grammar expansion. The slice should model a simple task-list application from top-level instructions through a deterministic AST/IR and executable behavior, including persistence only if its storage semantics and security boundary are specified.

The vertical slice should be developed on a separate branch, with v0.1 behavior preserved until a compatibility plan is approved. Its acceptance tests should focus on the user's learning experience as well as compiler correctness: no required conventional structural punctuation, clear instruction patterns, useful errors, and no hidden AI dependency.

## 8. Decision summary

ONE remains a programming language that users learn. Its distinctive source model is structured human instruction, not conventional symbol-heavy syntax and not arbitrary prompting. Formal grammar and deterministic execution remain mandatory. The current repository is a foundation for that direction, but the stronger no-conventional-structural-punctuation experience and the complete application-level instruction example must be treated as explicit design and implementation work until evidence proves them.
