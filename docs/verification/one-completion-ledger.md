## Live continuation checkpoint — 2026-11-17 18:30 WIB

This checkpoint supersedes stale PR-head and run references below where they conflict. It records live GitHub state, not merge or release acceptance.

### Canonical repository and PR state
- `panglimanetizen01/one` remains PUBLIC; default branch `main`; canonical main SHA `d35db95153f6ddd79885494c72f5aee16c899751`. No visibility/default-branch change.
- PR #95 remains OPEN / DRAFT / UNMERGED, branch `ci/circleci-migration-20261113`, exact HEAD `993f76351c7a119bea2e66f7d9e5bca07f392fbc`. Its CircleCI config is blob `9d05f65badffae75e7e22bd1f0962e056b4960bd`; all Cargo build/test/Clippy/release commands use `--locked`. Main still contains the three legacy workflows `.github/workflows/ci.yml`, `release-check.yml`, and `windows-native-verification.yml`; PR #95 removes them on its candidate branch. No migration is active on main.
- PR #95 exact-source independent Windows evidence: run #42 / `44048174334` and docs-only verifier run #43 / `44049102193`, both pinning source `993f763...`; run #43 verifier HEAD `56271be82645357c2feaa1134ae2dc1d2ceae734`, Windows job `133922119577`, terminal SUCCESS. Latest artifact #14313416141, ZIP SHA-256 `4b5102dfdb1fcecc41a155f2b97fa3caf244986120c263f50bd5733888e7bbcd`; native and CLI PE32+ AMD64, both output `Budi`, exit 0; CLI `one 0.1.0`. Verifier PR #42 remains OPEN / DRAFT / UNMERGED and `WINDOWS_SOURCE_COMMIT.txt` exactly pins `993f...`.
- PR #95's current source has zero GitHub status contexts in the combined-status endpoint. CircleCI Linux execution and optional `run_release_checks=true` remain NOT VERIFIED. CircleCI config source review is not a CircleCI run. The active session has no authenticated CircleCI operational tool/CLI; do not claim pipeline success/failure or trigger jobs without that access.
- PR #97 remains OPEN / DRAFT / UNMERGED, branch `docs/system-architecture-diagram-20261115`, live HEAD `41d835fb23a459e736e62639394c2fc0e8e0d005`. Architecture document links the main diagram and preserves the recorded image SHA-256 `e2f30a7...`; no re-upload is needed.
- PR #102 remains OPEN / DRAFT / UNMERGED, branch `fix/commit-cargo-lockfile-20261117`, exact HEAD `ac1bb214bc028395367cce7edc234a1909971e2a`. GitHub Actions run #2165 / `44049452639` terminal SUCCESS: Windows job `133923280923`, Ubuntu job `133923281493`; both passed toolchain, docs gates, build, Clippy, full tests, artifacts, vertical slices, release CLI and applicable smoke tests. This is GitHub Actions evidence, not CircleCI. Historical run #44002493159 on predecessor `a472b3b...` remains Windows FAIL / Ubuntu PASS and is superseded; do not transfer it to current HEAD.
- PR #103 remains OPEN / DRAFT / UNMERGED, branch `docs/restore-completion-ledger-dod-20261117`, live HEAD `736c23f063f17cea9201aca374bd9a7808148f0a`.
- Independent verifier PR #42 remains OPEN / DRAFT / UNMERGED, branch `verify/windows-pr95-20261114`, verifier HEAD `56271be82645357c2feaa1134ae2dc1d2ceae734`. Never conflate verifier SHA with ONE source SHA.

### Unix process containment follow-up — PR #106 / Issue #105
- Issue #105 remains OPEN: Unix descendants can escape the original process group/session using `setsid()`, survive the invocation, and retain inherited stdout/stderr handles.
- PR #106 is OPEN / DRAFT / UNMERGED, branch `fix/unix-reader-shutdown-deadline-20261117`, exact HEAD `060ce71c3bfb272c3d90c65e95b0982bf0d69020`. It now correctly targets PR #102 branch `fix/commit-cargo-lockfile-20261117` at base SHA `ac1bb214...`. GitHub compare confirms 8 commits ahead, 0 behind, and a one-file incremental diff limited to `crates/one-core/src/interop.rs`.
- Exact-SHA GitHub Actions run #2177 / `44067989787` is terminal SUCCESS on source `060ce71...`: Windows job `133986568685` PASS; Ubuntu job `133986569118` PASS. The escaped-session regression proves reader shutdown returns promptly despite a descendant retaining inherited pipes.
- Scope boundary: PR #106 does NOT terminate the escaped descendant in production; its regression cleans up its test-owned descendant after measuring the bounded return. It closes the unbounded reader-join subproblem only. It does not close Issue #105 or establish strong Unix process-tree containment. PR #106 description now records this boundary and exact evidence. No E4/E6 or release acceptance promotion.

### Historical health-scanner verifier finding
- The stale health-scanner workflow is absent from verifier main and PR #42; it exists only on historical verifier PR #39 and is pinned to historical ONE PR #91 source `9b38c246...` through its dedicated `HEALTH_SCANNER_SOURCE_COMMIT.txt`. Its ownership is historical developer tooling; preserve PR #39 evidence and do not reintroduce scanner code into current ONE product branches or delete the historical workflow speculatively.

### Completion and safety
- PR #95, #97, #102, #103, #106 and verifier PR #42 remain unmerged. Main remains unchanged.
- E6 remains **NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED**. The current ledger/DoD checkpoint previously recorded 3 checked / 10 unchecked / 13 total; no score or checkbox is promoted by these scoped CI results.
- No merge, release, force-push, history rewrite, visibility, secrets, billing, or default-branch action was performed.

### Next executable work
1. Continue Issue #105 with a genuine non-escapable Unix containment design or explicitly narrowed platform guarantee, plus production descendant termination proof; PR #106 alone is insufficient.
2. Continue PR #95 CircleCI activation verification when authenticated CircleCI operational access is available; require exact-source Linux PASS and keep optional release-check NOT VERIFIED until actually run.
3. Reconcile the product E1–E6 evidence matrix and release acceptance against live PRs and source, preserving all exact-SHA boundaries.

---

# ONE Completion Ledger — Human Instruction vNext Roadmap

## Current continuation state — 2026-11-17 16:55 WIB (superseding live refresh)

- Main remains PUBLIC at exact SHA `d35db95153f6ddd79885494c72f5aee16c899751`; no merge/release.
- PR #95 CircleCI migration remains OPEN / DRAFT / UNMERGED at exact source `993f76351c7a119bea2e66f7d9e5bca07f392fbc`. Its branch has zero `.github/workflows/*` files, one default CircleCI Linux workflow and optional release Linux workflow; no CircleCI Windows job. All Cargo commands use `--locked`; contract checker enforces it. `Cargo.lock` is tracked on this branch.
- PR #95 exact-source Windows PASS: run #42 / `44048174334` and docs-only verifier run #43 / `44049102193`, both source `993f763...`; run #43 verifier SHA `56271be...`, job `133922119577`, terminal SUCCESS. Artifact #14313416141 ZIP SHA `4b5102dfdb1fcecc41a155f2b97fa3caf244986120c263f50bd5733888e7bbcd`; manifest/native/CLI hashes in the verification README; both PE32+ AMD64, output Budi/exit 0, CLI one 0.1.0. Current Windows gate PASS.
- PR #42 verifier branch current HEAD `56271be82645357c2feaa1134ae2dc1d2ceae734`; exact pin matches PR #95 source `993f763...`.
- PR #95 current SHA has no CircleCI status context visible in GitHub; optional release-check remains NOT VERIFIED. CircleCI Pipeline #558 belongs to PR #102 SHA `5ee645...`, not PR #95. PR #102 GHA run #44044795819 PASS while generic CircleCI Pipeline ERROR; root cause unknown without logs.
- PR #102 current source `ac1bb214bc028395367cce7edc234a1909971e2a` passed GHA run #2165 / `44049452639`, Windows job `133923280923`, Ubuntu job `133923281493`; both SUCCESS. It includes NUL/empty ProcessSpec validation and installation-guide C/C++ prerequisites; docs consistency/checker tests, build, Clippy, full tests, artifact and platform CLI smoke passed. Four new ProcessSpec tests explicitly passed on both platforms in prior exact code run #2159 `fe3e4d0...`.
- Intermediate PR #102 SHA `790dd...` failed compilation due over-escaped NUL literals; corrected in `ac2ee...` and current source `ac1bb...` passes. This intermediate failure is historical/superseded; current candidate is green.
- PR #98–#101 remain OPEN / DRAFT / UNMERGED with exact-SHA GHA PASS on Ubuntu/Windows. PR #96 is superseded historical CircleCI Windows/MSYS2 fix; do not reintroduce retired Windows job.
- PR #97 docs current HEAD `1429bcb48b61bbe81e6263399cfd4ca811feb5a3`; PR #103 ledger/DoD current HEAD `41fed006a74347e33081def4ce4aa0b720e2fb33`; PR #102 current `ac1bb...`. All remain OPEN / DRAFT / UNMERGED.
- Historical implementation score 92/100 provisional; exact-SHA verified score 0/100. DoD 3 checked / 10 unchecked / 13 total. E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED. No score/checkbox promotion.
- No merge, release, force-push, history rewrite, secrets, visibility, billing, or default-branch change.

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



**Ledger version:** 1  
**Created:** 2026-10-22  
**Branch:** `spec/human-instruction-grammar-vnext-20261022`  
**Scope:** Human-instruction vNext foundation, its first runnable reference application, and subsequent language/runtime ecosystem work.  
**Rule:** Weights are fixed in this ledger. Do not change them to make a milestone pass. Add a versioned ledger change only after explicit review of the roadmap.

## Historical exact-SHA reconciliation — 2026-10-28 (superseded by active scanner handoff below)

The live PR #80 head is the sole current source candidate; source SHA changes after each accepted source or documentation commit, so the independent verifier pin and evidence index must be synchronized to the final live head. The current scanner delta adds JavaScript/TypeScript text coverage and a `security-javascript` detector for decoded dynamic execution, network-plus-execution correlation, environment/credential-to-egress correlation, and Node `child_process.exec/execSync` use. Three positive/negative regression tests were added. Detector behavior has been exercised against isolated positive/negative snippets in this session, but the full repository test suite and self-scan have **not** produced an executable hosted result.

The latest canonical source runs inspected for the scanner candidate all completed with failure while every job returned `steps: null` and `logs_url: null`; artifacts were absent. A job-log fetch returned `BlobNotFound`. Classify this as an unresolved job-start/evidence-access blocker, not source-test failure and not pass; no root cause is established. The scanner workflow is configured to run its unit suites and scan the checked-out ONE repository, but no self-scan report has been produced by an executable job for the current candidate.

For current-candidate acceptance, E1 remains **0/10 verified**, E2 **0/8**, E3 **0/8**, E4 **0/6**, E5 **0/4**, and E6 **0/4** until required evidence is rerun and inspected on the final exact SHA. The implementation subtotal remains **92/100 provisional**; the carried-forward foundation/core arithmetic is **60/100**, not a final acceptance score. **Final exact-SHA completion score: UNKNOWN / NOT RECONCILED. Gate: NOT PASSED.** Fixed weights are unchanged. No merge or production release is authorized.

## Percentage method

The cumulative project scale is divided into three fixed tranches:

- **Milestone 30% — Foundation (30 points):** formal starter grammar, parser/semantic validation, isolated official CLI entry point, first runnable application with persistence, and evidence/docs.
- **Milestone 60% — Core language/runtime (30 additional points):** canonical AST/IR integration, general frontend/runtime path, expanded semantics, diagnostics, reusable persistence APIs, and additional real programs.
- **Milestone 100% — Ecosystem and release quality (40 additional points):** complete required standard library/tooling, conformance and compatibility suite, cross-platform verification, security/reliability hardening, release documentation and final forensic audit.

For each row, **implemented points** are provisional progress based on code actually present; **verified points** are earned only when the row's acceptance evidence passes on a recorded SHA. Implemented-but-unverified work does not count toward a PASS gate. The public completion percentage is the sum of verified points out of 100. The implementation subtotal is reported separately so unverified work is visible but cannot masquerade as completion.

## Fixed weighted ledger

