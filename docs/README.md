# One documentation map

This directory is the normative documentation surface for One v0.1. When code and prose disagree, the implementation status document must identify the discrepancy; a silent contract change is not allowed.

## Foundation

- `superpowers/specs/2026-10-15-one-language-foundation-design.md` — locked architectural foundation and invariants.
- `superpowers/plans/2026-10-15-one-v0-1-implementation.md` — dependency-ordered implementation plan.
- `superpowers/plans/2026-10-15-one-v0-1-completion-addendum.md` — current task ledger, rulings, and remaining release gate.

## Language contracts

- `spec/one-v0.1-lexical.md` — tokens, indentation, literals, comments, and lexical errors.
- `spec/one-v0.1-grammar.md` — normative Indonesian v0.1 concrete grammar.
- `spec/one-v0.1-ast.md` — language-independent AST shape.
- `spec/one-v0.1-localization.md` — locale architecture and semantic equivalence rules.
- `spec/one-v0.1-diagnostics.md` — diagnostic codes, spans, and source-preserving behavior.
- `spec/one-v0.1-stdlib.md` — v0.1 standard-library surface.
- `spec/one-v0.1-interop.md` — explicit foreign-process interoperability boundary.

## Verification and release

- `testing/conformance.md` — fixture and golden-test contract.
- `release/one-v0.1.md` — release acceptance contract and supported/future matrix.
- `verification/one-v0.1-implementation-status.md` — implementation inventory.
- `verification/one-v0.1-verification.md` — evidence record and current verification blockers.
- `verification/one-v0.1-evidence-matrix.md` — requirement-to-artifact-to-evidence matrix.
- `verification/one-v0.1-offline-evidence-template.md` — record format for independent Linux/Windows execution evidence.
- `../tools/verify_offline.sh` — independent Linux verification path.
- `../tools/verify_offline.ps1` — independent Windows verification path.

## Canonical rule

GitHub `main` is the canonical source. Local/Termux copies are verification environments only. Documentation must never claim green cross-platform execution without actual evidence from a successful Linux and Windows run.
