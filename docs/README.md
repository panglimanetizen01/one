# One documentation map

This directory is the normative documentation surface for One v0.1. When code and prose disagree, the implementation status document must identify the discrepancy; a silent contract change is not allowed.

## Architecture navigation

- [Full architecture map](architecture/one-full-architecture-map.png) — visual navigation across language foundation, compiler/runtime, platform, capability, interoperability, and verification layers.
- [Architecture contract](ARCHITECTURE.md) — written architecture boundaries, semantic authority, and implementation constraints.

The diagram complements the written contract. It is a navigation reference, not a substitute for specifications, source code, tests, or exact-SHA verification evidence.

## User documentation

For people who want to **use One**, start here:

- `panduan-pengguna/README.md` — official beginner entry point.
- `panduan-pengguna/mulai-dari-nol.md` — first-run orientation.
- `panduan-pengguna/instalasi.md` — current installation/distribution boundary.
- `panduan-pengguna/program-pertama.md` — first usable program.
- `panduan-pengguna/konsep-dasar.md` — current language concepts.
- `panduan-pengguna/data-dan-nama.md` — data and multi-word names.
- `panduan-pengguna/kondisi-dan-perulangan.md` — conditions and loops.
- `panduan-pengguna/fungsi-dan-event.md` — functions and events.
- `panduan-pengguna/membuat-project.md` — project creation and layout.
- `panduan-pengguna/perintah-cli.md` — current CLI commands.
- `panduan-pengguna/test-build-package.md` — testing, build, and package boundaries.
- `panduan-pengguna/kesalahan-dan-debugging.md` — common mistakes and debugging.
- `panduan-pengguna/jaringan.md` — verified network boundary.
- `panduan-pengguna/interoperabilitas.md` — current interoperability boundary.
- `panduan-pengguna/faq.md` — common user questions.
- `panduan-pengguna/status-fitur.md` — capability status vocabulary.
- `panduan-pengguna/memperbarui-dokumentasi.md` — how documentation evolves with One.

User documentation is intentionally separate from normative engineering contracts. It is editable and versioned, but it does not override language semantics or verification evidence.

## Foundation

- `superpowers/specs/2026-10-15-one-language-foundation-design.md` — locked architectural foundation and invariants.
- `superpowers/specs/2026-10-15-one-end-to-end-network-platform-design.md` — approved end-to-end network/platform architecture.
- `superpowers/plans/2026-10-15-one-v0-1-implementation.md` — dependency-ordered implementation plan.
- `superpowers/plans/2026-10-15-one-v0-1-completion-addendum.md` — current task ledger and release gate.
- `superpowers/plans/2026-10-15-one-network-foundation.md` — One Network semantic vertical slice.
- `superpowers/specs/2026-10-16-one-general-purpose-human-language-tool-orchestration-and-governance.md` — orchestration architecture and current implemented boundary.
- `superpowers/plans/2026-10-16-one-human-language-tool-orchestration-and-governance.md` — orchestration implementation plan and next language-level gate.
- `superpowers/specs/2026-10-16-one-orchestration-implementation-contract.md` — executable orchestration contract.
- `superpowers/specs/2026-10-19-native-backends-linux-windows.md` — native hosted backend contract for Linux AArch64, Linux x86_64, and Windows x86_64.
- `superpowers/plans/2026-10-19-native-backend-linux-aarch64.md` — Linux AArch64 implementation and evidence plan.
- `superpowers/plans/2026-10-19-native-backend-linux-x86_64.md` — Linux x86_64 implementation and evidence plan.
- `superpowers/plans/2026-10-19-native-backend-windows-x86_64.md` — Windows x86_64 implementation and evidence plan.

## Language/tooling contracts

- `spec/one-v0.1-language-invariants.md` — immutable v0.1 language invariants.
- `spec/one-v0.1-lexical.md` — tokens, indentation, literals, comments, and lexical errors.
- `spec/one-v0.1-grammar.md` — normative Indonesian v0.1 concrete grammar.
- `spec/one-v0.1-ast.md` — language-independent AST shape.
- `spec/one-v0.1-localization-contract.md` — locale architecture and semantic equivalence rules.
- `spec/one-v0.1-diagnostics.md` — diagnostic codes, spans, and source-preserving behavior.
- `spec/one-v0.1-effect-capability.md` — explicit effect/capability boundary.
- `spec/one-v0.1-runtime-model.md` — runtime state, execution, errors, and determinism.
- `spec/one-v0.1-reference-interpreter.md` — executable semantic oracle contract.
- `spec/one-v0.1-conformance-suite.md` — layered conformance and golden-test contract.
- `spec/one-v0.1-platform-abstraction.md` — Windows/Linux host boundary.
- `spec/one-v0.1-module-system.md` — module identity, imports, exports, and initialization.
- `spec/one-v0.1-package-build.md` — project manifests, dependency resolution, and build/package model.
- `spec/one-v0.1-standard-library.md` — v0.1 standard-library boundary.
- `spec/one-v0.1-cli-contract.md` — canonical CLI commands, aliases, exit codes, project operations, and determinism.
- `spec/one-v0.1-interop.md` — explicit foreign-process interoperability boundary.
- `spec/one-network-foundation.md` — implemented typed network semantic/IR/reference boundary and explicit limits.
- `implementation/one-v0.1-core-implementation-gate.md` — implementation sequencing and evidence gate.

