# One v0.1 Evidence Matrix

This matrix ties each release claim to a canonical artifact. A claim is not considered verified merely because an implementation file exists.

| Requirement | Canonical artifact | Evidence required | Current state |
|---|---|---|---|
| Lexical rules | `docs/spec/one-v0.1-lexical.md` | lexer tests + negative lexical fixtures | IMPLEMENTED / independently exercised by the verification matrix |
| Concrete grammar | `docs/spec/one-v0.1-grammar.md` | parser tests + valid/invalid fixtures | IMPLEMENTED / independently exercised by the verification matrix |
| Normative grammar examples | `tests/spec/grammar_examples.one` | fixture scan + parser/conformance execution | ADDED / independently exercised by the verification matrix |
| AST | `docs/spec/one-v0.1-ast.md`, `crates/one-core/src/ast.rs` | AST parser tests | IMPLEMENTED / independently exercised by the verification matrix |
| Semantic analysis | `crates/one-core/src/semantic.rs` | scope/type tests | IMPLEMENTED / independently exercised by the verification matrix |
| Semantic IR | `crates/one-core/src/ir.rs` | IR tests | IMPLEMENTED / independently exercised by the verification matrix |
| Reference interpreter | `crates/one-core/src/interpreter.rs` | core execution tests + golden tests | IMPLEMENTED / independently exercised by the verification matrix |
| Diagnostics | `docs/spec/one-v0.1-diagnostics.md`, `diagnostic.rs` | stable diagnostic-code tests | IMPLEMENTED / independently exercised by the verification matrix |
| Capability boundary | `capability.rs`, `platform.rs`, `SECURITY.md` | denied-capability tests | IMPLEMENTED / independently exercised by the verification matrix |
| Standard library | `docs/spec/one-v0.1-stdlib.md`, `stdlib.rs` | behavior tests | IMPLEMENTED / independently exercised by the verification matrix |
| Process interoperability | `docs/spec/one-v0.1-interop.md`, `interop.rs` | portable process-boundary tests | IMPLEMENTED / independently exercised by the verification matrix |
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli.md` | CLI integration + smoke tests | IMPLEMENTED / independently exercised by the verification matrix |
| Contract consistency | `tools/check_contract.py` | required-artifact/index/fixture scan | ADDED / independently exercised by the verification matrix |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | ADDED / independently exercised by the verification matrix |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | ADDED / independently exercised by the verification matrix |
| Linux portability | `.github/workflows/ci.yml` + offline verifier | actual Linux run | GREEN for current canonical SHA `38dc98fcee49fa04c589d5a3c825b0d4006d6d61` in independent run `38848143359` |
| Windows portability | `.github/workflows/ci.yml` + offline verifier | actual Windows run | GREEN for current canonical SHA `38dc98fcee49fa04c589d5a3c825b0d4006d6d61` in independent run `38848143359` |
| Release gate | `.github/workflows/release-check.yml` | green gate run | GREEN for current canonical SHA via independent matrix `38848143359` |

## Final verification boundary

The current canonical source revision is `38dc98fcee49fa04c589d5a3c825b0d4006d6d61`. Independent run `38848143359` checked that exact SHA on Ubuntu and Windows Server 2025. Both jobs passed canonical SHA verification, contract artifacts, build, Clippy, workspace tests, real-network runtime, and platform-specific offline verification. The real-network suite passed on both platforms.

The prior source revision `e58da6393e53094f29a5121cb440c94973f71b6a` remains historical evidence from run `38846301365`.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification may provide independent execution evidence when hosted CI cannot run; its environment, toolchain, command, commit SHA, and result must be recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits; the final release candidate must be re-verified after canonical changes.
