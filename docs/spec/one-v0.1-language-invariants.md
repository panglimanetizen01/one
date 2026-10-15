# One v0.1 — Language Invariants

**Status:** Draft specification for review
**Scope:** Language-level invariants that constrain lexical, grammar, AST, semantic, IR, runtime, compiler, localization, and tooling design.
**Foundation:** `docs/foundation/one-v0.1-foundation.md`

## 1. Purpose

This document defines properties that must remain true throughout One v0.1 development. Later specifications may refine how an invariant is implemented, but may not silently weaken or contradict the invariant.

This document does **not** define the complete lexical grammar, concrete grammar, AST schema, type system, or IR schema. Those are subsequent specification gates.

## 2. Normative Language

The terms **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are normative.

A later specification is conformant only if it preserves every applicable MUST/MUST NOT rule in this document.

## 3. Core Invariants

### LI-001 — Formality

One source MUST be governed by a formally specified language definition. Natural-language-like surface syntax MUST NOT mean arbitrary natural-language interpretation.

**Acceptance criteria**

- Every accepted construct belongs to a defined grammar.
- Parser acceptance is reproducible from source and specification.
- No LLM is required to determine whether source is syntactically valid.

**Invalid design**

> The compiler asks an AI what the user probably meant when no grammar production matches.

### LI-002 — Deterministic Parsing

For a fixed One v0.1 specification and identical source bytes, parsing MUST produce the same result on every conforming implementation.

A parser MUST NOT select a production using probabilistic inference, model output, execution history, wall-clock state, network state, or unspecified heuristics.

**Acceptance criteria**

- Same source + same language version => same parse result or same parse diagnostic class.
- Parser behavior does not require network access or an AI service.

### LI-003 — Deterministic Meaning

A successfully parsed and semantically valid program MUST have one defined semantic interpretation under the applicable language version and platform-independent semantic rules.

Platform-specific behavior MAY exist only where the specification explicitly marks the operation as platform-dependent.

### LI-004 — Ambiguity Rejection

If source admits more than one valid interpretation under the applicable grammar or semantic rules, the implementation MUST reject the ambiguous construct with a diagnostic rather than silently choosing an interpretation.

**Example**

```text
nama adalah Budi
nama adalah Produk

tampilkan nama
```

If both bindings are simultaneously valid candidates for the reference `nama` under the future scope/name-resolution rules, the reference MUST be diagnosed as ambiguous rather than guessed.

**Acceptance criteria**

- Ambiguous source never silently selects one candidate.
- The diagnostic identifies the ambiguity when sufficient source information exists.

### LI-005 — No Silent Source Mutation

Compiler, interpreter, parser, checker, formatter, and build commands MUST NOT silently modify the user's source program as a side effect of parsing, checking, running, or building.

Any future formatter or explicit source-rewrite tool MUST be an explicit user action with a separately specified contract.

### LI-006 — Semantic Authority

The One language specification and conforming language implementation are the semantic authority.

AI/LLM systems MAY assist with explanation, documentation, learning, debugging, or migration, but MUST NOT be required to establish syntax validity, semantic meaning, type correctness, capability authorization, or runtime semantics.

### LI-007 — Stable Diagnostic Contract

Language errors MUST be deterministic and testable.

A diagnostic SHOULD identify:

1. stable error code;
2. source location when available;
3. what failed;
4. why it failed;
5. actionable correction guidance when available.

Diagnostic wording MAY evolve for clarity unless the relevant specification explicitly freezes wording. Diagnostic meaning and error-code semantics MUST NOT change silently.

### LI-008 — Source Location Integrity

Diagnostics and syntax structures that expose source locations MUST refer to positions in the original source, not to an internally rewritten or guessed source representation.

Source locations MUST be deterministic for identical source bytes.

### LI-009 — Structural Whitespace

Where newline and indentation participate in grammar, their meaning MUST be formally specified.

Whitespace MUST NOT acquire different structural meaning merely because an implementation happens to use a different parser or operating system.

The later lexical specification MUST define newline normalization, indentation measurement, blank lines, comments, continuation behavior, and invalid indentation cases.

### LI-010 — Versioned Language Meaning

Language meaning MUST be associated with an explicit language version or compatibility contract.

A future grammar change that alters the meaning or validity of existing source MUST be treated as a specification/versioning decision, not silently introduced as an implementation detail.

