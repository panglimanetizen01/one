# ONE Final Milestone Session Checkpoint

**Updated:** 2026-10-23
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
- Current ledger totals: **75/100 provisional implementation points; 60/100 verified points**. Provisional rows are E1=6/10, E2=5/8, E3=1/8, E4=2/6, E5=1/4. Fixed weights are unchanged; no new verified points have been awarded.
- E1 remains partial: source-level text/list/map/integer/JSON built-ins remain integrated. Filesystem adapters now cap reads and writes at 16 MiB and reject parent-directory components and existing symlink path components. HTTP responses are bounded to a 16 MiB body plus 64 KiB header allowance; header count is capped at 256, and ambiguous framing is rejected. Time/date, concurrency, logging/configuration, and full persistence integration remain incomplete.
- E2 remains partial: human-core build/test/package, stale-lock verification, and SHA-256 module lock v2 are implemented. External dependency declarations/registry resolution, signed package identity, authenticated provenance, target-specific native artifacts, and full provenance remain incomplete.
- E3 remains partial: valid/invalid human-core conformance fixtures are supported, but the complete compatibility matrix must pass on the latest exact source SHA.
- E4 remains partial: checked integer arithmetic, recursion-depth bounds, bounded JSON/HTTP/filesystem operations, strict HTTP framing, and static filesystem traversal/symlink rejection are implemented with regression tests. Race-resistant filesystem resolution, general memory/open-resource quotas, comprehensive threat review, and broader failure recovery remain incomplete.
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
- The latest verification run is pending, and E1–E6 are still only partially implemented. Completion remains **75/100 provisional, 60/100 verified** under the unchanged ledger. Final 100% is not established.

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

**NOT PASSED** — milestone 100% remains in progress. Text/list/map/integer/JSON helper operations are wired to the interpreter's built-in function dispatch, and an end-to-end human-core built-in test passes on Ubuntu. Windows verification for source SHA `d9d696fa87683027b78fdcbbec504696904055aa` remains in progress. The required wider standard-library, dependency, security, documentation, and final audit work remains incomplete.
