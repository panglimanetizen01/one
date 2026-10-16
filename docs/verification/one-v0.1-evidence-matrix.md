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
| Linux portability | `.github/workflows/ci.yml` + offline verifier | actual Linux run | GREEN in independent run `38786708863` for pre-documentation-sync source |
| Windows portability | `.github/workflows/ci.yml` + offline verifier | actual Windows run | GREEN in independent run `38786708863` for pre-documentation-sync source |
| Release gate | `.github/workflows/release-check.yml` | green gate run | PENDING final exact-SHA verification |

## Final verification boundary

The canonical `main` branch currently points to a documentation-synchronized commit. The last independent matrix run `38786708863` was fully green on Ubuntu and Windows Server 2025, including canonical SHA verification, contract artifacts, build, Clippy, tests, real-network runtime, and platform-specific offline verification, but it executed the preceding canonical source revision.

The current release candidate therefore remains **PENDING FINAL EXACT-SHA VERIFICATION**. The next independent run must check out the exact current `main` SHA and pass every required stage before any `VERIFIED` release claim is made.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification may provide independent execution evidence when hosted CI cannot run; its environment, toolchain, command, commit SHA, and result must be recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits; the final release candidate must be re-verified after canonical changes.
