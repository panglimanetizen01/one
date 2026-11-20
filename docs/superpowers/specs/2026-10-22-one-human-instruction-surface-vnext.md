# ONE Human Instruction Surface vNext

**Status:** PROPOSED DESIGN — NOT NORMATIVE AND NOT IMPLEMENTED  
**Scope:** A candidate source-language contract for the future human-instruction surface.  
**Compatibility:** Does not modify the locked ONE v0.1 grammar or claim existing source examples implement this design.

## 1. Product contract

ONE is a programming language people learn. It is not an arbitrary prompt, a natural-language guesser, or an AI wrapper.

The source should read like clear instructions from a person responsible for a task to a capable subordinate. Users learn a finite vocabulary and composition model. The compiler gives each accepted instruction one defined meaning.

The defining surface constraint is:

**Program structure and behavior must not require conventional programming punctuation.**

This prohibits relying on braces, parentheses, semicolons, commas, colons, symbolic operators, or quote marks as mandatory structural syntax in the intended surface. It does not prohibit those characters from appearing inside data where they are actual data. A future version may optionally offer alternate literal spellings, but the ordinary beginner path must not require punctuation to define program structure.

## 2. Example target

The following is a design target, not valid or executable v0.1 source today.

    Buat aplikasi daftar tugas

    Pengguna bisa menambahkan tugas

    Pengguna bisa melihat semua tugas

    Pengguna bisa menandai tugas sebagai selesai

    Simpan semua tugas agar tidak hilang saat aplikasi ditutup

A future implementation must define what each sentence means, how these requirements compose, what generated/runtime artifact is produced, and how persistence is verified. Until an end-to-end fixture passes, documentation must label this as illustrative only.

## 3. Language model

The source pipeline remains deterministic:

    UTF-8 source
      → line and indentation analysis
      → deterministic instruction grammar
      → language-independent AST
      → name and type resolution
      → semantic IR
      → capability and policy checks
      → runtime or backend
      → observable result

No AI service participates in canonical parsing, validation, compilation, execution, or test expectations. Optional authoring assistance may propose source, but the same compiler validates it and source meaning never depends on the model response.

The source grammar must be bounded. “Sounds reasonable in Indonesian” is not a production rule.

## 4. Surface principles

### 4.1 Instructions, not punctuation

A construct should be recognized through defined words, phrase order, line boundaries, and indentation. Newlines separate top-level instructions. Indentation may express a nested instruction body when the grammar explicitly expects one.

Do not replace punctuation with invisible heuristics. If a line has two plausible parses with different meanings, reject it and explain the ambiguity.

### 4.2 Sentence forms are language syntax

Instruction patterns are formal productions, not free-form prose. For example, the design may define a project declaration, capability declaration, user action, state declaration, output instruction, condition, repetition, and persistence instruction. Each pattern needs a canonical form and a testable meaning before implementation.

A phrase can be reserved in a grammar position without banning the same word from every user-defined name. Name boundaries must be specified per production and ambiguity must be rejected.

### 4.3 Data is not control syntax

A value must not accidentally become a command merely because its words resemble an instruction. The design must distinguish a value position from an instruction position.

The literal system must eventually support text containing spaces, newlines, Unicode, quotes, punctuation, and strings that resemble ONE commands. It must define empty text and escaping or an equivalent lossless representation. Candidate approaches include indented text blocks, explicit text-introduction phrases, and unambiguous block boundaries. This proposal intentionally does not select a final literal syntax before prototype fixtures establish that it is teachable and unambiguous.

The acceptance rule is stronger than “most strings can be written without quotes”: users must be able to express arbitrary text data losslessly without making conventional punctuation mandatory for ordinary program structure.

### 4.4 No semantic guessing

The compiler must reject unknown instruction forms, missing required parts, invalid nesting, unresolved names, conflicting declarations, and ambiguous interpretations. Diagnostics should state what ONE expected and identify the source location. The compiler must not silently rewrite source.

### 4.5 Localized vocabulary, stable semantics

Indonesian is the first surface locale. Locale-specific instruction phrases lower to the same language-independent semantic representation. Translation cannot change security policy, effects, types, or runtime behavior.

## 5. Proposed instruction families

These are semantic categories, not a finalized keyword list.

| Family | Purpose | Minimum contract |
| --- | --- | --- |
| Project | Name an application or program | Declared project identity and artifact kind |
| State and data | Create and update named values | Scope, type, initialization, and mutation rules |
| User interaction | Describe available user actions | Trigger, parameters, effects, and result |
| Observation | Display or return information | Value resolution and output channel |
| Decision | Choose behavior based on a condition | Boolean condition and branch boundaries |
| Repetition | Repeat a defined action | Iteration source or termination condition |
| Persistence | Retain state between runs | Storage target, lifecycle, failure behavior |
| Capability | Request filesystem, network, process, or other effects | Explicit permission, scope, denial, and audit behavior |
| Composition | Connect requirements into a complete program | Deterministic order, dependencies, and conflict rules |

