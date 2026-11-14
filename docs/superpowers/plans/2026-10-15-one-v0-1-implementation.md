# One v0.1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a deterministic, Indonesian-first, general-purpose One v0.1 reference implementation with a formal core grammar, AST, semantic analysis, interpreter, CLI, conformance tests, and first-class Windows/Linux portability foundations.

**Architecture:** Implement One as a Rust workspace with a dependency-ordered compiler front end and reference interpreter. The language surface is localized and conversational, but parsing produces a language-independent AST and Semantic IR; execution is performed without an LLM. Platform-sensitive operations are isolated behind a capability-aware runtime boundary.

**Tech Stack:** Rust stable; Cargo workspace; Rust standard library first; CircleCI as the canonical Linux CI provider, with Windows-host verification in the independent `panglimanetizen01/one-windows-verification` repository (superseding the original GitHub Actions selection); no runtime dependency on Python, Node.js, Java, or an LLM.

**Spec:** `docs/superpowers/specs/2026-10-15-one-language-foundation-design.md`

## Global Constraints

- One is general-purpose and must scale from small scripts to large projects.
- Indonesian is the first implementation locale; localization is a first-class architectural concern.
- Grammar is formal and deterministic; arbitrary prose is not accepted.
- Newlines and indentation may express program structure; required punctuation is minimized.
- Ambiguity is rejected rather than guessed.
- LLMs are not required for parsing, compilation, execution, testing, or runtime semantics.
- Windows and Linux are first-class platforms.
- One must be able to complete a project without another programming language being mandatory.
- Interoperability with JavaScript/Node.js, Python, C/C++, Java/JVM, HTML/CSS, shell/Linux tooling, databases, networking, and native libraries is an architectural target; only contracts that are implemented and tested may be claimed as supported.
- No compiler/runtime operation silently rewrites source.
- Semantic IR is the boundary between localized syntax and execution.

## Review Focus

1. Indentation and blank-line edge cases must never change block meaning unexpectedly — covered by parser/conformance tests.
2. Ambiguous identifiers and unresolved names must fail deterministically with source spans — covered by semantic tests.
3. Numeric/text/boolean literals must preserve exact intended values without implicit guessing — covered by lexer/value tests.
4. Windows and Linux path/process behavior must not leak into core semantics — covered by platform contract tests.
5. Foreign-process interoperability must not silently discard exit status, stdout/stderr, or failures — covered by interop tests.

---

### Task 1: Formalize One v0.1 lexical and concrete grammar contract

**Files:**
- Create: `docs/spec/one-v0.1-grammar.md`
- Create: `docs/spec/one-v0.1-lexical.md`
- Create: `tests/spec/grammar_examples.one`

**Interfaces:**
- Produces the exact token categories, indentation rules, statement forms, literal forms, and core construct grammar consumed by the lexer/parser tasks.

- [ ] Write failing conformance fixtures for valid and invalid source.
- [ ] Record expected tokenization and parse shape for each fixture.
- [ ] Define identifiers, strings, numbers, booleans, blank lines, comments, indentation, and statement boundaries.
- [ ] Define formal grammar for DATA, ACTION, CONDITION, LOOP, FUNCTION, EVENT.
- [ ] Define explicit ambiguity rejection rules.
- [ ] Commit `docs: formalize one v0.1 grammar contract`.

### Task 2: Create Rust workspace and CI matrix

**Files:**
- Create: `Cargo.toml`
- Create: `crates/one-core/Cargo.toml`
- Create: `crates/one-cli/Cargo.toml`
- Create: `.github/workflows/ci.yml`
- Create: `.gitignore`
- Create: `rust-toolchain.toml`

**Interfaces:**
- Provides `one-core` library and `one-cli` binary crates.
- CI must test Linux and Windows using the same command set.

- [ ] Add minimal workspace smoke test first.
- [ ] Run CI and verify the smoke test is green on Linux and Windows.
- [ ] Commit `build: initialize rust workspace and cross-platform ci`.

### Task 3: Implement lexical scanner

**Files:**
- Create: `crates/one-core/src/lexer.rs`
- Create: `crates/one-core/src/token.rs`
- Test: `crates/one-core/src/lexer.rs`

**Interfaces:**
- `Lexer::new(source: &str) -> Lexer`
- `Lexer::scan(&mut self) -> Result<Vec<Token>, Diagnostic>`
- `Token { kind: TokenKind, lexeme: String, span: Span }`

- [ ] Add failing tests for identifiers, keywords/phrases, strings, numbers, indentation, newline, comments, and invalid characters.
- [ ] Verify failures.
- [ ] Implement minimal scanner.
- [ ] Verify green tests.
- [ ] Commit `feat: add deterministic one lexer`.

