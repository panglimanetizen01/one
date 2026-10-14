# One Language — v0.1 Foundation & Architecture Design

**Status:** Draft for review
**Date:** 2026-10-15
**Canonical repository:** `panglimanetizen01/one`

## 1. Purpose

One is a new general-purpose programming language designed to let ordinary users create software using a human-oriented, conversational surface syntax while retaining formal, deterministic semantics suitable for serious software engineering.

The language must support the complete project lifecycle—from project creation through development, testing, building, packaging, deployment, and maintenance—without requiring another programming language as a mandatory dependency.

Other ecosystems remain first-class interoperability targets rather than prerequisites.

## 2. Locked Foundation

The following decisions are part of the approved One v0.1 foundation:

- Language name: **One**.
- Primary initial human language: Indonesian.
- Localization is a first-class architectural concern.
- Surface syntax is conversational and human-oriented.
- Grammar is formal and deterministic; One does not accept arbitrary natural-language prose.
- Required punctuation is minimized; newlines and indentation may express program structure.
- Semantic meaning is represented independently from any one human-language surface grammar.
- Core constructs: DATA, ACTION, CONDITION, LOOP, FUNCTION, EVENT.
- Ambiguity is rejected and diagnosed; the compiler/runtime must not guess between valid interpretations.
- Diagnostics must explain what, where, why, and useful correction guidance without silently changing source.
- LLMs are not required for parsing, compilation, execution, testing, or runtime semantics.
- One is general-purpose and must scale from small scripts to large projects.
- Windows and Linux are first-class platforms.
- One must be usable for a complete project from start to finish without another programming language being mandatory.
- Deep interoperability is required with JavaScript/Node.js, Python, C/C++, Java/JVM, HTML/CSS, shell/Linux tooling, databases, networking, and native libraries as the architecture matures.
- Cross-platform source/semantic behavior should be preserved through platform abstraction rather than duplicated application logic.
- Security/capability boundaries are architectural requirements for filesystem, process, network, shell, native, and OS operations.
- Prior art in natural-language programming is acknowledged; originality claims are limited to One's concrete combination of conversational syntax, deterministic formal grammar, semantic IR, native localization, progressive complexity, general-purpose scope, cross-platform support, and interoperability.

## 3. Architectural Invariants

### 3.1 Semantics over surface syntax

Different human-language grammars may map to the same semantic constructs. For example, Indonesian `kalau` and an English equivalent must ultimately represent the same conditional semantics.

### 3.2 Determinism

For every accepted source program, parsing and semantic analysis must produce a deterministic result under the specified language version and configuration. Ambiguous source must fail with a diagnostic rather than be guessed.

### 3.3 Runtime independence from AI

The canonical execution path is fully deterministic and must work without network access or an LLM.

### 3.4 Progressive complexity

The beginner surface must remain approachable while the underlying language provides modules, types, packages, error handling, concurrency, native integration, testing, build tooling, and other facilities required by large systems.

### 3.5 Platform abstraction

Platform-specific behavior must be isolated behind explicit runtime/standard-library abstractions wherever practical. Windows and Linux are peers, not a primary platform plus a compatibility afterthought.

### 3.6 Interoperability without semantic compromise

Interop boundaries must use explicit value, error, ownership/lifetime, calling-convention, and capability contracts. Foreign ecosystems must not redefine One semantics implicitly.

### 3.7 Source preservation

Tooling may offer suggestions or automated transformations, but no compiler/runtime operation may silently rewrite user source as a side effect of normal compilation or execution.

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

The semantic IR is the principal boundary between human-language surface syntax and execution.

## 5. Dependency-Ordered Development

The implementation roadmap must follow technical dependencies rather than feature popularity:

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

17. Conformance test suite
18. Golden parser/AST tests
19. Semantic and diagnostic tests
20. Cross-platform semantic tests

### Phase 4 — Toolchain

21. Compiler
22. Standard library
23. One CLI
24. Debugging and developer tooling

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

Independent work may proceed in parallel only when its dependency contracts are stable. No later layer may invent semantics that belong to an earlier contract.

## 6. Language Core Direction

The initial six semantic constructs are:

### DATA

```text
nama adalah Budi
umur adalah 25
umur menjadi 26
```

### ACTION

```text
tampilkan nama
```

### CONDITION

```text
kalau umur sudah 17 tahun
    tampilkan "Boleh masuk"
kalau tidak
    tampilkan "Belum boleh masuk"
```

### LOOP

```text
untuk setiap nama di daftar teman
    sapa nama
```

### FUNCTION

```text
buat pekerjaan hitung total
    terima harga
    terima pajak
    hasilnya adalah harga ditambah pajak
```

### EVENT

```text
ketika pengguna menekan tombol beli
    tambahkan produk ke keranjang
```

These examples are illustrative until the formal grammar specification is approved.

## 7. Localization Contract

Localization must be designed before the parser architecture is frozen.

Each locale may define:

- lexical rules;
- keywords/phrases;
- phrase ordering;
- grammatical markers;
- diagnostics language;
- locale-specific literals and conventions.

All locale grammars must map to the same semantic model.

The first implementation locale is Indonesian. English and other locales are future conformance targets, but the architecture must not make them impossible.

## 8. Platform Contract

### Windows

Windows is a first-class target for ordinary users. The platform layer must eventually cover filesystem, process, environment, networking, permissions, GUI integration where applicable, executable packaging, services, and required native APIs.

### Linux

Linux is a first-class target for developers and servers. The platform layer must eventually cover terminal/CLI, filesystem, process, environment, pipes, sockets, permissions, services, native libraries, and OS integration.

Application-level One source should remain portable unless it explicitly opts into platform-specific capabilities.

## 9. Interoperability Contract

Interoperability must be explicit and testable.

The design must eventually define:

- value conversion rules;
- ownership and lifetime rules;
- error propagation;
- calling conventions;
- async boundaries;
- thread/runtime boundaries;
- encoding rules;
- security/capability rules;
- version compatibility.

Supported ecosystems are targets of the architecture, not mandatory implementation dependencies for ordinary One programs.

## 10. Verification Strategy

Every language layer must have executable evidence.

Required classes of tests include:

- lexical tests;
- parser acceptance/rejection tests;
- AST golden tests;
- name/scope tests;
- type tests;
- semantic tests;
- ambiguity tests;
- diagnostic tests;
- IR tests;
- interpreter tests;
- compiler equivalence tests against the reference interpreter;
- standard-library tests;
- Windows/Linux cross-platform behavior tests;
- interoperability tests;
- localization conformance tests;
- regression tests.

The reference interpreter and conformance suite are the semantic guardrails for subsequent compiler/runtime implementations.

## 11. Non-Goals

One is not intended to be:

- an LLM-to-code chatbot;
- an arbitrary free-form natural-language interpreter;
- a thin Python/JavaScript wrapper;
- a translation of existing language keywords into Indonesian;
- a toy language limited to tutorials or tiny scripts;
- a system that requires another language to complete a project.

## 12. Acceptance Criteria for the Next Stage

Before implementation begins, the following must be formally specified and reviewed:

1. lexical tokens and identifier rules;
2. newline and indentation semantics;
3. literal forms;
4. comments;
5. statement boundaries;
6. expression grammar;
7. core construct grammar;
8. block structure;
9. syntax ambiguity rules;
10. source-span model for diagnostics;
11. initial AST schema;
12. localization extension points.

Only after these contracts are approved should the implementation plan be created and execution begin.

## 13. Decision

This document converts the already-approved One v0.1 foundation into an engineering architecture and dependency order. It is intentionally not the final formal grammar or implementation plan.

**Review required before implementation planning.**