No family is considered implemented solely because documentation names it. Every supported family needs parser, semantic, runtime, and conformance evidence appropriate to its claims.

## 6. Grammar contract to write before implementation

The normative grammar document must define:

1. Source encoding, line endings, whitespace, indentation, blank lines, and block boundaries.
2. Instruction starts, complete productions, legal nesting, and statement completion.
3. Reserved phrases, user-defined names, name boundaries, and duplicate-name behavior.
4. Data literal representation for text, numbers, booleans, empty values, and collections.
5. Expression and condition forms, precedence, and type compatibility where applicable.
6. How user actions and project-level requirements compose into executable behavior.
7. Persistence semantics, including read/write lifecycle, errors, and platform boundaries.
8. Exact diagnostics for incomplete, unknown, unsupported, and ambiguous instructions.
9. Canonical AST and semantic IR mappings independent of Indonesian wording.
10. Version identifier and a migration/compatibility policy with v0.1.

The grammar should be reviewable independently of implementation. Avoid encoding the entire language as a list of ad hoc string comparisons in the parser.

## 7. Compatibility and versioning

The current v0.1 grammar is normative for its version and currently requires quoted text literals and recognizes hash comments. This proposal does not retroactively redefine those files.

Before adopting the new surface, choose one explicit path:

- a separately versioned surface grammar that lowers into the existing AST/IR where semantics genuinely match;
- a versioned language mode selected by project metadata or an explicit CLI option; or
- a future major language version with a documented migration path.

Do not silently reinterpret valid v0.1 programs. Do not promise that all existing programs can be mechanically migrated until a migration tool and fixture suite prove it.

## 8. Acceptance gates

A future implementation is not complete until all applicable gates pass.

### Design gates

- The product contract explicitly rejects mandatory conventional structural punctuation.
- A grammar defines every accepted instruction; no arbitrary-prose fallback exists.
- The literal/data representation is lossless and does not confuse data with instructions.
- Versioning and v0.1 compatibility are decided.

### Compiler gates

- Positive fixtures cover every supported production and valid nesting.
- Negative fixtures cover missing parts, unsupported wording, malformed indentation, ambiguous names, and conflicting interpretations.
- AST and IR golden fixtures prove deterministic lowering.
- Repeated compilation of identical source produces equivalent results.
- Parsing and semantic validation work offline with no LLM credentials or network access.

### End-to-end application gate

The task-list example must compile and run through the actual public toolchain. Tests must prove that users can add tasks, list tasks, mark tasks complete, and retain state after a process restart. If any operation is only a mock or design stub, the status must say so.

### Governance gates

- Filesystem and network effects require explicit capability and scope checks.
- Denied or out-of-scope operations fail closed.
- Natural-language phrasing never grants authority.
- Errors are visible and do not silently discard state or results.

### Documentation gates

- The beginner guide uses canonical supported syntax only.
- Examples are tested against the exact commit they document.
- Every feature is labeled accurately as specified, designed, implemented, tested, or verified.
- An exact-SHA evidence record names the source revision and environment for each verification claim.

## 9. Required fixture suite

At minimum, add fixture groups for:

- a minimal program with multiple instructions and blank lines;
- a project with multiple user actions;
- state creation, reading, update, and invalid references;
- text containing spaces, Unicode, punctuation, quotes, and command-like phrases;
- nested decisions and repetition;
- persistent data across two separate process runs;
- ambiguous and incomplete instructions that must be rejected;
- capability granted, denied, and target-out-of-scope cases;
- equivalent Indonesian and future localized forms lowering to equivalent IR;
- a full task-list vertical slice from source through observable runtime behavior.

Every positive fixture should record expected parse/AST/IR or runtime output as appropriate. Every negative fixture should record a stable diagnostic category and source span.

## 10. Implementation sequence

1. Approve the product and punctuation contract.
2. Prototype competing text-literal representations with a focused fixture suite.
3. Write and review the complete bounded grammar and versioning decision.
4. Implement a small deterministic parser slice without changing v0.1.
5. Prove AST and IR lowering with golden tests.
6. Implement the task-list vertical slice and persistent storage boundary.
7. Verify capability behavior and error cases.
8. Run the exact-SHA verification matrix on supported environments.
9. Publish tested beginner documentation only after the examples pass.
10. Decide whether to promote the proposal to a normative versioned spec.

## 11. Explicit non-goals

- Accepting any sentence and asking an LLM what it probably means.
- Making AI mandatory to compile or run a ONE program.
- Claiming a full application exists because a natural-language description is present.
- Requiring conventional punctuation as the primary mechanism for program structure.
- Silently changing v0.1 semantics or examples.
- Treating source files, passing a single unit test, or a successful build as proof that every listed capability works.

## 12. Current status

This document is a proposed design artifact. It has not been implemented or accepted as normative. The current v0.1 lexer/parser and the beginner example remain as documented by the v0.1 contracts. The next evidence-bearing milestone is a reviewed grammar plus a small executable vertical slice, not a claim that the complete human-instruction language is finished.
