# One v0.1 Evidence Matrix

This matrix ties each release claim to a canonical artifact. A claim is not considered verified merely because an implementation file exists.

| Requirement | Canonical artifact | Evidence required | Current state |
|---|---|---|---|
| Lexical rules | `docs/spec/one-v0.1-lexical.md` | lexer tests + negative lexical fixtures | IMPLEMENTED / independently exercised |
| Concrete grammar | `docs/spec/one-v0.1-grammar.md` | parser tests + valid/invalid fixtures | IMPLEMENTED / independently exercised |
| AST | `docs/spec/one-v0.1-ast.md`, `crates/one-core/src/ast.rs` | AST parser tests | IMPLEMENTED / independently exercised |
| Semantic analysis | `crates/one-core/src/semantic.rs` | scope/type tests | IMPLEMENTED / independently exercised |
| Semantic IR | `crates/one-core/src/ir.rs` | IR tests | IMPLEMENTED / independently exercised |
| Reference interpreter | `crates/one-core/src/interpreter.rs` | core execution + golden tests | IMPLEMENTED / independently exercised |
| Diagnostics | `docs/spec/one-v0.1-diagnostics.md`, `diagnostic.rs` | diagnostic-code tests | IMPLEMENTED / independently exercised |
| Capability boundary | `capability.rs`, `platform.rs`, `SECURITY.md` | denied-capability tests | IMPLEMENTED / independently exercised |
| Standard library | `docs/spec/one-v0.1-stdlib.md`, `stdlib.rs` | behavior tests | IMPLEMENTED / independently exercised |
| Process interoperability | `docs/spec/one-v0.1-interop.md`, `interop.rs` | process-boundary tests | IMPLEMENTED / independently exercised |
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli-contract.md` | CLI integration + smoke tests | VERIFIED in independently exercised revision `f482c6b75505d9965dd33f4f25bb0cd04072cedb`; newer source changes are pending exact-SHA verification |
| Contract consistency | `tools/check_contract.py` | artifact/index/fixture scan | VERIFIED in independent run `38903950612` against `f482c6b75505d9965dd33f4f25bb0cd04072cedb` |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | IMPLEMENTED / independently exercised |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | IMPLEMENTED / independently exercised |
| Deterministic `.oneir` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two builds + byte comparison | VERIFIED in independent run `38903950612` against `f482c6b75505d9965dd33f4f25bb0cd04072cedb` |
| Deterministic `.onepkg` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two packages + byte comparison | VERIFIED in independent run `38903950612` against `f482c6b75505d9965dd33f4f25bb0cd04072cedb` |
| Orchestration domain/governance | `orchestration.rs`, `governance.rs`, `orchestrator.rs` | unit/integration tests | VERIFIED in independent run `38903950612` against `f482c6b75505d9965dd33f4f25bb0cd04072cedb` |
| Orchestration adapter boundary | `adapter.rs`, `network_dns_adapter.rs` | adapter tests + real-network runtime | VERIFIED in independent run `38903950612` against `f482c6b75505d9965dd33f4f25bb0cd04072cedb` |
| Orchestration audit/result normalization | `orchestration_audit.rs`, `orchestration_result.rs` | audit/normalization tests | VERIFIED in independent run `38903950612` against `f482c6b75505d9965dd33f4f25bb0cd04072cedb` |
| Language-level DNS orchestration | parser → semantic → typed IR → governance → DNS adapter → normalized result → CLI | end-to-end One-source execution + explicit permission test | EXPERIMENTAL / VERIFIED in independent run `38903950612` against `f482c6b75505d9965dd33f4f25bb0cd04072cedb` |
| Language-level filesystem-read orchestration | parser → semantic → typed IR → governance → filesystem.read adapter → normalized result → CLI | end-to-end One-source execution + allow/deny/out-of-scope/backend/unavailable-adapter tests | EXPERIMENTAL / VERIFIED in independent run `38903950612` against `f482c6b75505d9965dd33f4f25bb0cd04072cedb` |
| Language-level process execution orchestration | parser → semantic → typed IR → governance → process.execute adapter → normalized result → CLI | end-to-end One-source execution + explicit permission + denial + backend-failure tests | IN DEVELOPMENT / implementation and TDD tests present; exact-SHA execution evidence pending |
| DNS success | `network_dns_adapter.rs` | real resolver success test | VERIFIED in independent run `38903950612` |
| DNS failure classification | `network_dns_adapter.rs` | resolution-failure classification test | VERIFIED in independent run `38903950612` |
| Filesystem-read policy denial | `crates/one-cli/tests/cli.rs` + orchestration runtime tests | no-permission execution must fail with denial diagnostic | VERIFIED in independent run `38903950612` |
| Filesystem-read out-of-scope | `crates/one-cli/tests/cli.rs` + governance tests | target mismatch must be denied | VERIFIED in independent run `38903950612` |
| Filesystem-read backend failure/unavailable adapter | filesystem orchestration tests | backend failure and unavailable-adapter paths must fail deterministically | VERIFIED in independent run `38903950612` |
| Process-execution policy denial/backend failure | `crates/one-core/tests/process_orchestration.rs` + `crates/one-cli/tests/process_orchestration.rs` | no-permission denial and missing-program backend failure | IMPLEMENTED / TDD tests present; execution evidence pending |
| Broader language-level orchestration | same source-to-runtime chain for additional capabilities | per-capability end-to-end tests | IN DEVELOPMENT |
| Linux portability | verification mirror workflow | actual Linux run | GREEN in independent run `38903950612` for `f482c6b75505d9965dd33f4f25bb0cd04072cedb` |
| Windows portability | verification mirror workflow | actual Windows run | GREEN in independent run `38903950612` for `f482c6b75505d9965dd33f4f25bb0cd04072cedb` |
| Release gate | verification mirror workflow | green exact-SHA matrix | GREEN in independent run `38903950612` for `f482c6b75505d9965dd33f4f25bb0cd04072cedb`; newer source changes still require exact-SHA verification |

## Current exact-SHA evidence

Independent run `38903950612` in `panglimanetizen01/one-windows-verification` checked out canonical One source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` exactly. Ubuntu and Windows Server 2025 both passed canonical SHA assertion, contract checks, build, Clippy, workspace tests, real-network runtime tests, deterministic artifact checks, DNS success/failure classification, filesystem-read orchestration runtime, and platform-specific offline verification.

