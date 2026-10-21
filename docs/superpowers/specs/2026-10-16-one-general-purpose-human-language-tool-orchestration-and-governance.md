# One — General-Purpose Human-Language Tool Orchestration and Governance

**Status:** Runtime orchestration foundation verified. DNS, filesystem read/write, process execution, SQLite database query, and HTTP GET source-level slices have executable evidence at their recorded exact-SHA boundaries. Database query and HTTP GET were included in the successful Ubuntu/Windows Server 2025 matrix for source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`; they remain experimental with deliberately narrow operation boundaries.
**Date:** 2026-10-16
**Scope:** General-purpose One language, human-language programming, external-tool orchestration, capability/runtime governance, localization, interoperability, auditability, and platform control

## Current implementation boundary

The architecture is executable through a verified runtime foundation and multiple source-level vertical slices. The canonical source contains typed semantic capability requests, execution scopes/effects, deterministic semantic resolution, deterministic governance, typed external adapters, normalized results/evidence, audit records, localized semantic-resolution conformance, and governed source operations.

Independent exact-SHA matrix run [`39554661451`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451) checked out canonical source `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` and passed on Ubuntu and Windows Server 2025. The matrix ran contract checks, build, Clippy, workspace tests, real-network tests, and platform-appropriate offline verification; the exact-SHA verifier also explicitly runs the database and HTTP GET vertical-slice core/CLI suites. Canonical hosted CI failures with `steps: []` remain inconclusive infrastructure evidence and do not invalidate this independent executable evidence.

## 1. Purpose

One is a general-purpose programming language intended for all users, including people who do not already understand conventional programming syntax or technical tool command languages.

One separates language semantics from capability semantics, runtime governance, platform/tool policy, and adapter implementation. Human-language wording is an input surface, not an authority boundary.

## 2. Core principle

> **One is a human-language programming language, not a human-language disguise for technical commands.**

Every source-level orchestration path follows the same controlled semantic boundary:

```text
One source
        ↓
Localized parser / semantic validation
        ↓
Typed orchestration IR
        ↓
Explicit capability + scope governance
        ↓
Governed adapter
        ↓
Normalized result
        ↓
CLI/runtime output
```

The current source-level examples are DNS, filesystem read/write, process execution, database query, and HTTP GET. They are separate semantic operations, not arbitrary prose-to-command translation.

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

Current database source form:

```text
kueri database ":memory:" dengan "SELECT ?1" dan "42"
```

Current HTTP GET source form:

```text
ambil dari "http://127.0.0.1:8080/"
```

Both are bounded grammar productions. Neither authorizes external access by itself.

## 5. Intent is not authority

For every external effect, the runtime evaluates the required capability, requested scope, and execution policy before invoking an adapter. Changing wording or localization must not change authorization requirements for the same semantic operation.

## 6. Capability model

Every external operation exposed to One must have a machine-readable capability declaration covering semantic identity, effects, input/output schema, scope, privilege requirements, platform/backend availability, adapter identity, audit requirements, determinism, and verification status as applicable.

Current database capability:

```text
semantic: database.query
runtime capability: DatabaseQuery
scope: Resource(database-resource)
effect: Read
adapter: database.sqlite.query
```

Current HTTP GET capability:

```text
semantic: network.connect
runtime capability: NetworkConnect
scope: Target(exact-http-url)
effect: Connect
adapter: network.http.get
```

The current database capability is read-only and parameterized. Database mutation requires a separate capability contract.

The current HTTP source slice is limited to explicit `http://` URLs and GET over the existing HTTP/1.1 runtime. HTTPS, arbitrary methods, custom headers/bodies, redirects, cookies, and API-specific authentication require separate contracts.

## 7. Tool Capability Registry

The registry maps semantic capability contracts to available adapters. Registry presence does not prove implementation, verification, or support.

Status remains **Specified → Designed → Implemented → Verified → Supported**.

## 8. Tool abstraction

Users address semantic capabilities rather than external command syntax. Adapter selection must be deterministic and reject materially incompatible substitutions.

## 9. Technical control

Progressive precision remains an architectural goal: beginner semantic operations, named parameters, explicit capability configuration, and expert adapter/platform controls can all map to the same One semantic model.

## 10. Result normalization

