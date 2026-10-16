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
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli-contract.md` | CLI integration + smoke tests | VERIFIED in independently exercised revision `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| Contract consistency | `tools/check_contract.py` | artifact/index/fixture scan | VERIFIED in independent run `38901366447` against `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | IMPLEMENTED / independently exercised |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | IMPLEMENTED / independently exercised |
| Deterministic `.oneir` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two builds + byte comparison | VERIFIED in independent run `38901366447` against `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| Deterministic `.onepkg` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two packages + byte comparison | VERIFIED in independent run `38901366447` against `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| Orchestration domain/governance | `orchestration.rs`, `governance.rs`, `orchestrator.rs` | unit/integration tests | VERIFIED in independent run `38901366447` against `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| Orchestration adapter boundary | `adapter.rs`, `network_dns_adapter.rs` | adapter tests + real-network runtime | VERIFIED in independent run `38901366447` against `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| Orchestration audit/result normalization | `orchestration_audit.rs`, `orchestration_result.rs` | audit/normalization tests | VERIFIED in independent run `38901366447` against `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| Language-level DNS orchestration | parser → semantic → typed IR → governance → DNS adapter → normalized result → CLI | end-to-end One-source execution + explicit permission test | EXPERIMENTAL / VERIFIED in independent run `38901366447` against `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| Language-level filesystem-read orchestration | parser → semantic → typed IR → governance → filesystem.read adapter → normalized result → CLI | end-to-end One-source execution + allow/deny/out-of-scope/backend/unavailable-adapter tests | EXPERIMENTAL / VERIFIED in independent run `38901366447` against `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| DNS success | `network_dns_adapter.rs` | real resolver success test | VERIFIED in independent run `38901366447` |
| DNS failure classification | `network_dns_adapter.rs` | resolution-failure classification test | VERIFIED in independent run `38901366447` |
| Filesystem-read policy denial | `crates/one-cli/tests/cli.rs` + orchestration runtime tests | no-permission execution must fail with denial diagnostic | VERIFIED in independent run `38901366447` |
| Filesystem-read out-of-scope | `crates/one-cli/tests/cli.rs` + governance tests | target mismatch must be denied | VERIFIED in independent run `38901366447` |
| Filesystem-read backend failure/unavailable adapter | filesystem orchestration tests | backend failure and unavailable-adapter paths must fail deterministically | VERIFIED in independent run `38901366447` |
| Broader language-level orchestration | same source-to-runtime chain for additional capabilities | per-capability end-to-end tests | IN DEVELOPMENT |
| Linux portability | verification mirror workflow | actual Linux run | GREEN in independent run `38901366447` for `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| Windows portability | verification mirror workflow | actual Windows run | GREEN in independent run `38901366447` for `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |
| Release gate | verification mirror workflow | green exact-SHA matrix | GREEN in independent run `38901366447` for `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` |

## Current exact-SHA evidence

Independent run `38901366447` in `panglimanetizen01/one-windows-verification` checked out canonical One source `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b` exactly. Ubuntu and Windows Server 2025 both passed canonical SHA assertion, contract checks, build, Clippy, workspace tests, real-network runtime tests, deterministic artifact checks, DNS success/failure classification, filesystem-read orchestration runtime, and platform-specific offline verification.

The canonical `main` later moved beyond that verified implementation revision. Those later changes currently include documentation/changelog synchronization rather than source-code changes, but this does not satisfy the project's exact-SHA evidence rule. A new independent run is required before calling the newer `main` SHA exactly verified.

## Canonical CI status

The latest canonical `main` push run is `38903908051` for commit `f482c6b75505d9965dd33f4f25bb0cd04072cedb`. GitHub reports both Ubuntu and Windows jobs as failed, but the job-step API returned no steps and log retrieval returned `404 BlobNotFound`. Therefore this run is an observed CI failure/infrastructure evidence gap, not evidence of a Rust source failure. The failure must not be reclassified as a code regression without usable job evidence.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification is valid independent execution evidence only when its environment, toolchain, command, exact commit, and result are recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits.
- The DNS and filesystem-read source-level slices are experimental and do not imply that all orchestration capabilities are supported.
