# ONE vNext Session Checkpoint

**Updated:** 2026-10-24  
**Active target:** Milestone 100% — ecosystem and release quality  
**Repository:** [panglimanetizen01/one](https://github.com/panglimanetizen01/one)  
**Active PR:** [#80](https://github.com/panglimanetizen01/one/pull/80)  
**Working branch:** `milestone/100-ecosystem-20261023`  
**Base branch:** `main` — unchanged; no merge performed.

## Completion ledger (fixed weights; no reweighting)

Canonical ledger: [one-completion-ledger.md](one-completion-ledger.md).

- **Implementation subtotal:** 84/100 provisional points.
- **Verified completion:** 60/100 points.
- **E1–E6 verified points:** 0/40. E1, E2, E3, E4, E5, and E6 are not fully verified under the 100% acceptance criteria; the 24-point implementation/verification gap is not verified completion.
- **Milestone 100% gate:** **NOT PASSED**.
- The previously earned 60% milestone remains historical and scoped to its exact source SHA and evidence. It does not make the current 100% candidate green.

## Current Git and CI evidence

- PR #80 is **open, draft, and unmerged**; no merge is authorized by this checkpoint.
- Historical code-bearing source SHA for the previous narrow stdlib slice: `5da1f4dfcfba7bc64186eaa6a4ab018e6703c8a6`. The latest exact-SHA verification and continuation changes are recorded in the section appended below.
- Canonical workflow run [39994484534](https://github.com/panglimanetizen01/one/actions/runs/39994484534), “One CI”, completed with **failure**. The two reported jobs (`test (windows-2025)` and `test (ubuntu-latest)`) failed; job-step details/logs were not returned by the available connector. Root cause is **UNKNOWN** from available evidence; do not infer a source failure or a pass.
- The associated Linux Distribution Conformance run [39994484625](https://github.com/panglimanetizen01/one/actions/runs/39994484625) and Standard Library Focused Verification run [39994484574](https://github.com/panglimanetizen01/one/actions/runs/39994484574) were **cancelled**. Subsequent runs on the integer-remainder candidate also report failure: [One CI 39995112864](https://github.com/panglimanetizen01/one/actions/runs/39995112864), [Linux distro 39995112888](https://github.com/panglimanetizen01/one/actions/runs/39995112888), and [focused stdlib 39995112905](https://github.com/panglimanetizen01/one/actions/runs/39995112905). The available job-step endpoint returned empty step lists for these jobs, so the underlying failure cause is **UNKNOWN**; this does not establish that the source tests failed or passed. The latest canonical runs on this branch have repeatedly failed before steps start: jobs report `runner_id: 0` and `steps: []`; retry attempt 2 also failed identically. This is an Actions runner provisioning failure, not evidence that source tests failed. The independent mirror fallback runs the required source checks.
- Older independent-mirror PASS results only apply to their explicitly recorded older source SHAs. They do not verify the current PR head.
- Exact-SHA independent verification is established for code SHA `07780078c2563f3622d036f483735b262eb298d6`: focused stdlib, Ubuntu/Debian distro, and Ubuntu/Windows cross-platform runs passed. This evidence applies to that SHA only; after this checkpoint-only commit, the new branch tip must be re-verified before claiming it is the final tested SHA.

## Implemented scope and remaining acceptance

The current milestone branch contains ecosystem work including standard-library helpers and UTC formatting, JSON limits, HTTP framing/size bounds, filesystem adapter checks, package/build tooling and SHA-256 module lock v2. Source presence alone is not acceptance evidence.

Remaining mandatory requirements include, at minimum:

- Complete and tested standard-library APIs, including the still-missing/incomplete time/date, concurrency, logging/configuration and persistence integration contracts.
- External dependency declarations and registry resolution; authenticated package identity/provenance; target-specific artifact/release workflow.
- Full current-SHA conformance and compatibility matrix on required platforms, with no substituted historical evidence.
- Race-resistant filesystem confinement, general resource quotas, and complete threat/recovery review.
- Complete, tested documentation/examples/developer workflow and compatibility review.
- Final forensic Definition-of-Done audit and release evidence on the exact release SHA.

## Actions taken in this session

- Read `AGENTS.md`, `docs/ONE_CONSTITUTION.md`, `docs/ARCHITECTURE.md`, `docs/REQUIREMENTS.md`, `docs/COMPATIBILITY.md`, `docs/INTEROPERABILITY.md`, `docs/SECURITY.md`, ADR-0017, the weighted completion ledger, the prior checkpoint, the stdlib implementation and relevant CI workflow definitions.
- Confirmed PR #80 is the active open/draft/unmerged milestone-100 branch; no merge or main-branch change was performed.
- Implemented `one_core::stdlib::number_modulo`: checked signed-integer remainder rejects wrong value types, divisor zero, and the `i64::MIN % -1` overflow case.
- Integrated the source-level builtin `angka sisa bagi` through runtime dispatch and semantic arity/type/result handling.
- Added unit regression coverage and a canonical-pipeline integration test for positive and negative remainder, wrong argument type, and division-by-zero handling.
- Updated `docs/spec/one-v0.1-stdlib.md` with the exact operation/error contract.
- Relevant commits:
  - `17a27f667d7178298b29d58ae1e308be9b9d81f3` — checked stdlib helper and unit tests.
  - `961e3354f6fd655640ff60f82f919a2a0f08247f` — runtime builtin integration.
  - `7f1eab3feaa23cadaaeffc9eb746f0f96b46f6a7` — semantic type/arity integration.
  - `f93be7ea2e98a577abb3b764e7f0b72ab4d0fd08` — end-to-end regression test (follow-up test-fixture cleanup).
  - `0f826510595472777cceab887cf2e36bb77d015b` — standard-library contract documentation.
  - `cac099caa28bd9eeed3c72ded101a70ccec39b5a` — checkpoint update recording implementation and verification boundaries.
- Verification evidence for source SHA `809a26d12a411b492f0888610dc9ebabf1a6992b` from the independent mirror:
  - Focused stdlib run [39995613378](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39995613378): **PASS** — exact-SHA checkout succeeded; UTC unit/boundary test, UTC pipeline test, prefix/suffix pipeline test, checked integer remainder pipeline test, and strict one-core Clippy all passed.
  - Linux distro run [39995613439](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39995613439): **SUCCESS** — Ubuntu 24.04 and Debian 12 both passed contract checks, build, Clippy, native workspace tests and real-network tests.
  - Full cross-platform run [39995613393](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39995613393): **IN PROGRESS** at the last inspection. Ubuntu had passed build, AST contract tests, Clippy and workspace tests; real-network runtime tests were still running. Windows was still provisioning the cross-compiler. No full cross-platform PASS is claimed yet.
  - The first focused run [39995473148](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39995473148) correctly caught the invalid arithmetic-in-argument fixture (E1014). The fixture was fixed in `809a26d12a411b492f0888610dc9ebabf1a6992b` and the focused workflow then passed.
  - Local clone/test execution is unavailable because this environment cannot resolve `github.com`; verification evidence above comes from the independent GitHub Actions mirror.
- No verified ledger points are added for this narrow stdlib slice: E1 acceptance remains partial and E1–E6 verified points remain 0/40. The 100% gate remains NOT PASSED.

## Next execution sequence

1. Continue implementation on the existing PR branch; after any code-changing slice, pin the independent verification mirror to that exact source SHA and run focused, Linux-distro, and Ubuntu/Windows cross-platform workflows.
2. Prioritize E2 external dependency resolution and authenticated package provenance; do not treat the current unsigned module lock as publisher authentication.
3. Continue E1 standard-library breadth and E4 security work, especially race-resistant filesystem confinement and broader resource quotas, with focused regression tests.
4. Complete E5 documentation/tooling review and preserve explicit limitations; the Android AArch64 ELF build-and-execute test remains ignored/skipped and is not a pass.
5. Keep the source repository's runner-provisioning failure visible, use the reviewed exact-SHA mirror fallback, and perform E6 forensic audit only after all mandatory criteria are satisfied.

## Gate

**NOT PASSED — Milestone 100%.** PR #80 remains draft/unmerged. Canonical repository Actions still fail before executing steps (`runner_id=0`); the independent exact-SHA mirror matrix passed on source SHA `64985b51599c1531b6b8e07aa0aa9d16551e6e38`. Mandatory E1, E2, E4, E5, and E6 criteria remain incomplete.


## 2026-10-24 continuation checkpoint — exact source SHA verified

**Code-bearing source SHA:** `cec949631f3c91a5419957b1a8820904daa6bcd7`  
**Exact SHA selected and verified:** `64985b51599c1531b6b8e07aa0aa9d16551e6e38` (the selected commit adds verification-contract documentation after the code-bearing commit above).  
**Mirror selector commit:** `f0510674e46bf96a1a7356441c896d1e67697f4a`, file `CANONICAL_COMMIT.txt` points to the exact source SHA.  
**PR / Git state:** [PR #80](https://github.com/panglimanetizen01/one/pull/80) remains open and draft; base `main` was not changed and no merge occurred.

### Changes made

- SQLite file-backed queries now open read-only, enable `query_only`, and do not create a missing database file. Query resource budgets: 1 MiB SQL, 1 MiB per parameter, 16 MiB aggregate parameters, 256 parameters, 100,000 rows, and 16 MiB output.
- Added strict UTC timestamp parsing builtin `unix dari waktu utc` with Gregorian validation and end-to-end parser → semantic → IR → interpreter tests; documented the round-trip with `waktu utc dari unix`.
- Corrected `tools/verify_offline.ps1` to propagate native command failures and explicitly skip only the Android-native CLI test on Windows.
- Updated standard-library contracts and user documentation.

### Verification evidence at exact source SHA `64985b51599c1531b6b8e07aa0aa9d16551e6e38`

- Focused stdlib/database: [run 40006470057](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470057) — **SUCCESS**.
- Linux distro: [run 40006470246](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470246) — **SUCCESS**, Ubuntu 24.04 and Debian 12.
- Cross-platform: [run 40006470017](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470017) — **SUCCESS**, Ubuntu and Windows. Build, contract artifacts, AST tests, Clippy, workspace tests, real-network tests and platform-appropriate offline verification passed.
- Windows offline verifier log confirms `OFFLINE VERIFICATION: PASS`, contains no `test result: FAILED`, and the Android-target CLI test is filtered out rather than falsely counted as passed. The separate Android AArch64 execution test remains ignored; no Android-native runtime pass is claimed.
- Canonical repository Actions for source SHA `64985...` still fail with `runner_id=0` and empty steps; those runs did not execute source checks. The independent mirror is the documented fallback, not a claim that canonical Actions passed.
- Earlier run [40005692552](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40005692552) is **not credited**: its Windows offline log exposed a failed Android test swallowed by PowerShell. The verifier fix above was applied and the exact-SHA matrix rerun successfully.

### Completion and gate

- **Implementation subtotal:** 84/100 provisional points (E1=7/10, E2=5/8, E3=8/8, E4=3/6, E5=1/4, E6=0/4; fixed weights unchanged).
- **Verified completion:** 60/100. E1, E2, E3, E4, E5 and E6 have 0 verified points under the current 100% gate.
- **Next:** implement external dependency resolution/provenance, broaden stdlib, harden filesystem race/resource controls, finish developer tooling/docs, then perform E6 forensic audit.
- **GATE:** **NOT PASSED — Milestone 100% remains in progress.**

This checkpoint update is documentation-only; the exact source SHA and evidence above remain the code-verification reference.


## Latest exact-SHA verification — `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974`

The branch snapshot `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974` (after documentation-only ledger/checkpoint commits) was verified through the independent exact-SHA mirror:

- Focused standard-library/database: [40007463554](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463554) — SUCCESS.
- Linux distro: [40007463563](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463563) — Ubuntu 24.04 and Debian 12 SUCCESS.
- Cross-platform: [40007463527](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463527) — Ubuntu and Windows SUCCESS. Exact SHA is present in both job logs. Both platform jobs passed build, AST contract tests, Clippy, workspace tests, real-network tests and platform-appropriate offline verification. No `test result: FAILED` markers appear; each platform logs `OFFLINE VERIFICATION: PASS`.
- Main workspace suite summaries: Ubuntu 215 passed / 0 failed; Windows 210 passed / 0 failed. The separate Android AArch64 ELF build-and-execute test remains ignored on both platforms. The Android-target CLI test on Ubuntu only verifies artifact creation; it is not an Android runtime test. The Windows-specific CLI Android-target test was filtered out because Windows is not an Android toolchain.
- Canonical source repository workflows for this SHA failed before steps with `runner_id=0` and empty step lists; those are infrastructure failures, not source test results.

**Current totals:** implementation 84/100 provisional; verified 60/100; fixed ledger weights unchanged.  
**GATE: NOT PASSED — Milestone 100% remains in progress.** External dependency resolution/provenance, broader stdlib, race-resistant filesystem/resource security, release/tooling/documentation completion and the E6 final audit remain outstanding.

This checkpoint addition is documentation-only; the tested source snapshot is unchanged.


## Forensic scoring correction — Android ELF execution remains unverified

The prior checkpoint credited E3's 8/8 verified points based on Linux/Windows CI and an Android-target CLI build test. Inspection of `crates/one-core/tests/native_backend_android_aarch64.rs` shows the separate `build_native_menghasilkan_dan_menjalankan_elf_android_aarch64` test is explicitly `#[ignore]`; the CLI test `android_target_explicit_mempertahankan_build_native` only verifies that a build artifact exists. A successful artifact build is not a runtime execution pass.

Therefore E3 remains **8/8 implemented, 0/8 verified** until the declared Android AArch64 ELF build-and-execute acceptance is satisfied or the repository contract explicitly excludes it. This is a correction to the verified score, not a weight change: implementation remains **84/100**, verified completion is **60/100**, E1–E6 verified points are **0/40**, and the 100% gate remains **NOT PASSED**.

### Latest exact-SHA matrix

The latest tested source snapshot is `ebf254d1194a903a3b582f3a18abfc402f2003d2`:

- Focused stdlib/database [40009389100](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389100) — SUCCESS.
- Linux distro [40009389164](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389164) — Ubuntu 24.04 and Debian 12 SUCCESS.
- Cross-platform [40009389190](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389190) — Ubuntu and Windows SUCCESS. Exact SHA verified in both logs; build, Clippy, workspace tests, real-network tests and offline verification passed; no `test result: FAILED` marker; one `OFFLINE VERIFICATION: PASS` per platform.
- Main workspace summaries: Ubuntu 215 passed / 0 failed; Windows 210 passed / 0 failed. The separate Android ELF build-and-execute test remains ignored on both platforms and is not counted as passed. Canonical repository Actions fail before steps with `runner_id=0`.

This correction is documentation-only and does not change the tested source code snapshot.

## 2026-10-24 milestone-100 continuation pointer

The active continuation and latest E3/E4 evidence boundary are recorded in docs/verification/one-milestone-100-session-checkpoint.md and docs/verification/one-completion-ledger.md. Do not infer milestone-100 completion from the vNext foundation/core rows above.

- Code-bearing source SHA for the latest implementation slice: b5c3827840a691868a69428ae4032614407f1062.
- The Android AArch64 acceptance test now validates the produced ELF header and the canonical/mirror workflows include an Android arm64 emulator job to run the ignored build-and-execute test explicitly. Actual execution on this source SHA remains NOT YET VERIFIED.
- Linux/Android filesystem adapters now use descriptor-relative O_NOFOLLOW opens for parent components and final files. This does not yet provide a root-confined sandbox or equivalent race-resistant behavior on Windows/macOS.
- A CLI regression test now ensures unsupported external dependency declarations fail closed. Registry resolution, committed deterministic Cargo lock state, signed publisher identity, and authenticated provenance remain incomplete.
- Current milestone scoring is 85/100 provisional implementation and 60/100 verified; weights remain unchanged. Canonical Actions on the preceding source SHA failed before steps with empty job step lists; those runs are infrastructure failures, not source-test results.
- Gate: NOT PASSED. The mirror selector must be pinned to the latest checkpoint SHA and its exact-SHA jobs inspected before any verification points are awarded.

