# One v0.1 Evidence Matrix

This matrix ties each release claim to a canonical artifact. A claim is not considered verified merely because an implementation file exists.

| Requirement | Canonical artifact | Evidence required | Current state |
|---|---|---|---|
| Lexical rules | `docs/spec/one-v0.1-lexical.md` | lexer tests + negative lexical fixtures | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` |
| Concrete grammar | `docs/spec/one-v0.1-grammar.md` | parser tests + valid/invalid fixtures | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` |
| Normative grammar examples | `tests/spec/grammar_examples.one` | fixture scan + parser/conformance execution | ADDED / Linux execution evidenced on `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` |
| AST | `docs/spec/one-v0.1-ast.md`, `crates/one-core/src/ast.rs` | AST parser tests | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` |
| Semantic analysis | `crates/one-core/src/semantic.rs` | scope/type tests | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` |
| Semantic IR | `crates/one-core/src/ir.rs` | IR tests | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` |
| Reference interpreter | `crates/one-core/src/interpreter.rs` | core execution tests + golden tests | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` |
| Diagnostics | `docs/spec/one-v0.1-diagnostics.md`, `diagnostic.rs` | stable diagnostic-code tests | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` |
| Capability boundary | `capability.rs`, `platform.rs`, `SECURITY.md` | denied-capability tests | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` |
| Standard library | `docs/spec/one-v0.1-stdlib.md`, `stdlib.rs` | behavior tests | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af7088ed7e` |
| Process interoperability | `docs/spec/one-v0.1-interop.md`, `interop.rs` | portable process-boundary tests | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af7088ed7e` |
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli.md` | CLI integration + smoke tests | IMPLEMENTED / Linux execution evidenced on `95e01d2b2fcee6af7088ed7e` |
| Contract consistency | `tools/check_contract.py` | required-artifact/index/fixture scan | ADDED / Linux execution evidenced on `95e01d2b2fcee6af7088ed7e` |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | ADDED / Linux execution evidenced on `95e01d2b2fcee6af7088ed7e` |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | ADDED / Linux execution evidenced on `95e01d2b2fcee6af7088ed7e` |
| Linux portability | `.github/workflows/ci.yml` + offline verifier | actual Linux run | INDEPENDENTLY EVIDENCED on `95e01d2b2fcee6af7088ed7e` |
| Windows portability | `.github/workflows/ci.yml` + offline verifier | actual Windows run | PENDING / hosted runner blocked; independent Windows run still required |
| Release gate | `.github/workflows/release-check.yml` | green gate run | PENDING |

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification may provide independent execution evidence when hosted CI cannot run; its environment, toolchain, command, commit SHA, and result must be recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits; the final release candidate must be re-verified after canonical changes.
