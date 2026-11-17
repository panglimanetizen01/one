# Forensic Repository Audit — Project ONE — 2026-11-17

**Audit type:** repository-wide documentation, contract, derivative-artifact, CI ownership, exact-SHA evidence, and session-continuity audit.  
**Repository:** https://github.com/panglimanetizen01/one  
**Canonical main snapshot audited:** `d35db95153f6ddd79885494c72f5aee16c899751`  
**Current migration candidate:** PR #108, branch `ci/circleci-migration-rebased-20261117`, based on PR #102 / PR #107 source lineage. Refresh its exact HEAD from live PR metadata before relying on this report.

## Live continuation addendum — 2026-11-17

PR #108 now includes the source/test changes from PR #98–#101. Integration source commit: `e8cd41ec212c3224bdbecf85eaf18c5850096e16`. Normative contracts and verification/release records were synchronized in later commits. The candidate's live HEAD must be refreshed from PR metadata; the integrated source has NOT been executed as a combined exact-SHA candidate. No Ubuntu/Windows PASS or CircleCI PASS is claimed for it. Main remains unchanged. Issues #104, #105, and #82 remain open; DoD remains 3/13 and E6 remains NOT IMPLEMENTED.

## 1. Repository inventory and audit coverage

The recursive Git tree for canonical `main` was complete (not truncated):

- 232 tracked files total.
- 98 files under `docs/`: 26 specification files, 17 user-guide files, 15 implementation plans, 10 design specifications, 14 verification/continuity files, and 16 other architecture, decision, foundation, and implementation documents.
- 105 files were in the wider documentation/contract/CI/tooling scope when top-level README, workflows, scripts, and related contract artifacts were included.
- 200 branches and 25 open pull requests were returned by GitHub at the audit snapshot.
- 5 open issues were returned: #105, #104, #90, #82, and #81.

All 98 documentation paths were enumerated and retrieved for content review. The user-guide link/status consistency rules were independently replicated over all 17 files on the updated candidate branch: no broken internal links and no invalid status labels were found. This is a static readback check; it is not a claim that the CI checker itself was executed on the new candidate SHA.

### Architecture diagram

The architecture map is present on draft PR #97, not on canonical `main`:

- URL: https://github.com/panglimanetizen01/one/blob/docs/system-architecture-diagram-20261115/docs/architecture/one-full-architecture-map.png
- PR #97 source SHA: `029471e0396504c030db60833fcda5b5baff24dd`
- Blob SHA: `c3a24f010ed56886ec240ee1c9a38b450331b04d`
- Size: 2,052,034 bytes
- Declared dimensions: 5150 × 11552
- Declared SHA-256: `e2f30a7fe3d11acd9ae76d26d2a4cd71ff3d4c5a3fa27cc13bdf9f35b313d638`

The diagram's GitHub path, blob identity, size, and documentation references were verified. This session's file connector could not render the binary PNG for visual inspection. Do not re-upload or recreate it; the next session must open the exact PR #97 image URL and inspect the diagram visually. The written architecture contract remains authoritative.

## 2. Confirmed documentation drift and corrections made on the PR #108 candidate

The following drift was present on `main` or in its documentation tree:

1. `README.md` described the canonical repository as private although GitHub reports it PUBLIC.
2. `docs/verification/README.md` was a one-line instruction saying verification artifacts must remain outside a canonical private repository.
3. `docs/verification/windows-native-verification.md` also said the canonical repository remained private and described the verifier as future work.
4. The documentation index referenced a repository GitHub Actions Windows workflow as the derivative artifact; that workflow is retired on the CircleCI migration candidate.
5. Historical plan documents and status tables continued to describe PR #64 / October SHAs as the current baseline, despite newer PR #98–#108 work.
6. Some old specification dependency sections still described grammar, semantic IR, runtime, or foundation design as future gates even though those contracts now exist.
7. User-facing CLI/status documents did not clearly separate current-main behavior from branch-only candidate fixes.
8. The Notion Skill description and opening scope block still identified PR #95 as the canonical CircleCI candidate and the 2026-11-13 ci-local extraction as the active mission.

Corrections already committed to the PR #108 candidate include:

