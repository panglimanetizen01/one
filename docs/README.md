# One documentation map

This directory is the normative documentation surface for One v0.1. When code and prose disagree, the implementation status document must identify the discrepancy; a silent contract change is not allowed.

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
- `verification/forensic-audit-2026-11-17.md` — repository-wide forensic inventory, documentation/CI drift, exact-SHA evidence, and open security gates.
- `verification/MASTER_CONTINUATION_PROMPT_2026-11-17.md` — historical long-form prompt snapshot, superseded as the session entry point by `verification/MASTER_CONTINUATION_PROMPT.md`; preserve for history.
- `spec/one-v0.1-ci-verification-contract.md` — normative development-side CI and exact-SHA evidence contract.

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
- `../.circleci/config.yml` — candidate CircleCI Linux CI and opt-in release workflow (active only on the migration candidate branch until approved cutover).
- `verification/windows-native-verification.md` — Windows native verification contract for the independent verifier repository.

## Canonical rule

GitHub `main` is the canonical source. Local/Termux copies are verification environments only. PR #113 remains OPEN / DRAFT / UNMERGED at exact HEAD `5daf3337b4bcc93fa3ee6072f9633915d5fea85b` (base `181ad350d8f0db53cda654c4d378425fbdd6ab24`). Code-bearing source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` has CircleCI SUCCESS; current docs HEAD has CircleCI FAILURE, job #833, step #101 CANCELED, root cause UNKNOWN. The separate `main` pipeline #399 is ERROR, cause UNKNOWN. See the appended exact-SHA live status in the verification records. The CircleCI config remains outside `main` (`d35db95153f6ddd79885494c72f5aee16c899751`). The full architecture image remains in draft PR #97 and its binary has not been visually inspected. Documentation must never claim green cross-platform execution without exact-SHA Linux and Windows evidence. Native Linux AArch64, Linux x86_64, and Windows x86_64 claims remain bounded by their respective evidence records.

## Security design checkpoints

- `security/issue-104-filesystem-root-confinement-design.md` — normative design and acceptance contract for trusted-root handle authority, Unix/Windows handle-relative traversal, bounded I/O, atomic writes, and the Issue #104 adversarial matrix. The candidate implementation is tracked separately; the document itself is not a security PASS.


## Architecture diagrams

- [Filesystem root-confinement candidate diagram](architecture/filesystem-root-confinement.md) — source Mermaid view for the host-injected filesystem authority and platform-specific read/write boundaries in draft PR #113.
- [Full architecture map](https://github.com/panglimanetizen01/one/blob/docs/system-architecture-diagram-20261115/docs/architecture/one-full-architecture-map.png) — non-normative overview image in still-open PR #97. Its binary image has not been visually inspected in this session.

## Release execution and session continuity

- `verification/ONE_RELEASE_BLOCKER_REGISTER.md` — single prioritized release blocker queue.
- `verification/MASTER_CONTINUATION_PROMPT.md` — compact copy/paste entry point for every new session.
- The register links the existing release contract, completion roadmap, evidence matrix, implementation status, verification records, Mermaid architecture source, and changelog. It controls priority only; those linked documents remain authoritative for acceptance.

## Superseding PR #113 filesystem checkpoint — 2026-11-21

The canonical-rule snapshot above is historical. Current PR #113 remains OPEN / DRAFT / UNMERGED on branch security/issue-104-root-confinement-20261118. Language-bridge source SHA is 0fa43eb0be53c8e79424597ece055147872b35c4; subsequent commits in the branch update documentation only. The Windows native delete implementation is separately verified at 6f2d5e2d0b02d6aed6c692a8a7b7fa184a70fcb0 (One CI #44870781889 SUCCESS). The language bridge adds the explicit filesystem.delete permission and Indonesian source form "hapus berkas \"path\"", but remains pending exact-SHA Windows and CircleCI results at the time of this note. Issue #104 remains OPEN / NOT ACCEPTED.

The current filesystem design and language effect path are documented in:
- security/issue-104-filesystem-root-confinement-design.md
- architecture/filesystem-root-confinement.md
- spec/one-v0.1-effect-capability.md
- verification/ONE_RELEASE_BLOCKER_REGISTER.md
