# Live continuation checkpoint — 2026-11-18

## Exact repository state

- Main repository: `panglimanetizen01/one`.
- Canonical main SHA: `d35db95153f6ddd79885494c72f5aee16c899751` (read live; unchanged).
- PR #108: OPEN / DRAFT / UNMERGED; branch `ci/circleci-migration-rebased-20261117`; exact HEAD `286a0fd563e10611a3465cb6457ab069a0df8a52`; base SHA `6c72d8e548bbbc28d85be39b8de427614989a038`.
- PR #110: OPEN / DRAFT / UNMERGED; branch `security/issue-104-root-handle-design-20261118`; current exact HEAD `26d29c61944e51919161f5360bccfef4a09d370d`; code-bearing parent SHA `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b`; base SHA `c4bb4b690e698a83e4ddaf279596479d3dc282ca`.
- Issue #104: OPEN.
- Independent verifier `panglimanetizen01/one-windows-verification` main SHA: `3d915528e7771d9443ccf0da7e4d7c9e25231537`.

## Exact-SHA CI status

- PR #108: GitHub status context `ci/circleci: one-linux` SUCCESS, status timestamp `2026-11-18T02:00:42Z`, job [99216205-62aa-4e98-af42-d4b742a5feaa](https://app.circleci.com/workflow/a0e85852-848f-4c6b-8681-53438dea6bea/job/99216205-62aa-4e98-af42-d4b742a5feaa).
- PR #110 current HEAD `26d29c61944e51919161f5360bccfef4a09d370d`: GitHub status context `ci/circleci: one-linux` SUCCESS, status timestamp `2026-11-18T04:42:11Z`, job [c153ba39-4073-4548-aaa6-6370e6bed6bf](https://app.circleci.com/workflow/a15e7c3f-0492-4ac6-a025-d8d2fe31f054/job/c153ba39-4073-4548-aaa6-6370e6bed6bf). Parent code-bearing SHA `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b` has the earlier SUCCESS job `c9916871-9746-4c65-b8fb-615d9f96dc78`. The child commit changes docs only; the production source finding remains valid.
- These are GitHub commit-status observations. This session has no CircleCI connector/action, so expanded CircleCI job logs and step-level execution were not retrieved. Do not claim those logs were inspected.

## Issue #104 — confirmed implementation gap

Direct source inspection at PR #110 code-bearing parent SHA `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b` (current HEAD `26d29c6...` is documentation-only):

- `crates/one-core/src/adapter.rs`: `FileReadAdapter::invoke` still calls `std::fs::read(path)`.
- `FileWriteAdapter::invoke` still calls `std::fs::write(path, &content)`.
- Both adapters still expose parameterless `new()` constructors.
- `crates/one-core/src/interpreter.rs` still registers both adapters without a host-injected root authority.
- The PR #110 diff is design/docs plus the existing bounded Unix reader-shutdown and CircleCI config changes; it does not contain a production filesystem handle-relative implementation.

Classification: design documented; production confinement NOT IMPLEMENTED; adversarial Unix/Windows acceptance NOT VERIFIED. A green Linux CircleCI status does not satisfy Issue #104.

Required next implementation gate remains: a working compile/test route on Unix and Windows; then implement one immutable host-injected root authority, handle-relative traversal without path fallback, explicit read/write limits, atomic same-directory writes, positive in-root relative/absolute compatibility, fail-closed unsupported platforms, and nonzero-match adversarial tests. Preserve policy-first/PR #101 behavior and do not change ONE language grammar, semantics, CLI vocabulary, or beginner UX.

## Repository documentation and architecture inspection boundary

