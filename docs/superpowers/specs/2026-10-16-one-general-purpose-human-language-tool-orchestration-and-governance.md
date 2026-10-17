# One — General-Purpose Human-Language Tool Orchestration and Governance

**Status:** Runtime orchestration foundation verified; DNS, filesystem-read, process-execution, and filesystem-write source-level orchestration are implemented as experimental vertical slices within explicit boundaries; broader language-level capability coverage remains in development.
**Date:** 2026-10-16
**Scope:** General-purpose One language, human-language programming, external-tool orchestration, capability/runtime governance, localization, interoperability, auditability, and platform control

## Current implementation boundary

The architecture is executable through a verified runtime foundation and four source-level vertical slices. The canonical source contains typed semantic capability requests, execution scopes/effects, deterministic semantic resolution, deterministic governance, typed external adapters, normalized results/evidence, audit records, localized semantic-resolution conformance, and source-level `cari alamat`, `baca berkas`, `jalankan proses`, and `tulis ke berkas` operations that reach governed adapters through runtime execution.

The latest independently verified cross-platform source/runtime revision is `9038ac75f197753ff1ab6af7478e63e4831903a2` via mirror run `38929072800` on Ubuntu and Windows Server 2025. That evidence predates the filesystem-write executable slice and must not be generalized to later executable revisions.

The latest exact-SHA executable evidence for the filesystem-write CLI boundary is `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8`, verified on Termux/Android. Contract check, build, Clippy, targeted filesystem-write CLI test, full workspace tests, and offline verification passed for that SHA. This does not establish Ubuntu/Windows support for that executable revision.

## 1. Purpose

One is a general-purpose programming language intended for all users, including people who do not already understand conventional programming syntax or technical tool command languages.

One separates language semantics from capability semantics, runtime governance, platform/tool policy, and adapter implementation. Human-language wording is an input surface, not an authority boundary.

## 2. Core principle

> **One is a human-language programming language, not a human-language disguise for technical commands.**

The verified DNS path is:

```text
One source: cari alamat "127.0.0.1"
        ↓
Localized parser / semantic validation
        ↓
Typed orchestration IR
        ↓
Explicit target-scoped governance
        ↓
network.dns.resolve adapter
        ↓
Normalized result
        ↓
CLI output
```

The verified filesystem-read path is:

```text
One source: baca berkas "data/contoh.txt"
        ↓
Localized parser / semantic validation
        ↓
Typed orchestration IR
        ↓
Explicit target-scoped governance
        ↓
filesystem.read adapter
        ↓
Normalized UTF-8 text result
        ↓
CLI output
```

The implemented process-execution path is experimental and explicitly bounded to the currently verified single-program/no-arguments boundary.

The implemented filesystem-write path is experimental and explicitly bounded to one text payload and one explicitly targeted file with `filesystem.write` permission. It has executable evidence on Termux/Android only at `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8`.

These are experimental vertical slices, not the complete orchestration catalog.

## 3. Existing One semantic pipeline remains authoritative

```text
source
 → localized lexer
 → parser
 → AST
 → name/scope analysis
 → type/semantic analysis
 → CSIR/IR
 → runtime/compiler backend
```

The orchestration layer consumes this pipeline rather than creating a parallel interpreter or semantic authority.

## 4. Human-language programming model

One may express semantic operations in understandable language. Examples are normative only when covered by grammar/semantic contracts and executable tests.

Current source-level orchestration operations are:

- `cari alamat <target>` — experimental DNS slice;
- `baca berkas <path>` — experimental filesystem-read slice;
- `jalankan proses <program>` — experimental process-execution slice within its declared boundary;
- `tulis ke berkas <target> dengan <text>` — experimental filesystem-write slice within its declared target/payload boundary.

They resolve to distinct semantic capabilities and governed adapters.

## 5. Intent is not authority

For every external effect, the runtime evaluates the required capability, requested scope, and execution policy before invoking an adapter. Changing wording or localization must not change authorization requirements for the same semantic operation.

## 6. Capability model

Every external operation exposed to One must have a machine-readable capability declaration covering semantic identity, effects, input/output schema, scope, privilege requirements, platform/backend availability, adapter identity, audit requirements, determinism, and verification status as applicable.

## 7. Tool Capability Registry

The registry maps semantic capability contracts to available adapters. Registry presence does not prove implementation, verification, or support.

Status remains **Specified → Designed → Implemented → Verified → Supported**.

## 8. Tool abstraction

Users address semantic capabilities rather than external command syntax. Adapter selection must be deterministic and reject materially incompatible substitutions.

