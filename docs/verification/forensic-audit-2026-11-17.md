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

## CircleCI cache-identity refinement — 2026-11-18

- PR #108 branch: `ci/circleci-migration-rebased-20261117`.
- Change: the `one-linux` Cargo cache primary key now includes the `Cargo.lock` checksum and uses a new `one-cargo-v2` namespace. The save key matches the restore primary key. The previous broad v1 fallback remains available only as a performance fallback; Cargo dependency integrity remains governed by Cargo and all Cargo commands remain `--locked`.
- Normative update: Section 13 of `docs/spec/one-v0.1-ci-verification-contract.md` now requires architecture, workspace manifests, lockfile, and Rust toolchain file in the primary cache identity, and explicitly states that cache hits are not verification evidence.
- Resulting commit: `c968c6a9dc067be85bc1160c95c0a91c555468ea` (documentation commit follows config commit `e91e092e5fca5b1e7ee2f8502b779dd5afd4888c`).
- Validation boundary: GitHub readback confirms both restore/save primary keys include `Cargo.lock` and the same v2 key expression. CircleCI schema validation, pipeline provisioning, and job execution remain NOT VERIFIED because authenticated CircleCI job access is unavailable in this session. No CircleCI PASS is claimed.
- Scope: cache identity only. No source semantics, Rust toolchain version, resource class, runner image, release gate, or provider cutover was changed.

## CircleCI release-workflow deduplication — 2026-11-18

- Config change commit: `a7fc834bb7e88f3fee535d3167ab6b870f658964`.
- The duplicate `release-linux` job was removed. It previously repeated toolchain bootstrap, contract/docs checks, build, Clippy, workspace tests, and release CLI build/smoke already performed by `one-linux`.
- The two workflows are now mutually exclusive: `one-ci` runs when `run_release_checks` is false; `release-check` runs when it is true. Both select the single `one-linux` baseline job.
- Release-only checks remain present as a conditional step block in `one-linux`: real-network runtime tests and `bash tools/verify_offline.sh`. They are not removed or weakened.
- Normative contract update: Section 14 of `docs/spec/one-v0.1-ci-verification-contract.md` requires mutually exclusive workflow selection, one baseline job, and preservation of release-only gates. Contract commit: `d4152f1cf6b8f3b9300575f649824551b408f910`.
- Static review intent: avoid duplicate baseline execution while retaining the same baseline checks and additional release-only gates. No runtime/source semantics changed.
- Validation boundary: GitHub readback and structural inspection only. CircleCI CLI/schema validation and actual conditional workflow execution remain NOT VERIFIED because authenticated CircleCI access is unavailable in this session. No CircleCI PASS is claimed; the exact resulting HEAD must be refreshed after documentation synchronization.


## CircleCI configuration review — Rust toolchain reproducibility — 2026-11-18

- Review found that `.circleci/config.yml` and `rust-toolchain.toml` used floating `stable`, and the bootstrap path could install a default stable toolchain before explicitly installing it again. The rustup bootstrap also had no retry policy.
- Baseline evidence: independent verifier run `44243755612` logs show Rust `1.100.0` on both Ubuntu 24.04 and Windows Server 2025, with 239 Linux tests and 232 Windows tests passing (one ignored on each platform) for source SHA `bac3a2e6f3ba6db5b821b9eafa6c5576f9fa8554`.
- Remediation on the CircleCI candidate: pin `rust-toolchain.toml` to `1.100.0`; bootstrap rustup with bounded retries and no implicit default toolchain; derive the installed toolchain from the repository file; and enforce this contract in `tools/check_contract.py`.
- Required follow-up: set the independent verifier action to the same exact version, advance only its non-acceptance candidate pin to the resulting source SHA, and require fresh Linux/Windows plus CircleCI terminal evidence. Do not transfer the baseline result to the new SHA.


## Toolchain pin fix-up — 2026-11-18

Static readback after the initial toolchain-pin commit found an over-escaped `sed` capture reference and a malformed insertion boundary in the contract checker. Both are corrected in the immediate fix-up commit. The initial commit is not an accepted CI result; only the fix-up SHA may be used for fresh CircleCI and independent matrix evidence.

## Issue #104 filesystem confinement design checkpoint — 2026-11-18

- Created stacked draft PR #110 from current PR #108 exact parent SHA `c4bb4b690e698a83e4ddaf279596479d3dc282ca`; PR #110 head after documentation synchronization is to be refreshed from live metadata.
- Design document: `docs/security/issue-104-filesystem-root-confinement-design.md`. It specifies explicit trusted-root configuration and retained handle authority, Unix descriptor-relative traversal, Windows native handle-relative traversal, no-follow/reparse rejection, device/volume boundaries, hard-link policy, bounded I/O, atomic same-directory writes, and a real-side-effect adversarial test matrix.
- Normative effect/capability contract Section 20 added: path-based check-then-open and canonicalize-then-open are insufficient; policy-first ordering remains unchanged; unsupported platforms must fail closed; feature remains experimental until exact-SHA Unix and Windows tests pass.
- Documentation index links the design checkpoint.
- No product source code or tests were changed or executed. This is design/specification only, not remediation or a security PASS. Issue #104 remains OPEN; E4/DoD unchanged.
- The older branch `security/issue-104-filesystem-confinement-20261118` is stale relative to PR #108 (0 commits ahead, 8 behind at last comparison) and must not be used as the implementation base. PR #110 is based on current PR #108 instead.
- CircleCI validation/log access remains unavailable; no CircleCI PASS is claimed.
- Next implementation step: perform a complete adapter/interpreter/CLI call-site inventory, define the explicit host root configuration API, implement Unix and Windows handle-relative primitives without fallback, then add side-effect/race tests and run the independent Unix/Windows verifier on the same exact source SHA.

