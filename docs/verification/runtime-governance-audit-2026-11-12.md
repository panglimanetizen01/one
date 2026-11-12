# Runtime and Governance Source Audit — 2026-11-12

**Scope:** candidate PR #92 source branch `tooling/python-local-ci`, repository `panglimanetizen01/one`.

**Audit baseline:** `01e0ad4a198d8095340da8d10adc3a6fa5ed9fa5` (CircleCI SUCCESS and independent Ubuntu/Windows Server 2025 parity PASS).

**Candidate code/test commits:** `f16d100b2f416e14a708dc67de0fa9170ad23a10` and follow-up test correction `c512a902a61a3bdac61244d84a3851f5a15cbbbd`. This audit report is added in a subsequent documentation commit; the report commit itself changes the exact source SHA and must be reverified.

**Important evidence boundary:** the code/test snapshot at `c512a902...` had not yet received a terminal CircleCI result or fresh independent parity when this report was prepared. The follow-up documentation commit containing this report creates a new exact-SHA head; it must receive its own CircleCI and independent parity. The successful evidence on `01e0ad4...` must not be reused as proof for either later head.

## Confirmed defects and candidate remediations

| Finding | Source evidence | Candidate remediation | Verification status |
|---|---|---|---|
| Event failure leaked nested lexical scopes into persistent event environment | Conditional/while/foreach pushed a scope and propagated `?` before popping; `dispatch_event` restores the environment after errors | Commit `01e0ad4...` adds scope cleanup before error propagation and three regression tests | Verified PASS on `01e0ad4...` by CircleCI and verifier run #708 on Ubuntu + Windows Server 2025 |
| Read-only SQLite query could create a missing database file | `SqliteAdapter` used `Connection::open(path)`, whose default flags permit CREATE/WRITE despite `DatabaseQuery` / `EffectClass::Read` | Commit `f16d100...` uses `SQLITE_OPEN_READ_ONLY` for file-backed DBs and a separate `:memory:` path; tests assert missing file remains absent and memory SELECT still works | Pending fresh exact-SHA verification |
| Interpreter discarded orchestration audit records | A local `Vec<AuditRecord>` was passed to the orchestrator and dropped; interpreter exposed no audit accessor | Commit `f16d100...` retains records in `Interpreter`, exposes `audit_records()`, clears them at the start of a new program execution, and retains event records | Pending fresh exact-SHA verification |
| Unsuccessful adapter result could appear as successful interpreter execution | Interpreter normalized and emitted output without checking `AdapterResult.success` | Commit `f16d100...` returns runtime error `N3213` and keeps the orchestrator's observed-failure audit record; fake-adapter tests cover failure and policy denial | Pending fresh exact-SHA verification |
| Integer arithmetic could panic or wrap depending on build profile | Raw `i64 + - * /` operations had no checked overflow handling | Commit `f16d100...` uses checked operations; overflow including `i64::MIN / -1` returns `R3023`; tests cover add/subtract/multiply/divide overflow | Pending fresh exact-SHA verification |
| Runtime interpreter accepted text + text arithmetic although semantic analysis rejects it | `eval_binary` had a Text+Text Add branch while semantic type inference does not define text concatenation as arithmetic | Commit `f16d100...` removes that branch; direct-IR regression expects `R3018` | Pending fresh exact-SHA verification |
| Public `Environment::default()` produced an empty scope vector while `define()` unwraps the last scope | Derived `Default` created `scopes=[]`; `define` then panicked | Commit `f16d100...` implements Default through `Environment::new()`; regression test defines and reads a value | Pending fresh exact-SHA verification |

## Further architectural and security gaps — not silently marked fixed

1. **Semantic IR / type model:** at baseline `01e0ad4...`, `TypedProgram` wrapped the original AST; identifier references remained raw strings; CSIR lacked resolved declaration identities and semantic type metadata. This conflicts with the *draft* normative contract in `docs/spec/one-v0.1-semantic-ir.md` and the draft type/name-resolution contracts. These are not yet accepted normative contracts. Do not weaken them silently or claim full conformance; record an explicit decision or implement the intended representation.
2. **Filesystem scope hardening:** file adapters pass raw target strings directly to `std::fs`, while `ScopeRule::Target` compares raw strings. Canonical-path, symlink, and TOCTOU behavior needs adversarial testing and an explicit definition of what a target-scoped grant authorizes. No sandbox claim is made.
3. **Process resource controls:** `run_process` uses blocking `Command::output()` without a timeout or output bound. A child can hang or generate unbounded output. Define and test limits before claiming bounded process execution.
4. **Network runtime integration:** `Interpreter::execute_network` validates and checks capabilities but returns `performed_external_io=false`; real socket/HTTP runtime APIs exist as separate library components. Do not claim source-level network statements execute real external I/O unless a concrete integration path and evidence are added.
5. **Numeric type breadth:** runtime `Value::Number` is `i64`; the draft type-system contract describes a broader numeric model. Fractional-number support and Integer/Number distinction remain a spec/implementation gap.
6. **Native backend scope:** the C backend is a bounded supported subset, not a complete backend for all language constructs. Unsupported statements/expressions must remain explicit and must not be presented as native-complete.

## CI and evidence boundaries

- PR #92 remains OPEN / DRAFT / UNMERGED; no merge or release.
- CircleCI and independent verifier run #708 proved exact SHA `01e0ad4...` only.
- GitHub Actions runs #43145380932, #43146634935, and #43146748682 completed FAILURE on both matrix jobs with `steps=null`, `logs_url=null`, and log retrieval returning Azure BlobNotFound/404. Root cause is UNKNOWN. These are infrastructure/verification-insufficiency signals, not evidence of a source-code failure.
- Android runtime remains NOT RUN. Android SDK/NDK setup is not device/emulator execution evidence.
- PR #94 is a separate candidate; its `CANONICAL_COMMIT.txt` pin remains `5486c09b746254e65ec647e86db3e48fde975747`.

## Required next steps

1. Obtain terminal CircleCI SUCCESS for the current PR #92 head `c512a902...`.
2. Advance only the PR #92 verifier pin to the exact current source SHA, run Ubuntu + Windows Server 2025, inspect terminal logs and both artifacts, and independently verify ZIP/report/manifest digests.
3. Fix remaining security/resource gaps with failing tests and explicit contracts.
4. Reconcile draft semantic/type/CSIR contracts with implementation status; do not label draft contracts accepted without a decision.
5. Update this audit and the cross-session Notion skill after every material source or evidence change.

