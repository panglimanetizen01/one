# One Human-Language Tool Orchestration and Governance Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a deterministic, general-purpose human-language tool-orchestration foundation to One so semantic operations can invoke typed external capabilities through governed adapters without making tool command syntax the language semantics.

**Architecture:** Extend the existing One semantic/effect/capability pipeline rather than creating a parallel interpreter. Introduce typed capability contracts, scoped execution policy, adapter contracts, normalized results, and audit records; keep surface-language localization and semantic authority independent from external tools. Implement the generic mechanism first and promote concrete adapters only after executable evidence exists.

**Tech Stack:** Rust 2024 workspace, existing `one-core` capability/interop/runtime/IR modules, existing parser/semantic/CSIR pipeline, Rust unit/integration tests, existing contract and cross-platform verification scripts.

**Spec:** `docs/superpowers/specs/2026-10-16-one-general-purpose-human-language-tool-orchestration-and-governance.md`

## Global Constraints

- One remains a general-purpose programming language; orchestration is not a security-only or networking-only language feature.
- The existing deterministic pipeline remains authoritative: source → localized lexer → parser → AST → name/scope analysis → type/semantic analysis → CSIR → runtime/compiler backend.
- Natural-language wording must never grant authority; authorization is determined by the underlying capability/effect requirements.
- LLMs, external tools, network services, and heuristic runtime components must not become semantic authorities.
- Ambiguous source/intent must be rejected rather than guessed.
- Existing capability/effect enforcement remains the execution security boundary.
- External tools are implementation backends; their command syntax is not the semantic definition of One.
- Tool support status is evidence-based: Specified → Designed → Implemented → Verified → Supported.
- No production code without a failing test first.
- No adapter is declared Verified or Supported without executable evidence appropriate to its declared backend.
- Existing v0.1 behavior must remain compatible unless a failing test demonstrates a required correction.
- GitHub remains canonical; local/Termux is verification only.
- No GitHub paid upgrade is required for this work.

## Review Focus

1. **Surface-language bypass:** a localized or conversational alias must resolve to the same capability/effect requirements as its technical equivalent — test in Task 2.
2. **Scope confusion:** a capability grant must not silently authorize a materially different target or scope — test in Task 3.
3. **Adapter ambiguity:** two adapters with different semantics must not be selected interchangeably — test in Task 5.
4. **Malformed external output:** invalid adapter output must become a deterministic error, never a fabricated success — test in Task 6.
5. **Governance/audit drift:** policy decisions and adapter execution must produce auditable records without claiming unverifiable facts — test in Task 7.

---

### Task 1: Define the typed orchestration domain model

**Files:**
- Create: `crates/one-core/src/orchestration.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/src/orchestration.rs`

**Interfaces:**
- Consumes: existing `Capability`, `CapabilitySet`, and existing runtime/IR concepts.
- Produces: typed `SemanticCapabilityId`, `CapabilityRequest`, `ExecutionScope`, `EffectClass`, `AdapterId`, `AdapterStatus`, and deterministic validation errors for later policy and adapter tasks.

- [ ] **Step 1: Write failing tests** for stable capability identifiers, scope representation, effect classification, adapter status ordering, and invalid request rejection.
- [ ] **Step 2: Run the focused Rust tests** and verify they fail because the orchestration types do not exist.
- [ ] **Step 3: Implement the minimal typed domain model** with deterministic equality/ordering and no external-tool execution.
- [ ] **Step 4: Run focused tests** and verify PASS.
- [ ] **Step 5: Run the existing `one-core` tests** to verify no existing capability behavior regresses.
- [ ] **Step 6: Commit** `feat: add typed orchestration domain model`.

### Task 2: Add deterministic semantic-to-capability resolution

**Files:**
- Modify: `crates/one-core/src/orchestration.rs`
- Modify: `crates/one-core/src/capability.rs`
- Test: `crates/one-core/src/orchestration.rs`

**Interfaces:**
- Consumes: `CapabilityRequest`, existing `Capability`, and surface-independent semantic operation identifiers.
- Produces: deterministic capability/effect resolution with explicit denial for unknown or ambiguous mappings.

- [ ] **Step 1: Write failing tests** proving that equivalent localized labels resolve to one semantic capability and that aliases cannot reduce required effects.
- [ ] **Step 2: Run focused tests** and verify failure.
- [ ] **Step 3: Implement semantic capability resolution** without parsing arbitrary natural-language strings into unchecked commands.
- [ ] **Step 4: Add ambiguity rejection tests** for multiple materially different candidate capabilities.
- [ ] **Step 5: Run focused and existing capability tests** and verify PASS.
- [ ] **Step 6: Commit** `feat: resolve semantic operations to capabilities`.

### Task 3: Implement execution scope and governance policy