External results are normalized into typed semantic results without inventing facts. Material uncertainty remains uncertainty.

The database adapter currently normalizes SQLite scalar values into deterministic text rows for the declared UTF-8 result expectation.

The HTTP GET adapter currently returns the successful HTTP response body under the declared UTF-8 normalization expectation and rejects non-2xx HTTP responses as adapter failures.

## 11. Human-readable explanations

Explanation layers may render deterministic structured results and diagnostics for users. They are not semantic or execution authorities.

## 12. Localization

Localized surfaces may use different vocabulary and grammar but must preserve capability identity, effects, authorization requirements, scope semantics, result types, error semantics, and deterministic execution semantics.

## 13. General-purpose scope

The architecture is domain-neutral and can later cover filesystem/storage, processes/system services, networking, databases, HTTP/API clients, build/package tools, version control, media, graphics/design, testing, scientific/data tools, OS APIs, devices, developer tooling, security/network-analysis tooling, cloud services, and native libraries.

This is an architectural inventory, not a claim that all listed capabilities are currently implemented or supported.

## 14. Governance plane

The runtime governance plane may grant/withhold capabilities, restrict scope, require approval, restrict/disable adapters, and record execution/audit events. Governance state remains separate from language semantics.

Database queries use typed `Resource` scope and must match the explicitly granted resource.

HTTP GET uses typed `Target` scope and must match the exact URL supplied to `--target`.

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

The database and HTTP source slices follow this lifecycle in source, core tests, and CLI tests. Their exact-SHA tests passed in the recorded Ubuntu/Windows Server 2025 matrix; their verified boundaries remain experimental and do not imply support for database mutation, HTTPS, arbitrary HTTP methods, or untested target architectures.

## 17. Audit model

Audit data records observed/requested runtime facts such as capability identity, adapter identity, scope, policy decision, execution state, and relevant diagnostics. It must not claim facts the runtime cannot verify.

## 18. Failure semantics

Orchestration failures remain typed and distinguish invalid source/parameters, ambiguity, missing capability, policy denial, out-of-scope target, unavailable adapter, unsupported platform, adapter failure, malformed output, timeout, interruption, and uncertainty where applicable.

Database-specific failures currently map invalid arguments to adapter rejection and SQLite open/prepare/query failures to backend failure. Policy denial occurs before adapter invocation.

HTTP-specific failures currently reject unsupported schemes/invalid URL targets at the adapter boundary, map network/HTTP runtime failures to backend failure, and treat non-2xx responses as backend failure for the current UTF-8 body-returning slice. Policy denial occurs before adapter invocation.

## 19. Testing and conformance

Existing vertical slices have executable tests at their recorded revisions. The current database slice includes core tests for adapter success, default denial, resource-scope denial, backend failure, deterministic normalization, source-to-runtime success, and CLI success/denial/scope denial. The current HTTP slice includes source-to-runtime success against a local HTTP server, default capability denial, and out-of-scope denial, plus adapter URL validation coverage.

These tests are source evidence only until executed at the exact source/test revision.

## 20. Compatibility with existing One architecture

This architecture extends, rather than replaces, the lexer/parser, AST/name-scope model, type/semantic model, CSIR/IR, reference interpreter, capability model, diagnostics, conformance system, network model, and deterministic artifact model.

## 21. v0.1 boundary

The verified v0.1 boundary remains limited to recorded exact-SHA evidence. Database query and HTTP GET are executable-verified at source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` on Ubuntu and Windows Server 2025, but remain experimental and are not promoted to generally Supported capabilities.

## 22. Non-goals

This specification does not make every external tool part of the standard library, make command syntax the One language, use an LLM as execution authority, grant capabilities from wording alone, or claim implementation from registry entries.

## 23. Success criteria

The architecture succeeds when useful programming intent can be expressed in understandable language, localization preserves semantic identity, One can orchestrate real external capabilities through typed adapters, external tools remain implementation backends, advanced users retain precise controls, capability/policy enforcement is independent of wording, results are normalized without invented facts, supported adapters have executable evidence, One remains general-purpose, and platform/owner governance can control execution without changing language meaning.

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

Production changes for each new capability must use fail-first/TDD, evidence-first debugging, explicit policy/scope, and a fresh exact-SHA Linux/Windows verification before the capability is called Verified or Supported.