| ID | Requirement / weight | Acceptance evidence | Current implementation points | Current verified points | Status |
|---|---|---|---:|---:|---|
| F1 | Versioned grammar contract, lexical/name rules, block and newline rules — **8** | Reviewed formal grammar; Unicode, identifier, ambiguity and versioning rules; positive/negative fixtures | 8/8 | 0/8 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| F2 | Deterministic parser and semantic rejection — **7** | Exact text-block tests, CRLF, blank lines, indentation, Unicode, duplicate names, unresolved references, stable diagnostics | 7/7 | 0/7 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| F3 | Explicit CLI integration isolated from v0.1 — **5** | CLI command invokes vNext parser and supported runtime; v0.1 regression tests pass | 5/5 | 0/5 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| F4 | Task-list app actions — **5** | Source example supports add/list/complete with input validation through public CLI | 5/5 | 0/5 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| F5 | Durable task persistence — **3** | Data survives a new process; corrupt storage and write failures are reported | 3/3 | 0/3 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| F6 | Beginner docs, example, exact-SHA build/CI evidence and checkpoint — **2** | Reproducible commands, docs agree with implementation, build/test/Clippy/CI green at exact SHA | 2/2 | 0/2 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| C1 | Canonical vNext AST/IR integration — **8** | Human frontend lowers into canonical IR with golden semantic tests | 8/8 | 0/8 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| C2 | General frontend-to-runtime execution pipeline — **8** | More than the task-list special-case; ordinary programs run through canonical compiler/interpreter | 8/8 | 0/8 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| C3 | Expanded semantic/type/scope model and diagnostics — **6** | Explicit type/scope rules, list/map operations, source-located diagnostics and positive/negative fixtures | 6/6 | 0/6 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| C4 | Reusable persistence abstraction and recovery policy — **4** | Versioned storage contract, corruption handling, restart, rollback and bounded-size replacement tests | 4/4 | 0/4 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| C5 | Additional real application vertical slices — **4** | Calculator, score tracker, module and mapping examples executed through the same compiler/runtime | 4/4 | 0/4 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| C6 | Milestone 60 evidence and checkpoint — **0** | Documentation-only reporting; earns no points by itself | 0/0 | 0/0 | NOT APPLICABLE |
| E1 | Required standard library and reusable APIs — **10** | Scope-defined API contracts and executable tests | 7/10 | 0/10 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| E2 | Packaging, dependency and project/build tooling for vNext — **8** | Deterministic artifacts, version metadata, supported project workflow | 7/8 | 0/8 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| E3 | Native backends and conformance across Linux, Windows, and Android AArch64 — **8** | Exact-SHA Linux/Windows builds and tests, Linux distro matrix, artifact integrity, and actual Android API 35 arm64-v8a ELF execution | 8/8 | 0/8 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| E4 | Security, resource bounds, and recovery — **6** | Threat review and tested security/resource controls within declared Linux/Windows first-class targets and Android/Linux integration boundary; broader platform expansion remains out of scope | 6/6 | 0/6 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| E5 | Documentation and developer tooling — **4** | Contract checker, documentation consistency and relative links, examples, CLI create/check/run/build/test/package smoke tests, and developer workflow instructions | 4/4 | 0/4 | IMPLEMENTED — PROVISIONAL; CURRENT SHA NOT VERIFIED |
| E6 | Final forensic audit, release evidence and checkpoint — **4** | Definition-of-Done audit at release SHA; no unresolved required gates | 0/4 | 0/4 | NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED |
| | **TOTAL** | | **92/100** | **0/100** | **NOT PASSED — CURRENT SHA NOT VERIFIED** |

## Baseline and evidence