- Recursive Git tree at PR #110 HEAD returned 206 tracked files, including 103 documentation files (README plus docs/). Tree response was not truncated.
- The current documentation inventory includes all top-level contracts, specs, user guides, plans/specs, security design, and verification records. This is inventory evidence only; it is not proof that every document's full contents have been semantically reviewed.
- The architecture contract and Issue #104 normative/security documents were read in part for the current source mapping. The architecture PNG `docs/architecture/one-full-architecture-map.png` on branch `docs/system-architecture-diagram-20261115` was not visually inspected because the available GitHub fetch action does not return binary image data and remote image URLs cannot be rendered by the available visual inspection path. Do not claim visual review complete.
- Verifier main tree currently lists `.github/workflows/linux-distro-verification.yml`, `.github/workflows/stdlib-focused-verification.yml`, and `.github/workflows/windows-verification.yml`. This describes verifier main only; inspect the intended active verifier branch before changing its workflow/pin.

## Completed evidence not to repeat

Historical exact-SHA Ubuntu/Windows PASS results for PR #98, #99, #100, #101, #102, #107, #103, and #97 remain tied to the exact SHAs and run IDs recorded in the earlier evidence matrix. Do not rerun those same tests on those unchanged SHAs without a specific technical reason, and do not transfer their PASS results to PR #110.

## Safety and next action

- No merge, main modification, force-push, branch deletion, release, secret change, or verifier pin change was performed.
- Next action: establish a working Unix + Windows source execution route and inspect the current active verifier branch/pinning before implementing Issue #104. Do not commit speculative platform-specific security code without compile/test evidence.
- After implementation, update the normative spec, architecture/security docs, implementation status, evidence matrix, verification record, user feature status, completion ledger/DoD, verifier filters/pin as justified, and this Notion Skill with the exact new source and verifier SHAs.


## Exact-SHA status update — 2026-11-18 04:42:11Z

PR #110 current HEAD `26d29c61944e51919161f5360bccfef4a09d370d` now has GitHub CircleCI context `ci/circleci: one-linux` SUCCESS, job `c153ba39-4073-4538-aaa6-6370e6bed6bf`. This result applies to this exact documentation commit only. Expanded CircleCI job logs were not retrieved. Issue #104 remains OPEN; the source finding is from parent code SHA `19ef80d0ff4fa41ebfc96e9a3164ec96a6ae645b`.


## Current continuation refresh — 2026-11-18

- PR #110 exact HEAD at the start of this refresh: `4a1dbdf08e9235bb070e970dfb59c33cc9a6c549` (docs-only commit); branch remains OPEN / DRAFT / UNMERGED. Its CircleCI `ci/circleci: one-linux` status was PENDING at job `54d76c6c-e85d-4a91-984f-20dec0a0f701` (workflow `524e7b1e-de00-49fd-9a25-69266a2c5fd5`).
- Rechecked `crates/one-core/src/adapter.rs` at `4a1dbdf...`: production file reads/writes still use `std::fs::read` / `std::fs::write`; root-handle confinement is NOT IMPLEMENTED.
- The independent verifier workflow `ONE Fixed-Weight Ledger Gate` has a deterministic input-integrity failure: the canonical pin `e50748a1b25922d9090eba1f5fb90548d7c7d003` lacks `docs/verification/one-completion-ledger.md` and `docs/verification/DEFINITION_OF_DONE.md`. On run `44241822346`, all 15 verifier unit tests passed, then the live gate exited 2 because the ledger file was absent. Run `44241595784` failed at the same missing input after the same 15/15 tests. This is not a Rust source failure and not a valid completion-gate pass.
- Historical versions of the two missing docs exist at source SHA `2ead9dba45db4ce4105deaa81720aacc8fcadd6c`; verifier documentation explicitly prohibits silently substituting those stale copies. The appropriate repair is a reviewed, versioned current-source ledger/DoD and a separate documentation pin from the E4/security candidate pin. No historical scores or acceptance checks were copied, promoted, or altered here.
- The verifier branch current HEAD at refresh was `8e73ff8218b03dde73ad460bcd7745232b8c8a41`; its `LEDGER_GATE.md` records this root cause. No verifier pin or workflow was modified in this continuation.
- This source checkpoint commit is documentation-only. It does not implement Issue #104 or add test evidence.

