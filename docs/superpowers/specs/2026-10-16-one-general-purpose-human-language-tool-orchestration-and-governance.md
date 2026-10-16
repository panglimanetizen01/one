# One — General-Purpose Human-Language Tool Orchestration and Governance

**Status:** Proposed architectural specification — conversational design approved by project owner
**Date:** 2026-10-16
**Scope:** General-purpose One language, human-language programming, external-tool orchestration, capability/runtime governance, localization, interoperability, auditability, and platform control

## 1. Purpose

One is a general-purpose programming language intended for all users, including people who do not already understand conventional programming syntax or technical tool command languages.

One must be usable for programs and systems ranging from small personal utilities through medium applications and large software projects. Its surface language should allow ordinary human language to express programming intent while preserving a deterministic formal semantic model underneath.

One is not a cybersecurity-only language, networking-only language, automation-only language, or wrapper around a fixed collection of external tools.

The external-tool orchestration capability described here is one part of the general One platform. The same architecture applies to development tools, operating-system facilities, network software, databases, build systems, media tools, scientific software, platform APIs, and other external capabilities.

## 2. Design decision: language generality

The One language core must not encode a domain-specific prohibition merely because a capability can be misused.

The language therefore separates:

1. **Language semantics** — what a One program means.
2. **Capability semantics** — what an operation requires from its execution environment.
3. **Runtime governance** — what the current execution environment permits.
4. **Platform/tool policy** — what a deployment, administrator, project owner, or distribution permits.
5. **Tool adapter implementation** — how an approved semantic capability is realized on a particular platform.

A capability is not made semantically meaningless merely because its runtime environment may restrict it.

Conversely, a human-language phrase must never bypass the capability and governance model merely because the phrase appears harmless or conversational.

## 3. Core principle

> **One is a human-language programming language, not a human-language disguise for technical commands.**

The translation chain is:

```text
Human-readable One source
        ↓
Localized lexical/grammar layer
        ↓
Intent and program semantics
        ↓
One semantic model
        ↓
CSIR / capability-aware IR
        ↓
Runtime governance
        ↓
Tool / library / OS / platform adapter
        ↓
External capability
        ↓
Normalized result
        ↓
One value / diagnostic / human-readable explanation
```

The important property is that the external tool is an implementation backend. Its command syntax is not the semantic definition of One.

## 4. Existing One semantic pipeline remains authoritative

The existing deterministic pipeline remains unchanged:

```text
source
 → localized lexer
 → parser
 → AST
 → name/scope analysis
 → type/semantic analysis
 → CSIR
 → runtime/compiler backend
```

Human-language intent must ultimately become ordinary One semantic structures. No LLM, external tool, network service, or heuristic runtime component may become the semantic authority.

Ambiguous source must be rejected with a diagnostic unless the grammar and semantic model provide an unambiguous interpretation.

## 5. Human-language programming model

One should permit a beginner to express intent using vocabulary close to ordinary communication.

Illustrative examples:

```text
buat program yang mencatat pengeluaran saya

ketika ada data baru
    simpan data itu
    tampilkan jumlah pengeluaran
```

Tool orchestration example:

```text
periksa perangkat yang ada di jaringan saya

untuk setiap perangkat yang ditemukan
    tampilkan alamatnya
    tampilkan layanan yang terdeteksi
```

The examples describe semantic intent. They do not define the grammar by themselves.

An advanced programmer must be able to descend from the same semantic operation into precise parameters without changing programming languages.

## 6. Intent is not authority

Natural language expresses intent; it does not grant permission.

For every operation with an external effect, the runtime evaluates the required capability and current execution policy before invoking the backend.

Conceptually:

```text
intent
  ↓
required capability
  ↓
requested scope
  ↓
execution policy
  ↓
allow / deny / require explicit approval
  ↓
adapter invocation
```

This rule applies equally to filesystem access, process execution, networking, native APIs, device access, external applications, and security-related tooling.

Changing the wording of an operation must not change the authorization requirements for the underlying semantic operation.

## 7. Capability model

Every external operation exposed to One must have a machine-readable capability declaration.

A capability declaration should identify at minimum:

- semantic capability identifier;
- human-readable names and localized names;
- required effects;
- input schema;
- output schema;
- target/scope requirements;
- privilege requirements;
- whether the operation is read-only, state-changing, destructive, or otherwise high-impact;
- platform/backend availability;
- adapter identifier;
- audit requirements;
- deterministic versus environment-dependent behavior;
- verification status.

Capabilities must be composable. A high-level One operation may require several lower-level capabilities.

## 8. Tool Capability Registry

One shall maintain a registry separate from the language grammar.

The registry maps:

