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
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli.md` | CLI integration + smoke tests | IMPLEMENTED / independently exercised |
| Contract consistency | `tools/check_contract.py` | artifact/index/fixture scan | VERIFIED by independent run |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | IMPLEMENTED / independently exercised |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | IMPLEMENTED / independently exercised |
| Deterministic `.oneir` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two builds + byte comparison | VERIFIED on exact SHA |
| Deterministic `.onepkg` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two packages + byte comparison | VERIFIED on exact SHA |
| Orchestration domain/governance | `orchestration.rs`, `governance.rs`, `orchestrator.rs` | unit/integration tests | IMPLEMENTED / independently exercised |
| Orchestration adapter boundary | `adapter.rs`, `network_dns_adapter.rs` | adapter tests + real-network runtime | VERIFIED on exact SHA |
| Orchestration audit/result normalization | `orchestration_audit.rs`, `orchestration_result.rs` | audit/normalization tests | IMPLEMENTED / independently exercised |
| Language-level orchestration | parser → semantic → capability → governance → adapter → One result | end-to-end One-source execution test | NOT YET VERIFIED / next implementation boundary |
| Linux portability | verification mirror workflow | actual Linux run | GREEN in independent run `38871435579` |
| Windows portability | verification mirror workflow | actual Windows run | GREEN in independent run `38871435579` |
| Release gate | `.github/workflows/release-check.yml` + mirror workflow | green exact-SHA matrix | GREEN in independent run `38871435579` |

## Final verification boundary

Independent run `38871435579` checked exact canonical source SHA `be2e65852e9a4be833e659524a12c6359769665b` on Ubuntu and Windows Server 2025. Both jobs passed canonical SHA verification, contract artifacts, build, Clippy, workspace tests, real-network runtime tests, and platform-specific offline verification. The offline scripts also passed deterministic `.oneir` and `.onepkg` checks.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification is valid independent execution evidence only when its environment, toolchain, command, exact commit, and result are recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits.
- Implemented orchestration runtime foundation does not imply language-level One-source orchestration support.
