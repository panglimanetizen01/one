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
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli-contract.md` | CLI integration + smoke tests | VERIFIED in exact-SHA independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Contract consistency | `tools/check_contract.py` | artifact/index/fixture scan | VERIFIED in exact-SHA independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | IMPLEMENTED / independently exercised |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | IMPLEMENTED / independently exercised |
| Deterministic `.oneir` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two builds + byte comparison | VERIFIED in exact-SHA independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Deterministic `.onepkg` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two packages + byte comparison | VERIFIED in exact-SHA independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Orchestration domain/governance | `orchestration.rs`, `governance.rs`, `orchestrator.rs` | unit/integration tests | VERIFIED in exact-SHA independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Orchestration adapter boundary | `adapter.rs`, `network_dns_adapter.rs` | adapter tests + real-network runtime | VERIFIED in exact-SHA independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Orchestration audit/result normalization | `orchestration_audit.rs`, `orchestration_result.rs` | audit/normalization tests | VERIFIED in exact-SHA independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Language-level DNS orchestration | parser → semantic → typed IR → governance → DNS adapter → normalized result → CLI | end-to-end One-source execution + explicit permission test | EXPERIMENTAL / VERIFIED in independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Language-level filesystem-read orchestration | parser → semantic → typed IR → governance → filesystem.read adapter → normalized result → CLI | end-to-end One-source execution + allow/deny/out-of-scope/backend/unavailable-adapter tests | EXPERIMENTAL / VERIFIED in independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Language-level process execution orchestration | parser → semantic → typed IR → governance → process.execute adapter → normalized result → CLI | end-to-end One-source execution + explicit permission + denial + backend-failure tests | EXPERIMENTAL / VERIFIED in exact-SHA independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2`; intentionally limited to one targeted program without arguments |
| DNS success | `network_dns_adapter.rs` | real resolver success test | VERIFIED in independent run `38929072800` |
| DNS failure classification | `network_dns_adapter.rs` | resolution-failure classification test | VERIFIED in independent run `38929072800` |
| Filesystem-read policy denial | `crates/one-cli/tests/cli.rs` + orchestration runtime tests | no-permission execution must fail with denial diagnostic | VERIFIED in independent run `38929072800` |
| Filesystem-read out-of-scope | `crates/one-cli/tests/cli.rs` + governance tests | target mismatch must be denied | VERIFIED in independent run `38929072800` |
| Filesystem-read backend failure/unavailable adapter | filesystem orchestration tests | backend failure and unavailable-adapter paths must fail deterministically | VERIFIED in independent run `38929072800` |
| Process-execution policy denial/backend failure | `crates/one-core/tests/process_orchestration.rs` + `crates/one-cli/tests/process_orchestration.rs` | no-permission denial and missing-program backend failure | VERIFIED in exact-SHA independent run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Broader language-level orchestration | same source-to-runtime chain for additional capabilities | per-capability end-to-end tests | IN DEVELOPMENT |
| Linux portability | verification mirror workflow | actual Linux run | GREEN in independent run `38929072800` for `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Windows portability | verification mirror workflow | actual Windows run | GREEN in independent run `38929072800` for `9038ac75f197753ff1ab6af7478e63e4831903a2` |
| Release gate | verification mirror workflow | green exact-SHA matrix | GREEN in independent run `38929072800` for `9038ac75f197753ff1ab6af7478e63e4831903a2` |

## Current exact-SHA evidence

Independent run `38929072800` in `panglimanetizen01/one-windows-verification` checked out canonical One source `9038ac75f197753ff1ab6af7478e63e4831903a2` exactly. Ubuntu and Windows Server 2025 both passed canonical SHA assertion, contract checks, build, Clippy, workspace tests, real-network runtime tests, and platform-specific offline verification. The Windows job explicitly reported Microsoft Windows Server 2025.

PR #48 was then merged into canonical `main` as merge commit `123df544f15d7eec8d74ca231b5e55ccebf4a7ee`, whose second parent is the independently verified source commit `9038ac75f197753ff1ab6af7478e63e4831903a2`. Documentation-only synchronization commits now follow that merge and require their own exact-SHA executable verification before they can be used as new executable evidence.

## Canonical CI status

PR #48 produced GitHub Actions run `38927364159` against an earlier head. Both Ubuntu and Windows jobs failed before executing any steps and a retry produced the same zero-step infrastructure failure. This is infrastructure evidence, not evidence of a Rust source failure. The independent verification mirror, not that failed hosted run, is the executable evidence for the process/system source revision.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification is valid independent execution evidence only when its environment, toolchain, command, exact commit, and result are recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits.
- The DNS and filesystem-read source-level slices are experimental and do not imply that all orchestration capabilities are supported.
- The process-execution slice is `EXPERIMENTAL` only within its verified boundary: one explicitly targeted program without arguments and explicit `process.execute` permission.
