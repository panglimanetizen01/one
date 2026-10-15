# One documentation map

This directory is the normative documentation surface for One v0.1. When code and prose disagree, the implementation status document must identify the discrepancy; a silent contract change is not allowed.

## Foundation

- `superpowers/specs/2026-10-15-one-language-foundation-design.md` — locked architectural foundation and invariants.
- `superpowers/plans/2026-10-15-one-v0-1-implementation.md` — dependency-ordered implementation plan.
- `superpowers/plans/2026-10-15-one-v0-1-completion-addendum.md` — current task ledger, rulings, and remaining release gate.

## Language/tooling contracts

- `spec/one-v0.1-language-invariants.md` — immutable v0.1 language invariants.
- `spec/one-v0.1-lexical.md` — tokens, indentation, literals, comments, and lexical errors.
- `spec/one-v0.1-grammar.md` — normative Indonesian v0.1 concrete grammar.
- `spec/one-v0.1-ast.md` — language-independent AST shape.
- `spec/one-v0.1-localization-contract.md` — locale architecture and semantic equivalence rules.
- `spec/one-v0.1-localization.md` — retained v0.1 localization contract artifact.
- `spec/one-v0.1-diagnostics.md` — diagnostic codes, spans, and source-preserving behavior.
- `spec/one-v0.1-effect-capability.md` — explicit effect/capability boundary.
- `spec/one-v0.1-runtime-model.md` — runtime state, execution, errors, and determinism.
- `spec/one-v0.1-reference-interpreter.md` — executable semantic oracle contract.
- `spec/one-v0.1-conformance-suite.md` — layered conformance and golden-test contract.
- `spec/one-v0.1-platform-abstraction.md` — Windows/Linux host boundary.
- `spec/one-v0.1-module-system.md` — module identity, imports, exports, and initialization.
- `spec/one-v0.1-package-build.md` — project manifests, dependency resolution, and build/package model.
- `spec/one-v0.1-standard-library.md` — v0.1 standard-library boundary.
- `spec/one-v0.1-stdlib.md` — retained v0.1 standard-library contract artifact.
- `spec/one-v0.1-cli-contract.md` — canonical CLI commands, aliases, exit codes, project operations, and determinism.
- `spec/one-v0.1-cli.md` — retained v0.1 CLI contract artifact.
- `spec/one-v0.1-interop.md` — explicit foreign-process interoperability boundary.
- `implementation/one-v0.1-core-implementation-gate.md` — implementation sequencing and evidence gate.

## Verification and release

- `testing/conformance.md` — retained v0.1 conformance contract artifact.
- `verification/one-v0.1-implementation-status.md` — implementation inventory.
- `verification/one-v0.1-verification.md` — evidence record and current verification blockers.
- `verification/one-v0.1-evidence-matrix.md` — requirement-to-artifact-to-evidence matrix.
- `verification/one-v0.1-offline-evidence-template.md` — record format for independent Linux/Windows execution evidence.
- `verification/one-v0.1-linux-termux-2026-10-15.md` — recorded independent Linux/Termux execution evidence for commit `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0`.
- `release/one-v0.1.md` — release acceptance contract and supported/future matrix.

## Derivative artifacts

- `../tests/spec/grammar_examples.one` — canonical valid-source examples derived from the grammar contract.
- `../crates/one-core/tests/conformance.rs` — executable conformance runner derived from the fixture contract.
- `../crates/one-cli/tests/cli.rs` — CLI integration tests derived from the CLI contract.
- `../tests/conformance/valid/*.one` — accepted-language fixtures.
- `../tests/conformance/invalid/*.one` — rejected-language fixtures.
- `../tests/golden/*.snap` — deterministic execution outputs.
- `../tools/check_contract.py` — static contract-artifact consistency checker.
- `../tools/verify_offline.sh` — independent Linux verification path.
- `../tools/verify_offline.ps1` — independent Windows verification path.

## Canonical rule

GitHub `main` is the canonical source. Local/Termux copies are verification environments only. Documentation must never claim green cross-platform execution without actual evidence from a successful Linux and Windows run.
