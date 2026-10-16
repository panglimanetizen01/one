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
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli-contract.md` | CLI integration + smoke tests | VERIFIED in run `38896135846` |
| Contract consistency | `tools/check_contract.py` | artifact/index/fixture scan | VERIFIED in run `38896135846` |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | IMPLEMENTED / independently exercised |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | IMPLEMENTED / independently exercised |
| Deterministic `.oneir` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two builds + byte comparison | VERIFIED in run `38896135846` |
| Deterministic `.onepkg` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two packages + byte comparison | VERIFIED in run `38896135846` |
| Orchestration domain/governance | `orchestration.rs`, `governance.rs`, `orchestrator.rs` | unit/integration tests | VERIFIED in run `38896135846` |
| Orchestration adapter boundary | `adapter.rs`, `network_dns_adapter.rs` | adapter tests + real-network runtime | VERIFIED in run `38896135846` |
| Orchestration audit/result normalization | `orchestration_audit.rs`, `orchestration_result.rs` | audit/normalization tests | VERIFIED in run `38896135846` |
| Language-level DNS orchestration | parser → semantic → typed IR → governance → DNS adapter → normalized result → CLI | end-to-end One-source execution + explicit permission test | EXPERIMENTAL / VERIFIED in run `38896135846` |
| DNS success | `network_dns_adapter.rs` | real resolver success test | VERIFIED in run `38896135846` |
| DNS failure classification | `network_dns_adapter.rs` | resolution-failure classification test | VERIFIED in run `38896135846` |
| Broader language-level orchestration | same source-to-runtime chain for additional capabilities | per-capability end-to-end tests | IN DEVELOPMENT |
| Linux portability | verification mirror workflow | actual Linux run | GREEN in run `38896135846` |
| Windows portability | verification mirror workflow | actual Windows run | GREEN in run `38896135846` |
| Release gate | verification mirror workflow | green exact-SHA matrix | GREEN in run `38896135846` |

## Current exact-SHA evidence

Run `38896135846` in `panglimanetizen01/one-windows-verification` checked out canonical One source `e43b31fe9d7a5f62de4d3863b358aa28241fbf9c` exactly. Ubuntu job `116956935554` and Windows job `116956935967` both passed canonical SHA assertion, contract checks, build, Clippy, workspace tests, real-network runtime tests, and platform-specific offline verification.

The logs explicitly show deterministic `.oneir`/`.onepkg` tests, DNS success and resolution-failure classification tests, and orchestration runtime/source-level DNS tests passing on the relevant platforms.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification is valid independent execution evidence only when its environment, toolchain, command, exact commit, and result are recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits.
- The DNS source-level slice is experimental and does not imply that all orchestration capabilities are supported.