### Independent verifier filter contract for Issue #104 — 2026-11-18

The checked-in workflow `panglimanetizen01/one-windows-verification/.github/workflows/e4-adversarial-regression.yml` requires nonzero test execution for these names on the source candidate:
- Unix: 2 `descriptor_relative_read_rejects_`, 1 `descriptor_relative_write_rejects_`, 1 `descriptor_relative_walk_rejects_`, plus exact race and mount-boundary tests `adapter::filesystem_nofollow_tests::descriptor_relative_read_race_never_follows_swapped_parent_symlink` and `adapter::filesystem_nofollow_tests::final_component_mount_boundary_denial_does_not_truncate_file`.
- Windows: `filesystem_reader_rejects_directory_junction_escape`, `filesystem_adapter_reads_absolute_target_inside_project_root`, `filesystem_adapters_confine_absolute_targets_to_project_root_without_side_effects`, `filesystem_write_adapter_writes_text_target`, `filesystem_write_adapter_rejects_oversized_content`, and `filesystem_adapters_reject_parent_directory_traversal`.
- Workflow asserts the named tests execute and pass; zero-match/filtered-only output fails. Preserve these names/counts or review any verifier update separately; do not weaken the checks.

### Mount-boundary design correction — 2026-11-18

A device-number-only check is insufficient to reject same-device bind mounts. The Issue #104 design and normative Section 20 now require mount identity as well as device identity; Linux should use statx STATX_MNT_ID (or a stronger identifier), and other Unix targets must use an equivalent primitive or fail closed. This is a design correction, not implementation evidence.

### Host-owned filesystem authority API contract — 2026-11-18

The Issue #104 design and normative spec now specify a shared immutable authority injected into both filesystem adapters, explicit max read/write byte limits, and a dedicated interpreter constructor. The CLI may use current_dir as a trusted project root only when obtained independently from the program target. Library callers and tests must inject a root explicitly. No parameterless production adapter or legacy interpreter path may infer authority from the target, source, or ambient environment. This remains a design contract; source implementation and tests are not yet present.

### Historical PR #80 filesystem implementation review — 2026-11-18

PR #80 exact head d75b2846465ba53281198693750f2e6ba39e2e33 was reviewed as reference only. Its Unix code contains useful openat/O_NOFOLLOW, statx mount-id, regular-file and hard-link checks, and race-test patterns. It is not accepted as-is: root is recomputed from cwd rather than injected/retained; root traversal is reopened per operation; non-Linux Unix falls back to st_dev; writes are not atomic temp+rename; Windows remains path-based canonicalize/check/open. Current Issue #104 design explicitly rejects copying those gaps.

### Independent E4 verifier run diagnosis — 2026-11-18

GitHub Actions run `43998206030` was inspected at job-log level. Unix job `133755415781` and Windows job `133755416047` both failed before executing the first filtered test because `cargo test --locked` could not create `source/Cargo.lock`. The verifier repository's `CANONICAL_COMMIT.txt` pointed to `e50748a1b25922d9090eba1f5fb90548d7c7d003`, which has no Cargo.lock. Current PR #108 candidate `c4bb4b690e698a83e4ddaf279596479d3dc282ca` does contain Cargo.lock. Classify the historical failure as stale verifier candidate/lockfile precondition, not as a source test failure or PASS. Do not update canonical candidate to current source until the Issue #104 test filters are actually implemented; otherwise the verifier will fail on zero matches. The verifier branch was not changed.
### Issue #104 source integration map — 2026-11-18

Static source readback on PR #108 base SHA `c4bb4b690e698a83e4ddaf279596479d3dc282ca`:
- `adapter.rs`: filesystem adapters have metadata-only state, parameterless `new()`, and call `std::fs::read` / `std::fs::write` using `ExecutionScope::Target`. Write accepts one Text argument; read accepts none.
- `interpreter.rs`: `with_orchestration_policy` is the central adapter registration point and currently constructs both filesystem adapters without authority.
- `one-cli/src/main.rs`: CLI parses `--target` into the capability scope but does not independently capture/inject a trusted cwd root.
- `lib.rs`: public adapter re-exports need deliberate exports for the new authority/limits.
- `one-core/Cargo.toml`: libc and windows-sys 0.61 are already target dependencies; check exact API availability before adding dependencies.
- Existing adapter unit tests and `filesystem_vertical_slice.rs` directly instantiate no-root adapters and must migrate to explicit temp roots while preserving policy/scope assertions. CLI build/package direct file operations remain separate host operations.

Implementation order: shared immutable ProjectFilesystem + limits; Unix/Windows handle-relative primitives; adapters; dedicated interpreter constructor; host-owned CLI root injection; adversarial race/side-effect tests; same-SHA Unix/Windows validation. This is only a static integration map.

Execution blocker: no authenticated CircleCI tool or GitHub Actions dispatch action is exposed in this session, and the local clone route previously failed DNS resolution. Do not write uncompiled security-sensitive platform code. No source code changed; Issue #104 remains OPEN.

