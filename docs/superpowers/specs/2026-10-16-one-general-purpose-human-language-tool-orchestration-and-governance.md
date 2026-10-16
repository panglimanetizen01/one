# One — General-Purpose Human-Language Tool Orchestration and Governance

**Status:** Implemented orchestration foundation; language-level One-source orchestration remains in development.
**Date:** 2026-10-16
**Scope:** General-purpose One language, human-language programming, external-tool orchestration, capability/runtime governance, localization, interoperability, auditability, and platform control

## Current implementation boundary

The architectural design in this document is now partially executable through a verified runtime foundation. The canonical source contains typed semantic capability requests, execution scopes/effects, deterministic semantic-to-capability resolution, deterministic governance, typed external adapters, deterministic adapter selection, normalized results/evidence, audit records, localized semantic-resolution conformance coverage, and the real `network.dns.resolve` adapter.

Independent run `38872070590` verified this foundation at source baseline `dba4fad49854952e256ae41744f57f93e39167a8` on Ubuntu and Windows Server 2025, including build, Clippy, workspace tests, real-network runtime tests, contract checks, and offline verification.

This does **not** yet mean that a normal One source program can invoke the orchestration runtime end-to-end. The current CLI still follows the established source → parser → semantic analysis → IR → reference interpreter path. The missing language-level bridge is explicitly the next implementation boundary.

## 1. Purpose

One is a general-purpose programming language intended for all users, including people who do not already understand conventional programming syntax or technical tool command languages.

One separates language semantics from capability semantics, runtime governance, platform/tool policy, and adapter implementation. Human-language wording is an input surface, not an authority boundary.

## 2. Core principle

> **One is a human-language programming language, not a human-language disguise for technical commands.**

The intended translation chain is:

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

The chain above is the architectural target. The verified runtime foundation currently implements the lower capability/governance/adapter/result layers; the source-language bridge is not yet verified.

## 3. Existing One semantic pipeline remains authoritative

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

The orchestration layer must consume this pipeline rather than creating a parallel interpreter or semantic authority.

## 4. Human-language programming model

One may eventually permit beginner-oriented semantic operations expressed in ordinary language. Examples in this document are design examples unless an accompanying grammar/semantic contract and executable evidence says otherwise.

## 5. Intent is not authority

For every external effect, the runtime evaluates the required capability, requested scope, and execution policy before invoking an adapter. Changing wording or localization must not change the authorization requirements for the same semantic operation.

## 6. Capability model

Every external operation exposed to One must have a machine-readable capability declaration covering semantic identity, effects, input/output schema, scope, privilege requirements, platform/backend availability, adapter identity, audit requirements, determinism, and verification status as applicable.

## 7. Tool Capability Registry

The registry maps semantic capability contracts to available adapters. Registry presence does not prove implementation, verification, or support.

Status remains **Specified → Designed → Implemented → Verified → Supported**.

## 8. Tool abstraction

Users address semantic capabilities rather than external command syntax. Adapter selection must be deterministic and must reject materially incompatible substitutions.

## 9. Technical control

Progressive precision remains an architectural goal: beginner semantic operations, named parameters, explicit capability configuration, and expert adapter/platform controls can all map to the same One semantic model.

## 10. Result normalization

External results are normalized into typed semantic results without inventing facts. Material uncertainty must remain uncertainty.

## 11. Human-readable explanations

Explanation layers may render deterministic structured results and diagnostics for users. They are not semantic or execution authorities.

## 12. Localization

Localized surfaces may use different vocabulary and grammar but must preserve capability identity, effects, authorization requirements, scope semantics, result types, error semantics, and deterministic execution semantics.

## 13. General-purpose scope

The architecture is domain-neutral and can later cover filesystem/storage, processes/system services, networking, databases, HTTP/API clients, build/package tools, version control, media, graphics/design, testing, scientific/data tools, OS APIs, devices, developer tooling, security/network-analysis tooling, cloud services, and native libraries.

This is an architectural inventory, not a v0.1 implementation claim.

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

Steps 1–12 are implemented at the runtime foundation boundary. A verified One-source program path into these steps is the next implementation task.

## 17. Audit model

Audit data records observed/requested runtime facts such as capability identity, adapter identity, scope, policy decision, execution state, and relevant diagnostics. It must not claim facts the runtime cannot verify.

## 18. Failure semantics

Orchestration failures remain typed and distinguish invalid source/parameters, ambiguity, missing capability, policy denial, out-of-scope target, unavailable adapter, unsupported platform, adapter failure, malformed output, timeout, interruption, and uncertainty where applicable.

## 19. Testing and conformance

The architecture requires language, capability, adapter, and end-to-end tests. No adapter is marked Verified without executable evidence appropriate to its backend. The current foundation has executable tests for the runtime layers and a real DNS adapter; the missing end-to-end language-level test is the next release boundary.

## 20. Compatibility with existing One architecture

This architecture extends, rather than replaces, the lexer/parser, AST/name-scope model, type/semantic model, CSIR/IR, reference interpreter, capability model, diagnostics, conformance system, network model, and deterministic artifact model.

## 21. v0.1 boundary

The verified v0.1 boundary is the orchestration runtime foundation, not a claim that arbitrary human-language source can already invoke every external capability. Tool-specific support is incremental and evidence-based.

## 22. Non-goals

This specification does not make every external tool part of the standard library, make command syntax the One language, use an LLM as execution authority, grant capabilities from wording alone, or claim implementation from registry entries.

## 23. Success criteria

The architecture succeeds when useful programming intent can be expressed in understandable language, localization preserves semantic identity, One can orchestrate real external capabilities through typed adapters, external tools remain implementation backends, advanced users retain precise controls, capability/policy enforcement is independent of wording, results are normalized without invented facts, supported adapters have executable evidence, One remains general-purpose, and platform/owner governance can control execution without changing language meaning.

## 24. Implementation gate

The generic orchestration foundation is implemented and independently verified. The next implementation gate is language-level integration:

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

Production changes for this bridge must use fail-first/TDD, evidence-first debugging, and a fresh exact-SHA cross-platform verification before the language-level feature is called Verified or Supported.
