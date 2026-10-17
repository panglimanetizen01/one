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
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli-contract.md` | CLI integration + smoke tests | VERIFIED historically; newer database/HTTP CLI changes remain unverified |
| Contract consistency | `tools/check_contract.py` | artifact/index/fixture scan | VERIFIED historically; current-head execution pending |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | IMPLEMENTED / independently exercised |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | IMPLEMENTED / independently exercised |
| Deterministic `.oneir` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two builds + byte comparison | VERIFIED historically; current-head execution pending |
| Deterministic `.onepkg` | `tools/verify_offline.sh`, `tools/verify_offline.ps1` | two packages + byte comparison | VERIFIED historically; current-head execution pending |
| Orchestration domain/governance | `orchestration.rs`, `governance.rs`, `orchestrator.rs` | unit/integration tests | VERIFIED historically; database/HTTP changes require current-head re-verification |
| Orchestration adapter boundary | `adapter.rs`, `network_dns_adapter.rs`, `database.rs` | adapter tests + runtime | VERIFIED historically for existing adapters; SQLite and HTTP adapters require current-head execution evidence |
| Orchestration audit/result normalization | `orchestration_audit.rs`, `orchestration_result.rs` | audit/normalization tests | VERIFIED historically; database/HTTP current-head execution pending |
| Language-level DNS orchestration | parser → semantic → typed IR → governance → DNS adapter → normalized result → CLI | end-to-end source execution + explicit permission | EXPERIMENTAL / historically executable |
| Language-level filesystem-read orchestration | parser → semantic → typed IR → governance → filesystem.read adapter → normalized result → CLI | end-to-end source execution + allow/deny/out-of-scope/backend tests | EXPERIMENTAL / historically executable |
| Language-level process execution orchestration | parser → semantic → typed IR → governance → process.execute adapter → normalized result → CLI | source execution + permission + denial + backend-failure tests | EXPERIMENTAL within previously verified boundary |
| Language-level filesystem-write orchestration | parser → semantic → typed IR → governance → filesystem.write adapter → CLI | source execution + explicit permission + exact-target write + denial | EXPERIMENTAL; current-head execution pending |
| Language-level database-query orchestration | parser → semantic → typed IR → resource-scoped governance → `database.sqlite.query` → normalized result → CLI | source execution + explicit permission + out-of-scope denial + backend failure + deterministic normalization | IMPLEMENTED / UNVERIFIED at current head |
| Database-query CLI regression | `crates/one-cli/tests/database_orchestration.rs` | source + `--izinkan database.query` + matching resource + denial/scope tests | IMPLEMENTED / UNVERIFIED at current head |
| Language-level HTTP GET orchestration | parser → semantic → typed IR → target-scoped governance → `network.http.get` → normalized result → CLI | loopback source execution + explicit permission + out-of-scope denial + adapter validation | IMPLEMENTED / UNVERIFIED at current head |
| HTTP GET CLI regression | `crates/one-cli/tests/http_get_orchestration.rs` | real loopback GET + `--izinkan network.connect` + matching target + denial/scope tests | IMPLEMENTED / UNVERIFIED at current head |
| Filesystem-write CLI regression | `crates/one-cli/tests/filesystem_write.rs` | source file + `--izinkan filesystem.write` + actual target content | VERIFIED historically; current-head execution pending |
| Filesystem-write policy/runtime | `crates/one-core/tests/filesystem_write_runtime.rs` | default denial + target-scoped grant + exact text output | VERIFIED historically; current-head execution pending |
| Broader language-level orchestration | same source-to-runtime chain for additional capabilities | per-capability end-to-end tests | IN DEVELOPMENT |
| Linux portability | verification mirror workflow | actual Linux run | GREEN historically; current main has no fresh executable job evidence |
| Windows portability | verification mirror workflow | actual Windows run | GREEN historically; current main has no fresh executable job evidence |
| Release gate | verification mirror workflow | green exact-SHA matrix | GREEN historically; current main is not release-verified |

## Current exact-SHA evidence

Current canonical `main` is `dd572f1052aa85439015078e0d879a5388544016`.

No successful exact-SHA executable verification for this SHA has been recorded. The repository's hosted CI has repeatedly produced failures with zero reported steps on the affected runs, which is infrastructure/no-step evidence rather than source-failure evidence.

The canonical `tools/verify_exact_sha.sh` harness is present and non-destructive. It refuses SHA mismatch and dirty working trees, then runs contract verification, workspace build, Clippy with `-D warnings`, workspace tests, deterministic artifact tests, DNS/orchestration tests, filesystem/process vertical slices, database core+CLI slices, and HTTP core+CLI slices. It is the authoritative independent executable path for this verification gap when run in a real checkout with the required Rust/Python toolchain.

The latest successful exact-SHA executable evidence remains historical and predates the database and HTTP implementation. It must not be generalized to `dd572f1052aa85439015078e0d879a5388544016`.

## Canonical CI status

The latest inspected hosted CI run for the preceding canonical SHA `82b5fb53a434f2fd619cfcbfed52aa2dae0934fe` failed both Ubuntu and Windows matrix jobs with zero reported steps. The current merge commit has no usable hosted executable status recorded yet. A workflow that never executes steps cannot establish a source-level failure.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline or local verification is valid independent execution evidence only when its environment, toolchain, command, exact commit, and result are recorded.
- `VERIFIED` is reserved for claims backed by actual evidence.
- Evidence for one commit does not automatically verify later commits.
- Filesystem write is experimental and must not be generalized from Android/Termux evidence to Ubuntu/Windows support.
- Database query is implemented but must not be described as verified or supported until current-head executable evidence exists.
- HTTP GET is implemented but must not be described as verified or supported until current-head executable evidence exists.
- The process-execution slice is `EXPERIMENTAL` only within its previously verified boundary.
