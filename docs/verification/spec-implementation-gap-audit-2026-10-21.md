# One v0.1 Specification / Implementation Gap Audit

**Audit date:** 2026-10-21  
**Scope:** Canonical GitHub `main` source files inspected for the One v0.1 language core.  
**Disposition:** Findings recorded; no language contract or implementation source changed by this audit.

## Executive summary

The repository contains a concrete grammar and draft lexical and AST contracts. The current Rust AST/parser implementation does not implement the proposed source-preserving multi-word Name representation in the AST draft. This is a confirmed design/implementation gap against the proposed architecture, but it is not yet a violation of an approved AST contract: the AST document explicitly remains draft normative for review. Resolve the AST contract's approval and boundary details before treating this as a release-blocking conformance failure.

The audit was performed by reading canonical GitHub files. It did not run Rust tests, Clippy, the contract checker, or platform verification. No runtime or release status is upgraded by this document.

## Confirmed findings

### P0 candidate — AST name representation does not yet meet the proposed AST design

**Draft requirement:** `docs/spec/one-v0.1-ast.md` proposes a logical `Name` node containing component words and source spans. It states that a multi-word name must preserve its complete span and the span of each component. The same document labels itself **Draft normative contract for review**, so it must not be described as already approved.

**Observed implementation:** `crates/one-core/src/ast.rs` stores names as `String` in declarations, assignments, function names/parameters, loop bindings, and references. `crates/one-core/src/parser.rs` joins declaration words using `words.join(" ")`. The resulting AST does not retain per-word name spans or a structured `Name` value.

**Risk:** downstream diagnostics, refactoring/tooling, exact grammar-boundary validation, and future localization cannot rely on the source-preserving representation promised by the AST contract. Changing the draft to fit implementation may discard an important source-preservation goal. Conversely, the implementation must not be called nonconforming to an approved contract until the draft is reviewed and accepted. Decide the contract first, then implement against that explicit decision.

**Required remediation:** implement the specified logical name representation and preserve word/span data through parser construction and semantic analysis. Update all affected consumers deliberately. Add tests for single-word and multi-word names, exact spans, grammar terminators, and deterministic diagnostics. Run the complete offline verification workflow before proposing merge.

**Merge gate:** first approve or revise the AST draft with an explicit review decision; then implement and test the accepted contract. Do not claim AST conformance complete before both steps.

### P1 — Specification status and dependency gates must remain explicit

`docs/spec/README.md` describes lexical specification as the next review gate, followed by concrete grammar and AST. The existence of a document does not itself mean its review gate is approved. Keep each stage's status explicit and do not infer approval from file presence.

**Required remediation:** reviewers should record approval status for each gate and update the index only when evidence supports a status transition. Do not relabel draft specifications as approved solely to make the index appear complete.

### P1 — Localization overview and detailed contract have different authority/status presentation

`docs/spec/one-v0.1-localization.md` is a concise architecture overview. `docs/spec/one-v0.1-localization-contract.md` is a more detailed contract and explicitly labels itself a draft for review.

**Risk:** contributors may not know whether the detailed file is binding or still under review.

**Required remediation:** keep the overview explicitly non-normative and make the contract's approval state unambiguous. Before declaring localization fully specified, review name-boundary behavior, diagnostic parameter stability, AST/CSIR equivalence fixtures, locale versioning, and the six-construct mapping. Do not claim a multi-locale implementation is verified merely because the architecture permits it.

## Evidence reviewed

- `docs/spec/README.md`
- `docs/spec/one-v0.1-language-invariants.md`
- `docs/spec/one-v0.1-lexical.md`
- `docs/spec/one-v0.1-grammar.md`
- `docs/spec/one-v0.1-ast.md`
- `docs/spec/one-v0.1-localization.md`
- `docs/spec/one-v0.1-localization-contract.md`
- `crates/one-core/src/token.rs`
- `crates/one-core/src/ast.rs`
- `crates/one-core/src/parser.rs`
- `crates/one-core/tests/conformance.rs`
- `tools/check_contract.py`

## Explicitly not verified

- Whether `python tools/check_contract.py` passes on the current canonical HEAD.
- Whether `cargo test --workspace --all-targets` and Clippy pass on current canonical HEAD.
- Whether all AST-contract requirements are tested.
- Whether hosted CI is healthy.
- Whether Windows or Linux native release artifacts pass end-to-end verification.

## Safe next implementation sequence

1. Rebase or branch from the latest canonical `main` before implementation work.
2. Add fail-first tests for structured multi-word name spans and parser boundary cases.
3. Implement the AST/name representation without weakening the grammar or contract.
4. Update parser, semantic analysis, IR lowering, diagnostics, and fixtures affected by the new representation.
5. Run the contract checker, complete offline verification, Clippy, and all tests with an executable Cargo target directory.
6. Inspect the resulting GitHub diff and CI evidence before merging.