- Updated `README.md`, `CONTRIBUTING.md`, `docs/README.md`, and `docs/verification/README.md` for public repository visibility, CircleCI candidate state, Windows verifier ownership, and current evidence boundaries.
- Replaced the stale Windows verification note with an exact-source/verifier identity contract.
- Added `docs/spec/one-v0.1-ci-verification-contract.md` and indexed it from `docs/spec/README.md`.
- Added current exact-SHA checkpoints to the release contract, implementation status, evidence matrix, and verification record.
- Added user-facing warnings for unmerged security fixes and the limits of experimental process/filesystem/database operations.
- Corrected stale foundation/specification dependency wording while preserving the approved normative contracts.
- Preserved dated design/plan/evidence artifacts as historical records rather than rewriting their old facts. Current state is now stated in the live checkpoint/index documents.

## 3. CircleCI and GitHub workflow forensic findings

### Canonical main

At SHA `d35db95153f6ddd79885494c72f5aee16c899751`:

- `.circleci/config.yml` is absent.
- `.github/workflows/ci.yml`, `.github/workflows/release-check.yml`, and `.github/workflows/windows-native-verification.yml` are present.
- Therefore CircleCI has not replaced GitHub Actions on `main`.

### PR #95 and PR #108

- PR #95 remains OPEN/DRAFT at `3f0ddb83b70d91b73aad3875d9eb1e4f5637064c`. It is an older source snapshot and must not be merged directly over newer subprocess changes.
- PR #108 was recomposed on the latest PR #102 base and now also carries the four-file bounded-reader change from PR #107.
- The updated PR #108 candidate adds `.circleci/config.yml`, removes all three repository GitHub Actions workflow files on the candidate branch, updates the contract checker to require CircleCI rather than GitHub Actions workflow files, and enforces `--locked` in CircleCI and both offline verifier scripts.
- The CircleCI config retains Linux product/release workflows; Windows Server 2025 native execution remains a separate independent verifier responsibility.
- PR #108 has no CircleCI status context or GitHub Actions workflow run visible at the audit checkpoint. Config presence and static review are NOT CircleCI execution evidence.

### Static defects found and fixed on PR #108 candidate

1. `tools/check_contract.py` originally required the three GitHub Actions workflow files. It would therefore fail after the migration deleted those files. It now requires `.circleci/config.yml`, `Cargo.lock`, and the CI verification contract instead, and fails if any of the three retired GitHub Actions workflow files reappear.
2. The portable offline scripts used Cargo build/Clippy/test/run commands without `--locked`, contrary to the committed lockfile policy. All such commands in `tools/verify_offline.sh` and `tools/verify_offline.ps1` now include `--locked`.
3. The first PR #108 migration candidate removed only `ci.yml`, leaving the legacy `release-check.yml` and `windows-native-verification.yml`. All three legacy repository workflows are now absent from the candidate branch.
4. The initial PR #108 candidate did not contain PR #107's reader-shutdown implementation. The implementation and its three reconciled process/capability/runtime specifications have now been copied from exact PR #107 source SHA `21e393157e52728d9510699ac6c9120987e3852c`.

### CircleCI status interpretation

- PR #102, #103, and #107 show generic CircleCI Pipeline ERROR contexts (#671, #673, #669) on refs without `.circleci/config.yml`.
- The old PR #106 pipeline #643 had an explicit provider message: `No configuration was found in your project`.
- Do not claim the same exact error for other pipelines without their job logs.
- PR #108 currently has no CircleCI context; there is no confirmed CircleCI job execution or schema validation.
- The CircleCI application pages and job logs were inaccessible through the tools available in this session. No CircleCI token was requested or exposed. A terminal CircleCI PASS must be verified through an authenticated CircleCI connector/CLI or a user-provided redacted job report; never ask the user to paste a secret token into chat.

## 4. Exact-SHA evidence — do not repeat without a relevant change

All records below are exact-source GitHub Actions evidence, not CircleCI evidence:

