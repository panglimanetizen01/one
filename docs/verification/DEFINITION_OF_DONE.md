# ONE v0.1 — Definition of Done and Release Gate

**Status:** In progress — not a release authorization  
**Candidate branch:** `milestone/100-ecosystem-20261023`  
**Candidate source SHA:** see the synchronized source pin and evidence index; this reusable checklist intentionally does not embed a self-referential commit SHA.  
**Fixed-weight ledger:** [one-completion-ledger.md](one-completion-ledger.md)  
**Independent evidence index:** [CANDIDATE_VERIFICATION.md](https://github.com/panglimanetizen01/one-windows-verification/blob/verification/CANDIDATE_VERIFICATION.md)  
**PR:** [#80](https://github.com/panglimanetizen01/one/pull/80)

This checklist is an acceptance control, not a claim that the release is complete. A checked item requires direct evidence tied to the exact source SHA. Do not adjust ledger weights or award credit because a workflow was started, a bundle exists, or a test on another SHA passed.

## Active exact-SHA reconciliation — 2026-10-28

The source candidate observed before this documentation reconciliation was `65d09e3f9ed8639ff0d22a948ada06c3def36c22`; the live PR #80 head is authoritative after these commits. The independent verifier pin observed at recovery was stale (`c5ba60b16e0356753ffbd10d448b47dce64f01bb`). Six latest canonical source workflow runs reported failure with empty/null job steps; logs were unavailable (`BlobNotFound`). This does not prove source tests failed or passed. E3/E4/E5 evidence below is historical until re-run against the current exact SHA.

Current checkbox count after reconciliation: **3 checked, 10 unchecked**. Final exact-SHA score remains **UNKNOWN / NOT RECONCILED**; gate **NOT PASSED**. Do not merge or release.

## 1. Immutable evidence rules

- [x] Candidate source SHA is recorded explicitly in the PR and independent verification selector; the ledger records the freeze rule without embedding a self-referential commit SHA.
- [x] The pre-reconciliation candidate `53220eeb4bae9138b3a46b469004f8eac4a11420` passed the exact-SHA Ubuntu/Windows build and test matrix ([run 40631564946](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40631564946)); this ledger-only source update advances the pin, so the live gate must confirm the same checks on the new SHA before final PASS.
- [x] Candidate Linux and Windows bundles passed checksum-manifest and source-SHA integrity verification on pre-reconciliation SHA `53220eeb4bae9138b3a46b469004f8eac4a11420` ([run 40631564901](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40631564901)); the live gate must revalidate artifact integrity on the new pin before final PASS.
- [ ] Final candidate SHA is frozen after all remaining source changes; all evidence is rerun against that exact SHA.
- [ ] All evidence links, ledger rows, PR body, workflow pins, and artifact metadata agree on the final SHA.
- [ ] Any failed workflow is classified from logs/job metadata; no infrastructure failure is described as a passing test.
- [ ] No hosted-CI gate is silently replaced by a local or independent run. Any approved alternate evidence must execute equivalent checks on the exact SHA and state the remaining hosted-CI limitation.

## 2. Fixed-weight ledger acceptance

Keep the existing weights unchanged. Mark a row verified only after its acceptance criteria are demonstrated by executable tests and/or review artifacts.

- [ ] **E1 — standard library (10 points):** complete the scope-defined public reusable API contracts; tests cover valid/invalid inputs, deterministic behavior, diagnostics, and cross-platform behavior. Existing focused UTC/JSON/text/list/map/integer tests are partial evidence, not automatic full-row credit.
- [ ] **E2 — packaging and provenance (8 points):** deterministic package/lock behavior and stale-lock rejection pass; v2 source-bound provenance signature verifies against an independently configured trusted-publisher store; source tree is rechecked; protected `one-release` release path is exercised without exposing signing seeds or trust secrets.
- [ ] **E3 — conformance and platforms (8 points; historical prior-SHA evidence only; current candidate NOT VERIFIED):** pre-reconciliation candidate `53220eeb4bae9138b3a46b469004f8eac4a11420` passed Linux/Windows, Linux distro, artifact-integrity, and actual Android API 35 arm64-v8a ELF runtime checks ([cross-platform](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40631564946), [Linux distro](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40631564950), [artifact integrity](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40631564901), [native ARM64 Android runtime](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40631564953)). The live gate must reconfirm all required runs on the new pin before final PASS.
- [ ] **E4 — security, resource bounds, recovery (6 points; historical prior-SHA evidence only; current candidate NOT VERIFIED):** Scoped acceptance recorded for candidate SHA `28d1384588662cc93f405dc9dd5f3012183d249f` based on [E4 adversarial filesystem/resource bounds 40669813282](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40669813282), [SEC-05 timeout/output 40669813337](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40669813337), [resource guard 40669813307](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40669813307), and [cross-platform Linux/Windows 40669813367](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40669813367). The threat review accepts the tested controls within the current Linux/Windows first-class and Android/Linux integration scope, while retaining explicit limitations: macOS/additional Unix are planned expansion; non-Linux Unix mount identity is best-effort; Windows filesystem-provider races are not universally disproven; named quotas do not cap all heap/caller-retained memory. This is not a complete-sandbox claim. This documentation update changes the source SHA, so E4 must be revalidated on the new exact pin before final gate acceptance.
- [ ] **E5 — docs and developer tooling (4 points; historical prior-SHA evidence only; current candidate NOT VERIFIED):** on pre-reconciliation SHA `53220eeb4bae9138b3a46b469004f8eac4a11420`, Ubuntu and Windows passed contract checking, user-doc consistency and regression tests, CLI create/check/run/build/test/package smoke, and offline verification ([cross-platform run 40631564946](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40631564946)). The current-pin gate must reconfirm these checks after the ledger update before final PASS.
- [ ] **E6 — final forensic audit (4 points):** E1–E5 acceptance is reconciled; required docs, specs, plans, completion ledger, checkpoints, CI evidence, PR metadata, and release provenance are mutually consistent; no mandatory gate remains open.

## 3. Mandatory release gate

The release gate is **NOT PASSED** while any required checklist item or fixed-weight ledger acceptance row remains unchecked, while the final SHA is not frozen, or while a required platform/provenance/security gate lacks direct evidence.

Before authorizing a release:

1. Run the complete required verification matrix against the final SHA.
2. Verify artifact checksums and signed provenance independently.
3. Verify the trusted-publisher configuration and protected release environment without disclosing secrets.
4. Review the final diff and security limitations; ensure no unrelated changes were included.
5. Reconcile the completion ledger and PR description to the same evidence.
6. Record the exact final SHA and the URLs of every passing required workflow.
7. Only then may the ledger and release report say **PASS GREEN LOCKED — FINAL RELEASE**.

## 4. Current known open items

- Canonical-repository Actions job-step details remain unavailable through the current connector permission scope. Empty/unavailable step data is not a source-test result and cannot be counted as PASS; independent lab workflows executed actual steps and recorded logs on the exact candidate SHA.
- Android API 35 runtime is now directly proven on candidate source SHA `0d0202a1405ae12aaaadf05c6916bacc70316126`: [independent ARM64-host runtime workflow #40477658242](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40477658242), job [122100141841](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40477658242/job/122100141841), ran the generated AArch64 ELF on API 35 / `arm64-v8a`, returned exit code `0`, and emitted exactly `ONE_NATIVE_BACKEND_OK`. Source-bound evidence artifact ID `12722651198`, SHA-256 `03c3a79a44ae6511d5550bb932398575ebfd686592b720e3153a3f0d6066f770`. Earlier `HV_UNSUPPORTED` and incompatible emulator attempts remain historical failures, not current runtime status. This runtime result does not alone close E1, E2, E6, or authorize release.
- The implementation now has adapter-level root anchoring on Unix/Windows, Linux mount-ID checks, a shared active-operation quota, and an aggregate interpreter-output cap. Exact-final-SHA proof is still required; other Unix mount checks use `st_dev`, non-Unix/non-Windows filesystem adapters fail closed, and arbitrary heap/caller-retained result memory is not fully accounted. The E3/E4 evidence below remains partial and does not close E1–E6 or authorize a release.
- This document does not authorize merging PR #80 or publishing a release.

## 5. Prompt 3 E3/E4 execution record — code-bearing SHA `5751aff3404f000545549908a00ea322cc96e277`

This is an evidence record for the code-bearing SHA, not acceptance of the full E3/E4 rows. The source/documentation branch advances after this update; the independent verification selector must be pinned to the resulting exact branch HEAD and the required matrix rerun.

- [Linux distro matrix](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771630) — Ubuntu 24.04 and Debian 12 passed build, Clippy, workspace tests, and real-network tests.
- [Cross-platform matrix](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771649) — Windows Server 2025 passed the full Windows build/test/Clippy/network/offline/bundle steps. The Ubuntu job was still running at checkpoint time; its final status must be rechecked.
- [SEC-05 subprocess verification](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771660) — Linux and Windows focused/full one-core tests and static guards passed; scoped `rustfmt --check crates/one-core/src/interop.rs` passed. Whole-repository `cargo fmt --check --all` reported unrelated pre-existing formatting drift and is retained as a separate non-blocking diagnostic.
- [E4 adversarial regressions](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40213093522) — Unix descriptor-relative/no-follow (3 tests), SQLite resource/query limits (11 tests), governance policy (10 tests), and Windows directory-junction/reparse escape (1 test) passed. The full Windows workspace suite passed `orchestrator::tests::denied_request_never_reaches_adapter_invocation`.
- [E4 subprocess resource guard](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771671) — passed.
- [Android AArch64 device artifact build](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40213093523) — passed on the exact source SHA. The artifact is ELF64 little-endian AArch64 PIE, interpreter `/system/bin/linker64`, with SHA-256 `b3a52ce990c157332ab39f25264fbdab3de9e796c26f827c497551237855967c`. This does not prove runtime execution.
- [Android API 35 macOS runtime attempt](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771680) — API 35 arm64-v8a image and pre-boot contract checks passed, but emulator boot failed with `HV_UNSUPPORTED`; the ELF execution step was skipped.
- [Android Linux ARM64-host attempt](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771636) — failed before runtime because a compatible Android emulator package was unavailable. Neither Android attempt is a PASS.

**Disposition:** E3 remains unverified because no API 35 ELF execution/output assertion was captured. E4 remains unverified because project-root descriptor anchoring, mount-boundary enforcement, runtime-wide quotas, fallback-platform guarantees, and the complete threat review remain open. The fixed-weight ledger remains **91/100 provisional implementation, 60/100 verified**; release gate remains **NOT PASSED**. PR #80 remains unmerged; no release is authorized.
## 6. Exact-SHA follow-up — Windows output-flood stress tests and Android AVD setup

The first final-SHA matrix attempt on `26d3d30be773cfc403daa002f7116c176ef31fc8` failed two Windows output-flood tests: `stderr_flood_is_bounded` and `aggregate_output_limit_is_enforced`. Logs showed the test's 3-second deadline fired before the process reached the byte cap. Commit `c339da876f6ceb18084e8551cc08d4edbabc0f27` changes only the test timeout for stdout/stderr/aggregate flood cases to 15 seconds; the assertion still requires an output-limit error and still fails if a timeout occurs. Production timeout and output limits are unchanged. A fresh exact-SHA Windows matrix is mandatory.

The first software-acceleration fallback attempt also revealed that the lab workflow had not created the `one-api35-arm64` AVD. The workflow now explicitly creates and verifies the API 35 `arm64-v8a` AVD before attempting hardware acceleration and the bounded `-accel off` fallback. The failed launch is a harness failure, not runtime acceptance; only actual ELF execution and `ONE_NATIVE_BACKEND_OK` on API 35 count.

This follow-up and the remaining checkpoint documentation create a new candidate branch HEAD. Pin the independent lab to that final exact SHA and rerun the matrix before acceptance. E3 and E4 remain unchecked, and the release gate remains NOT PASSED.

## 7. Prompt 3 hardening follow-up — 2026-10-26

The previous final-matrix attempt exposed a Windows path-prefix bug: absolute paths emitted by the test harness did not match the canonical extended-length root prefix, so valid in-root targets were rejected as invalid arguments. The adapter now normalizes Windows absolute paths before deriving a root-relative path, and a fresh exact-SHA Windows suite must confirm the read/write/oversized-file cases. The Unix descriptor helper is now compiled only on Unix.

A shared resource-admission budget now caps active subprocess/SQLite operations at 8 total and SQLite queries at 4 concurrent. Interpreter output is capped at 16 MiB per `execute`/event dispatch. New regression tests exercise quota rollback/recovery, Linux `/proc` mount-boundary denial, and a concurrent directory/symlink swap that must never read outside-root bytes. These changes do not establish a complete heap quota or operating-system sandbox.

The final documentation commit must be followed by an exact-SHA rerun of Linux 24.04, Debian 12, Windows Server 2025, SEC-05, E4 adversarial/resource tests, artifact integrity, and Android API 35 `arm64-v8a` runtime execution. Until the exact source SHA and every mandatory result are recorded in the independent evidence index, E3/E4 remain unchecked and the release gate remains **NOT PASSED**.


## 8. Strict Clippy follow-up — 2026-10-26

The Linux distro job [40261657177](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261657177) failed because the new parent-swap race test declared an unused local path. The unused binding is removed without changing the test's race sequence or assertions. Because this changes the candidate source SHA, the full required exact-SHA matrix must be rerun; no evidence from the preceding SHA is promoted to final acceptance.


## 9. E3/E4 exact-source reconciliation — 2026-10-26

The PR #80 head and independent lab pin agreed on code-bearing SHA `73bd504ac81a14737b321817b94c7bc2fbb17a15` at the start of this audit. The independent lab recorded these exact-source results:

- Linux distribution matrix [40272900538](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900538): Ubuntu 24.04 and Debian 12 passed.
- Cross-platform suite [40272900560](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900560): Ubuntu and Windows Server 2025 passed.
- Candidate artifact integrity [40272900570](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900570): source metadata and bundle manifests verified; Linux binary SHA-256 `04cdcb64ea05782118004a916a039b49f560cb3111e28636b46922bc78da5fc4`, Windows binary SHA-256 `1e2c5e6826261918b33bc7842a835c0694a4d0e7e1bdf6268bb95a3a3a3118d6`.
- Focused SQLite/stdlib resource suite [40272900585](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900585): 247 passed.
- E4 adversarial filesystem/resource jobs [40272900540](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900540) and subprocess resource guard [40272900536](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900536) completed successfully for their focused scopes.
- Android API 35 cross-build/contract [40272900641](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900641) is build-only. Android runtime remains OPEN; [40275435454](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40275435454) is the latest boot/ELF attempt at the time of this update and has not yet provided a verified runtime result.
- SEC-05 run [40275249195](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40275249195) failed on Linux because the harness lacked the GNU x86_64 linker required by its test fixture; Windows passed. The harness now installs `binutils-x86-64-linux-gnu`; rerun [40275580929](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40275580929) is in progress.

The filesystem implementation on this candidate includes Unix descriptor-relative/no-follow traversal, Linux mount identity checks, Windows opened-handle root validation and reparse-point rejection, fail-closed fallback behavior on unsupported platforms, and shared active-operation admission (8 global operations, 4 concurrent SQLite queries). These implementation facts and focused tests do **not** equal full E4 acceptance: cross-platform mount semantics and heap/caller-retained result memory remain residual limits, and the full threat review is still open.

**Current disposition:** keep all E3/E4 acceptance boxes unchecked and the release gate **NOT PASSED**. This DoD update advances the source branch SHA; the independent lab must be re-pinned to the resulting final HEAD and the required matrix rerun before any result is called final-SHA evidence. No score/weight change, merge, or release is authorized.


## 8. Android runtime evidence reconciliation — 2026-10-28

The independent verification evidence index records the completed API 35 / AArch64 runtime acceptance above. The source SHA for that execution is the then-current candidate `0d0202a1405ae12aaaadf05c6916bacc70316126`. This Definition of Done edit creates a new source commit; therefore the runtime result remains valid evidence for the prior source SHA, and the required final-SHA matrix must rerun against the new HEAD before release acceptance. No E1/E2/E6 points or checkboxes are awarded by this documentation correction.
