# Native Compiler Generalization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Generalize native compilation so DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT all have real target-independent native compilation paths through the existing Semantic IR and Gate 1 toolchain boundary.

**Architecture:** Keep `IrProgram`, `IrStatement`, `IrExpr`, and `IrLoopKind` as the sole semantic contract. Replace the current limited native C emission path with one shared generator that can emit scalar data, actions, control flow, function boundaries/calls, and event handlers/dispatch; target backends remain responsible only for target metadata and toolchain/artifact handling.

**Tech Stack:** Rust, Cargo, existing `one-core` Semantic IR, native C emission, existing `TargetSpec`/`ToolchainSpec`, Gate 1 toolchain resolver/invoker, existing native artifact validation, integration tests and fixtures.

**Spec:** `docs/superpowers/specs/2026-10-21-native-compiler-generalization-design.md`

## Global Constraints

- One Semantic IR remains the only semantic contract for native compilation.
- No target-specific semantic fork or backend-specific replacement IR.
- DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT must each have a real native compilation path.
- EVENT uses handler + explicit dispatcher semantics; event declaration does not implicitly execute.
- Existing interpreter behavior remains regression-free.
- Gate 1 toolchain execution boundary remains intact and green.
- Native build success requires artifact existence and validation.
- Failures remain structured and deterministic through `NativeBackendError` or existing Gate 1 errors.
- Do not claim platform execution without actual platform/toolchain evidence.
- No Gate 3 work, firmware flashing, GPIO/HAL, or RTOS semantics.

## Review Focus

- Function definitions must be emitted at file scope rather than nested in `main`; test function declaration/call output and artifact build.
- Event declaration must not execute implicitly; test explicit dispatcher behavior and deterministic handler naming.
- ForEach lowering must preserve iterable/reference semantics rather than silently treating arbitrary expressions as C arrays; test supported iterable shape and reject unsupported types.
- Existing orchestration actions must not be silently converted into unrelated native I/O; test their existing IR category and deterministic native diagnostic.
- Target-specific codegen must not drift semantic ordering; test identical IR across two target generators produces identical semantic C apart from explicitly target-specific toolchain arguments.

---

### Task 1: Establish Gate 2 native compiler contract tests

**Files:**
- Create: `crates/one-core/tests/native_compiler_generalization.rs`
- Modify: `crates/one-core/src/native_backend.rs` only where test-visible interfaces are required
- Test fixtures: existing `crates/one-core/tests/fixtures/toolchain_fixture.rs` and current native backend fixtures

**Interfaces:**
- Consumes: existing `parse()`, `analyze()`, `ir::lower()`, native backend `codegen()`/`build()`, `NativeArtifact`, `NativeBackendError`.
- Produces: failing tests proving each construct family currently lacks native coverage, plus deterministic negative-test contracts used by later tasks.

- [ ] **Step 1: Write failing DATA/ACTION/CONDITION/LOOP/FUNCTION/EVENT tests**
  - Build representative ONE programs through parse → analyze → lower.
  - Assert native source contains the construct's expected C control/function/handler shape.
  - For build tests, assert returned artifact exists and has the expected executable format.

- [ ] **Step 2: Write failing negative tests**
  - Unsupported action/construct → structured `NATIVE-CODEGEN-UNSUPPORTED`.
  - Invalid type → structured `NATIVE-CODEGEN-TYPE`.
  - Target/backend mismatch → structured `NATIVE-TOOLCHAIN-INVALID`.
  - Invalid lowering input → structured deterministic codegen/lowering diagnostic.
  - Invalid/missing artifact → structured `NATIVE-ARTIFACT-MISSING` or artifact validation error.

- [ ] **Step 3: Write deterministic-output tests**
  - Compile identical IR twice and assert byte-identical native source.
  - Compare shared generator output across target backend instances where semantic source is target-independent.

- [ ] **Step 4: Run focused tests and verify RED**
  - Run: `cargo test -p one-core --test native_compiler_generalization --offline -- --nocapture`
  - Expected: FAIL because the current native generator rejects the uncovered construct families or does not emit their required native forms.

- [ ] **Step 5: Commit tests**

