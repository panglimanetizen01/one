# One v0.1 — Prior-Art Research: Natural and Human-Like Programming

**Status:** Research artifact for architecture and grammar design
**Date:** 2026-10-15
**Scope:** Prior art relevant to One's goal of a natural-looking, formally deterministic programming language.

## 1. Research conclusion

One is **not** the first project to explore programming or formal specification through natural-language-like syntax. Prior art is substantial and must be acknowledged.

The important architectural distinction for One is therefore not the generic claim "programming in natural language is new." It is the combination of:

1. a beginner-oriented Indonesian-first surface language;
2. conversational-looking but formally bounded syntax;
3. deterministic parsing and semantic analysis without an LLM in the compiler path;
4. explicit ambiguity rejection rather than probabilistic intent guessing;
5. a language-independent semantic model and IR so localized grammars can share meaning;
6. a path from beginner syntax to general-purpose, large-program development.

These characteristics may still contain novel engineering/design choices, but **no originality claim is made until a substantially broader literature, language, patent, and implementation search is completed.**

## 2. Controlled Natural Languages (CNL)

The strongest conceptual prior art is the field of Controlled Natural Languages. Kuhn's survey describes a large family of engineered languages positioned between ordinary natural language and formal languages. It explicitly identifies precision, expressiveness, naturalness, and semantic/formal properties as dimensions on which such languages differ.

Reference:
- Kuhn, Tobias. *A Survey and Classification of Controlled Natural Languages*. Computational Linguistics 40(1), 2014.
- https://direct.mit.edu/coli/article/40/1/121/1455/A-Survey-and-Classification-of-Controlled-Natural

**Lesson for One:** "natural-looking" and "formal" are not opposites. The surface can be familiar while the accepted subset remains strictly constrained.

## 3. Attempto Controlled English (ACE)

Attempto Controlled English is particularly relevant. ACE deliberately restricts English grammar and vocabulary so that sentences have formal, machine-processable interpretations. Its parser translates the controlled surface into a formal representation, and its documentation describes explicit construction and interpretation rules.

References:
- Attempto project: https://attempto.ifi.uzh.ch/site/main.php?text=resources%2Fresources.html
- ACE overview: https://attempto.ifi.uzh.ch/site/docs/ace_nutshell.html
- ACE syntax report: https://attempto.ifi.uzh.ch/site/pubs/papers/hoefler2004theSyntax.pdf

ACE demonstrates two techniques directly relevant to One:

- exclude ambiguous constructions when possible;
- when a construct is retained, define a deterministic interpretation rule or require rephrasing.

**Lesson for One:** ambiguity management must be part of the language contract, not an afterthought in the parser.

## 4. AppleScript

AppleScript demonstrates a different but important approach: English-like terminology is used for scripting commands and handlers, while the language remains a formal programming language with lexical conventions, variables, handlers, results, and errors.

References:
- AppleScript Language Guide: https://developer.apple.com/library/archive/documentation/AppleScript/Conceptual/AppleScriptLangGuide/
- AppleScript lexical conventions: https://developer.apple.com/library/archive/documentation/AppleScript/Conceptual/AppleScriptLangGuide/conceptual/ASLR_lexical_conventions.html
- AppleScript handlers: https://developer.apple.com/library/archive/documentation/AppleScript/Conceptual/AppleScriptLangGuide/conceptual/ASLR_about_handlers.html

**Lesson for One:** readable command phrasing can coexist with formal functions/handlers and a compiler/runtime model. However, English-like wording alone does not eliminate the need to learn a language grammar.

## 5. Naturalistic programming research

Research on naturalistic programming has repeatedly investigated the gap between programming notation and natural language. A 2019 study presents a naturalistic programming prototype whose grammar resembles natural language and explicitly frames ambiguity resolution as a central challenge.

Reference:
- *A Model for Naturalistic Programming with Implementation*, Applied Sciences, 2019.
- https://www.mdpi.com/2076-3417/9/18/3936

Research on novice programming also reports a practical boundary: unconstrained full-sentence natural language can introduce confusion and multiple semantically equivalent phrasings. Natural language can nevertheless be valuable for comprehension, collaboration, and debugging.

Reference:
- *Programming language, natural language? Supporting the diverse computational activities of novice programmers*, 2017.
- https://www.sciencedirect.com/science/article/abs/pii/S1045926X16301963

