# One v0.1 Evidence Matrix

This matrix ties each release claim to a canonical artifact. A claim is not considered verified merely because an implementation file exists.

| Requirement | Canonical artifact | Evidence required | Current state |
|---|---|---|---|
| Lexical rules | `docs/spec/one-v0.1-lexical.md` | lexer tests + negative lexical fixtures | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Concrete grammar | `docs/spec/one-v0.1-grammar.md` | parser tests + valid/invalid fixtures | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Normative grammar examples | `tests/spec/grammar_examples.one` | fixture scan + parser/conformance execution | ADDED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| AST | `docs/spec/one-v0.1-ast.md`, `crates/one-core/src/ast.rs` | AST parser tests | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Semantic analysis | `crates/one-core/src/semantic.rs` | scope/type tests | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Semantic IR | `crates/one-core/src/ir.rs` | IR tests | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Reference interpreter | `crates/one-core/src/interpreter.rs` | core execution tests + golden tests | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Diagnostics | `docs/spec/one-v0.1-diagnostics.md`, `diagnostic.rs` | stable diagnostic-code tests | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Capability boundary | `capability.rs`, `platform.rs`, `SECURITY.md` | denied-capability tests | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Standard library | `docs/spec/one-v0.1-stdlib.md`, `stdlib.rs` | behavior tests | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Process interoperability | `docs/spec/one-v0.1-interop.md`, `interop.rs` | portable process-boundary tests | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli.md` | CLI integration + smoke tests | IMPLEMENTED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Contract consistency | `tools/check_contract.py` | required-artifact/index/fixture scan | ADDED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | ADDED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | ADDED / Linux execution evidenced on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Linux portability | `.github/workflows/ci.yml` + offline verifier | actual Linux run | INDEPENDENTLY EVIDENCED on `db9af51149596a8b7f2a38d0a157c2ccf6f3e4a0` |
| Windows portability | `.github/workflows/ci.yml` + offline verifier | actual Windows run | PENDING / hosted runner blocked; independent Windows run still required |
| Release gate | `.github/workflows/release-check.yml` | green gate run | PENDING |

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification may provide independent execution evidence when hosted CI cannot run; its environment, toolchain, command, commit SHA, and result must be recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits; the final release candidate must be re-verified after canonical changes.
