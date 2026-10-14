# One v0.1 — Official Foundation

**Status:** Approved / Locked foundation
**Date:** 2026-10-15

## Identity

The programming language is named **One**.

The name is approved and locked. Compiler, runtime, CLI, package-manager, file-extension, and other component names remain subject to later technical naming decisions.

## Vision

One is a general-purpose programming language designed so ordinary users can create software using a human-oriented programming surface without sacrificing deterministic semantics, scalability, portability, or interoperability.

A user should be able to use One for a complete project from creation through development, testing, build, packaging, deployment, and maintenance without another programming language being mandatory.

## Core Principles

1. **Natural** — source should feel close to ordinary human instructions.
2. **Deterministic** — accepted source has formal, reproducible meaning.
3. **Minimal punctuation** — punctuation should not be the primary barrier to beginners.
4. **Semantic-first** — different surface languages map to a shared semantic model.
5. **Localization-native** — localization is an architectural feature, not keyword translation.
6. **Progressive complexity** — simple programs remain simple while advanced programs remain possible.

## Core Constructs

- DATA
- ACTION
- CONDITION
- LOOP
- FUNCTION
- EVENT

## Language Rules

- Natural-language style does not mean arbitrary free-form prose.
- Grammar must be formal and deterministic.
- Newlines and indentation may express program structure.
- Ambiguity must be rejected and diagnosed rather than guessed.
- Diagnostics must explain what, where, why, and useful correction guidance.
- Source must not be silently rewritten by the compiler/runtime.
- LLMs are not required for parsing, compilation, execution, or runtime semantics.

## Platforms

### Windows

Windows is a first-class platform because One targets ordinary users as well as developers.

### Linux

Linux is a first-class platform for technical users, servers, development, and system tooling. Ubuntu, Debian, Kali, Fedora, Arch, and other compatible distributions are target environments as the platform layer matures.

Cross-platform One source should preserve semantic behavior through platform abstractions wherever practical.

## Interoperability

One must be able to interoperate deeply with:

- JavaScript / Node.js
- Python
- C / C++
- Java / JVM
- HTML / CSS
- shell and OS tooling
- databases
- networking
- native libraries and APIs

These are interoperability capabilities, not mandatory dependencies for completing a One project.

## Project Scale

One must support the progression from:

- a simple script;
- a small application;
- a multi-module project;
- a large application;
- to multi-component systems.

The architecture must therefore account for modules, types, packages, dependencies, testing, build systems, error handling, concurrency, security, deployment, and maintenance.

## Architecture

The canonical conceptual pipeline is:

```text
Localized Source
    ↓
Localized Lexer
    ↓
Parser
    ↓
AST
    ↓
Semantic Analysis
    ↓
Common Semantic IR
    ↓
Runtime / Compiler Backend
    ↓
Windows / Linux / Other Targets
```

The semantic model must not depend on one human language.

## Originality Position

One does not claim that natural-language programming is unprecedented. Prior art exists. The intended identity of One is the concrete combination of:

- conversational programming surface;
- deterministic formal grammar;
- shared semantic IR;
- native localization architecture;
- progressive complexity;
- general-purpose project capability;
- Windows and Linux as first-class platforms;
- deep interoperability.

## Development Governance

GitHub is the canonical source of truth for One. Local/Termux environments are verification and validation environments, not the primary source of project changes.

Development must follow technical dependencies. Later layers must not invent contracts that belong to earlier layers.

The next design gate is the formal lexical and grammar specification, followed by AST, scope/name resolution, type system, semantic model, IR, reference interpreter, conformance tests, compiler, runtime, tooling, platforms, ecosystem, and interoperability.
