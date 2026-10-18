# One Target Toolchain Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a deterministic, target-independent `ToolchainSpec` abstraction and compatibility contract without changing One Semantic IR or claiming real cross-compilation support.

**Architecture:** Extend the existing target foundation in `crates/one-core/src/target.rs` with typed toolchain metadata, requested-operation capability, and deterministic compatibility validation. Keep `TargetBackend` authoritative for target support and use a separate compatibility method so toolchain metadata cannot become a semantic/compiler fork. Export the new public types from `one-core` and prove behavior with focused integration tests.

**Tech Stack:** Rust 2024, `one-core`, existing `TargetSpec`/`TargetBackend` model, workspace unit/integration tests, Cargo clippy, contract checker.

**Spec:** `docs/superpowers/specs/2026-10-18-one-target-toolchain-foundation.md`

## Global Constraints

- One Semantic IR remains target-independent.
- Toolchain selection is metadata and validation only; it must not execute external tools.
- No real cross-compilation support claim is permitted in this slice.
- No toolchain installation, executable discovery, network lookup, filesystem probing, hardware driver, device I/O, firmware flashing, MCU/vendor SDK, or RTOS integration is introduced.
- Validation must be deterministic and must not silently substitute a target, runtime, backend, or toolchain.
- Existing `TargetBackend::validate` behavior and validation ordering remain intact.
- Support claims for concrete targets/toolchains require separate evidence.

## Review Focus

- Target identity mismatch: a toolchain for a different architecture/environment must be rejected deterministically — covered by Task 2 integration tests.
- Operation capability mismatch: execute requested from a build-only toolchain must be rejected deterministically — covered by Task 2 integration tests.
- Backend/toolchain identity mismatch: a toolchain declared for another backend must be rejected before deeper compatibility checks — covered by Task 2 integration tests.
- Semantic IR mutation: attaching toolchain metadata must not alter the same lowered IR — covered by Task 2 integration tests.
- Non-concrete reference fixtures: the reference toolchain must remain a descriptive test fixture and must not imply an installed compiler/linker — covered by Task 1 unit tests and Task 3 documentation checks.

---

### Task 1: Define the typed toolchain model and deterministic compatibility contract

**Files:**
- Modify: `crates/one-core/src/target.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/src/target.rs` unit tests

**Interfaces:**
- Consumes: existing `TargetArchitecture`, `TargetEnvironment`, `TargetSpec`, `TargetBackend`, and `RuntimeClass`.
- Produces: public `ToolchainSpec`, `ToolchainTarget`, `LinkerStrategy`, `ArtifactFormat`, `ToolchainCapability`, `ToolchainOperation`, `ToolchainValidationError`, and `ReferenceToolchain` APIs.

- [ ] **Step 1: Write failing unit tests for deterministic toolchain construction**

Add unit tests in `target.rs` for:
- stable identity and target metadata;
- explicit linker strategy and artifact format;
- explicit capability state;
- deterministic equality of two identically constructed reference toolchains.

The tests must assert concrete enum/field values rather than only `Debug` output.

- [ ] **Step 2: Run the focused unit tests and verify RED**

Run:
```bash
cargo test -p one-core target::tests::toolchain -- --nocapture
```
Expected: FAIL because the toolchain types/constructors do not yet exist.

- [ ] **Step 3: Implement the typed model in `crates/one-core/src/target.rs`**

Add these exact conceptual interfaces:
- `ToolchainTarget { triple: String, architecture: TargetArchitecture, environment: TargetEnvironment }`.
- `LinkerStrategy::{PlatformDefault, Named(String)}`.
- `ArtifactFormat::{Executable, StaticLibrary, DynamicLibrary, Object, FirmwareImage, Other(String)}`.
- `ToolchainCapability::{None, Build, Execute, BuildAndExecute}`.
- `ToolchainOperation::{Build, Execute}`.
- `ToolchainSpec { id: String, backend: String, target: ToolchainTarget, linker: LinkerStrategy, artifact: ArtifactFormat, capability: ToolchainCapability }`.
- `ToolchainSpec::new(...)` with all fields explicit and deterministic.
- Accessors for every field needed by validation/tests.
- `ReferenceToolchain::new()` as a zero-sized descriptive fixture using stable metadata compatible with the existing hosted reference target.