**Lesson for One:** One should not attempt to accept unrestricted Indonesian. The useful target is a controlled programming language that feels natural within a defined grammar.

## 6. Indonesian-language prior art

There is direct prior art for Indonesian-oriented programming syntax.

### BAIK

Haris Hasanudin's paper describes BAIK (Bahasa Anak Indonesia untuk Komputer), an Indonesian lexical/parsing-oriented scripting language intended for multitier web development. The paper explicitly argues for local-language programming and proposes an Indonesian parsing engine.

Reference:
- Hasanudin, Haris. *BAIK – Programming Language Based on Indonesian Lexical Parsing for Multitier Web Development*.
- https://jiki.cs.ui.ac.id/index.php/jiki/article/view/147

### Indonesian natural-language-to-code translation

A UNIKOM research project describes rule-based translation from Indonesian natural-language commands to Pascal source code using preprocessing, scanning, parsing, and code generation. The work reports strong results on a bounded command set but also notes limitations as complexity grows.

References:
- https://elib.unikom.ac.id/id/eprint/991/
- https://elib.unikom.ac.id/id/eprint/5671/

There is also research translating Indonesian natural-language commands into SQL/DML operations using NLP.

Reference:
- https://jurnal.fikom.umi.ac.id/ojs2/index.php/BUSITI/article/view/790

**Lesson for One:** Indonesian programming and Indonesian natural-language programming are not unexplored territory. One must therefore differentiate itself by architecture, formal semantics, general-purpose scope, deterministic compiler behavior, localization architecture, and complete toolchain goals rather than by language choice alone.

## 7. Natural-language programming with AI is a different category

Recent work such as NoviCode studies generating executable programs from everyday-language descriptions by novice users. This is important prior art, but it represents **natural-language-to-program synthesis**, not necessarily a formally specified programming language whose source is itself the canonical program representation.

Reference:
- *NoviCode: Generating Programs from Natural Language Utterances by Novices*, Transactions of the Association for Computational Linguistics, 2024.
- https://direct.mit.edu/tacl/article/doi/10.1162/tacl_a_00694/125031/NoviCode-Generating-Programs-from-Natural-Language

**Lesson for One:** AI-assisted code generation must not be confused with the One language itself. One's compiler path remains formal and executable without an LLM.

## 8. Design consequences for One

The research supports the following architectural decisions:

### PA-001 — Controlled naturalness

One MUST define a bounded programming grammar. Ordinary Indonesian outside that grammar is not automatically valid One source.

### PA-002 — Formal semantics beneath natural syntax

Every accepted surface construction MUST map to a defined semantic node. Surface resemblance to Indonesian does not grant semantic freedom.

### PA-003 — Explicit ambiguity policy

Ambiguity MUST be detected by deterministic language rules. The compiler MUST NOT guess using context, probability, or an LLM.

### PA-004 — Rephrasing is a valid resolution strategy

When a phrase is structurally or semantically ambiguous, One MAY reject it and provide deterministic candidate information. It MUST NOT silently select a candidate.

### PA-005 — Localization is grammar, not string replacement

Each supported human language should have a localized lexical/concrete grammar that maps into the common semantic model. Translation of keywords alone is insufficient.

### PA-006 — Semantic model is the portability boundary

The common semantic model/IR should be language-neutral. Indonesian, English, Japanese, Spanish, Arabic, and other surface grammars should converge before runtime/compiler backends.

### PA-007 — No originality claim yet

One should not claim that "natural programming" or "programming in Indonesian" is itself novel. The project can later document its specific novel contributions only after a broader prior-art review.

## 9. Evidence limitations

This is an architectural prior-art scan, not a legal patent search, exhaustive academic systematic review, or freedom-to-operate opinion. Search coverage is necessarily incomplete. Before any public originality or patent-related claim, the project should perform a dedicated search across:

- programming-language literature;
- controlled-natural-language literature;
- natural-language programming systems;
- Indonesian programming languages;
- patents and patent applications;
- open-source implementations;
- multilingual/localized programming-language systems.

## 10. Design decision

**Decision:** One will pursue **formal controlled natural programming**, not unrestricted natural-language programming and not AI-to-code as its language definition.

The defining principle is:

> Natural on the surface; formal underneath.

This principle is consistent with substantial prior art, but One's specific combination of Indonesian-first beginner syntax, deterministic ambiguity rejection, shared semantic IR, localization architecture, and general-purpose project goals remains a design hypothesis rather than a proven originality claim.