**Files:**
- Create: `crates/one-core/src/governance.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/src/governance.rs`

**Interfaces:**
- Consumes: resolved capability requests from Task 2.
- Produces: `ExecutionPolicy`, `PolicyDecision`, `ScopeRule`, and deterministic `allow`, `deny`, or `require_approval` decisions.

- [ ] **Step 1: Write failing tests** for missing capability, out-of-scope target, explicit denial, explicit allow, and approval-required operations.
- [ ] **Step 2: Run focused tests** and verify failure.
- [ ] **Step 3: Implement policy evaluation** as a pure deterministic operation over typed requests and policy state.
- [ ] **Step 4: Test that wording/localization changes cannot alter policy decisions for the same semantic request.**
- [ ] **Step 5: Run workspace tests** and verify PASS.
- [ ] **Step 6: Commit** `feat: add deterministic execution governance`.

### Task 4: Define the external adapter contract

**Files:**
- Create: `crates/one-core/src/adapter.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/src/adapter.rs`

**Interfaces:**
- Consumes: typed capability requests and approved policy decisions.
- Produces: `ToolAdapter` contract, `AdapterRequest`, `AdapterResult`, `AdapterMetadata`, and adapter compatibility checks.

- [ ] **Step 1: Write failing tests** for adapter identity, declared capability compatibility, platform availability, deterministic selection metadata, and rejection of undeclared capabilities.
- [ ] **Step 2: Run focused tests** and verify failure.
- [ ] **Step 3: Implement the adapter trait and metadata contract** without executing external commands.
- [ ] **Step 4: Test that arbitrary free-form command strings cannot substitute for a typed adapter request.**
- [ ] **Step 5: Run existing workspace tests** and verify PASS.
- [ ] **Step 6: Commit** `feat: define governed external adapter contract`.

### Task 5: Implement deterministic adapter selection and orchestration lifecycle

**Files:**
- Create: `crates/one-core/src/orchestrator.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/tests/orchestration_runtime.rs`

**Interfaces:**
- Consumes: semantic capability request, governance decision, and registered adapters.
- Produces: `Orchestrator`, deterministic adapter selection, and typed lifecycle errors.

- [ ] **Step 1: Write failing integration tests** covering request validation → scope validation → policy decision → adapter selection → invocation boundary.
- [ ] **Step 2: Run the integration tests** and verify failure.
- [ ] **Step 3: Implement the orchestrator** so it cannot invoke an adapter before policy approval.
- [ ] **Step 4: Add deterministic adapter-priority/compatibility selection tests** and reject materially incompatible substitutes.
- [ ] **Step 5: Verify that denied requests never reach adapter invocation.**
- [ ] **Step 6: Run workspace tests and Clippy.**
- [ ] **Step 7: Commit** `feat: add governed tool orchestration runtime`.

### Task 6: Normalize external results and diagnostics

**Files:**
- Modify: `crates/one-core/src/adapter.rs`
- Create: `crates/one-core/src/orchestration_result.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/tests/orchestration_result.rs`

**Interfaces:**
- Consumes: typed adapter output/error values.
- Produces: `NormalizedResult`, `ExecutionEvidence`, `OrchestrationError`, and deterministic diagnostics.

- [ ] **Step 1: Write failing tests** for successful typed output, malformed output, timeout, interrupted execution, uncertain/incomplete results, and adapter failure.
- [ ] **Step 2: Run tests** and verify failure.
- [ ] **Step 3: Implement normalization** that preserves material uncertainty and never fabricates facts.
- [ ] **Step 4: Verify deterministic conversion of the same adapter result into the same One result/diagnostic.**
- [ ] **Step 5: Run workspace tests and Clippy.**
- [ ] **Step 6: Commit** `feat: normalize external tool results`.

### Task 7: Add auditable governance/execution records

**Files:**
- Create: `crates/one-core/src/orchestration_audit.rs`
- Modify: `crates/one-core/src/orchestrator.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/tests/orchestration_audit.rs`

**Interfaces:**
- Consumes: policy decisions, adapter metadata, execution lifecycle, and normalized results.
- Produces: `AuditEvent`, `AuditRecord`, and deterministic audit serialization suitable for the existing artifact model.

- [ ] **Step 1: Write failing tests** for allow/deny/approval decisions, adapter execution, errors, and absence of unverifiable claims.
- [ ] **Step 2: Run tests** and verify failure.
- [ ] **Step 3: Implement immutable execution records** containing only observed/requested fields defined by the spec.
- [ ] **Step 4: Verify audit records distinguish attempted execution from verified external facts.**
- [ ] **Step 5: Run workspace tests and Clippy.**
- [ ] **Step 6: Commit** `feat: add orchestration audit records`.

### Task 8: Integrate the first safe, general-purpose real adapter