Do not add environment discovery, command execution, or path fields.

- [ ] **Step 4: Add deterministic compatibility validation**

Add `ToolchainValidationError` variants for at least:
- backend mismatch;
- target architecture mismatch;
- target environment mismatch;
- unsupported requested operation;
- invalid toolchain/runtime compatibility.

Add a deterministic validation method, owned by `ToolchainSpec`, with the signature:
`validate_for<B: TargetBackend>(&self, target: &TargetSpec, backend: &B, operation: ToolchainOperation) -> Result<(), ToolchainValidationError>`.

Validation order must be:
1. existing backend target validation;
2. toolchain backend identity;
3. toolchain architecture;
4. toolchain environment;
5. requested operation capability;
6. runtime compatibility where the target/backend contract requires it.

Use the existing target/backend contract rather than duplicating backend support rules. Diagnostics must name the expected and actual identity/value deterministically.

- [ ] **Step 5: Add reference-toolchain unit tests for validation order and diagnostics**

Pin:
- identical validation results across repeated calls;
- backend mismatch;
- architecture mismatch;
- environment mismatch;
- build-only vs execute capability mismatch;
- successful compatible hosted build/execute validation;
- runtime incompatibility where applicable.

- [ ] **Step 6: Run the focused unit tests and verify GREEN**

Run:
```bash
cargo test -p one-core target::tests::toolchain -- --nocapture
```
Expected: all matching tests PASS.

- [ ] **Step 7: Export the public API**

Update `crates/one-core/src/lib.rs` so callers can import the new toolchain types through `one_core` alongside the existing target foundation exports.

- [ ] **Step 8: Run formatting/checks only on the changed source files**

Run:
```bash
rustfmt --edition 2024 crates/one-core/src/target.rs crates/one-core/src/lib.rs
```
Do not include unrelated module roots or run a broad formatter traversal.

- [ ] **Step 9: Commit the model and contract**

```bash
git add crates/one-core/src/target.rs crates/one-core/src/lib.rs
git commit -m "feat: add target toolchain foundation"
```

---

### Task 2: Add integration coverage for target/toolchain selection invariants

**Files:**
- Create: `crates/one-core/tests/target_toolchain_foundation.rs`
- Modify: `crates/one-core/src/target.rs` only if a test exposes a contract defect

**Interfaces:**
- Consumes: public `TargetSpec`, `TargetBackend`, `ReferenceTargetBackend`, `ToolchainSpec`, `ReferenceToolchain`, `ToolchainOperation`, and IR pipeline APIs.
- Produces: executable evidence for compatibility, deterministic diagnostics, and semantic IR invariance.

- [ ] **Step 1: Write integration tests before relying on implementation details**

Add tests for:
1. compatible hosted reference target + reference toolchain succeeds for build;
2. compatible hosted reference target + reference toolchain succeeds for execute;
3. target architecture mismatch is deterministic;
4. target environment mismatch is deterministic;
5. backend/toolchain mismatch is deterministic;
6. requested operation not declared by toolchain is deterministic;
7. repeated validation produces equal errors/results;
8. same source lowers to equivalent `IrProgram` before and after toolchain selection metadata.

For the IR test, compile one existing simple source through parse → semantic analysis → lower, clone the resulting IR, perform toolchain validation separately, and assert the IR values remain equal. Do not introduce a target-specific IR type.

- [ ] **Step 2: Run the integration tests to verify RED where appropriate**

Run:
```bash
cargo test -p one-core --test target_toolchain_foundation -- --nocapture
```
Expected before implementation: FAIL due to missing public toolchain API; after Task 1: PASS. If the tests expose a mismatch between the approved spec and Task 1 implementation, fix the implementation/test contract rather than weakening the test.

- [ ] **Step 3: Verify the integration test count and diagnostics**

The focused test binary must contain all eight behaviors above, with no skipped or ignored tests. Confirm mismatch assertions use typed error variants and exact deterministic messages where the contract fixes wording.

- [ ] **Step 4: Run focused clippy and tests**

Run:
```bash
cargo clippy -p one-core --all-targets -- -D warnings
cargo test -p one-core --all-targets
```
Expected: no clippy warnings and zero test failures.

- [ ] **Step 5: Commit integration evidence**