| PR | Exact source SHA | Run | Ubuntu | Windows Server 2025 | Scope |
|---|---|---:|---|---|---|
| #98 | `a83321bc41b3d0d28378841e29a6169abed8ac15` | #2018 / `43923604400` | PASS | PASS | project manifest validation and safe package filename components |
| #99 | `8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9` | #2019 / `43923948205` | PASS | PASS | read-only SQLite file-open behavior |
| #100 | `6375356c9c275ace5fd35cf4a81799a9546e1e1d` | #2020 / `43924465030` | PASS | PASS | interpreter scope restoration after runtime errors |
| #101 | `cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f` | #2138 / `44032422058` | PASS | PASS | inherited filesystem denials and approval requirements |
| #102 | `6c72d8e548bbbc28d85be39b8de427614989a038` | #2229 / `44086429416` | PASS | PASS | Cargo.lock, bounded subprocess execution, Windows Job Object regression |
| #103 | `041b359b8504551c2c1705093050d9c467368bac` | #2231 / `44089449162` | PASS | PASS | ledger/DoD documentation consistency |
| #107 | `21e393157e52728d9510699ac6c9120987e3852c` | #2230 / `44088328030` | PASS | PASS | bounded Unix reader shutdown and cross-platform tests |
| #97 | `029471e0396504c030db60833fcda5b5baff24dd` | #2220 / `44082263286` | PASS | PASS | architecture/documentation branch |

These are separate draft branches. The PASS records do not prove that PR #98–#102 and #107 are all integrated into one source revision. PR #108 is being used as the CircleCI migration candidate; after integration, its new exact SHA requires its own CircleCI execution evidence.

## 5. Open product/security gates

- **Issue #104 — filesystem root confinement:** main's file adapters use path-based standard library reads/writes and do not yet provide race-resistant project-root confinement. Required direction is trusted root/parent handles and handle-relative operations (Unix `openat` + `O_NOFOLLOW`; Windows handle/reparse-point validation), with traversal, symlink/junction, mount, hard-link, resource-limit, and side-effect tests. Capability checks remain necessary but are not a sandbox. Keep filesystem features experimental.
- **Issue #105 — Unix escaped descendants:** process-group cleanup can be escaped by a descendant creating a new session/group. PR #107's nonblocking reader and 100 ms grace bounds reader shutdown but does not kill that escaped descendant. Do not call it process-tree containment. A strong design needs a host-owned, non-escapable containment boundary; cgroup v2 is only a candidate when host-created/delegated, atomically assigned, and host-owned for teardown.
- **Issue #82 — Android AArch64 E3:** actual device/runtime execution remains unaccepted. Cross-build or ELF inspection is not execution evidence.
- **Issue #81 — E4 subprocess:** historical issue remains open; reconcile it against PR #102/#107 exact-SHA evidence and Issue #105 before changing its state.
- **Issue #90 — hosted runner allocation:** historical infrastructure issue remains open; it is not a source-code failure.
- **PR #98–#101:** each exact SHA passed independently, but all four fixes remain unmerged. Integrate and re-run once on a single combined SHA rather than repeating the already successful isolated runs.

## 6. Completion and safety state

- Definition of Done remains 3 checked / 10 unchecked / 13 total.
- E1–E6 verified points remain 0 pending full acceptance contracts on a frozen final SHA.
- E6 remains **NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED**.
- PRs #95, #97, #102, #103, #106, #107, and #108 remain open/draft as applicable. No merge, release, branch rewrite, repository visibility change, paid plan/credit purchase, secret change, or billing change was performed.
- No product grammar, semantics, compiler/runtime meaning, CLI vocabulary, or beginner-facing UX was changed to satisfy CI tooling.

## 7. Post-ledger-sync live delta

- PR #103's previously verified ledger SHA `041b359b8504551c2c1705093050d9c467368bac` passed run #2231 / `44089449162` on Ubuntu and Windows.
- The subsequent ledger/DoD synchronization advanced current PR #103 HEAD to `fde2b81b2f63743b990e9fbbade47ec8076e2151`. Its run #2232 / `44092982606` is IN_PROGRESS on Ubuntu and Windows; CircleCI pipeline #709 has a generic ERROR context. No terminal result or root cause is claimed for this newer head.

## 8. Immediate next actions

