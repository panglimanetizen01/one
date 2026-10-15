# One Network Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first independently testable One Network vertical slice: a One-owned, deterministic network semantic model with explicit capabilities, protocol-family mapping, diagnostics, IR representation, and a reference execution boundary, without making external protocol names part of ordinary source semantics.

**Architecture:** Extend the existing deterministic pipeline with a focused network semantic domain. User-facing network intent is represented as typed One Network operations, lowered into deterministic network IR, checked against explicit capabilities, and only then delegated to a platform/backend adapter. External protocols remain implementation targets in a machine-readable registry; they do not define One semantics.

**Tech Stack:** Rust workspace, `one-core`, existing AST/semantic/IR/diagnostic/capability infrastructure, Rust unit/integration tests, offline verification.

**Spec:** `docs/superpowers/specs/2026-10-15-one-end-to-end-network-platform-design.md`

## Global Constraints

- The canonical semantic pipeline remains `source → lexer → parser → AST → name/scope analysis → type/semantic analysis → CSIR → reference interpreter`.
- Each stage remains deterministic.
- LLMs and network services are never semantic authorities.
- Ambiguity is rejected rather than guessed.
- Network and system operations are capabilities, not unrestricted language primitives.
- Ordinary application networking must not silently grant administrative network authority.
- The protocol capability registry is a compatibility inventory, not proof that every listed protocol is implemented.
- Support status must distinguish `Specified`, `Designed`, `Implemented`, `Verified`, and `Supported`.
- The exact grammar and vocabulary remain subject to the formal language specification; examples in the architecture spec are illustrative, not grammar contracts.
- No external networking dependency is required merely to execute the semantic tests or reference interpreter tests.
- Existing One v0.1 behavior and deterministic artifacts must remain unchanged unless a test explicitly proves the new network behavior.

## Review Focus

- Ambiguous network requests must fail deterministically instead of selecting a protocol or transport implicitly. Test in `network::tests::ambiguous_operation_is_rejected`.
- A network operation without the required capability must fail before backend execution. Test in `network::tests::capability_denial_precedes_backend`.
- Protocol names must not become semantic keywords merely because they appear in the registry. Test in `network::registry_tests::registry_entries_do_not_define_language_actions`.
- Human-readable diagnostics must preserve deterministic technical detail without guessing. Test in `network::diagnostic_tests::network_failure_diagnostic_has_stable_layers`.
- IPv4/IPv6, reliable/unreliable transport, secure/insecure mode, and protocol-family selection must remain explicit typed properties rather than stringly-typed options. Test in `network::tests::typed_constraints_round_trip_without_loss`.

---

## File Map

- **Create:** `crates/one-core/src/network.rs` — One Network semantic types, typed operations, capability requirements, deterministic validation, protocol registry model, and network-specific diagnostics helpers.
- **Modify:** `crates/one-core/src/lib.rs` — export the `network` module.
- **Modify:** `crates/one-core/src/capability.rs` — add granular network capabilities while preserving the existing coarse `Capability::Network` compatibility behavior.
- **Modify:** `crates/one-core/src/ir.rs` — add typed network IR statements/operations and deterministic canonical rendering.
- **Modify:** `crates/one-core/src/semantic.rs` — expose a deterministic lowering/validation boundary for network operations without changing unrelated semantic rules.
- **Modify:** `crates/one-core/src/interpreter.rs` — enforce network capabilities at the reference execution boundary and return deterministic network diagnostics; no real network access is performed by the reference interpreter.
- **Create:** `crates/one-core/tests/network_foundation.rs` — end-to-end semantic/IR/capability tests for the first network vertical slice.
- **Modify:** `docs/spec/one-v0.1-language-invariants.md` only if a new invariant must be recorded; otherwise do not touch the existing v0.1 language contract.
- **Create:** `docs/spec/one-network-foundation.md` — normative network foundation contract, limited to the implemented slice.
- **Modify:** `README.md` only after implementation is verified, to distinguish the network foundation as implemented/verified rather than implying broad protocol support.
- **Modify:** `CHANGELOG.md` only after implementation is verified, recording the new network foundation slice and its explicit limits.

---

### Task 1: Establish the typed One Network semantic model

