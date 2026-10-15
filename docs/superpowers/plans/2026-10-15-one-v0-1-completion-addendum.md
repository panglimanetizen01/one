# One v0.1 Completion Addendum

This addendum records the execution state of `docs/superpowers/plans/2026-10-15-one-v0-1-implementation.md` without rewriting its original task history.

## Task ledger

| Task | State | Canonical result |
|---|---|---|
| 1. Lexical/grammar contract | COMPLETE | lexical + concrete grammar specs + normative grammar example fixture |
| 2. Workspace/CI | COMPLETE | Cargo workspace + Linux/Windows workflow + contract-artifact gate |
| 3. Lexer | IMPLEMENTED | lexer/token modules + tests |
| 4. AST/parser | IMPLEMENTED | AST/parser modules + tests |
| 5. Names/scopes/types | IMPLEMENTED | semantic/value/type modules + tests |
| 6. Semantic IR | IMPLEMENTED | IR module + lowering tests |
| 7. Reference interpreter | IMPLEMENTED | runtime/interpreter + execution tests |
| 8. Diagnostics | IMPLEMENTED | structured diagnostic model + tests |
| 9. Capability/platform boundary | IMPLEMENTED | capability/platform modules + security tests |
| 10. CLI | IMPLEMENTED | `new/check/run/build/ast` + CLI tests |
| 11. Standard library/project support | IMPLEMENTED | stdlib + project scaffolding |
| 12. Process interoperability | IMPLEMENTED | explicit process boundary + tests |
| 13. Conformance/golden suite | COMPLETE AS ARTIFACTS | fixtures, runner, golden outputs, contract docs |
| 14. Documentation/security/release contract | COMPLETE AS ARTIFACTS | release docs, evidence matrix, README/security docs, documentation index |
| 15. Full release verification | BLOCKED | hosted Linux/Windows runners fail before usable step evidence; offline verifiers are the no-upgrade path |

## Artifact closure

The normative Markdown contracts and their repository derivatives are now explicitly indexed and mechanically checked. The canonical derivative set includes:

- formal foundation/specification and dependency-ordered plan;
- lexical, grammar, AST, localization, diagnostics, standard-library, interop, and CLI contracts;
- conformance/release/verification documentation;
- valid/invalid One fixtures and golden outputs;
- executable conformance tests;
- static contract-artifact consistency checker;
- independent Linux and Windows verification scripts;
- CI/release workflows that invoke the contract checks before claiming execution evidence.

The repository's documentation index is the navigation surface for these artifacts, while `tools/check_contract.py` is the machine-checkable presence/consistency gate.

## Rulings

- **Ruling:** Do not purchase a GitHub upgrade solely to obtain hosted CI execution — the release contract explicitly permits independent offline verification and requires hosted-CI status to remain honestly marked as blocked.
- **Ruling:** The repository root remains a virtual Cargo workspace, so the executable conformance integration test lives under `crates/one-core/tests/conformance.rs`; a root `tests/conformance.rs` would not be compiled automatically by Cargo.
- **Ruling:** v0.1 does not silently accept multi-word user identifiers or the conversational `selama masih ada ...` loop form; both are documented as future syntax extensions.
- **Ruling:** `one build` is documented as an IR/debug artifact generator, not a native compiler, until a native backend contract and verification exist.
- **Ruling:** Independent offline verification evidence may close the execution gate without changing historical hosted-CI results. The exact commit SHA, OS, Rust/Cargo versions, commands, and results must be recorded.

## Completion gate

Source, normative documentation, fixtures, executable conformance artifacts, and verification tooling are complete for the defined v0.1 implementation scope. The only remaining release-gate work is actual execution evidence on Linux and Windows. If hosted GitHub runners remain unavailable, record independent offline verifier results rather than paying for an upgrade or fabricating CI evidence.
