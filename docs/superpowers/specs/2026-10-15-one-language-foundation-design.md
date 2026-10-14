# One Language — v0.1 Foundation & Architecture Design

**Status:** APPROVED
**Date:** 2026-10-15
**Canonical repository:** `panglimanetizen01/one`

## 1. Purpose

One is a general-purpose programming language designed to let ordinary users create software with a human-oriented conversational surface while retaining formal, deterministic semantics suitable for serious software engineering.

One must ultimately support the complete project lifecycle without requiring another programming language as a mandatory dependency. Other ecosystems are interoperability targets, not prerequisites.

## 2. Locked Foundation

- Language name: **One**.
- Indonesian is the initial implementation language.
- Localization is a first-class architectural concern.
- Surface syntax is conversational and human-oriented.
- Grammar is formal and deterministic; arbitrary prose is not accepted.
- Required punctuation is minimized; newlines and indentation may express structure.
- Semantic meaning is independent of any one human-language grammar.
- Core constructs: DATA, ACTION, CONDITION, LOOP, FUNCTION, EVENT.
- Ambiguity is rejected and diagnosed; the implementation never guesses.
- Diagnostics explain what, where, why, and useful correction guidance without silently changing source.
- LLMs are not required for parsing, compilation, execution, testing, or runtime semantics.
- One is general-purpose and must scale from small scripts to large projects.
- Windows and Linux are first-class platforms.
- One must be usable for a complete project from start to finish without another programming language being mandatory.
- Deep interoperability is an architectural requirement for JavaScript/Node.js, Python, C/C++, Java/JVM, HTML/CSS, shell/Linux tooling, databases, networking, and native libraries as their contracts mature.
- Cross-platform semantic behavior is preserved through platform abstraction.
- Security/capability boundaries are required for filesystem, process, network, shell, native, and OS operations.
- Prior art in natural-language programming is acknowledged; originality claims are limited to One's concrete design combination.

## 3. Architectural Invariants

### 3.1 Semantics over surface syntax

Different human-language grammars may map to the same semantic constructs.

### 3.2 Determinism

An accepted program has one specified parse and semantic result for a given language version/configuration. Ambiguous source fails.

### 3.3 Runtime independence from AI

The canonical execution path works deterministically without network access or an LLM.

### 3.4 Progressive complexity

The beginner surface remains approachable while the underlying language can provide modules, types, packages, error handling, concurrency, native integration, testing, build tooling, and large-system facilities.

### 3.5 Platform abstraction

Windows and Linux are peer platforms. Platform-specific behavior is isolated behind explicit runtime/standard-library abstractions wherever practical.

### 3.6 Interoperability without semantic compromise

Interop boundaries use explicit value, error, ownership/lifetime, calling-convention, and capability contracts.

### 3.7 Source preservation

Compilation/execution never silently rewrites user source.

## 4. Layered Architecture

```text
Localized Source
      |
      v
Localized Lexer
      |
      v
Concrete Parser
      |
      v
AST
      |
      +--> Name / Scope Resolution
      +--> Type Analysis
      +--> Effect / Capability Analysis
      |
      v
Semantic Model
      |
      v
One Semantic IR
      |
      +--> Reference Interpreter
      +--> Compiler Backend(s)
      |
      v
Runtime / Standard Library
      |
      +--> Windows Platform Layer
      +--> Linux Platform Layer
      +--> Web / Native / Foreign Interop
```

The Semantic IR is the principal boundary between localized syntax and execution.

## 5. Dependency-Ordered Development

### Phase 0 — Foundation

1. Language invariants
2. Architecture constraints
3. Localization contract
4. Platform abstraction contract

### Phase 1 — Language Core

5. Lexical specification
6. Concrete grammar
7. AST specification
8. Identifier/name resolution
9. Scope model
10. Type system
11. Semantic specification
12. Diagnostics model

### Phase 2 — Execution Core

13. Semantic IR
14. Runtime model
15. Effect/capability model
16. Reference interpreter

### Phase 3 — Correctness

17. Conformance suite
18. Golden parser/AST tests
19. Semantic and diagnostic tests
20. Cross-platform semantic tests

### Phase 4 — Toolchain

21. Compiler
22. Standard library
23. One CLI
24. Debugging/developer tooling

### Phase 5 — Platforms

25. Windows runtime/platform support
26. Linux runtime/platform support
27. Cross-platform packaging

### Phase 6 — Ecosystem

28. Module system
29. Package/dependency system
30. Build system
31. Project scaffolding
32. Testing framework
33. Deployment tooling

### Phase 7 — Interoperability

34. C/C++ FFI
35. Python integration
36. JavaScript/Node.js integration
37. Java/JVM integration
38. HTML/CSS/web tooling

### Phase 8 — Expansion

39. Additional human-language grammars
40. IDE/editor tooling
41. Documentation ecosystem
42. Production hardening

Independent work may proceed in parallel only when dependency contracts are stable. No later layer may invent semantics belonging to an earlier contract.

## 6. Core Constructs

```text
nama adalah Budi
umur adalah 25
umur menjadi 26

tampilkan nama

kalau umur sudah 17 tahun
    tampilkan "Boleh masuk"
kalau tidak
    tampilkan "Belum boleh masuk"

untuk setiap nama di daftar
    sapa nama

buat pekerjaan hitung
    terima harga
    terima pajak
    hasilnya adalah harga ditambah pajak

ketika pengguna menekan tombol beli
    tampilkan "dibeli"
```

These examples are governed by the formal grammar documents under `docs/spec/`.

## 7. Localization Contract

Each locale may define lexical rules, keywords/phrases, phrase ordering, grammatical markers, diagnostics language, and locale-specific conventions. Every locale maps to the same semantic model. Indonesian is the first locale; the architecture must not make English or future locales impossible.

## 8. Platform Contract

### Windows

First-class target for ordinary users, eventually covering filesystem, process, environment, networking, permissions, GUI integration where applicable, executable packaging, services, and required native APIs.

### Linux

First-class target for developers and servers, eventually covering terminal/CLI, filesystem, process, environment, pipes, sockets, permissions, services, native libraries, and OS integration.

Application-level One source should remain portable unless it explicitly opts into platform-specific capabilities.

## 9. Interoperability Contract

Interop must eventually define value conversion, ownership/lifetime, error propagation, calling conventions, async/thread boundaries, encoding, security/capability rules, and version compatibility. Implemented interoperability may not be advertised beyond its verified contract.

## 10. Verification Strategy

Required evidence classes include lexical, parser, AST, scope, type, semantic, ambiguity, diagnostic, IR, interpreter, compiler-equivalence, standard-library, Windows/Linux, interoperability, localization, and regression tests.

The reference interpreter and conformance suite are semantic guardrails for later compiler/runtime implementations.

## 11. Non-Goals

One is not an LLM-to-code chatbot, arbitrary free-form natural-language interpreter, thin Python/JavaScript wrapper, keyword translation exercise, tutorial-only toy language, or system that requires another language to finish a project.

## 12. Approval Decision

The architecture and dependency ordering in this document were explicitly approved by the project owner before implementation began.

The detailed lexical specification, concrete grammar, implementation plan, and v0.1 reference implementation now live alongside this document in the canonical repository.

**Status: APPROVED / LOCKED FOUNDATION**