```text
One semantic capability
        ↓
capability contract
        ↓
available adapters
        ↓
external tool / library / platform implementation
```

A tool entry must not be interpreted as proof that the tool is supported.

Status remains evidence-based:

- **Specified** — semantic contract exists.
- **Designed** — adapter boundary exists.
- **Implemented** — adapter code exists.
- **Verified** — required tests and execution evidence pass.
- **Supported** — verified for declared platforms/backends.

The registry may contain capabilities whose adapters are unavailable on the current platform.

## 9. Tool abstraction

Users normally address capabilities rather than tool command names.

For example, the conceptual request:

```text
cari perangkat di jaringan lokal
```

may map to a network-discovery semantic capability. The runtime may select an available implementation backend according to the declared adapter registry.

If multiple adapters satisfy the same semantic contract, selection must be deterministic according to explicit policy, adapter priority, compatibility, and availability rules.

The runtime must not silently substitute an adapter whose semantics materially differ from the requested operation.

## 10. Technical control remains available

The human-language layer must not make advanced users powerless.

One therefore supports progressive precision:

```text
beginner
  → natural-language semantic operation

intermediate
  → named semantic parameters

advanced
  → explicit capability configuration

expert
  → protocol/tool/platform-specific adapter controls
```

The final level remains inside the One semantic model. External command syntax may be represented as an adapter interface or interoperability boundary rather than becoming mandatory One syntax.

## 11. Result normalization

External tools produce heterogeneous output formats. One must normalize them into typed semantic results.

```text
external output
    ↓
adapter parser
    ↓
normalized One result
    ↓
typed value / diagnostic / evidence
```

Normalization must preserve material information required for correctness.

It must not invent facts that were not present in the tool output.

When a tool reports uncertainty, the normalized result must retain that uncertainty.

## 12. Human-readable explanations

One may provide human-readable explanations of technical results.

The explanation layer receives deterministic structured results and diagnostics. It must not invent execution results, authorization decisions, or technical facts.

For example:

```text
Saya menemukan 7 perangkat.

3 perangkat merespons layanan yang dapat dikenali.
2 perangkat tidak memberikan informasi layanan.
2 perangkat tidak dapat diverifikasi lebih lanjut.
```

Advanced users may request the underlying technical evidence.

## 13. Localization

Human-language programming is localization-aware.

Indonesian, English, and future localized surfaces may express the same semantic operation using different vocabulary and grammar.

Localization must not alter:

- capability identity;
- effect identity;
- authorization requirements;
- scope semantics;
- result types;
- error semantics;
- deterministic execution semantics.

Thus:

```text
Bahasa A ─┐
Bahasa B ─┼→ same One semantic operation
Bahasa C ─┘
```

## 14. General-purpose scope

The orchestration architecture must be domain-neutral.

Initial adapter families may include:

- filesystem and storage;
- processes and system services;
- networking;
- databases;
- HTTP/API clients;
- compilers and build tools;
- package managers;
- version-control systems;
- media processing;
- graphics and design tools;
- testing tools;
- scientific and data tools;
- operating-system APIs;
- device and hardware interfaces;
- developer tooling;
- security and network-analysis tooling;
- cloud/platform services;
- native libraries.

This list is an architectural inventory, not a v0.1 implementation claim.

## 15. Security and governance plane

Because One is general-purpose, governance must not be encoded as accidental restrictions in the natural-language grammar.

The platform therefore provides a separate governance plane capable of:

- defining execution policies;
- granting or withholding capabilities;
- restricting scope;
- requiring explicit confirmation for selected operations;
- restricting adapters;
- disabling adapters;
- recording execution/audit events;
- identifying the active policy source;
- terminating or suspending execution where the host environment supports it;
- applying distribution or deployment policy;
- supporting owner/administrator controls.

The exact policy authority hierarchy must be specified separately and must never be ambiguous.

## 16. Owner-controlled safety architecture

One may provide a platform-level control plane through which the maintainer/distributor can publish policy, revoke an adapter, disable a capability family, or require a newer runtime/policy version.

This mechanism must be treated as governance infrastructure, not as hidden language semantics.

The design must avoid a single opaque remote service becoming a mandatory semantic authority. Offline execution must remain possible where the selected capabilities and platform permit it.

Emergency controls, when implemented, must have explicit semantics, visible policy state, deterministic enforcement points, and auditable changes.

## 17. No language-level privilege escalation

The following transformations must never increase authority merely because of wording:

```text
technical command → ordinary sentence
ordinary sentence → technical command
localized phrase → English phrase
English phrase → localized phrase
```

All must resolve to the same underlying capability/effect requirements.