## Continuation update — 2026-11-18 — independent Linux and Windows baseline PASS

- The Linux-only retry workflow completed successfully on candidate SHA `a20676be53e5f78c465832379d86c3675a2ed7c8`: run [44267886845](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44267886845), job `134676844501`, 239 passed, 0 failed, 1 ignored across 27 test-result summaries.
- The Windows verifier job completed successfully on the exact same candidate SHA: [run 44263674485 / job 134662858839](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44263674485/job/134662858839), 232 passed, 0 failed, 1 ignored across 27 result summaries.
- The successful baseline candidate is PR #108 HEAD at verification time. PR #110 is a distinct stacked branch at `2ab82ea9cccb1c9aa0fef25c3591a1310ab84b5c`; its CircleCI `one-linux` status is SUCCESS. Keep evidence tied to its actual source SHA/provider.
- The original matrix run's Linux job remains stale/in-progress at its old MinGW install step; do not wait on or rerun the already successful Windows job. The separate Linux-only run is the authoritative Linux result.
- Issue #104 is still OPEN / NOT IMPLEMENTED. The production adapter source still calls `std::fs::read(path)` and `std::fs::write(path, ...)`. No filesystem root authority, race-resistant handle-relative operations, size limits, or Issue #104 adversarial tests are present in this candidate. E4 and DoD must not advance.
- Next work: implement Issue #104 only on the current PR #110 branch after establishing a validated runner route for Rust + Unix + Windows native tests. Keep main untouched and all PRs draft/unmerged.

## Reconciliation branch checkpoint — 2026-11-18

- A non-destructive reconciliation branch was created from current base `ci/circleci-migration-rebased-20261117` at exact SHA `a20676be53e5f78c465832379d86c3675a2ed7c8`: `reconciliation/pr-110-base-sync-20261118`.
- The first reconciliation commit imported only PR #110 files that were unchanged on the base since the common merge base, plus the two PR-added design/checkpoint documents. It preserved the base's newer CI migration and release workflow changes.
- The next reconciliation commit starts from that branch and keeps the six dedicated focused test steps and the base printf smoke fixture, adds strict x.y.z toolchain validation, preserves the comprehensive contract checker, and appends PR #110's additive prompt/forensic sections to the newer base documents.
- Current reconciliation candidate SHA: `42997e7ce0701796f045a6d130fa515c295f5971` (this SHA is superseded if a subsequent correction commit is made).
- GitHub compare against the base reports 2 commits ahead / 0 behind; 12 changed files. CircleCI `ci/circleci: one-linux` for this exact SHA is PENDING at the last status read. No Linux/Windows independent verifier result exists yet for this new candidate.
- Canonical `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`. Original PR #110 remains open/draft/unmerged and its source branch has not been changed.
- Issue #104 remains OPEN / NOT IMPLEMENTED. The imported design/specification is not implementation evidence. Do not advance E4 or DoD.

## Static review correction — 2026-11-18

- Readback of candidate `f6cb9639683bab2cf17d9c068fb86ef782c4e18a` caught a double-escaped regular-expression literal in the newly merged toolchain checker before acceptance. The checker was corrected to validate exactly three numeric dot-separated version components using split-based validation, avoiding escape ambiguity.
- Readback also confirms the base `.circleci/config.yml` retains the `printf` smoke fixture and all six focused test steps; an earlier automated string probe used an incorrectly escaped search string and did not reflect the file content.
- The corrected commit must receive fresh exact-SHA CircleCI and independent Linux/Windows verification. No PASS is inferred from earlier candidate SHAs.


## Live checkpoint addendum — PR #113, 2026-11-18 22:xx WIB

This addendum supersedes the PR #110 status above for the current Issue #104 workstream. Historical PR #110 facts remain unchanged.

### Current repositories and source identity

