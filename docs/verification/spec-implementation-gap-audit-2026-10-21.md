# One v0.1 Specification / Implementation Gap Audit

**Audit date:** 2026-10-21  
**Scope:** Canonical GitHub `main` source files inspected for the One v0.1 language core.  
**Disposition:** Documentation corrections merged in PR #66. The AST design/implementation gap remains open and requires an explicit AST-contract decision and coordinated implementation.

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

**Additional schema mismatches found in the same draft (review together, not as isolated patches):**

- The draft's `Program` and several expression/structural nodes require spans that the current AST types do not carry.
- The draft proposes explicit action variants (`Display`, `CreateList`, `AddToList`, `RemoveFromList`), while implementation uses a generic `Statement::Action { name, args, span }` and supports additional actions.
- The draft models structured `Condition`, `ForEach`, `Function`, `Parameter`, `EventTrigger`, and `Reference` nodes; the current implementation uses different enum/field shapes and plain strings in multiple places.
- The draft describes an implementation-neutral integer representation, while the current AST uses `i64`.
- The draft says it depends on a name-boundary contract; the implementation gate must verify that the referenced contract exists, is authoritative, and is linked from the index before relying on it.

These differences mean a one-field name patch would not make the AST conformant. The review must either define a staged, implementation-compatible v0.1 AST contract or accept the full source-preserving schema and plan a coordinated migration. Do not silently mix the two models.

### RESOLVED — Specification index gate wording

The pre-audit index described the lexical specification as the next gate and grammar/AST as later gates without distinguishing file presence from approval. PR #66 updated `docs/spec/README.md` to make gate status and review policy explicit. This resolves the documentation ambiguity; it does not certify any gate as approved or implemented.

### RESOLVED — Localization document authority labels

PR #66 labels `docs/spec/one-v0.1-localization.md` as a non-normative architecture overview and links to `docs/spec/one-v0.1-localization-contract.md`, which remains a draft for review. The authority/status ambiguity is resolved at the documentation level. Localization is not considered fully specified or multi-locale conformant until the detailed contract and its lexical, grammar, AST/CSIR equivalence, diagnostic, versioning, and six-construct conformance criteria are reviewed and tested.

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

## Verification boundary

- PR #66 changed documentation only; it did not change language contracts or implementation code.
- Rust tests, Clippy, the contract checker, and platform release verification were not run for PR #66.
- The CI runs reported failure on Ubuntu and Windows, while job-log artifacts were unavailable; the repository's existing issue #2 documents this hosted-runner evidence limitation and its independent verification route.
- The AST draft/implementation gap remains unresolved; no AST conformance or release-readiness claim is made.

## Safe next implementation sequence

1. Rebase or branch from the latest canonical `main` before implementation work.
2. Add fail-first tests for structured multi-word name spans and parser boundary cases.
3. Implement the AST/name representation without weakening the grammar or contract.
4. Update parser, semantic analysis, IR lowering, diagnostics, and fixtures affected by the new representation.
5. Run the contract checker, complete offline verification, Clippy, and all tests with an executable Cargo target directory.
6. Inspect the resulting GitHub diff and CI evidence before merging.
