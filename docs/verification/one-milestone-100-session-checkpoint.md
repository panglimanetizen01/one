# ONE Final Milestone Session Checkpoint

## Active scanner A–E handoff — 2026-10-28

The current implementation work is on the milestone PR branch. The live PR #80 head and independent verifier pin are authoritative for the exact source SHA; this documentation update is not a substitute for reading those refs. The scanner now includes repository integrity, Python AST correlation for obfuscated execution / downloaded-code execution / sensitive-data egress, JavaScript/TypeScript static heuristics, agent-instruction prompt-injection correlation, and supply-chain checks for mutable Action/dependency refs. Regression tests include positive and negative fixtures; **no executable result is yet available for the latest candidate**.

The source candidate immediately before this documentation synchronization was `29e20838825fe074f6821b663158fdcafc433478`. Canonical workflows on the prior scanner candidate `ae4b8c9cee65b24700f4a0827309fc3b3142e7cb` were created for repository health `40725202584`, One CI `40725202524`, E1 `40725202471`, focused stdlib `40725202455`, E2 `40725202439`, and Linux distro `40725202433`. The repository-health jobs were cancelled with no steps, and other jobs returned empty/null steps and no log URLs; artifacts were unavailable. This is unresolved execution/evidence infrastructure, not a source-test failure or pass. Do not repeat identical jobs without a changed condition or transient-failure diagnosis.

Acceptance remains: implementation 92/100 provisional; E3 0/8, E4 0/6, E5 0/4 verified; final score UNKNOWN / NOT RECONCILED; DoD 3 checked / 10 unchecked; gate NOT PASSED. No merge, release, protected E2 dispatch, or security-control bypass is authorized.

**Updated:** 2026-10-28
**Active target:** Milestone 100% — ecosystem and release quality
**Repository:** https://github.com/panglimanetizen01/one
**Branch:** `milestone/100-ecosystem-20261023`
**PR:** https://github.com/panglimanetizen01/one/pull/80 (open, draft, unmerged)
**Base:** `main` (PR only; no direct changes to main)

## Historical recovery state — 2026-10-28 (superseded by active scanner handoff below)

PR #80 remains open, draft, and unmerged. The live PR head is authoritative and must be read after each source/documentation commit; the independent verifier pin and index must then be synchronized to the final candidate.

**Scanner delta implemented:** JavaScript/TypeScript source suffixes are now scanned, with `security-javascript` rules for decoded dynamic execution, network-plus-execution correlation, sensitive-source/network-sink correlation, and Node `child_process.exec/execSync`. Three positive/negative regression fixtures were added; detector docs and verification README were updated. Isolated positive/negative detector snippets were exercised, but the full unit suite and actual repository self-scan have not produced executable hosted evidence.

Canonical source workflow runs for the scanner candidate repeatedly failed before any executable step: returned job objects had null steps/log URLs and no artifacts, and a job-log fetch returned `BlobNotFound`. This is an unresolved execution/evidence-access blocker, not source-test failure or pass; no root cause is known. The repository-health workflow is configured to run scanner/collector/quality tests and scan ONE itself, but current-SHA output still must be obtained and inspected.

Ledger acceptance remains: 92/100 implementation provisional; carried-forward foundation/core arithmetic 60/100 only; E1 0/10, E2 0/8, E3 0/8, E4 0/6, E5 0/4, E6 0/4 verified for current acceptance. DoD 3 checked / 10 unchecked. **Final score UNKNOWN / NOT RECONCILED. Gate NOT PASSED.** No merge, release, protected E2 dispatch, secret change, trust-store change, or protection bypass.

## Superseded checkpoint — before forensic reconciliation (2026-10-28)

- PR #80 remains draft, open, and unmerged. No merge or production release is authorized.
- Canonical weights remain fixed at 100. The latest ledger acceptance records **92/100 implementation** and **78/100 provisional verified** after scoped E4 acceptance on candidate `28d1384588662cc93f405dc9dd5f3012183d249f`, alongside provisional E3/E5 evidence. This checkpoint/ledger update advances the source SHA; re-pin and rerun the exact-SHA matrix before final gate acceptance.
- E3 evidence on the pre-reconciliation candidate includes Linux/Windows cross-platform checks, Linux distro, artifact integrity, and actual Android API 35 arm64-v8a ELF execution on native ARM64 with exit code 0 and `ONE_NATIVE_BACKEND_OK`. E5 contract/docs/tooling checks passed on Ubuntu and Windows.
- E1 remains partial (0/10 verified); E2 remains 0/8 because no protected production release run exists for the exact source SHA and no release authorization is granted; E4 is provisionally 6/6 accepted within the documented Linux/Windows/Android boundary, pending revalidation on the new SHA; E6 remains 0/4. Gate: **NOT PASSED**.
- Canonical repository workflows that failed with runner ID 0 and no steps are infrastructure/job-start failures. Independent exact-SHA mirror results are evidence only for their recorded source SHA.
- Always read the current source SHA from PR #80 and the independent selector; do not reuse any older SHA or duplicate completed discovery/tests.

## Fixed completion method

The canonical ledger remains `docs/verification/one-completion-ledger.md`, ledger version 1. Weights remain unchanged: F1–F6 = 30 points, C1–C5 = 30 additional points, and E1–E6 = 40 additional points. Verified completion is based only on acceptance evidence at the exact recorded source SHA.

## Verified baseline inherited from milestone 60