## 9. Technical control

Progressive precision remains an architectural goal: beginner semantic operations, named parameters, explicit capability configuration, and expert adapter/platform controls can all map to the same One semantic model.

## 10. Result normalization

External results are normalized into typed semantic results without inventing facts. Material uncertainty remains uncertainty.

## 11. Human-readable explanations

Explanation layers may render deterministic structured results and diagnostics for users. They are not semantic or execution authorities.

## 12. Localization

Localized surfaces may use different vocabulary and grammar but must preserve capability identity, effects, authorization requirements, scope semantics, result types, error semantics, and deterministic execution semantics.

## 13. General-purpose scope

The architecture is domain-neutral and can later cover filesystem/storage, processes/system services, networking, databases, HTTP/API clients, build/package tools, version control, media, graphics/design, testing, scientific/data tools, OS APIs, devices, developer tooling, security/network-analysis tooling, cloud services, and native libraries.

This is an architectural inventory, not a claim that all listed capabilities are currently implemented or supported.

## 14. Governance plane

The runtime governance plane may grant/withhold capabilities, restrict scope, require approval, restrict/disable adapters, and record execution/audit events. Governance state remains separate from language semantics.

## 15. No language-level privilege escalation

Localization, aliases, macros, generated source, libraries, and adapters must not silently bypass capability checks or increase authority merely by changing wording.

## 16. External tool orchestration lifecycle

The required lifecycle is:

```text
1. Receive typed One capability request.
2. Validate input schema.
3. Validate target and scope.
4. Resolve required capabilities.
5. Evaluate runtime policy.
6. Select an eligible adapter.
7. Construct adapter invocation from typed values.
8. Execute through the controlled runtime boundary.
9. Capture result/error information.
10. Normalize the result.
11. Record required audit information.
12. Return a typed One result or deterministic diagnostic.
```

The runtime foundation implements these governance/orchestration stages. The current source-level slices drive them from One source through CLI/runtime. Additional capabilities require their own source grammar/semantic boundary and evidence.

## 17. Audit model

Audit data records observed/requested runtime facts such as capability identity, adapter identity, scope, policy decision, execution state, and relevant diagnostics. It must not claim facts the runtime cannot verify.

## 18. Failure semantics

Orchestration failures remain typed and distinguish invalid source/parameters, ambiguity, missing capability, policy denial, out-of-scope target, unavailable adapter, unsupported platform, adapter failure, malformed output, timeout, interruption, and uncertainty where applicable.

## 19. Testing and conformance

The current conformance and runtime tests cover the implemented orchestration foundation and source-level slices within their declared boundaries. The cross-platform evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2` covers the earlier DNS/process/system runtime boundary on Ubuntu and Windows Server 2025. The later filesystem-write executable boundary is independently verified on Termux/Android at `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8`. No additional adapter is marked Verified without executable evidence appropriate to its backend and platform boundary.

## 20. Compatibility with existing One architecture

This architecture extends, rather than replaces, the lexer/parser, AST/name-scope model, type/semantic model, CSIR/IR, reference interpreter, capability model, diagnostics, conformance system, network model, and deterministic artifact model.

## 21. v0.1 boundary

The current v0.1 implementation boundary includes experimental DNS, filesystem-read, process-execution, and filesystem-write source-to-runtime slices with explicit limits and platform evidence. It does not claim arbitrary human-language source can already invoke every external capability, nor does it claim general filesystem management, general process management, unrestricted command orchestration, or native executable compilation.

## 22. Non-goals

This specification does not make every external tool part of the standard library, make command syntax the One language, use an LLM as execution authority, grant capabilities from wording alone, or claim implementation from registry entries.

## 23. Success criteria

The architecture succeeds when useful programming intent can be expressed in understandable language, localization preserves semantic identity, One can orchestrate real external capabilities through typed adapters, external tools remain implementation backends, advanced users retain precise controls, capability/policy enforcement is independent of wording, results are normalized without invented facts, supported adapters have executable evidence, One remains general-purpose, and platform/owner governance can control execution without changing language meaning.

The current source-level slices establish multiple executable boundaries, while broader capability coverage remains future work.

## 24. Implementation gate

The next implementation gate is broader language-level integration using the proven vertical-slice pattern:

```text
One source
→ parser
→ AST/semantic operation
→ capability resolution
→ governance
→ adapter
→ normalized result
→ One value/diagnostic
→ CLI/runtime
```

Production changes for each new capability must use fail-first/TDD, evidence-first debugging, explicit policy/scope, and a fresh exact-SHA platform verification before the capability is called Verified or Supported.
