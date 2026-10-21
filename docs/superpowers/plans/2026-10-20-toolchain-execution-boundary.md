# Gate 1 — Toolchain Execution Boundary Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox syntax for tracking.

**Goal:** Implement a deterministic, shell-free toolchain resolution and process execution boundary with structured failures and artifact validation, without changing One language semantics.

**Architecture:** Keep target/toolchain compatibility in `target.rs`; add a focused `toolchain.rs` execution boundary containing command data, resolver/invoker traits, a path resolver, a `std::process::Command` invoker, deterministic command construction, and high-level build execution. Native backends may adopt the boundary only where required by existing contracts; no parser, analyzer, or IR changes are allowed.

**Tech Stack:** Rust 2024, `std::process::Command`, `std::ffi::OsString`, `std::path::PathBuf`, existing `TargetSpec`/`ToolchainSpec`/`TargetBackend` contracts, integration tests.

**Spec:** `docs/superpowers/specs/2026-10-20-toolchain-execution-boundary-design.md`

## Global Constraints

- Preserve the existing One Source → Parser → Analyzer → One Semantic IR pipeline.
- Use `ToolchainSpec::validate_for` before external execution.
- Execute commands without shell parsing.
- Do not add dependencies unless existing workspace APIs cannot satisfy the boundary.
- Do not add toolchain installation/discovery/network probing.
- Do not add GPIO/HAL/RTOS/firmware flashing.
- Do not claim platform support beyond executed evidence.

## Review Focus

- Arguments containing spaces/quotes/metacharacters must remain discrete arguments — `command_preserves_executable_and_arguments_without_shell_parsing`.
- Missing executables must be classified separately from generic process-start errors — `missing_executable_is_toolchain_not_found` and `process_start_failure_is_distinct`.
- Non-zero process completion must retain status/stdout/stderr and remain distinct from start failure — `non_zero_exit_is_distinct_from_start_failure`.
- Target mismatch must prevent resolver/invoker execution — `target_mismatch_is_rejected_before_execution`.
- A successful process without a freshly produced artifact must fail, while a pre-existing output path must be rejected before execution — `successful_execution_without_artifact_is_failure`, `successful_execution_returns_artifact`, and `high_level_build_rejects_preexisting_artifact_to_prevent_stale_success`.
- A failed invocation must not leave a regular partial output artifact — `failed_toolchain_removes_partial_artifact`.

---

### Task 1: Establish RED regression boundary

**Files:**
- Create: `crates/one-core/tests/toolchain_execution_boundary.rs`

**Interfaces:**
- Consumes: existing `TargetSpec`, `ToolchainSpec`, `ReferenceTargetBackend`, and test-only filesystem/process primitives.
- Produces: failing tests defining the Gate 1 public boundary.

- [x] **Step 1: Write the failing tests**
  - Add tests for command argument preservation, missing executable, process-start failure, non-zero exit, target mismatch before execution, missing artifact, successful artifact, and high-level resolver+invoker+artifact flow.
- [ ] **Step 2: Run the Gate 1 test target and verify RED**
  - Run: `cargo test -p one-core --test toolchain_execution_boundary`
  - Expected: compile failure because the new `toolchain` boundary does not yet exist.

### Task 2: Implement command, resolver, invoker, and structured errors

**Files:**
- Create: `crates/one-core/src/toolchain.rs`
- Modify: `crates/one-core/src/lib.rs`

**Interfaces:**
- Produces `ToolchainCommand`, `ToolchainInvocation`, `ToolchainExecutionError`, `ToolchainResolver`, `PathToolchainResolver`, `ToolchainInvoker`, `CommandToolchainInvoker`.

- [x] **Step 1: Implement `ToolchainCommand` and deterministic command construction**
  - Preserve executable and ordered `OsString` arguments.
  - Construct `--target=<triple>`, source, `-o`, output as discrete arguments.
- [x] **Step 2: Implement `ToolchainExecutionError`**
  - Include `ToolchainNotFound`, `ProcessStart`, `NonZeroExit`, `ArtifactMissing`, and `Validation` variants with diagnostic data needed by tests.