- Recorded milestone-60 code SHA: `3027e43b9e2481d73bdb67dbbbbb2e27c3419639`.
- Recorded independent cross-platform verification: [run 39936123376](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39936123376), Ubuntu and Windows successful per the canonical checkpoint.
- This is historical, scope-limited evidence for the 60-point milestone. It does not verify new milestone-100 code.
- Canonical repository CI run [39936862004](https://github.com/panglimanetizen01/one/actions/runs/39936862004) failed before executing steps (both jobs reported zero steps and runner ID 0); it is not a source test result.

## Work added in this session

- `crates/one-core/src/stdlib.rs`: deterministic public Rust helpers for text length/contains/trim/literal replace/literal split/integer conversion; list length/index access; deterministic text-keyed map construction/get/set; checked integer absolute value/division/min/max; bounded deterministic JSON serialization/parsing for text, numbers, booleans, empty values, lists, and maps; JSON float/exponent values are intentionally rejected because ONE has no float type; Unicode case/prefix/suffix operations; checked list joining and integer summation. Unit tests cover Unicode, bounds, invalid types, overflow, division by zero, stable map ordering, JSON escaping, and rejection of excessive JSON nesting (test commit `d2f77bb34be33780b24685899be5c773dbde1b7a`).
- `crates/one-core/src/semantic.rs`: registers 22 built-ins with arity, return-type, and static argument-type rules (`E2043`), including bounded JSON serialization/parsing, Unicode text helpers, and checked collection operations.
- `crates/one-core/src/interpreter.rs`: dispatches source-level text/list/map/integer built-ins through the canonical IR interpreter and checks runtime arity before indexing arguments; invalid builtin operations return `R3022`. Integer arithmetic uses checked add/subtract/multiply/divide and reports `R3023` on overflow. Recursive user-function calls are bounded by `max_call_depth` (default 128) and return `R3024`.
- `crates/one-core/tests/stdlib_builtins.rs`: adds end-to-end tests through parse → semantic analysis → IR → interpreter, including opt-in mapping tests, JSON serialization/parsing round trips, Unicode text and collection built-ins, a checked-in example, overflow, invalid conversion, invalid index, missing-map-key, and recursion-depth cases.
- `docs/spec/one-v0.1-runtime-model.md`: documents actual execution/call-depth limits and checked integer overflow behavior without claiming unimplemented memory/open-file quotas.
- `crates/one-cli/src/main.rs`: `one build`, `one test`, and `one package` detect a `Buat program` entry and use `human_compiler::compile_file` to process relative modules through canonical IR. Build writes a `.oneir`, test executes the project and supports valid/invalid human-core conformance fixtures, and package writes a separate deterministic `One human-core package v1` artifact and a sidecar module lock v2 with sorted relative module paths and SHA-256 content digests. The lock is not signed and does not authenticate publisher provenance. Build/test reject stale existing locks with `ETOOL006`; package refreshes the lock. Ordinary v0.1 `.onepkg` format remains unchanged.
- `crates/one-cli/tests/cli.rs`: tests deterministic packaging of a modular human-core project, exercises build/test/package, includes valid/invalid human-core conformance fixtures, and verifies stale-lock rejection/refresh.
- `crates/one-cli/src/main.rs` and `crates/one-cli/Cargo.toml`: use SHA-256 content digests for module lock v2, with known-vector regression tests. This detects content mismatch against a trusted lock but is not a signature or provenance mechanism.
- `docs/spec/one-v0.1-package-build.md` and `docs/spec/one-v0.1-cli.md`: document the experimental human-core project workflow and explicitly leave dependency/lockfile support incomplete.
- `crates/one-cli/tests/cli.rs`: verifies that the official CLI can execute the checked-in stdlib example.
- `examples/stdlib.one`: runnable example exercising the default-parser text/list/integer builtin subset.
- `examples/human-vnext/pemetaan-kontak.one`: now exercises mapping get/update built-ins through the opt-in human-core CLI path.
- `crates/one-cli/tests/human_core.rs`: asserts the checked-in mapping example outputs the updated value.
- `docs/panduan-pengguna/fungsi-dan-event.md`: documents supported builtin syntax and explicit limitations, including opt-in mapping grammar.
- `docs/spec/one-v0.1-standard-library.md`, `docs/spec/one-v0.1-stdlib.md`, and `docs/panduan-pengguna/fungsi-dan-event.md`: document helper boundaries, source-level built-in signatures, JSON serialization, type/arity checks, and error behavior.
- Main implementation commits include `652d4dc6662b62538dde602ef215cfe05b340cd1`, `7f196f64b2b181ebb87ac6a2fb35642b3f6d5112`, `449caf6b33c28d10d59674e6bb4e850d6ad4be1c`, `c2f16baa1ab26184c74c7908faa1557419c1e277`, `66cf560fe30928318ea619a565422303c8806ef9`, `abd105a4515c3c1b96b8695f6c9061646a6dfca2`, `c56d841ed914f384975f9e224e1db014336a72c7`, `6656abed12c0b0d5dba983c2261c516ca98e5b55`, and `2b98153907eaa1e96f160bf2efc7b59aef06eaf0`.

## Verification status

- Canonical repository CI run [39940416046](https://github.com/panglimanetizen01/one/actions/runs/39940416046) failed before steps (`runner_id=0`, empty step lists); it is infrastructure evidence, not a source pass.
- Exact-SHA matrix [39941625828](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39941625828) **PASSED on Ubuntu and Windows** for source SHA `2cf1e567357da960b8b592afc67a29454533bbd7`, validating the earlier standard-library source slice.
- Exact-SHA matrix [39942950596](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39942950596) **PASSED on Ubuntu and Windows** for source SHA `c617695a5e45e41e6c5b21e0c99da94cfb9a559b`, validating the modular human-core package slice. This snapshot predates build/test additions and the module lockfile.
- Run [39943377785](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39943377785) passed Ubuntu's required build/AST/Clippy/workspace/network/offline steps for the human-core build/test/package workflow.
- Run [39943724686](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39943724686) passed Ubuntu's required checks for human-core conformance fixtures.
- Run [39944046531](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39944046531) passed Ubuntu's required checks including deterministic JSON serialization.
- Run [39944600422](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39944600422) **PASSED on Ubuntu and Windows** for the text/list/JSON serializer/depth-limit slice.
- Run [39944870126](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39944870126) passed Ubuntu and Windows build, AST contract tests, Clippy, and workspace tests for JSON parsing and round-trip support; Windows offline verification was still in progress at the last query.
- Run [39945264052](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39945264052) failed before contract/build steps because the mirror was incorrectly pointed at blob SHA `c66b31c64319e471552fe89021a51856b00761bc` instead of commit SHA `f4b404596f1bcd4720c39855b76eee8ffaaec676`. Git reported `fatal: unable to read tree`; no source checks ran. This was a verification-target error, not a source test result.
- The module-lockfile implementation now emits lock v2 with SHA-256 digests for sorted relative module paths. Publisher identity and authenticated provenance remain unimplemented.
- Historical ledger totals at the 2026-10-23 checkpoint: **75/100 provisional implementation points; 60/100 verified points**. This historical subtotal is superseded by the 2026-10-24 continuation section below; fixed weights remain unchanged.
- E1 remained partial at this checkpoint: source-level text/list/map/integer/JSON built-ins, filesystem limits, and bounded HTTP framing were integrated. UTC formatting and strict UTC parsing were added in the 2026-10-24 continuation; broader time/date, concurrency, logging/configuration, and full persistence integration remain incomplete.
- E2 remains partial: human-core build/test/package, stale-lock verification, and SHA-256 module lock v2 are implemented. External dependency declarations/registry resolution, signed package identity, authenticated provenance, target-specific native artifacts, and full provenance remain incomplete.
- E3 was partial at this historical checkpoint; the exact-SHA Linux/Windows matrix for the later source snapshot is recorded in the 2026-10-24 continuation below.
- E4 remained partial at this checkpoint: checked arithmetic, recursion-depth bounds, and JSON/HTTP/filesystem bounds were implemented. The 2026-10-24 continuation adds read-only SQLite access and SQL/parameter resource budgets. Race-resistant filesystem resolution, broader memory/open-resource quotas, comprehensive threat review, and broader failure recovery remain incomplete.
- The previous exact-SHA run [39945846717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39945846717) built and passed AST contract tests on both platforms, then failed strict Clippy because an obsolete duplicate module collector had 8 arguments. The collector state was refactored into `ModuleCollection`. A subsequent strict-Clippy run found a nested-if lint in stale-lock validation; this was corrected using a let-chain, without suppressing the lint. Exact-SHA run [39958882035](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39958882035) is checking source commit `d9d696fa87683027b78fdcbbec504696904055aa`: Ubuntu build, AST regression tests, strict Clippy, workspace tests, real-network tests, and offline verification **PASSED**; Windows is still running its build/test matrix. No new verified ledger points are awarded until the full required matrix completes. The final 100% gate remains **NOT PASSED**.
## Latest security/package implementation changes

- crates/one-core/src/network_runtime.rs: replaced unbounded HTTP response buffering with a capped reader; enforces a 16 MiB body and 64 KiB header allowance, rejects duplicate Content-Length, Content-Length/Transfer-Encoding conflicts, unsupported transfer encodings, oversized headers, and more than 256 response headers. Regression tests cover oversized input and ambiguous framing.
- crates/one-core/src/adapter.rs: filesystem adapter reads/writes are capped at 16 MiB; target validation rejects NUL, parent-directory components, and existing symlink path components. This is defense-in-depth, not race-resistant sandbox confinement.
- crates/one-cli/src/main.rs and crates/one-cli/Cargo.toml: module lock v2 uses SHA-256 content digests with known-vector tests. It does not sign the package or authenticate provenance.
- Latest code-bearing SHA: e14b63ba0e40e45eb8a0c0f79a78886d0a83bdf1. Mirror run [39963778066](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39963778066) failed Ubuntu workspace tests (181 passed, 1 failed) because a linker test executed a program through a symlink located in a noexec temp directory. Run [39964166884](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39964166884) passed Linux and Windows on SHA 985bce88498daa0bbb6566226353251fe6e3c4ac, but run [39964952853](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39964952853) exposed the same noexec issue in the revised fixture. Commit e14b63ba0e40e45eb8a0c0f79a78886d0a83bdf1 fixes the root cause: use /bin/true directly as the Linux override, /bin/false as the PATH candidate, and scope the test to Linux. The final exact-SHA matrix must verify this correction on the final branch HEAD; no new verified ledger points are awarded.
- Canonical CI has continued to fail before steps on zero-step runner jobs; this is not a source test result. The mirror fallback is permitted by the repository contract, but only a complete exact-SHA Linux/Windows matrix can count.
## Latest execution continuation — 2026-10-23

- Fixed a hosted Ubuntu regression in `crates/one-core/src/native_backend.rs`: the linker-override priority test no longer assumes `/bin/true` and `/bin/false` exist. It uses the current Rust test executable for the successful probe, preserving the invalid-override test and the actual production linker compatibility check.
- The noexec regression is not treated as passed until the corrected Linux-specific fixture passes in the final exact-SHA matrix. The latest code fix is e14b63ba0e40e45eb8a0c0f79a78886d0a83bdf1; earlier run 39964925469 is historical and does not establish final-SHA verification.
- The module-lock validation work also adds stale-lock rejection for human-core `build`/`test` with `ETOOL006`, lock refresh via `package`, and regression coverage. This is a limited packaging improvement, not completion of external dependency resolution or cryptographic provenance.
- The latest verification run was pending at this historical checkpoint. Current completion is recorded in the 2026-10-24 continuation below; final 100% remains NOT PASSED.

## Remaining required work

- Complete the required standard library: safe capability-gated filesystem/IO, HTTP/networking, time/date, configuration/logging, persistence/transaction integration, and any contract-required concurrency APIs.
- Complete external dependency declarations/registry resolution, trusted lockfile semantics, signed package identity, authenticated provenance, target-specific artifacts, and supply-chain verification. SHA-256 module digests now detect content mismatch against a trusted lock, but do not authenticate the lock or publisher.
- Complete the native vNext grammar and module visibility/source-map contracts; keep v0.1 compatibility explicit.
- Complete conformance/compatibility across all declared platforms, security threat review, resource bounds, and failure recovery.
- Finish tutorials, reference docs, debugging/developer tooling, end-to-end modular application examples, and docs/source reconciliation.
- Run the final exact-SHA Linux/Windows and any other required platform matrix, then perform the full forensic Definition-of-Done audit.

## Git state and safety

- Current feature branch: `milestone/100-ecosystem-20261023`.
- PR #80 is open and draft; not merged.
- `main` was not modified directly.
- `~/one-termux` was not accessed.
- Do not claim milestone 100%, do not change ledger weights, and do not merge without explicit user authorization.

## Gate

**NOT PASSED** — milestone 100% remains in progress. The later exact-SHA evidence and current remaining work are recorded in the 2026-10-24 continuation section below.


## Session continuation — JSON parser input bound evidence (2026-10-23)

- Direct source inspection on branch `milestone/100-ecosystem-20261023` confirms `crates/one-core/src/stdlib.rs` defines `MAX_JSON_INPUT_BYTES = 16 * 1024 * 1024`, rejects larger UTF-8 inputs before constructing `JsonParser`, and includes `json_parser_rejects_inputs_above_the_byte_limit_before_allocating_parser_state`. This is direct repository-content evidence for implementation presence, not a fresh execution result.
- Related commit `fc8f7548722c21522e9be773cbf4773e0deb8ee4` is available at https://github.com/panglimanetizen01/one/commit/fc8f7548722c21522e9be773cbf4773e0deb8ee4. The exact current branch HEAD SHA was not exposed by the branch-search/read tools in this session, so this commit is not asserted to be the current branch HEAD.
- Canonical CI run [39969715091](https://github.com/panglimanetizen01/one/actions/runs/39969715091) reports Ubuntu and Windows jobs as failed, but the available job payload exposes no step results/logs. Cause is **UNKNOWN**; no source-test failure or pass is inferred from that payload. Earlier run [39965785392](https://github.com/panglimanetizen01/one/actions/runs/39965785392) likewise reports both jobs failed with no step details available through the current connector.
- No verified ledger points are added. Ledger weights and totals remain unchanged at **75/100 provisional implementation, 60/100 verified** until full exact-SHA acceptance evidence is available.
- Gate remains **NOT PASSED**. This entry does not assert a new release SHA or a complete milestone.


## Session continuation — JSON boundary regression quality (2026-10-23)

- Added code-level regression tests in `crates/one-core/src/stdlib.rs` for valid JSON exactly at the 16 MiB byte limit, valid JSON one byte above the limit, UTF-8 input whose character count is below the limit but byte count exceeds it, empty input, and nesting beyond the parser's depth limit.
- Source changes: [`34a4e2b696ecdf6be09f42c04a0d8ce33117a24c`](https://github.com/panglimanetizen01/one/commit/34a4e2b696ecdf6be09f42c04a0d8ce33117a24c) and follow-up compile correction [`7d3137b814984f97df366da8ba52ccc93519b57e`](https://github.com/panglimanetizen01/one/commit/7d3137b814984f97df366da8ba52ccc93519b57e). The prior oversized-whitespace test was inadequate because that input was invalid JSON even without a size guard; the replacement uses otherwise-valid JSON padded with legal whitespace.
- The branch tip before this checkpoint update was source SHA `7d3137b814984f97df366da8ba52ccc93519b57e`, three commits ahead of `a9e864721cbe0b046b91d67c04f671df5096e520` and four commits ahead of `fc8f7548722c21522e9be773cbf4773e0deb8ee4`. No history rewrite or merge was performed.
- Matrix [39969837787](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39969837787) completed successfully on Ubuntu and Windows, but tested only `a9e864721cbe0b046b91d67c04f671df5096e520`; it does not verify these new regression tests.
- The verification mirror's `CANONICAL_COMMIT.txt` was updated in commit [`6da05045c1cc76a5e3d751a3c3401e2b22b48eb4`](https://github.com/panglimanetizen01/one-windows-verification/commit/6da05045c1cc76a5e3d751a3c3401e2b22b48eb4) to target source SHA `7d3137b814984f97df366da8ba52ccc93519b57e`. The available connector cannot enumerate push-triggered mirror workflow runs, so a successful run for this target is **not yet evidenced** here. Do not award verification points based on the trigger or commit alone.
- Canonical repository CI run [39972654697](https://github.com/panglimanetizen01/one/actions/runs/39972654697) reports both jobs failed with no step details, and the log endpoint returns BlobNotFound. Root cause remains unknown; this is not evidence that source tests ran or failed.
- Ledger weights and points remain unchanged: **75/100 provisional implementation, 60/100 verified**. The JSON boundary tests are committed but not yet verified at their source SHA. Gate remains **NOT PASSED**.

## Continuous security hardening update — 2026-10-23

- Source changes on the working branch after the previous checkpoint:
  - crates/one-core/src/stdlib.rs: JSON parsing now walks UTF-8 source with a byte cursor instead of collecting the entire input into a second Vec<char>. The 16 MiB check is based on UTF-8 byte length; tests cover exact-limit input, one byte over, multibyte UTF-8, empty input, deep nesting, raw Unicode, and signed-integer boundaries.
  - crates/one-core/src/network_runtime.rs: TCP send uses write_all and reports the full payload length only after the complete write; IPv6 resolution is bracketed correctly; HTTP authority, request tokens, header names/values, status codes, reason phrase controls, Content-Length, chunk size lines, chunk trailers, and response framing are validated; response and chunk/header accumulation have explicit bounds. Regression cases cover request smuggling/header injection, malformed framing, oversized chunk lines, integer overflow, and authority ambiguity.
  - crates/one-core/src/network_tls.rs: HTTPS uses the shared authority parser and bounded response reader instead of unbounded read_to_end, with the shared limits applied before parsing.
  - crates/one-core/src/network_http2.rs: HTTP/2 authority and path validation now use shared strict parsing, and response header count/bytes are capped with invalid header text rejected. Tests cover invalid authorities/paths, capability denial, and response header budgets.
  - crates/one-core/src/network_mail.rs: IMAP login and mailbox arguments are quoted with escaping; POP3 usernames reject whitespace; command control characters are rejected; POP3/IMAP response accumulation is bounded; IMAP tag counter overflow is checked. Regression tests cover argument quoting and injection boundaries.
- Latest code-bearing commit recorded at the time of this checkpoint: e0629ebaada2e12090126ba53fc415e026b6b280 (fix: prevent IMAP command tag counter overflow).
- The canonical workflow runs observed during this hardening burst repeatedly reported immediate failure with both jobs listing zero steps and runner ID 0; logs are unavailable through the configured GitHub Actions endpoints. Those runs do not establish a source-test failure or a source-test pass. No new verified ledger points are awarded.
- The workflow config includes build, strict Clippy, workspace tests, deterministic artifact tests, DNS/orchestration/database/HTTP vertical slices, and the Ubuntu/Windows matrix. The latest exact-SHA green result for this code-bearing snapshot has not been obtained. The verification mirror's previous green run applies only to an earlier SHA.
- Fixed ledger weights and accounting remain unchanged: 75/100 provisional implementation; 60/100 verified. The broader mandatory gaps recorded above (complete standard-library breadth, external dependency/registry and authenticated provenance, full conformance/platform matrix, race-resistant filesystem confinement, comprehensive threat review, docs/tooling audit, and final forensic release audit) remain open.
- Gate remains NOT PASSED. This checkpoint deliberately awards no completion points merely for commits or unverified tests.

### Additional resource-bound correction

- crates/one-core/src/network_runtime.rs now rejects attempts to exceed 1,024 active connections, checks connection ID increment for overflow before creating a socket, and rejects outbound payloads above 16 MiB before connection lookup/write. A regression test checks oversized payload rejection.
- This adds implementation coverage only; no ledger points are added without the full acceptance matrix. The next exact-SHA matrix must target the resulting checkpoint commit.

### Exact-SHA retarget after final code edit

- Additional source commit: 24f454188dfb6ef8d97d63964af78bb5b65bd079. It factors active-connection quota validation into a pure helper and adds boundary tests for 1,023 versus 1,024 active connections. It also includes the borrow correction for the oversized HTTP/2 header regression fixture.
- The in-progress mirror run 39976171105 targeted the prior checkpoint SHA 54779adc74032cc88fa30950e6700d135b521353 and therefore does not verify this newer source edit. The mirror selector must be updated again to the checkpoint SHA produced by this update.
- Canonical repository CI continues to return zero-step failures; it cannot substitute for the independent mirror's required build, Clippy, workspace, real-network, and offline checks.
- Ledger stays at 75/100 provisional and 60/100 verified. No new points are awarded until the final exact-SHA matrix and remaining acceptance requirements pass.

### HTTPS default-port correction

- Source commits 4859114360ccd41943230a0f8faf95eda129a1da and 07e73e1deb7100c01094523fd3f1c9a005ebf70 add a protocol-specific default-port argument to the shared HTTP authority parser and ensure HTTPS uses port 443 when no port is supplied. Regression coverage checks default ports for DNS names and bracketed IPv6 while preserving explicit ports.
- This fix was made after the earlier mirror target; it invalidates any verification run targeting the earlier SHA. The final matrix target must be the commit produced by the next checkpoint update.
- No ledger points are added until the required exact-SHA checks and broader Definition of Done pass.

### Build failure found and corrected from the exact-SHA matrix

- The Ubuntu build in earlier matrix run 39976287701 exposed a real compile error in IMAP FETCH: the tag counter API had changed to return Result<String, MailProtocolError>, but fetch_body still used it without propagating the error. This produced E0277 when formatting the tag.
- Fixed in source commit 510ce2e97c89246b25a3c8ca162e877284804338 by changing the remaining FETCH call site to use the checked result, self.tag()? . This is a root-cause fix, not a test waiver.
- The prior matrix is a failed run and is not credited. A fresh exact-SHA matrix must target the checkpoint commit produced by this update; build, Clippy, workspace tests, real-network tests, and offline checks must all execute and pass on that SHA.

### Additional protocol and resource-safety fixes

- Exact-SHA build/test evidence from matrix 39976584347 identified the IMAP integration fixture mismatch after safe argument quoting. The fixture was corrected in commit 406d5eb602aed72c4729759ab53483fca0ef423a to expect quoted LOGIN and SELECT arguments; the implementation's escaping remains in place.
- TFTP reads now bind a transfer to the first responding peer from the requested server IP, reject peer changes, cap received files at 16 MiB, and check block-number increments. TFTP writes cap payloads before network I/O and require ACKs from the negotiated transfer peer.
- NTP queries now validate the exact server peer, version/mode/leap/stratum fields, a nonzero transmit timestamp, and the echoed originate timestamp. Tests cover valid responses and unsynchronized stratum-zero replies.
- FTP/SMTP reply accumulation is bounded by 16 MiB and 100,000 lines; application fields are capped and control bytes rejected; SMTP message size is capped; FTP reply parsing rejects non-ASCII status prefixes before byte slicing.
- HTTP request serialization now caps method length, path length, body size, user-supplied header count and header bytes before building the wire buffer. HTTP/2 has a real loopback request/response test; QUIC rejects outbound payloads over 16 MiB before copying.
- These edits follow the previous checkpoint SHA, so the next independent matrix must target this checkpoint's exact SHA. Prior results remain tied to their recorded source snapshots and do not verify these changes.

### Final source snapshot for the next exact-SHA run

- Additional source changes after the previous checkpoint:
  - network_datagrams.rs: TFTP write-size validation now happens before sending the write request; NTP validates the server's originate timestamp and rejects unsynchronized replies; regression fixtures echo the request timestamp and set a valid stratum.
  - network_application.rs: FTP multiline reply accumulation has explicit byte/line limits, non-ASCII status prefixes are rejected before byte-index slicing, SMTP fields/body and the assembled message are bounded, and control bytes are rejected. Empty SMTP subjects remain valid.
  - network_runtime.rs: HTTP request method/path/body/header-count/header-byte bounds are checked before serializing the wire buffer.
  - network_http2.rs: the real HTTP/2 loopback test uses a fixed-size byte array for the oversized-header fixture, avoiding an unnecessary heap allocation.
- Latest code-bearing commit before this checkpoint: 9f2ed1647b7a85a551b8b7dd48886296779640bd.
- The prior matrix run 39977368466 targeted the previous checkpoint and does not verify these latest edits. The next exact-SHA matrix must use the checkpoint SHA produced by this update.
- Ledger weights remain unchanged. No additional points are awarded for source changes without passing exact-SHA verification and full milestone acceptance.

### QUIC timeout and release-verification target

- Source commit d67a98a2c7dd2e0b4273814b6ebecf35015049d2 applies the configured timeout to QUIC payload writes as well as handshake, stream opening, and response reads.
- The current exact-SHA matrix run 39977590301 targets the prior checkpoint SHA cbf1c08ce0d0e3a55df07b92b72e23409f80bda3. Ubuntu build, AST contract checks, strict Clippy, and workspace tests completed successfully on that prior snapshot; real-network tests were still running at the last inspection. This does not verify the subsequent QUIC timeout edit.
- The next matrix target is the checkpoint SHA produced by this update. Do not transfer results from earlier source SHAs to the newer release candidate.

### HTTP/2 integration-test compile correction

- The real-network build on the previous source snapshot found E0596 in the new HTTP/2 loopback test because the response handle was not mutable, plus an unused Read import left after HTTPS switched to the bounded reader.
- Fixed in commits 31e20504da8c58620c1a40a18bf7be8336f75ed8 and 763fd6765c17928a070d3b77e04381ea9fa1c1dc: the test now binds the response handle mutably, and the obsolete TLS Read import is removed.
- The failed prior matrix remains failed and earns no points. The next exact-SHA run must target this checkpoint commit and execute the real-network suite again.

### TLS test-only trait import

- Source commit c44ee995bd6db99d55c419aa8bd2390b45c78636 moves the Read trait import into the TLS test module after bounded response reads stopped needing it in production. This avoids both the unused-import warning and a test-only method-resolution error.
- The exact-SHA matrix must target the new checkpoint commit after this change; the previous matrix's E0596 result is already addressed, but this latest import correction still needs compilation on the selected SHA.

### Exact-SHA matrix result — 2026-10-23

- Source SHA tested: aaa1245ac44e95eef3a564a08ba88e10aeb16433.
- Independent Linux/Windows matrix: run 39978071592 — SUCCESS for both ubuntu-latest and windows-2025.
- Both jobs checked out the exact source SHA above and passed the contract-target check, build, AST contract regression tests, strict Clippy, workspace tests, real-network runtime tests, and platform-appropriate offline verification. The Ubuntu log records the core suite as 204 passed, 0 failed; additional integration/conformance suites passed. Real-network and offline steps completed successfully on both platforms.
- The matrix still explicitly ignores one Android native execution test because the hosted runner lacks the required Android execution-capable Clang setup. It is not counted as passed. The full milestone 100 Definition of Done also still requires complete standard-library coverage, external dependency resolution and authenticated provenance, full compatibility/conformance coverage, comprehensive threat review, completed docs/tooling audit, and forensic release audit.
- The previous matrix failure was actionable: E0277 in IMAP FETCH and E0596 in the HTTP/2 loopback fixture were fixed and then the corrected source passed the matrix above. No checks were disabled or removed.
- This checkpoint update creates a new source SHA. To keep release evidence exact-SHA, the independent matrix selector must now be retargeted to the resulting checkpoint commit and rerun. The successful run above remains evidence for aaa1245ac44e95eef3a564a08ba88e10aeb16433 only.
- Ledger weights and point totals are unchanged: 75/100 provisional implementation, 60/100 verified. A green cross-platform engineering matrix does not by itself complete E1–E6; no extra ledger points are awarded until each row's full acceptance evidence is met.

### Bound HTTP/2 loopback test teardown

- Source commit 88703a34885c8727ea3317a690ae0bf024028f99 wraps the HTTP/2 test server's accept loop in a five-second timeout. The client request still has to complete and return the expected status, body, and response header; the timeout only bounds test-server teardown if peer closure is delayed.
- The exact-SHA matrix for d1951f14d79e4255cc6bc9374af6c33833017bed had Linux pass and Windows reach the real-network test step; Windows completion was still pending when this change was made. That run cannot verify this teardown change.
- The next matrix must target the checkpoint SHA produced by this update.


## 2026-10-24 continuation — exact-SHA hardening and verification

- **Working branch:** `milestone/100-ecosystem-20261023`.
- **PR:** [#80](https://github.com/panglimanetizen01/one/pull/80), open/draft/unmerged; `main` unchanged.
- **Exact source SHA verified:** `64985b51599c1531b6b8e07aa0aa9d16551e6e38`. Code-bearing change `cec949631f3c91a5419957b1a8820904daa6bcd7` fixes the Windows offline verifier; the tested SHA adds verification-contract documentation. Subsequent ledger/checkpoint edits are documentation-only and do not alter the tested source code.

### Changes

- File-backed SQLite `database.query` uses SQLite read-only flags and `query_only`; missing database files are not created. Added enforced budgets for SQL bytes (1 MiB), per-parameter bytes (1 MiB), total parameter bytes (16 MiB), parameter count (256), result rows (100,000), and serialized output (16 MiB), with regression tests.
- Added strict Gregorian UTC text parsing as the source-level builtin `unix dari waktu utc`, paired with the existing `waktu utc dari unix` formatter. Both are integrated through semantic checks and canonical interpreter execution, with unit/integration tests and updated docs.
- Fixed `tools/verify_offline.ps1` false-green behavior by propagating nonzero native exit codes and skipping only the Android-native CLI test on Windows. The Android-target CLI build test passed on Ubuntu, but the separate Android AArch64 ELF build-and-execute test remains ignored on both platforms. The Windows-specific CLI Android-target test was filtered out and is not counted as a Windows pass.

### Evidence

- Focused standard-library/database regressions: [run 40006470057](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470057) — SUCCESS.
- Linux distro matrix: [run 40006470246](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470246) — Ubuntu 24.04 and Debian 12 SUCCESS.
- Exact-SHA cross-platform matrix: [run 40006470017](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470017) — Ubuntu and Windows SUCCESS for build, contract artifacts, AST tests, Clippy, workspace tests, real-network tests and platform-appropriate offline verification. Test logs show 215 core tests passed on Ubuntu and 210 on Windows. One Android AArch64 execution test remains ignored; the Windows-only CLI test is explicitly filtered out.
- The earlier run [40005692552](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40005692552) is not credited as valid full verification: its offline script log exposed a failed Android-target test followed by a false PASS. The corrected script was verified on SHA `64985...`; the new Windows log contains no `test result: FAILED` marker and ends with `OFFLINE VERIFICATION: PASS`.
- Canonical repository runs [40006462899](https://github.com/panglimanetizen01/one/actions/runs/40006462899), [40006463227](https://github.com/panglimanetizen01/one/actions/runs/40006463227), and [40006463315](https://github.com/panglimanetizen01/one/actions/runs/40006463315) failed before steps with `runner_id=0` and empty step lists. No source test result is inferred from those failures; the reviewed exact-SHA mirror fallback supplied actual Linux/Windows evidence.

### Completion ledger and remaining work

- **Implementation subtotal:** 84/100 provisional points; E1=7/10, E2=5/8, E3=8/8, E4=3/6, E5=1/4, E6=0/4. Fixed weights unchanged.
- **Verified completion:** 60/100. E1, E2, E3, E4, E5 and E6 are not fully verified under the current 100% acceptance criteria.
- Remaining mandatory work: external dependency resolution and authenticated package provenance; wider standard-library APIs; race-resistant filesystem confinement and broader quotas; compatibility and developer tooling/docs completion; Android ELF build-and-execute verification in a toolchain-capable environment if required by declared targets; and E6 forensic Definition-of-Done audit.
- **GATE: NOT PASSED — Milestone 100% remains in progress.**


## Final continuation evidence — checkpoint snapshot `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974`

- [Focused stdlib/database](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463554) — SUCCESS.
- [Linux distro matrix](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463563) — Ubuntu 24.04 and Debian 12 SUCCESS.
- [Ubuntu/Windows cross-platform matrix](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463527) — both jobs SUCCESS on exact source SHA `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974`. Both passed build, contract artifacts, AST tests, Clippy, workspace tests, real-network tests and platform-appropriate offline verification. No failed test summary was found in the logs; each platform printed `OFFLINE VERIFICATION: PASS`.
- Main workspace suite summaries: Ubuntu 215 passed / 0 failed; Windows 210 passed / 0 failed. The Android-target CLI test passed on Ubuntu but only checks artifact creation. The separate Android AArch64 ELF build-and-execute test remains ignored on both platforms because Android AArch64 Clang/runtime setup is unavailable. The Windows-specific CLI Android-target test is filtered out on Windows.
- The previous Windows false-green was corrected in `tools/verify_offline.ps1` by enabling native-command failure propagation and explicitly excluding only the Windows-inapplicable Android CLI test. The earlier run with a swallowed test failure is not credited.
- Canonical repository Actions still fail before steps with `runner_id=0`; the independent exact-SHA mirror supplied actual verification evidence.

**Current ledger:** 84/100 provisional implementation points, 60/100 verified.  
**Remaining:** external dependency resolution/provenance, wider standard library, race-resistant filesystem confinement and broader quotas, full compatibility/release and developer-tooling/documentation acceptance, and E6 forensic audit.  
**GATE: NOT PASSED — Milestone 100% remains in progress.**

This is a documentation-only checkpoint update; the verified source code snapshot is unchanged.


## Forensic correction — E3 Android runtime acceptance

The earlier 68/100 verified subtotal credited E3's 8 points based on Linux/Windows tests and an Android-target CLI build test. Source inspection confirms that `android_target_explicit_mempertahankan_build_native` only asserts an artifact was created; the separate `build_native_menghasilkan_dan_menjalankan_elf_android_aarch64` test is explicitly ignored on both platforms. Therefore the Android ELF runtime acceptance is **not verified**, and E3 is 8/8 implemented but 0/8 verified. Verified completion is corrected to **60/100**; implementation remains **84/100**. No row weights changed.

## Latest exact-SHA verification — `ebf254d1194a903a3b582f3a18abfc402f2003d2`

- Focused stdlib/database [40009389100](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389100) — SUCCESS.
- Linux distro [40009389164](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389164) — Ubuntu 24.04 and Debian 12 SUCCESS.
- Cross-platform [40009389190](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389190) — Ubuntu and Windows SUCCESS, exact source SHA verified in both logs. Build, Clippy, workspace tests, real-network tests and offline verification passed; no `test result: FAILED` marker; each platform printed `OFFLINE VERIFICATION: PASS`.
- Main workspace summaries show 215 passed / 0 failed on Ubuntu and 210 passed / 0 failed on Windows. The separate Android ELF build-and-execute test remains ignored on both platforms; no Android runtime pass is claimed. Canonical repository Actions still fail before steps with `runner_id=0`.
- **Gate: NOT PASSED** — external dependency resolution/provenance, broader stdlib, race-resistant filesystem/resource security, developer tooling/docs, Android runtime acceptance if required by the declared target contract, and E6 forensic audit remain outstanding.

## 2026-10-24 execution checkpoint — E3/E4 implementation slices

### MILESTONE
Milestone 100 — ecosystem and release quality. Active scope remains E1–E6 only.

### COMPLETION
- Implementation: **85/100 provisional** (fixed ledger weights retained; E4 implementation moved 3/6 → 4/6 because Linux/Android descriptor-relative no-follow opens are now wired into the file adapters and regression tests are present).
- Verified: **60/100**. No new verified points were awarded because the new code has not yet completed exact-SHA verification and the overall E1–E6 acceptance remains incomplete.
- The user's initial 82/100, prior 84/100 implementation, and 60/100 verified values are not interchangeable; 85/100 is the current provisional implementation subtotal only.

### DONE
- crates/one-core/tests/native_backend_android_aarch64.rs: Android runtime acceptance now checks ELF magic, ELFCLASS64, little-endian encoding, and AArch64 machine ID before pushing and executing the binary.
- .github/workflows/ci.yml: added a dedicated Android API 35 arm64 emulator job to explicitly execute the previously ignored ELF build-and-run acceptance test.
- crates/one-core/Cargo.toml and crates/one-core/src/adapter.rs: added the direct libc dependency and Linux/Android directory-descriptor traversal with no-follow opens for parent components and final files.
- crates/one-core/src/adapter.rs: added regression tests for symlinked parent directories, final-component read symlinks, and rejected write symlinks leaving the referent unchanged.
- docs/SECURITY.md: documented the actual Linux/Android guarantee and the remaining platform/confinement boundaries.
- crates/one-cli/tests/cli.rs: added a regression test that unsupported dependency declarations fail closed rather than being silently ignored.
- Independent verification mirror .github/workflows/windows-verification.yml: added an exact-SHA Android AArch64 emulator job and pinned CANONICAL_COMMIT.txt to source SHA 6361461a81718594c78921abb3106b1fdf86e1c5 before the final CLI regression was added. That selector is stale relative to the current source and must be updated to the final checkpoint SHA before evidence can be credited.
- Completion ledger updated at commit fb5ffdc542e850b25ed967f6512d1799aea9377e.

### EVIDENCE
- Code-bearing source SHA before documentation checkpoint commits: b5c3827840a691868a69428ae4032614407f1062.
- Canonical workflow runs for prior source SHA 6361461a81718594c78921abb3106b1fdf86e1c5:
  - One CI run 40051833403 — failed before steps; Android, Ubuntu, and Windows jobs had empty step lists.
  - Linux distro run 40051833390 — failed before steps; Ubuntu and Debian jobs had empty step lists.
  - Focused stdlib run 40051833400 — failed before steps; empty step list.
- These canonical failures are runner-provisioning failures, not source pass/fail evidence. The mirror is the fallback, but its current run must be inspected for exact source SHA, Android runtime job result, Ubuntu/Windows job results, and ignored/skipped counts.
- Android ELF execution and new filesystem regression tests on the current source SHA: **NOT YET VERIFIED**.

### REMAINING
- E1: complete standard-library acceptance, only where existing contracts require it.
- E2: external dependency/registry resolution, committed deterministic lock state, signed publisher identity and authenticated provenance remain unimplemented; the new test only confirms unsupported declarations fail closed.
- E3: actual Android AArch64 ELF execution must pass on an Android arm64 runtime; a build-only result is insufficient.
- E4: Windows/macOS race-resistant path resolution, explicit confinement root, broader resource quotas, and complete threat/recovery review remain open.
- E5: full compatibility/release/tooling/documentation acceptance remains open.
- E6: final forensic Definition-of-Done audit remains open and must follow the E1–E5 evidence review.

### GIT STATE
- Repository: panglimanetizen01/one.
- Branch: milestone/100-ecosystem-20261023.
- PR #80: open, draft, unmerged; base main; no merge performed.
- HEAD before this checkpoint update: fb5ffdc542e850b25ed967f6512d1799aea9377e (ledger/checkpoint evidence commit).
- After this checkpoint update, re-read PR #80 head_sha, pin the mirror selector to that exact SHA, and inspect workflow logs before updating verified score.

### NEXT
1. Pin the mirror to the new checkpoint commit SHA and inspect exact-SHA cross-platform, distro, focused stdlib, and Android emulator job logs.
2. Fix any compilation, Clippy, test, or emulator failure and rerun on the corrected SHA.
3. Continue only with demonstrated E1–E6 gaps; do not credit E3 until ELF execution succeeds.
4. Keep E2, non-Linux/Android filesystem security, and E6 open until their full acceptance evidence exists.

### GATE
**NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — E2 Cargo lockfile determinism

This entry supersedes the earlier 85/100 provisional implementation subtotal in this checkpoint; the verified subtotal is unchanged.

### MILESTONE
Milestone 100, scope locked to E1–E6.

### COMPLETION
- Implementation: **86/100 provisional**. E2 moved 5/8 → 6/8 for a committed Cargo.lock plus --locked enforcement across CI and local verification scripts. E4 remains 4/6.
- Verified: **60/100**. No new verified points awarded yet.

### DONE
- Root Cargo.lock generated by Cargo and committed; .gitignore no longer ignores it.
- Canonical CI, Linux distro workflow, focused stdlib workflow, verify-human-vnext.sh, verify_offline.ps1, and verify_offline.sh now pass --locked to Cargo build/test/run/Clippy commands.
- Exact-source lock generation was performed by independent workflow against source SHA cd4aa9f9be450ecc43a0419d057867967d23e469; lock SHA-256 84b2111b5d1ab0c942c6fc68040285bccc3065a5f4e4150814e202c0fe15bc21; 113 package entries.
- Verification policy added to docs/verification/human-vnext-exact-sha-verification-contract.md.

### EVIDENCE
- Lockfile generation: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40052223860 (success; exact canonical source SHA recorded in logs).
- Latest code-bearing source before this checkpoint documentation: d9196cf97c4f42de8a8dbf8e1de93ced5b8db4dd.
- Latest source SHA still requires exact-SHA verification after the lockfile and --locked workflow changes. Do not transfer prior pass results to the new SHA.
- E2 external One package registry, dependency graph/version-conflict handling, trusted publisher identity, and authenticated signatures remain incomplete.

### REMAINING
- E1: complete the already-defined standard-library acceptance set.
- E2: external One package dependency resolution and authenticated provenance remain open; Cargo lockfile determinism alone does not close E2.
- E3: Android arm64 ELF must execute successfully and report the actual result.
- E4: Windows/macOS race-resistant filesystem resolution, confinement root, broader resource quotas and threat review remain open.
- E5: compatibility and release readiness remain open.
- E6: final forensic DoD audit remains open.

### GIT STATE
- Branch: milestone/100-ecosystem-20261023.
- PR #80 remains open, draft, unmerged; base main.
- HEAD before this checkpoint update: c2b91deeb6d01440ff37b1254ed2f75bd042d048.
- After this checkpoint commit, re-read PR #80 head_sha, update the mirror selector to that exact SHA, and inspect exact-SHA runs.

### NEXT
1. Re-pin the independent mirror to the final checkpoint SHA.
2. Confirm Cargo --locked build, Clippy, workspace tests, network tests, focused stdlib tests and Windows jobs on that exact SHA.
3. Confirm Android arm64 ELF execution on that exact SHA.
4. Continue remaining E1–E6 gaps without changing weights or claiming PASS before evidence.

### GATE
**NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — pinned toolchain

This entry supersedes the earlier 86/100 provisional implementation subtotal above.

### MILESTONE
Milestone 100, E1–E6 only.

### COMPLETION
- Implementation: **87/100 provisional**. E5 moved 1/4 → 2/4 for a pinned Rust toolchain and documented reproducibility policy; E2 remains 6/8 and E4 remains 4/6.
- Verified: **60/100**. No new verified points awarded.

### DONE
- Root rust-toolchain.toml pins Rust 1.99.0 with rustfmt and Clippy.
- Canonical CI, Linux distro conformance and focused stdlib workflows explicitly request toolchain 1.99.0.
- Independent mirror Android, Linux distro and focused stdlib workflows explicitly request toolchain 1.99.0; mirror cross-platform commands now use --locked.
- This corrects a detected drift: a mirror focused run had resolved the moving stable channel to Rust 1.99.1 despite the lockfile having been generated under 1.99.0.

### EVIDENCE
- The successful lockfile generation log reports rustc 1.99.0 and cargo 1.99.0.
- Exact-SHA focused verification for source SHA 9e01367179ea1e5714cb6e72308656f580d5fdeb passed before explicit toolchain action inputs were added. It does not prove the final workflow revisions pass.
- Final source SHA after this checkpoint update must be read from PR #80, then pinned in the mirror selector and rerun. Current code-bearing/workflow source before the latest ledger/checkpoint docs: ec0d3472c0fa9e6551c5db1973dd4300838004f0.
- Android AArch64 ELF runtime acceptance is still in progress; no pass claim until its run log confirms the exact source SHA, ABI, ELF checks and execution exit code.

### REMAINING
- E1: full standard-library acceptance.
- E2: external One registry/dependency resolution and authenticated publisher provenance.
- E3: exact-SHA Linux/Windows matrix and actual Android arm64 runtime execution.
- E4: Windows/macOS race-resistant filesystem resolution, root confinement and full resource/threat review.
- E5: full compatibility, packaging and release readiness.
- E6: final forensic Definition-of-Done audit.

### GIT STATE
- Branch milestone/100-ecosystem-20261023; PR #80 open, draft, unmerged; base main.
- HEAD before this checkpoint update: 73d4e66a208ba5394ebfa21b41bac36d18295521.
- Pin mirror selector only after this checkpoint commit and confirm the final PR head SHA.

### NEXT
1. Pin the independent mirror to the exact new checkpoint SHA.
2. Confirm --locked and Rust 1.99.0 on Ubuntu, Debian, Windows, focused stdlib and Android runtime.
3. Repair any remaining source failures; then continue E1–E6 acceptance.

### GATE
**NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — latest E2/E3/E4/E5 evidence reconciliation

This entry supersedes prior subtotal notes in this checkpoint.

### MILESTONE
Milestone 100 — only E1–E6.

### COMPLETION
- Implementation: **87/100 provisional**.
- Verified: **60/100**. No new verified points awarded in this continuation.

### DONE
- E2: Cargo.lock is committed (113 packages; SHA-256 84b2111b5d1ab0c942c6fc68040285bccc3065a5f4e4150814e202c0fe15bc21), Cargo.lock is no longer ignored, and canonical/local/mirror Cargo build/test/run/Clippy commands use --locked.
- E5: root rust-toolchain.toml pins Rust 1.99.0; canonical and independent workflows explicitly select 1.99.0. This corrects drift observed when the moving stable channel resolved to 1.99.1.
- E3: the dedicated exact-SHA Android runtime workflow installs an Android 35 arm64-v8a emulator and explicitly invokes the previously ignored ELF build-and-execute test. The test now asserts ELF magic, ELFCLASS64, little-endian encoding and EM_AARCH64 before execution.
- E4: Linux/Android file adapters use descriptor-relative no-follow opens; regression tests cover symlinked parents, final-component read symlinks and rejected write symlinks.
- Fixed two implementation defects exposed by real CI: malformed NUL termination for absolute path roots, then strict Clippy's manual C-string lint. Fixed invalid duplicate YAML with blocks in the mirror distro workflow after its jobs failed to parse.

### EVIDENCE
- Cargo lock generation succeeded: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40052223860, generated against source SHA cd4aa9f9be450ecc43a0419d057867967d23e469.
- Updated focused stdlib/database verification passed on source SHA c740982ef40b680ac8ae8682e738272ffad05331; logs show Rust 1.99.0 and 28 --locked Cargo commands.
- That focused pass is not a substitute for full exact-SHA workspace/network/Windows/Android acceptance on the current source.
- Current code/workflow source before this checkpoint documentation: 34f2162918eaff316f68659b51d5666333160a29.
- The latest mirror Ubuntu/Debian and Windows/Linux jobs for source SHA c740982ef40b680ac8ae8682e738272ffad05331 are still in progress; Android is still installing the arm64 emulator image and has not executed the ELF. Re-read latest run IDs and logs after this checkpoint update.
- Canonical Actions on source SHA f93a960635d648d6b9390bef42ed42328bf2ddec failed before steps with empty step lists; infrastructure failure only.

### REMAINING
- E1: full standard-library acceptance remains incomplete.
- E2: One-language external dependency graph/registry resolution, conflict handling, trusted publisher identity, and signed/authenticated provenance remain incomplete despite the Cargo lockfile improvement.
- E3: actual ELF AArch64 execution on Android arm64 remains NOT YET VERIFIED.
- E4: Windows/macOS race-resistant path resolution, explicit confinement root and full resource/threat review remain open.
- E5: full compatibility/release readiness remains open.
- E6: final forensic Definition-of-Done audit remains open.

### GIT STATE
- Repository panglimanetizen01/one; branch milestone/100-ecosystem-20261023.
- PR #80 remains open, draft, unmerged; base main.
- HEAD before this checkpoint update: 86e902c08fe536e33ca67812a2d185bcd4f0a6f7.
- After this checkpoint commit, read PR #80 head_sha and update mirror CANONICAL_COMMIT.txt to that exact SHA before crediting final evidence.

### NEXT
1. Pin mirror to the new checkpoint SHA and rerun exact-SHA verification.
2. Fix any failure in --locked, toolchain, workspace/network tests or Windows.
3. Finish Android emulator boot and inspect ELF execution result; if it fails, fix the cause and rerun.
4. Continue E1–E6 acceptance, then perform final forensic audit.

### GATE
**NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — Linux pass, exact-SHA command audit, Android pending

This entry supersedes earlier status summaries above.

### MILESTONE
Milestone 100, E1–E6 only.

### COMPLETION
- Implementation: **87/100 provisional**.
- Verified: **60/100**; no verified score increase.

### DONE
- Committed Cargo.lock and Rust 1.99.0 pin remain in place.
- release-check.yml and tools/verify_exact_sha.sh now use --locked as well; recursive audit found no unlocked Cargo build/test/run/Clippy command in source workflows/scripts.
- Fixed the mirror Linux distro workflow's duplicate YAML with blocks; the valid workflow now passes.

### EVIDENCE
- Focused standard-library/database run passed on source SHA 102741eddac431cfe4f1b4b24bec3d7a2032428b, using Rust 1.99.0 and 28 --locked Cargo commands: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40052736966
- Linux distro run passed on the same source SHA on Ubuntu 24.04 and Debian 12, each with Rust 1.99.0, seven --locked Cargo commands, and 64 successful test-result groups: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40052736970
- Those passes include the new Linux filesystem no-follow regression tests and unsupported dependency declaration fail-closed test. They do not prove the newer SHA after release-check.yml and tools/verify_exact_sha.sh changes.
- Windows cross-platform run on SHA 102741 was cancelled by a subsequent mirror workflow update after the test steps had completed but before cleanup; it is not counted as a full pass.
- Android arm64 runtime job on SHA 102741 is still at emulator boot and has not executed the ELF. The revised bounded-boot workflow is now running separately and will preserve emulator diagnostics if boot fails.
- Latest source before this checkpoint update: 9bf679bb52810031a533025cbc5a18e97450d63b. After this update, pin the mirror to the new PR head SHA and rerun all gates.

### REMAINING
- E1: complete the full standard-library acceptance contract.
- E2: One-language registry/dependency resolution and authenticated publisher provenance.
- E3: complete exact-SHA Windows/Ubuntu matrix and actual Android AArch64 ELF execution.
- E4: Windows/macOS race-resistant filesystem resolution, confinement root, and full threat/resource review.
- E5: compatibility, package distribution and release-readiness audit.
- E6: final forensic Definition-of-Done audit after E1–E5 evidence is complete.

### GIT STATE
- Branch milestone/100-ecosystem-20261023; PR #80 open, draft, unmerged; base main.
- HEAD before this checkpoint update: 9f2eae322104ecd4a655b4e5673e329ca537100f.
- Update mirror CANONICAL_COMMIT.txt after this checkpoint update to the exact new source head.

### NEXT
1. Pin the mirror to the new checkpoint SHA.
2. Complete the exact-SHA Windows/Ubuntu and Linux distro jobs.
3. Resolve Android emulator boot; run and inspect ELF execution exit code and logs.
4. Continue remaining E1–E6 gaps; do not claim 100/100 until all mandatory acceptance is proved.

### GATE
**NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — bounded package provenance and exact-SHA matrix

### MILESTONE
Milestone 100 — scoped strictly to E1-E6.

### COMPLETION
- Implementation: **88/100 provisional**. E4 moved from 4/6 to 5/6 for bounded package/provenance reads plus a regression test. The fixed ledger weights are unchanged.
- Verified: **60/100**. No milestone-100 verified points are awarded yet.

### DONE
- Package signing and verification now bound input reads: artifact 64 MiB, private-key seed 256 bytes, sidecar 16 KiB, and publisher trust store 1 MiB. Oversized input is rejected after reading at most limit + 1 bytes.
- Added a regression test for the bounded-read helper and documented the limits in SECURITY.md and the package/build contract.
- Corrected E2 ledger wording: Ed25519 signing and separately trusted publisher verification are implemented; the external One package registry, dependency graph/version conflict resolution, automatic package signing and full build-provenance envelope remain open.

### EVIDENCE
- Source SHA tested: `0a0f250dfc2e556bca40f2201752ee5d06f56854`.
- Focused standard-library/database workflow [40053737845](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40053737845) passed with Rust 1.99.0 and 28 `--locked` Cargo invocations.
- Linux distro workflow [40053737844](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40053737844) passed on Ubuntu 24.04 and Debian 12. Each log confirms source SHA `0a0f250dfc2e556bca40f2201752ee5d06f56854`, Rust 1.99.0, seven `--locked` commands and 64 successful test-result groups.
- Cargo lock bootstrap workflow [40053737791](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40053737791) passed for the same selected source SHA.
- Cross-platform workflow [40053737784](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40053737784) is still running; Ubuntu has passed the main test steps, but Windows has not completed the full job.
- Android arm64 workflow [40053737740](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40053737740) is still running; emulator boot and actual ELF execution are not yet proven.

### REMAINING
- E1: complete all standard-library acceptance criteria, not just the focused subset.
- E2: external One registry/dependency graph resolution, version-conflict handling, automatic signing and full build provenance.
- E3: complete Windows exact-SHA job and actual Android AArch64 ELF execution with ABI, ELF-header checks, exit code and logs.
- E4: Windows/macOS race-resistant filesystem resolution, explicit root confinement and complete threat/resource review.
- E5: full compatibility and release-readiness review.
- E6: final forensic Definition-of-Done audit after E1-E5 evidence is complete.

### GIT STATE
- Repository: `panglimanetizen01/one`; branch `milestone/100-ecosystem-20261023`.
- PR #80 remains open, draft, unmerged, based on `main`.
- Source HEAD before this checkpoint update: `0d74116d11bcd30c10bd42e6bd118d69afc7b084`.
- After this checkpoint commit, update mirror `CANONICAL_COMMIT.txt` to the new exact PR head and rerun the required workflows. Prior SHA evidence remains historical for that prior source snapshot.

### NEXT
1. Pin mirror to the post-checkpoint exact SHA and rerun lockfile, focused stdlib, Linux distro, cross-platform and Android runtime workflows.
2. Fix any current-SHA failure rather than relying on the prior pass.
3. Complete the minimum E2 dependency-resolution acceptance within the existing package contract without building a large registry service.
4. Continue E1-E6 and perform the final forensic audit only after the prerequisite evidence is complete.

### GATE
**NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — module path confinement regression coverage

### MILESTONE
100% ecosystem/release, scope limited to E1-E6.

### COMPLETION
Implementation **88/100 provisional**; verified **60/100**. No score change from this continuation: added regression coverage is awaiting the exact-SHA cross-platform run. Fixed ledger weights remain unchanged.

### DONE
- Added Linux/Android module-reader regression tests for parent-directory traversal and symlink escape in `crates/one-core/src/human_compiler.rs`.
- The existing reader opens root/path components descriptor-relatively with `openat` + `O_NOFOLLOW`, requires a regular file, and bounds module source to 16 MiB. Non-Linux/Android currently uses canonical-path containment and is **not claimed race-resistant**.
- Source commit with these tests: `dbbd26bf0e9f56ce0e4fe82292297dbf4eee5686`.

### EVIDENCE
- Exact-source focused standard-library/database run [40054619379](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40054619379): **success**.
- Exact-source Cargo lockfile bootstrap [40054619371](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40054619371): **success**.
- Exact-source Linux distro [40054619377](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40054619377), cross-platform [40054619427](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40054619427), and Android AArch64 [40054619387](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40054619387) were still in progress at checkpoint time. Their outcomes are not credited as PASS here.

### REMAINING
- E1 full standard-library acceptance beyond the focused suite.
- E2 reconcile dependency behavior against the normative package contract: executable v0.1 explicitly supports an empty dependency set only; do not invent a registry requirement beyond that contract. Package signature/trust-store and complete provenance acceptance still require audit.
- E3 complete exact-SHA cross-platform and Android ELF execution evidence.
- E4 race-resistant confinement/root controls on Windows/macOS and full threat/resource review.
- E5 compatibility and release-readiness audit.
- E6 final forensic DoD audit on the final release SHA.

### GIT STATE
PR #80 remains open, draft and unmerged; branch `milestone/100-ecosystem-20261023`, base `main`. The source SHA above was current before this checkpoint update. Re-pin the verification mirror to the resulting PR head and rerun required exact-SHA workflows; do not treat evidence for the prior SHA as evidence for the checkpoint commit.

### NEXT
1. Update the mirror's `CANONICAL_COMMIT.txt` to the post-checkpoint PR head and rerun focused, lockfile, Linux, Windows cross-platform and Android runtime workflows.
2. Fix any current-SHA failure, especially compiler/test failures from the new confinement regression tests.
3. Finish E1-E6 acceptance and update the ledger only with criteria and evidence actually passing.

### GATE
**NOT PASSED — exact-SHA cross-platform/Android verification and mandatory E1-E6 acceptance remain incomplete.**


## 2026-10-24 continuation — latest Android CI audit

### CURRENT STATE
- Code/workflow change under review: `4291a820e07b99820f1789f344bc2ab427f7f9d5`.
- Verification-ledger documentation update: `0e21cce77aae4302ce61cafdfefdad1e04ad4d5e`.
- PR #80 remains open, draft and unmerged; no merge was performed.
- Implementation remains **88/100 provisional**; verified remains **60/100**. Do not promote unverified implementation points to verified points.

### WORK COMPLETED
- Replaced the Android SDK license pipeline's blanket `|| true` with explicit capture and checking of `sdkmanager`'s exit status while tolerating `yes` receiving SIGPIPE.
- Inspected the Android AArch64 acceptance test. It verifies ELF magic/class/endianness/machine type, then executes the generated binary on the Android emulator and checks exact output.
- Recorded the latest run IDs and their evidence limits in `docs/verification/one-completion-ledger.md`.

### EXACT-SHA EVIDENCE
- One CI: [40060363429](https://github.com/panglimanetizen01/one/actions/runs/40060363429) — failure.
- Focused standard-library verification: [40060363449](https://github.com/panglimanetizen01/one/actions/runs/40060363449) — failure.
- Linux distribution conformance: [40060363463](https://github.com/panglimanetizen01/one/actions/runs/40060363463) — failure.
- For all six jobs, the available job-step endpoint returned an empty step list, log retrieval returned `BlobNotFound`, and no artifacts were listed. Therefore no successful test execution is evidenced and no specific failing test can be responsibly identified from these run records.

### REMAINING — STRICT E1–E6 SCOPE
1. Obtain usable exact-SHA workflow evidence; inspect actual steps/logs and repair any concrete failure rather than guessing.
2. Complete E1 standard-library contract coverage.
3. Complete E2 package/dependency contract audit; do not overstate external registry or dependency-resolution support.
4. Complete E3 Linux/Windows verification and actual Android AArch64 ELF runtime execution on the selected exact SHA.
5. Complete E4 resource, path confinement and cross-platform race-resistance review; do not claim Windows/macOS race-resistant confinement without implementation and tests.
6. Complete E5 compatibility/release readiness audit against the declared support contract.
7. Complete E6 forensic Definition-of-Done audit and reconcile ledger/checkpoint to the final tested SHA.

### GATE
**NOT PASSED — do not label GREEN LOCKED or release-ready until every required criterion has evidence on the recorded source SHA.**


## 2026-10-24 follow-up: runner provisioning evidence

- Ledger clarification commit: `e5b31ed1cc558ca0a6d2095fe3a6d4f631527f01`.
- The latest failed canonical jobs expose `runner_id: 0`, empty `steps: []`, and completion within a few seconds. This supports a runner/job provisioning failure before any test step, not a conclusion about source correctness.
- The two failed workflows were retried; they again had no steps and no retrievable logs. Avoid endless retries without new runner evidence.
- The independent mirror selector on branch `verification` is pinned to `1c450863cdfcb824dcf7cef6b64e86e58bc25b44`. The mirror workflow must fetch private source using `CANONICAL_READ_TOKEN`; successful mirror execution has not yet been observed, so do not count this as verification.
- This checkpoint update itself must be treated as documentation only. Final verification still needs to target the exact resulting branch HEAD and successful runnable evidence.
- **Gate remains NOT PASSED.** No score increase.


## Branch-head reconciliation — 2026-10-24

- Previously observed latest code/documentation commit: `6826e3a1b41525bbfa5d3f02dda100c7e59a7004`, updating signed package provenance documentation to bind the validated package format.
- Canonical exact-SHA runs for that SHA are listed in the completion ledger; all returned `runner_id: 0`, empty step arrays, and failure before any test step. This does not establish source tests as failing or passing.
- Completion-ledger reconciliation commit: `5b2d28ac4ea596ab211b8cfd5a925a59611924a7`.
- This checkpoint is now the current branch update and must be the next exact-SHA verification target. The mirror's previous selector is behind this documentation commit and must be updated before its run can verify the final branch HEAD.
- **Gate remains NOT PASSED.** No merge or release claim.


## 2026-10-24 continuation — exact-SHA reconciliation after filesystem audit

### CURRENT BRANCH STATE AT CHECKPOINT PREPARATION
- PR #80 head at the start of this checkpoint update: `b4f19b7c2a401d4e97d4e998937264673903b15e`; PR remains open, draft, and unmerged.
- The completion ledger was corrected to reflect current Unix descriptor-relative no-follow filesystem opens (Linux/Android/macOS) and Windows reparse-point/opened-handle validation. The remaining E4 gaps are explicit root confinement, broader runtime-wide quotas, and full threat review; E4 remains 5/6 provisional, 0/6 verified.
- The verification mirror selector was advanced to `b4f19b7c2a401d4e97d4e998937264673903b15e`.

### EXACT-SHA VERIFICATION RUNS FOR `b4f19b7c2a401d4e97d4e998937264673903b15e`
- Focused standard-library/database regressions: **PASS**, [run 40066726176](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066726176).
- Cargo lockfile bootstrap evidence: **PASS**, [run 40066726181](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066726181).
- Cross-platform Ubuntu/Windows matrix: [run 40066726184](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066726184), in progress at checkpoint preparation.
- Ubuntu 24.04 and Debian 12 conformance: [run 40066726205](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066726205), in progress at checkpoint preparation.
- Android AArch64 ELF build-and-execute: [run 40066726187](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066726187), in progress at checkpoint preparation on a macOS ARM64 host. The earlier x86-host TCG attempt timed out booting the emulator; the Linux ARM64 attempt could not locate a compatible Android emulator package. Neither failed attempt counts as Android runtime acceptance.
- Canonical private-repository CI for this SHA again failed before any executable job steps; the available job records contain no steps. The independent mirror provides usable exact-SHA execution evidence and remains the fallback.

### REMAINING ACCEPTANCE — DO NOT EXPAND SCOPE
1. Finish all exact-SHA workflows above and inspect actual logs/artifacts. Any documentation-only change after them creates a new verification target.
2. Award no E3 verification points unless the Android AArch64 ELF is built, pushed to Android, executed, and returns the expected output.
3. Keep the fixed ledger totals at **88/100 provisional implementation, 60/100 verified** until every relevant E1–E6 criterion is accepted with evidence.
4. Continue remaining E1 standard-library breadth; E2 final release signing/verification integration and trusted source-bound provenance acceptance (external registry/version-conflict resolution is outside the executable v0.1 subset); E4 root confinement/quotas/threat review; E5 release documentation/tooling review; and E6 final forensic audit. Do not represent implemented features as fully verified.
5. Reconcile the checkpoint and ledger with the final exact-SHA results, then run the required verification against the resulting branch head before any final release claim.

### GATE
**NOT PASSED.** This checkpoint is a continuation record, not release approval. PR #80 is not merged; no PASS GREEN LOCKED or final release claim is authorized by the current evidence.



## 2026-10-24 scope clarification — E2 package acceptance

- The completion ledger now reconciles E2 against the approved `docs/spec/one-v0.1-package-build.md`: external registry and dependency/version-conflict resolution are outside the executable v0.1 subset, whose supported dependency set is empty and whose unsupported declarations must fail closed.
- E2's actual remaining acceptance is release-process integration and exact-SHA evidence that the final built artifact receives Ed25519 V2 source-bound provenance signing and is verified against an independently provisioned trusted-publisher store. V1 artifact-only signatures do not satisfy this gate.
- The ledger/checkpoint correction changes documentation only; it does not award verified points. The branch head created by this checkpoint update is a new exact-SHA verification target.



## 2026-10-24 continuation — release provenance gate and release-document reconciliation

### CURRENT SOURCE REVISION BEFORE THIS CHECKPOINT UPDATE
- PR #80 source head before this checkpoint change: `b90c2bd630416cc77f0d84658d9458d459f98eb3`; PR remains open, draft, and unmerged.
- `.github/workflows/release-check.yml` now includes an explicit `one-cli` integration test for package provenance signing, trusted-publisher verification, tamper rejection, and source-tree mismatch rejection: `cargo --locked test -p one-cli --test cli package_signature_cli_authenticates_trusted_publisher_and_detects_tampering -- --exact`.
- `docs/release/one-v0.1.md` now distinguishes the historical v0.1 baseline from the current milestone-100 candidate and records the current release gate as NOT PASSED.
- E2 scope was reconciled to the approved package-build contract: external registry and dependency/version-conflict resolution are outside the executable v0.1 subset; the actual remaining gate is final release-process signing of the final artifact with V2 source-bound provenance and verification against an independently provisioned trusted-publisher store. The automated test uses an isolated test key/trust store and is not proof of a real published artifact signed with the release publisher's key.

### EXACT-SHA CHECKS STARTED FOR `b90c2bd630416cc77f0d84658d9458d459f98eb3`
- Cross-platform Ubuntu/Windows: [run 40067125143](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40067125143), queued when checked.
- Ubuntu/Debian conformance: [run 40067124952](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40067124952), in progress when checked.
- Focused standard-library/database: [run 40067124848](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40067124848), in progress when checked.
- Cargo lockfile evidence: [run 40067124879](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40067124879), in progress when checked.
- Android AArch64 ELF build-and-execute on macOS ARM64: [run 40067124955](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40067124955), queued when checked.
- Canonical private-repository CI remains unusable as source-level evidence because jobs fail before executable steps. The independent mirror remains the runnable verification path and must target the exact resulting PR head.

### REMAINING
1. This checkpoint edit creates a new PR head and therefore a new exact-SHA verification target; update the mirror selector and rerun every required workflow after this commit.
2. E1 standard-library breadth, E2 final publisher-key release integration, E4 explicit root confinement/general quotas/full threat review, E5 full compatibility/tooling review, and E6 final forensic acceptance remain incomplete.
3. Keep scores at **88/100 provisional implementation and 60/100 verified** until evidence supports any change. Do not award E3 verified points without actual Android ELF execution success on the final SHA.

### GATE
**NOT PASSED.** No merge, release tag, or PASS GREEN LOCKED claim until the final exact-SHA matrix and all mandatory E1–E6 acceptance criteria are complete.


## 2026-10-24 continuation — E2 release integration and exact-SHA revalidation

- New source-bound package release workflow: `.github/workflows/package-provenance-release.yml`. It requires the protected `one-release` environment and externally provisioned `ONE_RELEASE_SIGNING_SEED` plus `ONE_TRUSTED_PUBLISHERS` secrets, builds the committed project package, signs the final artifact with V2 provenance, verifies publisher trust and source-tree revision/digest, and uploads the package, sidecar, and digests.
- Release operations are documented in `docs/RELEASE_PROVENANCE.md`. The workflow has not yet been dispatched with real protected environment configuration, so the operational release gate remains open.
- The CLI regression now packages the committed fixture using `one package` before signing and verifying it. An initial exact-SHA build caught a missing `artifact_bytes` binding; commit `435fefdabc2b90d52883e08ef6edd9dc82b9173f` corrected the regression.
- Latest source SHA at this checkpoint update is the new branch head after this documentation commit. Re-read the canonical branch head and update the independent verification selector to that exact SHA before accepting any run. The final release claim remains prohibited until Android AArch64 execution, cross-platform and distro checks, the complete E1–E6 acceptance, and the final forensic DoD audit all pass on the final source revision.
- **Gate: NOT PASSED. Verified score remains 60/100; no points are awarded from workflow/code presence alone.**


- Added committed release fixture `examples/package-release` and documented it for the protected manual workflow. The next acceptance run must use the canonical branch head after this checkpoint update. The release environment has not been dispatched or approved; E2 remains unverified.


- Release workflow hardening: the workflow validates the publisher ID and signing-seed shape, rejects project paths outside the checkout or containing control line breaks, checks the selected publisher against the protected trust store, and uploads the exact package path emitted by the build step. This is not an actual signed release run; environment protection and independently provisioned trust material remain operational acceptance requirements.
- Canonical source head after this checkpoint update must be re-read and pinned in the independent verification mirror before final claims. Keep the gate NOT PASSED until all required E1–E6 criteria and exact-SHA platform checks pass.


## 2026-10-24 continuation — E4 filesystem root scope

- Added `ScopeRule::Root`, CLI `--root <directory>` for filesystem read/write capabilities, path-component boundary regressions, and a CLI inside-root/outside-root regression. Existing exact-target grants remain unchanged when `--root` is omitted.
- Documented the limitation: this is an explicit policy boundary combined with the existing no-follow/reparse-point adapter checks, not a complete OS sandbox. Runtime-wide memory/open-resource quotas and the full threat review remain outstanding.
- Re-read the canonical branch head after this checkpoint commit and pin the verification mirror to that exact SHA. Do not award E4 points until the final exact-SHA test matrix and threat/DoD review pass.


## 2026-10-24 continuation — documentation coverage

- Added a source-level Standard Library API reference and an end-to-end developer verification/release guide; expanded diagnostic troubleshooting and CLI root-scope documentation; linked the new guides from documentation indexes and clarified candidate status.
- Documentation examples and links must be reviewed against the final code; no E5 verified points are awarded solely from file presence.
- Re-read the canonical branch head after this checkpoint update and pin the independent verification selector to that exact SHA. Current gate remains NOT PASSED; verified score remains 60/100.


- Documentation review follow-up: repaired literal escaped-newline text in `docs/README.md` so the verification map renders as separate links. This is documentation-only and still needs exact-SHA acceptance after the current candidate is pinned.


- Security contract note: the `--root` rule is recorded as a defense-in-depth partial policy only. Root-relative descriptor anchoring, race-resistant resolution, mount behavior, and cross-platform path/case semantics still need implementation/threat review; E4 is not accepted from lexical containment alone.


## 2026-10-24 continuation — documentation gate and final verification pin

- Added user-facing `docs/panduan-pengguna/provenance-rilis.md`, linked it in both indexes, and kept the development guide's internal links inside the user-doc tree. The guide now states that the manual release workflow is dispatchable after it reaches the default branch.
- Added `tools/check_user_docs.py` and its regression tests to canonical CI and the independent mirror's Linux/Windows contract-artifact stage. The mirror run must confirm the new relative-link/status check on the final candidate.
- Android hosted-runner failure was traced to unavailable HVF acceleration (`failed to initialize HVF: Invalid argument`). The mirror Android workflow now requests software emulation with `-accel off`; this is a workaround attempt, not evidence of successful Android execution.
- This checkpoint/ledger update changes the source SHA. Re-read PR #80 head, set the mirror selector to the exact new SHA, and require a fresh complete matrix after the documentation updates. Earlier successful Linux/Windows, distro, focused stdlib/database, and lockfile runs are evidence for their recorded SHAs only.
- **Scores remain 88/100 provisional implementation and 60/100 verified. Gate NOT PASSED.** E1–E6 acceptance, actual Android ELF execution, protected release dispatch, and final forensic DoD audit remain open.


## 2026-10-24 — E2/E5 implementation score reconciliation

- E2 provisional implementation increases 6/8 → 7/8 because the protected release workflow is now present; protected-environment dispatch and independently provisioned trust-store acceptance remain open.
- E5 provisional implementation increases 2/4 → 3/4 because the user-facing release provenance guide, CLI signing/verification documentation, and enforced documentation consistency checks are now present. Tested user-workflow review remains open.
- **Implementation subtotal: 90/100 provisional. Verified completion: 60/100. Gate: NOT PASSED.** This checkpoint commit changes the source SHA; repin the verification mirror and rerun every required check. Android AArch64 ELF execution is still mandatory for E3.


## 2026-10-24 continuation — ledger arithmetic reconciliation

### MILESTONE
100% — continue E1–E6 only; do not widen scope.

### COMPLETION
- Fixed-weight implementation subtotal: **91/100 provisional**. This follows the unchanged ledger rows: 60 previously verified foundation/core points plus 31 provisional E1–E5 implementation points.
- Verified completion: **60/100**. No E1–E6 verified points are awarded by this arithmetic correction.

### DONE
- Reconciled a reporting inconsistency: the ledger row values sum to 91/100, while its total row and prior checkpoint summaries said 90/100. Updated the ledger total and recorded the arithmetic basis. No weights or row-level scores changed.

### EVIDENCE
- Prior candidate source SHA: `f46b95ebdd0e26c5ba17f6acb953538f32401aeb`.
- Prior candidate exact-SHA mirror runs: cross-platform [40085406746](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40085406746), Linux distro [40085406859](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40085406859), focused stdlib/SQLite/provenance [40085406818](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40085406818), and Cargo lockfile [40085406783](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40085406783) passed on that prior SHA.
- Android AArch64 ELF execution [40085406752](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40085406752) was still in progress when inspected; its boot step had not yet completed and ELF execution had not started. Do not count Android as passed.
- This documentation correction creates a new candidate SHA. Prior passes do not verify the new branch head and must be rerun against the exact resulting SHA.

### FAILURE / BLOCKED
- E1 full standard-library acceptance remains incomplete.
- E2 protected `one-release` dispatch with independently provisioned signing seed and trusted-publisher store has not been demonstrated.
- E3 Android AArch64 ELF runtime acceptance remains unproven; exact-SHA platform reruns are required after this commit.
- E4 filesystem root/mount confinement, parent-component TOCTOU disposition, unsupported-platform fallback guarantees, and runtime-wide resource quotas remain open.
- E5 exact-SHA acceptance must be rerun for the resulting candidate.
- E6 final forensic audit remains blocked on E1–E5 acceptance.
- Canonical repository Actions have failed before executable steps (`runner_id: 0`, `steps: []`); those runs are infrastructure failures, not source-test evidence.

### GIT STATE
- Branch: `milestone/100-ecosystem-20261023`.
- PR #80: open, draft, unmerged; base `main`.
- Ledger arithmetic correction committed to the feature branch. The checkpoint update is being committed next; then the resulting branch HEAD must be re-read and pinned in the verification mirror.

### NEXT
1. Read the final PR head SHA after this checkpoint commit.
2. Update the independent verification selector to that exact SHA and rerun focused, lockfile, Linux distro, cross-platform, and Android AArch64 workflows.
3. Continue closing E1–E5 gaps and complete E6 only after direct acceptance evidence exists.
4. Do not merge PR #80 or claim PASS GREEN LOCKED without explicit authorization and complete evidence.

### GATE
**NOT PASSED — milestone 100% remains in progress.**


## 2026-10-24 — Prompt 1/4 continuation: E1 JSON regression attempt

### MILESTONE
Prompt 1/4 — E1 standard-library contract audit and regression-test foundation. This checkpoint is **not a completed milestone**.

### LIVE STATE
- Branch: `milestone/100-ecosystem-20261023`.
- PR #80 remains open, draft, and unmerged; base branch is `main`.
- The historical checkpoint SHA `54f3de667e05075805fcc5cd5dc621b653075667` is no longer the branch HEAD.
- Source changes made during this continuation:
  - `crates/one-core/src/stdlib.rs`: added a regression test for malformed JSON string escapes, invalid surrogate pairs, invalid container syntax, trailing input, invalid integer forms, and valid negative zero.
  - Follow-up commits corrected the test fixture encoding and removed an extra closing brace.
- Test-related commits: `d4aefb440f787910c0f7b4132f880160c41c5966`, `376bf49a8ced6e2fd64184ef4393f79f75ebf724`, `7502ffbb92d71bf01964a2f3c468563d3b91654e`, and syntax correction `9a9e31180bb37227ace697433a6a24313a695ddf`.
- Ledger documentation update: `52cbfc2243f92407f4e8a0c8c11b016d7f95d0de`.
- Current source branch HEAD at time of this checkpoint: `52cbfc2243f92407f4e8a0c8c11b016d7f95d0de`.

### EVIDENCE
- Focused standard-library workflow after the regression-test changes: [run 40099262558](https://github.com/panglimanetizen01/one/actions/runs/40099262558), conclusion FAILURE.
- Workspace CI: [run 40099262555](https://github.com/panglimanetizen01/one/actions/runs/40099262555), conclusion FAILURE.
- Linux distribution conformance: [run 40099262553](https://github.com/panglimanetizen01/one/actions/runs/40099262553), conclusion FAILURE.
- The available GitHub connector returned no job step details and could not retrieve the job log blobs (404). Therefore the failure cause for these attempts is UNKNOWN from the accessible evidence. Do not call these runs green or claim that the new test executed successfully.
- Subsequent workflow attempts were triggered by the syntax correction and ledger update; they must be inspected against the resulting final HEAD. Any result from a prior SHA is historical evidence only.

### E1 STATUS
- **IMPLEMENTED: PARTIAL (unchanged at 7/10). VERIFIED: 0/10.**
- The standard-library specification explicitly includes deterministic bounded JSON conversion and strict UTC conversion, text/list/map helpers, checked arithmetic, stable errors, and compatibility across reference interpreter/compiler backends.
- The new regression test is a test addition, not proof that the contract passes. The full public API inventory and complete contract acceptance have not been established in this session.
- Do not award verified points until the required tests execute successfully on an identified SHA and broader E1 acceptance is demonstrated.

### BLOCKERS / NEXT ACTIONS
1. Inspect the exact failed workflow jobs for current HEAD using a GitHub connection that can read private-repository Actions logs; distinguish runner/permission failures from compiler/test failures.
2. Run the focused standard-library test and workspace checks successfully on the exact candidate SHA. If Actions remains unavailable, run the repository-prescribed offline verifier in an authorized checkout and record it as offline evidence, not hosted CI.
3. Verify the new JSON test fixtures compile and behave as intended; fix any actual assertion or parser defects, then rerun the tests.
4. Complete the API-by-API acceptance matrix from `docs/spec/one-v0.1-standard-library.md`, including positive/negative inputs, limits, overflow, determinism, diagnostics, and backend compatibility.
5. Rerun required checks after the latest ledger/checkpoint documentation commits and reconcile E5's historical exact-SHA evidence without granting it current-SHA acceptance prematurely.
6. Continue to E3 platform matrix only after tracking its separate Android AArch64 ELF execution requirement.

### GATE
**NOT PASSED — Prompt 1/4 remains in progress. No final release claim is authorized.**


## 2026-10-24 — Prompt 1/4 correction and latest exact-SHA status

### CORRECTION
The previous test-edit sequence accidentally removed the closing delimiter of the Rust test module. This was identified by inspecting the source's raw brace balance and fixed in commit `e51244958228ab4257aa63ee2411e25c673a9316` by restoring the module-closing brace. Do not treat runs from before this correction as evidence about the corrected source.

### LATEST SOURCE / DOCUMENTATION COMMITS
- Rust regression test and module delimiter correction: `e51244958228ab4257aa63ee2411e25c673a9316`.
- E1 ledger evidence-boundary update: `c9db4a40243a94497c0010eb509714a3cdee96db`.
- This checkpoint update is the next commit; the resulting branch HEAD must be reread after the write.

### EXACT-SHA CANONICAL WORKFLOW EVIDENCE
On source commit `e51244958228ab4257aa63ee2411e25c673a9316`:
- Focused stdlib [run 40099635185](https://github.com/panglimanetizen01/one/actions/runs/40099635185): FAILURE.
- Workspace CI [run 40099635191](https://github.com/panglimanetizen01/one/actions/runs/40099635191): FAILURE.
- Linux distro [run 40099635162](https://github.com/panglimanetizen01/one/actions/runs/40099635162): FAILURE.
- Direct job API details establish all these jobs have `runner_id: 0` and `steps: []`. They failed before runner execution. This is strong evidence of a GitHub Actions infrastructure/runner allocation failure, **not** evidence that the source tests passed or failed.
- The private repository's job-log download endpoint is unavailable through the current connector (404 BlobNotFound). No test output or artifact proving the new regression passed is accessible.

### CURRENT ACCEPTANCE
- E1 remains **7/10 implementation, 0/10 verified**. The new test is not accepted as executed evidence; the full standard-library API contract matrix remains incomplete.
- The branch head has advanced beyond the original checkpoint. Any prior evidence must remain bound to its original SHA.
- PR #80 remains open, draft, and unmerged. Do not merge or publish a release as part of this checkpoint.

### NEXT ACTIONS — PROMPT 1 REMAINS OPEN
1. Obtain a working authorized Actions runner/log path or run the repository-prescribed offline verification in a legitimate checkout. Keep offline evidence distinct from hosted CI.
2. Execute the focused stdlib test, workspace suite, and lint/Clippy against the exact candidate SHA; fix any real compile/test failure, then rerun.
3. Complete the specification-derived API acceptance matrix for JSON, UTC, text, lists, mappings, checked arithmetic, diagnostics, determinism, and backend compatibility.
4. Reconcile the mirror's canonical SHA and rerun required exact-SHA checks after the final source/documentation commit.
5. Proceed to Prompt 2 only after E1 acceptance is proven and the handoff contains the exact verified SHA.

### GATE
**NOT PASSED — Prompt 1/4 is incomplete. No E1 verified points are awarded.**


## 2026-10-24 — Prompt 2/4 E2 packaging/provenance acceptance audit

### Candidate and immutable evidence
- Candidate branch: `milestone/100-ecosystem-20261023`; PR #80 remains open/draft/unmerged.
- Audited candidate SHA: `a4a146ae22c6629fe606185481055b667321d203`.
- Package/build contract: `docs/spec/one-v0.1-package-build.md` explicitly scopes executable v0.1 to an empty external dependency set; unsupported dependency declarations must fail closed.
- Existing implementation includes deterministic package/build and human-core module-lock tests, stale-lock rejection (`ETOOL006`), `Cargo.lock` with `cargo --locked`, Ed25519 V2 source-bound provenance, source tree digest/revision capture, and trusted-publisher verification.

### Exact-SHA CI and retry outcome
- [One CI run 40103499619](https://github.com/panglimanetizen01/one/actions/runs/40103499619), [Linux distribution run 40103499599](https://github.com/panglimanetizen01/one/actions/runs/40103499599), and [focused stdlib run 40103499595](https://github.com/panglimanetizen01/one/actions/runs/40103499595) report failure on candidate SHA `a4a146ae22c6629fe606185481055b667321d203`.
- Their inspected jobs report `runner_id: 0` and empty step lists. A rerun request was accepted by the API, but the same jobs still expose no executable steps. Private job-log download returns `BlobNotFound`. Therefore these are runner/infrastructure evidence, not test-pass evidence and not enough to diagnose a source defect.
- No source or workflow code was changed during this E2 audit. The ledger row was updated in commit `e808ab0dd5fd135f537101b27f3458bd331775f1` to record this evidence and preserve E2 at 0/8 verified points.

### Protected release acceptance — NOT VERIFIED
- Workflow: `.github/workflows/package-provenance-release.yml`; it requires protected GitHub Actions environment `one-release`, secret `ONE_RELEASE_SIGNING_SEED` (32-byte hex), and secret `ONE_TRUSTED_PUBLISHERS` populated from an independently authenticated publisher-key process.
- The workflow signs the final `.onepkg` with Ed25519 V2 provenance, then verifies against the supplied trust store and exact source tree before uploading package, provenance sidecar, and SHA-256 digests.
- The available GitHub integration does not provide a workflow-dispatch action or authorized access to environment secret values / environment protection configuration. No protected release run, trusted publisher signature, or release artifact was available to verify. Do not place secrets in chat, commits, logs, or artifacts. Do not substitute checksums for signature verification.
- No ledger point is awarded. E2 remains **IMPLEMENTED — PARTIAL, 0/8 VERIFIED**. Do not label PASS GREEN LOCKED.

### Required next actions
1. An authorized repository administrator verifies environment reviewers and provisions the signing seed and independently sourced publisher trust store as GitHub environment secrets; no secret value should be disclosed.
2. Dispatch `ONE Package Provenance Release` from the reviewed candidate ref using project `examples/package-release` and its trusted publisher ID; inspect approval and all job steps.
3. Independently verify the uploaded sidecar against the separately authenticated trust store and verify source revision/tree digest, artifact digest, metadata, and package bytes. Exercise negative cases: wrong signature, untrusted publisher, source/metadata mismatch, and post-signing package mutation.
4. Restore a functioning runner/log path; rerun focused package/provenance tests, stale-lock/determinism tests, workspace tests, lint/Clippy, contract checker, and required platform jobs against the resulting final SHA.
5. After any source or documentation change, record the new HEAD and repeat required checks. Update DoD/checkpoint only with actual outcomes.

### Gate
**NOT PASSED — E2 remains open.** This checkpoint is an evidence record, not release authorization. Do not merge PR #80, publish a release, or alter gate status manually.


## 2026-10-24 — Continued work: isolated E1/E2 GitHub test labs

### Repository changes made
- Expanded `.github/workflows/milestone-1-e1-lab.yml` to a non-fail-fast Ubuntu 24.04 + Windows Server 2025 matrix. Each job prints the exact source SHA and toolchain and runs focused stdlib tests, core unit tests, workspace tests, Clippy with warnings denied, and the repository contract checker.
  - Workflow change commit: `0f9a5bc2ad96288b8b3584df32013df954b29363`.
- Added `.github/workflows/milestone-1-e2-lab.yml` to run the provenance cryptographic unit tests, CLI packaging/lock/provenance integration tests, full workspace tests, and Clippy. It uses synthetic test fixtures and does not read or print protected release secrets.
  - Workflow commit: `c69cf07472c50944c30a1855c64791a9b69e0783`.
- Added `docs/verification/milestone-1-e2-test-lab.md`, describing local/hosted commands, positive and negative acceptance cases, exact-SHA evidence rules, and the separation between synthetic-key tests and protected release acceptance.
  - Documentation commit: `29ee8faf15913a8255a80239155c1acc3a11faf9`.
- The existing E1 lab guide is `docs/verification/milestone-1-e1-test-lab.md`.

### Live hosted evidence — still blocked before execution
- E1 matrix run on commit `bf48b274c42582ccf7f435c0245670f342885806`: [run 40105479624](https://github.com/panglimanetizen01/one/actions/runs/40105479624). Both Ubuntu and Windows jobs report failure with empty step arrays; log retrieval returns `BlobNotFound`. No test command ran.
- E2 lab on commit `c69cf07472c50944c30a1855c64791a9b69e0783`: [run 40105655927](https://github.com/panglimanetizen01/one/actions/runs/40105655927) and duplicate trigger [run 40105651315](https://github.com/panglimanetizen01/one/actions/runs/40105651315). Both report failure before executable steps; no compiler/test output or artifact is available.
- Standard focused/workspace workflows triggered on the same commits also fail before steps. The symptom persists across workflow types and both matrix operating systems, supporting an Actions runner/log-infrastructure classification rather than a source test diagnosis. It is not a pass.
- A Linux distribution run for documentation commit `29ee8faf15913a8255a80239155c1acc3a11faf9` was still queued at the last API read: [run 40105681068](https://github.com/panglimanetizen01/one/actions/runs/40105681068). Queued status is not evidence of acceptance.
- The latest E1/E2 lab definitions have been committed, but no successful test execution is available from this session. Local execution also remains unavailable here: this environment has no `cargo`/Rust compiler and cannot resolve `github.com`.

### Acceptance status (unchanged)
- E1: **7/10 implementation, 0/10 verified; PARTIAL / NOT VERIFIED**.
- E2: **7/8 implementation, 0/8 verified; PARTIAL / NOT VERIFIED**.
- No ledger weights or verified points were changed. No green/locked state was fabricated.
- PR #80 remains open/draft/unmerged; `main` is untouched; no release was published.

### Next actionable path
1. Use an authorized runner or GitHub administrator to restore/enable a working runner and downloadable logs for this repository; rerun the E1/E2 labs against the resulting exact candidate SHA.
2. Diagnose and repair any actual compilation, assertion, Clippy, or contract-check failure revealed by executed steps; repeat the complete matrix after each fix.
3. For E2, separately configure and approve protected environment `one-release`, provision secrets through GitHub's secure settings only, execute the release workflow, and independently verify the uploaded provenance using an independently authenticated publisher key.
4. Reconcile all current-SHA evidence in the ledger and Definition of Done only after every required acceptance criterion passes. Never convert runner failure or queued status into PASS.

### Gate
**NOT PASSED — 25% / Milestone 1 acceptance is not proven.** The repository now has explicit E1 and E2 labs, but the required exact-SHA executions and E2 protected-release verification remain outstanding.


## Prompt 2/4 — E2 packaging and signed release provenance (2026-10-24)

- Candidate source before this documentation update: `a75362fcb9dd7514c1253f661443252122053aa9`. The focused independent exact-SHA lab passed at [run 40107005038](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40107005038), including the package-signing/trusted-publisher test step and the focused regression suite. Cargo lockfile bootstrap passed at [run 40106909806](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40106909806). Cross-platform package/build and artifact-integrity checks also passed on this exact source SHA in [run 40106909764](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40106909764) and [run 40106909785](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40106909785).
- Current code includes deterministic package generation, lockfile bootstrap/locked Cargo behavior, stale module-lock rejection, Ed25519 V2 provenance binding artifact digest + artifact format + source-tree digest + Git revision + toolchain/build metadata, trust-store mapping, and bounded verification reads. Focused tests include tampered signed metadata rejection, altered artifact rejection, untrusted publisher rejection, changed source-tree rejection, and unsupported dependency declarations rejection.
- **Acceptance remains OPEN / NOT GREEN LOCKED.** These tests use synthetic fixture keys and do not establish that the canonical GitHub `one-release` environment is protected, that `ONE_RELEASE_SIGNING_SEED` and `ONE_TRUSTED_PUBLISHERS` are configured there, or that the trusted publisher key was independently authenticated. The release workflow exists on the PR branch but is not available on `main` for workflow dispatch; available GitHub actions do not provide a workflow-dispatch or environment-secret administration action. No secret was requested, exposed, or committed. No release was published.
- The ledger keeps E2 at **0/8 verified** and the overall release gate remains **NOT PASSED**. The documentation commit changes the canonical branch HEAD; all new exact-SHA acceptance evidence must use the resulting HEAD, not the preceding source SHA.

## 2026-10-25 — Prompt 3 E3/E4 continuation checkpoint

**Scope remains locked to Prompt 3 only: E3 platform conformance/Android AArch64 and E4 security/resource bounds. Do not begin Prompt 4 while the E3/E4 acceptance gates below remain open.**

- Main repository: `https://github.com/panglimanetizen01/one`.
- Candidate branch: `milestone/100-ecosystem-20261023`; PR #80 remains open and unmerged. The code-bearing candidate SHA was advanced to `5751aff3404f000545549908a00ea322cc96e277` after exact-SHA verification in the independent lab. This checkpoint/ledger/DoD/security-doc update advances the branch HEAD again; use the final branch HEAD after these documentation commits as the next lab pin and rerun the matrix. Do not reuse 5751 results as the final-SHA result.
- Independent lab: `https://github.com/panglimanetizen01/one-windows-verification`, verification branch `verification`. The lab resolves source SHA from `CANONICAL_COMMIT.txt`; workflows must verify `git rev-parse HEAD` against that pin.

### Evidence on code-bearing SHA 5751aff3404f000545549908a00ea322cc96e277

- Linux distro matrix passed Ubuntu 24.04 and Debian 12: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771630
- Windows Server 2025 cross-platform job passed full build/Clippy/tests/network/offline/bundle checks: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771649. The Ubuntu job of this same workflow was still in progress at this checkpoint during `Install Linux cross-compilers and Android NDK`; do not count the whole two-job workflow as PASS until its final conclusion is read.
- SEC-05 bounded subprocess Linux/Windows tests and static guard passed: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771660. The suite covers timeout, bounded output, quoting, Unicode arguments/working directory, immediate descendant spawn stress, and Windows Job Object containment from process creation.
- E4 adversarial Unix filesystem/SQLite/policy and Windows reparse-point regressions passed: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40213093522. The full Windows workspace suite also passed `orchestrator::tests::denied_request_never_reaches_adapter_invocation`.
- E4 subprocess static resource guard passed: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771671.
- Android AArch64 artifact build passed: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40213093523. Binary is ELF64 little-endian AArch64 PIE, interpreter `/system/bin/linker64`, SHA-256 `b3a52ce990c157332ab39f25264fbdab3de9e796c26f827c497551237855967c`. This is build/header evidence only.
- Android API 35 runtime did not pass. The hosted macOS ARM64 attempt failed emulator boot with `HV_UNSUPPORTED`: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771680. The Linux ARM64-host attempt failed because the Android emulator package was unavailable: https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771636. Neither produced runtime output/assertion evidence.
- Scoped `rustfmt --check crates/one-core/src/interop.rs` passed. Whole-repository `cargo fmt --check --all` found unrelated pre-existing formatting drift; it is a separately reported non-blocking diagnostic, not a reason to weaken the scoped gate.

### Remaining mandatory blockers

1. E3 is not accepted until the exact final candidate SHA executes the ELF on Android API 35 `arm64-v8a`, returns exit code 0, and emits the expected `ONE_NATIVE_BACKEND_OK` assertion/output. A successful build, ELF header, API 34 result, or emulator image installation is insufficient.
2. E4 is not accepted while explicit project-root descriptor anchoring, mount-boundary behavior, runtime-wide quotas, fallback-platform no-follow guarantees, and the full threat review remain incomplete. The capability-denial side-effect test and listed filesystem/SQLite/Windows regressions passing do not establish a complete sandbox.
3. After the final documentation commit, pin `CANONICAL_COMMIT.txt` to the resulting exact branch HEAD and rerun the required Linux distro, Windows, SEC-05, E4 adversarial/resource guard, Android build, and Android API 35 runtime attempts. Store final run IDs/logs/artifact metadata in the lab evidence index; do not alter the source docs again merely to add run IDs.
4. Keep E3 verified at 0/8, E4 verified at 0/6, verified total at 60/100, implementation subtotal at 91/100 provisional, and release gate `NOT PASSED`. Do not merge PR #80, merge PR #83, publish a release, or start Prompt 4 without explicit authorization and evidence closure.
## 2026-10-25 — Required follow-up after the first final-SHA matrix

- The matrix on `26d3d30be773cfc403daa002f7116c176ef31fc8` found two Windows output-flood regression failures: `stderr_flood_is_bounded` and `aggregate_output_limit_is_enforced`. The observed error was the test's 3-second timeout, not a failed output byte cap. Source commit `c339da876f6ceb18084e8551cc08d4edbabc0f27` raises only the timeout in the three output-flood tests to 15 seconds; assertions still require the output-limit error. Production limits are unchanged. Rerun on the final post-documentation SHA.
- The first software-acceleration fallback attempt failed because `one-api35-arm64` was not created. The independent lab workflow has now been fixed to create and verify the API 35 `arm64-v8a` AVD before hardware/software boot attempts. This is a lab workflow change only; it does not change the source SHA.
- The ledger and DoD now record this failure/fix. After this checkpoint commit, resolve the candidate branch HEAD, update `CANONICAL_COMMIT.txt` to that exact SHA, and rerun Linux distro, Windows cross-platform, SEC-05, E4 adversarial/resource guard, Android artifact build, and Android API 35 runtime fallback. Do not reuse run results from `5751aff3404f000545549908a00ea322cc96e277` or `26d3d30be773cfc403daa002f7116c176ef31fc8` as final evidence for the new HEAD.
- The gate remains NOT PASSED. Even if the test flake and Android emulator setup are resolved, E4 still requires explicit project-root descriptor anchoring, mount-boundary enforcement, runtime-wide quotas, fallback-platform guarantees, and a complete threat review. Do not merge or release.

## Prompt 3 continuation checkpoint — 2026-10-26

- Source branch at audit start: `milestone/100-ecosystem-20261023`; code-bearing SHA checked: `85ded41bea3d7e2f64f0758e1bfacb376f0cb2cd`. PR #80 remains open/draft/unmerged; `main` was not changed.
- E3 Linux/Windows matrix: PASS for the exact source SHA in [40217709918](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40217709918) and [40217709951](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40217709951). Android AArch64 ELF build/header is available in [40217710176](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40217710176), but runtime acceptance is blocked: [API 35 run 40221042296](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40221042296) fails during emulator boot with `HV_UNSUPPORTED`, and ELF execution is skipped. E3 remains 0/8 verified.
- E4 adversarial test suite passed the cases listed in [40217709863](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40217709863); Linux/Windows subprocess suite passed in [40217709933](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40217709933). E4 remains 0/6 verified: the current candidate uses `PROC_THREAD_ATTRIBUTE_JOB_LIST` during `CreateProcessW`, and Windows process-creation guard/test evidence passed on the parent SHA `a8e76ab6f3288c4391e059663e1815a361a54924` in [run 40251750642](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40251750642). However, project-root anchoring, mount-boundary enforcement, runtime-wide quotas, fallback guarantees, and complete threat review remain open.
- Independent lab evidence index: https://github.com/panglimanetizen01/one-windows-verification/blob/verification/evidence/PROMPT3-E3-E4-85ded41.md. Latest Android runner failure is documented there. No PR merged, no release published, no weights changed.
- Next action stays within Prompt 3: implement/test remaining E4 blockers and obtain a supported physical/API 35 `arm64-v8a` runtime. Do not label E3/E4 or global release gate PASS without direct evidence.


## Prompt 3 continuation checkpoint — 2026-10-26 final E3/E4 hardening

- Scope remains strictly E3 + E4. Source branch `milestone/100-ecosystem-20261023`; code-bearing source before this documentation commit: `b513fab420ebe09777cc86077b4e89f632e41a23`. This documentation commit advances HEAD again; the lab selector must be pinned to the resulting branch HEAD before the final matrix.
- Implemented changes in this continuation: `open_directory_relative_to_root` is Unix-gated to restore Windows compilation; Windows absolute paths are normalized before root-relative derivation; shared process-wide resource admission allows 8 active subprocess/SQLite operations total and 4 concurrent SQLite queries; interpreter aggregate output is capped at 16 MiB per execution/event dispatch; quota tests cover denial/rollback/recovery; filesystem tests now cover Linux `/proc` mount boundary and parent-directory/symlink swaps.
- Preliminary results on older candidate SHA `dce1db683a17419d581274c1254cf62b6134484e`: E4 adversarial run [40261107951](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261107951) passed before the latest race/mount tests; Windows complete one-core suite passed in [40261107886](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261107886), while its final job conclusion was still pending at last inspection; Linux distro run [40261107750](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261107750) failed strict Clippy on constant assertions, corrected in `68817d60646558a09bcbbdc08cd13ba9c0aeb220`. These are not final-SHA evidence.
- Cross-build/contract runs [40261107880](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261107880) and [40261107989](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261107989) passed on the older SHA; they are build-only. Android runtime attempts on that SHA include boot failures and/or runs still in progress; no final runtime PASS is claimed.
- After this commit, resolve the new source branch HEAD and update lab `CANONICAL_COMMIT.txt` to exactly that SHA. Rerun Ubuntu 24.04, Debian 12, Windows Server 2025, SEC-05, E4 adversarial/resource guard, artifact integrity, Android AArch64 ELF/header checks, and Android API 35 `arm64-v8a` runtime. Update only the independent lab evidence index with final run IDs/logs after the run results are available; avoid another source-doc commit merely to paste run IDs.
- Ledger weights and scores remain fixed: implementation `91/100` provisional, verified `64/100`; E3 `0/8` verified, E4 `0/6` verified; release gate **NOT PASSED**. No merge of PR #80/#83, no `main` changes, no release, and no work outside E3/E4.


### Strict Clippy follow-up — 2026-10-26

- The final-source Linux distro run [40261657177](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261657177) failed strict Clippy due to one unused local variable in the new Unix parent-swap race test. The binding was removed; the test sequence/assertions and production behavior are unchanged.
- This is a new source revision after the matrix was started. Resolve the new branch HEAD, update lab `CANONICAL_COMMIT.txt`, and rerun Linux 24.04, Debian 12, Windows, SEC-05, E4 adversarial/resource, artifact integrity, and Android API 35 runtime on that exact SHA. Previous results are not final-SHA evidence.


## Prompt 3 continuation — E3/E4 audit reconciliation, 2026-10-26

Scope remains locked to E3 + E4. PR #80 is still open/draft/unmerged, base `main`; no merge, release, or main-branch modification is authorized. At the beginning of this audit, PR #80 head SHA and lab `CANONICAL_COMMIT.txt` both matched `73bd504ac81a14737b321817b94c7bc2fbb17a15`.

Exact-source code-bearing SHA results recorded:
- Linux Ubuntu 24.04 + Debian 12: [run 40272900538](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900538), both jobs passed.
- Ubuntu + Windows Server 2025: [run 40272900560](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900560), both jobs passed.
- Linux/Windows artifact manifest and executable-format verification: [run 40272900570](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900570), passed; binary SHA-256 values and artifact digests are recorded in the completion ledger.
- SQLite/standard-library resource suite: [run 40272900585](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900585), 247 passed.
- Focused E4 adversarial filesystem/resource suite: [run 40272900540](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900540), both Unix and Windows jobs passed for their stated coverage. Resource guard [run 40272900536](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900536) passed.
- Android API 35 AArch64 cross-build/contract: [run 40272900641](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900641) passed build/contract only, not runtime.
- Android API 35 `arm64-v8a` runtime remains OPEN. The latest attempt at this checkpoint update is [run 40275435454](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40275435454); require guest API 35 + ABI `arm64-v8a`, execution of the actual ELF, exact output `ONE_NATIVE_BACKEND_OK`, exit code 0, and retained logs/checksums before credit.
- SEC-05 Linux rerun [40275249195](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40275249195) exposed a missing GNU x86_64 linker package in the harness; Windows passed. The lab now installs `binutils-x86-64-linux-gnu`, and [run 40275580929](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40275580929) is the exact-source rerun in progress.

The ledger and Definition of Done now record this evidence and explicitly retain E3 0/8 and E4 0/6 verified. This checkpoint update advances the source branch again. Resolve the resulting branch HEAD, set the lab pin to exactly that SHA, rerun the mandatory Linux/Windows/E4/SEC-05/artifact/Android matrix, and then update the independent evidence index with the resulting run IDs. Do not reuse the pre-documentation SHA as final-SHA evidence.

Residual limits remain: no complete OS sandbox claim; non-Linux Unix mount identity is best-effort `st_dev`; arbitrary heap/caller-retained output memory is not fully accounted; and the full threat review remains open. The global fixed-weight gate remains **NOT PASSED**. No weights or scores change.


## 2026-10-28 cross-repository audit reconciliation — current acceptance status

A cross-audit found that the completion ledger carried E3, E4, and E5 verified points from older source SHAs even though the Definition of Done explicitly prohibits carrying acceptance forward without exact-SHA revalidation. This contradicted the latest checkpoint notes that E3 runtime acceptance and the complete E4 threat review remained open. The historical run records are preserved, but they are no longer credited as current-candidate verification.

- Implementation subtotal: **92/100 provisional**.
- Verified completion corrected from **78/100 to 60/100**; this is an evidence-accounting correction, not a code regression or a change to fixed weights.
- E1 **0/10**, E2 **0/8**, E3 **0/8**, E4 **0/6**, E5 **0/4**, E6 **0/4** verified for the final candidate until their full acceptance criteria are satisfied.
- The Definition of Done now leaves E3/E4/E5 and current-candidate cross-platform/artifact evidence unchecked until fresh exact-SHA acceptance is recorded.
- This reconciliation itself advances the source branch. Read the resulting branch HEAD, update the independent verifier pin and PR #80 metadata, then rerun the mandatory exact-SHA matrix. No evidence from a prior source SHA may be promoted to the resulting HEAD.

**Gate: NOT PASSED.** No merge, release, or PASS GREEN LOCKED claim is authorized.


## 2026-10-28 exact-SHA matrix update — recorded candidate 938ad378c4f0fece2cb806151c03bbc8a86b5b2c

The current candidate's independent exact-SHA matrix completed the Android runtime requirement successfully. The native ARM64 Linux host run verified Android API level 35, guest ABI `arm64-v8a`, an AArch64 ELF, exit code 0, and output `ONE_NATIVE_BACKEND_OK` ([run 40628912894](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40628912894)). Cross-platform Linux/Windows verification and the documentation checker/tests passed ([run 40628913019](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40628913019)); candidate artifact integrity passed ([run 40628912883](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40628912883)); E4 adversarial, SEC-05, and resource-guard regression runs also passed. The community fallback remains an additional, non-required attempt.

- E3 receives 8/8 verified points and E5 receives 4/4 on the recorded candidate above; verified subtotal is now **72/100** and implementation subtotal remains **92/100 provisional**.
- E4 remains **0/6** because the full threat review and the explicit root/mount/quota/fallback guarantees remain open. E1 remains 0/10 due incomplete standard-library scope; E2 remains 0/8 because protected production release acceptance has not run; E6 remains 0/4.
- The ledger/DoD reconciliation itself advances the source SHA. The exact-SHA matrix must rerun on the resulting pin; the overall gate remains **NOT PASSED** until current-candidate evidence and the remaining acceptance criteria pass. No merge or release is authorized.