**Files:**
- Create: `crates/one-core/src/network.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/src/network.rs` unit tests

**Interfaces:**
- Produces `NetworkAddressKind`, `NetworkTransport`, `NetworkSecurity`, `NetworkOperation`, `NetworkTarget`, `NetworkConstraints`, and `NetworkProtocolFamily` as typed semantic values.
- Produces `NetworkOperation::Connect { target, constraints }`, `NetworkOperation::Send { connection, data }`, and `NetworkOperation::Receive { connection }` as the minimum first-slice operation set.
- Produces deterministic validation returning a `DiagnosticBundle` when a request is incomplete or contradictory.

- [ ] **Step 1: Write failing unit tests**

Add tests named exactly:
- `typed_constraints_round_trip_without_loss`
- `ambiguous_operation_is_rejected`
- `invalid_timeout_is_rejected`
- `secure_connection_requires_explicit_security_mode_when_required`

The tests must assert typed equality and stable diagnostic codes/messages, not string matching on debug output.

- [ ] **Step 2: Run the focused tests and verify failure**

Run: `cargo test -p one-core network::tests -- --nocapture`

Expected: FAIL because the network semantic types and operations do not yet exist.

- [ ] **Step 3: Implement the minimum semantic model**

Define typed enums/structs with deterministic `Debug`, `Clone`, `PartialEq`, and `Eq` behavior. Keep endpoint names/addresses as data, not protocol-specific syntax. Represent timeout as a validated duration type and transport/security/address constraints as enums rather than free-form strings.

- [ ] **Step 4: Run focused tests and verify pass**

Run: `cargo test -p one-core network::tests -- --nocapture`

Expected: PASS.

- [ ] **Step 5: Commit**

Commit message: `feat: add typed One Network semantic foundation`

---

### Task 2: Integrate granular network capabilities

**Files:**
- Modify: `crates/one-core/src/capability.rs`
- Modify: `crates/one-core/src/network.rs`
- Test: capability unit tests

**Interfaces:**
- Adds granular capabilities: `NetworkConnect`, `NetworkListen`, `NetworkDiscover`, `NetworkAdmin`.
- `Capability::Network` remains accepted as the coarse compatibility grant for ordinary network operations, but `NetworkAdmin` is never implied by it.
- `NetworkOperation::required_capabilities()` returns the minimum capability set required for the operation.

- [ ] **Step 1: Write failing tests**

Add tests named:
- `ordinary_connect_requires_network_connect`
- `network_discovery_requires_network_discover`
- `network_admin_is_not_implied_by_network_connect`
- `coarse_network_capability_preserves_existing_connect_compatibility`

- [ ] **Step 2: Run tests and verify failure**

Run: `cargo test -p one-core capability -- --nocapture`

Expected: FAIL because granular capabilities and operation requirements do not exist.

- [ ] **Step 3: Implement capability requirements**

Extend `Capability` with the four granular variants. Add a deterministic capability requirement check used by network operations. Do not alter filesystem/process/native semantics.

- [ ] **Step 4: Run focused and full core tests**

Run: `cargo test -p one-core capability -- --nocapture`

Then: `cargo test -p one-core`

Expected: PASS for both.

- [ ] **Step 5: Commit**

Commit message: `feat: enforce granular One Network capabilities`

---

### Task 3: Add the protocol capability registry

**Files:**
- Modify: `crates/one-core/src/network.rs`
- Test: `crates/one-core/src/network.rs` registry tests

**Interfaces:**
- Produces `ProtocolCapability` and `ProtocolRegistry`.
- `ProtocolRegistry::builtin()` returns a deterministic registry of protocol families required by the first slice: DNS/name resolution, TCP, UDP, TLS, HTTP, and QUIC/HTTP3 as registry metadata only.
- Registry entries contain external protocol names, semantic capabilities, and implementation status; they do not become parser keywords or language actions.

- [ ] **Step 1: Write failing registry tests**

Add:
- `registry_is_deterministically_ordered`
- `registry_entries_do_not_define_language_actions`
- `unsupported_protocol_is_not_reported_as_supported`

- [ ] **Step 2: Run tests and verify failure**

Run: `cargo test -p one-core network::registry_tests -- --nocapture`

Expected: FAIL because the registry does not exist.