1. Read the updated Notion Skill and this audit record; open the architecture diagram on PR #97 and read the complete documentation map before editing.
2. Refresh live metadata for PR #108, PR #102, PR #107, PR #103, PR #97, and Issues #104/#105. Do not rely on embedded historical head pointers.
3. Obtain CircleCI schema validation and a terminal pipeline/job report for the updated PR #108 exact HEAD. Inspect every job/step; do not infer PASS from a green pipeline context without required job evidence.
4. Integrate the already-PASS PR #98–#101 fixes into one candidate based on the current PR #108 source without force-pushing or merging. Resolve conflicts by preserving normative contracts; do not copy unrelated legacy milestone branches.
5. Implement Issue #104 with race-resistant handle-relative filesystem confinement and adversarial Unix/Windows tests.
6. Continue Issue #105 host-owned Unix containment design. If a robust boundary cannot be implemented portably, explicitly define the supported security contract and retain the limitation; do not weaken tests or claim containment.
7. Revisit Android E3 Issue #82 only with a real device/runtime execution path.
8. After each completed task, update source tests, normative specs, user docs, implementation status, evidence matrix, completion ledger/DoD, and the Notion Skill using exact-SHA evidence. Do not promote scores/checkboxes until acceptance criteria are actually satisfied.



## 9. Scanner migration correction — source branches recovered — 2026-11-17

This section supersedes the earlier interpretation in §5 and §8 that Issue #90 is only a historical runner-allocation problem. The runner incident is infrastructure-related, but the CircleCI candidate has a separate, concrete scanner migration gap.

### Native scanner (PR #89)

- PR #89 remains OPEN/DRAFT/UNMERGED at source SHA `242b88e4c8b6f2c2c2039f2bc95ebeab20ae96de`.
- Native module `crates/one-cli/src/integrity.rs` exists at blob `8f5f162b1ba19cc730df77fcbe83595f8d9e455d`.
- CLI integration is in `crates/one-cli/src/main.rs` blob `4b42efe2467cefc5a6a5822dd3c1408b86d6b344`; it adds `mod integrity`, `one pindai`, and `one pindai keamanan`.
- ADR-0018 is present in `docs/DECISIONS.md` blob `e20ef64e1dbdf5eec526f3955a8d0bf08598ed52`. It explicitly retains Python as the developer/CI reference until native/Python parity is reviewed and exact-SHA platform evidence is established.
- Source-repository preview run #40795245283 failed; its job logs are unavailable (BlobNotFound). Do not classify that as a code/test failure.

### Independent validation of native scanner source SHA

The independent verifier PR #40 remains OPEN/DRAFT/UNMERGED, but completed exact-source jobs provide positive evidence for source SHA `242b88e4...`:
- Run #40797612541: Ubuntu and Windows jobs passed, including build, scanner/self-scan, Clippy, workspace tests, and real-network tests.
- Run #40797612556: Debian 12 and Ubuntu 24.04 build, Clippy, workspace tests, and real-network tests passed.
- Run #40797612448: Android API 35 AArch64 ELF build and ARM64-host runtime passed. This is native backend evidence at the same source SHA, not scanner runtime evidence on Android.

### Python reference scanner (PR #91)

- PR #91 remains OPEN/DRAFT/UNMERGED. Its live branch head is `9b38c246f0ef18f6a39a79ff2be81c4ba06b63db`; older head references in its PR description are stale.
- Latest `tools/one_health.py` blob: `5e6b1b4fe25bdaac8dd7ef3d387fb3da7617dba2`.
- Latest `tools/test_one_health.py` blob: `7795d9ed3ea4b4af2b9ad154f7759427c5f92679` (72 test methods).
- No workflow run surfaced for current head `9b38c246...`; earlier passing evidence on d67f/8c455/9197 does not validate this latest head.

### Exact PR #108 candidate gap

At previously verified candidate HEAD `039c7d70f3ccdde5c9fe91551e54595afb0dbd05`:
- Native `integrity.rs` is absent.
- Python `tools/one_health.py` is absent.
- CLI `main.rs` has no native integrity module or dispatch.
- `docs/DECISIONS.md` does not contain ADR-0018.
- `.circleci/config.yml` defines only `one-linux` and `release-linux`; no scanner/reference job exists.

### Corrected required action