### Task 4: Implement AST and parser

**Files:**
- Create: `crates/one-core/src/ast.rs`
- Create: `crates/one-core/src/parser.rs`
- Test: `crates/one-core/src/parser.rs`

**Interfaces:**
- `parse(source: &str) -> Result<Program, DiagnosticBundle>`
- AST nodes for declaration, assignment, action, conditional, loop, function, event, expression, and block.

- [ ] Add failing tests for every core construct and nested blocks.
- [ ] Add rejection tests for bad indentation, malformed blocks, and ambiguous syntax.
- [ ] Implement parser against the locked grammar.
- [ ] Verify parser tests and AST snapshots.
- [ ] Commit `feat: add one parser and ast`.

### Task 5: Implement names, scopes, values, and initial type system

**Files:**
- Create: `crates/one-core/src/value.rs`
- Create: `crates/one-core/src/types.rs`
- Create: `crates/one-core/src/scope.rs`
- Create: `crates/one-core/src/semantic.rs`
- Test: `crates/one-core/src/semantic.rs`

**Interfaces:**
- `analyze(program: &Program) -> Result<TypedProgram, DiagnosticBundle>`
- Initial types: Text, Number, Boolean, List, Nothing.
- Deterministic lexical scope and declaration/reference rules.

- [ ] Add failing tests for declaration, reassignment compatibility, undefined names, duplicate names, and ambiguous references.
- [ ] Implement semantic analysis.
- [ ] Verify green semantic tests.
- [ ] Commit `feat: add one semantic analysis and types`.

### Task 6: Define and implement Semantic IR

**Files:**
- Create: `crates/one-core/src/ir.rs`
- Test: `crates/one-core/src/ir.rs`
- Modify: `crates/one-core/src/semantic.rs`

**Interfaces:**
- `lower(program: &TypedProgram) -> Result<IrProgram, DiagnosticBundle>`
- IR must be independent of Indonesian surface keywords.

- [ ] Add failing IR tests for all six core constructs.
- [ ] Implement lowering.
- [ ] Verify IR invariants.
- [ ] Commit `feat: add language independent semantic ir`.

### Task 7: Implement reference interpreter

**Files:**
- Create: `crates/one-core/src/runtime.rs`
- Create: `crates/one-core/src/interpreter.rs`
- Test: `crates/one-core/src/interpreter.rs`

**Interfaces:**
- `Interpreter::new(capabilities: CapabilitySet) -> Interpreter`
- `Interpreter::execute(program: &IrProgram) -> Result<ExecutionResult, RuntimeError>`

- [ ] Add failing execution tests for DATA, ACTION, CONDITION, LOOP, FUNCTION, EVENT registration/dispatch, and expression evaluation.
- [ ] Implement minimal deterministic interpreter.
- [ ] Verify behavior.
- [ ] Commit `feat: add one reference interpreter`.

### Task 8: Implement diagnostics and source-preserving error reporting

**Files:**
- Create: `crates/one-core/src/diagnostic.rs`
- Test: `crates/one-core/src/diagnostic.rs`
- Modify: lexer/parser/semantic/runtime modules

**Interfaces:**
- `Diagnostic { severity, code, message, span, notes, suggestions }`
- No diagnostic path mutates source.

- [ ] Add failing tests for lexical, syntax, semantic, and runtime errors.
- [ ] Implement structured diagnostics and stable error codes.
- [ ] Verify source spans and human-readable output.
- [ ] Commit `feat: add structured diagnostics`.

### Task 9: Implement capability-aware platform boundary

**Files:**
- Create: `crates/one-core/src/capability.rs`
- Create: `crates/one-core/src/platform.rs`
- Test: `crates/one-core/src/platform.rs`

**Interfaces:**
- `CapabilitySet` explicitly controls filesystem/process/network/native operations.
- Core semantics must not directly depend on OS-specific APIs.

- [ ] Add failing tests proving denied capabilities fail deterministically.
- [ ] Implement platform-neutral capability interfaces.
- [ ] Add Windows/Linux implementations only where needed by v0.1.
- [ ] Verify tests on both CI platforms.
- [ ] Commit `feat: add capability aware platform boundary`.

### Task 10: Implement One CLI

**Files:**
- Create: `crates/one-cli/src/main.rs`
- Test: `crates/one-cli/tests/cli.rs`

**Interfaces:**
- `one check <file>` parses and analyzes.
- `one run <file>` executes using the reference interpreter.
- `one ast <file>` prints AST/IR diagnostics for developers.
- Exit codes are stable and documented.

- [ ] Add failing CLI tests for valid run, syntax error, semantic error, and missing file.
- [ ] Implement CLI.
- [ ] Verify Linux product checks in CircleCI and Windows host execution through the independent exact-SHA verifier.
- [ ] Commit `feat: add one cli`.