### LI-011 — Semantic Separation from Localization

Human-language surface syntax MUST be separated from the common semantic model.

A localized grammar MAY use different words, phrase structures, or lexical conventions, but equivalent constructs MUST map to the same defined semantic concepts unless the localization specification explicitly defines a language-specific semantic extension.

Localization MUST NOT require duplicating the semantic engine for every human language.

### LI-012 — Construct Closure

The six approved One v0.1 core constructs are the initial language construction set:

- DATA
- ACTION
- CONDITION
- LOOP
- FUNCTION
- EVENT

No additional core language construct MAY be introduced into the v0.1 semantic contract without an explicit specification and acceptance criteria.

Existing constructs MAY contain sub-forms required for their implementation, but those sub-forms MUST NOT silently become unrelated semantic constructs.

### LI-013 — Semantic Construct Identity

Surface wording MUST NOT be treated as semantic identity by itself.

For example, two localized spellings or phrases MAY represent the same semantic construct, while one phrase MUST NOT acquire multiple semantic meanings solely from natural-language interpretation.

### LI-014 — AST Boundary

The parser MUST produce a structured AST or an equivalent formally specified syntax representation before semantic analysis.

Semantic analysis MUST NOT depend on reparsing arbitrary natural-language text or on an LLM interpretation of source.

The exact AST schema is intentionally deferred to the AST Contract stage.

### LI-015 — Semantic Analysis Boundary

Semantic validity MUST be determined after syntactic structure is established and before execution or backend lowering relies on the program's meaning.

Name resolution, scope, type rules, reference validity, and capability checks MUST have deterministic specification-defined behavior.

Exact ordering between semantic subpasses is deferred until the semantic model is specified.

### LI-016 — IR Boundary

The common One IR MUST represent semantic meaning independently of the localized surface syntax.

IR generation MUST be deterministic for the same accepted program and applicable language/toolchain version.

The IR specification will define the exact canonical representation and serialization rules later.

### LI-017 — Runtime/Backend Independence

Runtime execution and backend compilation MUST consume a defined semantic representation. They MUST NOT infer source-language meaning independently from localized text.

A backend MAY reject an unsupported capability or target-specific feature, but MUST NOT silently reinterpret the program.

### LI-018 — Cross-Platform Semantic Consistency

For a program that uses only platform-independent One semantics, conforming Windows and Linux implementations MUST preserve the same language meaning and observable semantic result, subject only to explicitly specified environmental differences.

Platform-specific APIs and capabilities MUST be explicitly identified as such.

### LI-019 — Explicit Capability Boundary

Operations that can access filesystem, process, shell, network, native libraries, or other privileged system facilities MUST pass through an explicit capability/permission boundary defined by the language/runtime architecture.

A normal language construct MUST NOT silently acquire unrestricted operating-system authority.

### LI-020 — Interoperability Is Boundary-Based

Interoperability with other ecosystems MUST occur through explicit, specified boundaries.

Using Python, JavaScript, C/C++, Java, shell, databases, or native APIs MUST NOT silently become a semantic dependency of the One language core.

### LI-021 — No Mandatory Foreign Language for One Projects

The One language/toolchain MUST preserve the architectural possibility of completing a supported project entirely in One.

Foreign-language integration MAY be used where useful or necessary for interoperability, but it MUST NOT be an architectural prerequisite for all One programs.

### LI-022 — Deterministic Build Inputs

Parsing, checking, IR generation, and v0.1 build behavior MUST be reproducible from declared inputs.

Undeclared network calls, AI responses, local machine state, random choices, or wall-clock time MUST NOT alter language meaning.

If build metadata intentionally contains nondeterministic information, that metadata MUST be explicitly separated from semantic artifacts.

### LI-023 — No Hidden Recovery

When source violates a required lexical, grammatical, semantic, or capability rule, the implementation MUST reject it or report a formally specified recovery result.

Error recovery used internally for diagnostics MUST NOT cause invalid source to execute or produce a falsely successful semantic result.

### LI-024 — Compatibility Must Be Explicit

Backward compatibility MUST be based on documented language/toolchain rules.

An implementation MUST NOT preserve obsolete behavior through undocumented parser heuristics that contradict the current specification.

### LI-025 — Testability