- [ ] **Step 3: Implement registry metadata**

Represent implementation status explicitly. The first slice may mark protocol entries `Specified` or `Designed`; do not mark an adapter `Implemented`, `Verified`, or `Supported` without corresponding code/tests/evidence.

- [ ] **Step 4: Run tests and verify pass**

Run: `cargo test -p one-core network::registry_tests -- --nocapture`

Expected: PASS.

- [ ] **Step 5: Commit**

Commit message: `feat: add deterministic network protocol registry`

---

### Task 4: Add typed network IR and deterministic canonical rendering

**Files:**
- Modify: `crates/one-core/src/ir.rs`
- Modify: `crates/one-core/src/network.rs`
- Test: `crates/one-core/src/ir.rs` tests

**Interfaces:**
- Adds `IrNetworkOperation` and `IrStatement::Network`.
- `IrNetworkOperation` preserves target, address kind, transport, security, timeout, and payload semantics without collapsing them into opaque strings.
- `IrProgram::canonical_text()` renders network IR deterministically and stably.

- [ ] **Step 1: Write failing canonicalization tests**

Add:
- `network_ir_canonical_text_is_stable`
- `network_ir_preserves_security_and_transport_constraints`

- [ ] **Step 2: Run tests and verify failure**

Run: `cargo test -p one-core ir::tests -- --nocapture`

Expected: FAIL because network IR variants do not exist.

- [ ] **Step 3: Implement typed network IR**

Lower the network semantic operation into IR without losing typed constraints. Keep canonical output independent of hash-map iteration or platform ordering.

- [ ] **Step 4: Run tests and verify pass**

Run: `cargo test -p one-core ir::tests -- --nocapture`

Expected: PASS.

- [ ] **Step 5: Commit**

Commit message: `feat: add deterministic network IR`

---

### Task 5: Add the reference interpreter network boundary

**Files:**
- Modify: `crates/one-core/src/interpreter.rs`
- Modify: `crates/one-core/src/network.rs`
- Test: `crates/one-core/tests/network_foundation.rs`

**Interfaces:**
- Produces a reference-interpreter boundary that accepts an `IrNetworkOperation` plus `CapabilitySet` and either returns a deterministic execution result or a capability/validation diagnostic.
- The reference interpreter must not perform real external network I/O.

- [ ] **Step 1: Write failing execution-boundary tests**

Add:
- `capability_denial_precedes_backend`
- `network_operation_with_granted_capability_reaches_reference_boundary`
- `reference_interpreter_does_not_perform_external_network_io`

- [ ] **Step 2: Run tests and verify failure**

Run: `cargo test -p one-core --test network_foundation -- --nocapture`

Expected: FAIL because the interpreter has no network boundary.

- [ ] **Step 3: Implement deterministic boundary**

Validate operation, check required capabilities, and return a typed reference result describing what would be delegated. Do not open sockets, resolve DNS, or contact a remote endpoint in the reference interpreter.

- [ ] **Step 4: Run tests and verify pass**

Run: `cargo test -p one-core --test network_foundation -- --nocapture`

Expected: PASS.

- [ ] **Step 5: Commit**

Commit message: `feat: add reference interpreter network boundary`

---

### Task 6: Add deterministic human-readable network diagnostics

**Files:**
- Modify: `crates/one-core/src/network.rs`
- Modify: `crates/one-core/src/diagnostic.rs` only if the existing diagnostic model requires a new stable code category
- Test: `crates/one-core/tests/network_foundation.rs`

**Interfaces:**
- Produces stable network diagnostic codes for invalid network intent, capability denial, unsupported protocol adapter, name-resolution failure, connection failure, and identity-verification failure.
- Diagnostic rendering exposes a human explanation plus deterministic technical layers; it never invents a cause that the runtime did not provide.

- [ ] **Step 1: Write failing diagnostic tests**

Add `network_failure_diagnostic_has_stable_layers` and assert stable code, user-facing explanation, and technical stage fields.

- [ ] **Step 2: Run test and verify failure**

Run: `cargo test -p one-core --test network_foundation network_failure_diagnostic_has_stable_layers -- --nocapture`

Expected: FAIL because network-specific diagnostics are not implemented.

- [ ] **Step 3: Implement diagnostics**

