# ONE Final Milestone Session Checkpoint

**Updated:** 2026-10-24
**Active target:** Milestone 100% — ecosystem and release quality
**Repository:** https://github.com/panglimanetizen01/one
**Branch:** `milestone/100-ecosystem-20261023`
**PR:** https://github.com/panglimanetizen01/one/pull/80 (open, draft, unmerged)
**Base:** `main` (PR only; no direct changes to main)

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
- Fixed `tools/verify_offline.ps1` false-green behavior by propagating nonzero native exit codes and skipping only the Android-native CLI test on Windows. Android AArch64 native execution passed on Ubuntu in the later exact-SHA matrix (run 40007463527); the Windows-specific CLI Android-target test was filtered out on Windows and is not counted as a Windows pass.

### Evidence

- Focused standard-library/database regressions: [run 40006470057](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470057) — SUCCESS.
- Linux distro matrix: [run 40006470246](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470246) — Ubuntu 24.04 and Debian 12 SUCCESS.
- Exact-SHA cross-platform matrix: [run 40006470017](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470017) — Ubuntu and Windows SUCCESS for build, contract artifacts, AST tests, Clippy, workspace tests, real-network tests and platform-appropriate offline verification. Test logs show 215 core tests passed on Ubuntu and 210 on Windows. One Android AArch64 execution test remains ignored; the Windows-only CLI test is explicitly filtered out.
- The earlier run [40005692552](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40005692552) is not credited as valid full verification: its offline script log exposed a failed Android-target test followed by a false PASS. The corrected script was verified on SHA `64985...`; the new Windows log contains no `test result: FAILED` marker and ends with `OFFLINE VERIFICATION: PASS`.
- Canonical repository runs [40006462899](https://github.com/panglimanetizen01/one/actions/runs/40006462899), [40006463227](https://github.com/panglimanetizen01/one/actions/runs/40006463227), and [40006463315](https://github.com/panglimanetizen01/one/actions/runs/40006463315) failed before steps with `runner_id=0` and empty step lists. No source test result is inferred from those failures; the reviewed exact-SHA mirror fallback supplied actual Linux/Windows evidence.

### Completion ledger and remaining work

- **Implementation subtotal:** 84/100 provisional points; E1=7/10, E2=5/8, E3=8/8, E4=3/6, E5=1/4, E6=0/4. Fixed weights unchanged.
- **Verified completion:** 68/100. E1, E2, E4, E5 and E6 remain unverified.
- Remaining mandatory work: external dependency resolution and authenticated package provenance; wider standard-library APIs; race-resistant filesystem confinement and broader quotas; compatibility and developer tooling/docs completion; Android-native execution if required by declared targets; and E6 forensic Definition-of-Done audit.
- **GATE: NOT PASSED — Milestone 100% remains in progress.**


## Final continuation evidence — checkpoint snapshot `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974`

- [Focused stdlib/database](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463554) — SUCCESS.
- [Linux distro matrix](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463563) — Ubuntu 24.04 and Debian 12 SUCCESS.
- [Ubuntu/Windows cross-platform matrix](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463527) — both jobs SUCCESS on exact source SHA `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974`. Both passed build, contract artifacts, AST tests, Clippy, workspace tests, real-network tests and platform-appropriate offline verification. No failed test summary was found in the logs; each platform printed `OFFLINE VERIFICATION: PASS`.
- Ubuntu workspace tests: 215 passed, 0 failed, 0 ignored. The Android AArch64 native execution test passed on Ubuntu. Windows workspace tests: 210 passed, 0 failed, 0 ignored; its Android-target CLI test is filtered out because the Windows runner is not an Android toolchain.
- The previous Windows false-green was corrected in `tools/verify_offline.ps1` by enabling native-command failure propagation and explicitly excluding only the Windows-inapplicable Android CLI test. The earlier run with a swallowed test failure is not credited.
- Canonical repository Actions still fail before steps with `runner_id=0`; the independent exact-SHA mirror supplied actual verification evidence.

**Current ledger:** 84/100 provisional implementation points, 68/100 verified.  
**Remaining:** external dependency resolution/provenance, wider standard library, race-resistant filesystem confinement and broader quotas, full compatibility/release and developer-tooling/documentation acceptance, and E6 forensic audit.  
**GATE: NOT PASSED — Milestone 100% remains in progress.**

This is a documentation-only checkpoint update; the verified source code snapshot is unchanged.