- This ledger is the first weighted completion ledger located for the vNext workstream; no prior canonical weighted formula was found during repository discovery.
- Current source branch: `milestone/100-ecosystem-20261023`.
- Historical code-changing SHA at the initial ledger checkpoint: `26199c0114ea0da5f9784ea4d000ab185799e4fc`. The latest continuation evidence and tested source SHA are recorded in the 2026-10-24 section below.
- Historical Foundation checkpoint update commit: `d936b96283162eea3eff7b4bccec7c6abbcb5f2b`; later documentation-only commits may advance the branch head without changing the tested source snapshot.
- Current code includes the experimental parser, the dedicated task-list runner, an explicit CLI command, a runnable example, and regression tests. These changes are not credited as verified until they execute successfully.
- The GitHub Actions runs inspected for earlier commits failed before any steps started; the jobs reported an empty step list and logs were unavailable. This is an infrastructure/runner blocker, not evidence of a Rust source failure or a pass.
- Exact-code-SHA Actions run: [39752818112](https://github.com/panglimanetizen01/one/actions/runs/39752818112), triggered for `8c4f7c4706365972797446382989b81335434d92`; both OS jobs failed before any steps started. No build/test result was produced.

## Milestone gate definitions

### 30% — Foundation

PASS only when F1–F6 are fully implemented, relevant checks pass on the exact SHA, v0.1 behavior remains compatible, and all six rows earn their full **30 verified points**. An open PR or committed source alone is insufficient.

### 60% — Core language/runtime

PASS only when all F1–F6 and C1–C5 are verified (60 points total) and the checkpoint is updated.

### 100% — Ecosystem and release

PASS only when every weighted row earns its full verified points, all required CI/release gates pass on the recorded release SHA, and the final forensic audit finds no outstanding mandatory Definition-of-Done requirement.

## Milestone 60 — exact-SHA cross-platform verification (2026-10-23)

- **Canonical source branch:** `spec/human-instruction-grammar-vnext-20261022`
- **PR:** [#78](https://github.com/panglimanetizen01/one/pull/78), open and unmerged.
- **Code-bearing source SHA:** `3027e43b9e2481d73bdb67dbbbbb2e27c3419639`
- **Independent verification run:** [39936123376](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39936123376)
- **Result:** `ubuntu-latest` **SUCCESS** and `windows-2025` **SUCCESS** on the exact selected source SHA. Contract artifacts, build, AST contract regression tests, strict Clippy, workspace tests, real-network runtime tests, and platform-appropriate offline verification all completed successfully.
- Workspace test summaries include 160 core tests passed on Linux and 156 on Windows; integration suites passed. One Android AArch64 native execution test is explicitly **ignored** on each platform because the hosted environment lacks the required Android execution-capable Clang setup. That test is not counted as passed and is outside the milestone-60 required gate.
- Canonical repository Actions for the same SHA ([run 39936118585](https://github.com/panglimanetizen01/one/actions/runs/39936118585)) reported failure with `runner_id=0` and zero steps on both OS jobs. The repository's [exact-SHA verification contract](human-vnext-exact-sha-verification-contract.md) authorizes the independent mirror fallback; the canonical run is not misreported as green.
- **Implementation subtotal:** 60/100. **Verified completion:** 60/100. F1–F6 contribute 30 verified points and C1–C5 contribute the next 30, using unchanged ledger weights.
- Integrated scope: explicit opt-in human-core parser/CLI through canonical parse → semantic analysis → IR → interpreter; source-located semantic diagnostics; functions, control flow, lists and maps; relative multi-file modules with duplicate/cycle/path traversal rejection; reusable bounded `ONE-KV-1` persistence; and executable calculator, score-tracker, module, and mapping examples.
- Compatibility: experimental mapping grammar is gated behind `parse_human_core` and the explicit human-core CLI; default v0.1 parsing does not accept the new mapping actions. The existing task-list app's persistence format was not silently migrated.
- **Gate:** **PASS GREEN LOCKED — milestone 60% only.** No claim is made that milestone 100% or the full ONE ecosystem is complete.

## Current result

- **Security hardening added after the previous checkpoint:** HTTP responses are read with a 16 MiB body limit and a 64 KiB header allowance; HTTP headers are limited to 256 fields, duplicate Content-Length and Content-Length/Transfer-Encoding conflicts are rejected, and unsupported Transfer-Encoding is rejected. Filesystem adapters limit reads/writes to 16 MiB and reject parent-directory components and existing symlink path components. These changes are defensive bounds, not a complete sandbox; race-resistant path resolution and the broader threat review remain open.
- **SHA-256 module lock v2:** module content digests now use the sha2 crate and known-vector tests. This improves content mismatch detection against a trusted lock, but does not provide signed package identity or authenticated provenance. E2 remains partial.


- **Implementation progress:** 84/100 provisional points (60 core/Foundation points plus 7 provisional E1, 5 provisional E2, 8 provisional E3, 3 provisional E4, and 1 provisional E5 implementation points).
- **Verified completion:** 60/100 points. E3's 8 implementation points are not yet verified: the Linux/Windows matrix passes, but the declared Android AArch64 ELF build-and-execute test remains ignored because Android AArch64 Clang/runtime setup is unavailable. The prior 68/100 verified subtotal overstated E3 acceptance and is superseded by this forensic correction.
- **Milestone target:** 100%.
- **Gate:** **NOT PASSED** for milestone 100%. The prior 60% gate remains scope-limited to code SHA `3027e43b9e2481d73bdb67dbbbbb2e27c3419639`.
- **E1 partial work:** public deterministic text/list/map/integer/JSON helper APIs, UTC formatting and strict UTC parsing are integrated through source-level built-ins. Filesystem read/write adapters enforce 16 MiB bounds and reject parent-directory and existing symlink components; HTTP GET uses bounded response reads and strict response framing. JSON floating-point/exponent parsing remains unsupported because ONE has no float value type. Broader time/date, concurrency, logging/configuration, external dependency resolution, and complete persistence integration remain incomplete. E1 verified points remain 0/10.
- **E2 partial work:** `one build`, `one test`, and `one package` process human-core projects through the canonical compiler path; package emits a deterministic versioned artifact and module lock v2 with SHA-256 content digests. Build/test reject stale locks with `ETOOL006`. The lock is not signed and does not authenticate publisher identity or provenance. External dependency declarations/registry resolution, signed package identity, authenticated provenance, target-specific artifacts, and full provenance remain incomplete; E2 verified points remain 0/8.
- **E3 implementation; verification incomplete:** valid/invalid human-core conformance fixtures are dispatched through the matching compiler path. Exact-SHA Linux/Windows workflows pass, but the declared Android AArch64 ELF build-and-execute test `build_native_menghasilkan_dan_menjalankan_elf_android_aarch64` is marked ignored on both platforms. The Android-target CLI test only verifies artifact creation; it does not execute the Android ELF. Therefore E3 remains 8/8 implemented but 0/8 verified until the Android execution acceptance is satisfied or the contract explicitly excludes it.
- **E4 partial work:** checked integer arithmetic, recursion-depth bounds, bounded JSON parsing/serialization, HTTP response/body/header/count limits, strict HTTP framing, filesystem read/write bounds, and SQLite read-only/query-only enforcement with SQL/parameter budgets are implemented with regression tests. FTP/SMTP protocol line reads retry `Interrupted` system calls. Filesystem adapters still lack race-resistant path resolution; general memory/open-resource quotas, comprehensive threat review, and broader recovery remain incomplete. E4 verified points remain 0/6.
- **E5 partial work:** standard-library/runtime/package contracts, runnable examples, and CLI integration tests exist. Complete tutorials, API reference coverage, debugging workflow, and end-to-end developer workflow review remain incomplete; E5 verified points remain 0/4.
- **E6:** final forensic audit and release evidence have not been completed; 0/4.
- **Latest exact-SHA verification:** source snapshot `ebf254d1194a903a3b582f3a18abfc402f2003d2` passed [focused run 40009389100](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389100), [Linux distro run 40009389164](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389164), and [Ubuntu/Windows cross-platform run 40009389190](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389190). The cross-platform run checked out the exact SHA, passed build/Clippy/workspace/real-network/offline steps, had no `test result: FAILED` markers, and logged one `OFFLINE VERIFICATION: PASS` per platform. The separate Android AArch64 ELF build-and-execute test remains ignored; it is not counted as passed. Canonical repository Actions on this SHA still fail before steps (`runner_id=0`, empty step lists), so the mirror supplies the actual test evidence.
- **Historical verification:** mirror run [39958882035](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39958882035) passed its Linux/Windows matrix on source SHA `d9d696fa87683027b78fdcbbec504696904055aa`; it predates the current security and SHA-256 lock changes. Run [39963120976](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39963120976) targets the earlier source SHA `3c0febb47409f0af6c43c6a46996c784bcd8a0a9`, before the SHA-256 module-lock implementation. Exact-SHA mirror run [39963778066](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39963778066) checked out `d74eda614be6688164dbd294fa07bcdf9c204ecf`. Ubuntu build, AST contract tests, and strict Clippy passed, but workspace tests failed: `native_backend::linker_discovery_tests::linux_x86_64_linker_override_diprioritaskan` tried to execute shell scripts in the temporary directory, which is mounted `noexec` on this runner (181 passed, 1 failed). The test fixture was corrected in commit `8ee1b34dd0bba1955840581c5932135ae32b76a9` to symlink `/bin/true` and `/bin/false`, preserving the override-priority assertion without depending on temp-directory execution. The follow-up exact-SHA mirror run [39964166884](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39964166884) completed successfully on canonical source SHA `985bce88498daa0bbb6566226353251fe6e3c4ac`: Linux and Windows build, AST contract regression tests, strict Clippy, workspace tests, real-network tests, and their platform-specific offline verification all passed. This verifies the security/package code at that SHA. The subsequent exact-SHA mirror run [39964952853](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39964952853) checked out documentation-reconciled SHA `e6ed974bcb19f6cf659508b9ac905bcfe8f8fed1` and again failed the same linker override test because the expected executable was still a symlink located inside the noexec temporary directory. The follow-up fix in commit `ab09f8c6a8dc5bf16d39f3fc8f1df15248d77749` moved the test executable outside temp, but run [39965400077](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39965400077) still failed because the Rust test executable did not exit successfully for the probe's --version argument. Commit `e14b63ba0e40e45eb8a0c0f79a78886d0a83bdf1` now uses `/bin/true` directly as the Linux override and `/bin/false` as the PATH candidate, with the test scoped to Linux; neither executable is run from the noexec temp path. A fresh exact-SHA Linux/Windows matrix is required on the updated branch head. No additional verified milestone points are awarded because the full E1–E6 acceptance criteria remain incomplete.
- **Not claimed complete:** ONE is not 100% complete. No ledger weights have changed.

## Repository work checkpoint — 2026-10-22

Changes committed directly to the feature branch after the baseline above:

- `c31258c98932293203507f0566c258d519632668` — added an implementation-bounded human-vNext starter grammar contract.
- `c73950e16acb1df32be32b4f52898fe001ca878c` — added parser tests for Unicode identifiers, invalid indentation, duplicate data/action names, and orphaned nested instructions.
- `212a48ac37522d16c76cb525f043aed6adf26db3` — aligned the text-block contract with observed parser behavior (empty text is accepted).
- `e364af0b5768628f40e3f3de5c52fd3f0833607d` — linked the experimental contract from the specification index.
- `48afad1a35f4911e3a415de3fdfc1463091a8aaf` — made the contract checker require the new grammar document.

- `f7ca5c5d35a59b6928de20fe722e308a7f11fb19` — applied the four strict-Clippy fixes to the GitHub source.
- `7f02a4563024cb60e37d95bf3e2639ddffa8a4f3` — added runtime tests for blank-title rejection, invalid/unknown task IDs, and a storage write failure.

## Exact-SHA local verification — 2026-10-23

Verified in Debian 12/Termux against the clean GitHub worktree at commit `217815f1e16771b730d69f752c64018fd2faa250` (branch `spec/human-instruction-grammar-vnext-20261022`):

- Contract checker: PASS — 61 required artifacts, 73 indexed artifacts, 6 valid fixtures, 6 invalid fixtures, 3 golden fixtures.
- `git diff --check`: PASS.
- `cargo build --workspace --all-targets`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `ONE_ANDROID_CLANG=/tmp/one-android-clang-wrapper cargo test --workspace --all-targets`: PASS for every executed test suite, including 135 one-core unit tests and 16 CLI tests. One Android AArch64 native execution test was ignored because the required Android AArch64 Clang environment was unavailable.
- Human-vNext regression coverage includes Unicode names/text, indentation and text-block boundaries, declaration binding, duplicate names/persistence, restart persistence, blank-title rejection, invalid/unknown task IDs, corrupt storage, and a parent-path write failure.
- GitHub Actions run [39814473056](https://github.com/panglimanetizen01/one/actions/runs/39814473056) still failed before steps began on both Ubuntu and Windows; no hosted build/test evidence was produced.

**Evidence boundary:** local verification is tied to the exact tested commit above. It does not turn the hosted CI failure into a pass. Verified-point assignments remain unchanged pending a reviewed replacement verification contract and completion of every F1–F6 acceptance criterion. The 30% gate remains NOT PASSED.


## Exact-SHA local verification — 2026-10-23 (fixture checkpoint)

Verified in Debian 12 from the fetched GitHub branch at exact commit `742d30c506eabde0eb9c1ccb258eeb2b2b9d9b36`:

- `python3 tools/check_contract.py`: PASS — 68 required artifacts, 73 indexed concrete artifacts, 6 existing valid fixtures, 6 existing invalid fixtures, 3 golden fixtures, 3 human-vNext valid fixtures, and 4 human-vNext invalid fixtures.
- `git diff --check`: PASS.
- `cargo build --workspace --all-targets`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `ONE_ANDROID_CLANG=/tmp/one-android-clang-wrapper cargo test --workspace --all-targets`: PASS for all executed suites. CLI: 16 passed; one-core unit tests: 137 passed; conformance: 5 passed; remaining displayed suites passed. One Android AArch64 native execution test was ignored because the Android AArch64 Clang verification environment was unavailable.
- New human-vNext fixture tests passed for all 3 valid and 4 invalid fixtures.
- GitHub Actions has not been shown passing for this SHA; this evidence is local Debian verification only.

**Evidence boundary:** this confirms the fixture checkpoint's local contract/build/Clippy/test checks, not hosted CI. It does not alone satisfy F1–F6. Verified points and the 30% gate remain unchanged until every acceptance criterion and the reviewed CI/replacement-verification contract are satisfied.

## Exact-SHA local verification — 2026-10-23 (expanded fixture checkpoint)

Verified in Debian 12/Termux from a clean worktree fetched from GitHub at exact commit `d76360b661a16caceb840d2a1eba8c7beb60882f` on branch `spec/human-instruction-grammar-vnext-20261022`:

- `python3 tools/check_contract.py`: PASS — 75 required artifacts, 73 indexed concrete artifacts, 6 existing valid fixtures, 6 existing invalid fixtures, 3 golden fixtures, 4 human-vNext valid fixtures, and 10 human-vNext invalid fixtures.
- `git diff --check`: PASS.
- `cargo build --workspace --all-targets`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `ONE_ANDROID_CLANG=/tmp/one-android-clang-wrapper cargo test --workspace --all-targets`: all executed test suites passed. CLI: 16 passed; one-core unit tests: 137 passed; conformance: 5 passed; other displayed suites passed. One Android AArch64 native execution test was ignored because the Android AArch64 Clang verification environment was unavailable.
- The expanded human-vNext fixture tests passed for all 4 valid and 10 invalid fixtures.
- GitHub Actions is not confirmed passing at this SHA. This is local verification only; earlier Actions runs failed before steps began.

**Evidence boundary:** this checkpoint provides exact-SHA local contract/build/Clippy/test evidence, but does not by itself satisfy all F1–F6 acceptance criteria or substitute for hosted CI. Verified points remain unchanged pending the remaining acceptance work and an explicitly reviewed CI/replacement-verification contract. The 30% gate remains NOT PASSED.

## Exact-SHA local verification — 2026-10-23 (grammar clarification checkpoint)

Verified by the user in Debian 12/Termux from the clean GitHub branch worktree at code commit `89b6ace035450511496764510867d3bd82cc109d`:

- `python3 tools/check_contract.py`: PASS — 75 required artifacts, 73 indexed concrete artifacts, 6 existing valid fixtures, 6 existing invalid fixtures, 3 golden fixtures, 4 human-vNext valid fixtures, and 10 human-vNext invalid fixtures.
- `git diff --check`: PASS.
- `cargo build --workspace --all-targets`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `ONE_ANDROID_CLANG=/tmp/one-android-clang-wrapper cargo test --workspace --all-targets`: all executed suites passed, including 16 CLI tests, 137 one-core unit tests, 5 conformance tests, and the remaining displayed integration suites. One Android AArch64 native execution test was ignored because the Android AArch64 Clang environment was unavailable.
- Human-vNext fixture suite passed for all 4 valid and 10 invalid fixtures.

**Evidence boundary:** this is user-run local verification, not hosted GitHub Actions evidence. The prior Actions failures before any steps started remain unresolved. No verified points are assigned by this entry; F1–F6 and the 30% gate remain NOT PASSED until the replacement verification contract is explicitly reviewed and each acceptance criterion is satisfied.



## Exact-SHA cross-platform verification — 2026-10-23 (30% Foundation gate)

Verified canonical source SHA `26199c0114ea0da5f9784ea4d000ab185799e4fc` on branch `spec/human-instruction-grammar-vnext-20261022` through the independent free verification mirror, pinned by `CANONICAL_COMMIT.txt` in commit `d03eeecbae523237c66aa553aebfcda68036df5b`:

- Workflow: [One Cross-Platform Verification run 39929244544](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39929244544) — **overall SUCCESS**.
- `ubuntu-latest`: **SUCCESS** — contract artifacts, build, AST contract regression tests, strict Clippy, all workspace tests, real-network runtime tests, and offline verification all passed.
- `windows-2025`: **SUCCESS** — contract artifacts, build, AST contract regression tests, strict Clippy, all workspace tests, real-network runtime tests, and Windows offline verification all passed.
- Both jobs checked out and printed the same canonical source SHA `26199c0114ea0da5f9784ea4d000ab185799e4fc`.
- Human-vNext fixture inventory at this SHA: 6 valid and 18 invalid fixtures; the contract checker passed (91 required artifacts and 77 indexed concrete artifacts).
- CLI tests: 22 passed on both platforms. Core tests: 153 passed on Ubuntu and 149 passed on Windows. All other displayed test suites passed.
- One Android AArch64 native execution test remained explicitly ignored on both platforms because the test requires an Android execution-capable verification environment. It is not claimed as passed; this does not block the scoped Foundation milestone.
- The canonical ONE repository's own Actions still is not claimed as passing; the independent mirror is the documented cross-platform replacement, and its Linux/Windows matrix actually executed successfully on the exact source SHA.

### F1–F6 acceptance review at the verified SHA

- **F1 — 8/8:** reviewed versioned starter grammar, Unicode/name and binding rules, structural indentation and newline/text-block rules, explicit compatibility/versioning boundaries, and required positive/negative fixtures.
- **F2 — 7/7:** deterministic parser and semantic rejection tests cover opaque text-block fidelity, LF/CRLF, blank lines, Unicode, canonical indentation, duplicate data/action and persistence names, unresolved references, and stable diagnostics.
- **F3 — 5/5:** explicit `jalankan-human-vnext` CLI path; invalid/missing argument diagnostics and unsupported task-app contracts are tested; default v0.1 does not auto-detect human-vNext and regression tests pass.
- **F4 — 5/5:** the documented runnable task-list example adds, lists, and completes tasks through the public CLI with title and task-ID validation.
- **F5 — 3/3:** task data survives a new process; corrupt records, write/replace failures, temporary-file cleanup, invalid titles/IDs, and atomic replacement behavior have tests.
- **F6 — 2/2:** beginner documentation, runnable example, contract checker, exact-SHA verifier, reviewed fallback contract, and successful exact-SHA Linux/Windows hosted evidence are recorded above.

**Gate decision:** award 30 Foundation points plus 30 Core/Runtime points. The 60% target is reached at the exact code SHA recorded above without changing weights or counting the ignored Android-native test as passed. E1–E6 remain unimplemented and uncredited.


## 2026-10-24 continuation — exact-SHA hardening and verification

**Tested source SHA:** `64985b51599c1531b6b8e07aa0aa9d16551e6e38`  
**Latest code-changing commit in this slice:** `cec949631f3c91a5419957b1a8820904daa6bcd7`; the tested SHA above adds verification-contract documentation after that code commit.  
**Branch / PR:** `milestone/100-ecosystem-20261023` / [PR #80](https://github.com/panglimanetizen01/one/pull/80), still open and draft; no merge performed.

### Implemented and integrated

- SQLite `database.query` now opens file-backed databases with `SQLITE_OPEN_READ_ONLY` and enables `PRAGMA query_only`. The `:memory:` resource remains an isolated ephemeral connection. A regression test verifies a read query against a missing file returns a backend error and does not create that file.
- SQLite query resource limits are explicit and enforced before prepare/bind work: SQL statement 1 MiB, each text parameter 1 MiB, aggregate parameter bytes 16 MiB, parameter count 256, result rows 100,000, serialized output 16 MiB. Regression tests cover SQL length, individual parameter length, aggregate parameter bytes, and parameter count.
- Added `utc_text_to_unix_seconds` and the source-level builtin `unix dari waktu utc`, integrated through semantic type/arity validation and the canonical interpreter. The parser accepts only Gregorian UTC `YYYY-MM-DDTHH:MM:SSZ` for years 0000–9999 and rejects invalid dates/times, offsets, fractional seconds, leap seconds, and non-canonical forms. Unit and parse→semantic→IR→interpreter tests cover valid boundaries, leap dates, pre-epoch values, invalid types and malformed dates.
- Updated the standard-library contract, standard-library boundary document, user guide, and focused verification workflow to document and exercise the new behavior.
- Fixed a false-green in `tools/verify_offline.ps1`: it now enables `$PSNativeCommandUseErrorActionPreference = $true` so failed native commands terminate the script, and skips only `android_target_explicit_mempertahankan_build_native` on Windows. The Android-target CLI build test passed on Ubuntu but only checks artifact creation; the separate Android AArch64 ELF build-and-execute test remains ignored on both platforms. No Android runtime pass is claimed.

### Exact-SHA evidence

All three independent verification workflows checked out the exact source SHA above from the canonical repository:

- Focused standard-library/database regressions: [run 40006470057](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470057) — **SUCCESS**. UTC formatter/parser unit and integration tests, JSON output boundary, SQLite SQL/parameter resource-limit tests, missing-file read-only regression, and strict one-core Clippy passed.
- Linux distro matrix: [run 40006470246](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470246) — **SUCCESS** on Ubuntu 24.04 and Debian 12; contract checks, build, Clippy, workspace tests, and real-network tests passed.
- Cross-platform matrix: [run 40006470017](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40006470017) — **SUCCESS** on both `ubuntu-latest` and `windows-2025`. Both jobs passed contract artifacts, build, AST contract tests, Clippy, workspace tests, real-network runtime tests, and platform-appropriate offline verification. Main workspace suite summaries show 215 passed / 0 failed on Ubuntu and 210 passed / 0 failed on Windows; a separate integration suite has an ignored Android AArch64 ELF build-and-execute test on both platforms. The Android-target CLI build test passed on Ubuntu but only checks artifact creation. The Windows-only CLI suite filtered out its Android-target test because the Windows runner is not an Android toolchain. No Android ELF runtime pass is claimed.
- The source repository's own Actions on this same SHA remain infrastructure failures: [CI run 40006462899](https://github.com/panglimanetizen01/one/actions/runs/40006462899), [Linux distribution run 40006463227](https://github.com/panglimanetizen01/one/actions/runs/40006463227), and [focused run 40006463315](https://github.com/panglimanetizen01/one/actions/runs/40006463315) failed with `runner_id=0` and empty step lists. They produced no source build/test result. The documented independent mirror was used instead; no paid plan upgrade or fabricated CI result was used.

### Forensic correction

The earlier mirror run [40005692552](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40005692552) was initially marked successful, but its Windows offline-script log contained a failed Android-target native CLI test followed by `OFFLINE VERIFICATION: PASS`. That run is **not credited as valid full verification**. The failure-propagation and platform-specific skip fix was committed, and the exact-SHA cross-platform run 40006470017 was then rerun successfully. Its Windows offline log has no `test result: FAILED` marker and ends with `OFFLINE VERIFICATION: PASS`.

### Updated point accounting

- E1 implementation: **7/10** provisional; verified remains **0/10** because the full required standard-library acceptance set is incomplete.
- E4 implementation: **3/6** provisional; verified remains **0/6** because race-resistant filesystem resolution, broader quotas, and full threat review remain incomplete.
- E2 remains 5/8; E3 remains 8/8 implemented but 0/8 verified until Android AArch64 ELF build-and-execute acceptance is satisfied; E5 remains 1/4; E6 remains 0/4.
- **Implementation subtotal: 84/100. Verified completion: 60/100.** Fixed row weights are unchanged. The 24-point implementation/verification gap does not count toward a PASS gate.

**Remaining mandatory work:** external dependency resolution and authenticated package provenance; broader standard-library coverage; race-resistant filesystem confinement and wider resource quotas; compatibility/release review; full developer-tooling/docs acceptance; Android AArch64 ELF build-and-execute verification in a toolchain-capable environment if required by the declared target contract; and the E6 forensic audit.  
**Gate:** **NOT PASSED — Milestone 100% remains in progress.**

**Forensic scoring correction (2026-10-24):** The earlier 68/100 verified subtotal credited E3's 8 points based on Linux/Windows matrix results and an Android-target CLI build check. Inspection of `crates/one-core/tests/native_backend_android_aarch64.rs` shows the separate Android ELF build-and-execute test is `#[ignore]`; the CLI check only asserts that an artifact file is created. Since ignored tests are not passes and Android AArch64 is a declared target, E3's verified points are reset from 8/8 to 0/8 until the runtime acceptance is satisfied. This does not change E3's 8 implementation points or any row weight; verified completion is now 60/100.


## Post-checkpoint exact-SHA confirmation — `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974`

The documentation-only checkpoint snapshot `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974` was also checked out and verified exactly:

- [Focused stdlib/database run 40007463554](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463554) — SUCCESS.
- [Linux distro run 40007463563](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463563) — Ubuntu 24.04 and Debian 12 SUCCESS.
- [Cross-platform run 40007463527](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40007463527) — Ubuntu and Windows SUCCESS. Logs verify exact source SHA `c21ba26f4f7cac7a50f183b650ecc86a6e8c4974`, no `test result: FAILED` markers, and one clean `OFFLINE VERIFICATION: PASS` per platform.
- Main workspace logs show 215 passed / 0 failed on Ubuntu and 210 passed / 0 failed on Windows. The Android-target CLI build test passed on Ubuntu (logged as `test android_target_explicit_mempertahankan_build_native ... ok`), but the separate `build_native_menghasilkan_dan_menjalankan_elf_android_aarch64` test remains ignored on both platforms. The Windows-only CLI variant is filtered out on Windows; no Android ELF runtime pass is claimed.
- Canonical repository workflows on the same source SHA still failed before steps with `runner_id=0`; independent mirror runs provide the actual test evidence.

The later ledger/checkpoint corrections are documentation-only and do not change the tested code snapshot. **Gate remains NOT PASSED** because the full E1/E2/E4/E5/E6 Definition of Done is not complete.


## Final exact-SHA run set — `ebf254d1194a903a3b582f3a18abfc402f2003d2`

- Focused stdlib/database [40009389100](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389100) — SUCCESS (25 successful steps).
- Linux distro [40009389164](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389164) — SUCCESS on Ubuntu 24.04 and Debian 12.
- Cross-platform [40009389190](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40009389190) — SUCCESS on Ubuntu and Windows; exact source SHA confirmed in both logs. Both jobs passed build, Clippy, workspace tests, real-network tests and offline verification. Logs contain zero `test result: FAILED` markers and one `OFFLINE VERIFICATION: PASS` per platform.
- Android AArch64 ELF build-and-execute remains ignored on both platforms; the Android-target CLI check verifies artifact creation only. Therefore E3 stays 8/8 implemented, 0/8 verified. Implementation is 84/100, verified completion 60/100, gate NOT PASSED.
- The subsequent ledger/checkpoint edit is documentation-only; source implementation remains the tested snapshot.

## 2026-10-24 continuation — Android runtime gate and filesystem no-follow hardening

- **Code-bearing source SHA:** `b5c3827840a691868a69428ae4032614407f1062` (latest source branch head before this ledger/checkpoint update).
- **Git state at implementation checkpoint:** branch `milestone/100-ecosystem-20261023`; PR #80 remains open, draft, unmerged, base `main`. No merge or direct-main write was performed.
- **E3 — Android AArch64 runtime:** strengthened `crates/one-core/tests/native_backend_android_aarch64.rs` to assert ELF magic, ELFCLASS64, little-endian encoding, and `EM_AARCH64` before pushing the artifact to Android and executing it. Added a dedicated Android API 35 arm64 emulator job to canonical CI and the independent verification mirror. The job explicitly runs the previously ignored build-and-execute acceptance test; a build-only pass does not count. **Runtime result on this SHA: NOT YET VERIFIED.**
- **E4 — filesystem race-sensitive resolution:** added direct `libc` dependency and Linux/Android descriptor-relative directory traversal using `openat`, `O_DIRECTORY`, and `O_NOFOLLOW` for each parent component and final file opens. Read/write targets are checked as regular files after opening. Added regression tests for symlinked parent directories, final-component read symlinks, and final-component write symlinks preserving their referent. Updated `docs/SECURITY.md` to state the guarantee and boundaries accurately.
- **E4 boundary remains open:** Windows/macOS do not yet have equivalent descriptor-relative race-resistant opens; the capability contract still lacks a root confinement directory. Do not claim a complete cross-platform filesystem sandbox.
- **E2 regression:** added a CLI test ensuring unsupported dependency declarations fail closed instead of being silently ignored. This does **not** implement external registry resolution, a committed Cargo lockfile, signed publisher identity, or authenticated provenance; E2 remains partial.
- **Verification:** canonical source Actions on prior source SHA `6361461a81718594c78921abb3106b1fdf86e1c5` failed before any steps started; all inspected jobs returned empty step lists. This is a runner provisioning failure, not a source-test result. The independent mirror selector must be pinned to the final checkpoint SHA after this documentation update. No new code verification points are awarded yet.
- **Score reconciliation:** E4 implementation increases from 3/6 to 4/6 because descriptor-relative no-follow opens are now connected to the Linux/Android file adapters and regression tests exist in source. **Implementation: 85/100 provisional. Verified: 60/100.** E1 0/10, E2 0/8, E3 0/8, E4 0/6, E5 0/4, E6 0/4 verified points remain unchanged under full milestone-100 acceptance.
- **Remaining E1–E6:** full standard-library contract breadth; external dependency/registry resolution and authenticated package provenance; actual Android AArch64 ELF execution; Windows/macOS race-resistant filesystem handling and defined confinement root; broader resource quotas/threat review; release/compatibility/tooling acceptance; final exact-SHA forensic audit.
- **GATE: NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — committed Cargo lockfile and locked verification

- **Code-bearing source SHA before this documentation update:** d9196cf97c4f42de8a8dbf8e1de93ced5b8db4dd.
- **Lockfile generation evidence:** independent workflow [ONE Cargo Lockfile Bootstrap Evidence](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40052223860) successfully ran Cargo lockfile generation against canonical source SHA cd4aa9f9be450ecc43a0419d057867967d23e469. Cargo.lock SHA-256: 84b2111b5d1ab0c942c6fc68040285bccc3065a5f4e4150814e202c0fe15bc21; 113 package entries.
- Added the generated root Cargo.lock to the canonical repository and removed Cargo.lock from .gitignore. Updated canonical CI, Linux distro conformance, focused standard-library verification, and local verification scripts to invoke Cargo build/test/run/Clippy with --locked.
- Added the lockfile policy to docs/verification/human-vnext-exact-sha-verification-contract.md. The policy explicitly distinguishes Rust workspace dependency locking from One-language registry resolution and publisher authentication.
- **E2 remaining:** no One-language external dependency graph/registry resolver, version-conflict resolution, publisher trust store, digital signature verification, or authenticated provenance envelope. The lockfile change closes only Cargo workspace dependency determinism.
- **Verification on the current lockfile-bearing source SHA:** NOT YET VERIFIED. The mirror must pin the final checkpoint SHA and confirm --locked succeeds on Ubuntu and Windows and the distro/stdlib jobs complete.
- **Score reconciliation:** E2 implementation increases 5/8 → 6/8 because the Cargo lockfile is committed and the repository's verification commands now enforce it. Implementation subtotal: **86/100 provisional**. Verified subtotal remains **60/100**; no verification credit is assigned for the lockfile until exact-SHA checks pass.
- **GATE: NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — toolchain reproducibility and workflow pin

- Root rust-toolchain.toml now pins Rust 1.99.0 with rustfmt and Clippy. This version was observed in the successful lockfile-generation log; the previous moving stable channel resolved to 1.99.1 in a verification job, so relying on stable alone was not deterministic.
- Canonical CI, Linux distro conformance, and focused stdlib workflows now explicitly request Rust 1.99.0 from dtolnay/rust-toolchain. The independent mirror's Android, distro, and focused stdlib workflows now do the same. The mirror cross-platform workflow uses the source root rust-toolchain.toml and now invokes Cargo commands with --locked.
- Source commits containing the lockfile and toolchain pin are followed by workflow changes; the current source branch head must be re-read and mirrored before any release-gate credit.
- **E5 implementation score:** 1/4 → 2/4 because the toolchain pin and documented reproducibility contract are now present alongside the committed lockfile policy. Full compatibility, release packaging and developer workflow review remain incomplete.
- **Current implementation subtotal: 87/100 provisional. Verified subtotal: 60/100.**
- **GATE: NOT PASSED — exact-SHA verification for the lockfile/toolchain workflow revisions is still required.**

## 2026-10-24 continuation — exact-SHA lock/toolchain verification and workflow repair

- **Current code/workflow source SHA before this ledger update:** 34f2162918eaff316f68659b51d5666333160a29.
- **Cargo.lock:** committed root lockfile has 113 package entries and SHA-256 84b2111b5d1ab0c942c6fc68040285bccc3065a5f4e4150814e202c0fe15bc21. The independent generator workflow succeeded against source SHA cd4aa9f9be450ecc43a0419d057867967d23e469.
- **Toolchain drift was found and fixed:** an exact-source focused run initially resolved moving stable to Rust 1.99.1. Root rust-toolchain.toml and explicit workflow inputs now pin Rust 1.99.0. The updated independent focused stdlib/database workflow passed on source SHA c740982ef40b680ac8ae8682e738272ffad05331; its log reports rustc 1.99.0, and 28 Cargo commands include --locked. This result is valid for that SHA, not automatically for the current source SHA.
- **Filesystem regression:** on an earlier exact SHA, workspace tests exposed malformed NUL-terminated root literals in the new absolute-path resolver. This was corrected, then strict Clippy exposed manual C-string literals; those were corrected to Rust C-string literals. The focused lint/test workflow passed after the path correction on the source snapshot before the lockfile/toolchain revisions. The final snapshot still requires a fresh full test run.
- **Mirror workflow repair:** explicit Rust toolchain configuration initially introduced duplicate YAML with blocks in the Linux distro workflow. The malformed workflow runs had no jobs; the duplicate blocks were merged and a new run was queued. Do not credit those parse failures as source-test failures or passes.
- **Latest exact-source mirror jobs:** source SHA c740982ef40b680ac8ae8682e738272ffad05331 is undergoing Ubuntu/Debian and Windows/Linux cross-platform verification; the focused job passed. Android arm64 runtime job is still installing its SDK/NDK/system image and has not reached ELF execution.
- **Canonical Actions:** inspected jobs on source SHA f93a960635d648d6b9390bef42ed42328bf2ddec failed before any steps started (empty step lists). This remains a runner-provisioning failure, not source-test evidence.
- **E3:** the dedicated Android workflow now explicitly runs the ignored acceptance test and asserts the emulator ABI is arm64-v8a. The workflow has not yet produced an ELF execution result; E3 verified points remain zero.
- **Score:** implementation remains **87/100 provisional** and verified remains **60/100**. No verified points were added by the current work. E1/E2/E3/E4/E5/E6 acceptance is not complete.
- **GATE: NOT PASSED — milestone 100% remains in progress.**

## 2026-10-24 continuation — full Linux acceptance and exact-SHA script closure

- **Code/workflow source SHA before this ledger update:** 9bf679bb52810031a533025cbc5a18e97450d63b.
- The release-check workflow and tools/verify_exact_sha.sh were also updated to use Cargo --locked. A recursive audit of all workflow/shell/PowerShell files found no remaining unlocked Cargo build/test/run/Clippy command; the sole non-locked match is cargo clippy --version, which does not resolve dependencies.
- **Exact-SHA focused standard-library/database job passed** on source SHA 102741eddac431cfe4f1b4b24bec3d7a2032428b. Log reports rustc 1.99.0 and 28 --locked Cargo invocations.
- **Exact-SHA Linux distro job passed** on source SHA 102741eddac431cfe4f1b4b24bec3d7a2032428b on both Ubuntu 24.04 and Debian 12. Each job reported Rust 1.99.0, seven --locked Cargo commands, and 64 successful test-result groups; the logs include successful filesystem no-follow regression tests and the fail-closed unsupported-dependency test.
- These passes are evidence for SHA 102741, not for the newer source SHA 9bf679 containing the release-check/exact-SHA script updates. A final mirror run must target the post-checkpoint SHA.
- **Current Windows cross-platform job:** prior run on SHA 102741 was cancelled by a later mirror-workflow push after the Windows test steps had completed and before cleanup finished. It is not credited as a complete run; rerun required.
- **Android runtime:** exact-SHA Android arm64 emulator job has installed the SDK/NDK/system image but has not reached the ELF execution step. The old unbounded boot step is still running; a newer workflow revision now bounds adb wait/boot and uploads emulator logs on failure.
- **Score remains 87/100 provisional implementation, 60/100 verified.** No new verified points are awarded from the partial/cancelled matrix.
- **GATE: NOT PASSED — exact-SHA cross-platform and Android runtime results remain pending.**

## 2026-10-24 continuation — bounded package-provenance inputs

- Added bounded file reads to package signing and verification: artifact <=64 MiB, private-key seed file <=256 bytes, provenance sidecar <=16 KiB, and publisher trust store <=1 MiB. The bounded-read regression rejects inputs above the configured limit.
- Implementation score: E4 4/6 -> 5/6 (+1) because the new bounds are integrated into the signing/verification path with a regression test. Overall implementation subtotal 87/100 -> 88/100. Verified subtotal remains 60/100; the full workspace test matrix for the latest source SHA is still running.
- Exact source SHA 0a0f250dfc2e556bca40f2201752ee5d06f56854: focused standard-library/database verification passed using Rust 1.99.0 and 28 --locked Cargo commands. Linux distro verification passed on Ubuntu 24.04 and Debian 12; each reports Rust 1.99.0, seven --locked commands, and 64 successful test-result groups.
- Current exact-SHA Windows cross-platform and Android arm64 emulator runs are still in progress. Do not credit E3 verified points until Windows and the actual Android ELF execution acceptance both complete.
- E2 correction: artifact signing/trusted publisher verification is implemented, but the external One package registry, dependency graph/version conflict resolution, automatic package signing, and complete build provenance remain unimplemented.
- GATE: NOT PASSED — mandatory E1-E6 acceptance is incomplete.


## 2026-10-24 continuation — Android SDK license handling and latest-SHA evidence audit

- **Code/workflow SHA:** `4291a820e07b99820f1789f344bc2ab427f7f9d5` ([commit](https://github.com/panglimanetizen01/one/commit/4291a820e07b99820f1789f344bc2ab427f7f9d5)).
- Corrected the Android emulator job's SDK license pipeline. The previous `yes | sdkmanager --licenses || true` masked genuine SDK-manager failures. The replacement tolerates the expected SIGPIPE from `yes` while explicitly checking `sdkmanager`'s exit status and failing closed when it is nonzero.
- Exact-SHA GitHub Actions runs inspected for this commit: One CI [40060363429](https://github.com/panglimanetizen01/one/actions/runs/40060363429), focused standard-library verification [40060363449](https://github.com/panglimanetizen01/one/actions/runs/40060363449), and Linux distribution conformance [40060363463](https://github.com/panglimanetizen01/one/actions/runs/40060363463). All three report `failure`.
- The GitHub job-details API returned empty step arrays for all six jobs; log retrieval returned `BlobNotFound` and there were no workflow artifacts. Consequently, these runs do **not** provide executable build/test logs or establish whether test steps ran. Do not infer a source-code failure from this alone, and do not infer a pass.
- The Android acceptance test source explicitly checks ELF magic, ELFCLASS64, little-endian encoding and `EM_AARCH64`, then pushes and executes the artifact through ADB and asserts exact output. This source-level inspection is not a substitute for successful emulator execution evidence.
- PR #80 remains open, draft and unmerged. The verified score remains **60/100**; implementation remains **88/100 provisional**. No new verified points are awarded.
- **GATE: NOT PASSED.** E1–E6 acceptance and usable exact-SHA verification evidence are still incomplete.


### CI infrastructure evidence — run attempt 2 (2026-10-24)

- Follow-up inspection of the raw GitHub Actions job payloads for run `40060644366` and `40060644370` confirms every failed job has `runner_id: 0`, an empty `steps: []` array, and a runtime of only a few seconds. The One CI check runs on the same source SHA `1c450863cdfcb824dcf7cef6b64e86e58bc25b44` also completed in approximately two seconds with failures before any step was recorded.
- This is evidence of a job/runner provisioning failure before workflow execution, not evidence that the source tests themselves failed. The API exposes no annotation details through the available connector and log downloads return `BlobNotFound`; therefore the exact underlying service/configuration cause remains unproven.
- Retried the failed focused stdlib and Linux distro jobs. The retry again reported `runner_id: 0` and no steps/logs. Do not repeat retries indefinitely as if they were source-level tests.
- The verification mirror's `verification` branch selector was advanced to the exact source SHA `1c450863cdfcb824dcf7cef6b64e86e58bc25b44`, which should trigger its push workflow. The mirror workflow requires `CANONICAL_READ_TOKEN` to fetch the private source. No successful mirror run has yet been independently observed for this selector; treat its execution and secret configuration as unverified until a concrete run and logs are available.
- No verified points are added. **GATE remains NOT PASSED.**


### Subsequent branch-head reconciliation

- A later branch commit was observed at `6826e3a1b41525bbfa5d3f02dda100c7e59a7004` (`docs(security): document signed artifact format binding`). This commit updates the provenance documentation to bind the validated package format alongside the artifact digest.
- Exact-SHA runs for this later commit: One CI [40062110808](https://github.com/panglimanetizen01/one/actions/runs/40062110808), focused stdlib [40062110699](https://github.com/panglimanetizen01/one/actions/runs/40062110699), and Linux distro [40062110726](https://github.com/panglimanetizen01/one/actions/runs/40062110726) all failed before recorded steps; each job reports `runner_id: 0`, empty steps, and a 2–3 second runtime.
- The verification mirror selector has independently advanced to `6826e3a1b41525bbfa5d3f02dda100c7e59a7004`; its runner result is not confirmed. No points are added and the gate remains **NOT PASSED**.


## 2026-10-24 — E4 ledger reconciliation and exact-SHA evidence audit

- **E4 documentation correction:** the current source snapshot already contains Unix descriptor-relative opens with `openat`/`O_NOFOLLOW` (the Unix implementation includes macOS) and Windows reparse-point plus opened-handle validation. The current `docs/SECURITY.md` describes these controls and their limits; `crates/one-core/src/adapter.rs` contains the implementation and regression tests. Earlier checkpoint notes stating that Windows/macOS race controls were still absent describe an older revision and are superseded for current-state reporting.
- **E4 remains partial:** the adapter contract still authorizes explicit target paths rather than a separately configured project root; broader runtime-wide quotas and the complete threat review remain open. Keep E4 implementation at 5/6 provisional and verified at 0/6 until final-revision acceptance is complete.
- On canonical source SHA `e745b6b46ec436e7f7d005a95cb24333a4128533`, independent exact-SHA cross-platform verification passed on Ubuntu and Windows in [run 40065575079](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40065575079); Ubuntu 24.04 and Debian 12 conformance passed in [run 40065353608](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40065353608); focused standard-library/database regressions passed in [run 40064936650](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40064936650). All three logs explicitly checked out and verified that exact source SHA.
- Linux and Windows release bundles for that SHA were created with SHA-256 digests and remain available from [run 40065575079](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40065575079). These results do not satisfy the Android runtime gate.
- Android attempts on the same source SHA: x86-host ARM64 TCG failed with `Timeout waiting for emulator to boot` ([run 40066472833](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066472833)); Linux ARM64 runner setup failed before emulator launch because the Android SDK emulator package was unavailable ([run 40066577198](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066577198)). The independent workflow was moved to a macOS ARM64 host for a native ARM64 Android emulator; [run 40066663909](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40066663909) was still in progress when this ledger correction was prepared.
- This is a documentation-only ledger correction. Per the exact-revision rule, the resulting commit SHA must still receive fresh exact-SHA verification; historical successful runs for the prior source SHA are not promoted to final-head proof.
- **Scores unchanged:** implementation 88/100 provisional; verified 60/100. **GATE: NOT PASSED.** E1, E2, E3, E4, E5, and E6 acceptance is not complete.


## 2026-10-24 — E2 scope reconciliation against normative package contract

- The current `docs/spec/one-v0.1-package-build.md` explicitly states that the executable v0.1 package contract supports an empty dependency set only. External One package registry, dependency graph and version-conflict resolution are outside that executable v0.1 subset; unsupported dependency declarations must fail closed. Do not list the absence of a registry as a v0.1 conformance defect or use it alone to block the milestone.
- The same normative contract documents the milestone-100 Ed25519 V2 sidecar binding the validated artifact format/digest to source-tree digest, clean Git revision, CLI version, builder OS/architecture, Rust/Cargo versions, and the empty-v0.1 dependency model. Verification checks the signature and a separately configured trusted-publisher mapping; optional source-root verification recomputes source digest/revision.
- **Actual E2 remaining gate:** release process integration and evidence that the final artifact is signed after the final build and verified against a trust store delivered through an independently trusted channel, plus exact-SHA acceptance. Artifact-only V1 provenance is not enough. The ledger row has been corrected to match this contract; the E2 implementation score remains 6/8 provisional and verified remains 0/8 until acceptance evidence passes.
- This corrects the scope wording without relaxing any approved contract or changing point weights. Overall totals remain **88/100 provisional implementation, 60/100 verified**. **GATE: NOT PASSED.**


## 2026-10-24 — E2 release-process integration added; final acceptance pending

- Added `.github/workflows/package-provenance-release.yml` as a protected manual release path. It builds a committed project with `one package`, signs the final `.onepkg` artifact using Ed25519 V2 source-bound provenance, verifies it against the configured trusted-publisher store and the same source tree, and uploads the artifact, sidecar, and SHA-256 digests.
- The release workflow requires the protected `one-release` environment and the externally provisioned `ONE_RELEASE_SIGNING_SEED` and `ONE_TRUSTED_PUBLISHERS` environment secrets. The trust store must be provisioned independently; the workflow does not synthesize a trust store from the signing key.
- Added `docs/RELEASE_PROVENANCE.md` with setup and operational acceptance requirements. Expanded the CLI integration regression to build the package through `one package` before signing and verification, retaining untrusted-publisher, artifact-tampering, and source-mismatch checks.
- The first exact-SHA build exposed a compile error in the new regression because the original artifact bytes were not retained. Commit `435fefdabc2b90d52883e08ef6edd9dc82b9173f` retains the built artifact bytes; the follow-up exact-SHA matrix must confirm this correction and all release workflow changes.
- This is implementation evidence, not release acceptance. The protected environment secrets/review rules have not been exercised by an actual release dispatch in this session, and no verified milestone points are awarded until exact-SHA tests pass and the trust-store provisioning is demonstrated.
- **Scores remain 88/100 provisional implementation and 60/100 verified. GATE: NOT PASSED.**


- Follow-up: added the committed release fixture at `examples/package-release/{one.toml,src/main.one}` and documented it in `docs/RELEASE_PROVENANCE.md`. It gives the protected manual release workflow a reproducible project input. This does not count as a release run; the workflow still requires the independently provisioned environment secrets and approval controls.


- Release workflow hardening follow-up: validated the publisher ID and 32-byte seed format before use, bounded the project path to the checked-out repository, required an exact publisher entry in the protected trust store, and changed artifact upload to use the exact package path emitted by the build step. The workflow uploads only the signed package, its provenance sidecar, and checksums.
- The workflow and committed fixture are implementation artifacts; they do not substitute for a protected-environment release dispatch. Keep E2 verified points at 0 until the final exact-SHA test matrix passes and the actual protected signing/trust-store process is exercised.


## 2026-10-24 — E4 explicit filesystem root policy added

- Added `ScopeRule::Root` for filesystem `Target` requests. The rule requires path-component containment, rejects parent-directory components, and rejects absolute/relative scope mismatches.
- Added CLI `--root <directory>` for `filesystem.read` and `filesystem.write`, preserving the prior exact-target behavior when `--root` is omitted. Added policy regressions for allowed descendants, prefix-sibling denial, parent-traversal denial, and a CLI integration test for inside-root versus outside-root reads.
- Documented the option in `docs/SECURITY.md` and explicitly limited the claim: this is a capability-policy boundary backed by existing adapter no-follow/reparse-point protections, not an operating-system sandbox or a substitute for general memory/open-resource quotas and full threat review.
- These changes are not credited as verified until the latest exact-SHA build, Clippy, workspace, network, distro, and Android checks complete successfully. E4 verified points remain 0/6 pending full acceptance.


## 2026-10-24 — E5 documentation coverage added; acceptance pending

- Added `docs/panduan-pengguna/referensi-standard-library.md` documenting registered source-level built-ins, parameter/return types, deterministic behavior, limits, and known unsupported domains.
- Added `docs/panduan-pengguna/alur-pengembangan.md` for the end-to-end project → local verification → exact-SHA evidence → release gate workflow; expanded `kesalahan-dan-debugging.md` with diagnostic families and capability/scope troubleshooting; updated CLI documentation for `--root`, linked the new pages from the documentation indexes, and recorded the current candidate boundary in the feature-status page.
- These documentation artifacts still require example/link review and exact-SHA contract/build/test evidence. No E5 verified points are awarded from documentation presence alone. Keep the ledger at **88/100 provisional implementation, 60/100 verified** until a fixed-weight forensic reconciliation is completed.
- The latest source revision after this documentation update must be pinned in the independent verification selector. The release gate remains **NOT PASSED**.


- Documentation index review found and repaired literal `\\n` sequences that had collapsed several verification links into single lines in `docs/README.md`. The final doc candidate now includes the standard-library reference, developer workflow, expanded debugging guidance, CLI root-scope instructions, and release provenance operations. The examples and links still require final review on the tested revision.


- Acceptance-boundary correction: `docs/spec/one-v0.1-effect-capability.md` now records that the new `--root` policy is defense-in-depth but **not full E4 root-confinement acceptance**. Root-relative descriptor anchoring, race-resistant resolution, mount behavior, and cross-platform case/path semantics remain open; E4 verified points remain 0/6.


## 2026-10-24 — E5 user-facing release guidance and documentation gate

- Added `docs/panduan-pengguna/provenance-rilis.md`, linked it from both documentation indexes, and changed the end-to-end guide to keep its relative links within the user-documentation tree. The new page explains that the manual release workflow becomes dispatchable after it is present on the default branch, and documents protected environment, key/trust-store separation, and the acceptance boundary.
- The independent verification mirror now runs `tools/check_user_docs.py` and its regression tests alongside contract checks on both Linux and Windows. Canonical `.github/workflows/ci.yml` now also runs the user-doc consistency checker and its tests on both platforms.
- Exact-SHA runs started after these documentation/CI changes; the final ledger/checkpoint commits will change the source SHA again, so the selector must be repinned and the full matrix rerun after those commits. Do not reuse results from earlier source SHAs as final-release evidence.
- Android emulator diagnostics identified `qemu-system-aarch64: failed to initialize HVF: Invalid argument` on the hosted macOS ARM64 runner. The independent Android workflow now uses `-accel off` to avoid unavailable host acceleration. Actual Android ELF execution remains unverified until a run completes successfully.
- **Scores unchanged: 88/100 provisional implementation, 60/100 verified.** E1–E6 remain incomplete; **GATE: NOT PASSED**. This entry awards no points from documentation or workflow presence alone.


## 2026-10-24 — provisional implementation reconciliation after E2/E5 integration

- **E2 implementation:** 6/8 → 7/8. The protected release workflow now builds, signs, verifies, and uploads the final source-bound V2 package path; actual protected-environment dispatch and trusted-store provisioning have not been demonstrated.
- **E5 implementation:** 2/4 → 3/4. Added a user-facing release-provenance guide, explicit CLI documentation for signing/verifying, clearer unsigned-package boundaries, and enforced documentation consistency checks in canonical CI plus the independent Linux/Windows mirror.
- These are provisional implementation points only. **Implementation: 90/100; verified completion: 60/100; gate NOT PASSED.** The final candidate SHA changed with this ledger update and must be pinned and tested exactly. E3 earns no verified points until the Android AArch64 ELF is actually executed on the current source SHA.


## 2026-10-24 — final acceptance checklist and candidate freeze preparation

- Added [`docs/verification/DEFINITION_OF_DONE.md`](DEFINITION_OF_DONE.md) as the auditable fixed-weight E1–E6 release checklist. It distinguishes implementation artifacts from verified acceptance and explicitly prohibits credit for a running workflow, prior-SHA result, checksum-only provenance, or infrastructure-only CI failure.
- Reconciled the provisional score: the latest E2/E5 adjustment is authoritative — **90/100 implementation, 60/100 verified, release gate NOT PASSED**. Earlier 88/100 statements in historical entries are superseded, not additional active score.
- Candidate source was advanced by the Definition of Done document. The independent verification selector must pin the resulting commit SHA and rerun cross-platform, Linux distro, focused standard-library/database/provenance, lockfile, and Android workflows. Results from earlier source revisions are historical only.
- Do not make further source changes after the next candidate SHA is selected unless a required test reveals a real defect. Any defect fix creates a new candidate and requires the exact-SHA matrix to be rerun.
- Outstanding acceptance is unchanged: E1–E6 row-level verification is not complete; protected signing/trust-store dispatch has not been exercised; Android AArch64 ELF runtime has not passed; E4 full threat/root-confinement review is open. **No release authorization.**


## 2026-10-24 — SQLite final-symlink hardening follow-up

- Security review found that the SQLite adapter performed a path precheck and then reopened the path through SQLite, leaving a final-component symlink time-of-check/time-of-use window.
- File-backed SQLite connections now combine `SQLITE_OPEN_READ_ONLY` with `SQLITE_OPEN_NOFOLLOW`; a Unix regression checks that SQLite rejects a final symlink and that the adapter denies the target.
- This is defense in depth only: it does not eliminate parent-component races or establish root confinement across mount points/platforms. E4 verified credit remains **0/6** until the complete scoped security review and exact-SHA matrix pass.
- The reusable Definition of Done no longer embeds an old candidate SHA. The next source commit is the freeze candidate for the independent matrix. **Implementation 90/100; verified 60/100; gate NOT PASSED.**


## 2026-10-24 — E5 user-workflow review and documentation reconciliation

- Reviewed the user-facing start guide, CLI command reference, test/build/package guide, standard-library reference, feature-status page, development workflow, and package-provenance instructions against the current CLI contract and executable tests.
- Corrected stale claims in `docs/panduan-pengguna/perintah-cli.md` and `docs/panduan-pengguna/status-fitur.md`: filesystem write, SQLite query, and HTTP GET now reference the exact-SHA Ubuntu/Windows tests and retain their explicit experimental scope.
- Independent review artifact: [USER_WORKFLOW_ACCEPTANCE.md](https://github.com/panglimanetizen01/one-windows-verification/blob/verification/USER_WORKFLOW_ACCEPTANCE.md). The review passes for the tested workflow and documentation consistency, while final exact-SHA reruns remain mandatory after the documentation commits.
- **E5 implementation score:** 3/4 → 4/4 because the documentation gaps are corrected and the user workflow has been reviewed. **E5 verified remains 0/4** until the final candidate's cross-platform, focused, distro, and lockfile evidence passes.
- Current source revision is a documentation/ledger reconciliation candidate; the independent selector must be pinned to the resulting commit SHA and all mandatory exact-SHA checks rerun. No other score or fixed weight changes.


## 2026-10-24 — fixed-weight arithmetic reconciliation

- Forensic arithmetic check of the fixed ledger rows: F1–F6 = 30 verified points and C1–C5 = 30 verified points; E1–E6 implementation rows currently total 31 provisional points (E1 7/10 + E2 7/8 + E3 8/8 + E4 5/6 + E5 4/4 + E6 0/4). Therefore the consistent implementation subtotal is **91/100**, not 90/100. Verified completion remains **60/100** because E1–E6 have 0 verified points.
- This is a correction of arithmetic/reporting only; weights and row scores are unchanged. It does not award verified points or pass the release gate.
- Candidate SHA before this ledger-only correction: `f46b95ebdd0e26c5ba17f6acb953538f32401aeb`. The ledger commit changes the candidate SHA, so all final-SHA evidence must be rerun against the resulting branch head.
- **Gate: NOT PASSED.**


## 2026-10-24 — E5 verified-point adjustment from executable evidence

- E5 acceptance evidence at candidate SHA `2ead9dba45db4ce4105deaa81720aacc8fcadd6c`: [cross-platform Ubuntu/Windows run 40088230116](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40088230116) passed CLI smoke and documentation consistency on both platforms; [focused standard-library/SQLite/provenance run 40088230099](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40088230099), [Linux distro run 40088230098](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40088230098), and [lockfile bootstrap 40088230148](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40088230148) passed on the same source SHA. The Linux artifact was independently executed through create/check/run/build/test/package; see the independent evidence index.
- Manual user-workflow review is recorded in [USER_WORKFLOW_ACCEPTANCE.md](https://github.com/panglimanetizen01/one-windows-verification/blob/verification/USER_WORKFLOW_ACCEPTANCE.md). This supports E5's 4/4 verified points at the stated SHA; it does not close E1, E2, E3, E4, E6, or the overall release gate.
- Ledger subtotal is now **91/100 implementation, 64/100 verified**. The ledger commit changes the candidate SHA, so this is a provisional row adjustment until the complete required matrix is rerun on the final frozen source SHA. Overall gate remains **NOT PASSED**.


## 2026-10-25 — Prompt 3 E3/E4 execution checkpoint (code-bearing SHA `5751aff3404f000545549908a00ea322cc96e277`)

This entry supersedes earlier “Android runtime pending” notes for the code-bearing SHA above, but it does **not** mark E3 or E4 accepted. The ledger's verified-point totals remain unchanged.

### Exact-SHA evidence

- **Linux distributions:** [ONE Linux Distro Verification run 40212771630](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771630) passed on Ubuntu 24.04 and Debian 12, including build, Clippy, workspace tests, and real-network tests.
- **Windows cross-platform suite:** [One Cross-Platform Verification run 40212771649](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771649) passed the Windows Server 2025 job, including build, Clippy, AST contracts, workspace tests, real-network tests, offline verification, and release-candidate bundle inspection. The Ubuntu job was still running at this checkpoint; do not count that workflow as a complete two-job PASS until its final result is available.
- **SEC-05 bounded subprocess:** [run 40212771660](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771660) passed Linux and Windows focused/full one-core tests and static guards. Scoped `rustfmt --check crates/one-core/src/interop.rs` passed. The whole-repository `cargo fmt --check --all` diagnostic reports pre-existing formatting drift in unrelated files; it is recorded separately and is not the scoped SEC-05 gate.
- **E4 filesystem/SQLite/policy regressions:** [run 40213093522](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40213093522) passed 3 Unix descriptor-relative/no-follow tests, 11 SQLite query/resource-limit tests, 10 governance policy tests, and 1 Windows directory-junction/reparse-point test. The full Windows workspace suite also passed `orchestrator::tests::denied_request_never_reaches_adapter_invocation`, confirming a denied request does not invoke the adapter.
- **Resource guard:** [run 40212771671](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771671) passed the bounded-subprocess static audit.
- **Android AArch64 build evidence:** [device artifact run 40213093523](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40213093523) built an ELF64 little-endian AArch64 PIE with interpreter `/system/bin/linker64` from exact source SHA `5751aff3404f000545549908a00ea322cc96e277`. Binary SHA-256: `b3a52ce990c157332ab39f25264fbdab3de9e796c26f827c497551237855967c`. This is **build/header evidence only**, not Android runtime acceptance.
- **Android API 35 runtime:** [macOS ARM64 runtime attempt 40212771680](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771680) installed the API 35 arm64-v8a image and passed pre-boot contract tests, but emulator boot failed because Hypervisor Framework acceleration was unsupported (`HV_UNSUPPORTED`). [Linux ARM64-host attempt 40212771636](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771636) failed before runtime because a compatible Android emulator package was unavailable on the host. Neither is a runtime PASS.
- **Canonical repository Actions:** job-step details for the canonical repository were unavailable through the current connector permission scope; those attempts are not counted as PASS or as code failures. The independent lab runs above executed actual steps and provide the available logs.

### E3/E4 disposition and residual risks

- **E3 remains unverified (0/8):** no exact-SHA API 35 `arm64-v8a` runtime execution and expected-output assertion has been captured. Keep the device artifact available for execution on a qualifying API 35 device/emulator; do not substitute API 34, build success, or emulator-image installation.
- **E4 remains unverified (0/6):** implemented controls and the listed regressions pass, but explicit project-root descriptor anchoring and mount-boundary behavior are not implemented across supported platforms. Runtime-wide quotas and fallback-platform no-follow guarantees remain incomplete. The capability-denial no-side-effect path is covered at orchestrator/adapter-invocation level; this does not establish full filesystem sandboxing.
- No release gate is changed by these partial results. **Milestone 3 remains NOT PASSED**, verified total remains **60/100**, implementation subtotal remains **91/100 provisional**, and PR #80 remains unmerged with no release authorization.

This ledger update is documentation-only after code-bearing SHA `5751aff3404f000545549908a00ea322cc96e277`; it creates a new branch head. Re-pin the independent lab to the resulting exact branch HEAD and rerun the required matrix before accepting any final result. Later results must be stored in the independent evidence index without silently reusing runs from the older SHA.

## 2026-10-25 — Follow-up correction after exact-SHA Windows full-suite run

The final-matrix attempt on source SHA `26d3d30be773cfc403daa002f7116c176ef31fc8` exposed two Windows-only stress-test failures in `stderr_flood_is_bounded` and `aggregate_output_limit_is_enforced`. The logs showed the test harness's 3-second deadline firing before the output flood reached the configured byte cap; the assertions correctly rejected the timeout error instead of accepting it as an output-limit PASS.

Code fix commit `c339da876f6ceb18084e8551cc08d4edbabc0f27` increases only the timeout used by the stdout/stderr/aggregate output-flood regression tests to 15 seconds. The tests still require the explicit output-limit error (`batas output`); a timeout continues to fail. Production process timeout and output byte limits are unchanged. This is a CI robustness correction, not a reduction in the resource-bound assertion.

The Android API 35 fallback workflow also exposed a harness defect: the AVD named `one-api35-arm64` was not created before emulator launch. The lab workflow now explicitly creates and verifies the API 35 `arm64-v8a` AVD before trying hardware acceleration and then the bounded `-accel off` fallback. The failed attempt is not runtime evidence.

This follow-up advances the source branch again. Pin the independent lab to the final branch HEAD after the remaining documentation checkpoint updates and rerun the complete exact-SHA matrix. Do not treat any run from SHA `5751aff3404f000545549908a00ea322cc96e277` or `26d3d30be773cfc403daa002f7116c176ef31fc8` as final acceptance for the new HEAD. E3 remains unverified until real API 35 runtime output is captured; E4 remains unverified because root descriptor anchoring, mount boundaries, runtime-wide quotas, fallback-platform guarantees, and the full threat review remain open.

## 2026-10-26 — E3/E4 final candidate hardening checkpoint

- Scope remains limited to E3 platform conformance and E4 security/resource bounds. PR #80 remains unmerged; PR #83 remains unmerged; no release was published.
- Source changes after code-bearing SHA `85ded41bea3d7e2f64f0758e1bfacb376f0cb2cd` include: Unix-only compilation gating for the descriptor traversal helper; Windows absolute-path normalization before root-relative resolution; a shared process-wide quota of 8 active resource-heavy operations with at most 4 concurrent SQLite queries; a 16 MiB aggregate interpreter-output cap per execution/event dispatch; and Linux mount-boundary plus parent-directory/symlink-swap regression tests.
- A Windows full `one-core` suite on the preliminary candidate passed after the Unix helper was cfg-gated, but it exposed three Windows filesystem adapter failures caused by extended-length/canonical path prefixes. The path normalization fix is now in the candidate. The exact-SHA E4 adversarial run [40261107951](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261107951) passed before the new mount/race tests were added. The Linux distro run [40261107750](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261107750) failed strict Clippy because a test asserted compile-time constants; that test was removed in source commit `68817d60646558a09bcbbdc08cd13ba9c0aeb220`. These are preliminary results, not evidence for the final source SHA.
- The updated cross-build workflow [40261107989](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261107989) and the x86_64 cross-build fallback [40261107880](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261107880) passed on the preliminary source SHA; they prove build/contract behavior only, not Android runtime acceptance. Runtime attempts on that SHA are still being evaluated; any boot failure remains a failure, not a pass.
- The final documentation commit changes the source SHA again. Re-pin `CANONICAL_COMMIT.txt` to the resulting branch HEAD, rerun the mandatory Linux distribution, Windows, SEC-05, E4 adversarial/resource, exact-SHA artifact, and Android API 35 `arm64-v8a` runtime workflows, then record final evidence in the independent evidence index. Do not reuse preliminary run results as final-SHA evidence.
- Fixed weights and scores remain unchanged: implementation `91/100` provisional, verified `64/100`; E3 remains `0/8` verified and E4 `0/6` verified. The overall gate remains **NOT PASSED** until every mandatory criterion has direct evidence. No merge or release authorization.


## 2026-10-26 — strict Clippy follow-up after final-SHA matrix start

- The exact-source Ubuntu job in Linux distro run [40261657177](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261657177) failed strict Clippy because the new parent-swap race test declared an unused local `temporary` path. The failure is a real lint failure, not a source test pass. The unused binding is removed; race behavior and assertions are unchanged.
- This source edit advances the candidate SHA after the previous matrix began. Re-pin the independent lab to the new exact branch HEAD and rerun the mandatory matrix. The earlier Linux E4 adversarial pass [40261657208](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40261657208), Windows reparse checks, cross-builds, and any Android run on the previous SHA are preliminary only.
- Scores/weights remain fixed: implementation 91/100 provisional, verified 64/100; E3 0/8 verified, E4 0/6 verified. Release gate remains **NOT PASSED**.


## 2026-10-26 — E3/E4 exact-source reconciliation (candidate SHA 73bd504)

This section supersedes earlier **status descriptions** for E3/E4 only; prior history and all fixed weights remain intact. The current PR #80 head was read from the GitHub pull-request API as `73bd504ac81a14737b321817b94c7bc2fbb17a15` on branch `milestone/100-ecosystem-20261023`; the independent lab's `CANONICAL_COMMIT.txt` matched that SHA at the time of this audit. PR #80 remains open/draft/unmerged; base is `main` at `d86c3be75c81a27d6d4f040f21718f6248413e47`.

Exact-source lab evidence recorded for SHA `73bd504ac81a14737b321817b94c7bc2fbb17a15`:

- Linux distribution matrix [run 40272900538](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900538): Debian 12 and Ubuntu 24.04 jobs completed successfully.
- Cross-platform suite [run 40272900560](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900560): Ubuntu and Windows Server 2025 jobs completed successfully.
- Candidate artifact integrity [run 40272900570](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900570): Linux bundle artifact ID `12629141490`, ZIP SHA-256 `2c78501d69f537af186131435e8c85eabd9889058837010de65978192ae13806`, binary SHA-256 `04cdcb64ea05782118004a916a039b49f560cb3111e28636b46922bc78da5fc4`, ELF64 little-endian EM_X86_64; Windows bundle artifact ID `12629650216`, ZIP SHA-256 `621755d22fa9de572ef4156395bd51c55483fe3cb12b41815028349ba659fc28`, binary SHA-256 `1e2c5e6826261918b33bc7842a835c0694a4d0e7e1bdf6268bb95a3a3a3118d6`, PE32+ x86-64. Embedded source metadata matched the pinned SHA.
- Focused database/resource-bound suite [run 40272900585](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900585): 247 tests passed, including SQLite query timeout/resource and standard-library boundary tests.
- E4 adversarial filesystem/resource suite [run 40272900540](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900540): Unix no-follow/SQLite and Windows reparse-point/handle jobs completed successfully. This is focused coverage, not a complete threat review.
- E4 subprocess resource guard [run 40272900536](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900536) completed successfully.
- Android API 35 AArch64 cross-build/contract [run 40272900641](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40272900641) completed successfully for build/contract only. It is not runtime acceptance.
- Android runtime is **not yet accepted**. [Run 40274054238](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40274054238) built and verified an API 35 AArch64 ELF but failed before runtime output. Subsequent harness retries exposed early-exit defects in the diagnostic/setup script; the workflow now supplies the native ARM64 ADB path and captures emulator diagnostics. [Run 40275435454](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40275435454) is the current API 35 `arm64-v8a` boot-and-execute attempt at the time of this entry; no PASS may be recorded until it captures guest API 35, ABI `arm64-v8a`, actual ELF output `ONE_NATIVE_BACKEND_OK`, and exit code 0.
- SEC-05 revalidation [run 40275249195](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40275249195) had a Linux job failure because the harness did not install the `x86_64-linux-gnu-ld` dependency used by its cross-linker fixture; the Windows job passed. The lab workflow now explicitly installs `binutils-x86-64-linux-gnu`; exact-SHA rerun [40275580929](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40275580929) is in progress. This environment failure is not a source-code PASS.

**Disposition:** E3 remains 0/8 verified and E4 remains 0/6 verified until all required criteria are directly proven on the frozen final SHA. These results are exact-source evidence for the code-bearing SHA above, but this ledger update itself advances the source branch. Re-pin the independent lab to the resulting final branch HEAD and rerun the required matrix before treating any result as final-SHA acceptance. The global gate remains **NOT PASSED**; no weights or points changed, no PR was merged, and no release was published.


## 2026-11-17 — PR #102 Windows Clippy correction

Exact run #44010448799 on source `eaa7cf77bd9971f1fd9b5205cef975e6acad270f`: Ubuntu SUCCESS; Windows FAILURE. Windows contract check and build passed; Clippy failed at `crates/one-core/src/interop.rs:894` because `.into()` converted `String` to the same type (`clippy::useless_conversion`, `-D warnings`). Windows tests and later gates were skipped, so the stable process-handle descendant regression did not execute. Minimal correction committed on PR #102 branch as `da740c4f7f9c72e9b76e573d23651334272582c5`; remove redundant `.into()` only. No workflow run was associated with this commit at the time of update. Keep Windows containment NOT VERIFIED until an exact-SHA run for `da740c4...` passes Clippy and executes the retained process-handle regression. Global release gate remains NOT PASSED.

## 2026-11-17 — PR #102 retained-handle failure and identity diagnostics

Exact run #44012805081 on source `da740c4f7f9c72e9b76e573d23651334272582c5`: Ubuntu SUCCESS; Windows FAILURE. Windows build and Clippy passed; tests reached `windows_job_object_terminates_descendants_when_parent_exits` and failed with `WAIT_TIMEOUT=258` on the retained process handle for descendant PID 3444. This confirms the original process object was still unsignaled after `run_process` returned; PID-reuse ambiguity is removed. The test-owned descendant is terminated through that retained handle after the observation. Follow-up diagnostic commit `efc5422845927c2a46e08e61aeb6a497e2f9980b` adds `GetProcessId` and `GetExitCodeProcess` checks on the retained handle so the next run confirms handle identity and reports its exit state. Run #44018063886 for exact SHA `efc5422...` is IN_PROGRESS; Windows and Ubuntu jobs are still running. Keep containment NOT VERIFIED until diagnostics identify the process object and a later exact-SHA run proves termination.

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

- PR #102 current HEAD is `7e10e758d7e1ca70f450bfbb6cf397d805500e53`. GitHub Actions run [#44019833034](https://github.com/panglimanetizen01/one/actions/runs/44019833034) is terminal SUCCESS on this exact SHA. Windows 2025 job `133826414765` and Ubuntu job `133826415064` both succeeded, including Build, Clippy, Tests, deterministic artifacts and configured vertical slices. The Windows descendant Job Object regression passed. PR #102 remains OPEN / DRAFT / UNMERGED.
- PR #95 current HEAD is `cef1f41c97585b6e25bf91af6e8b4281efe299c4`. CircleCI context `ci/circleci: one-linux` is terminal SUCCESS on this exact SHA; workflow `930378d8-49bb-416e-a42c-63e1797b991c`, job `0e2642b0-bd94-4892-baca-df5b6c8fa4ed`, status timestamp 2026-11-17 05:49:29 UTC. This is commit-status evidence; detailed CircleCI step logs were not exposed by the connected integration.
- PR #95 remains OPEN / DRAFT / UNMERGED. CircleCI is not active on main until separately authorized merge. The independent verifier PR #42 still pins `9b233dfe75da8c0752c203af3711bf3da319dc02`; no Windows verification is evidenced for `cef1f41...`. Current release acceptance remains NOT PASSED.
- Fixed-weight ledger remains unchanged; no points or DoD items are promoted by these CI results alone. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`. No merge or release.


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


## PR #95 exact-source Windows verification — run #42 — 2026-11-17

- Current PR #95 source `993f76351c7a119bea2e66f7d9e5bca07f392fbc` passed independent Windows Server 2025 run #42 / GitHub Actions ID `44048174334`, verifier SHA `6070e2ec839cc10b49068a84733d301ec6375b58`, job `133919144866`; all 31 primary steps terminal SUCCESS.
- Artifact `14312034494`, ZIP SHA-256 `0d86f4876414d032d4062554958f23d7308ae38c76ff3a4cf491292757df7ef0`; native EXE SHA `45ebf54262ab9d77ba14f63b79f7d0e80987df27659ea65311b078453dacdf9a`; release CLI EXE SHA `a53e5f5e41e493a3c98123a592321242f8efa2883f3d2ee8b538d4c6385c0a88`; manifest SHA `62297b0ea101d53fdcf93e610503d02e5ae803a2e1ec19f4680e6a7635f83834`. Artifact and manifest hashes match; both executables parse as PE32+ AMD64; native and CLI outputs `Budi`, exit 0; CLI `one 0.1.0`.
- Verifier PR #42 docs current commit `56271be82645357c2feaa1134ae2dc1d2ceae734` records run #42; documentation-only verifier run #43 / `44049102193` is IN_PROGRESS on the same exact source pin. No product source change.
- CircleCI status for PR #95 source `993f763...` is not visible through GitHub; optional `release-check` remains NOT VERIFIED. CircleCI Pipeline #558 belongs to PR #102 SHA `5ee645...`, not PR #95. No merge/release or DoD promotion.


## User installation prerequisites correction — PR #102 — 2026-11-17

- Review of workspace dependency `rusqlite = { version = "0.40.2", features = ["bundled"] }` confirmed the build compiles bundled SQLite C source. The installation guide previously listed Git/Rust only and omitted the native compiler toolchain, which could cause first-time builds to fail on clean Linux/Windows machines.
- Updated `docs/panduan-pengguna/instalasi.md`: Debian/Ubuntu installs `build-essential`; other Linux distributions need GCC/Clang and `make`; Windows requires Visual Studio Build Tools 2022+ with Desktop development with C++, MSVC x64/x86 build tools and Windows SDK, using Developer PowerShell. Added `cc`, `make`, `cl`, and `link` checks. The doc still accurately states no installer or official prebuilt release exists.
- Current PR #102 source HEAD `ac1bb214bc028395367cce7edc234a1909971e2a` is a docs-only descendant of exact-SHA GHA PASS `fe3e4d0...`. Fresh current-SHA GHA run has not yet appeared; no PASS is transferred. PR remains OPEN / DRAFT / UNMERGED.


## PR #95 and PR #102 exact-SHA verification checkpoint — 2026-11-17

- PR #95 source `993f76351c7a119bea2e66f7d9e5bca07f392fbc` passed Windows Server 2025 run #42 / `44048174334` and verifier-doc revision run #43 / `44049102193`. Run #43 verifier SHA `56271be82645357c2feaa1134ae2dc1d2ceae734`, job `133922119577`, terminal SUCCESS.
- Run #43 artifact ID `14313416141`, ZIP SHA-256 `4b5102dfdb1fcecc41a155f2b97fa3caf244986120c263f50bd5733888e7bbcd`; native EXE SHA `0706a1fb786283f673ca7f6bca4f596afcef1c70473f2bc1ac8f186bb1ddb5ed`; CLI EXE SHA `4b2115e3188380e1610290cb1e51336055fd22f54dbd3530ec138d0ecb568ed0`; manifest SHA `cedc87f7e982d9cc8837bb67b93a75efbc7ebbb22e6488422fac8801b7fcef2c`. ZIP/manifest hashes verified; both executables PE32+ AMD64; manifest source `993f...`, verifier `56271...`, native/CLI output `Budi`, exit 0, CLI `one 0.1.0`.
- PR #102 current source `ac1bb214bc028395367cce7edc234a1909971e2a` passed GHA run #2165 / `44049452639`, Windows job `133923280923`, Ubuntu job `133923281493`; both SUCCESS. This includes the NUL/empty process-spec validation and the updated installation guide documenting bundled SQLite C/C++ build prerequisites. Both jobs passed docs consistency/checker tests, build, Clippy, tests, artifacts and platform CLI smoke.
- Four new ProcessSpec regression tests were explicitly observed passing on Windows and Ubuntu in run #2159 exact code SHA `fe3e4d0...`; current docs-only descendant `ac1bb...` run #2165 also passed the full suite.
- CircleCI Pipeline #558 is attributed to PR #102 predecessor SHA `5ee645...`, not PR #95. PR #95 current SHA has no CircleCI context visible in GitHub; optional release-check NOT VERIFIED. E6/release acceptance remain NOT PASSED.


---

## Follow-up addendum — 2026-11-17 18:42 WIB

### PR #106 exact-SHA correction and latest test result
- PR #106 remains OPEN / DRAFT / UNMERGED. Its base is now `fix/commit-cargo-lockfile-20261117` (PR #102 branch), and its current head is `3b5c245d615066a5e9a00d4039da53bf8fe664c9`. The compare against PR #102 is 16 commits ahead / 0 behind with four incremental files: `crates/one-core/src/interop.rs` and the three effect/interop/runtime specification files.
- Previous head `4883c8b255c45bf9cec177d61fd02678dec24eca`, GHA run #2187 / `44075728210`: Ubuntu job `134013109795` FAIL; Windows job `134013109380` PASS. Ubuntu build and Clippy passed; 128/129 one-core tests passed. The failure was `escaped_session_writer_cannot_hold_output_readers_open`.
- Root cause was an over-strong test assumption, not a compile failure: the continuous escaped writer may remain after the direct child exits, while the reader is deliberately told to stop after bounded draining. In that race, captured output can equal the 1 MiB cap without observing an additional read, so successful bounded capture is valid. The regression now verifies prompt return and both stream-size ceilings, accepting either bounded success or the explicit output-limit error. Fix commit: `3b5c245d615066a5e9a00d4039da53bf8fe664c9`.
- Current exact-head GHA run has not yet been returned by the workflow-run lookup. Combined commit status currently shows CircleCI context `error` linking pipeline #608, but job-level details are unavailable in this session; cause remains NOT VERIFIED. Do not claim CI PASS.
- Prior GHA run #2177 / `44067989787` remains SUCCESS only for predecessor SHA `060ce71c3bfb272c3d90c65e95b0982bf0d69020`; it does not verify current HEAD.

### Issue #105 containment design review
- Issue #105 remains OPEN. PR #106 only closes the unbounded reader-join subproblem; it does not terminate escaped descendants.
- Linux cgroup v2 is a candidate, but “create nested cgroup + move PID + cgroup.kill” is not sufficient by itself. The kernel's delegation rule requires write access to both the destination `cgroup.procs` and the common ancestor's `cgroup.procs`; a same-UID workload may therefore be able to move itself out of a nested per-invocation cgroup if the ancestor is writable.
- `clone3(CLONE_INTO_CGROUP)` can create a child directly in a cgroup (Linux 5.7+, cgroup v2), avoiding spawn-then-move race. `cgroup.kill` kills the cgroup and descendants and handles concurrent forks. A valid backend still needs a permission/namespace boundary preventing the workload from migrating out, plus a kill handle inaccessible for tampering. Otherwise fail closed for strong containment or explicitly narrow the platform guarantee.
- References: Linux kernel cgroup v2 delegation and containment (<https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html#delegation>); cgroup.kill (<https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html#core-interface-files>); clone3 (<https://man7.org/linux/man-pages/man2/clone3.2.html>).

### Current acceptance boundary
- Windows Job Object containment evidence on PR #102 remains tied to exact source SHA `ac1bb214bc028395367cce7edc234a1909971e2a` and GHA run #2165 / `44049452639` (Windows job `133923280923`, Ubuntu job `133923281493`, both PASS).
- Unix strong process-tree containment remains NOT VERIFIED / OPEN. Do not promote E4/E6 or release acceptance based on PR #106 reader-shutdown evidence.
- PR #95 CircleCI migration remains NOT VERIFIED on its current candidate; CircleCI operational job details are not available in this session.


## Health-scanner verifier provenance clarification — 2026-11-17 18:42 WIB

The earlier stale-workflow concern has now been reconciled against the live verifier PRs and exact source pins:

- Verifier PR #39 is OPEN / DRAFT / UNMERGED, branch `verify/one-health-pr91-20261029`, HEAD `6e2ce12eeb36ec2ffa0db38c9fce7e98faf2fbdd`, base `verification`. Its changed files include `.github/workflows/health-scanner-verification.yml`, `HEALTH_SCANNER_SOURCE_COMMIT.txt`, and separate Linux/Windows workflows.
- The health-scanner workflow on PR #39 reads the dedicated `HEALTH_SCANNER_SOURCE_COMMIT.txt`, not the shared `CANONICAL_COMMIT.txt`. Dedicated source pin: `9b38c246f0ef18f6a39a79ff2be81c4ba06b63db`. At this exact ONE commit, both `tools/one_health.py` (blob `5e6b1b4fe25bdaac8dd7ef3d387fb3da7617dba2`) and `tools/test_one_health.py` (blob `7795d9ed3ea4b4af2b9ad154f7759427c5f92679`) exist.
- Verifier base branch `verification` shared `CANONICAL_COMMIT.txt` is `e50748a1b25922d9090eba1f5fb90548d7c7d003`, which is the current ONE PR #95 candidate source pin, not the Python health-scanner source pin. The scanner files are absent at `e507...`, as expected. PR #39's dedicated pin prevents the shared source pin from retargeting its historical scanner test workflow.
- The health-scanner workflow is absent from verifier `main` (GitHub contents API 404). It is present on the verifier `verification` branch (workflow blob `1a8302cebc893fc709c1ea1fadc3e2f10b82cea3`) and inherited by the PR #42 branch, but it is not part of PR #42's changed-file list. The current `verification` branch workflow reads shared `CANONICAL_COMMIT.txt`, whose value is `e50748a1b25922d9090eba1f5fb90548d7c7d003`; the ONE scanner files are absent at that source SHA. This is a stale source-pin mismatch on the verifier `verification` branch. PR #39 corrects the mismatch: it changes the health workflow to read `HEALTH_SCANNER_SOURCE_COMMIT.txt`, pins that dedicated file to `9b38c246f0ef18f6a39a79ff2be81c4ba06b63db`, and sets its shared `CANONICAL_COMMIT.txt` to the same PR #91 source for the other PR #39 Linux/Windows workflows. At exact source SHA `9b38...`, `one_health.py`, `test_one_health.py`, `test_collect_ci_health.py`, and `test_health_quality.py` all exist. PR #39 remains OPEN / DRAFT / UNMERGED and its workflow has no recorded run; do not claim execution PASS. PR #42's branch snapshot has a different shared pin (`5486c09b746254e65ec647e86db3e48fde975747`), where the scanner files are present, so do not conflate that snapshot with the current `verification` branch pin. Preserve PR #39 as the corrective path; do not reintroduce scanner code into current ONE product branches.
- Verifier PR #40 is a separate native-scanner verification for ONE PR #89, pinned by its `CANONICAL_COMMIT.txt` to `242b88e4c8b6f2c2c2039f2bc95ebeab20ae96de`; it is not the Python health-scanner workflow. PR #40 remains OPEN / DRAFT / UNMERGED.
- PR #39 workflow triggers on pushes to verifier branch `verification` or manual dispatch. Since PR #39 remains draft/unmerged, its new workflow execution is NOT VERIFIED; do not claim it ran. No merge or dispatch was performed.
- Conclusion: the workflow is historical PR #39 verification scope, not a stale workflow in verifier main or PR #42. Preserve it and its source pin; do not reintroduce developer scanner files into current ONE product branches.


## Session continuation checkpoint — 2026-11-17 18:55 WIB

### PR #106 — bounded Unix reader drain hardening

- PR #106 remains OPEN / DRAFT / UNMERGED on branch `fix/unix-reader-shutdown-deadline-20261117`, based on PR #102 branch `fix/commit-cargo-lockfile-20261117` at base SHA `ac1bb214bc028395367cce7edc234a1909971e2a`.
- Current exact source HEAD: `5c4b482009e88f7e4db7d2585afd29c3996c2df9`. Compare to PR #102: 21 commits ahead / 0 behind; four incremental files only: `crates/one-core/src/interop.rs`, `docs/spec/one-v0.1-effect-capability.md`, `docs/spec/one-v0.1-interop.md`, `docs/spec/one-v0.1-runtime-model.md`.
- Further review found that the prior test-only correction (accepting bounded success when the escaped writer had not yet been observed beyond the cap) could mask output overflow or a pipe that never became idle. The implementation now drains nonblocking after shutdown until the pipe is idle, output exceeds 1 MiB per stream, or a 100 ms shutdown grace expires. It returns an explicit error for output overflow or a non-idle reader at the grace deadline. The continuous-writer regression now requires one of those errors and uses `yes x` as its high-throughput escaped writer fixture.
- Code commit `15edfef435f8054c505d7ff4b451f5503a5108e4` adds the bounded drain/error behavior and Unix-only compilation guard. Contract documentation commits: interop `39f9daa2778be5b09ab640daa060361d9c612367`, effect-capability `90fe9c64e0e2344c8a1712f430c0fab19cb49a2a`, runtime-model `5c4b482009e88f7e4db7d2585afd29c3996c2df9`.
- Previous candidate `4883c8b255c45bf9cec177d61fd02678dec24eca`, GHA run #2187 / `44075728210`: Ubuntu job `134013109795` FAIL at continuous escaped-writer regression; Windows job `134013109380` PASS. The current exact HEAD has no GHA run returned and no GitHub status contexts.
- The GHA workflow `.github/workflows/ci.yml` has `pull_request.branches: [main]`, while PR #106 currently targets PR #102's branch. Therefore the normal GHA PR workflow does not trigger for this dependent-base configuration. The execution environment also has no Cargo/Rust toolchain for local test execution.
- Current exact HEAD combined status has CircleCI context `CircleCI Pipeline` = `error`, pipeline #622. The CircleCI page is inaccessible through the available session tools and no CircleCI connector is exposed; job-level root cause and test result remain NOT VERIFIED. Do not infer code failure or PASS from the status context alone.
- PR #106 body now records the 100 ms grace, exact SHA, prior failure, trigger limitation, CircleCI pipeline status, and security boundary.

### Issue #105 security boundary

- Issue #105 remains OPEN. This work bounds output-reader shutdown; it does not terminate escaped descendants. No E4/E6 or release-acceptance promotion.
- Linux cgroup v2 remains only a candidate. A nested cgroup under a shared writable delegated ancestor is insufficient if the workload can migrate itself out. Any strong backend must establish a verified permission/namespace boundary, atomic placement, and a kill handle the workload cannot tamper with; otherwise fail closed or explicitly narrow the platform guarantee.

### Health-scanner verifier provenance correction

- Verifier `main` does not contain `.github/workflows/health-scanner-verification.yml` (contents API 404). The verifier `verification` base branch does contain an earlier workflow version (blob `1a8302cebc893fc709c1ea1fadc3e2f10b82cea3`) that reads shared `CANONICAL_COMMIT.txt`; current base pin is `e50748a1b25922d9090eba1f5fb90548d7c7d003`, where ONE scanner files are absent. This is a stale source-pin mismatch on the verifier `verification` branch, not on verifier `main`.
- PR #39 branch `verify/one-health-pr91-20261029`, HEAD `6e2ce12eeb36ec2ffa0db38c9fce7e98faf2fbdd`, corrects this by using dedicated `HEALTH_SCANNER_SOURCE_COMMIT.txt` pin `9b38c246f0ef18f6a39a79ff2be81c4ba06b63db`. At that exact ONE PR #91 SHA, `tools/one_health.py`, `tools/test_one_health.py`, `tools/test_collect_ci_health.py`, and `tools/test_health_quality.py` all exist. PR #39 also sets its shared canonical pin to the same source for its PR #91 Linux/Windows workflows.
- PR #39 remains OPEN / DRAFT / UNMERGED; no workflow run for its current head was returned. The workflow's push trigger targets verifier branch `verification`; a push to the draft PR branch does not execute it. No merge or dispatch was performed. PR #39 body now records the branch-level pin mismatch and corrected ownership.
- PR #42 changed files do not include the health-scanner workflow. Its branch snapshot has a different shared pin `5486c09b746254e65ec647e86db3e48fde975747`, where scanner files are present; do not conflate that snapshot with the current verifier `verification` branch pin.

### Documentation and release status

- Updated PR #97 `docs/verification/README.md` and PR #103 `docs/verification/one-completion-ledger.md` / `DEFINITION_OF_DONE.md` to correct the health-scanner provenance statement and preserve the current exact-SHA evidence boundary.
- E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED. No score or checkbox promotion. PRs remain open/draft/unmerged. No merge, release, force-push, history rewrite, visibility, secrets, or billing changes.

### Next executable action

- Obtain an authorized exact-SHA CI execution path for PR #106's dependent-base branch (CircleCI pipeline #622 job details or an appropriately triggered GHA run), then fix any reproduced issue and verify Linux + Windows. In parallel, keep Issue #105 open until strong Unix containment or an explicitly narrowed per-platform contract has production descendant-cleanup evidence.