```bash
GIT_PAGER=cat git add crates/one-core/tests/native_compiler_generalization.rs && \
GIT_PAGER=cat git commit -m "test: define Gate 2 native compiler contract"
```

### Task 2: Generalize native scalar types, expressions, and variable scopes

**Files:**
- Modify: `crates/one-core/src/native_backend.rs`
- Test: `crates/one-core/tests/native_compiler_generalization.rs`

**Interfaces:**
- Consumes: `IrExpr`, existing `CType` concept, Semantic IR variable/reference semantics.
- Produces: deterministic expression/type emission used by actions, conditions, loops, functions, and events.

- [ ] **Step 1: Add failing tests for binary expressions, references, call expressions, and function-local scopes**
- [ ] **Step 2: Run focused tests and confirm RED**
- [ ] **Step 3: Implement shared typed expression emission and scoped variable lookup without changing `IrExpr`**
- [ ] **Step 4: Reject unsupported/incompatible types with deterministic `NativeBackendError` codes**
- [ ] **Step 5: Run focused tests and confirm GREEN**
- [ ] **Step 6: Commit**

```bash
GIT_PAGER=cat git add crates/one-core/src/native_backend.rs crates/one-core/tests/native_compiler_generalization.rs && \
GIT_PAGER=cat git commit -m "feat: generalize native expression and scope emission"
```

### Task 3: Implement DATA and ACTION native emission

**Files:**
- Modify: `crates/one-core/src/native_backend.rs`
- Test: `crates/one-core/tests/native_compiler_generalization.rs`

**Interfaces:**
- Consumes: Task 2 typed expression/scoping emitter and existing `IrStatement::Declaration`, `Assignment`, `Action`.
- Produces: native DATA/ACTION emission shared by all target backends.

- [ ] **Step 1: Add/strengthen failing DATA/ACTION artifact tests**
- [ ] **Step 2: Implement declaration/assignment/reference emission using shared type rules**
- [ ] **Step 3: Implement the supported native action family with deterministic arity/type diagnostics**
- [ ] **Step 4: Preserve existing orchestration IR instead of silently reinterpreting orchestration actions**
- [ ] **Step 5: Build and validate a native artifact from DATA + ACTION input**
- [ ] **Step 6: Run focused tests and commit**

### Task 4: Implement CONDITION and LOOP control flow

**Files:**
- Modify: `crates/one-core/src/native_backend.rs`
- Test: `crates/one-core/tests/native_compiler_generalization.rs`

**Interfaces:**
- Consumes: shared expression/type emitter; `IrStatement::Conditional`; `IrStatement::Loop`; `IrLoopKind`.
- Produces: target-independent C control-flow emission.

- [ ] **Step 1: Add failing condition/loop tests, including nested control flow**
- [ ] **Step 2: Implement boolean condition emission and `if/else`**
- [ ] **Step 3: Implement `While` emission**
- [ ] **Step 4: Implement only the iterable representation supported by the existing Semantic IR; reject unsupported iterable types deterministically**
- [ ] **Step 5: Build and validate a native artifact containing CONDITION + LOOP**
- [ ] **Step 6: Run focused tests and commit**

### Task 5: Implement FUNCTION boundaries and calls

**Files:**
- Modify: `crates/one-core/src/native_backend.rs`
- Test: `crates/one-core/tests/native_compiler_generalization.rs`

**Interfaces:**
- Consumes: shared expression/scoping emitter; `IrStatement::Function`; `IrExpr::Call`.
- Produces: file-scope C functions, typed parameters/results, native calls, and deterministic function diagnostics.

- [ ] **Step 1: Add failing function-definition/call tests**
- [ ] **Step 2: Implement function collection and file-scope emission**
- [ ] **Step 3: Implement parameter/local scope handling and result emission**
- [ ] **Step 4: Implement `IrExpr::Call` with arity/type validation**
- [ ] **Step 5: Build and validate a native artifact using a function**
- [ ] **Step 6: Run focused tests and commit**

### Task 6: Implement EVENT handler + dispatcher model

**Files:**
- Modify: `crates/one-core/src/native_backend.rs`
- Test: `crates/one-core/tests/native_compiler_generalization.rs`
- Possibly modify: existing native/interpreter integration test files only if required to pin shared event semantics

