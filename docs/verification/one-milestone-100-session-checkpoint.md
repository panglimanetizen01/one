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

- `crates/one-core/src/stdlib.rs`: deterministic public Rust helpers for text length/contains/trim/literal replace/literal split/integer conversion; list length/index access; deterministic text-keyed map construction/get/set; checked integer absolute value/division/min/max; and unit tests for Unicode, bounds, invalid types, overflow, division by zero, and stable map ordering.
- `crates/one-core/src/semantic.rs`: registers 14 built-ins with arity, return-type, and static argument-type rules (`E2032`).
- `crates/one-core/src/interpreter.rs`: dispatches source-level text/list/map/integer built-ins through the canonical IR interpreter and checks runtime arity before indexing arguments; invalid builtin operations return `R3022`. Integer arithmetic uses checked add/subtract/multiply/divide and reports `R3023` on overflow rather than panicking.
- `crates/one-core/tests/stdlib_builtins.rs`: adds end-to-end tests through parse → semantic analysis → IR → interpreter, including an opt-in mapping test, a checked-in example, and overflow/error cases.
- `crates/one-cli/tests/cli.rs`: verifies that the official CLI can execute the checked-in stdlib example.
- `examples/stdlib.one`: runnable example exercising the current source-level builtin subset.
- `docs/panduan-pengguna/fungsi-dan-event.md`: documents supported builtin syntax and explicit limitations, including opt-in mapping grammar.
- `docs/spec/one-v0.1-standard-library.md`: documents which helpers are source-level built-ins and which remain Rust-only helpers.
- Main implementation commits include `652d4dc6662b62538dde602ef215cfe05b340cd1`, `7f196f64b2b181ebb87ac6a2fb35642b3f6d5112`, `449caf6b33c28d10d59674e6bb4e850d6ad4be1c`, `c2f16baa1ab26184c74c7908faa1557419c1e277`, `66cf560fe30928318ea619a565422303c8806ef9`, `abd105a4515c3c1b96b8695f6c9061646a6dfca2`, `c56d841ed914f384975f9e224e1db014336a72c7`, `6656abed12c0b0d5dba983c2261c516ca98e5b55`, and `2b98153907eaa1e96f160bf2efc7b59aef06eaf0`.

## Verification status

- Canonical repository CI run [39940416046](https://github.com/panglimanetizen01/one/actions/runs/39940416046) targeted exact SHA `66cf560fe30928318ea619a565422303c8806ef9` but both OS jobs failed before any steps (`runner_id=0`, empty step lists). This is an infrastructure failure, not a source test result.
- Independent mirror run [39940968766](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39940968766) targeted SHA `b6362a15bdd59715726be424453bc64c23e24063`. Contract artifacts and strict Clippy passed on Ubuntu; the workspace test suite failed at `integer_arithmetic_overflow_returns_a_runtime_diagnostic` because the returned error did not match the expected `R3023`. Root cause identified by source inspection: the overflow fixture contained double-escaped newline sequences, so the parser received literal backslash-n characters instead of separate source lines. The fixture is corrected in commit `f6337de8e970b7d78d86e05a92af625af9e99e64`; exact-SHA re-verification is required.
- Windows for that run is still in progress at the time of this checkpoint. Later map builtin and CLI test changes are not covered by that run.
- Current branch code SHA has advanced beyond `b6362a15bdd59715726be424453bc64c23e24063`; it also includes source-level mapping get/update built-ins, a CLI integration test, and the corrected overflow fixtures. Exact-SHA build, strict Clippy, and workspace tests for the latest branch remain **NOT YET VERIFIED**.
- No points are added to the verified completion total for this work. The completion ledger has not been altered.

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
