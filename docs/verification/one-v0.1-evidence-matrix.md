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
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli-contract.md` | CLI integration + smoke tests | VERIFIED historically at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; latest executable verification additionally recorded at `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android; newer database CLI changes are unverified |
| Contract consistency | `tools/check_contract.py` | artifact/index/fixture scan | VERIFIED historically at `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android and at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; not current-head evidence |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | IMPLEMENTED / independently exercised |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | IMPLEMENTED / independently exercised |
| Deterministic `.oneir` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two builds + byte comparison | VERIFIED historically cross-platform at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; latest executable evidence also passes offline verification on Termux/Android |
| Deterministic `.onepkg` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two packages + byte comparison | VERIFIED historically cross-platform at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; latest executable evidence also passes offline verification on Termux/Android |
| Orchestration domain/governance | `orchestration.rs`, `governance.rs`, `orchestrator.rs` | unit/integration tests | VERIFIED historically cross-platform at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; newer database changes require re-verification |
| Orchestration adapter boundary | `adapter.rs`, `network_dns_adapter.rs`, `database.rs` | adapter tests + runtime | VERIFIED historically for existing adapters; SQLite adapter is newly implemented and UNVERIFIED at current head |
| Orchestration audit/result normalization | `orchestration_audit.rs`, `orchestration_result.rs` | audit/normalization tests | VERIFIED historically cross-platform at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; database normalization test exists but current-head execution is unverified |
| Language-level DNS orchestration | parser → semantic → typed IR → governance → DNS adapter → normalized result → CLI | end-to-end source execution + explicit permission | EXPERIMENTAL / cross-platform evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800` |
| Language-level filesystem-read orchestration | parser → semantic → typed IR → governance → filesystem.read adapter → normalized result → CLI | end-to-end source execution + allow/deny/out-of-scope/backend tests | EXPERIMENTAL / cross-platform evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800` |
| Language-level process execution orchestration | parser → semantic → typed IR → governance → process.execute adapter → normalized result → CLI | end-to-end source execution + permission + denial + backend-failure tests | EXPERIMENTAL / cross-platform evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; one targeted program without arguments |
| Language-level filesystem-write orchestration | parser → semantic → typed IR → governance → filesystem.write adapter → CLI | source execution + explicit permission + exact-target write + denial | EXPERIMENTAL / latest executable evidence `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android; CLI regression test and core runtime test PASS; cross-platform evidence pending |
| Language-level database-query orchestration | parser → semantic → typed IR → resource-scoped governance → `database.sqlite.query` → normalized result → CLI | source execution + explicit permission + out-of-scope denial + backend failure + deterministic normalization | IMPLEMENTED / UNVERIFIED at PR #57 head `0aa4acdf3796235eac432138fec829b6508879e3` |
| Database-query CLI regression | `crates/one-cli/tests/database_orchestration.rs` | source + `--izinkan database.query` + matching resource + denial/scope tests | IMPLEMENTED / UNVERIFIED; current hosted CI has no executable steps |
| Filesystem-write CLI regression | `crates/one-cli/tests/filesystem_write.rs` | source file + `--izinkan filesystem.write` + actual target content | VERIFIED historically on `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android; newer commits require re-verification |
| Filesystem-write policy/runtime | `crates/one-core/tests/filesystem_write_runtime.rs` | default denial + target-scoped grant + exact text output | VERIFIED historically in Termux/Android evidence; newer commits require re-verification |
| Broader language-level orchestration | same source-to-runtime chain for additional capabilities | per-capability end-to-end tests | IN DEVELOPMENT |
| Linux portability | verification mirror workflow | actual Linux run | GREEN historically for `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; current PR #57 head has no executable hosted job evidence |
| Windows portability | verification mirror workflow | actual Windows run | GREEN historically for `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; current PR #57 head has no executable hosted job evidence |
| Release gate | verification mirror workflow | green exact-SHA matrix | GREEN historically for `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`; current PR #57 head is not release-verified |

## Current exact-SHA evidence

Current PR #57 head is `0aa4acdf3796235eac432138fec829b6508879e3`. The current hosted GitHub Actions run is #550. Both Ubuntu and Windows jobs failed with zero reported steps, so the run provides no source execution evidence.

The latest successful exact-SHA executable evidence remains `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android. It predates the database implementation. The latest successful cross-platform exact-SHA evidence remains mirror run `38929072800` for source `9038ac75f197753ff1ab6af7478e63e4831903a2` on Ubuntu and Windows Server 2025. It also predates the database implementation.

## Canonical CI status

Current PR #57 hosted CI run #550 produced zero-step failures on both matrix jobs. This is infrastructure evidence, not evidence of a Rust source failure. The implementation therefore remains UNVERIFIED until an independent exact-SHA executable path records checkout, environment, command, and results.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline verification is valid independent execution evidence only when its environment, toolchain, command, exact commit, and result are recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits.
- Filesystem write is experimental and must not be generalized from Android/Termux evidence to Ubuntu/Windows support.
- Database query is implemented but must not be described as verified or supported until current-head executable evidence exists.
- The process-execution slice is `EXPERIMENTAL` only within its cross-platform verified boundary: one explicitly targeted program without arguments and explicit `process.execute` permission.