**Interfaces:**
- Consumes: `IrStatement::Event`, trigger tokens, existing interpreter `dispatch_event` semantics.
- Produces: deterministic file-scope event handlers and explicit dispatcher path; no implicit event execution.

- [ ] **Step 1: Add failing event emission/dispatcher tests**
- [ ] **Step 2: Implement deterministic handler identifiers from trigger metadata**
- [ ] **Step 3: Implement explicit dispatcher function and trigger matching**
- [ ] **Step 4: Prove event declaration alone does not execute the handler**
- [ ] **Step 5: Build and validate a native artifact containing EVENT**
- [ ] **Step 6: Run focused tests and commit**

### Task 7: Route all native targets through the shared generator and Gate 1 toolchain

**Files:**
- Modify: `crates/one-core/src/native_backend.rs`
- Modify: `crates/one-core/src/toolchain.rs` only if existing Gate 1 interfaces expose a proven missing capability
- Test: existing native backend tests plus `native_compiler_generalization.rs`

**Interfaces:**
- Consumes: shared native generator from Tasks 2–6.
- Produces: identical semantic C generation across Linux AArch64, Linux x86_64, Windows x86_64, and Android AArch64, with target-specific differences confined to toolchain invocation and artifact validation.

- [ ] **Step 1: Add cross-target generator equivalence tests**
- [ ] **Step 2: Route every native backend through the shared generator**
- [ ] **Step 3: Ensure target mismatch is rejected before compiler invocation**
- [ ] **Step 4: Ensure Gate 1 resolver/invoker/error behavior remains unchanged**
- [ ] **Step 5: Run Gate 1 focused tests and existing native backend tests**
- [ ] **Step 6: Commit**

### Task 8: Documentation and verification evidence

**Files:**
- Modify: `docs/ARCHITECTURE.md`
- Modify: `docs/verification/toolchain-execution-boundary-2026-10-20.md` to correct its Gate 1 evidence status if required by the audit
- Create: `docs/verification/native-compiler-generalization-2026-10-21.md`
- Modify: this plan only if implementation decisions materially differ from the approved design

**Interfaces:**
- Consumes: verified implementation and test evidence from Tasks 1–7.
- Produces: documentation describing the shared generator, six construct paths, event model, diagnostics, target boundary, and actual verification limitations.

- [ ] **Step 1: Document the Gate 2 native compilation architecture**
- [ ] **Step 2: Record construct-family evidence and negative diagnostics**
- [ ] **Step 3: Record deterministic output evidence and artifact validation**
- [ ] **Step 4: Record any real platform/toolchain limitations without converting them into source claims**
- [ ] **Step 5: Run documentation consistency checks**
- [ ] **Step 6: Commit documentation**

### Task 9: Full verification and final diff audit

**Files:**
- No planned source changes; fix only verified failures discovered by this task.

- [ ] **Step 1: Run formatter verification**
  - `cargo fmt --all -- --check`
- [ ] **Step 2: Run relevant Gate 2 tests**
- [ ] **Step 3: Run Gate 1 focused tests**
- [ ] **Step 4: Run `cargo test -p one-core`**
- [ ] **Step 5: Run `cargo clippy -p one-core --all-targets --offline -- -D warnings`**
- [ ] **Step 6: Run contract check and native artifact verification**
- [ ] **Step 7: Run `GIT_PAGER=cat git diff --check`**
- [ ] **Step 8: Audit final diff and scope against `origin/main`**
- [ ] **Step 9: Verify worktree clean and record exact commit/revision evidence**
- [ ] **Step 10: Commit any final verified fix and rerun affected verification before considering Gate 2 complete**

## Completion Contract

Gate 2 may be marked `GREEN LOCKED 🔒 FINAL` only when all of the following have fresh evidence:

- all six construct families have native compilation paths;
- each family reaches a validated native artifact in supported build environments;
- negative diagnostics are structured and deterministic;
- no semantic fork exists;
- existing interpreter tests remain green;
- Gate 1 focused tests remain green;
- documentation matches implementation and evidence;
- final diff is audited and scoped;
- no unresolved failure remains.
