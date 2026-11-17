# ONE v0.1 — Definition of Done and Release Gate

## Current continuation state — 2026-11-17 15:55 WIB (superseding live refresh)

- Canonical repo remains PUBLIC at main SHA `d35db95153f6ddd79885494c72f5aee16c899751`; no merge/release.
- PR #95 CircleCI migration remains OPEN / DRAFT / UNMERGED at source SHA `993f76351c7a119bea2e66f7d9e5bca07f392fbc`. `.circleci/config.yml` exists on PR #95 only; main and PR #102 do not contain it. CircleCI is not active on main.
- Previous source `ed2382...` passed independent Windows run #39 / `44046670869`, verifier `a07c79d...`, job `133913951181`; all primary steps succeeded. Artifact `14312311019`, ZIP SHA-256 `aa56f8275042969b18c12283eb152e0572b8970159e3df7db62e9aad9f109da1`; native/CLI hashes and manifest SHA are recorded in the ledger. ZIP/manifest/binary hashes and PE32+ AMD64 headers were checked. Native and CLI output `Budi`, exit 0; CLI version `one 0.1.0`. This PASS is not transferred to `993f763...`.
- PR #42 pin matches `993f763...`; current verifier PR head `f9185e84b5f136f1ba43946e358febaa204cb466`. Run #41 / `44047540578` was PENDING at last poll. Run #40 / `44046678525` remained IN_PROGRESS on prior source pin. Current-source Windows gate remains pending until exact-SHA run and artifact inspection.
- PR #95 current SHA has no CircleCI status visible through GitHub at last read. Optional `release-check` remains NOT VERIFIED.
- Correction: CircleCI Pipeline #558 belongs to PR #102 SHA `5ee645507199bdd94955099f6633aa67bb697ee5`, not PR #95. PR #102 GHA run #44044795819 is PASS on Ubuntu/Windows, while generic CircleCI Pipeline status is ERROR. Main and PR #102 lack `.circleci/config.yml`, whereas PR #95 contains it. Pipeline root cause remains NOT VERIFIED without CircleCI logs.
- PR #102 remains OPEN / DRAFT / UNMERGED at `5ee645...`; PR #98–#101 each have exact-SHA Ubuntu+Windows GHA PASS. These do not constitute final release acceptance. PR #96's old CircleCI Windows/MSYS2 fix is superseded; do not reintroduce retired Windows job.
- PR #97 current docs head `e474f2d29c3b6fe2aaae3696d99b90c0b65613db`; PR #103 current ledger/DoD head will be this commit. All remain OPEN / DRAFT / UNMERGED.
- Fixed-weight inventory remains 92/100 provisional implementation, 0/100 exact-SHA verified. Historical DoD count 3 checked / 10 unchecked / 13 total; revalidate all at final source SHA. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; release acceptance NOT PASSED.
- No score, checkbox, E6, or release acceptance is promoted. No merge/release, protected dispatch, force-push, history rewrite, secrets, visibility, billing or default-branch changes.

---


## Superseded current-state snapshot — 2026-11-05 (replaced by continuation state above)

