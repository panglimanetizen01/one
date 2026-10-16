# One Human-Language Tool Orchestration and Governance Implementation Plan

> **Current status:** Tasks 1–11 are complete for the orchestration runtime foundation and have fresh exact-SHA cross-platform evidence. The next dependency-ordered boundary is language-level One-source orchestration.

**Goal:** Add a deterministic, general-purpose human-language tool-orchestration foundation to One so semantic operations can invoke typed external capabilities through governed adapters without making tool command syntax the language semantics.

**Architecture:** One source remains authoritative through the existing localized lexer → parser → AST → semantic analysis → CSIR/IR → runtime pipeline. Orchestration is a typed capability/runtime layer behind that semantic boundary. Natural-language wording does not grant authority; policy is evaluated over typed semantic requests.

**Evidence rule:** `Specified → Designed → Implemented → Verified → Supported`. Source presence is not execution evidence. Every later canonical SHA requires fresh exact-SHA verification.

## Completed foundation tasks

### Task 1 — Typed orchestration domain model — COMPLETE

Implemented `SemanticCapabilityId`, `CapabilityRequest`, `ExecutionScope`, `EffectClass`, `AdapterId`, `AdapterStatus`, and deterministic validation in `crates/one-core/src/orchestration.rs`.

### Task 2 — Semantic-to-capability resolution — COMPLETE

Implemented deterministic semantic resolution and Indonesian/English localized equivalence/ambiguity fixtures. Localization does not change capability identity or required effects.

### Task 3 — Execution scope and governance policy — COMPLETE

Implemented `ExecutionPolicy`, `PolicyDecision`, `ScopeRule`, explicit deny/allow/approval behavior, and deterministic policy tests in `governance.rs`.

### Task 4 — External adapter contract — COMPLETE

Implemented typed `ToolAdapter`, `AdapterRequest`, `AdapterResult`, `AdapterMetadata`, platform compatibility, and rejection of undeclared capabilities. No free-form command-string field is used as the semantic execution boundary.

### Task 5 — Orchestration lifecycle — COMPLETE

Implemented `Orchestrator` with request validation → policy evaluation → deterministic compatible-adapter selection → adapter invocation. Denied requests do not reach adapter invocation.

### Task 6 — Result normalization — COMPLETE

Implemented `NormalizedResult`, explicit normalization expectations, evidence certainty, deterministic failure classification, and normalization tests.

### Task 7 — Audit records — COMPLETE

Implemented orchestration audit records distinguishing policy denial from attempted execution and preserving observed failure/uncertainty without inventing external facts.

### Task 8 — First real adapter — COMPLETE

Implemented `network.dns.resolve` using the platform resolver with typed target scope. The adapter has success/failure classification tests and governed end-to-end runtime coverage.

### Task 9 — Localization/conformance — COMPLETE

Added Indonesian/English semantic-equivalence and ambiguity coverage for orchestration capabilities.

### Task 10 — Cross-contract integration — COMPLETE

Added the orchestration implementation contract and synchronized capability/runtime/verification boundaries. Contract-artifact checks pass in the independent matrix.

### Task 11 — Exact-SHA cross-platform verification — COMPLETE

Independent verification run `38871435579` checked out canonical One source SHA `be2e65852e9a4be833e659524a12c6359769665b` and passed on Ubuntu and Windows Server 2025:

- canonical SHA checkout and equality check;
- contract-artifact verification;
- workspace build;
- Clippy with `-D warnings`;
- workspace tests;
- real-network runtime tests;
- offline verification;
- deterministic `.oneir` and `.onepkg` checks through the offline verifiers.

## Next dependency boundary: language-level orchestration

### Task 12 — Connect One source semantics to the orchestration runtime — IN DEVELOPMENT

**Target path:**

```text
One source
→ parser
→ AST
→ semantic operation
→ capability resolution
→ governance
→ adapter
→ normalized result
→ One value/diagnostic
→ CLI/runtime
```

**Required constraints:**

- Do not create a second interpreter or parallel semantic authority.
- Reuse the existing parser, semantic model, capability model, IR/runtime, diagnostics, `Orchestrator`, and normalization layer.
- Ambiguous source must be rejected deterministically.
- Policy must execute before any adapter invocation.
- Adapter command syntax must remain outside One semantic meaning.
- A denied or unavailable operation must become a deterministic One diagnostic/result.
- Existing non-orchestration programs must remain compatible.

**Required TDD sequence:**

1. Write a failing end-to-end test for a concrete One source operation that should resolve to a typed capability request.
2. Prove the current pipeline fails to reach `Orchestrator`.
3. Implement the smallest semantic/IR/runtime bridge necessary.
4. Add policy-denied, out-of-scope, unavailable-adapter, adapter-failure, and successful-result cases.
5. Prove the successful path reaches the real DNS adapter and returns a normalized One result.
6. Add CLI/runtime integration evidence.
7. Run workspace tests and Clippy.
8. Run independent exact-SHA verification after the implementation is merged.
9. Only then change language-level orchestration status from `Dalam pengembangan` to `Verified` or `Supported`, as justified by evidence.

## Later capability expansion

After Task 12 is verified, extend the same dependency chain per capability:

`foundation → semantic → runtime → capability → adapter → CLI → test → verification → documentation`.

Prioritized families:

1. filesystem/storage;
2. process/system services;
3. database;
4. HTTP/API;
5. build/package;
6. testing;
7. platform APIs;
8. native interoperability;
9. frontend/backend tooling;
10. desktop/mobile adapters;
11. localization.

Each family remains subject to the same evidence status model. A registry entry or architectural specification is not an implementation or support claim.

## Completion gate

The original orchestration foundation plan is complete through Task 11. The overall orchestration feature is not complete until the language-level end-to-end path in Task 12 has executable evidence and the resulting canonical SHA has fresh Linux/Windows verification.
