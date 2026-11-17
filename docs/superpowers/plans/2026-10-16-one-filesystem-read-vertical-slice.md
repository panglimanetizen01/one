# One Filesystem Read Vertical Slice Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Expose one governed filesystem-read operation from One source through the existing generic orchestration runtime and CLI.

**Architecture:** Reuse the existing parser/AST/semantic/IR pipeline, capability request validation, policy/scope gate, deterministic adapter registry, normalized result, audit path, interpreter, and CLI. Add only the minimum generic IR/interpreter adapter-selection information required by evidence.

**Tech Stack:** Rust workspace, One lexer/parser/semantic/IR/interpreter, CircleCI canonical Ubuntu/Windows verification (supersedes the original GitHub Actions verification selection).

**Spec:** `docs/superpowers/specs/2026-10-16-one-filesystem-vertical-slice.md`

## Global Constraints

- GitHub canonical repository is the source of truth.
- Filesystem operation is read-only; no write/delete/shell/process behavior.
- Capability and target scope are explicit and independent of wording.
- Denial and out-of-scope decisions occur before adapter invocation.
- Adapter failures remain typed; no fake success.
- Stable normalized results must not depend on raw platform error text.
- Do not call the capability Verified or Supported before exact-SHA Linux/Windows evidence.

## Review Focus

- Source wording without explicit capability must deny before I/O — covered by CLI denial test.
- A granted capability with a different target must deny before I/O — covered by CLI scope test.
- Missing adapter must remain `NoCompatibleAdapter` — covered by orchestration registry test.
- Backend I/O failure must remain typed and deterministic — covered by filesystem adapter failure test.
- Existing DNS orchestration must remain behaviorally compatible — covered by existing DNS and workspace tests.

### Task 1: Filesystem adapter contract

**Files:**
- Modify: `crates/one-core/src/adapter.rs`
- Test: `crates/one-core/src/adapter.rs`

- [x] Add filesystem adapter success/failure tests.
- [x] Implement `FileReadAdapter` with `filesystem.file.read` identity, `Capability::Filesystem`, and stable `BackendFailure` classification.
- [ ] Run focused adapter tests and workspace tests.

### Task 2: Source grammar and semantic boundary

**Files:**
- Modify: `crates/one-core/src/parser.rs`
- Modify: `crates/one-core/src/semantic.rs`
- Modify: `crates/one-core/src/orchestration.rs`

- [x] Add `baca berkas <target>` parsing.
- [x] Validate exactly one text target.
- [x] Resolve Indonesian alias to `filesystem.read` without granting authority.
- [ ] Run parser/semantic/orchestration tests.

### Task 3: Generic orchestration IR

**Files:**
- Modify: `crates/one-core/src/ir.rs`
- Modify: `crates/one-core/src/orchestrator.rs`

- [x] Represent adapter identity and normalization expectation in orchestration IR.
- [x] Preserve DNS lowering while adding filesystem lowering.
- [x] Add deterministic named-adapter selection without duplicating policy evaluation.
- [ ] Run IR/orchestrator tests.

### Task 4: Interpreter and CLI execution

**Files:**
- Modify: `crates/one-core/src/interpreter.rs`
- Modify: `crates/one-cli/src/main.rs`
- Test: `crates/one-cli/tests/cli.rs`

- [x] Register filesystem adapter in the governed interpreter.
- [x] Execute generic orchestration using IR-selected adapter and normalization.
- [x] Accept `--izinkan filesystem.read --target <path>`.
- [x] Add source-level success, denial, and out-of-scope tests.
- [ ] Verify CLI behavior on Linux and Windows.

### Task 5: Independent verification and documentation

**Files:**
- Modify: `README.md` and relevant living user/verification documentation only after implementation is green.

- [ ] Run exact-SHA Ubuntu/Windows verification through the independent mirror.
- [ ] Fix any source failure using reproduce → root cause → minimal fix → reverify.
- [ ] Update documentation status from in-development to the evidence-backed level only.
- [ ] Merge only after verification is green and canonical read-back matches the verified commit.

## Completion Gate

Filesystem read is complete only when all tasks are green, the exact implementation commit has independent Ubuntu/Windows evidence, DNS remains green, documentation matches the actual boundary, and no proven failure remains.


## CI provider update — 2026-11-13

CircleCI is the canonical CI provider under ADR-0017. Historical independent exact-SHA verification remains supplemental evidence and must not be mislabeled as CircleCI execution.