Do not close or de-scope Issue #90. Port the native preview and ADR-0018 from PR #89 onto a branch based on the current PR #108 candidate; restore the latest Python reference scanner and required test/collector/quality assets from PR #91; add development-side CircleCI jobs with exact checkout SHA, least privilege, and redacted findings; run both scanners on the same integrated checkout and adjudicate parity differences. Keep native scanner preview-only until parity and platform acceptance are established. No code port or current-candidate scanner validation has yet been performed.

The earlier isolated PR #89/#91 PASS evidence remains tied to its exact historical source SHA and must not be attributed to PR #108 or any later integration SHA.


### Scanner restoration inventory (exact paths; no bulk branch copy)

The latest PR #91 branch head is `9b38c246f0ef18f6a39a79ff2be81c4ba06b63db`. Its scanner-specific Python assets absent from the PR #108 candidate are:
- `tools/one_health.py` — `5e6b1b4fe25bdaac8dd7ef3d387fb3da7617dba2`
- `tools/test_one_health.py` — `7795d9ed3ea4b4af2b9ad154f7759427c5f92679` (72 tests)
- `tools/collect_ci_health.py` — `18134b37256009802b8ca0d1909225a88e10e2f7`
- `tools/test_collect_ci_health.py` — `6b914399964d5c1b89712ec3515ed02390b38eb9`
- `tools/health_quality.py` — `80955db25665ee25dbdbaf455289569d5b6b86b7`
- `tools/test_health_quality.py` — `062bd422f8b353bbfdf90b3fdc54961a923663d4`

Scanner governance assets absent from PR #108:
- `docs/verification/health-detectors.md` — `4427cee8790520194b379530440ab390a11929b6`
- `docs/verification/repository-health.md` — `777c13e89e576a055c1c215a20d73a3e4eb0a1fd`
- `docs/verification/health-quality-ledger.md` — `aeb800e237d61b69aa258c5e50b64e6338605283`
- `docs/verification/health-quality-ledger.example.json` — `9bc7f946c2cad51750e9174d58aab3f21ed295b7`
- ADR-0018 and native preview docs from PR #89.

Do not bulk-copy unrelated milestone files `tools/milestone1-e1-lab.sh` or `tools/verify-human-vnext.sh`. Restore only scanner assets and their required docs/tests, then add a development-only CircleCI job. PR #91 current head `9b38c246...` has no exact-head verification run surfaced; do not credit earlier scanner PASS evidence to it.


## 10. Scanner restoration implementation — child PR #109 — 2026-11-17