Canonical `main` is now `ab7df1e2a0dbcc8c3fab3b425137194d1ca0ccbe`. The merge from the documentation synchronization PR is documentation-only relative to the pre-merge `main`, but this does not by itself constitute an exact-SHA executable verification for the new `main` commit. The process/system vertical-slice branch also contains source changes and therefore requires its own exact-SHA executable evidence before merge or promotion.

The available mirror branches currently document the canonical SHA boundary, but the mirror itself is explicitly marked non-executable until its source tree exactly matches the recorded canonical commit. That is an infrastructure/evidence gap, not source-pass evidence.

## Canonical CI status

PR #48 produced GitHub Actions run `38927364159` against head `90086153f01275bbc2cc968c66c9646a525014e0`. Both Ubuntu job `117062224733` and Windows job `117062225163` failed with zero job steps; retrying the failed jobs produced queued jobs `117062384884` and `117062384610`, and both ultimately failed again with no steps/logs. This is an infrastructure evidence failure, not evidence of a Rust source failure. The current branch therefore still lacks executable test evidence.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification is valid independent execution evidence only when its environment, toolchain, command, exact commit, and result are recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits.
- The DNS and filesystem-read source-level slices are experimental and do not imply that all orchestration capabilities are supported.
- The process-execution slice remains `IN DEVELOPMENT` until an exact-SHA executable run proves the full source-to-runtime path.
