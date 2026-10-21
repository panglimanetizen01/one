# One v0.1 Specification / Implementation Gap Audit

**Audit date:** 2026-10-21  
**Scope:** Canonical GitHub `main` source files inspected for the One v0.1 language core.  
**Disposition:** Documentation corrections merged in PR #66. The AST contract has now been explicitly narrowed to the accepted v0.1 implementation schema in commit `21793da8ee05e50977b7d38b7262daaae18b065d`, and ADR-0015 records that decision in commit `901a5e5345d09c877e8c063673b0acf8e141f34d`. Parser regression tests were added in commit `2984d0eca5fb7de3a1a68e1f270a66f14906af00`. The complete independent verification matrix passed on both Ubuntu and Windows for PR #69 head `253f1f7bdf34fec02d7fb8369b55d9fe3af0c1ed`; the merged canonical commit `90db05209bdec3eb40ca57524b3694e98cc75c79` was separately verified successfully in independent matrix run `39553591078`.

## Executive summary

The concrete grammar and lexical specification are normative. The AST contract now explicitly describes the current v0.1 AST schema and its limitations rather than claiming an unimplemented source-preserving tree. This resolves the specification/implementation mismatch at the v0.1 contract level. Full statement/expression ranges and per-word name spans remain documented limitations, not supported capabilities.

The audit began as a source review. Subsequent independent exact-SHA verification for PR #69 passed the contract checker, build, AST regression tests, Clippy, workspace tests, real-network tests, and offline verification on Ubuntu and Windows. The merged main SHA is undergoing a separate exact-SHA confirmation; this audit does not by itself claim release readiness.

## Confirmed findings

### RESOLVED — AST contract scope and implementation shape

Before the correction, `docs/spec/one-v0.1-ast.md` proposed structured `Name` nodes, component spans, full node ranges, and action-specific variants while remaining draft normative for review. That proposal was broader than the existing public AST.

**Decision:** ADR-0015 accepts the existing v0.1 public AST schema: normalized string names, the current statement/expression variants, and statement-introducer span anchors. The revised contract explicitly records that full statement/expression spans and per-word name spans are not available in v0.1.

**Residual limitation:** source tooling and diagnostics cannot rely on component-level name spans or full expression ranges in v0.1. This is now an explicit limitation rather than an implied capability. Future expansion requires a separately reviewed compatibility change.

**Verification:** the new parser tests passed on both Ubuntu and Windows in the independent matrix for PR #69 head `253f1f7bdf34fec02d7fb8369b55d9fe3af0c1ed`. The full workspace suite, Clippy, contract checker, network runtime tests, and offline verification also passed on both operating systems for that exact SHA.

**Schema differences intentionally scoped out of v0.1 by the accepted decision:**

- The draft's `Program` and several expression/structural nodes require spans that the current AST types do not carry.
- The draft proposes explicit action variants (`Display`, `CreateList`, `AddToList`, `RemoveFromList`), while implementation uses a generic `Statement::Action { name, args, span }` and supports additional actions.
- The draft models structured `Condition`, `ForEach`, `Function`, `Parameter`, `EventTrigger`, and `Reference` nodes; the current implementation uses different enum/field shapes and plain strings in multiple places.
- The draft describes an implementation-neutral integer representation, while the current AST uses `i64`.
- The draft says it depends on a name-boundary contract; the implementation gate must verify that the referenced contract exists, is authoritative, and is linked from the index before relying on it.

These differences are no longer presented as a conformance failure: the accepted v0.1 contract now documents the implementation-aligned schema and limitations. A future richer AST requires a coordinated migration and tests.

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

- PR #66 changed documentation only.
- Subsequent commits revised the AST contract, recorded ADR-0015, and added parser regression tests.
- Independent exact-SHA verification passed for PR #69 head `253f1f7bdf34fec02d7fb8369b55d9fe3af0c1ed` on Ubuntu and Windows, including contract checker, build, AST tests, Clippy, workspace tests, real-network tests, and offline verification.
- The merged canonical SHA `e0c09d5dd99f4b8eb9c9af961739cf9ba0c8e2e8` was independently verified successfully on Ubuntu and Windows in matrix run `39554088367`; evidence is tied to this exact SHA.
- Contract alignment and the documented cross-platform verification issues are resolved on the tested PR head. Release readiness remains a separate project-level decision.

## Future AST expansion

1. If v0.2 requires component-level name spans or full statement/expression ranges, write a separate compatibility proposal first.
2. Add fail-first tests and coordinate parser, semantic analysis, IR lowering, diagnostics, and tooling changes.
3. Verify the exact commit on every supported platform before declaring the expanded AST conformant.