Similarly, aliases, localization, macros, imported libraries, adapters, and generated source must not silently bypass capability checks.

## 18. External tool orchestration lifecycle

A real adapter follows this lifecycle:

```text
1. Receive typed One capability request.
2. Validate input schema.
3. Validate target and scope.
4. Resolve required capabilities.
5. Evaluate runtime policy.
6. Select an eligible adapter.
7. Construct the adapter invocation from typed values.
8. Execute through the controlled runtime boundary.
9. Capture exit/result/error information.
10. Normalize the result.
11. Record required audit information.
12. Return a typed One result or deterministic diagnostic.
```

The adapter must not accept arbitrary natural-language text as an unchecked command string.

## 19. Audit model

For effectful external operations, the runtime should be able to record:

- program/project identity;
- capability identity;
- adapter identity;
- requested scope;
- policy decision;
- execution start/end;
- result status;
- relevant diagnostics;
- policy/version identifiers where applicable.

Sensitive output must follow the host platform's data-protection rules.

Audit data is evidence of what the runtime attempted and observed. It must not claim facts that the runtime cannot verify.

## 20. Failure semantics

An orchestration failure must be represented as a typed One error/diagnostic rather than silently translated into success.

Required distinctions include, where applicable:

- invalid source;
- ambiguous intent;
- invalid parameters;
- missing capability;
- denied by policy;
- out-of-scope target;
- unavailable adapter;
- unsupported platform;
- adapter execution failure;
- malformed external output;
- timeout;
- interrupted execution;
- incomplete/uncertain result.

## 21. Testing and conformance

The architecture requires tests at multiple layers:

### 21.1 Language tests

- natural-language grammar fixtures;
- localization equivalence;
- ambiguity rejection;
- semantic equivalence;
- parser/AST alignment.

### 21.2 Capability tests

- capability declaration validation;
- effect propagation;
- scope validation;
- policy allow/deny behavior;
- capability inheritance/composition;
- no bypass through aliases or localization.

### 21.3 Adapter tests

- input translation;
- invocation construction;
- output parsing;
- malformed-output handling;
- timeout/error propagation;
- platform compatibility.

### 21.4 End-to-end tests

```text
One source
 → semantic operation
 → capability gate
 → adapter
 → real/sandboxed backend
 → normalized result
 → One result
```

No adapter is marked Verified without executable evidence appropriate to its declared backend.

## 22. Compatibility with existing One architecture

This specification extends the existing end-to-end platform architecture.

It does not replace:

- the lexer/parser contract;
- AST and name/scope model;
- type and semantic model;
- CSIR;
- runtime model;
- effect/capability model;
- reference interpreter;
- module system;
- package/build model;
- standard-library contract;
- network semantic model;
- protocol registry;
- diagnostics and conformance system;
- deterministic artifact model.

The new orchestration layer consumes these existing foundations rather than creating a parallel semantic system.

## 23. v0.1 boundary

This specification defines the long-term architecture. It does not claim that every external tool or adapter is implemented in v0.1.

The first executable slices should establish the generic mechanism with a small number of low-risk, fully testable adapters before broadening the registry.

Candidate early adapters should be selected based on:

- deterministic behavior;
- clear input/output contracts;
- local or sandboxable execution;
- straightforward verification;
- useful general-purpose capability.

Tool-specific support is added incrementally and only promoted according to the evidence-based status model.

## 24. Non-goals

This specification does not:

- make every external tool part of the One standard library;
- make tool command syntax the One language;
- use an LLM as an execution authority;
- grant capabilities based solely on natural-language wording;
- claim implementation merely from registry entries;
- require one centralized online service for ordinary language execution;
- make localization a separate semantic language;
- collapse materially different tool behaviors into a misleading generic abstraction.

## 25. Success criteria

The architecture succeeds when:

1. An ordinary user can express useful programming intent in understandable language.
2. The same semantic operation can be localized without changing its meaning.
3. One can orchestrate real external capabilities through typed adapters.
4. External tools remain implementation backends rather than semantic authorities.
5. Advanced users can reach precise technical controls without abandoning One.
6. Capability and policy enforcement occur independently of surface wording.
7. Results are normalized without invented facts.
8. Every supported adapter has executable evidence.
9. The language remains general-purpose rather than becoming tied to a particular security or tooling domain.
10. Platform/owner governance can control execution without silently changing the meaning of the language itself.

## 26. Implementation gate

This document is a design specification only.

Before implementation begins, it must be reviewed for consistency against the existing One contracts and then converted into an implementation plan. Production changes require the project's existing fail-first/TDD and evidence-first workflow.

No external-tool adapter should be declared implemented, verified, or supported merely because this document describes it.