## Verification and release

- `verification/one-completion-ledger.md` — fixed-weight completion ledger; current implementation estimate and exact-SHA verified points.
- `verification/DEFINITION_OF_DONE.md` — release acceptance checklist and fail-closed Definition of Done.

- `testing/conformance.md` — retained v0.1 conformance contract artifact.
- `verification/one-v0.1-implementation-status.md` — implementation inventory and current orchestration/native-backend boundary.
- `verification/one-v0.1-verification.md` — current exact-SHA evidence record for canonical main plus branch-bound evidence where explicitly labeled.
- `verification/one-v0.1-evidence-matrix.md` — requirement-to-artifact-to-evidence matrix.
- `verification/one-v0.1-offline-evidence-template.md` — record format for independent Linux/Windows execution evidence.
- `verification/one-v0.1-linux-termux-2026-10-15.md` — historical independent Linux/Termux execution evidence for its explicitly recorded source revision.
- `verification/one-v0.1-linux-windows-2026-10-15.md` — historical independent Linux/Windows execution evidence for its explicitly recorded source revision.
- `verification/native-backends-2026-10-19.md` — exact verification boundary for PR #64 native hosted backends.
- `release/one-v0.1.md` — release acceptance contract and supported/future matrix.

## Derivative artifacts

- `../tests/spec/grammar_examples.one` — canonical valid-source examples derived from the grammar contract.
- `../crates/one-core/tests/conformance.rs` — executable conformance runner derived from the fixture contract.
- `../crates/one-cli/tests/cli.rs` — CLI integration tests derived from the CLI contract.
- `../crates/one-core/tests/process_orchestration.rs` — source-to-runtime process-execution TDD tests.
- `../crates/one-cli/tests/process_orchestration.rs` — CLI process-execution integration tests.
- `../crates/one-core/tests/native_backend_linux_windows.rs` — native Linux/Windows backend contract and regression tests.
- `../crates/one-cli/tests/native_cli.rs` — native target CLI regression tests.
- `../tests/conformance/valid/*.one` — accepted-language fixtures.
- `../tests/conformance/invalid/*.one` — rejected-language fixtures.
- `../tests/golden/*.snap` — deterministic execution outputs.
- `../crates/one-core/tests/network_foundation.rs` — executable One Network foundation tests.
- `../crates/one-core/tests/orchestration_runtime.rs` — executable orchestration lifecycle tests.
- `../tools/check_contract.py` — static contract-artifact consistency checker.
- `../tools/verify_offline.sh` — independent Linux verification path.
- `../tools/verify_offline.ps1` — independent Windows verification path.
- `../.circleci/config.yml` — canonical Linux product CI and opt-in Linux release checks.
- [Independent Windows verification repository](https://github.com/panglimanetizen01/one-windows-verification) — Windows Server 2025 native build and exact-SHA execution evidence.

## CI and developer operations

- [CI migration contract on PR #95](https://github.com/panglimanetizen01/one/blob/ci/circleci-migration-20261113/docs/CI.md) — CircleCI Linux-only provider boundary, required product checks, opt-in Linux release checks, and independent Windows exact-SHA verification.
- [`verification/README.md`](verification/README.md) — ownership and evidence rules for canonical CI and independent verification artifacts.
- [`verification/windows-native-verification.md`](verification/windows-native-verification.md) — independent Windows-host native artifact verification contract.
- [CircleCI config on PR #95](https://github.com/panglimanetizen01/one/blob/ci/circleci-migration-20261113/.circleci/config.yml) — canonical Linux product CI and opt-in Linux release checks.

## Canonical rule

GitHub `main` is the canonical source. Local/Termux copies are verification environments only. Documentation must never claim green cross-platform execution without actual evidence from a successful Linux and Windows run. Native Linux AArch64 is currently verified on the exact PR #64 SHA within the AArch64/Linux host boundary; Linux x86_64 and Windows x86_64 are currently build-proven but lack target execution evidence. The process/system and other source-level orchestration capabilities retain their separately recorded evidence boundaries.

## CI and verification infrastructure

- [CI and verification evidence contract](spec/one-v0.1-ci-verification-contract.md) — normative classification rules for executed failures, toolchain bootstrap failures, runner blockers, plan-credit blockers, and exact-SHA evidence.
- [CircleCI infrastructure incident record — 2026-11-16](verification/circleci-infrastructure-incident-2026-11-16.md) — forensic evidence for historical MSYS2 job #254, explicit no-credit jobs #409/#424/#426, pre-run jobs #407/#408 with cause unconfirmed, and independent Windows run #21.

These are development/tooling records. They do not alter ONE language semantics or user-facing CLI behavior.
