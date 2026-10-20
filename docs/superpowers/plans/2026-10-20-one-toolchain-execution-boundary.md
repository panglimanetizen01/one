# ONE Toolchain Execution Boundary Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans (current session) to implement this plan task-by-task. Steps use checkbox syntax for tracking.

**Goal:** Introduce a deterministic, testable execution boundary between `ToolchainSpec` and compiler/linker processes while preserving existing target and native-backend contracts.

**Architecture:** Keep `TargetSpec`/`ToolchainSpec` validation pure. Add a focused resolver/invoker layer that turns an already validated toolchain specification into explicit process commands, executes them without a shell, and returns structured failures. Refactor only the smallest existing native-backend seam needed to prove the boundary is useful; do not redo the existing cross-linker resolver.

**Tech Stack:** Rust, `std::process::Command`, existing ONE target/toolchain types, Cargo tests.

**Spec:** `docs/superpowers/specs/2026-10-20-one-toolchain-execution-boundary.md`

## Global Constraints

- `ToolchainSpec::validate_for(...)` remains deterministic and performs no host probing.
- Process execution uses structured argument vectors; no shell interpolation.
- Host executable identity and target identity remain separate.
- No package installation, SDK download, network discovery, firmware flashing, GPIO/HAL/RTOS/MCU implementation.
- Existing native backend behavior and historical cross-linker resolver behavior must remain intact.
- Production code follows TDD: a new behavior must first have a failing test.

## Review Focus

1. Shell metacharacters or whitespace in tool paths/arguments must never become shell syntax — test invocation with a fixture path containing spaces and arguments passed as separate values.
2. Missing compiler/linker must produce a stable resolution error rather than a panic — test an explicit nonexistent executable.
3. Non-zero process exit must remain distinguishable from process-start failure — test a fixture process that exits unsuccessfully.
4. A toolchain target mismatch must be rejected before any process is started — test that an incompatible `ToolchainSpec` cannot reach the invoker.
5. Successful execution without the expected artifact must be reported as artifact failure — test the post-invocation artifact contract.

---

### Task 1: Add the execution-boundary contract and error model

**Files:**
- Create or modify: `crates/one-core/src/toolchain.rs` (or the smallest equivalent focused module selected from the existing module layout)
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/tests/toolchain_execution.rs` or module-local tests if that is the established pattern

**Interfaces:**
- Consumes: existing `ToolchainSpec`, `ToolchainOperation`, `ToolchainValidationError`.
- Produces: a resolved toolchain representation, structured invocation command/output types, and a stable execution error enum.

- [ ] **Step 1: Write the failing tests**
  - Test that an explicit compiler executable and argument vector are preserved without shell parsing.
  - Test that a missing executable is reported as a resolution/start failure.
  - Test that a non-zero exit is reported separately from start failure.

- [ ] **Step 2: Run the focused tests to verify RED**
  - Run: `cargo test -p one-core --test toolchain_execution -- --nocapture`
  - Expected: compilation/test failure because the execution-boundary API is not implemented.

- [ ] **Step 3: Implement the minimum contract**
  - Keep command construction data-only.
  - Use `std::process::Command` only inside the concrete invoker.
  - Preserve stdout/stderr/status in the invocation result where needed for diagnostics.

- [ ] **Step 4: Run the focused tests to verify GREEN**
  - Run the same command.
  - Expected: all new execution-boundary tests pass.

- [ ] **Step 5: Commit**
  - Commit: `feat: add toolchain execution boundary`

### Task 2: Add fixture resolver/invoker tests without external toolchains

**Files:**
- Modify: `crates/one-core/tests/toolchain_execution.rs`
- Modify: execution module from Task 1 only if required

**Interfaces:**
- Consumes: Task 1 resolver/invoker contracts.
- Produces: deterministic fixture-based proof that process invocation does not depend on installed cross-compilers.

- [ ] **Step 1: Write the failing fixture tests**
  - Test a fixture executable that records separate arguments and exits successfully.
  - Test a fixture executable that exits non-zero and emits diagnostics.
  - Test that a path containing spaces remains one executable path.

- [ ] **Step 2: Run focused tests and verify RED**
  - Run: `cargo test -p one-core --test toolchain_execution -- --nocapture`
  - Expected: the new fixture assertions fail before the implementation is extended.

- [ ] **Step 3: Implement the minimum fixture-safe execution behavior**
  - Reuse the same structured command/invoker path; do not add a second execution mechanism for tests.

- [ ] **Step 4: Run focused tests and verify GREEN**
  - Expected: all fixture invocation tests pass.

- [ ] **Step 5: Commit**
  - Commit: `test: cover toolchain process execution boundaries`

### Task 3: Integrate the boundary at the smallest native-backend seam

**Files:**
- Modify: `crates/one-core/src/native_backend.rs`
- Modify: execution module from Task 1 only as necessary
- Modify: existing native backend tests only where behavior is preserved

**Interfaces:**
- Consumes: Task 1 execution boundary and existing `ToolchainSpec`/target validation.
- Produces: native backend process execution routed through the common boundary without reimplementing the existing linker resolver.

- [ ] **Step 1: Add the regression test first**
  - Pin one existing backend build path to the common invocation boundary while preserving its target triple, linker selection, artifact format, and error behavior.

- [ ] **Step 2: Run the targeted regression test and verify RED**
  - Run the smallest existing native-backend test that proves the new seam is not currently used.

- [ ] **Step 3: Refactor minimally**
  - Delegate process invocation only; leave target-specific source generation and already-proven linker resolution in place.
  - Do not alter the historical x86_64 Linux cross-linker resolver.

- [ ] **Step 4: Run native backend regression tests and verify GREEN**
  - Run the affected one-core native backend test targets.
  - Expected: existing Linux/Windows backend behavior remains unchanged.

- [ ] **Step 5: Commit**
  - Commit: `refactor(native): route backend process execution through toolchain boundary`

### Task 4: Full verification and evidence update

**Files:**
- Modify: `docs/verification/one-v0.1-implementation-status.md` only if new evidence changes a documented boundary

- [ ] **Step 1: Run formatting/checks**
  - Run: `cargo fmt --all -- --check`
  - Run: `git diff --check`

- [ ] **Step 2: Run focused and workspace tests**
  - Run: `cargo test -p one-core --test toolchain_execution`
  - Run: `cargo test --workspace`
  - Expected: all tests pass except any already documented environment-dependent linker test if the verification host still lacks that linker.

- [ ] **Step 3: Run Clippy**
  - Run: `cargo clippy --workspace --all-targets -- -D warnings`
  - Expected: PASS with no warnings.

- [ ] **Step 4: Inspect the final diff and evidence boundaries**
  - Confirm no unrelated files changed.
  - Confirm no documentation claims stronger than the executed evidence.

- [ ] **Step 5: Commit documentation if needed**
  - Commit: `docs: record toolchain execution boundary evidence`

- [ ] **Step 6: Final branch verification**
  - Record exact HEAD, diff stat, test results, Clippy result, and any environment-only limitation.