Every normative language rule introduced by the v0.1 specification MUST be testable.

The conformance suite MUST eventually contain, as applicable:

- valid fixtures proving acceptance;
- invalid fixtures proving rejection;
- ambiguity fixtures proving rejection of ambiguous source;
- diagnostic assertions proving stable error classification;
- golden fixtures proving deterministic AST/IR or other canonical artifacts.

### LI-026 — Specification Precedence

When implementation behavior conflicts with an approved specification, implementation behavior is not the authority.

The conflict MUST be recorded and resolved by specification review before implementation is changed or the contract is intentionally revised.

### LI-027 — No Implicit Contract Expansion

A passing implementation test MUST NOT by itself promote an implementation behavior into the language contract.

A behavior becomes normative only when the relevant specification explicitly defines it and its acceptance criteria are approved.

### LI-028 — Minimal-Punctuation Principle

One MAY use punctuation where it is technically necessary, but punctuation MUST NOT become the primary mechanism that beginners must learn to express ordinary v0.1 constructs.

The lexical and grammar specifications MUST preserve this principle while remaining formally deterministic.

### LI-029 — Progressive Complexity Without Semantic Forking

Beginner-oriented syntax and advanced language facilities MUST share the same semantic foundation.

Adding advanced features MUST NOT create a second incompatible language semantics for experienced developers.

### LI-030 — Security Claims Require Evidence

Security properties MUST be stated only to the extent supported by tests, architecture, and documented platform guarantees.

The project MUST NOT claim sandboxing, isolation, hardware enforcement, or other security properties that have not been demonstrated or formally justified for the relevant target.

## 4. Required Edge-Case Classes

The following edge-case classes are mandatory inputs to later lexical, grammar, and conformance design:

1. empty source;
2. source containing only blank lines;
3. source containing only comments;
4. trailing newline vs no trailing newline;
5. CRLF vs LF line endings;
6. blank lines inside and around blocks;
7. indentation increase by one level;
8. indentation increase by more than one level;
9. indentation decrease to a previously valid level;
10. indentation decrease to an unseen level;
11. tabs mixed with spaces;
12. Unicode letters in identifiers;
13. underscores in identifiers;
14. identifier/keyword collision;
15. numeric boundaries and malformed numbers;
16. empty and escaped text literals;
17. unterminated text literals;
18. comment markers inside text literals;
19. comment after source code;
20. unsupported punctuation;
21. ambiguous names;
22. ambiguous phrase boundaries;
23. invalid construct ordering;
24. invalid nesting;
25. invalid or unavailable capability use;
26. localized equivalent constructs mapping to common semantics.

These are test obligations, not yet complete lexical or grammar rules.

## 5. Acceptance Gate for This Specification

Language Invariants v0.1 is complete only when all of the following are true:

- Every foundation-level language principle has a corresponding invariant or an explicit reason why it is outside this document's scope.
- No invariant contradicts the locked One v0.1 foundation.
- No invariant silently locks details intentionally deferred to lexical, grammar, AST, type, semantic, or IR specifications.
- Every invariant has a deterministic acceptance interpretation.
- Ambiguity, localization, platform, AI boundary, security, and interoperability are explicitly covered.
- Required edge-case classes are recorded for downstream conformance work.
- The specification is reviewed against the existing implementation without treating existing implementation behavior as authoritative.

## 6. Known Implementation Alignment Note

The repository already contains lexer, parser, AST, semantic, IR, interpreter, diagnostics, capability, and CLI implementation artifacts. Those artifacts are evidence about the current implementation, not automatic authority over this specification.

In particular, existing lexer behavior includes explicit indentation and newline handling. The later Lexical Specification MUST audit that implementation against the normative rules established here before lexical behavior is declared contract-complete.

## 7. Next Specification Gate

After approval of this document, the next stage is:

**Lexical Specification One v0.1**

It MUST define, at minimum:

- source encoding;
- Unicode policy;
- identifier grammar;
- keyword/reserved-word policy;
- number literals;
- text literals and escapes;
- whitespace;
- newline normalization;
- indentation/dedentation;
- comments;
- token boundaries;
- lexical errors;
- lexical ambiguity rules;
- valid/invalid examples;
- edge-case tests;
- acceptance criteria.

Only after that gate should the Concrete Grammar v0.1 be finalized.