```bash
git add crates/one-core/tests/target_toolchain_foundation.rs
 git commit -m "test: verify target toolchain compatibility"
```

---

### Task 3: Reconcile specification, decisions, and verification documentation

**Files:**
- Modify: `docs/superpowers/specs/2026-10-18-one-target-toolchain-foundation.md`
- Modify: `docs/superpowers/plans/2026-10-18-one-target-toolchain-foundation.md`
- Modify: `docs/DECISIONS.md` only if an architectural decision is required by the implemented contract

**Interfaces:**
- Consumes: implemented public API and test evidence from Tasks 1–2.
- Produces: documentation that distinguishes abstraction readiness from actual cross-compilation support.

- [ ] **Step 1: Re-read the implementation against every spec requirement**

Check that each model field, validation rule, non-goal, and test requirement is represented by code or an explicit test. Remove no requirement merely because it is inconvenient to implement.

- [ ] **Step 2: Reconcile the plan with actual implementation**

Update only factual execution notes: actual test counts, commit identifiers, and any approved interface adjustment discovered during implementation. Do not claim real toolchain support.

- [ ] **Step 3: Add an ADR only if implementation introduces a durable architecture decision not already covered by existing ADRs**

If no new durable decision is needed, leave `docs/DECISIONS.md` unchanged. Do not add a redundant ADR solely for documentation volume.

- [ ] **Step 4: Run repository contract validation**

Run:
```bash
python tools/check_contract.py
```
Expected:
`CONTRACT CHECK: PASS`

- [ ] **Step 5: Commit documentation reconciliation**

```bash
git add docs/superpowers/specs/2026-10-18-one-target-toolchain-foundation.md docs/superpowers/plans/2026-10-18-one-target-toolchain-foundation.md docs/DECISIONS.md
git commit -m "docs: reconcile target toolchain foundation"
```

---

### Task 4: Full verification and evidence gate

**Files:**
- No intended source changes. Any unexpected modification must be investigated before proceeding.

**Interfaces:**
- Consumes: complete branch from Tasks 1–3.
- Produces: fresh executable evidence suitable for review/merge; no support claim beyond the implemented foundation.

- [ ] **Step 1: Verify worktree and exact branch head**

Run:
```bash
git status --short
git rev-parse HEAD
git log -1 --oneline
```
Expected: only intentional committed changes and a known HEAD.

- [ ] **Step 2: Run workspace build**

Run:
```bash
cargo build --workspace --all-targets
```
Expected: success with exit code 0.

- [ ] **Step 3: Run workspace clippy**

Run:
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
Expected: success with no warnings promoted to errors.

- [ ] **Step 4: Run the complete workspace test suite**

Run:
```bash
cargo test --workspace --all-targets
```
Expected: zero failures.

- [ ] **Step 5: Run contract and whitespace checks**

Run:
```bash
python tools/check_contract.py
git diff --check
```
Expected: `CONTRACT CHECK: PASS` and no diff-check output/errors.

- [ ] **Step 6: Verify deterministic artifacts and existing vertical slices**

Run the repository's existing deterministic-artifact and vertical verification commands already required by the project's verification workflow. Record exact command/output evidence; do not infer support from compilation alone.

- [ ] **Step 7: Verify exact SHA and clean status**

Run the project's exact-SHA verification script against the intended branch head, then:
```bash
git status --short
```
Expected: exact expected SHA and clean worktree.

- [ ] **Step 8: Perform final static review**

Review the diff specifically for:
- accidental host/toolchain execution;
- environment probing;
- semantic IR specialization;
- duplicated backend support logic;
- nondeterministic diagnostics;
- concrete hardware/toolchain support claims without evidence;
- unrelated formatting or file churn.

- [ ] **Step 9: Commit only if verification documentation changed during evidence capture**

If documentation needs final evidence updates, commit them separately with a verification-focused message. Otherwise do not create a no-op commit.

---

## Completion Gate

Do not describe this slice as “cross-compilation supported.” Completion means only that One has a deterministic, typed abstraction and validation boundary for toolchain metadata, backed by executable tests and repository-wide verification.

The next independent slice may evaluate an actual cross-compilation/toolchain adapter. That later work requires its own target/toolchain-specific evidence and must not be inferred from this foundation.