- Canonical source: `panglimanetizen01/one`; branch `milestone/100-ecosystem-20261023`; PR #80 open/draft/unmerged; base `main` at `d35db95153f6ddd79885494c72f5aee16c899751`.
- Current source HEAD: `85b035a9f0ec8512a13bb66c987e2f8ec024d8fd`; parent: `fecf6e0883124e47aebdda13294714b5d9bc15d5`. The independent verifier `verification` branch currently pins `1248a39199322fd4007cbd043bb58864096842c8`; this mismatch blocks current-HEAD acceptance.
- Canonical source workflows on `85b035a9...`: One CI [42019582401](https://github.com/panglimanetizen01/one/actions/runs/42019582401), E1 [42019582386](https://github.com/panglimanetizen01/one/actions/runs/42019582386), E2 [42019582378](https://github.com/panglimanetizen01/one/actions/runs/42019582378), Linux distro [42019582389](https://github.com/panglimanetizen01/one/actions/runs/42019582389), Repository Health [42019582606](https://github.com/panglimanetizen01/one/actions/runs/42019582606), and focused stdlib [42019582717](https://github.com/panglimanetizen01/one/actions/runs/42019582717). All completed FAILURE with returned job `steps: null`; representative log downloads returned `BlobNotFound`; artifact lists were empty. Classify as pre-step execution/evidence failure, root cause UNKNOWN—not source-test failure and not PASS. Do not repeat identical retries without a changed condition or transient diagnosis.
- Last observable verifier matrix is pinned to `1248a391...`, not current source. It found actual Linux fixture failures in `crates/one-cli/tests/project.rs`; the later source commits added the missing temporary-directory creation. It also found Windows CLI/package regressions involving reparse-point/canonical-path rejection in ordinary test temp directories. These failures are real for the tested `1248a391...` source and remain historical until the changed current source is re-run. Android API 35 AArch64 runtime run [41262527363](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41262527363) passed on `1248a391...` only; not current-HEAD evidence.
- Verifier ledger gate [41262533958](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/41262533958) returned NOT PASSED: evidence index did not name exact pin, 10 DoD items unchecked, E1–E6 all 0 verified, several matrix jobs incomplete/failed, and no protected production provenance run.
- DoD checkbox count on the current source file is **3 checked / 10 unchecked / 13 total**. Last measured ledger subtotal remains **92/100 implementation provisional / 60/100 verified on an older source SHA**; it is not a current-SHA acceptance score. Final score **UNKNOWN / NOT RECONCILED**. E1–E6 remain 0 verified; fixed weights unchanged.
- PR #84 remains open/draft and is two commits ahead/three behind current `main`; its only substantive README delta is already represented in PR #80's branch. PR #85 remains open/draft and is three commits ahead/three behind `main`; its CI/release-check delta overlaps checks already present in PR #80. Neither PR is merged; do not merge or close automatically. PR #83 remains an unaccepted SEC-05 experiment; #78 is scope-limited historical milestone-60 evidence; #77 is proposal/audit only; #76 remains a separate HTTP IPv6 fix requiring fresh base/diff review.
- No merge, release, protected E2 dispatch, secret/trust-store mutation, or direct `main` edit is authorized. Pin synchronization and all further evidence must target the final resulting source SHA.

---


## Historical forensic audit — parent SHA fecf6e0

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


## 10. Windows subprocess containment — Clippy correction checkpoint — 2026-11-17

Run #44010448799 on PR #102 source `eaa7cf77bd9971f1fd9b5205cef975e6acad270f` completed with Ubuntu SUCCESS and Windows FAILURE. Windows build and contract validation passed; strict Clippy failed on one redundant `.into()` at `crates/one-core/src/interop.rs:894`. Tests were skipped, so this run provides no result for the stable process-object handle regression. The redundant conversion was removed in commit `da740c4f7f9c72e9b76e573d23651334272582c5`. This creates a new source SHA and requires fresh exact-SHA Windows verification. Keep subprocess containment unchecked until the new run passes Clippy and the descendant-handle test; do not infer a containment failure or pass from the skipped test.

## 11. Windows subprocess containment — retained process-object failure — 2026-11-17

Run #44012805081 on PR #102 SHA `da740c4f7f9c72e9b76e573d23651334272582c5`: Ubuntu SUCCESS; Windows build and Clippy SUCCESS; Windows tests 119 passed / 1 failed. The failure is the stable retained-process-handle regression: `WaitForSingleObject` returned `WAIT_TIMEOUT=258` for descendant PID 3444 after `run_process` returned. Because the test holds the original process object handle duplicated before the wrapper exits, this is not PID reuse. The test then terminates the test-owned descendant through that same handle. A diagnostic follow-up at SHA `efc5422845927c2a46e08e61aeb6a497e2f9980b` adds a `GetProcessId` identity check and `GetExitCodeProcess` snapshot. Run #44018063886 is IN_PROGRESS on this exact SHA. Keep DoD containment unchecked until the next run identifies the handle's process ID/exit state and a subsequent implementation fix passes.

## Current PR #95 CircleCI migration candidate — 2026-11-17

- PR #95 current source HEAD: `c71d81a60f7792ed8536ba2b70464b9922f33c18`; OPEN / DRAFT / UNMERGED.
- The candidate now tracks `Cargo.lock`, synchronizes the `one-core` Unix/Windows target dependencies, uses `--locked` for all CircleCI Cargo build/Clippy/test commands, and enforces the lockfile/locked-command contract in `tools/check_contract.py`.
- CircleCI `ci/circleci: one-linux` is PENDING for this exact SHA (workflow `07e89069-4b33-49f6-b32c-8e70519f5221`, job `781e65c4-e43d-4e55-89d3-beaa61623629`). Earlier CircleCI failure root cause remains NOT VERIFIED because authenticated job logs are unavailable in the current tool context.
- Independent verifier PR #42 now pins this exact SHA, but no workflow run has been observed. Windows run #32 is PASS only for older source SHA `a10a69afdf8675e703f38f1620abc935d8361897`; no Windows PASS is claimed for `c71d81a...`.
- PR #95 migration acceptance remains NOT PASSED. The optional release-check remains NOT RUN. No merge or release.


## PR #95 latest status correction — 2026-11-17

- Code/config candidate `c71d81a60f7792ed8536ba2b70464b9922f33c18` has CircleCI `ci/circleci: one-linux` terminal FAILURE (workflow `07e89069-4b33-49f6-b32c-8e70519f5221`, job `781e65c4-e43d-4e55-89d3-beaa61623629`). First failing step/root cause remains NOT VERIFIED: the active session has no CircleCI CLI, token/auth configuration, connected CircleCI job-log action, or accessible CircleCI job page.
- Documentation-only descendant PR #95 HEAD is `dd9960ff52d005c4c5bf02bc11fc3c8ef8482ab9`; its fresh CircleCI status is PENDING (workflow `e02ca4dd-a15a-4d44-a5bb-7c2a2db13629`, job `d7aaea88-f864-489d-b717-b95b60dc0078`). Do not attribute the older failure to the descendant or claim its pending run passed.
- Independent verifier PR #42 now pins `dd9960f...`, but no Windows run is observed. Run #32 is PASS only for `a10a69...`.
- PR #95 migration acceptance remains NOT PASSED; optional release-check NOT RUN; no merge/release.


## PR #95 CircleCI first-failure diagnosis and APT mirror mitigation — 2026-11-17

- User-retrieved authenticated CircleCI output identifies the first failure on source `c71d81a60f7792ed8536ba2b70464b9922f33c18`: cross-toolchain provisioning step 103, `apt-get install`, exit 100. Multiple MinGW package URLs from `us-east1.gce.archive.ubuntu.com` returned HTTP 404 while other packages downloaded. This establishes an Ubuntu regional APT mirror/index-to-pool inconsistency, not a Rust build/test failure.
- Targeted CircleCI config mitigation committed as `481873f277cb8601fc8881f8e5cc3fb0383f9115`: rewrite the affected regional mirror hostname to `archive.ubuntu.com`, clean and refresh APT metadata, and install the same package set with Acquire retries.
- CI documentation commit: `23bb9ef79df129edaa848aa3388d8dbe39309fa2`. PR #95 HEAD at the latest status check: `23bb9ef79df129edaa848aa3388d8dbe39309fa2`; CircleCI `ci/circleci: one-linux` PENDING (workflow `74af91bc-d622-4abf-8301-3c52575a2069`, job `29ae0085-428e-42ce-a4d6-80a37442870a`). The mitigation is not yet proven.
- Independent Windows verifier PR #42 now pins the same SHA: pin commit `3a1e6fadcf9c72927186b3baebe61ab3cca2354a`; verification-doc commit `42ae8720f197aaf312867776984a4f8fce2cf5fd`. No Windows run for this SHA is yet claimed.
- PR #95 remains OPEN / DRAFT / UNMERGED; CircleCI migration acceptance NOT PASSED. E6 remains NOT IMPLEMENTED / final release gate NOT PASSED. No merge or release.


## PR #95 APT mirror mitigation shell correction — 2026-11-17

- The user-provided CircleCI output established the initial provisioning failure: MinGW package HTTP 404 responses from `us-east1.gce.archive.ubuntu.com`, APT exit 100 (regional mirror/index-to-pool inconsistency).
- The targeted mirror mitigation was committed as `481873f277cb8601fc8881f8e5cc3fb0383f9115`. Static review then found and corrected doubled shell escapes in the generated `find` command; correction commit `0dbc23dd47243a7b2ae5736c32943e3014353d4f`.
- CI docs record the diagnosis and correction in `4db76fbaf651dd069c7000fb46a4dea61b11fbd8`. PR #95 latest HEAD at this checkpoint: `4db76fbaf651dd069c7000fb46a4dea61b11fbd8`; CircleCI status had not yet appeared for this exact SHA. No PASS claim.
- Independent Windows verifier PR #42 advanced its pin to `4db76fb...`: pin commit `860d9c2d2aaa0eda8c76572271e0c18c49cf3d37`; docs commit `8c493983fc95b75f32f5862afccb0489786ca302`. No Windows result for this SHA is claimed.
- PR #95 remains OPEN / DRAFT / UNMERGED; CircleCI migration acceptance NOT PASSED. E6 remains NOT IMPLEMENTED / final release gate NOT PASSED. No merge/release.


## PR #95 post-correction CircleCI rerun — 2026-11-17

- CircleCI is PENDING for the exact APT shell-correction source `4db76fbaf651dd069c7000fb46a4dea61b11fbd8`: workflow `d436555e-6a03-4167-a844-acc2ac5e50d6`, job `b8478a26-15f6-457c-b8e4-f30e461e17e9`.
- PR #95 then advanced to documentation-only descendant `9b233dfe75da8c0752c203af3711bf3da319dc02`, which had no status reported at the subsequent check. No terminal PASS is claimed for either SHA.
- Independent verifier PR #42 pin advanced to `9b233df...`: pin commit `e88b2be4e0845f932cd978e26305d25f9730f777`; docs commit `af52f1ab4897f8a464b787fa895ba2051f311372`. No Windows run is yet claimed for this pin.
- PR #95 remains OPEN / DRAFT / UNMERGED; CircleCI migration acceptance NOT PASSED. E6 remains NOT IMPLEMENTED / final release gate NOT PASSED.


### PR #95 APT mitigation and latest CircleCI terminal status refresh — 2026-11-17

- Live PR #95 remains OPEN / DRAFT / UNMERGED; exact current HEAD `9b233dfe75da8c0752c203af3711bf3da319dc02`.
- CircleCI `ci/circleci: one-linux` is terminal SUCCESS on the APT shell-correction code/config SHA `4db76fbaf651dd069c7000fb46a4dea61b11fbd8`: workflow `d436555e-6a03-4167-a844-acc2ac5e50d6`, job `b8478a26-15f6-457c-b8e4-f30e461e17e9`. This verifies the targeted mirror mitigation and corrected shell expression on that exact source SHA.
- The current PR HEAD `9b233dfe75da8c0752c203af3711bf3da319dc02` has terminal CircleCI FAILURE: workflow `91d704b2-516e-4736-9fb2-d4c8504e001b`, job `948e41bd-2090-4708-8b1b-1dc142672580`.
- The current connected tools cannot retrieve the authenticated CircleCI job page/logs. The first failing step and root cause for `9b233d...` remain UNKNOWN. Do not attribute this failure to APT, docs, or product code without the first-failure output. Do not claim the current PR head is green.
- Independent verifier PR #42 remains OPEN / DRAFT / UNMERGED and is pinned to `9b233d...`; no Windows run for this exact pin is observed. Historical Windows run #32 applies only to `a10a69...`.
- Migration acceptance remains NOT PASSED. E6 remains NOT IMPLEMENTED / final release gate NOT PASSED. No merge or release.


### PR #95 latest docs/CI checkpoint — 2026-11-17

- Corrected stale current-status content in `docs/CI.md` on PR #95 branch. Commit `cef1f41c97585b6e25bf91af6e8b4281efe299c4`; exact new PR #95 HEAD `cef1f41c97585b6e25bf91af6e8b4281efe299c4`.
- The new docs section records CircleCI SUCCESS on APT-fix code/config SHA `4db76fbaf651dd069c7000fb46a4dea61b11fbd8` and terminal FAILURE on docs-only descendant `9b233dfe75da8c0752c203af3711bf3da319dc02` (workflow `91d704b2-516e-4736-9fb2-d4c8504e001b`, job `948e41bd-2090-4708-8b1b-1dc142672580`; cause UNKNOWN due inaccessible authenticated logs).
- New exact PR #95 HEAD `cef1f41...` currently has no status reported. Do not claim PASS; this exact docs descendant requires its own terminal CircleCI result.
- Independent Windows verifier pin remains `9b233d...`; no run for `cef1f41...`. Migration acceptance NOT PASSED; E6 NOT IMPLEMENTED / release gate NOT PASSED; no merge/release.


### PR #95 latest exact-SHA CircleCI rerun — 2026-11-17

- Exact PR #95 HEAD `cef1f41c97585b6e25bf91af6e8b4281efe299c4` now has CircleCI `ci/circleci: one-linux` PENDING: workflow `930378d8-49bb-416e-a42c-63e1797b991c`, job `0e2642b0-bd94-4892-baca-df5b6c8fa4ed`.
- This is a fresh run on the corrected documentation descendant. No terminal outcome is available yet; do not claim PASS until terminal SUCCESS.
- APT shell-correction code/config SHA `4db76fb...` remains CircleCI SUCCESS. Preceding docs-only SHA `9b233d...` remains terminal FAILURE, first failing step/root cause UNKNOWN because authenticated logs are inaccessible.
- Independent Windows verifier PR #42 remains pinned to `9b233d...`; no Windows run for `cef1f41...` is observed. Migration acceptance NOT PASSED; E6 NOT IMPLEMENTED / release gate NOT PASSED; no merge/release.


## Live evidence correction — 2026-11-17 12:51 WIB

This addendum supersedes older current-state statements above where they conflict; historical evidence remains preserved.

- PR #102 current HEAD `7e10e758d7e1ca70f450bfbb6cf397d805500e53`: GitHub Actions run [#44019833034](https://github.com/panglimanetizen01/one/actions/runs/44019833034) terminal SUCCESS on exact SHA. Windows job `133826414765` and Ubuntu job `133826415064` both succeeded; the Windows descendant Job Object regression passed. PR #102 remains OPEN / DRAFT / UNMERGED.
- PR #95 current HEAD `cef1f41c97585b6e25bf91af6e8b4281efe299c4`: CircleCI `ci/circleci: one-linux` terminal SUCCESS on exact SHA; workflow `930378d8-49bb-416e-a42c-63e1797b991c`, job `0e2642b0-bd94-4892-baca-df5b6c8fa4ed`, status timestamp 2026-11-17 05:49:29 UTC. This is terminal commit-status evidence; step-level logs were not available through the connected integration.
- PR #95 remains OPEN / DRAFT / UNMERGED, so CircleCI is not yet active on main. Independent verifier PR #42 still pins older source `9b233dfe75da8c0752c203af3711bf3da319dc02`; same-SHA Windows verification for `cef1f41...` is NOT VERIFIED. Release acceptance remains NOT PASSED.
- Do not promote completion points or DoD items based on these results alone. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`. No merge or release.


## Superseding live verification refresh — 2026-11-17

- ONE `main` remains PUBLIC at exact SHA `d35db95153f6ddd79885494c72f5aee16c899751`; no merge or main update occurred.
- PR #102 exact source `7e10e758d7e1ca70f450bfbb6cf397d805500e53`: GitHub Actions run #44019833034 terminal SUCCESS; Windows job `133826414765` and Ubuntu job `133826415064` both PASS. Windows descendant Job Object regression passed. PR remains OPEN / DRAFT / UNMERGED.
- PR #95 exact current HEAD `cef1f41c97585b6e25bf91af6e8b4281efe299c4`: CircleCI context `ci/circleci: one-linux` SUCCESS, workflow `930378d8-49bb-416e-a42c-63e1797b991c`, job `0e2642b0-bd94-4892-baca-df5b6c8fa4ed`. This is terminal exact-SHA commit-status evidence; no step-level CircleCI logs are available in the current integration. PR remains OPEN / DRAFT / UNMERGED, so CircleCI is not active on main.
- Independent verifier PR #42 branch `verify/windows-pr95-20261114` now pins the same source SHA `cef1f41...`. Pin commit `f5113c084fa978f275bc8426a4ca1e4d74376549`; documentation commit and current PR head `44b36a1894382cc2508edf0166f60b06c9434d7a`. The PR-branch pin update does not trigger the base `verification` branch workflow. No authorized workflow-dispatch action is exposed; Windows exact-SHA verification remains NOT VERIFIED. Historical Windows run #32 remains PASS only for `a10a69afdf8675e703f38f1620abc935d8361897`.
- PR #97 remains OPEN / DRAFT / UNMERGED at `3cb0a52f92bb1d9ee6497e82c27425aebeab21b9`. PR #103 remains OPEN / DRAFT / UNMERGED at `049f4680c6a86545a0eea27fc8bf840b4872edd2`.
- Health-scanner provenance is branch-scoped: verifier PR #39 pins ONE PR #91 source `9b38c246f0ef18f6a39a79ff2be81c4ba06b63db`; verifier PR #40 pins ONE PR #89 source `242b88e4c8b6f2c2c2039f2bc95ebeab20ae96de`. Both exact source revisions contain the Python scanner and tests. The workflow is absent from current default branches and the scanner is absent from ONE main. Preserve these historical draft verifier paths; do not reintroduce the scanner into ONE main or delete the branch workflows without a supersession decision.
- E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`. The provisional implementation inventory remains 92/100 and exact-SHA verified score remains 0/100; no score or DoD item is promoted by these CI results alone.
- No merge, release, visibility change, force-push, history rewrite, secret, billing, or default-branch mutation was performed.


## Superseding live checkpoint — 2026-11-17 (latest exact-SHA refresh)

This section supersedes prior current-head statements where they conflict. Historical run outcomes remain preserved.

- Main remains PUBLIC, default branch `main`, exact HEAD `d35db95153f6ddd79885494c72f5aee16c899751`; no main mutation occurred.
- PR #102 current source SHA `7e10e758d7e1ca70f450bfbb6cf397d805500e53`: GitHub Actions run #44019833034 terminal SUCCESS. Windows job `133826414765` and Ubuntu job `133826415064` both succeeded; Windows test `windows_job_object_terminates_descendants_when_parent_exits` passed. The older Windows failure on SHA `a472b3b...` is historical and superseded for this newer source SHA. PR #102 remains OPEN / DRAFT / UNMERGED. CircleCI status for this branch is a separate error status and does not negate the GitHub Actions result or establish PR #95 migration acceptance.
- PR #95 source SHA `5ba239ac3964dc147d1568dac2b819459f31b463` is a documentation-only descendant. CircleCI `ci/circleci: one-linux` is PENDING (workflow `5ec5c4c5-e6ab-4518-8d46-1178ba75333f`, job `59fcdba6-1647-4352-ad02-1add7eebae22`). Prior CircleCI SUCCESS on `cef1f41...` is not transferred to this new exact SHA.
- Independent verifier PR #42 pin is synchronized to `5ba239...`; pin commit `764d197864b19181ba7a814bf93c56c596bf8600`, verifier docs/current head `6fd76ed85189923a1c0f661638f8077f290955a1`. No Windows run is observed for `5ba239...`; Windows status remains NOT VERIFIED. Historical Windows run #32 applies only to `a10a69...`.
- PR #97 remains OPEN / DRAFT / UNMERGED at `02e6b0155d35e10e9d66e10278566df3e86faae2`. PR #103 remains OPEN / DRAFT / UNMERGED while this documentation update is being made.
- The optional CircleCI `release-check` parameter `run_release_checks=true` has not been evidenced as executed. Release acceptance remains NOT PASSED.
- No score, weight, or Definition of Done checkbox is promoted by these CI results. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; historical provisional implementation inventory remains 92/100 and exact-SHA verified score remains 0/100.
- No merge, release, force-push, history rewrite, visibility change, secret change, billing change, or default-branch mutation occurred.


## Latest locked-dependency CI checkpoint — 2026-11-17

- PR #95 current source SHA: `d42689718870236694f6cab841f8b7be1afccfcc`. CircleCI `ci/circleci: one-linux` is PENDING (workflow `6af2386b-dd8d-47c8-b414-4237e92130fd`, job `6cf4f8c5-7b2d-4876-a448-89dc0c596c68`). No status from an ancestor SHA is transferred.
- Offline scripts `tools/verify_offline.sh` and `tools/verify_offline.ps1` now use `--locked` for every Cargo build, Clippy, test, and run command; `tools/check_contract.py` rejects any omission.
- Independent verifier PR #42 pin matches `d426897...`; verifier workflow enforces `--locked` on all Cargo commands and includes a self-check for the rule. Verifier docs/current PR head: `9475956ae0ddccb969a5d14d1c9e803c7e44d097`.
- Windows exact-SHA run for `d426897...` is NOT VERIFIED. Optional CircleCI `release-check` remains NOT VERIFIED.
- No implementation score, verified score, DoD checkbox, or E6 status is promoted. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; release acceptance remains NOT PASSED.


## Superseding exact-SHA evidence boundary — 2026-11-17

- PR #95 code/tooling candidate `d42689718870236694f6cab841f8b7be1afccfcc` has CircleCI `ci/circleci: one-linux` terminal SUCCESS (workflow `6af2386b-dd8d-47c8-b414-4237e92130fd`, job `6cf4f8c5-7b2d-4876-a448-89dc0c596c68`). It includes `--locked` enforcement in both offline scripts and the source contract checker.
- Current PR #95 HEAD `c207fb15bc19889a8c8e60923581ef1b1f7a6374` is a documentation-only descendant and its CircleCI status is PENDING (workflow `15aa40bb-0225-4ad6-a2ea-fdaa0f364b0b`, job `cdcc88bd-abdc-43b0-9885-1e696bd07ac8`). Do not transfer the prior SUCCESS automatically.
- Independent verifier PR #42 pin matches `c207fb...` (pin commit `8db417749753009ad5dbc69f8b4d2bb00c5d1642`; verifier docs commit/current head `b45f9a259ea072168a1efc3300300438e78fbbf6`). No matching Windows run is observed; status NOT VERIFIED. The verifier workflow now uses `--locked` for all Cargo commands and self-checks the invariant.
- PR #102 exact source `7e10e758d7e1ca70f450bfbb6cf397d805500e53` remains GitHub Actions PASS on Ubuntu and Windows (run #44019833034); the Windows descendant Job Object regression passed.
- Optional CircleCI `release-check` with `run_release_checks=true` remains NOT VERIFIED. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; no score or DoD checkbox is promoted.


## Current PR #95 docs-only descendant and Windows rerun — 2026-11-17

- Previous PR #95 source `c207fb15bc19889a8c8e60923581ef1b1f7a6374` has CircleCI Linux terminal SUCCESS and independent Windows run #34 terminal SUCCESS (run `44030597595`, verifier SHA `3f8e9b662805a89fd5145e8691a221218b098e11`, job `133861337521`). Artifact ID `14305081962`; ZIP SHA-256 `7cbbf251efe2f56ba24965a76cd420ccddae8e84e8f0dcd727b09943014f3b8f`; executable SHA-256 `0c79f06f37f793061884acf43c9ba419bb0fc4ccf48c4a487f511e462cf55af5`; manifest SHA-256 `7ac05a71f1f5e5149f1dd721d2ca9c610bfe7504acb6b164938a51255c60b1d6`; native exit 0 / output `Budi`.
- Current PR #95 HEAD `048f87f5e9c14fdda33c7fa7a3d6dd77f469b9d4` is a docs-only descendant recording the run #34 evidence. CircleCI on this exact SHA is PENDING (workflow `875e39ae-399d-4803-81f7-407c7dc3f0d3`, job `40fbbc41-4a32-4796-b817-d144f8186e46`).
- Independent verifier PR #42 pin matches `048f87...`; Windows run #35 / ID `44031205160` is IN_PROGRESS with verifier SHA `62f401db17e5bc2ff3ae4c53e02cc721ed1d191e`. Run #34 remains bound only to `c207fb...`.
- Optional CircleCI `release-check` remains NOT VERIFIED. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; no score/DoD promotion or release acceptance.


## Latest PR #95 exact-SHA candidate — 2026-11-17

- Previous source `048f87f5e9c14fdda33c7fa7a3d6dd77f469b9d4` has CircleCI Linux PASS and independent Windows run #35 PASS; artifact ID `14304459543`, ZIP SHA-256 `0238718f261ba9f9840d26eb243ce3196931f1f9927c98e438b3077f7cf1a58b`, executable SHA-256 `7d9eaa745e561bafe4203404c9d6b0601b584ff93eec46a379e5fa1d0e9afd0a`, manifest SHA-256 `eace9930574e939e94a3f5d18c75901ffe883a43852ff43c281b236ece22543e`; exit 0 / `Budi`.
- Current PR #95 HEAD `54bf0973f59c2705decd91a8457cf08d6bbb7f8e` is a docs-only descendant recording run #35. CircleCI is PENDING (workflow `4e04be28-3664-4842-a4c8-6844d1d45817`, job `cc67ef7a-1df1-4a31-a694-05725f702652`).
- Verifier PR #42 pin matches `54bf097...`; Windows run #36 / `44031839506` is IN_PROGRESS with verifier SHA `c40752bd134c83a671e928a0a2aab25953e4ad6e`. Do not transfer run #35 to this descendant.
- Optional CircleCI release-check remains NOT VERIFIED. E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED.


## Differential open-PR verification checkpoint — 2026-11-17

### PR #95 / PR #42 — latest exact-SHA PASS

- PR #95 source `54bf0973f59c2705decd91a8457cf08d6bbb7f8e`: CircleCI Linux SUCCESS (workflow `4e04be28-3664-4842-a4c8-6844d1d45817`, job `cc67ef7a-1df1-4a31-a694-05725f702652`) and independent Windows run #36 / `44031839506` SUCCESS (verifier `c40752bd134c83a671e928a0a2aab25953e4ad6e`, job `133865371299`). Artifact `14304669534`; ZIP SHA-256 `b281dc52a781063fdda0570e357d5f1cee30a86fc6b77fa4c287b52514c8adb2`; executable SHA-256 `515bb17d95223108934dbedce435d93ab823f231a3d6ce1be95ab2c532e51dd4`; manifest SHA-256 `73472141307457975dcc10dc9e923afa5be7e6d1b83e796cba05168de02f86f4`; runtime exit 0 / `Budi`.
- The optional CircleCI `release-check` remains NOT VERIFIED. PR #95 and verifier PR #42 remain OPEN / DRAFT / UNMERGED.

### PR #98, #99, #100 — exact-SHA legacy GHA PASS

- PR #98 SHA `a83321bc41b3d0d28378841e29a6169abed8ac15`: GHA run #2018 / `43923604400`, Ubuntu job `133497053339` and Windows job `133497052786`, both SUCCESS including build, Clippy, tests and vertical slices.
- PR #99 SHA `8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9`: GHA run #2019 / `43923948205`, Ubuntu job `133498292797` and Windows job `133498293201`, both SUCCESS.
- PR #100 SHA `6375356c9c275ace5fd35cf4a81799a9546e1e1d`: GHA run #2020 / `43924465030`, Ubuntu job `133500117683` and Windows job `133500117243`, both SUCCESS.
- Their CircleCI Pipeline contexts are ERROR because main has no `.circleci/config.yml` before PR #95 migration. This is not evidence of Rust test failure. Their GHA evidence remains exact-SHA historical PASS; reassess CircleCI after the authorized migration.

### PR #101 — compile failure fixed; new CI running

- Old SHA `bbcc86876553ffd9a97af12c11bf99bdabae2792`, GHA run #2021 / `43924632960`: Ubuntu and Windows both failed during Build, before Clippy/tests. Root cause: two new filesystem denial/approval regression tests were appended outside the closing brace of `#[cfg(test)] mod tests`, so `SemanticCapabilityId`, `ExecutionScope`, and `EffectClass` were out of scope.
- Fixed in commit `cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f` by moving the premature module-closing brace after the new tests. No test was removed or weakened; production behavior unchanged.
- New GHA run #2138 / `44032422058` is IN_PROGRESS on exact SHA `cdf7bd...`; Ubuntu job `133867259381` and Windows job `133867259061`. No PASS claim until both finish.

### PR #96 — superseded CircleCI Windows patch

- PR #96 only fixes MSYS2 keyring initialization in the historical CircleCI Windows executor path. Current PR #95 architecture intentionally uses CircleCI Linux plus the independent Windows verifier; the current CircleCI config has no Windows executor. Merging #96 would reintroduce the superseded Windows job. Its disposition is historical/superseded; leave open/unmerged, preserve original failure evidence.

### Completion controls

- PR #102 remains GHA PASS on exact source `7e10e758d7e1ca70f450bfbb6cf397d805500e53`, including Windows Job Object descendant termination.
- E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; no score or DoD checkbox is promoted. Optional release-check, clean installation/use acceptance and release authorization remain outstanding.
- Main remains PUBLIC and unchanged at `d35db95153f6ddd79885494c72f5aee16c899751`. No merge or release.


## Product-PR reconciliation — terminal update — 2026-11-17

### PR #101 is now PASS

- Exact source `cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f`; GHA run #2138 / `44032422058` terminal SUCCESS on Ubuntu job `133867259381` and Windows job `133867259061`.
- Root cause was the two filesystem-inheritance tests being outside the existing `#[cfg(test)] mod tests`; commit `cdf7bd5...` moved the module-closing brace after the tests. Build, Clippy, full tests and all listed vertical slices passed on both platforms. No tests weakened/deleted.
- PR #101 remains OPEN / DRAFT / UNMERGED.

### PR #98–#100 historical exact-SHA PASS

- #98 `a83321bc41b3d0d28378841e29a6169abed8ac15`: GHA #2018 / `43923604400`, Ubuntu `133497053339` and Windows `133497052786` SUCCESS.
- #99 `8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9`: GHA #2019 / `43923948205`, Ubuntu `133498292797` and Windows `133498293201` SUCCESS.
- #100 `6375356c9c275ace5fd35cf4a81799a9546e1e1d`: GHA #2020 / `43924465030`, Ubuntu `133500117683` and Windows `133500117243` SUCCESS.
- Their CircleCI Pipeline errors reflect that main has not yet received the CircleCI config; GHA results remain valid historical evidence for those exact SHAs. Recheck CircleCI only when the migration is authorized and active.

### Additional open-PR inventory and disposition

- #96 MSYS2 keyring fix targets the retired CircleCI Windows executor path; current architecture is CircleCI Linux + independent Windows verifier. Superseded; keep open/unmerged, do not reintroduce the old Windows executor.
- #80 is a 153-file milestone branch, head `d75b2846465ba53281198693750f2e6ba39e2e33`, with no exact-head executable CI evidence. It includes nine legacy GHA workflows, health scanner/developer tooling, experimental human-vNext code and broad product/runtime changes. Its own historical ledger records E1/E2/E4/E6 gaps and a NOT PASSED gate. It is not safe to merge as-is; decompose and compare product code against accepted v0.1 contracts before promotion.
- #94 is based on #80 and contains repository-health scanner/tooling; out of current product scope. Do not reintroduce scanner files into ONE.
- #92 main was scanned: no paths match `ci-local`/`ci_local`, and code search returned no content matches. However PR #92 is not a pure cleanup: its diff also changes product code and re-adds `.github/workflows/ci.yml`. Treat the extraction cleanup as already satisfied on main; do not merge #92 as a cleanup PR or reintroduce GitHub Actions. Review its product changes separately if still needed.
- #93 and #84 are older exact-SHA evidence documentation corrections, now superseded by PR #103's completion ledger/DoD and current evidence index; preserve history, do not merge stale snapshots.
- #85 adds legacy GitHub Actions user-documentation checks; current CircleCI source contract and docs checker supersede the old workflow path. Keep unmerged unless a missing check is proven.
- #83 remains a WIP SEC-05 process-launch experiment on a non-main base; no current exact-SHA CI evidence. PR #102 is a separate, main-targeted Windows Job Object/resource-containment fix with its own PASS evidence; do not conflate the two.
- #77 is a language-surface audit/proposal, not normative grammar acceptance. #78 is an older human-vNext implementation branch; PR #80 contains a broader milestone snapshot but is unverified and vNext remains non-normative until separately accepted.
- #76 HTTP IPv6 authority parsing remains an open non-draft PR on a stale base (5 ahead / 3 behind main). Its only GHA run #39606316737 ended with zero steps on both platforms; status NOT VERIFIED, not a code failure. It has parser regression tests, but requires a current-base update and executable verification before any merge decision. PR #80 contains a more extensive parser implementation, but that branch is also unverified; do not treat it as evidence.

### Current completion guard

- PR #95 source `54bf097...`: CircleCI Linux + independent Windows run #36 PASS exact SHA; PR #102 exact SHA `7e10e...` GHA PASS; PR #101 exact SHA `cdf7bd5...` GHA PASS; PR #98–#100 exact SHA GHA PASS.
- E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED. Optional CircleCI release-check, clean installation/use acceptance and final release authorization remain outstanding.
- No PR merge, branch deletion, release, visibility change, secret change, or history rewrite performed.


## Source installation and release CLI smoke evidence — 2026-11-17

- PR #102 exact source SHA `cb2748c5bb7143d62818068bd2a7128520ae2c69`; GitHub Actions run #44043930974 terminal SUCCESS on Ubuntu job `133904876367` and Windows Server 2025 job `133904877181`.
- Both platforms built the release CLI with `cargo build --release --locked -p one-cli --bin one` and passed their platform-specific smoke test: version output `one 0.1.0`, execution of a temporary One source program, expected output `Budi`.
- `docs/panduan-pengguna/instalasi.md` on the PR #102 candidate now documents source build prerequisites (Rust/Cargo 1.97+), clone/build commands, Linux `target/release/one` and Windows `target/release/one.exe`, version verification, and first program. It explicitly does not claim a prebuilt installer or official binary release; `.oneir`/`.onepkg` are not installers.
- All Cargo commands in the PR #102 GitHub Actions workflow now use `--locked`.
- CircleCI's separate error context on this main-based PR #102 branch is not the result of the PR #102 GitHub Actions workflow and does not negate the exact-SHA Ubuntu/Windows PASS. CircleCI migration remains a separate PR #95 candidate.
- This is implementation/verification evidence only; no release acceptance or DoD completion is inferred.


## Latest source-install and documentation-check gates — 2026-11-17

- PR #102 current exact source SHA `5ee645507199bdd94955099f6633aa67bb697ee5`; GHA run #44044795819 terminal SUCCESS on Ubuntu job `133907717583` and Windows Server 2025 job `133907717844`.
- Both platforms passed user documentation consistency and checker unit tests; both built the release CLI with `cargo build --release --locked -p one-cli --bin one`; both passed version and sample execution smoke tests (`one 0.1.0`, output `Budi`). Full workflow jobs were terminal SUCCESS.
- PR #102 installation guide now documents a truthful source-build path (Rust/Cargo 1.97+, clone, locked release build, platform binary path, version check, and first program); no installer or official binary release is claimed.
- PR #102 CI workflow includes user-doc consistency tests and Linux/Windows release CLI smoke. All Cargo build/Clippy/test commands use `--locked`.
- This is exact-SHA implementation/verification evidence only. It does not promote E6, final release acceptance, or DoD checkboxes.


## Latest PR #95 exact-SHA verification checkpoint — 2026-11-17

- ONE source `876b83170af2e77adcc43e408901e99ea37f6b0f` passed full independent Windows Server 2025 verifier run #38 / `44045612644`, verifier SHA `564fd1e0853dd424eb8d451678faaa77aaa59d11`, job `133910422388`. All 31 primary steps passed, including locked Cargo guard, contract/docs checks, workspace build/Clippy/tests, release CLI, product vertical slices, real-network/offline, Windows native backend, PE32+ inspection, native execution, manifest and artifact upload.
- Run #38 artifact ID `14310959518`, ZIP SHA-256 `8e10e35d04709eee8429f143ef8bde0ca358941b51a4785f49e934e94f9f163c`; native EXE SHA `11ed652bd05e09145c8ca304762b72391de952eb2f441344dd259efda38221c7`; CLI EXE SHA `61a5e8166831153c2b98f69e09ceeda84759cee53e8d060f406c5d8a7ea12dba`; manifest SHA `9844c92e89ed1ac805adbf757c8a111ba66163acf265f00f428ba587f090ba23`. ZIP readback, manifest hashes and PE32+ AMD64 headers independently matched.
- PR #95 docs-only descendant/current HEAD is `ed2382eed9022a8c67a13d72a5de6a4d9134f631`; verifier PR #42 pin matches it. New exact-source verifier runs #39 / `44046670869` (IN_PROGRESS) and #40 / `44046678525` (PENDING) were triggered by successive verifier branch pushes. Do not transfer run #38 to this descendant.
- CircleCI Pipeline #558 reported pipeline-level ERROR on the previous candidate without exposing a `one-linux` job context/log. This is not enough to diagnose a source test failure. Current integrations do not expose authenticated CircleCI pipeline details; optional `release-check` remains NOT VERIFIED.
- PR #95/#42/#97/#103 remain OPEN / DRAFT / UNMERGED. Main remains unchanged. E6 and release acceptance remain NOT PASSED.


## Active product remediation PR exact-SHA CI matrix — 2026-11-17

The following open product PRs were rechecked against live GitHub Actions run metadata. Each listed run is terminal SUCCESS on the exact current PR source SHA, with both Windows Server 2025 and Ubuntu jobs SUCCESS. These are implementation/test evidence only; the PRs remain open/draft/unmerged and do not independently complete release acceptance.

| PR | Exact source SHA | GitHub Actions run | Windows job | Ubuntu job | Scope |
|---|---|---:|---:|---:|---|
| #98 | `a83321bc41b3d0d28378841e29a6169abed8ac15` | `43923604400` | `133497052786` PASS | `133497053339` PASS | Validate project manifest in `one test`; reject unsafe package filename components and path traversal |
| #99 | `8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9` | `43923948205` | `133498293201` PASS | `133498292797` PASS | Open existing SQLite database files read-only; never create missing DB on query; preserve `:memory:` fixture behavior |
| #100 | `6375356c9c275ace5fd35cf4a81799a9546e1e1d` | `43924465030` | `133500117243` PASS | `133500117683` PASS | Restore lexical scope frames after nested block errors and failed event dispatch |
| #101 | `cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f` | `44032422058` | `133867259061` PASS | `133867259381` PASS | Apply coarse Filesystem parent coverage consistently to grants, scopes, denials and approvals; explicit parent denial precedes inherited grants |

- All four changes are confined to CLI/runtime/governance contract behavior and tests; no changes to the language grammar or beginner-facing syntax are claimed.
- CircleCI pipeline statuses for these PR branches are separately ERROR (pipeline numbers #369, #373, #377 and #550 respectively) while GitHub Actions is green. These are pipeline-level CircleCI statuses and are not enough to claim Rust test failures. The current PR #95 CircleCI migration candidate is separately being diagnosed; do not conflate CI providers.
- PR #96 (`25ea36051eb6fb4cefc40c1867f5a6fc3e777bca`) modifies the historical CircleCI Windows/MSYS2 setup, but it is superseded by the current PR #95 design (CircleCI Linux + independent Windows verifier). Its exact status shows CircleCI Linux PASS and Windows FAIL on the old workflow; do not merge/reintroduce the retired CircleCI Windows job. Preserve as historical evidence.
- These PRs remain implementation candidates until integrated through authorized review. No DoD checkbox, E6, verified score, or release acceptance is promoted solely by CI PASS.


### Superseding exact-SHA and CircleCI attribution correction — 2026-11-17
- PR #95 current source SHA `993f76351c7a119bea2e66f7d9e5bca07f392fbc`; verifier PR #42 pin matches. Run #41 / `44047540578` is PENDING at last poll.
- Previous source `ed2382...` passed full Windows run #39 / `44046670869`; artifact `14312311019`, ZIP SHA-256 `aa56f8275042969b18c12283eb152e0572b8970159e3df7db62e9aad9f109da1`; native/CLI outputs `Budi`, exit 0; CLI version `one 0.1.0`.
- Correction: CircleCI Pipeline #558 belongs to PR #102 SHA `5ee645...`, not PR #95. PR #95 `ed2382...` had no CircleCI context visible in GitHub; current PR #95 `993f763...` also has no status context at last read. PR #102 GHA PASS and generic CircleCI ERROR are separate; pipeline root cause NOT VERIFIED without CircleCI logs.
- PR #103 ledger/DoD current head is being updated in this commit; PR #97 README current head is being updated in its own docs branch. E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED. No merge/release.


## ProcessSpec input validation remediation — PR #102 — 2026-11-17

- Differential code review found that the custom Windows `CreateProcessW` path did not reject embedded NUL characters in executable path, arguments, or working directory. Win32 null-terminated strings could truncate these values, diverging from Unix `Command`; empty executable path also needed rejection before Windows PATH resolution.
- Added cross-platform `validate_process_spec` after the Process capability check and before OS-specific spawn. It rejects empty executable paths and embedded NUL in executable path, all args, and optional working directory. Added four regression tests and documented the rule in `docs/spec/one-v0.1-interop.md`.
- Current PR #102 exact source HEAD: `fe3e4d0ae55a90a0b07deec124def54f7c79f041`. Current-SHA GitHub Actions run has not yet appeared in the branch run list at this evidence boundary; do not claim PASS until Windows and Ubuntu jobs complete.
- Intermediate source `790dd187...` contained over-escaped NUL literals, corrected in `ac2ee8ea...`; final formatting commit `fe3e4d0...` is the current candidate. Any result for `790dd...` is superseded and not evidence for the current SHA.
- This is process interop validation only; ONE grammar, language semantics, CLI UX and release acceptance are unchanged. PR #102 remains OPEN / DRAFT / UNMERGED; no DoD checkbox or E4 score promoted pending exact-SHA verification.