A separate draft integration PR was opened rather than rewriting or merging PR #108:
- PR #109: https://github.com/panglimanetizen01/one/pull/109
- Head branch: `ci/scanner-restoration-20261117`
- Base branch: `ci/circleci-migration-rebased-20261117` (PR #108 source branch)
- Current checkpoint head after the verification-index update: `4aa0198d63db22258385cf6afc9d9f5f5446c6c2`. Refresh live metadata before using it.
- PR #109 remains OPEN/DRAFT/UNMERGED. PR #108 remains OPEN/DRAFT/UNMERGED at `a6f91c07b04d7b9fafb9e99d0356954012ae3001`.

### Implemented on PR #109 branch
- Restored native module `crates/one-cli/src/integrity.rs` from PR #89 exact blob `8f5f162b1ba19cc730df77fcbe83595f8d9e455d`; added CircleCI config structural checks and a Rust regression test.
- Updated `crates/one-cli/src/main.rs` blob `f51b61da558fd1b539d6a90ede58e816d262958e` with developer-tool commands `one pindai` and `one pindai keamanan`. No beginner-facing command guide or language semantics changed.
- Restored Python scanner and test suite from PR #91 latest branch head `9b38c246...`; updated `tools/one_health.py` blob `389aaf081ca053d65e668a294f3052397a4cecf8` to include generic `.circleci/config.yml` workflow integrity checks and recognize `CIRCLE_SHA1`.
- Added Python CircleCI config regression test in `tools/test_one_health.py` blob `899caa80186277658973cb110011f1fe6dadcae0`.
- Restored historical GitHub Actions collector/tests and health-quality ledger tools. Collector is explicitly Actions-only; it is not a CircleCI collector.
- Restored ADR-0018; updated detector governance, health scanner docs, verification index, CONTRIBUTING, docs index, and `tools/check_contract.py`.
- CircleCI config initially added development-side Python regression tests, native focused tests, and exact-SHA preview scan commands (earlier blob `6631ef2df82c69512417655a752493ffc8fe7d46`). A follow-up corrected the exact-SHA fail-closed logic; latest config blob at the code checkpoint is `750df015567cbba6927eb015e8d4054ffb8ca22d`. Python JSON is checked for invalid/missing/mismatched checkout SHA; native output is checked for expected-SHA mismatch or unavailable HEAD. Ordinary high/critical scanner findings remain report-only at preview stage; execution errors and SHA mismatches fail.

### Current evidence boundary
- Combined GitHub status, check-runs, and check-suites for source SHA `3ce4c0f5821738c0c12a2670e54ab0a3a28edf3f` were empty. This does not prove a CircleCI run; no terminal CircleCI output has been inspected.
- No local Rust/Python execution has been performed in this environment. The latest docs-only commit advanced PR #109 from `3ce4c0f...` to `4aa0198...`; neither SHA has test PASS evidence.
- Historical PR #89/#91 results remain exact-source evidence for their original revisions only.
- Native/Python parity is not claimed. Native scanner remains preview-only, not default/release gate.
- Independent Linux/Windows verification must target the eventual exact integrated SHA.
- Issue #90 remains OPEN; implementation is in child PR #109 and is not yet integrated into PR #108 until an authorized merge of PR #109.

### Next actions without rework
1. Refresh PR #109 and branch HEAD; do not repeat PR #89/#91 historical test runs.
2. Obtain a CircleCI pipeline/job report for the current PR #109 head. If no status appears, verify the CircleCI project/VCS integration through an available authorized connector/CLI; do not ask for raw tokens in chat.
3. Fix the first confirmed test/config/source failure on PR #109 and rerun only affected tests, then full required matrix once on the final integration SHA.
4. Run independent Linux and Windows verification against that same exact SHA.
5. Compare native and Python findings on that exact checkout; document detector coverage and differences.
6. Only after successful review may PR #109 be merged into the PR #108 branch, and only with explicit owner authorization. Do not merge PR #108 to main or release without separate explicit authorization.


### Exact-SHA guard correction — follow-up
The initial preview shell wrapper incorrectly treated exit code 1 as report-only even though both scanners use that code for critical/high findings, including exact-SHA mismatch. This was corrected: Python report JSON is inspected for invalid/missing/mismatched checkout SHA; native output is checked for `expected-sha-mismatch` and `head-unavailable`. The correction was posted to Issue #90 comment `6953106805`. Latest config blob at this point is `750df015567cbba6927eb015e8d4054ffb8ca22d`. This is source readback only; no CircleCI job execution is evidenced.


## 11. Native scanner CircleCI schema heuristic correction — 2026-11-17

During static review of PR #109 native scanner code, the first CircleCI structural rule incorrectly required a top-level `workflows:` key. Official CircleCI documentation describes a valid implicit workflow for a single job named `build`; therefore absence of `workflows:` alone is not an error. The scanner was corrected on source commit `408b6c6adb57c935c34235c9316f8417a421b143`:
- Removed `circleci-workflows-missing`.
- Checks supported top-level versions `2`, `2.0`, `2.1` and required `jobs:` only.
- Replaced the regression test with a valid implicit single-job build fixture, a quoted supported 2.0 fixture, and invalid version/missing jobs assertions.
- Updated detector and scanner docs with the boundary: heuristic checks are not CircleCI config compilation.
- Official reference: https://circleci.com/docs/guides/server/getting-started/introduction-to-yaml-configurations/.

Current PR #109 HEAD after docs and PR-body synchronization: `d15a0aa321cc906d0bc989fe9b517c979631d362`. PR #109 remains OPEN/DRAFT/UNMERGED. This correction has not been compiled or executed locally or in CircleCI. Combined GitHub status/check-runs remain empty; no CircleCI terminal job evidence is available.


## 12. Native scanner isolated from beginner CLI — 2026-11-17

A product-boundary review found that the initial scanner port added `one pindai` / `one pindai keamanan` to the main beginner-facing CLI. This violated the standing constraint that security/repository tooling remain development-side and not expand ONE's beginner vocabulary. Corrected on the PR #109 branch:
- Removed scanner module import, dispatch, and help entries from `crates/one-cli/src/main.rs`.
- Added explicit Cargo target `one-integrity` at `crates/one-cli/src/bin/one-integrity.rs`, sharing the read-only scanner module.
- CircleCI now tests/runs `--bin one-integrity` rather than the main `one` CLI.
- Contract checker enforces separation and rejects scanner import/subcommand leakage into main CLI.
- Updated CONTRIBUTING, ADR-0018, scanner docs, detector parity matrix, PR description, and Issue #90 comment.

The main `one` CLI help and dispatch are restored to the pre-scanner vocabulary. No ONE language grammar, semantics, runtime behavior, or beginner user-guide vocabulary changed. This is static readback only; compilation/tests and CircleCI exact-SHA evidence remain NOT VERIFIED.


### Exact current PR #109 checkpoint after developer-binary separation

- Latest exact HEAD: `94aeb1a336eae2348dfc2910b69f00bd6b00b156`.
- PR #109 remains OPEN/DRAFT/UNMERGED; PR #108 remains OPEN/DRAFT/UNMERGED.
- Changed-file set now includes explicit Cargo target `one-integrity`, its entrypoint, updated CircleCI invocation, and contract guard. Main CLI help/dispatch no longer includes scanner vocabulary.
- GitHub check-runs: 0; combined status: empty. No local `cargo`/`rustc`, no CircleCI CLI/token, no CircleCI terminal logs. No compile/test PASS is claimed.


## 13. Live-head reconciliation after scanner static review — 2026-11-17

The current GitHub branch ref and PR #109 metadata were refreshed together. Both report exact HEAD `b66324a6cca8f72c52a9c2ec8d1af3e7ec8526fb`; PR #109 remains OPEN/DRAFT/UNMERGED and targets PR #108's branch. This supersedes the earlier `94aeb1a...` pointer in §12.

Additional exact-HEAD readback:
- The repository tree is complete (217 tracked blobs, not truncated); all native scanner required foundation paths exist, including `AGENTS.md`.
- The native scanner is an explicit `one-integrity` Cargo binary; `crates/one-cli/src/main.rs` remains free of scanner imports/dispatch/help entries.
- CircleCI runs Python scanner regression suites, Rust focused tests for `one-integrity`, and exact-SHA previews. The GitHub check-runs collection for this SHA is empty; no CircleCI terminal job logs are available.
- Python scanner test file contains 73 test methods including CircleCI config script-path coverage. This is source inspection, not test execution.
- Python and native scanners are not parity-equivalent; native gaps are documented in the parity matrix.

No local Cargo/Python execution was available in this environment. Do not claim build/test success. Docs-only synchronization after this audit will advance the branch head; keep `b66324a...` as the implementation SHA and record the new docs commit separately.


## 14. Live PR #109 exact-head reconciliation — 2026-11-17

- Live PR metadata confirms HEAD `0866a6df8733df092ed287423b2e2d505aa464b2`, branch `ci/scanner-restoration-20261117`, base `ci/circleci-migration-rebased-20261117` (PR #108 branch).
- PR #109 remains OPEN/DRAFT/UNMERGED; PR #108 remains OPEN/DRAFT/UNMERGED. No merge or main-branch change was made.
- Native scanner is a separate `one-integrity` Cargo binary. The main beginner-facing `one` CLI remains free of scanner imports, dispatch, and help vocabulary.
- Exact-SHA GitHub combined status has zero status contexts and GitHub Actions workflow-runs collection is empty. These are GitHub observations, not proof that CircleCI has no pipeline.
- CircleCI terminal job logs and local Rust/Python execution remain unavailable. No compile/test PASS, CircleCI PASS, or Python/native parity claim is made.
- Earlier SHA references in §§11–13 are historical checkpoints and must not be treated as the live HEAD.
- Required next evidence: authenticated CircleCI pipeline/job details for this exact SHA, followed by terminal logs and independent Linux/Windows verification. Do not request raw tokens in chat.