- [x] **Step 3: Implement `ToolchainResolver` and `PathToolchainResolver`**
  - Resolve a configured executable path for the requested `ToolchainSpec` without shell/path searching side effects.
- [x] **Step 4: Implement `ToolchainInvoker` and `CommandToolchainInvoker`**
  - Use `std::process::Command::new(...).args(...)` and `output()`; never invoke a shell.
  - Map `NotFound` separately from other process-start errors and retain status/stdout/stderr for completed processes.
- [x] **Step 5: Export the public boundary from `one_core`**
  - Add `pub mod toolchain` and the selected public re-exports in `lib.rs`.
- [ ] **Step 6: Run Gate 1 tests**
  - Run: `cargo test -p one-core --test toolchain_execution_boundary`
  - Expected: all Gate 1 tests pass.

### Task 3: Implement high-level execution and compatibility/artifact validation

**Files:**
- Modify: `crates/one-core/src/toolchain.rs`
- Modify: `crates/one-core/tests/toolchain_execution_boundary.rs`

**Interfaces:**
- Produces `execute_toolchain_build` and fake/test resolver+invoker fixtures proving the high-level boundary composition.

- [x] **Step 1: Add high-level composition regression assertions**
  - Prove target validation occurs before resolver/invoker calls.
  - Prove successful invocation still fails on missing artifact and that a pre-existing output path is rejected before resolver/invoker calls.
  - Prove a fresh artifact created by the current invocation returns `NativeArtifact`.
  - Prove a non-zero toolchain that writes a partial output does not leave that file behind.
- [x] **Step 2: Implement `execute_toolchain_build`**
  - Validate target/toolchain compatibility before resolving or invoking.
  - Reject an already-existing output path before resolution so stale output cannot create false success.
  - Resolve and invoke the command; reject non-zero `ToolchainInvocation` status even for custom invokers, then require the fresh output to be a regular file.
  - Return `NativeArtifact { path, format }` on success.
- [ ] **Step 3: Run focused tests**
  - Run: `cargo test -p one-core --test toolchain_execution_boundary`
  - Expected: all tests pass.

### Task 4: Full GREEN, Clippy, repository verification

**Files:**
- Modify only files already changed if failures require fixes.

- [ ] **Step 1: Format check**
  - Run: `cargo fmt --check`
- [ ] **Step 2: Full one-core test suite**
  - Run: `cargo test -p one-core`
- [ ] **Step 3: Workspace checks required by protocol**
  - Run: `cargo check --workspace --all-targets`
  - Run: `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] **Step 4: Contract and diff verification**
  - Run: `python tools/check_contract.py`
  - Run: `git diff --check`
  - Inspect final diff against the Gate 1 scope and confirm no semantic-model changes.
- [ ] **Step 5: Hosted CI evidence**
  - Inspect workflow jobs/steps for the final revision. Classify zero-step infrastructure failures as verification insufficiency, not source failure.

### Task 5: Documentation consistency and final evidence

**Files:**
- Modify: `docs/ARCHITECTURE.md`
- Modify: `docs/verification/toolchain-execution-boundary-2026-10-20.md`

- [x] **Step 1: Document the execution boundary**
  - Add the resolver → command → invoker → artifact-validation boundary, shell-free execution rule, and error classes to architecture documentation without expanding platform claims.
- [ ] **Step 2: Record fresh verification evidence**
  - Record exact revision, commands, test counts/results, clippy, contract check, diff check, and CI classification.
- [ ] **Step 3: Re-run documentation-sensitive verification on the final revision**
  - Repeat required checks after documentation changes so the evidence applies to the final head.

## Completion Gate

Gate 1 may be reported `GREEN LOCKED 🔒 FINAL` only when every acceptance criterion has direct evidence, focused and full tests pass, Clippy is clean, contract/diff checks pass, documentation matches the final revision, and any hosted CI failure is either green or explicitly proven infrastructure-only with sufficient job/step evidence.