### Task 11: Add standard library core and project execution support

**Files:**
- Create: `crates/one-core/src/stdlib.rs`
- Create: `docs/spec/one-v0.1-stdlib.md`
- Test: `crates/one-core/src/stdlib.rs`

**Interfaces:**
- Initial safe built-ins for display, text, list operations, numeric operations, and deterministic time-independent helpers.
- Platform-sensitive operations require explicit capabilities.

- [ ] Add failing stdlib behavior tests.
- [ ] Implement minimal standard library.
- [ ] Verify interpreter integration.
- [ ] Commit `feat: add one v0.1 standard library core`.

### Task 12: Add process interoperability foundation

**Files:**
- Create: `crates/one-core/src/interop.rs`
- Create: `docs/spec/one-v0.1-interop.md`
- Test: `crates/one-core/src/interop.rs`

**Interfaces:**
- Explicit process execution API for foreign tools.
- Preserve argv, exit status, stdout, stderr, and failure state.
- Python/Node/Java/C++ integration begins through explicit process boundaries; native FFI remains a later contract and is not falsely claimed as complete.

- [ ] Add failing tests using portable test helper commands.
- [ ] Implement process boundary with capability checks.
- [ ] Verify cross-platform behavior.
- [ ] Commit `feat: add explicit process interoperability`.

### Task 13: Add conformance suite and golden tests

**Files:**
- Create: `tests/conformance/*.one`
- Create: `tests/golden/*.snap`
- Create: `tests/conformance.rs`
- Create: `docs/testing/conformance.md`

**Interfaces:**
- Every accepted/rejected language construct is represented by a stable fixture.
- Reference interpreter is the semantic oracle for later compiler work.

- [ ] Add failing conformance fixtures.
- [ ] Implement fixture runner.
- [ ] Verify all core constructs, ambiguity, diagnostics, and localization mapping.
- [ ] Commit `test: add one v0.1 conformance suite`.

### Task 14: Documentation, security baseline, and release contract

**Files:**
- Create: `SECURITY.md`
- Create: `CONTRIBUTING.md`
- Create: `CHANGELOG.md`
- Create: `docs/release/one-v0.1.md`
- Modify: `README.md`

**Interfaces:**
- Documentation must state exactly what v0.1 supports and what remains future work.
- No unsupported interoperability/platform claims.

- [ ] Add release acceptance checklist.
- [ ] Verify docs against implementation and tests.
- [ ] Commit `docs: define one v0.1 release contract`.

### Task 15: Full CI verification and canonical release gate

**Files:**
- Modify: `.github/workflows/ci.yml`
- Create: `.github/workflows/release-check.yml`
- Create: `docs/verification/one-v0.1-verification.md`

**Interfaces:**
- CI must run formatting, compilation, unit tests, integration tests, conformance tests, and CLI smoke tests on Linux and Windows.
- Release check must fail if the documented v0.1 acceptance criteria are not met.

- [ ] Add failing release-gate checks where coverage is incomplete.
- [ ] Implement gate.
- [ ] Run full CI and inspect every job.
- [ ] Fix all failures using TDD and rerun until green.
- [ ] Record commit SHAs and workflow evidence in verification document.
- [ ] Commit `ci: establish one v0.1 release gate`.

---

## Completion Contract

One v0.1 is not considered complete until:

- formal grammar and lexical rules exist;
- parser, AST, semantic analysis, and Semantic IR exist;
- reference interpreter executes the six core constructs;
- diagnostics are structured and source-preserving;
- capability-aware platform boundary exists;
- CLI supports check/run/ast;
- core standard library exists;
- explicit foreign-process interoperability exists and is tested;
- conformance/golden tests cover the language core;
- Windows and Linux CI are green;
- documentation accurately describes supported and unsupported features;
- release gate is green;
- no known failing test or contradictory contract remains in the canonical repository.

Deep native FFI, package registry, full compiler backends, GUI framework, IDE, and additional locales are follow-on milestones unless their contracts are explicitly implemented and verified in v0.1. They must not be represented as already complete.


## CI provider supersession — 2026-11-13

The original workflow tasks above record the historical GitHub Actions implementation. ADR-0017 selects CircleCI as the canonical Linux CI execution provider. The current migration configuration is `.circleci/config.yml` and retains Linux product/release jobs only; Windows Server 2025 host verification is performed by the independent `panglimanetizen01/one-windows-verification` workflow at a full pinned source SHA. The three original GitHub Actions workflow files are deleted from the migration branch; `main` remains unchanged until exact-SHA Linux CircleCI and independent Windows evidence are inspected and the PR is reviewed. Do not treat this historical plan's GitHub Actions task references as the current provider decision.