Keep technical causes structured. The human-readable text is derived directly from those structured causes. No LLM or heuristic cause selection is permitted.

- [ ] **Step 4: Run focused and full core tests**

Run: `cargo test -p one-core --test network_foundation -- --nocapture`

Then: `cargo test -p one-core`

Expected: PASS.

- [ ] **Step 5: Commit**

Commit message: `feat: add deterministic One Network diagnostics`

---

### Task 7: Publish the normative network foundation contract

**Files:**
- Create: `docs/spec/one-network-foundation.md`
- Test/validation: `tools/check_contract.py` if the contract index requires registration

**Interfaces:**
- Documents only the implemented first slice: typed network semantics, capabilities, registry status, network IR, reference boundary, and diagnostics.
- Explicitly lists unsupported real-I/O adapters and protocol implementation limits.

- [ ] **Step 1: Write the contract document**

Include exact support states and examples as illustrative syntax only unless grammar support has been implemented.

- [ ] **Step 2: Validate contract artifacts**

Run: `python3 tools/check_contract.py`

Expected: PASS.

- [ ] **Step 3: Commit**

Commit message: `docs: define One Network foundation contract`

---

### Task 8: Add full offline regression coverage and status alignment

**Files:**
- Create/modify: `crates/one-core/tests/network_foundation.rs`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `docs/verification/one-v0.1-implementation-status.md`
- Modify: `tools/verify_offline.sh` only if the network tests require an explicit invocation beyond `cargo test --workspace --all-targets`
- Modify: `tools/verify_offline.ps1` only if the same is required on Windows

**Interfaces:**
- No new runtime API; this task proves the first slice is deterministic and does not claim broad network support.

- [ ] **Step 1: Add end-to-end semantic fixtures**

Cover connect, send, receive, capability denial, contradictory constraints, deterministic registry ordering, and canonical IR output.

- [ ] **Step 2: Run the complete offline verifier on Linux**

Run: `CARGO_TARGET_DIR="$HOME/.one-target" bash tools/verify_offline.sh`

Expected: `OFFLINE VERIFICATION: PASS`.

- [ ] **Step 3: Run the Windows offline verifier in the independent Windows verification environment**

Run: `./tools/verify_offline.ps1`

Expected: `OFFLINE VERIFICATION: PASS`.

- [ ] **Step 4: Update status documents only from evidence**

Mark only the implemented first-slice capabilities as `Implemented`/`Verified`. Leave real protocol adapters and full network-stack support as future/registry inventory unless evidence exists.

- [ ] **Step 5: Commit**

Commit message: `test: verify One Network foundation end to end`

---

## Execution Order

Tasks 1–3 establish the semantic and capability boundary. Task 4 consumes those types to produce deterministic IR. Task 5 consumes the IR and capabilities to establish the reference execution boundary. Task 6 consumes the structured failure model for diagnostics. Task 7 publishes the normative contract only after code exists. Task 8 is the release gate and documentation alignment.

## Completion Gate

The first One Network foundation slice is complete only when all of the following are true:

1. Focused Rust tests pass.
2. `cargo test --workspace --all-targets` passes.
3. `cargo clippy --workspace --all-targets -- -D warnings` passes.
4. `python3 tools/check_contract.py` passes.
5. Linux `tools/verify_offline.sh` passes.
6. Windows `tools/verify_offline.ps1` passes in the independent verification environment.
7. No real network I/O occurs in the reference interpreter tests.
8. No protocol registry entry is labeled supported without implementation evidence.
9. Existing v0.1 deterministic CLI/artifact tests remain green.
10. Only after all gates pass may README/CHANGELOG/status documents describe the slice as verified.

## Follow-on Plans

This plan deliberately does not attempt the entire end-to-end platform in one change set. After this foundation is verified, create separate independently testable plans for:

1. One Network real client/runtime adapters.
2. One UI/UX and design semantic model.
3. One web/mobile/desktop application runtime.
4. One backend/service model.
5. One data/database model and adapters.
6. One test/QA language and runner.
7. One build/package/deployment system.
8. Advanced network engineering, physical/link, industrial, storage, mesh, P2P, and telemetry adapters.
9. IDE/designer/tooling.
10. Interoperability and platform backend expansion.
