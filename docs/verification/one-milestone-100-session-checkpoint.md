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

- `crates/one-core/src/stdlib.rs`: deterministic public Rust helpers for text length/contains/trim/literal replace/literal split/integer conversion; list length/index access; deterministic text-keyed map construction/get/set; checked integer absolute value/division/min/max; bounded deterministic JSON serialization for text, numbers, booleans, empty values, lists, and maps; Unicode case/prefix/suffix operations; checked list joining and integer summation. Unit tests cover Unicode, bounds, invalid types, overflow, division by zero, stable map ordering, JSON escaping, and rejection of excessive JSON nesting (test commit `d2f77bb34be33780b24685899be5c773dbde1b7a`).
- `crates/one-core/src/semantic.rs`: registers 21 built-ins with arity, return-type, and static argument-type rules (`E2043`), including bounded JSON serialization, Unicode text helpers, and checked collection operations.
- `crates/one-core/src/interpreter.rs`: dispatches source-level text/list/map/integer built-ins through the canonical IR interpreter and checks runtime arity before indexing arguments; invalid builtin operations return `R3022`. Integer arithmetic uses checked add/subtract/multiply/divide and reports `R3023` on overflow. Recursive user-function calls are bounded by `max_call_depth` (default 128) and return `R3024`.
- `crates/one-core/tests/stdlib_builtins.rs`: adds end-to-end tests through parse → semantic analysis → IR → interpreter, including opt-in mapping tests, JSON serialization, Unicode text and collection built-ins, a checked-in example, overflow, invalid conversion, invalid index, missing-map-key, and recursion-depth cases.
- `docs/spec/one-v0.1-runtime-model.md`: documents actual execution/call-depth limits and checked integer overflow behavior without claiming unimplemented memory/open-file quotas.
- `crates/one-cli/src/main.rs`: `one build`, `one test`, and `one package` detect a `Buat program` entry and use `human_compiler::compile_file` to process relative modules through canonical IR. Build writes a `.oneir`, test executes the project and supports valid/invalid human-core conformance fixtures, and package writes a separate deterministic `One human-core package v1` artifact. Ordinary v0.1 `.onepkg` format remains unchanged.
- `crates/one-cli/tests/cli.rs`: tests deterministic packaging of a modular human-core project, exercises build/test/package, and includes valid/invalid human-core conformance fixtures.
- `docs/spec/one-v0.1-package-build.md` and `docs/spec/one-v0.1-cli.md`: document the experimental human-core project workflow and explicitly leave dependency/lockfile support incomplete.
- `crates/one-cli/tests/cli.rs`: verifies that the official CLI can execute the checked-in stdlib example.
- `examples/stdlib.one`: runnable example exercising the default-parser text/list/integer builtin subset.
- `examples/human-vnext/pemetaan-kontak.one`: now exercises mapping get/update built-ins through the opt-in human-core CLI path.
- `crates/one-cli/tests/human_core.rs`: asserts the checked-in mapping example outputs the updated value.
- `docs/panduan-pengguna/fungsi-dan-event.md`: documents supported builtin syntax and explicit limitations, including opt-in mapping grammar.
- `docs/spec/one-v0.1-standard-library.md`, `docs/spec/one-v0.1-stdlib.md`, and `docs/panduan-pengguna/fungsi-dan-event.md`: document helper boundaries, source-level built-in signatures, JSON serialization, type/arity checks, and error behavior.
- Main implementation commits include `652d4dc6662b62538dde602ef215cfe05b340cd1`, `7f196f64b2b181ebb87ac6a2fb35642b3f6d5112`, `449caf6b33c28d10d59674e6bb4e850d6ad4be1c`, `c2f16baa1ab26184c74c7908faa1557419c1e277`, `66cf560fe30928318ea619a565422303c8806ef9`, `abd105a4515c3c1b96b8695f6c9061646a6dfca2`, `c56d841ed914f384975f9e224e1db014336a72c7`, `6656abed12c0b0d5dba983c2261c516ca98e5b55`, and `2b98153907eaa1e96f160bf2efc7b59aef06eaf0`.

## Verification status

- Canonical repository CI run [39940416046](https://github.com/panglimanetizen01/one/actions/runs/39940416046) targeted an earlier SHA and both OS jobs failed before steps (`runner_id=0`, empty step lists). This is infrastructure evidence, not a source pass.
- Independent mirror run [39941625828](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39941625828) **PASSED on Ubuntu and Windows** for exact source SHA `2cf1e567357da960b8b592afc67a29454533bbd7`. Contract artifacts, build, AST contract tests, strict Clippy, workspace tests, real-network tests, and platform-appropriate offline verification completed. This validates only that earlier source snapshot.
- Run [39942193741](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39942193741) passed Ubuntu build/AST contract tests/Clippy but failed workspace tests because a new `Ani` assertion was placed in the wrong test; corrected in `259f7fd87de43e1db4678b6e1b6de8a913ac6e38`.
- Run [39942565741](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39942565741) targeted `711e3e9af557a7d57df11daa27f6ad63cdd0c949`; Ubuntu build, AST contract tests, strict Clippy, workspace tests, real-network tests, and offline verification passed. Windows remains in progress at the last query. This SHA predates the human-core package extension.
- Human-core package code-bearing SHA `c617695a5e45e41e6c5b21e0c99da94cfb9a559b` passed Ubuntu build, AST contract tests, strict Clippy, workspace tests, real-network tests, and offline verification in run [39942950596](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39942950596); Windows build and Clippy passed but its full test matrix was still in progress at the last query. This does not verify later build/test workflow additions.
- Later commits `c50fe33d61e9b5d13971b9ea07a873b8dd95108f` and `cec0baf0e4f39eb78cf73fbab7985ce121570dee` add human-core `build` and `test` workflows and extend the modular project CLI test. Exact-SHA verification for this latest change set is pending.
- Current ledger totals: **72/100 provisional implementation points; 60/100 verified points**. Provisional rows are E1=5/10, E2=3/8, E3=1/8, E4=2/6, E5=1/4. Fixed weights are unchanged; no new verified points have been awarded.
- E2 remains partial: the experimental human-core package format resolves relative modules into canonical IR and is designed to be deterministic, but dependency declarations/lockfile, package registry/resolution, and the full provenance envelope remain unimplemented.
- E4 remains partial: checked integer arithmetic and recursion-depth bounds are implemented; comprehensive threat review, general memory/open-resource quotas, and broader failure recovery remain unimplemented.
- Latest branch full Linux/Windows verification is **NOT YET VERIFIED**. The final 100% gate remains **NOT PASSED**.
## Remaining required work

- Integrate the standard-library helpers into the official source-level ONE runtime with structured errors and conformance tests.
- Complete the remaining E1–E6 acceptance criteria without changing their weights.
- Reconcile package/build/dependency and module behavior with normative contracts.
- Complete exact-SHA Linux/Windows and any other contract-required platform verification.
- Perform the final forensic audit only after all implementation and verification gates are satisfied.

## Git state and safety

- Current feature branch: `milestone/100-ecosystem-20261023`.
- PR #80 is open and draft; not merged.
- `main` was not modified directly.
- `~/one-termux` was not accessed.
- Do not claim milestone 100%, do not change ledger weights, and do not merge without explicit user authorization.

## Gate

**NOT PASSED** — milestone 100% remains in progress. The new helper code is implemented but not yet verified or fully integrated into the ONE source-language runtime.