- Main repo: PR #113, branch `security/issue-104-root-confinement-20261118`, OPEN / DRAFT / UNMERGED.
- Latest code-bearing source SHA: `cc54597b471493f4350d0dc0f3a2896babb33ea4`.
- Latest documentation HEAD: `211f6c5283296afc033bd7e7fa1f56e63c84e6c6`.
- PR #113 base remains PR #112 SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Canonical main is unchanged. Issue #104 remains OPEN.
- Independent verifier diagnostic PR #44 branch `probe/windows-obj-dont-reparse-20261118` pins `cc54597...`; protected `verification` branch accepted pin remains unchanged.

### Filesystem implementation state

- Linux reads: retained root descriptor + `openat2` with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`.
- Linux writes: direct children of retained root only; exclusive temporary sibling, write + sync, root-dirfd-relative `renameat`. Existing symlink/hard-link/non-regular leaf and oversize writes fail closed. Nested writes explicitly return UnsupportedPlatform because a nested directory handle can be moved outside the root after open; no nested write is attempted.
- Windows writes: retained native handles, exclusive temp sibling, native `NtSetInformationFile(FileRenameInformation)` same-directory rename; exact E4 PASS at source `f6acb...`. Latest Windows test cleanup fix is source `a796650...`.
- Other platforms: fail closed / NOT IMPLEMENTED.
- Issue #104 is not ACCEPTED; full nested Linux writes, parent/leaf race coverage, failure-injection, and remaining acceptance gates are outstanding.

### Exact-SHA evidence

- Windows/Unix E4 on `20ede9...`: PASS, [run #44329905887](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44329905887).
- E4 on `f6acb...`: PASS, [run #44330374509](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44330374509), including Windows atomic write positive/denied replacement and Unix adversarial suite.
- The full cross-platform run on `f6acb...` found Windows Clippy failure only in test cleanup (`permissions.set_readonly(false)`). Fixed in `a796650...` by using a Windows sharing blocker without `FILE_SHARE_DELETE`; the production backend was not the Clippy failure.
- Linux direct-child write implementation + additional Linux tests are now at `cc54597...`. The verifier E4 contract now expects five Linux `descriptor_relative_write_*` tests and three Linux `descriptor_relative_read_rejects_*` tests, including actual `/dev/shm` mount-boundary read denial.
- Fresh verifier runs pinned to `cc54597...`: E4 [#44332316080](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332316080), cross-platform [#44332316009](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332316009), SEC-05 [#44332315829](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332315829), distro [#44332315865](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332315865), Android artifact [#44332315952](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332315952), Android runtime [#44332316141](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332316141), resource guard [#44332316209](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332316209) were queued/pending at last inspection.
- CircleCI `ci/circleci: one-linux` PENDING for `cc54597...`: [job](https://app.circleci.com/workflow/faea4025-82ee-456f-9848-5b4c903e1838/job/36627b08-5916-445b-bd4a-71bd489c94e1). No CircleCI log connector is exposed; do not infer cause.

### Documentation and architecture

- User guide, Issue #104 design, implementation status, evidence matrix, compatibility contract, security contract, and this live checkpoint have current addenda.
- The full architecture PNG is tracked in still-open PR #97, branch `docs/system-architecture-diagram-20261115`, path `docs/architecture/one-full-architecture-map.png`. PR #97 remains OPEN / UNMERGED. The architecture contract says the image is a non-normative navigation reference. The binary image could not be retrieved through the current GitHub connector, so visual inspection and image-to-code reconciliation remain NOT VERIFIED. Do not claim the diagram has been reviewed or updated.

### Next actions

1. Inspect E4 run #44332316080 logs on exact source `cc54597...`; fix any failures without removing tests.
2. Confirm Linux direct-child positive write, symlink denial, oversize preservation, nested fail-closed, and actual mount-boundary read test results.
3. Inspect cross-platform Clippy and Windows test results on the same exact SHA.
4. Complete Android API 35 runtime, Linux distro matrix, SEC-05, and resource guard on the current candidate.
5. Obtain CircleCI terminal status/logs through a supported authenticated route; no speculative diagnosis.
6. Reconcile remaining parent/leaf race and failure-injection acceptance, then update the normative contract/evidence and user documentation. Keep PR #113 draft and main/protected verifier pin unchanged.


## Live checkpoint addendum — Linux writer compile fix, 2026-11-18

- Current code-bearing source SHA: `090c84dbab02e48af0a493faeb8d4fe0ff80a876`.
- Latest documentation HEAD before this addendum: `5902e130844bd7d1f824fea53e8ec920361b083b`.
- PR #113 remains OPEN / DRAFT / UNMERGED; canonical `main` unchanged; Issue #104 OPEN.
- Linux writer candidate `cc54597...` failed Android AArch64 compile with four unresolved `parent_handle` references. No artifact uploaded. Fixed in `090c84d...` by using the retained root fd consistently. This is a compile failure, not a runtime security failure.
- Diagnostic verifier pin is `090c84d...`; exact-SHA E4 [#44332808624](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808624), cross-platform [#44332808543](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808543), Linux distro [#44332808528](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808528), Android artifact [#44332808717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808717), Android runtime [#44332808569](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808569), SEC-05 [#44332808586](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808586), and resource guard [#44332808575](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808575) are queued/pending.
- CircleCI `ci/circleci: one-linux` PENDING on source `090c84d...`: [job](https://app.circleci.com/workflow/60cfc357-04fc-47e2-8a5c-6cd85784c6af/job/a3279063-9cb6-4c54-8125-9b1b3b89ad8e). No CircleCI logs were inspected.
- Linux write scope remains direct children of root only; nested writes fail closed to avoid a parent-directory move race. Exact-SHA verification is pending.


## Android artifact status update — 2026-11-18

On corrected source SHA `090c84dbab02e48af0a493faeb8d4fe0ff80a876`, Android AArch64 artifact build [#44332808717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808717) is PASS. It verified the exact source pin, generated and inspected ELF64 AArch64 output using `/system/bin/linker64`, and uploaded artifact ID `14438938784` (SHA-256 `5971ef900ead08e6f571327594625ba2856a1917d59196d095c785bdf51a0ea4`). This is not runtime evidence. Android API 35 runtime [#44332808569](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808569) remains queued.


## Strict Clippy correction and diagram source checkpoint — 2026-11-18

- Current code-bearing source SHA: `3d8e0d0e3af54310837eb06d442d30a02df821d0`.
- Latest docs HEAD before this addendum: `7bcd225720e48d03318c745638ac3c494468ebf3`.
- PR #113 remains OPEN / DRAFT / UNMERGED; main unchanged; Issue #104 OPEN.
- Ubuntu distro run #44332808528 failed Clippy only on `clippy::redundant_field_names` in the Linux temp cleanup guard. Fixed by field shorthand in `3d8e0d0...`.
- Diagnostic verifier pin is `3d8e0d0...`; E4 [#44333965599](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44333965599) QUEUED. Prior Android artifact PASS on `090c84d...` is not evidence for the new SHA. Fresh exact-SHA artifact/runtime and platform checks are required.
- A focused Mermaid diagram source now exists at `docs/architecture/filesystem-root-confinement.md`, linked from `docs/ARCHITECTURE.md` and `docs/README.md`. It covers host authority, Linux direct-child writes/nested fail-closed, Windows atomic replacement, and failure cleanup.
- The full overview PNG in PR #97 remains non-normative and visually NOT VERIFIED because the binary image could not be retrieved through the current connector.


## Linux CLI integration alignment checkpoint — 2026-11-18

- Current code-bearing source SHA: `2fa1436daa102d3cf32125da6c440ae19cc83ba7`.
- Debian 12 workspace run #44333980452 on `3d8e0d0...` failed because a stale non-Windows CLI test still expected all Linux writes to be unsupported. Updated test expectations now require direct-child absolute target success and nested relative target fail-closed with original bytes preserved.
- Fresh E4 #44334598268, cross-platform #44334598774, distro #44334598316, Android artifact #44334598355, Android runtime #44334598284, SEC-05 #44334598278, resource guard #44334598250 are queued/pending on the diagnostic pin `2fa1436...`.
- CircleCI one-linux PENDING on `2fa1436...`: [job](https://app.circleci.com/workflow/8f351193-5da8-4b6a-ad2f-75075c6a51e7/job/604385cb-7f5f-4c2f-8a38-92cc8330b83f).
- Focused Mermaid source is present and indexed. Root README now explicitly warns that the source-level filesystem.write slice is not a project-root sandbox; candidate confinement remains unmerged and unaccepted.


## CLI host-root boundary correction — 2026-11-18

- Current code-bearing source SHA: `79a98861080fed2ceaa0a7a2f56e07d6fbc19fca`.
- The CLI no longer derives filesystem authority from the requested target's parent. It selects root from source/project context (nearest `one.toml`, otherwise working directory for sources beneath it or canonical source parent for standalone sources outside cwd).
- Both filesystem.read and filesystem.write use this fixed root. Absolute targets outside root are rejected.
- New tests cover outside-root write (outside sentinel preserved), outside-root read (no disclosure), relative source path, Linux direct-child write, and nested Linux fail-closed behavior.
- Focused Mermaid source updated at `docs/architecture/filesystem-root-confinement.md` to show project-root selection and target-independent confinement.
- Verifier diagnostic pin is `79a9886...`; exact-SHA workflow status must be checked before claiming VERIFIED.


## Manifest-root integration test update — 2026-11-18

- Current code-bearing source SHA: `1b7c3bb1dbc29a33c086b4df21a9996025c1e075`.
- Added cross-platform CLI regression proving the nearest `one.toml` ancestor is the authority root for a source under `src/main.one`; direct-child output under the project root succeeds.
- Diagnostic verifier pin updated to `1b7c3bb...`; fresh exact-SHA runs must be inspected before host-root integration is marked VERIFIED.


## Project-root ancestor lookup correction — 2026-11-18

- Current code-bearing source SHA: `722d662e7a81711ba3607d26571f4048a292b3ed`.
- CLI root resolver now searches all ancestors of the canonical source parent for the nearest `one.toml`, including ancestors above cwd, before falling back to cwd/source parent.
- Diagnostic verifier pin updated to `722d662...`; exact-SHA E4/workspace verification pending.


## Exact-SHA checkpoint refresh — 18 November 2026

The latest code-bearing candidate remains `722d662e7a81711ba3607d26571f4048a292b3ed`. The current PR #113 HEAD is a documentation-only descendant `1eebed322c696fe4f958578a3c7947abce75e173`; the focused architecture diagram now explicitly shows retained Windows root/parent handles opened without `FILE_SHARE_DELETE` to prevent parent-directory rename races during the handle-relative walk and atomic replacement.

The independent diagnostic verifier pin is confirmed as `722d662...`. Fresh exact-source verification is still incomplete: Linux distro run #44335797294 completed SUCCESS: Debian 12 and Ubuntu 24.04 jobs both passed; E4 #44335797344 is queued; cross-platform #44335797200 is pending; Android artifact #44335797320, Android runtime #44335797240, SEC-05 #44335797365, and resource guard #44335797191 are queued. CircleCI `ci/circleci: one-linux` is PENDING on source `722d662...`: https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94.

Classification remains IMPLEMENTED / NOT VERIFIED for the CLI host-root resolver and Linux direct-child writer; Windows handle-relative atomic replacement has historical E4 evidence only on earlier exact source SHA and needs the current exact-SHA run. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 OPEN / DRAFT / UNMERGED; canonical main and protected verifier pin remain unchanged.


## Exact-SHA verification refresh — 2026-11-18

Code candidate: 722d662e7a81711ba3607d26571f4048a292b3ed. The independent diagnostic verifier pin matches this exact source.

- E4 Unix + Windows filesystem adversarial matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797344.
- Windows Server 2025 build 26100 native probe: ordinary file open with OBJ_DONT_REPARSE succeeded; direct and intermediate junctions were denied with STATUS_REPARSE_POINT_ENCOUNTERED (0xC000050B); outside sentinel remained unchanged.
- Seven Windows filesystem regressions passed, including in-root absolute read, outside-root confinement without side effects, normal atomic write, oversized-content rejection, denied atomic replacement preserving original bytes and cleaning the temp sibling, junction escape rejection, and parent traversal rejection.
- Ubuntu + Windows Server 2025 cross-platform matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797200.
- Debian 12 + Ubuntu 24.04 distro matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797294.
- Android AArch64 artifact build: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797320; artifact ID 14442665831; archive SHA-256 3bc37c20a92087464abb087be1fe174fac314dbddd635116656e34ad1cee4123. Android API 35 ARM64 runtime, run #44335797240, remains IN PROGRESS.
- SEC-05 Linux + Windows: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797365. The targeted interop.rs formatting check passed; the separate whole-workspace formatting diagnostic reports diffs but is configured non-blocking.
- Subprocess resource guard: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797191.

CircleCI one-linux on source SHA 722d662 reports FAILURE: https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94. The prior documentation HEAD b94e85db5d83c9a4bec582609405d9c65e4db841 separately reports PENDING: https://app.circleci.com/workflow/a09cd902-a88f-4884-a212-148ba0c6b80c/job/125775a6-33ba-4bac-b5f5-74249b8e0a19. CircleCI logs are unavailable through the current tools; failure cause is unknown and is not inferred.

Classification: host-root resolver, Linux direct-child writer, Windows adapter regressions, E4, cross-platform, distro, SEC-05, and resource guard are VERIFIED on the exact source SHA and stated environments. Linux nested writes remain fail-closed. Windows native probe evidence is limited to Windows Server 2025 build 26100. Android artifact is BUILD-VERIFIED only; Android runtime is not yet verified. CircleCI remains unresolved. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 remains OPEN / DRAFT / UNMERGED; main and the protected verifier pin remain unchanged.


## CircleCI status correction — 2026-11-18

A fresh combined-status read confirms that CircleCI `ci/circleci: one-linux` is FAILURE on the code-bearing source `722d662e7a81711ba3607d26571f4048a292b3ed` (job https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94). The previously recorded documentation HEAD `b94e85db5d83c9a4bec582609405d9c65e4db841` is PENDING, and its descendant documentation HEAD `add130412cf42088ca9171fa2660295f7c30ae65` is also PENDING (job https://app.circleci.com/workflow/bfe4df8d-85a8-49bc-b7f2-21910a4692cd/job/d2aa7f1d-0c7a-4e7b-8ba4-ecaf595674c2). The failure cause on the code SHA remains UNKNOWN because CircleCI logs are inaccessible through the current tools. This correction does not change the exact-source independent verification results or Issue #104 acceptance classification.


## Latest CI/runtime observation — 2026-11-18 23:08 WIB

- Android API 35 ARM64 runtime workflow #44335797240 remains IN_PROGRESS. Build job passed; the runtime job is still in `Boot Android API 35 AArch64 on ARM64 host`; artifact/evidence upload has not started. Runtime is NOT VERIFIED yet.
- CircleCI `ci/circleci: one-linux` on code source `722d662e7a81711ba3607d26571f4048a292b3ed` is FAILURE: https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94.
- CircleCI `ci/circleci: one-linux` on docs HEAD `a57a64ed8552e0fff8a8d78a97d34118cccadc06` has transitioned from PENDING to FAILURE: https://app.circleci.com/workflow/57a85d45-3b99-4eb7-887f-74e8a19b4380/job/9c378131-6ee0-48d7-962e-6581342d13ba.
- CircleCI job logs are not accessible through current tools; failure root cause is UNKNOWN. No speculative fix or blind rerun was performed.
- PR #113 remains OPEN / DRAFT / UNMERGED; Issue #104 remains OPEN / NOT ACCEPTED; canonical `main` and protected verifier pin remain unchanged.