**Files:**
- Modify: existing interop/adapter integration files identified by Tasks 4–5.
- Create: `crates/one-core/tests/adapter_real.rs`
- Modify: contract/registry documentation only after executable evidence exists.

**Interfaces:**
- Consumes: the typed adapter/orchestrator contract.
- Produces: one real, locally verifiable adapter demonstrating end-to-end execution without making the adapter command syntax part of One semantics.

- [ ] **Step 1: Select a low-risk adapter with deterministic local behavior** and document its exact capability/input/output contract before coding.
- [ ] **Step 2: Write failing real/sandboxed integration tests.**
- [ ] **Step 3: Implement the adapter using typed arguments and the governed execution boundary.**
- [ ] **Step 4: Verify real execution, normalized output, policy enforcement, and audit records.**
- [ ] **Step 5: Only then update its registry status to Implemented/Verified/Supported as justified by platform evidence.**
- [ ] **Step 6: Commit** `feat: verify first real tool adapter`.

### Task 9: Add localization and human-language equivalence fixtures

**Files:**
- Modify: existing grammar/semantic fixture locations identified from the current conformance layout.
- Create: orchestration-specific conformance fixtures under the existing fixture convention.
- Test: existing conformance runner plus new orchestration tests.

**Interfaces:**
- Consumes: semantic capability IDs from Task 2.
- Produces: localization-equivalence fixtures proving surface language does not change semantics, effects, or authorization requirements.

- [ ] **Step 1: Write failing fixtures** for Indonesian and at least one alternate surface representation of the same semantic operation.
- [ ] **Step 2: Run the conformance suite** and verify the new fixtures fail before implementation/alignment.
- [ ] **Step 3: Integrate the orchestration semantic mapping with the existing localization pipeline.**
- [ ] **Step 4: Add ambiguity and conflicting-intent fixtures.**
- [ ] **Step 5: Run full conformance tests and verify PASS.**
- [ ] **Step 6: Commit** `test: add localized orchestration conformance fixtures`.

### Task 10: Cross-contract documentation and verification integration

**Files:**
- Modify: existing capability, standard-library, network, package/build, verification, and release documents only where the new executable behavior requires synchronization.
- Modify: `tools/check_contract.py` only if existing contract checks cannot express the new invariants.
- Test: repository contract/verification suite.

**Interfaces:**
- Consumes: executable evidence from Tasks 1–9.
- Produces: synchronized specifications, registry status, contract checks, and verification records with no unsupported claims.

- [ ] **Step 1: Write failing contract checks** for the new required orchestration artifacts/invariants where the existing checker lacks coverage.
- [ ] **Step 2: Run the contract checker** and verify the expected failure.
- [ ] **Step 3: Update documentation and checks** to reflect only verified implementation status.
- [ ] **Step 4: Run contract, build, Clippy, workspace tests, real-network tests where applicable, and offline verification.**
- [ ] **Step 5: Review all changed documentation for stale claims, contradictory support statuses, and architecture duplication.**
- [ ] **Step 6: Commit** `docs: integrate orchestration contracts and verification`.

### Task 11: Exact-SHA cross-platform verification and release gate

**Files:**
- Existing canonical verification workflow and mirror files only if required by evidence.
- Existing verification/release records.

**Interfaces:**
- Consumes: the final canonical commit containing Tasks 1–10.
- Produces: fresh exact-SHA Ubuntu/Windows evidence and synchronized verification records.

- [ ] **Step 1: Trigger the existing cross-platform verification against the exact canonical SHA.**
- [ ] **Step 2: Verify Ubuntu build, Clippy, workspace tests, relevant integration tests, contract checks, and offline verification.**
- [ ] **Step 3: Verify Windows performs the same applicable gates.**
- [ ] **Step 4: Verify no documentation or registry claim points to an older source SHA.**
- [ ] **Step 5: Update verification/release artifacts only after fresh evidence exists.**
- [ ] **Step 6: Commit final evidence synchronization** and record the exact canonical SHA.

## Execution Notes

- Tasks 1–7 establish the generic mechanism before broad tool coverage.
- Task 8 deliberately proves the architecture with one real adapter rather than declaring a large external-tool inventory supported prematurely.
- Security/network tooling is represented as a future adapter family under the same typed capability/governance model; registry presence alone does not imply executable support.
- Any newly discovered architecture conflict upgrades the work back to design review before production changes continue.
- Every production change follows Red → verify Red → Green → verify Green → refactor where applicable.
- No production fix is made from a hypothesis without a failing test or other concrete evidence.
- If an adapter's behavior is platform-dependent, platform-specific verification is required before Supported status.

## Completion Gate

The plan is complete only when the final canonical SHA has fresh verification evidence and all required contract/release artifacts agree with that SHA. A plan completion statement must not be based solely on source inspection or local success.
