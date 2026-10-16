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
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli-contract.md` | CLI integration + smoke tests | VERIFIED at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; latest executable verification additionally recorded at `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android |
| Contract consistency | `tools/check_contract.py` | artifact/index/fixture scan | VERIFIED at `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android; also cross-platform at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800` |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | IMPLEMENTED / independently exercised |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | IMPLEMENTED / independently exercised |
| Deterministic `.oneir` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two builds + byte comparison | VERIFIED cross-platform at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; latest executable evidence also passes offline verification on Termux/Android |
| Deterministic `.onepkg` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two packages + byte comparison | VERIFIED cross-platform at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; latest executable evidence also passes offline verification on Termux/Android |
| Orchestration domain/governance | `orchestration.rs`, `governance.rs`, `orchestrator.rs` | unit/integration tests | VERIFIED cross-platform at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; latest executable workspace tests also pass on Termux/Android |
| Orchestration adapter boundary | `adapter.rs`, `network_dns_adapter.rs` | adapter tests + runtime | VERIFIED cross-platform at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; filesystem.write adapter also passes latest executable tests on Termux/Android |
| Orchestration audit/result normalization | `orchestration_audit.rs`, `orchestration_result.rs` | audit/normalization tests | VERIFIED cross-platform at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; latest executable workspace tests pass on Termux/Android |
| Language-level DNS orchestration | parser → semantic → typed IR → governance → DNS adapter → normalized result → CLI | end-to-end source execution + explicit permission | EXPERIMENTAL / cross-platform evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800` |
| Language-level filesystem-read orchestration | parser → semantic → typed IR → governance → filesystem.read adapter → normalized result → CLI | end-to-end source execution + allow/deny/out-of-scope/backend tests | EXPERIMENTAL / cross-platform evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800` |
| Language-level process execution orchestration | parser → semantic → typed IR → governance → process.execute adapter → normalized result → CLI | end-to-end source execution + permission + denial + backend-failure tests | EXPERIMENTAL / cross-platform evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; one targeted program without arguments |
| Language-level filesystem-write orchestration | parser → semantic → typed IR → governance → filesystem.write adapter → CLI | source execution + explicit permission + exact-target write + denial | EXPERIMENTAL / latest executable evidence `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android; CLI regression test and core runtime test PASS; cross-platform evidence pending |
| Filesystem-write CLI regression | `crates/one-cli/tests/filesystem_write.rs` | source file + `--izinkan filesystem.write` + actual target content | VERIFIED on `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android; 1 targeted test PASS |
| Filesystem-write policy/runtime | `crates/one-core/tests/filesystem_write_runtime.rs` | default denial + target-scoped grant + exact text output | VERIFIED in latest executable evidence on Termux/Android; workspace test PASS |
| Broader language-level orchestration | same source-to-runtime chain for additional capabilities | per-capability end-to-end tests | IN DEVELOPMENT |
| Linux portability | verification mirror workflow | actual Linux run | GREEN for `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; current filesystem-write executable evidence not yet cross-platform verified |
| Windows portability | verification mirror workflow | actual Windows run | GREEN for `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; current filesystem-write executable evidence not yet cross-platform verified |
| Release gate | verification mirror workflow | green exact-SHA matrix | GREEN for `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; latest executable evidence is independent Termux evidence only |

## Current exact-SHA evidence

Latest exact-SHA executable evidence for the current executable boundary is `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8`. The exact-SHA Termux/Android verification passed contract checks, build, Clippy `-D warnings`, targeted filesystem-write CLI test, full workspace tests, and offline verification.

The latest cross-platform exact-SHA evidence remains mirror run `38929072800` for source `9038ac75f197753ff1ab6af7478e63e4831903a2` on Ubuntu and Windows Server 2025. It predates the filesystem-write commits and is not generalized to newer SHAs.

## Canonical CI status

The filesystem-write CLI PR's hosted GitHub Actions execution produced zero-step job failures and retries also produced no executable jobs. This is infrastructure evidence, not evidence of a Rust source failure. The Termux verification is therefore the executable evidence for the current filesystem-write CLI boundary.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification is valid independent execution evidence only when its environment, toolchain, command, exact commit, and result are recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits.
- Filesystem write is experimental and must not be generalized from Android/Termux evidence to Ubuntu/Windows support.
- The process-execution slice is `EXPERIMENTAL` only within its cross-platform verified boundary: one explicitly targeted program without arguments and explicit `process.execute` permission.
