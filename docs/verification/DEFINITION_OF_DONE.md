# ONE v0.1 — Definition of Done and Release Gate

## Current exact-SHA state — 2026-11-05

- Canonical source: `panglimanetizen01/one`; branch `milestone/100-ecosystem-20261023`; PR #80 open/draft/unmerged; base `main` at `d35db95153f6ddd79885494c72f5aee16c899751`.
- Current source HEAD: `85b035a9f0ec8512a13bb66c987e2f8ec024d8fd`; parent: `fecf6e0883124e47aebdda13294714b5d9bc15d5`. The independent verifier `verification` branch currently pins `1248a39199322fd4007cbd043bb58864096842c8`; this mismatch blocks current-HEAD acceptance.
- Canonical source workflows on `85b035a9...`: One CI [42019582401](https://github.com/panglimanetizen01/one/actions/runs/42019582401), E1 [42019582386](https://github.com/panglimanetizen01/one/actions/runs/42019582386), E2 [42019582378](https://github.com/panglimanetizen01/one/actions/runs/42019582378), Linux distro [42019582389](https://github.com/panglimanetizen01/one/actions/runs/42019582389), Repository Health [42019582606](https://github.com/panglimanetizen01/one/actions/runs/42019582606), and focused stdlib [42019582717](https://github.com/panglimanetizen01/one/actions/runs/42019582717). All completed FAILURE with returned job `steps: null`; representative log downloads returned `BlobNotFound`; artifact lists were empty. Classify as pre-step execution/evidence failure, root cause UNKNOWN—not source-test failure and not PASS. Do not repeat identical retries without a changed condition or transient diagnosis.
- Last observable verifier matrix is pinned to `1248a391...`, not current source. It found actual Linux fixture failures in `crates/one-cli/tests/project.rs`; the later source commits added the missing temporary-directory creation. It also found Windows CLI/package regressions involving reparse-point/canonical-path rejection in ordinary test temp directories. These failures are real for the tested `1248a391...` source and remain historical until the changed current source is re-run. Android API 35 AArch64 runtime run [41262527363](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41262527363) passed on `1248a391...` only; not current-HEAD evidence.
- Verifier ledger gate [41262533958](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41262533958) returned NOT PASSED: evidence index did not name exact pin, 10 DoD items unchecked, E1–E6 all 0 verified, several matrix jobs incomplete/failed, and no protected production provenance run.
- DoD checkbox count on the current source file is **3 checked / 10 unchecked / 13 total**. Last measured ledger subtotal remains **92/100 implementation provisional / 60/100 verified on an older source SHA**; it is not a current-SHA acceptance score. Final score **UNKNOWN / NOT RECONCILED**. E1–E6 remain 0 verified; fixed weights unchanged.
- PR #84 remains open/draft and is two commits ahead/three behind current `main`; its only substantive README delta is already represented in PR #80's branch. PR #85 remains open/draft and is three commits ahead/three behind `main`; its CI/release-check delta overlaps checks already present in PR #80. Neither PR is merged; do not merge or close automatically. PR #83 remains an unaccepted SEC-05 experiment; #78 is scope-limited historical milestone-60 evidence; #77 is proposal/audit only; #76 remains a separate HTTP IPv6 fix requiring fresh base/diff review.
- No merge, release, protected E2 dispatch, secret/trust-store mutation, or direct `main` edit is authorized. Pin synchronization and all further evidence must target the final resulting source SHA.

---


## Historical forensic audit — parent SHA fecf6e0 (superseded by current exact-SHA state above)

**Audited parent candidate:** `fecf6e0883124e47aebdda13294714b5d9bc15d5` on `milestone/100-ecosystem-20261023`. PR [#80](https://github.com/panglimanetizen01/one/pull/80) remains open, draft, and unmerged; base `main` is `d35db95153f6ddd79885494c72f5aee16c899751`. This remediation commit advances the branch HEAD. The run results below are evidence for the parent SHA only; re-read the live HEAD and rerun required checks before assigning current-SHA acceptance.

### Evidence inspected

- [One CI 41262698742](https://github.com/panglimanetizen01/one/actions/runs/41262698742): Ubuntu and Windows reached runner setup, checkout, toolchain and documentation checks. Both stopped at the repository-health scanner regression test because one assertion expected scanner version `0.1.3`, while the implementation emits `0.2.0`. Build and later tests were skipped.
- [Repository Health 41262698698](https://github.com/panglimanetizen01/one/actions/runs/41262698698): same stale scanner-version assertion failed on Ubuntu and Windows; the self-scan and evidence collection did not run.
- [Linux Distribution Conformance 41262698730](https://github.com/panglimanetizen01/one/actions/runs/41262698730): build and Clippy passed on Ubuntu and Debian 12. Workspace tests exposed two project-test fixtures that attempted to create a symlink/write a source file beneath a temporary directory that had not been created.
- [E1 lab 41262698708](https://github.com/panglimanetizen01/one/actions/runs/41262698708) and [E2 lab 41262698676](https://github.com/panglimanetizen01/one/actions/runs/41262698676): their focused standard-library and package/provenance tests passed, but full workspace tests also invoked Android/Windows cross-target CLI tests without installing those cross-compilers in these labs. Their summary step additionally used unescaped Markdown backticks in shell double-quoted echo strings, causing command substitution and spurious shell/Cargo errors.
- [Focused standard-library run 41262698707](https://github.com/panglimanetizen01/one/actions/runs/41262698707) completed successfully.
- The Android AArch64 job in One CI failed before the test script ran: the emulator never registered as `emulator-5554`, and the runner timed out waiting for boot. This is an emulator/harness failure, not proof of a ONE runtime defect or a PASS.
- Debian's rustup bootstrap emitted a `$HOME`/UID-home mismatch warning (`/github/home` versus `/root`), but Rust installation and the subsequent build/Clippy steps completed. Do not misclassify that warning as the cause of the workspace test failures.

### Remediation in this commit

- Align the scanner regression expectation with the emitted `0.2.0` scanner version.
- Create the temporary project directory in the two failing project-test fixtures before writing files or creating symlinks.
- Exclude cross-target CLI tests from E1/E2 generic workspace labs; those labs do not provision the required target compilers. Dedicated native-target verification remains a separate gate.
- Replace fragile shell backtick substitutions in E1/E2 test-lab summaries with safe `printf` output.

### Acceptance boundary

The previous ledger measurement (92/100 provisional implementation, 60/100 verified; 3/13 DoD boxes checked) belongs to an older exact source SHA and is not recalculated by these fixes. No new E1–E6 points are awarded here. Current final score remains **UNKNOWN / NOT RECONCILED** and the milestone-100 gate remains **NOT PASSED** until the new exact HEAD completes the required matrix, Android runtime evidence is resolved, and the independent ledger gate is rerun. No merge or release is authorized.

---


## Superseded exact-SHA recovery — 2026-10-31

This current-state block supersedes prior active summaries. The parent candidate before this documentation reconciliation was `669fdd2e069181dd4db9268804db7e77dd3c6f78`. This commit advances the source SHA; the live PR #80 head and independent verifier pin are the authoritative exact current SHA, avoiding a self-referential SHA inside this file.

- **Source/PR:** `panglimanetizen01/one`, branch `milestone/100-ecosystem-20261023`, PR #80 open/draft/unmerged; base `main` `d35db95153f6ddd79885494c72f5aee16c899751`.
- **Implemented delta:** source ancestry contains immutable action pins (commit `e58a68987fbdf0c6cb0498d8a5ed52fb5e73e439`) and the Check Run collector work (commits `b418b31ecd779c82972819604182a5585b4d8236`, `fecf9cfb88aa44cd599abde010362ea803ffc32b`, `297b169cb4cbb753b6a041f20a3f7eb22d5562d0`). The collector fetches exact-SHA Check Runs and up to 100 annotations per Check Run, redacts selected high-signal secret patterns, rejects missing/mismatched Check Run SHA values, and detects explicit billing/spending-limit denial. The scanner classifies this as an informational/high-confidence infrastructure/account blocker, not a source failure. The workflow grants read-only `checks: read`.
- **Latest code/test correction:** commit `c5293b2aab07b54f44deaf53ad3f0e23da48cf4a` aligns the scanner version/rule expectations, removes an invalid escape, and inlines the base-rooted Check Run annotation URL so the scanner's narrow trusted GitHub API client exception applies. Commit `ea515c321f337125ba2d29da0756e8c2f9a50066` builds the synthetic billing token fixture at runtime. Commit `9d2bb9152d0fb4d9202ffc9bd0c410ddc6bbd559` restores the scanner's concrete fake-token redaction fixture using a runtime-generated value. Commit `669fdd2e069181dd4db9268804db7e77dd3c6f78` updates the stale expected subprocess rule identifier to `subprocess-shell-true-ast`. No ONE grammar, semantic, runtime, or CLI source changed.
- **Current exact-SHA scanner verification:** independent verifier run [41219231598](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41219231598) checked out exact source `9f75a73a2e804d7053e9dffb0dfcab9a62aa1984` on Ubuntu and Windows. Both jobs completed successfully: scanner 62/62, collector 9/9, quality 5/5, ONE CLI build, and repository self-scan. Both reports matched `SOURCE_SHA=EXPECTED_SOURCE_SHA=9f75a73a2e804d7053e9dffb0dfcab9a62aa1984` and reported 0 critical, 0 high, 0 medium, 0 low, and 27 informational findings. This supersedes the earlier scanner failures on parent `ea515...` and `9d2bb...` for the tested scanner scope; it does not award milestone E1–E6 acceptance points by itself.
- **Prior canonical source runs on `8ddcfb8...`:** One CI [41217339585](https://github.com/panglimanetizen01/one/actions/runs/41217339585), Repository Health [41217339577](https://github.com/panglimanetizen01/one/actions/runs/41217339577), focused stdlib [41217339600](https://github.com/panglimanetizen01/one/actions/runs/41217339600), E1 [41217339578](https://github.com/panglimanetizen01/one/actions/runs/41217339578), E2 [41217339580](https://github.com/panglimanetizen01/one/actions/runs/41217339580), Linux distro [41217339576](https://github.com/panglimanetizen01/one/actions/runs/41217339576). All inspected jobs had runner_id=0, zero steps, and no artifacts. This is pre-run infrastructure/evidence failure; root cause UNKNOWN.
- **Parent `297b169...` Check Runs:** 15 exact-SHA Check Runs were returned; all had null output title/summary. No explicit billing block was established. The annotation API was not independently retrievable through the available connector. The collector changes have not yet run inside repository-health CI.
- **Exact-SHA parent evidence:** verifier health run [41218265996](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41218265996) confirmed checkout `ea515c321f337125ba2d29da0756e8c2f9a50066` on Ubuntu and Windows. The scanner suite failed with one error (secret-redaction fixture) and one failure (expected subprocess-shell-true vs actual subprocess-shell-true-ast); collector 9/9, quality 5/5, CLI build, and self-scan succeeded. Self-scan counts were 0 critical/high/medium/low and 27 info. E1 cross-platform [41218265882](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41218265882), E1 Windows [41218265877](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41218265877), focused stdlib [41218266019](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41218266019), and Debian 12 distro subjob in [41218266177](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41218266177) passed their stated scopes; Ubuntu distro subjob was cancelled after its build/Clippy/tests/network steps reported success. These are parent-SHA scoped results, not final acceptance.
- **Verifier:** `CANONICAL_COMMIT.txt` and the current-state section of `CANDIDATE_VERIFICATION.md` are synchronized to source `9f75a73a2e804d7053e9dffb0dfcab9a62aa1984`; verifier branch head is `993c5236bf0da7a8b4f20aefbe8a6e9a4811307b`. Scanner run [41219231598](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41219231598) is a completed exact-SHA Ubuntu/Windows PASS for its declared scope. Keep separate `LOCAL_CI_CANONICAL_COMMIT.txt` at PR #92 SHA `62e391e699fe4ac5a18d6feb808568c192dd82e1`.
- **DoD/ledger:** live `docs/verification/DEFINITION_OF_DONE.md` has 3 checked / 10 unchecked / 13 total. Exact-SHA gate [41219231670](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41219231670) checked out source `9f75a73a2e804d7053e9dffb0dfcab9a62aa1984` and reported implementation 92/100, verified 60/100 (F1–F6 and C1–C5), 18 rows checked, and gate NOT PASSED. It retained E1 0/10, E2 0/8, E3 0/8, E4 0/6, E5 0/4, E6 0/4; its evaluation occurred while some exact-SHA matrix runs were still queued/in progress. The current scoped E1/E4 results below do not satisfy full-row acceptance. Re-run the ledger gate after the full exact-SHA matrix has completed; final score remains UNKNOWN / NOT RECONCILED. Fixed weights unchanged.
- **Current scoped acceptance evidence (not row promotion):** E1 cross-platform lab [41219231685](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41219231685) and E1 Windows lab [41219231643](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41219231643) completed successfully, including the declared standard-library suites, workspace tests, Clippy, and contract check. E4 SEC-05 [41219231615](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41219231615), E4 filesystem/resource regression [41219231672](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41219231672), and E4 subprocess resource guard [41219231678](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41219231678) completed successfully in their declared scopes. Linux distro verification [41219231695](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41219231695) completed successfully for Ubuntu 24.04 and Debian 12. These independent-verifier runs are not canonical source CI and do not independently satisfy full E1/E3/E4 rows; keep E1–E6 verified points at zero until each full acceptance contract is met.
- **No-repeat/safety:** do not repeat the 132-file inventory or identical zero-step workflow retries. No merge, production release, protected E2 dispatch, secret/trust-store mutation, branch-protection bypass, or direct main edit. E2 release remains unauthorized without explicit approval.



**Status:** In progress — not a release authorization  
**Candidate branch:** `milestone/100-ecosystem-20261023`  
**Candidate source SHA:** see the synchronized source pin and evidence index; this reusable checklist intentionally does not embed a self-referential commit SHA.  
**Fixed-weight ledger:** [one-completion-ledger.md](one-completion-ledger.md)  
**Independent evidence index:** [CANDIDATE_VERIFICATION.md](https://github.com/panglimanetizen01/one-windows-verification/blob/verification/CANDIDATE_VERIFICATION.md)  
**PR:** [#80](https://github.com/panglimanetizen01/one/pull/80)

This checklist is an acceptance control, not a claim that the release is complete. A checked item requires direct evidence tied to the exact source SHA. Do not adjust ledger weights or award credit because a workflow was started, a bundle exists, or a test on another SHA passed.

## Historical exact-SHA reconciliation — 2026-10-28 (superseded by active scanner handoff below)

The live PR #80 head is authoritative; the verifier pin/index must match that exact final candidate. The repository-health scanner now includes JavaScript/TypeScript source files and heuristic rules for obfuscated dynamic execution, network-plus-execution correlation, sensitive-source/network-sink correlation, and Node shell-oriented child-process calls. Three positive/negative regression fixtures were added. Isolated detector snippets were exercised, but no full scanner unit suite or self-scan result is accepted yet.

Latest canonical source workflow runs for the scanner candidate returned failed job summaries with null steps/log URLs and no artifacts; log retrieval returned `BlobNotFound`. This is a blocked execution/evidence path, not proof that source tests failed or passed. Root cause remains unknown. The canonical repository-health workflow includes unit tests and a direct scan of the checked-out ONE repository; it must produce inspected steps, exit codes, and report artifacts before acceptance.

Current checkbox count: **3 checked, 10 unchecked**. Final exact-SHA score remains **UNKNOWN / NOT RECONCILED**; gate **NOT PASSED**. Do not merge or release.

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
