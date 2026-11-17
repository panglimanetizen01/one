# Forensic Repository Audit — Project ONE — 2026-11-17

**Audit type:** repository-wide documentation, contract, derivative-artifact, CI ownership, exact-SHA evidence, and session-continuity audit.  
**Repository:** https://github.com/panglimanetizen01/one  
**Canonical main snapshot audited:** `d35db95153f6ddd79885494c72f5aee16c899751`  
**Current migration candidate:** PR #108, branch `ci/circleci-migration-rebased-20261117`, based on PR #102 / PR #107 source lineage. Refresh its exact HEAD from live PR metadata before relying on this report.

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

1. `tools/check_contract.py` originally required the three GitHub Actions workflow files. It would therefore fail after the migration deleted those files. It now requires `.circleci/config.yml`, `Cargo.lock`, and the CI verification contract instead.
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

## 7. Immediate next actions

1. Read the updated Notion Skill and this audit record; open the architecture diagram on PR #97 and read the complete documentation map before editing.
2. Refresh live metadata for PR #108, PR #102, PR #107, PR #103, PR #97, and Issues #104/#105. Do not rely on embedded historical head pointers.
3. Obtain CircleCI schema validation and a terminal pipeline/job report for the updated PR #108 exact HEAD. Inspect every job/step; do not infer PASS from a green pipeline context without required job evidence.
4. Integrate the already-PASS PR #98–#101 fixes into one candidate based on the current PR #108 source without force-pushing or merging. Resolve conflicts by preserving normative contracts; do not copy unrelated legacy milestone branches.
5. Implement Issue #104 with race-resistant handle-relative filesystem confinement and adversarial Unix/Windows tests.
6. Continue Issue #105 host-owned Unix containment design. If a robust boundary cannot be implemented portably, explicitly define the supported security contract and retain the limitation; do not weaken tests or claim containment.
7. Revisit Android E3 Issue #82 only with a real device/runtime execution path.
8. After each completed task, update source tests, normative specs, user docs, implementation status, evidence matrix, completion ledger/DoD, and the Notion Skill using exact-SHA evidence. Do not promote scores/checkboxes until acceptance criteria are actually satisfied.

