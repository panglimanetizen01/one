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
| CLI | `crates/one-cli/src/main.rs`, `docs/spec/one-v0.1-cli-contract.md` | CLI integration + smoke tests | VERIFIED on current canonical SHA / Termux-Android boundary |
| Contract consistency | `tools/check_contract.py` | artifact/index/fixture scan | VERIFIED on current canonical SHA / Termux-Android boundary |
| Conformance | `docs/testing/conformance.md`, `crates/one-core/tests/conformance.rs` | fixture runner | IMPLEMENTED / independently exercised |
| Golden outputs | `tests/golden/*.snap` | executable comparison tests | IMPLEMENTED / independently exercised |
| Deterministic `.oneir` | `tools/verify_exact_sha.sh`, deterministic CLI test | byte-deterministic build/package evidence | VERIFIED on current canonical SHA / Termux-Android boundary |
| Deterministic `.onepkg` | `tools/verify_exact_sha.sh`, deterministic CLI test | byte-deterministic build/package evidence | VERIFIED on current canonical SHA / Termux-Android boundary |
| Orchestration domain/governance | `orchestration.rs`, `governance.rs`, `orchestrator.rs` | unit/integration tests | VERIFIED on current canonical SHA / Termux-Android boundary |
| Orchestration adapter boundary | `adapter.rs`, `network_dns_adapter.rs`, `database.rs` | adapter tests + runtime | VERIFIED on current canonical SHA / Termux-Android boundary |
| Orchestration audit/result normalization | `orchestration_audit.rs`, `orchestration_result.rs` | audit/normalization tests | VERIFIED on current canonical SHA / Termux-Android boundary |
| Language-level DNS orchestration | parser → semantic → typed IR → governance → DNS adapter → normalized result → CLI | end-to-end source execution + explicit permission | EXPERIMENTAL / verified within current Termux-Android boundary |
| Language-level filesystem-read orchestration | parser → semantic → typed IR → governance → filesystem.read adapter → normalized result → CLI | end-to-end source execution + allow/deny/out-of-scope/backend tests | EXPERIMENTAL / verified within current Termux-Android boundary |
| Language-level process execution orchestration | parser → semantic → typed IR → governance → process.execute adapter → normalized result → CLI | source execution + permission + denial + backend-failure tests | EXPERIMENTAL / verified within current Termux-Android boundary |
| Language-level filesystem-write orchestration | parser → semantic → typed IR → governance → filesystem.write adapter → CLI | source execution + explicit permission + exact-target write + denial | EXPERIMENTAL / verified within current Termux-Android boundary |
| Language-level database-query orchestration | parser → semantic → typed IR → resource-scoped governance → `database.sqlite.query` → normalized result → CLI | source execution + explicit permission + out-of-scope denial + backend failure + deterministic normalization | EXPERIMENTAL / verified within current Termux-Android boundary |
| Database-query CLI regression | `crates/one-cli/tests/database_orchestration.rs` | source + `--izinkan database.query` + matching resource + denial/scope tests | VERIFIED on current canonical SHA / Termux-Android boundary |
| Language-level HTTP GET orchestration | parser → semantic → typed IR → target-scoped governance → `network.http.get` → normalized result → CLI | loopback source execution + explicit permission + out-of-scope denial + adapter validation | EXPERIMENTAL / verified within current Termux-Android boundary |
| HTTP GET CLI regression | `crates/one-cli/tests/http_get_orchestration.rs` | real loopback GET + `--izinkan network.connect` + matching target + denial/scope tests | VERIFIED on current canonical SHA / Termux-Android boundary |
| Filesystem-write CLI regression | `crates/one-cli/tests/filesystem_write.rs` | source file + `--izinkan filesystem.write` + actual target content | VERIFIED on current canonical SHA / Termux-Android boundary |
| Filesystem-write policy/runtime | `crates/one-core/tests/filesystem_write_runtime.rs` | default denial + target-scoped grant + exact text output | VERIFIED on current canonical SHA / Termux-Android boundary |
| Broader language-level orchestration | same source-to-runtime chain for additional capabilities | per-capability end-to-end tests | IN DEVELOPMENT |
| Linux portability | verification mirror workflow | actual Linux run | GREEN historically; current main has no fresh Linux executable evidence |
| Windows portability | verification mirror workflow | actual Windows run | GREEN historically; current main has no fresh Windows executable evidence |
| Release gate | verification mirror workflow | green exact-SHA matrix | CURRENT MAIN NOT RELEASE-VERIFIED CROSS-PLATFORM |

## Current exact-SHA evidence

Canonical `main` is currently verified at `55a8474ad5b19e20fcd51c3be79f39aaa374bbf3` on Termux/Android using `tools/verify_exact_sha.sh`.

The exact-SHA run passed: exact SHA check, contract verification, workspace build, Clippy with `-D warnings`, full workspace tests, deterministic artifact test, DNS targeted tests, orchestration targeted tests, filesystem vertical slice, filesystem capability isolation, process vertical slice, database vertical slice, database CLI, HTTP GET vertical slice, and HTTP GET CLI.

The full workspace suite passed all discovered tests, including 90 one-core unit tests, 15 CLI tests, database 7/7, HTTP GET 3/3, process 8/8, filesystem 7/7, and all remaining suites with zero failures.

This is current executable evidence for the stated Termux/Android boundary. It does not establish Ubuntu/Windows support for `55a8474ad5b19e20fcd51c3be79f39aaa374bbf3`.

## Cross-platform evidence boundary

The latest cross-platform exact-SHA evidence remains mirror run `38929072800` for revision `9038ac75f197753ff1ab6af7478e63e4831903a2`, verified on Ubuntu and Windows Server 2025. That evidence predates the current database, HTTP, and later filesystem-write revisions and must not be generalized to current `main`.

The repository's hosted CI attempts for affected historical work produced zero-step failures. Those are infrastructure/no-step evidence, not source-failure evidence.

## Evidence rules

- Source presence is implementation evidence, not execution evidence.
- A failed workflow without usable job steps cannot establish a Rust source failure.
- A runner that never starts is an infrastructure blocker, not a reason to invent a code result.
- Offline or local verification is valid independent execution evidence only when its environment, toolchain, command, exact commit, and result are recorded.
- `VERIFIED` is reserved for claims backed by actual evidence for the exact recorded source SHA and stated platform boundary.
- Evidence for one commit does not automatically verify later commits.
- Filesystem write, database query, and HTTP GET are verified only within the current Termux/Android exact-SHA boundary and must not be generalized to Ubuntu/Windows.
- The language-level orchestration slices remain `EXPERIMENTAL` because their scope is intentionally limited even where executable evidence exists.
- Cross-platform support for current `main` remains unverified.
