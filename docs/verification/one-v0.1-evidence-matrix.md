# One v0.1 Evidence Matrix

This matrix ties each release claim to a canonical artifact. A claim is not considered verified merely because an implementation file exists.

| Requirement | Canonical artifact | Evidence required | Current state |
|---|---|---|---|
| Lexical rules | `docs/spec/one-v0.1-lexical.md` | lexer tests + negative lexical fixtures | IMPLEMENTED / execution pending |
| Concrete grammar | `docs/spec/one-v0.1-grammar.md` | parser tests + valid/invalid fixtures | IMPLEMENTED / execution pending |
| AST | `docs/spec/one-v0.1-ast.md`, `crates/one-core/src/ast.rs` | AST parser tests | IMPLEMENTED / execution pending |
| Semantic analysis | `crates/one-core/src/semantic.rs` | scope/type tests | IMPLEMENTED / execution pending |
| Semantic IR | `crates/one-core/src/ir.rs` | IR tests | IMPLEMENTED / execution pending |
| Reference interpreter | `crates/one-core/src/interpreter.rs` | core execution tests + golden tests | IMPLEMENTED / execution pending |
| Diagnostics | `docs/spec/one-v0.1-diagnostics.md`, `diagnostic.rs` | stable diagnostic-code tests | IMPLEMENTED / execution pending |
| Capability boundary | `capability.rs`, `platform.rs`, `SECURITY.md` | denied-capability tests | IMPLEMENTED / execution pending |
| Standard library | `docs/spec/one-v0.1-stdlib.md`, `stdlib.rs` | behavior tests | IMPLEMENTED / execution pending |
| Process interoperability | `docs/spec/one-v0.1-interop.md`, `interop.rs` | portable process-boundary tests | IMPLEMENTED / execution pending |
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli.md` | CLI integration + smoke tests | IMPLEMENTED / execution pending |
| Contract consistency | `tools/check_contract.py` | required-artifact/index/fixture scan | ADDED / execution pending |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | ADDED / execution pending |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | ADDED / execution pending |
| Linux portability | `.github/workflows/ci.yml` + offline verifier | actual Linux run | BLOCKED by hosted runner evidence |
| Windows portability | `.github/workflows/ci.yml` + offline verifier | actual Windows run | BLOCKED by hosted runner evidence |
| Release gate | `.github/workflows/release-check.yml` | green gate run | PENDING |

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification may provide independent execution evidence when hosted CI cannot run; its environment, toolchain, command, commit SHA, and result must be recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
