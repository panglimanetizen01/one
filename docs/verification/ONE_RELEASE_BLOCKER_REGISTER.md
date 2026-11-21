# ONE RELEASE BLOCKER REGISTER

**Status:** ACTIVE — single release-sprint work queue  
**Last verified:** 2026-11-21 (implementation-first refresh)  
**ONE PR:** [#113](https://github.com/panglimanetizen01/one/pull/113)  
**Verifier PR:** [#51](https://github.com/panglimanetizen01/one-windows-verification/pull/51)

> This register controls priority only. It does not replace or modify the release contract, specifications, fixed acceptance ledger, Definition of Done, or evidence matrix. Those remain authoritative for acceptance. Update this register instead of creating another blocker list or duplicate roadmap.

## 1. Last verified identity — refresh before acting

| Item | Exact value |
|---|---|
| ONE canonical `main` | `d35db95153f6ddd79885494c72f5aee16c899751` |
| PR #113 branch | `security/issue-104-root-confinement-20261118` |
| PR #113 HEAD at pre-register refresh read | `20d16862bc7f51d0a5122884eb31223fc1b7bc7d` |
| PR #113 base SHA | `181ad350d8f0db53cda654c4d378425fbdd6ab24` |
| Code-bearing source | `4c01b6eb2b8ba2d75a316a1a792352657a74c73a` |
| Verifier PR #51 HEAD | `7c3ded0d811fe71bd14034880b24b0f8e396349e` |
| Verifier branch-local source pin | `c4f3800bc12b70a4d59d72fbc5a80029cb053cea` |
| Protected verifier source pin | `722d662e7a81711ba3607d26571f4048a292b3ed` |

PR #113 and #51 were OPEN / DRAFT / UNMERGED at the last readback. Refresh all live refs, issue states, status contexts, and provider results before using this snapshot.

## 2. Active blockers

### B-001 — CircleCI pre-execution termination

**Priority:** P2 · **State:** PARTIAL — exact-SHA workflow/job success on `979973cd...`; historical pipeline #1201 root cause UNKNOWN; retain as non-active follow-up  
**Affected candidate:** PR #113 exact HEAD `509b7b197115adb1ba80d730342e9f6fbf981645`

| Run | Pipeline | Workflow | Job | Pipeline revision | Job start |
|---|---:|---|---|---|---|
| Current | #1201 | `41becc6d-b56e-45e8-b273-629ba08c4390` | #836 / `9dbc05b4-a601-4fbc-b6a4-61f64ecc8e9a` | `509b7b197115adb1ba80d730342e9f6fbf981645` | `started_at=null` |
| Previous | #1199 | `916983e9-dbde-4db2-a967-438a666ebb19` | #834 / `a130c5d9-c078-4c18-8ae5-571470f46d60` | `a0d94bc...` | `started_at=null` |
| Earlier | #1198 | `5ed6d9e7-976e-4322-b7ba-6deea915e1a9` | #833 / `8d2d756e-9297-4a00-9a7d-a0c9bd734cba` | `5daf333...` | `started_at=null` |

**Exact #1201 mapping and observations**
- Webhook trigger: `2026-11-20T09:47:02.580Z`; pipeline created `09:47:02.849Z`; pipeline state `created`.
- Workflow `one-ci` created `09:47:03.040Z`, stopped `09:47:03.544Z`, status `failed` (about 0.5 seconds).
- Job `one-linux` #836 status `failed`; `started_at=null`, stopped `09:47:03.544Z`, no duration, no dependencies.
- API v3 exposes step #101 `CANCELED` / `Error: Task information unavailable`; stdout/stderr are empty.
- Pipeline config endpoint returns source and compiled configuration strings; setup-config fields are absent. Source config SHA-256: `b7827cd95925851482b833a5f45033e1b26ef0f731857b29e2df0821e08fd155`; compiled config SHA-256: `9f3f77d48351a10c3222a4a1b79634d30de29038c562c9c0e548046026b85aee`.
- Compiled config includes `one-linux`, machine image `android:default`, resource class `large`. Config payload presence does not prove executor provisioning or job start.
- Diagnostic script classifier has a false positive: it scans the report's own explanatory phrase `failed/timed_out`, then labels it `TIMEOUT / NO-OUTPUT`. This is not timeout evidence; actual evidence remains canceled step/no job start.

**CircleCI account/settings evidence — 2026-11-20**
- Project Advanced screenshot: **Free and Open Source ON**, **Block all new work OFF**, **Only build pull requests OFF**, dynamic config ON.
- Plan screenshot: Free plan; usage snapshot shows 110,089 of 30,000 regular credits used. The page explicitly states OS project contributions are not subject to the 30,000-credit limit and advertises up to 400,000 monthly credits for Linux open-source builds.
- Screenshot does not show remaining/consumed balance for the separate 400,000 OSS allocation. An earlier provider error was recorded as `free-plan-no-credits-available`; screenshots do not prove whether the OSS allocation was exhausted or whether that error still applies.
- Personal API token authentication is valid (`/api/v2/me` HTTP 200); project metadata read is HTTP 200. Project settings API returns HTTP 403 Permission denied for both `gh` and `github` slugs. Do not interpret this as OSS OFF.
- Do not buy credits, upgrade, toggle settings, or retry CircleCI while the recorded no-credit provider block remains unresolved.

**Classification:** confirmed pre-execution termination / no job start. It is NOT a test timeout, test assertion failure, compiler failure, or source regression. Root cause remains UNKNOWN; credit eligibility/allocation remains unresolved.

**Resolution evidence — 2026-11-20**
- Exact tested source SHA `6ef875291b599ef144dba5333e9a40c455f3e013` has GitHub combined status `ci/circleci: one-linux = success`, target workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`.
- Compare from original failed SHA `509b7b197115adb1ba80d730342e9f6fbf981645` to tested SHA shows changes only in the three existing verification documents; no runtime source or CI workflow files changed. Thus the same code/workflow state subsequently received a successful CircleCI status.
- The pre-execution failure is resolved operationally for this candidate code/workflow state. The provider's internal cause for historical pipeline #1201 remains UNKNOWN.
- Latest PR #113 HEAD `7a9ac7ae720eba3b8f64c1bfa46feda3927dd410` adds only further changes to the same three docs beyond the tested SHA. CircleCI has no status entry for that newer docs-only HEAD.
- GitHub Actions run #2247: Windows job SUCCESS; Ubuntu job remains IN_PROGRESS at `Install cross-compilers (Ubuntu)`; all subsequent Ubuntu steps pending. No full matrix PASS is claimed.
- Free and Open Source remains ON per user's confirmed UI observation. Settings API 403 is an access limitation, not evidence of OFF.

**Next action:** retain historical #1201 evidence; do not rerun or change CircleCI config for this operationally resolved blocker. Continue tracking the separate Ubuntu GitHub Actions job and remaining Issue #104 acceptance blockers.

**Separate GitHub Actions bootstrap route**
- PR #115 `ci/bootstrap-actions-on-fix-windows-verifier-20261120` is OPEN / NOT MERGED; base remains `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Initial run #2243 failed on both OS jobs because the PR #113 contract checker rejected the newly introduced `.github/workflows/ci.yml`; Build/Clippy/Tests did not run.
- Corrective workflow commit on PR #115: `3b684acd1f3e68f1b40dd6d600fad0e013044495`, blob `35015ded4f7635ea37ec2b035f2ba0c33d7d4bf5`. It preserves merge-result checkout for normal PRs, uses exact PR head checkout for the stacked base branch, and skips only the contract check on the bootstrap PR itself. Full contract check remains enabled for PR #113.
- GitHub Actions run #2244 (`44748865610`) is terminal SUCCESS on exact PR #115 HEAD `3b684acd1f3e68f1b40dd6d600fad0e013044495`. Ubuntu and Windows jobs both completed SUCCESS; Build, Clippy, workspace tests, and all focused tests passed. The bootstrap PR's Contract artifacts step was intentionally skipped; it is not claimed as executed on this bootstrap branch. [Run #2244](https://github.com/panglimanetizen01/one/actions/runs/44748865610).
- CircleCI status on the same PR #115 SHA is FAILURE at workflow `e6e6a97c-2cda-4a7a-a5ff-3b0c9129d9ec`, job `667d82b1-c817-41f3-91f2-5bd09b55e879`; provider logs are inaccessible in this session, so root cause is not inferred. This does not change the separate B-001 mapping for PR #113 pipeline #1201.
- PR #113 remains OPEN / DRAFT / UNMERGED at `509b7b197115adb1ba80d730342e9f6fbf981645`; base remains `181ad350d8f0db53cda654c4d378425fbdd6ab24`.

### B-002 — Issue #104 security acceptance

**Priority:** P0 · **State:** OPEN / NOT ACCEPTED  
**Latest source/test candidate:** `74df3aa901732c8eb3adaac728162715fc3a4961` (NOT BUILT/TESTED); PR #113 HEAD at this register update is recorded below.

Exact-source verifier evidence remains bound to its own historical source revisions; none transfers to this candidate.

Current implementation boundaries:
- Linux reads use retained-root `openat2`; direct-child writes use atomic replacement; nested writes use detached-mount + `mount_setattr(NOSYMFOLLOW)` + `openat2`, failing closed when required kernel/security primitives are unavailable.
- Windows retains the native handle-relative/reparse-point backend.
- Android nested reads use kernel `openat2` with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`, followed by regular-file, single-link, device, and `STATX_MNT_ID` checks. Nested writes now route through the detached recursive mount transaction shared with Linux: `open_tree` + `mount_setattr(MOUNT_ATTR_NOSYMFOLLOW)` + `openat2` + `renameat2`, with fail-closed behavior when kernel/SELinux policy denies required primitives. Direct-child writes retain the statx mount-ID checks.
- macOS nested reads now use XNU `O_RESOLVE_BENEATH | O_NOFOLLOW_ANY | O_UNIQUE`, then verify regular-file, single-link, and `fstatfs` filesystem-ID + mount-point identity. Unsupported kernels fail closed. Direct-child atomic writes remain supported; nested writes remain fail-closed because a race-safe atomic publication primitive that also excludes mount crossings is not yet proven.
- The Linux nested-write test hook is now forwarded to the detached-mount helper instead of being replaced by a no-op.
- Android/macOS hard-link, nested-path, and side-effect regressions remain NOT RUN. No build or platform support is claimed.
- Verifier success does not itself constitute Issue #104 acceptance.

**Next action:** continue closing implementation gaps against the unchanged Issue #104 contract. Keep Issue #104 OPEN / NOT ACCEPTED until exact-SHA Unix and Windows adversarial execution proves actual side effects and race confinement. Do not alter criteria, fixed ledger weights, protected verifier pin, or DoD checkboxes.

### B-003 — Formal v0.1 release gates

**Priority:** P1 · **State:** OPEN / NOT RELEASE-APPROVED**

The v0.1 release contract, completion addendum, fixed ledger/DoD, and E1–E6 evidence remain authoritative. Reconcile every remaining gate against its exact-SHA evidence. Do not infer release readiness from a subset of green workflows or documentation completion. Issue #105 remains an open engineering item; classify it as release-blocking only if the normative contract/ledger says so.

**Next action:** produce a gate-by-gate closeout from existing records. Do not change gate weights or acceptance rules.

### B-004 — Architecture and documentation traceability

**Priority:** P2 · **State:** PARTIALLY VERIFIED**

- Existing Mermaid source: [filesystem-root-confinement.md](../architecture/filesystem-root-confinement.md).
- Full overview image remains in draft [PR #97](https://github.com/panglimanetizen01/one/blob/docs/system-architecture-diagram-20261115/docs/architecture/one-full-architecture-map.png); its binary has not been visually inspected in this session.
- Do not recreate/re-upload a duplicate diagram. Update existing Mermaid source only if architecture changes, then synchronize links and evidence.

### B-005 — Issue #105 non-escapable process containment

**Priority:** P1 · **State:** OPEN / NOT ACCEPTED
**Latest code-bearing source/test candidate:** `b4b4b3ee82e013eb3c001606820deaa76d50a9df` (NOT BUILT/TESTED); it includes the prior `waitid(WNOWAIT)` cleanup hardening from `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` and adds explicit 2 MiB aggregate stdout/stderr capture enforcement alongside 1 MiB per-stream limits. PR #113 HEAD before this register sync: `d5ddd7f39bbc54a7c0e88db907dfae5f8c49afc5`.

- Linux and Android process startup now install a child-inherited seccomp filter before exec. It denies `setsid`, `setpgid`, `unshare`, `setns`, namespace-bearing `clone` flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` to permit compatible fallback. Process-group cleanup is intended to cover descendants on normal completion, timeout, and output overflow.
- Filter setup failure is fail-closed. Windows Job Object containment is unchanged. ProcessAdapter advertises Linux, Android, and Windows; macOS remains fail-closed until equivalent containment exists.
- Three adversarial `setsid` tests and bounded-reader regression are target-gated for Linux and Android, covering normal completion, timeout, and output overflow. They have NOT been run.
- Acceptance requires reviewing the seccomp architecture/syscall policy and executing the adversarial cases on supported Linux architectures and a real Android/Termux target, proving no descendant survives all three paths. Reconcile existing Windows Job Object evidence. Do not claim process-tree containment or close Issue #105 before that evidence.

## 3. Release sprint execution order

1. **P0:** Complete Issue #104 production filesystem implementation against its unchanged acceptance contract; preserve fail-closed behavior for unsupported nested operations.
2. **P1:** Complete Issue #105 Linux/Android/Windows process containment implementation; macOS process execution remains fail-closed until equivalent containment exists.
3. **P1:** Reconcile every remaining formal v0.1 product/release gate from the existing contract and fixed ledger. Do not reopen B-001 absent a new relevant provider failure.
4. **P2:** After implementation scope is complete, freeze one release-candidate SHA and run the required integrated platform matrix. Inspect terminal job results/logs. Parent-SHA evidence never transfers.
5. **P2:** Synchronize only affected existing docs and Notion Skill; preserve historical evidence and exact-SHA boundaries.
6. **P3:** Prepare release artifacts/notes. Merge, tag, publish, and distribute only after all gates pass and the owner explicitly authorizes it.

## 4. Connected documentation ecosystem

- [Agent operating contract](../../AGENTS.md)
- [Documentation map](../README.md)
- [v0.1 release contract](../release/one-v0.1.md)
- [Completion roadmap/addendum](../superpowers/plans/2026-10-15-one-v0-1-completion-addendum.md)
- [Implementation plan](../superpowers/plans/2026-10-15-one-v0-1-implementation.md)
- [Evidence matrix](one-v0.1-evidence-matrix.md)
- [Implementation status](one-v0.1-implementation-status.md)
- [Verification index](README.md)
- [Live checkpoint](live-checkpoint-2026-11-20.md)
- [CircleCI diagnostics](CIRCLECI_DIAGNOSTICS.md)
- [Existing Mermaid architecture](../architecture/filesystem-root-confinement.md)
- [Changelog](../../CHANGELOG.md)
- [Compact master prompt](MASTER_CONTINUATION_PROMPT.md)
- [Notion continuity Skill](https://app.notion.com/p/407a9d23afaa813cb942d4d6a4d1c5ce)

The register is the one work queue; the linked documents remain their own authorities. Do not fork their responsibilities into duplicate files.

## 5. Closure and safety rules

Close a blocker only when its acceptance condition has direct evidence, exact source/verifier SHA and platform/job references are recorded, affected docs are synchronized, and the Notion Skill is refreshed. A plan, queued workflow, job that never started, or parent-SHA PASS is not closure.

Never merge PRs, modify `main`, force-push, delete branches, promote protected pins, alter fixed ledger/DoD/acceptance criteria, weaken security boundaries, claim unsupported platform support, expose tokens/secrets, purchase hosted CI capacity, or publish a release without explicit authorization.


## 2026-11-20 — Authorized CI bootstrap merge and follow-through

- User authorized continuation after being asked whether PR #115 could be merged.
- PR #115 was re-read immediately before merge: OPEN, ready-for-review (not draft), mergeable=true, exact head `3b684acd1f3e68f1b40dd6d600fad0e013044495`, base branch `fix/windows-process-verifier-timeouts-20261118` at `181ad350d8f0db53cda654c4d378425fbdd6ab24`; Ubuntu and Windows GitHub Actions jobs both terminal SUCCESS. CircleCI status on PR #115 was FAILURE, provider logs unavailable; this limitation was visible before the authorized merge.
- PR #115 successfully merged using a merge commit with expected-head-SHA guard. Merge commit: `3adace165890bb69de346880e3c612c5cd6c2ff8`. The merge adds only `.github/workflows/ci.yml` (87 lines); no runtime source or verifier pin changes. [Merge commit](https://github.com/panglimanetizen01/one/commit/3adace165890bb69de346880e3c612c5cd6c2ff8).
- Base branch now contains the workflow; comparing its previous base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24` to the branch shows exactly one changed file, `.github/workflows/ci.yml`, and three commits ahead (bootstrap implementation, correction, merge commit).
- PR #113 remains OPEN / DRAFT / UNMERGED. Its previously recorded head before follow-up documentation sync was `673c057421ad3a3293dab990563e7f83cd7f3681`; its base is the same verifier branch. After this merge, no GitHub Actions PR run was yet returned for PR #113's recorded head or the merge commit at the immediate readback. CircleCI on the merge commit is PENDING at `https://app.circleci.com/workflow/509828d0-3d7d-434a-8f00-a58645eabe9e/job/83e5ce02-f0ee-4d30-a0df-5dbfe5c77fc6`.
- PR #113's previous CircleCI status remains PENDING at `https://app.circleci.com/workflow/97d5e3d0-c7ff-4668-a5ee-569e4726febc/job/5c1135d8-d3dd-4d7c-a564-14478bc2508a`. Do not claim B-001 resolved until a job starts and exact-SHA evidence verifies the run.
- Next: update only the existing verification docs on PR #113 branch (which should now trigger the newly installed PR workflow), then inspect exact run and require the full Contract artifacts, Build, Clippy, workspace tests, and focused tests on both Ubuntu and Windows. Keep PR #113 draft/open and do not merge it. CircleCI root cause remains unknown; no blind rerun or credit purchase.


## Post-merge verification checkpoint — 2026-11-20

- PR #115 is MERGED; merge commit `3adace165890bb69de346880e3c612c5cd6c2ff8`. Its diff adds only `.github/workflows/ci.yml`.
- Follow-up documentation commits on PR #113 branch:
  - register update `02fafe353a6a57627037d4d88df8966ba0c0e916`
  - live checkpoint update `ff0d3ded5233b819e66375ff14471d8baa320171`
  - CircleCI diagnostics update / latest PR #113 HEAD at last readback `9d9e4ac785c18194f9821156c0c56e9b0c08a3da`.
- PR #113 remains OPEN / DRAFT / UNMERGED at exact HEAD `9d9e4ac785c18194f9821156c0c56e9b0c08a3da`. The workflow file is now present on its base branch. The GitHub connector's exact-commit workflow-run lookup returned no PR-triggered Actions run for this SHA; do not claim GitHub Actions verification started or passed.
- CircleCI on PR #113 latest HEAD is PENDING: workflow `418906bd-0e0e-4f86-aac5-df6042c21e61`, job `c6073561-4718-49be-afe7-aa686cf3589a`. CircleCI on base merge commit is also PENDING: workflow `509828d0-3d7d-434a-8f00-a58645eabe9e`, job `83e5ce02-f0ee-4d30-a0df-5dbfe5c77fc6`.
- B-001 is NOT CLOSED. The earlier pre-execution failure on pipeline #1201 remains historically valid; the current pending statuses are not proof of job start or resolution. The 400,000 OSS allocation balance remains unknown.
- No PR #113 merge, no main update, no protected verifier pin change, and no fixed ledger/DoD/acceptance change.


## PR #113 GitHub Actions run #2247 — 2026-11-20

- Exact candidate HEAD: `6ef875291b599ef144dba5333e9a40c455f3e013`. GitHub Actions run #2247 / `44750302610` is QUEUED then RUNNING on this exact SHA; [run #2247](https://github.com/panglimanetizen01/one/actions/runs/44750302610).
- Ubuntu job `136336114373`: runner started, correct exact-head checkout succeeded, Android SDK setup succeeded; currently in `Install cross-compilers (Ubuntu)`. Contract artifacts, Build, Clippy, workspace tests, and focused tests have not yet started.
- Windows 2025 job `136336114808`: runner started, correct exact-head checkout succeeded, Android SDK setup succeeded; currently in `Install cross-compilers (Windows)`. Contract artifacts, Build, Clippy, workspace tests, and focused tests have not yet started.
- Both jobs are still IN_PROGRESS with no failed steps at latest readback. Do not claim any test pass yet.
- CircleCI on this same PR #113 SHA remains PENDING: workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. B-001 remains open until exact CircleCI job start/outcome and cause are established.
- PR #113 remains OPEN / DRAFT / UNMERGED. PR #115 remains merged at `3adace165890bb69de346880e3c612c5cd6c2ff8`.


## Run #2247 progress refresh — 2026-11-20 (latest readback)

- PR #113 remains OPEN / DRAFT / UNMERGED. Current PR HEAD: `796831e9636c4aa322b4d30c4e96796f62bb45ff`. The exact source SHA being executed by GitHub Actions run #2247 / `44750302610` is `6ef875291b599ef144dba5333e9a40c455f3e013`; comparison confirms the three commits between these SHAs modify only the three existing verification documents (10 insertions each), no source or workflow files.
- Ubuntu job `136336114373`: exact checkout and Android SDK setup PASS; `Install cross-compilers (Ubuntu)` remains IN_PROGRESS. Contract artifacts, Build, Clippy, and Tests have not started on Ubuntu.
- Windows 2025 job `136336114808`: exact checkout, Android SDK setup, Windows cross-compilers, Android NDK, and Toolchain PASS; `Contract artifacts` PASS; Build IN_PROGRESS. Clippy and Tests are pending. No failed steps at latest readback.
- Run #2247 remains IN_PROGRESS; do not claim overall PASS. [Run #2247](https://github.com/panglimanetizen01/one/actions/runs/44750302610).
- CircleCI on exact run SHA `6ef875291b599ef144dba5333e9a40c455f3e013` remains PENDING at workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. CircleCI job page is inaccessible via current browser session; no job start, result, or root cause can be inferred. B-001 remains OPEN / ROOT CAUSE UNKNOWN.
- PR #115 is MERGED at `3adace165890bb69de346880e3c612c5cd6c2ff8`. The merge added only `.github/workflows/ci.yml` to the verifier base branch.
- Prior historical snapshots above are not current status; this progress refresh supersedes them. No PR #113 merge, main change, verifier pin change, or acceptance/ledger/DoD change.


## Run #2247 progress update — Windows build/clippy passed — 2026-11-20

- Run #2247 / `44750302610` remains IN_PROGRESS on source SHA `6ef875291b599ef144dba5333e9a40c455f3e013`.
- Windows 2025 job `136336114808`: checkout, Android SDK, Windows cross-compilers, Android NDK, Toolchain, Contract artifacts, Build, and Clippy are SUCCESS; Tests are IN_PROGRESS. All focused tests are pending. No failed step.
- Ubuntu job `136336114373`: checkout and Android SDK are SUCCESS; `Install cross-compilers (Ubuntu)` remains IN_PROGRESS. Contract artifacts, Build, Clippy, and Tests are pending. No failed step.
- The job log-download endpoint returned BlobNotFound/404 for both running jobs; this is a log retrieval limitation, not evidence that the jobs failed. Step summaries remain available and show the states above.
- Exact current PR #113 HEAD is `07745db32bb1051cc02e602c64f27bc14649238b`. Comparing from tested SHA `6ef875291b599ef144dba5333e9a40c455f3e013` shows six commits ahead, with only the three verification documentation files modified (21 insertions each; no source/workflow changes). The run therefore covers the same source/workflow state as the current PR head.
- CircleCI on tested SHA remains PENDING: workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. B-001 remains OPEN / ROOT CAUSE UNKNOWN.
- PR #115 remains MERGED; PR #113 remains OPEN / DRAFT / UNMERGED. No source change, main change, verifier pin change, or acceptance/ledger/DoD change.


## Run #2247 — Windows matrix leg terminal PASS — 2026-11-20

- Windows Server 2025 job `136336114808` is terminal SUCCESS. All workflow steps succeeded: checkout, Android SDK, Windows cross-compilers, Android NDK, Toolchain, Contract artifacts, Build, Clippy, workspace Tests, deterministic .oneir/.onepkg, DNS classification, orchestration policy, database vertical slice, HTTP GET vertical slice, HTTP GET CLI vertical slice, and cleanup. Job logs were retrieved and show the focused database tests (9/9), HTTP GET vertical slice tests (3/3), and HTTP GET CLI tests (3/3) passing.
- Ubuntu job `136336114373` remains IN_PROGRESS in `Install cross-compilers (Ubuntu)`; all later stages are pending. No failure is recorded. Its log download currently returns BlobNotFound/404 while the job is active, so no cause for the long-running step is inferred.
- Run #2247 / `44750302610` remains IN_PROGRESS overall until Ubuntu completes. [Run #2247](https://github.com/panglimanetizen01/one/actions/runs/44750302610).
- Tested source SHA remains `6ef875291b599ef144dba5333e9a40c455f3e013`; latest PR #113 HEAD `70409c742d2707b1f277b5185787e2e7bdb7184f` differs only in the three existing verification docs (21 insertions each, no source/workflow changes).
- CircleCI on tested SHA remains PENDING at workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. B-001 remains OPEN / ROOT CAUSE UNKNOWN.
- PR #115 MERGED; PR #113 OPEN / DRAFT / UNMERGED. No main/source/verifier pin/acceptance/ledger/DoD change.


## CircleCI status correction — exact tested SHA success — 2026-11-20 21:17 WIB

- Fresh GitHub combined-status readback for exact source SHA `6ef875291b599ef144dba5333e9a40c455f3e013` now reports context `ci/circleci: one-linux` = `success`, target workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. This supersedes the earlier PENDING observation for that exact SHA. The CircleCI status check is successful; detailed CircleCI pipeline/job metadata and underlying log were not independently retrieved in this session, so do not infer details beyond the successful check.
- Current PR #113 HEAD at readback: `441df4590ba6d356671408371a5fc5b87739f4a7`. Comparing tested SHA to current HEAD shows 12 documentation-only commits touching only the existing `ONE_RELEASE_BLOCKER_REGISTER.md`, `live-checkpoint-2026-11-20.md`, and `CIRCLECI_DIAGNOSTICS.md` (42 insertions each; no runtime source or workflow files). GitHub combined-status query for current docs HEAD returned no status entries. Therefore CircleCI success is proven for tested source SHA, not a new CircleCI check on latest docs-only HEAD.
- GitHub Actions run #2247 / `44750302610`: Windows job `136336114808` terminal SUCCESS; Ubuntu job `136336114373` remains IN_PROGRESS at `Install cross-compilers (Ubuntu)`; all following Ubuntu steps pending. Run remains IN_PROGRESS overall, with no recorded failure. Do not claim full matrix success.
- PR #113 remains OPEN / DRAFT / UNMERGED. PR #115 remains MERGED at `3adace165890bb69de346880e3c612c5cd6c2ff8`. No merge, main change, source/workflow modification, verifier pin change, or acceptance/ledger/DoD change.


## Live exact-SHA refresh — 2026-11-20 (B-001 / B-002 / release-gate reconciliation)

This is the newest readback. Preserve all earlier snapshots as history; this block supersedes only conflicting live-state claims.

### Repository and pull requests
- ONE PR #113: OPEN / DRAFT / UNMERGED; exact HEAD `7296c573cbd7eab4a1d7ff08bced2b397a5c555f`; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. Compare from code-bearing source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` shows 48 commits and documentation-only changes across 18 existing docs paths; no runtime source, test, or CI workflow path changed.
- PR #115 is MERGED at `3adace165890bb69de346880e3c612c5cd6c2ff8`.
- Verifier PR #51: OPEN / DRAFT / UNMERGED; HEAD `6c0979e62eb01e4b929c48b4224a3d4f742bc8af`; branch-local `CANONICAL_COMMIT.txt` pins ONE source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. Compare from verifier HEAD `226360d00bc26f858128d3127591fce13ecd52d9` is three documentation-only commits (CANDIDATE_VERIFICATION.md, MIRROR_STATUS.md, README.md, Android evidence note); no workflow/source implementation change.
- Protected verifier branch remains `verification` at `b14abf0020472dc45842be30854e0a09317d6cb4`; protected canonical source pin remains `722d662e7a81711ba3607d26571f4048a292b3ed`. Protected `LEDGER_SOURCE_COMMIT.txt` remains `d6c382a245c49a028bc60809f22ca956c7f894cc`. No protected pin was changed.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`; no main mutation.

### B-001 — CircleCI pre-execution termination
- Historical pipeline #1201 on `509b7b197115adb1ba80d730342e9f6fbf981645` remains a confirmed pre-execution termination; provider root cause UNKNOWN.
- Code-bearing ONE source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` has GitHub combined status `ci/circleci: one-linux = SUCCESS`, workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`. Provider step logs were not inspected; claim only exact-SHA status success.
- Current PR #113 HEAD `7296c573...` has no combined-status entries. Its predecessor docs SHA `a0d94bc9a26cf9effd4b202f29af50f81c8e4ee5` had FAILURE at workflow `916983e9-dbde-4db2-a967-438a666ebb19`, job `a130c5d9-c078-4c18-8ae5-571470f46d60`; available step #101 was CANCELED with unavailable task information and empty logs. Do not call it a test timeout or source failure.
- Exact main SHA `d35db951...` separately shows context `CircleCI Pipeline = ERROR`, target pipeline #1202. Pipeline-to-workflow/job/revision attribution and provider logs remain unavailable. Do not conflate this with the PR context or the source-SHA SUCCESS.
- B-001 is operationally resolved only for the tested code-bearing source/workflow state; provider root cause remains UNKNOWN and the latest docs HEAD has no new CircleCI result.

### B-002 — Issue #104 exact-source verification and remaining product gap
- Twelve PR-triggered verifier workflows associated with verifier HEAD `6c0979e62eb01e4b929c48b4224a3d4f742bc8af` are terminal SUCCESS, all using branch-local source pin `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`:
  - E4 adversarial #44658361321: Unix job #136030099861; Windows reparse/handle job #136030099812; Windows native probe #136030099584 — all SUCCESS.
  - Linux distro #44658361778: Ubuntu 24.04 job #136030102055; Debian 12 job #136030102321 — SUCCESS.
  - Windows native runtime #44658361296: job #136030100371 — SUCCESS.
  - E1 Windows #44658361571: job #136030101715 — SUCCESS.
  - E1 Milestone 1 #44658361518: Windows job #136030101394 and Ubuntu job #136030101588 — SUCCESS.
  - SEC-05 #44658361512: Linux job #136030100674 and Windows job #136030100987 — SUCCESS.
  - Cross-platform #44658361514: Windows job #136030101518 and Ubuntu job #136030101784 — SUCCESS.
  - Artifact integrity #44658361320: job #136030099580 — SUCCESS.
  - Resource guard #44658361457: job #136030100840 — SUCCESS.
  - Android AArch64 build #44658361365: job #136030100171 — SUCCESS.
  - Focused stdlib/database #44658361557: job #136030101561 — SUCCESS.
  - Android API 35 ARM64 runtime #44658361350: build job #136030099698 and runtime job #136030558684 — SUCCESS.
- E4 job logs were inspected, not just run summaries. Linux executed 31 named tests (3 read, 5 write, 1 walk, race, mount-boundary, hard-link, database, 11 governance, 5 network, policy-denial, platform metadata), 0 failed. Windows executed 9 named regressions, 0 failed. The Linux write suite explicitly includes `descriptor_relative_write_fails_closed_for_nested_parent`; this is proof of fail-closed behavior, not positive nested-write support.
- Android API 35 ARM64 runtime is native-backend execution evidence only; it does not establish an Android filesystem adapter.
- Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Do not promote full filesystem support from the scoped E4 pass.

### Release gate / B-003 boundary
- The protected ledger document pin remains `d6c382a245c49a028bc60809f22ca956c7f894cc`; the protected ledger gate #44519577312 remains historical FAILURE (92/100 provisional implementation, 0/100 verified at that gate's ledger source, 18 rows checked, 10 DoD items unchecked, no protected exact-source ONE Package Provenance Release).
- ONE PR #103 remains OPEN / DRAFT / UNMERGED at HEAD `5f616ae08e303081666a5e0d6a4b8b63b9d69c16`. Its branch does not replace the protected ledger source pin. Keep fixed weights, verified points, DoD checkboxes, canonical pins, and release criteria unchanged. No new gate evaluation is justified until relevant acceptance/evidence inputs change.
- No source code, CI configuration, main branch, protected verifier branch/pin, ledger weights, DoD checkbox, merge, or release was changed in this refresh.

### Next executable work
1. Continue B-002 from the remaining Linux nested-write acceptance gap. Keep the current fail-closed behavior; do not weaken it. Any implementation proposal must prove parent-directory move/race confinement with real Linux and Windows runners before changing security-sensitive code.
2. Keep unsupported Android/macOS filesystem adapters explicitly unavailable unless native backends and their own adversarial matrices are implemented and verified.
3. Continue B-003 only after the exact release-candidate SHA, complete E1–E6 evidence, source-bound package provenance, and ledger/DoD reconciliation are genuinely available. Do not rerun the unchanged historical gate or promote scores.


## B-003 exact-head refresh — PR #103 ledger draft (2026-11-20)

- ONE PR #103 remains OPEN / DRAFT / UNMERGED at HEAD `5f616ae08e303081666a5e0d6a4b8b63b9d69c16`. It is 7 commits ahead of protected ledger source pin `d6c382a245c49a028bc60809f22ca956c7f894cc`, and 2 commits ahead of prior CI-verified docs commit `e053b5ffae46a7e1d696bea5eb5a13999c722ee0`. The 2-commit delta changes only `docs/verification/one-completion-ledger.md` and `docs/verification/DEFINITION_OF_DONE.md`.
- GitHub Actions run #44573696031 is COMPLETED / SUCCESS on exact PR #103 HEAD `5f616ae...`: Ubuntu job #135735331182 and Windows Server 2025 job #135735332010 both terminal SUCCESS. This is documentation-branch CI evidence, not a release gate or E1–E6 acceptance promotion.
- CircleCI combined status for exact PR #103 HEAD is `CircleCI Pipeline = ERROR`, target pipeline #1168. Provider logs and pipeline-to-workflow/job/revision mapping are not available; classify the CircleCI cause as UNKNOWN, not a source/test failure.
- The unmerged PR #103 draft ledger records 92/100 IMPLEMENTED, 8/100 VERIFIED (E3 8/8), and DoD 4 checked / 9 unchecked / 13. These are draft values only. The protected verifier ledger source pin remains `d6c382a...`; the protected snapshot at that pin remains 92/100 implementation, 0/100 verified, DoD 3 checked / 10 unchecked / 13.
- Historical protected fixed-weight gate #44519577312 remains FAILURE. Do not treat PR #103's GHA success as a protected ledger-gate pass, and do not merge PR #103, move the ledger source pin, change weights, or check DoD boxes without explicit authorization and full acceptance evidence.
- No product source, CI configuration, main branch, protected verifier branch/pin, ledger weights, or acceptance criteria changed in this refresh.


## B-001 classification correction — 2026-11-20 (pipeline mapping not verified)

This correction supersedes any earlier wording in this document that called B-001 fully or operationally resolved. Do not delete the historical notes.

- Exact ONE source SHA `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` has GitHub combined status `ci/circleci: one-linux = success`. The status target URL identifies workflow `3a9c0613-a1bc-43ae-b2d6-319841061048` and job `c4faf162-80e9-4f17-a83a-a72634bb592f`.
- The CircleCI pipeline UUID/number for that job, pipeline `vcs.revision`, and independent proof that the job belongs to that pipeline/workflow were not retrieved. The GitHub status is exact-SHA context evidence, but it does not complete the required job → workflow → pipeline → `vcs.revision` chain.
- The historical #1201 pre-execution termination remains documented; its provider root cause remains UNKNOWN. Current PR #113 docs HEAD `abfae2b95c179650b362426a001a47e164f7ac20` has no CircleCI status context. Main pipeline #1202 and PR #103 pipeline #1168 show separate ERROR contexts, but their `vcs.revision`, job mapping, and provider logs are unavailable.
- CircleCI UI pipeline pages #1202, #1168, and #1199 were not accessible through the available web session. No authenticated CircleCI API/CLI provider-log access was available. Do not infer timeout, test failure, source regression, or root cause.
- **B-001 status: PARTIAL / NOT CLOSED.** The exact-source GitHub status is SUCCESS, but pipeline mapping and provider diagnostics are still required before claiming full operational resolution. Do not trigger blind retries or change CI configuration to hide the gap.


## Exact-SHA Windows parent-rename regression — 2026-11-20

- New code-bearing test SHA: `c4f3800bc12b70a4d59d72fbc5a80029cb053cea`; the only source change relative to the prior candidate is a Windows-only test in `crates/one-core/src/filesystem.rs`, not production behavior.
- Verifier PR #51 exact HEAD: `7c3ded0d811fe71bd14034880b24b0f8e396349e`; branch-local canonical source pin: `c4f3800...`. Protected verifier branch and canonical pin remain unchanged.
- New test: `filesystem::windows_native::tests::retained_parent_handle_blocks_directory_rename`. It opens a nested parent with the same `FILE_SHARE_READ | FILE_SHARE_WRITE` share flags used by `write_relative` (no `FILE_SHARE_DELETE`), proves rename fails while the handle is retained, then proves rename succeeds after handle close.
- E4 push run #44760172674 is terminal SUCCESS on exact source `c4f3800...` / verifier `7c3ded0...`: Unix job #136370280796 SUCCESS; Windows reparse/handle job #136370280317 SUCCESS; Windows native probe job #136370280733 SUCCESS. Windows log shows the new test executed exactly once and passed (1 passed, 0 failed).
- Duplicate E4 pull-request run #44760180858 is also terminal SUCCESS on the same exact source/verifier pair; Unix job #136370312121, Windows reparse/handle job #136370312012, and native probe job #136370311602 all SUCCESS. Count the duplicate event as confirmation, not a second acceptance point.
- Other exact-verifier-SHA PR workflows: SUCCESS — E4 resource guard #44760181208, focused stdlib/database #44760180912, Android AArch64 artifact build #44760180867, Linux distro #44760180826, E1 Windows #44760181124. IN_PROGRESS — Windows native runtime #44760181204, candidate artifact integrity #44760180923, SEC-05 #44760180951, E1 Milestone 1 #44760180919 (Ubuntu job #136370312623 SUCCESS; Windows job #136370313083 IN_PROGRESS), cross-platform #44760181947, Android API35 runtime #44760180953. Do not call the complete verifier matrix green yet.
- ONE source PR #113 GitHub Actions run #44760138732 is IN_PROGRESS on exact source `c4f3800...`: Ubuntu job #136370164396 SUCCESS; Windows job #136370165192 IN_PROGRESS. CircleCI `ci/circleci: one-linux` is PENDING on this exact SHA, workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Pipeline UUID/number, `vcs.revision`, and job→workflow→pipeline membership remain unverified.
- B-001 remains PARTIAL / NOT CLOSED pending CircleCI pipeline mapping/provider logs. Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes fail-closed; Android/macOS filesystem adapters unavailable. This test verifies one Windows parent-handle exclusion primitive, not complete E4/Issue #104 acceptance.
- No production runtime code, CI configuration, main, protected verifier branch/pin, ledger/DoD, acceptance criteria, merge, or release changed.


## Exact-SHA verification refresh — 2026-11-20 15:14Z

### Candidate identity
- Code-bearing source: `c4f3800bc12b70a4d59d72fbc5a80029cb053cea` (only source delta from `fb5c8bc...` is the Windows-only parent-handle rename regression test).
- Verifier PR #51 HEAD: `7c3ded0d811fe71bd14034880b24b0f8e396349e`; branch-local source pin: `c4f3800...`.
- ONE PR #113 current docs-only HEAD before this sync: `043390cf0673247524ffb44c444cbbe4745be91f`. This sync changes existing docs only and uses `[skip ci]`; no CI result transfers to the resulting docs HEAD.

### Terminal results on exact source/verifier pair
- ONE GitHub Actions run #44760138732 on source `c4f3800...`: Ubuntu job #136370164396 SUCCESS; Windows Server 2025 job #136370165192 SUCCESS. Windows log explicitly ran and passed `interop::tests::subprocess_execution_is_bounded_by_a_deadline`, `subprocess_stdout_capture_has_a_hard_limit`, `subprocess_stderr_capture_has_a_hard_limit`, and `windows_job_object_terminates_descendants_when_parent_exits`.
- Verifier PR matrix on verifier SHA `7c3ded0...` / source pin `c4f3800...`: 11 of 12 named workflows are terminal SUCCESS:
  - E4 adversarial #44760180858: Unix #136370312121, Windows #136370312012, native probe #136370311602 — SUCCESS. New `retained_parent_handle_blocks_directory_rename` test executed once and passed.
  - E4 subprocess resource guard #44760181208 / job #136370313217 — SUCCESS.
  - Focused stdlib/database #44760180912 / job #136370312551 — SUCCESS.
  - Android AArch64 artifact build #44760180867 / job #136370311618 — SUCCESS.
  - Linux distro #44760180826: Debian #136370312102 and Ubuntu #136370312874 — SUCCESS.
  - Windows native runtime #44760181204 / job #136370312887 — SUCCESS; Windows Server 2025, Rust 1.100.0, runtime exit 0, output `Budi`.
  - SEC-05 #44760180951: Linux #136370311990 and Windows #136370312357 — SUCCESS. Source PR Windows GHA also proves the dynamic deadline/stdout/stderr/Job Object tests executed and passed.
  - E1 Windows #44760181124 / job #136370313677 — SUCCESS.
  - E1 Milestone 1 #44760180919: Ubuntu #136370312623 and Windows #136370313083 — SUCCESS.
  - Cross-platform #44760181947: Windows #136370329101 and Ubuntu #136370329521 — SUCCESS.
  - Candidate artifact integrity #44760180923 / job #136370311819 — SUCCESS.
- E4 push event #44760172674 is also SUCCESS on the same exact source/verifier pair (Unix #136370280796, Windows #136370280317, native probe #136370280733); it is a duplicate trigger and is not a second acceptance point.

### Artifact provenance and remaining runtime
- Artifact integrity selected cross-platform run #44760181947 at verifier SHA `7c3ded0...` for exact source `c4f3800...`.
- Linux artifact #14640868829, SHA-256 `7e8e443e2ae7c736f18243043f355840801afebdddca46a5efe41c159bac5732`; metadata target `x86_64-unknown-linux-gnu`, Rust/Cargo 1.100.0, ONE CLI 0.1.0, ELF64 little-endian EM_X86_64 PASS.
- Windows artifact #14641018247, SHA-256 `f68c7325ff39ae33b2cfadf0b896896c0ba4c3eeb581b033f8e1a75974900846`; metadata target `x86_64-pc-windows-msvc`, Rust/Cargo 1.100.0, ONE CLI 0.1.0, PE32+ x86-64 PASS.
- The only verifier workflow not terminal is Android API 35 ARM64 runtime #44760180953. ELF build job #136370311867 is SUCCESS; runtime job #136371143664 is still executing the “Boot Android API 35 AArch64 on ARM64 host” step. Workflow timeout is 150 minutes; no timeout/failure is claimed.
- Do not claim Android filesystem adapter support from this native-backend runtime. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

### CircleCI and release gate
- CircleCI `ci/circleci: one-linux` on source `c4f3800...` remains PENDING; workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Pipeline ID/`vcs.revision` mapping and provider logs remain unavailable. B-001 remains PARTIAL / NOT CLOSED.
- Protected verifier branch/pin, protected ledger pin `d6c382a...`, fixed weights, DoD, main, acceptance criteria, merge, and release state remain unchanged. Historical fixed-weight gate #44519577312 remains FAILURE.


## CircleCI diagnostic utility correction — 2026-11-20 (exact source SHA refresh)

This entry supersedes earlier live diagnostic-utility conclusions where they conflict; retain prior incident records as history.

- Exact ONE PR #113 HEAD: `36c0abff77d20016dc0afc11e8428a6c3fb1248d` (OPEN / DRAFT / UNMERGED; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`). The diagnostic code/test delta adds no product runtime or CircleCI configuration changes.
- CircleCI combined status for this exact SHA: `ci/circleci: one-linux = PENDING`; status target workflow UUID `376028bf-901b-42b1-bd3e-0d93b64350ce`, job UUID `553f3bfc-8bd6-457d-8656-6a65837b1725`. This GitHub status does not establish that the provider job has started. Pipeline UUID/number, pipeline `vcs.revision`, workflow membership, provider start timestamp, and provider logs remain unverified. Do not label this a timeout or failure.
- Earlier exact source `c4f3800bc12b70a4d59d72fbc5a80029cb053cea` separately had CircleCI `ci/circleci: one-linux = SUCCESS`, target workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Do not transfer that result to the new diagnostic SHA.
- Static review found a false-positive classifier path: the utility passed its entire rendered report to `classify_failure()`; its own explanatory text `failed/timed_out` could therefore turn a canceled/no-output step into a false TIMEOUT label. Corrected to classify captured stdout/stderr only, while using explicit provider step outcomes separately. A timed-out outcome remains TIMEOUT; explicitly null job start plus empty logs becomes `PRE-EXECUTION TERMINATION / ROOT CAUSE UNKNOWN`; missing start-time metadata remains inconclusive; a started job with no logs becomes `NO-OUTPUT / ROOT CAUSE UNKNOWN`.
- Static review also found exact-SHA discovery selected only the newest matching pipeline, potentially hiding an earlier failed pipeline for the same commit. It now inspects every pipeline whose `vcs.revision` equals the requested SHA and reports pipeline number/UUID/status/revision, workflow UUID/status, job UUID/number, and job start metadata. Direct `--job-id` mode continues to mark source identity UNVERIFIED.
- Added six unit tests (suite count 14 → 20) for classifier self-text false positives, canceled pre-start vs started/no-output vs missing-start metadata, explicit timed-out outcomes, and multiple pipelines for one exact SHA. **These tests have not yet been observed executing on this exact SHA**; CircleCI currently reports PENDING. Do not claim test PASS until the exact job reaches a terminal result and its step log proves the test step ran.
- Historical pipeline #1201 remains confirmed pre-execution termination (`started_at=null`, canceled step, empty logs) with provider root cause UNKNOWN. Historical account-credit/provider eligibility ambiguity remains unresolved; no credit purchase, plan change, settings toggle, retry, or trigger was performed.
- **B-001 remains PARTIAL / NOT CLOSED.** No root cause has been established for #1201, and the new diagnostic candidate has not yet produced a terminal CircleCI result or complete pipeline → workflow → job → revision provenance.


## CircleCI diagnostic follow-up — 2026-11-20 (job visibility and current provider status)

This is the latest diagnostic checkpoint; earlier incident history remains intact.

- Latest diagnostic source/test SHA: `3db852f68d8bb12fb655524c7133c4cfa4002ca2`; PR #113 remains OPEN / DRAFT / UNMERGED. Changes are limited to `tools/ci/diagnose_circleci.py` and `tools/ci/test_diagnose_circleci.py`; `.circleci/config.yml` is unchanged.
- The diagnostic now discovers **all jobs from every pipeline whose `vcs.revision` exactly matches the requested SHA**, not only failed jobs from the newest matching pipeline. This keeps pending/running jobs visible and reports pipeline number/UUID/status/revision, workflow UUID/status, job number/UUID/status, and start-time metadata. Direct `--job-id` mode continues to mark source SHA as UNVERIFIED.
- The classifier now uses captured provider stdout/stderr and explicit step outcomes, never its own explanatory report text. It distinguishes explicit timed-out outcomes, explicit null start time with no output, known-start/no-output, and missing start-time metadata without inventing a timeout/root cause.
- Diagnostic unit suite now contains 20 test methods (14 baseline + 6 targeted regression tests). These tests have **not yet been verified as executed successfully on SHA `3db852f...`**.
- Exact-SHA CircleCI status readbacks:
  - `36c0abff77d20016dc0afc11e8428a6c3fb1248d`: `ci/circleci: one-linux = FAILURE`; workflow `376028bf-901b-42b1-bd3e-0d93b64350ce`, job `553f3bfc-8bd6-457d-8656-6a65837b1725`.
  - `2bcf397474019cd4415f6506d8dac83a3c5c4010`: `ci/circleci: one-linux = FAILURE`; workflow `fa1500e3-c2e2-42c7-a8c1-dc932bd439ab`, job `3fd1b528-f2a4-42f5-85e3-1d1716d8ad72`.
  - Latest code/test SHA `3db852f...`: `ci/circleci: one-linux = PENDING`; workflow `92fd294f-c8d5-480c-af6f-9ddbb7ffe479`, job `5666c0ea-e06e-475b-8967-5ccb24ccb383`.
  - Earlier source SHA `c4f3800bc12b70a4d59d72fbc5a80029cb053cea` remains a separate historical `SUCCESS` (workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd)); it does not validate the diagnostic changes.
- The CircleCI app job pages remain inaccessible in this connected session, and no authenticated CircleCI CLI/MCP action is available. Pipeline UUID/number, `vcs.revision` readback, job membership readback, step logs, and terminal provider outcomes are not available for the new jobs. GitHub status FAILURE alone does not reveal whether the cause is the new test, configuration, provider setup, or another condition.
- **B-001 remains PARTIAL / NOT CLOSED.** Do not infer test failure, timeout, or provider root cause from the status labels alone. No blind retry, configuration edit, plan purchase, or settings change was performed. Required next evidence: a redacted exact-SHA diagnostic report/provider step log for the failed workflow/job above, plus the latest pending job's terminal status and full pipeline → workflow → job → `vcs.revision` mapping.


## CircleCI terminal-status correction — 2026-11-20

A fresh exact-SHA status readback supersedes the PENDING observation above for diagnostic SHA `3db852f68d8bb12fb655524c7133c4cfa4002ca2`:

- `ci/circleci: one-linux = FAILURE`; workflow UUID `92fd294f-c8d5-480c-af6f-9ddbb7ffe479`; job UUID `5666c0ea-e06e-475b-8967-5ccb24ccb383`.
- The earlier diagnostic SHAs remain FAILURE: `36c0abf...` (workflow `376028bf-901b-42b1-bd3e-0d93b64350ce`, job `553f3bfc-8bd6-457d-8656-6a65837b1725`) and `2bcf397...` (workflow `fa1500e3-c2e2-42c7-a8c1-dc932bd439ab`, job `3fd1b528-f2a4-42f5-85e3-1d1716d8ad72`).
- The status target URLs still expose workflow/job UUIDs only. The CircleCI app pages remain inaccessible here; no provider step output, pipeline UUID/number, `vcs.revision`, job membership, or `started_at` has been independently read. Therefore the three FAILURE labels do **not** establish a test failure, timeout, or root cause.
- The diagnostic tests on SHA `3db852f...` must be treated as **NOT VERIFIED** until the actual CircleCI step log is obtained. B-001 remains PARTIAL / NOT CLOSED.
- Next safe evidence collection: run the repository's updated exact-SHA diagnostic locally for `3db852f...` using an already-configured local `CIRCLECI_TOKEN`; do not paste the token. Share only the redacted report. The utility now discovers every matching pipeline and lists all jobs, including pending/running, while preserving exact revision and job-start evidence.

## Implementation-first continuation — 2026-11-20

Owner direction: complete the actual v0.1 product implementation before spending another cycle on broad test/verification runs. Add or update necessary regression tests as part of implementation, but defer the full matrix and repetitive audits until the implementation scope is complete. Do not claim a feature complete merely because tests or docs are green.

### Current Issue #104 implementation delta

- PR #113 exact HEAD at this checkpoint: `7cbcb1bf575defe03a2d9991dc5094569de95f03` (OPEN / DRAFT / UNMERGED; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`).
- CLI filesystem authority now derives only from the host process current working directory; source-file location and program-controlled `--target` no longer select or widen the root.
- Linux root acquisition now opens the supplied directory with `O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC` and checks canonical-path device/inode identity against the retained handle; a mismatch fails closed.
- CLI regression fixture updated to place the source file outside the working directory while targeting an in-root file.
- Implementation commits: `8b1d13e02c181fc06b97fb7f147e1ccd013d3aa1`, `5e39430a341629eb9fda11ee4e11b0f6f40487ab`, `7cbcb1bf575defe03a2d9991dc5094569de95f03`.
- These changes are NOT BUILT OR TESTED yet. Existing prior-SHA results do not transfer. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

### B-001 operational status

- Exact SHA `979973cd6fb6f76802e13dbbbe21530d68b4cf53` has a terminal successful CircleCI workflow/job mapping: pipeline #1233 revision matches the SHA; workflow `934cd811-3a44-4737-bb62-943e677d979b` SUCCESS; job #865 `4b9419f9-9b1a-4823-902e-dcd872073e39` SUCCESS, started `2026-11-20T15:49:44.969Z`.
- The diagnostic classifier correction is locally 20/20 on that source. Historical pipeline #1201 cause remains UNKNOWN, but no longer blocks the current implementation queue. Do not rerun it without a new relevant failure.

### Next implementation priority

Continue Issue #104 production work and reconcile the normative contract against the current source. Preserve fail-closed behavior where a race-resistant primitive is not available; do not weaken the boundary to make nested Linux writes appear supported. Then continue remaining v0.1 product implementation gaps. Only after implementation is complete should the integrated final platform/security/release matrix run.

## Linux root no-follow regression — 2026-11-20

- Added `filesystem::tests::authority_rejects_symlink_root_without_following_final_component` in commit `ab72ec5d65a5e237cdc1b10e40ea209149f42e74` to prove that a symlink cannot be supplied as the final trusted-root component.
- This test has NOT been run. Latest code-bearing candidate is `ab72ec5d65a5e237cdc1b10e40ea209149f42e74`; current PR #113 HEAD is documentation-only `b8f5569333723cf916ad918735934b9b5d774095` (OPEN / DRAFT / UNMERGED). No exact-SHA test result is claimed.
- The current implementation delta comprises CLI cwd-only root selection, Linux no-follow root-handle acquisition with identity check, and the two new CLI/filesystem regression fixtures. Linux nested writes remain fail-closed; Android/macOS adapters remain unavailable; Issue #104 remains OPEN / NOT ACCEPTED.

## Windows root no-reparse implementation — 2026-11-20

- Windows root acquisition no longer canonicalizes through the final component before opening it. It canonicalizes only the parent, preserves the final name, opens it with the existing `OBJ_DONT_REPARSE` / `FILE_OPEN_REPARSE_POINT` native path, then compares volume serial and file index against the path identity.
- Added `filesystem::windows_root_tests::authority_rejects_reparse_root_without_following_final_component` in commit `8dd92aebc033a94abb14a8d7ec498f6933d4ba8e`; it creates a directory junction and requires root acquisition to fail closed.
- Windows implementation commit: `b208a1a5bdf4a9ffb00a7c17aa4a401ed4a237ce`. This and the Linux/CLI root changes are NOT BUILT OR TESTED. Latest code-bearing candidate: `8dd92aebc033a94abb14a8d7ec498f6933d4ba8e`.
- Preserve Issue #104 OPEN / NOT ACCEPTED. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable.

## CLI root-widening negative regression — 2026-11-20

- Added `run_command_does_not_allow_source_location_to_widen_filesystem_root` in test commit `ea376531e99f37b874d3d3a8edd4e8b4d656095f`. It places both the `.one` source and target under a parent outside the host working directory and asserts that the filesystem read is denied.
- Together with `run_command_uses_working_directory_as_filesystem_root_not_source_location`, the CLI regressions cover both positive in-root access from an external source file and denial of source-parent widening.
- These tests have NOT been run. Latest candidate source/test SHA: `ea376531e99f37b874d3d3a8edd4e8b4d656095f`; prior-SHA results do not transfer.

## Preserve host CWD path through root open — 2026-11-20

- Follow-up review found that the CLI was canonicalizing `cwd` before passing it to `FilesystemAuthority::open`, which could bypass the new final-component no-follow check on hosts where `current_dir()` preserves a reparse/junction path.
- Fixed in `crates/one-cli/src/main.rs` commit `56bd643571f8a371c5d3cabb5175113672d9198f`: pass the host `cwd` path directly to `FilesystemAuthority::open`; platform authority code performs safe parent normalization and final-component no-follow/identity validation itself.
- This source change is NOT BUILT OR TESTED. Latest source/test candidate is `56bd643571f8a371c5d3cabb5175113672d9198f`. Issue #104 remains OPEN / NOT ACCEPTED.

## Linux atomic-write adversarial coverage added — 2026-11-20

- Added `descriptor_relative_write_race_never_modifies_outside_symlink_target` in `crates/one-core/src/adapter.rs` (commits `7ca8e84230383cc84684539ee87dc2d27e72d4fa` and `ab772225832c8b3cacc1772e9b384b974c1a2884`). It repeatedly races leaf symlink/regular-entry replacement against atomic writes and asserts the outside sentinel remains unchanged and no `.one-write-*` temp entry leaks.
- Refactored the Linux atomic writer with internal deterministic failure checkpoints (`TempCreated`, `Written`, `Synced`, `BeforeRename`) in `c7235ab925f2aff7d248b54f9ea74686236b4668`; added `authority_atomic_write_failures_cleanup_temp_and_preserve_target` in `8b95e3bb8b739083eb4e6f52b6aee81ed4cc088a` to assert cleanup and target preservation at every stage.
- These tests have NOT been run. Latest source/test candidate: `8b95e3bb8b739083eb4e6f52b6aee81ed4cc088a`; current PR #113 HEAD at readback: `8b95e3bb8b739083eb4e6f52b6aee81ed4cc088a`. No prior exact-SHA evidence transfers.
- Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

## Android host identity — 2026-11-20

- `crates/one-core/src/platform.rs` now exposes `Platform::Android` and returns it for `target_os = "android"` instead of collapsing Android into `Platform::Other` (`f8e46856803c9f45f54402e71d7e19853bdf46bc`).
- This is host identity only; it does NOT enable Android filesystem operations. The filesystem adapter remains fail-closed on Android until a safe resolver is available that avoids process-killing seccomp behavior and preserves root/symlink/mount/race guarantees.
- The platform unit test was updated but has NOT been run.

## Explicit desktop/mobile host identity — 2026-11-20

- `Platform` now identifies Windows, Linux, Android, and macOS explicitly rather than collapsing Android/macOS into `Other`; the platform unit test covers all four target cfgs (`5846eb70f2a462331873cc0023d1f1ab1106f86a`).
- This is platform identification only. It does not imply filesystem adapter support on Android/macOS; those adapters remain fail-closed pending platform-safe resolver implementations.
- The platform test has NOT been run.

## Rejected Linux nested-write workaround: Landlock-only worker

Do not treat a Landlock ruleset in a helper process as sufficient proof for nested writes. The Linux `landlock(7)` contract states that file descriptors opened before sandboxing are not subject to the restrictions, and `LANDLOCK_ACCESS_FS_WRITE_FILE` is checked when opening a file, not on subsequent `write(2)` calls. A parent directory can therefore be moved after a temporary file is opened; the already-open descriptor may still be writable. See the [Linux Landlock manual](https://man7.org/linux/man-pages/man7/Landlock.7.html).

Until a different kernel-enforced primitive or threat-model change is proven on real Linux and Windows runners, keep nested Linux writes fail-closed. Do not replace this limitation with a Landlock-only helper, post-open ancestry check, advisory lock, or userspace watcher.

## Candidate for safe Linux nested writes — NOT APPROVED FOR PRODUCTION

Research identified a possible kernel-enforced path-resolution approach that must be proven before production changes:

1. Clone the retained root mount as a detached mount tree with `open_tree(root_fd, "", AT_EMPTY_PATH | OPEN_TREE_CLONE | AT_RECURSIVE | OPEN_TREE_CLOEXEC)`.
2. Apply `MOUNT_ATTR_NOSYMFOLLOW` recursively with `mount_setattr(mount_fd, "", AT_EMPTY_PATH | AT_RECURSIVE, ...)`.
3. Use the detached mount fd as the `dirfd` for `*at()` operations (the `open_tree(2)` contract explicitly permits this). Create the temporary file at the clone root, then commit with a full relative destination path rooted at the same mount fd. Do not retain a nested parent fd for the final rename.
4. If a parent is moved out, the destination path should no longer resolve from the cloned root; if an intermediate symlink is substituted, the mount's no-symlink attribute should reject traversal; cross-mount rename should fail with `EXDEV`.

Hard limitation: `OPEN_TREE_CLONE` requires `CAP_SYS_ADMIN`; unsupported kernels/hosts must remain fail-closed. References: [open_tree(2)](https://man7.org/linux/man-pages/man2/open_tree.2.html), [mount_setattr(2)](https://man7.org/linux/man-pages/man2/mount_setattr.2.html).

Status: design hypothesis only. No production code changed for this approach. Before adoption, a dedicated adversarial prototype must demonstrate parent move-out, symlink substitution, mount-boundary, temp cleanup, and outside-sentinel preservation on the actual Linux runner; Windows behavior must remain green. Do not claim this solves the gap until those results exist.

## Android filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `7dd7b0049a7f5d71ac8a3d59570c2975fed835ec` (NOT BUILT OR TESTED).
- Android now has a native `FilesystemAuthority` root open and a handle-relative direct-child read/atomic-replace backend. Root and leaf are opened without following final symlinks; `statx(STATX_MNT_ID)` is required to reject mount crossings; reads reject non-regular and hard-linked files; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The backend deliberately avoids `openat2` because Android seccomp may terminate the process with `SIGSYS` for that syscall. It requires kernel `statx` mount-ID support; if unavailable it fails closed.
- Scope is intentionally limited to direct children of the trusted root. Nested Android paths return `UnsupportedPlatform`; no claim of full Android filesystem support or Issue #104 acceptance is made.
- Three Android-targeted regression tests cover direct-child read/write, leaf-symlink and nested-path denial, and final-component root-symlink rejection. They have NOT been run. Actual Termux/device execution is still required.
- Linux nested writes remain fail-closed; macOS filesystem adapter remains unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

## IMPLEMENTATION-FIRST LIVE OVERRIDE — 2026-11-20 (Android filesystem)

- Current PR #113 HEAD before this register refresh: `d2aaa345717fcd86f4998a603f2756c1a58317ed`; branch remains OPEN / DRAFT / UNMERGED; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Latest code-bearing source/test candidate: `7dd7b0049a7f5d71ac8a3d59570c2975fed835ec` (NOT BUILT/TESTED).
- Implemented Android native root acquisition and direct-child file read/atomic replace using a retained root fd, O_NOFOLLOW, statx mount-ID checks, hard-link rejection, exclusive temp creation, fsync, and root-relative renameat. Android nested paths remain fail-closed; statx MNT_ID is mandatory; macOS adapter remains unavailable.
- Added three Android-only regression tests. No build, test, device run, or verifier result is claimed for this candidate.
- Continue implementation work; do not run the broad matrix until implementation scope is complete. Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes remain fail-closed. Preserve fixed acceptance, ledger/DoD, verifier pins, main, and release controls.


- Documentation-only follow-up after the Android source candidate: current PR #113 HEAD is `9aa16bfcc641e5e1608669b440c88f8ea6dd2fee`; Android source/test candidate remains `7dd7b0049a7f5d71ac8a3d59570c2975fed835ec`. No build/test/device evidence is claimed.

## macOS filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `4be1b8fd579092d8195c3fb775b7c765f6221814` (NOT BUILT OR TESTED).
- macOS now has a native retained-root open and direct-child read/atomic-replace backend. Root and leaf operations use directory/file descriptors with `O_NOFOLLOW`; file and root device identity are compared; hard-linked and non-regular targets are rejected; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The macOS adapter is deliberately limited to direct children of the trusted root. Nested paths return `UnsupportedPlatform`. This implementation has no target-host execution evidence yet; its mount/device and race behavior still require platform-specific review.
- Three macOS-only regression tests cover direct-child read/write, symlink/nested-path denial, and final-component root-symlink rejection. They have NOT been run.
- Android direct-child backend remains statx-mount-ID dependent; Linux nested writes remain fail-closed. Issue #104 remains OPEN / NOT ACCEPTED.


- Current PR #113 HEAD `d14a679f20cf49bb408a5b72ddaf9ecc9155d254` includes the macOS adapter source candidate `4be1b8fd579092d8195c3fb775b7c765f6221814` plus documentation synchronization. No build/test/device evidence is claimed for either new Android/macOS backend.

- Live readback after Android/macOS implementation documentation: PR #113 HEAD `5f5b0db544a5d54b856e0708417b8e4e3c1cbd28`, OPEN / DRAFT / UNMERGED. Code-bearing source candidate remains `4be1b8fd579092d8195c3fb775b7c765f6221814`; docs-only descendants do not change its unverified status.

## Issue #105 — Linux process-group containment implementation candidate — 2026-11-20

- Latest code-bearing candidate: `0a9685bded788ae46596227bd61457d10db1179c` (NOT BUILT OR TESTED).
- Linux subprocess startup now installs an inherited seccomp filter in the child before `exec`. It denies `setsid`, `setpgid`, `unshare`, `setns`, clone namespace flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` so compatible runtimes can fall back to `clone`. The existing dedicated process group is therefore intended to remain non-escapable for descendants; group cleanup still runs on normal return, timeout, and output overflow.
- If the kernel refuses `PR_SET_NO_NEW_PRIVS` or filter installation, process spawn fails closed. Windows Job Object containment remains unchanged.
- Process execution is now explicitly unavailable on Android and macOS until equivalent non-escapable containment exists; `ProcessAdapter` advertises only Linux and Windows.
- Replaced the old escaped-session reader-only probes with three Linux adversarial regressions for attempted `setsid` on normal completion, timeout, and output overflow. These tests have NOT been run.
- This is an implementation candidate, not Issue #105 acceptance. Exact-SHA build/execution and adversarial evidence are still required. Preserve bounded reader shutdown and all existing process limits.

- Live implementation refresh: PR #113 HEAD `412d87f6e05d2d996538093623bcb52412216553`; latest code-bearing source/test candidate `0a9685bded788ae46596227bd61457d10db1179c` adds Linux seccomp process-group containment and fail-closed Android/macOS process execution. No build/test evidence is claimed.

- Live PR identity refresh: #113 HEAD `955d37489e44c4b638d047477283160db62dc609`, OPEN / DRAFT / UNMERGED. Current code-bearing source/test candidate remains `0a9685bded788ae46596227bd61457d10db1179c`; no build/test evidence is claimed.

- Architecture source updated for Android/macOS direct-child backends in commit `b023e6df94db416224a22211aa466fb854cabca0`; current PR #113 HEAD is now that SHA. Latest code-bearing source remains `0a9685bded788ae46596227bd61457d10db1179c`.


## 2026-11-21 implementation refresh

- PR #113 was OPEN / DRAFT / UNMERGED at code-bearing candidate `4c46a41fe45baa7627557a2a25b9c71812840479`; current candidate has not been built or tested.
- Issue #104: Android/macOS direct-child filesystem backends are implemented; Android verifies statx mount ID; macOS verifies `fstatfs` filesystem ID and mount point; both add hard-link regressions, exclusive temp replacement, and directory sync. Linux nested writes use a guarded detached-mount backend and fail closed when unsupported. Windows backend remains unchanged. Issue #104 remains OPEN / NOT ACCEPTED.
- Issue #105: Linux/Android inherited seccomp process-group containment candidate and Windows Job Object remain in scope; macOS process execution fails closed. Adversarial tests are present but unrun. Issue #105 remains OPEN / NOT ACCEPTED.
- Keep broad matrix deferred until product implementation scope is complete. B-001 is not reopened absent a new provider failure.

- Latest implementation refresh: source/test candidate and PR #113 HEAD `fd1ccc5ca56f48a306ac8f8c551189f46b8fe44c` (OPEN / DRAFT / UNMERGED). Process containment regressions now invoke ignored helper tests from the current Rust test binary and exercise `fork` + direct `setsid(2)` rather than depending on an external `setsid` executable; the helper records any escaped PID so the parent test can clean it up before failing. These regressions remain NOT RUN.

- Issue #105 follow-up source `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` adds `waitid(WNOWAIT)` observation of the direct child. The process-group leader remains unreaped until SIGKILL is sent to its group, preventing the group ID from being recycled before cleanup. The existing syscall-level adversarial tests remain unrun.

- Documentation synchronization after code-bearing source `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` places PR #113 at HEAD `b91e8965b90270b3da4175df807043206a434b9a`; subsequent commits are documentation-only. PR remains OPEN / DRAFT / UNMERGED.


## IMPLEMENTATION-FIRST CONTINUATION — 2026-11-21

This checkpoint follows the owner's explicit instruction to finish the in-scope implementation before starting broad tests, verification, or release-gate reruns. It does not change any acceptance criteria or protected state.

### Exact candidate identity
- ONE PR #113 remains OPEN / DRAFT / UNMERGED; branch `security/issue-104-root-confinement-20261118`; base `fix/windows-process-verifier-timeouts-20261118` at `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Latest code-bearing implementation source: `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` (Issue #105 Linux/Android child-PID retention using `waitid(WNOWAIT)`).
- Test-fixture correction commits: `46c777e70f396cfee6d796d0667769dabe2d7208` and `2600ec18d4e3fb05558ca50d7cf466638e1ab015`; both affect only `crates/one-cli/tests/filesystem_write.rs`.
- PR #113 HEAD after those commits: `2600ec18d4e3fb05558ca50d7cf466638e1ab015`. No source implementation or CI config changed in those two commits.

### Implementation/source inventory — presence is not execution evidence
- Issue #104 source contains Linux retained-root `openat2` reads, atomic direct-child writes, guarded detached-mount nested writes with fail-closed fallback; Windows native handle-relative/reparse-point backend; Android direct-child read/write with no-follow and `statx(STATX_MNT_ID)`; macOS direct-child read/write with no-follow and filesystem identity checks; exclusive temporary files, sync/rename paths, and hard-link rejection.
- Issue #105 source contains inherited Linux/Android seccomp process-group containment, Windows Job Object containment, macOS process execution fail-closed, and Linux/Android `waitid(WNOWAIT)` so the direct child PID is retained until process-group cleanup.
- Source contains three direct-`fork` + `setsid` adversarial regressions for normal completion, timeout, and output overflow, with escaped-PID recording/cleanup. These tests have NOT been run on source `72eae...`.
- Android/macOS nested filesystem paths and macOS process execution remain fail-closed. Do not claim broader support.

### Test-fixture implementation corrections
- Replaced the incorrect “nearest manifest selects root” expectation with the contract-compliant host working-directory root test.
- Added an explicit CLI test runner working-directory helper and used it where the trusted host root must be controlled by the fixture.
- Updated Linux nested-write integration expectations: success is accepted only when the detached-mount backend works; otherwise only `UnsupportedPlatform` with unchanged target bytes and no leaked temp file is accepted.
- Restricted the unsupported-platform fixture to targets without native filesystem backends; Android and macOS are no longer incorrectly classified as unsupported.
- Commits use `[skip ci]` by design. No tests/builds/verifier workflows were run as part of this implementation-first checkpoint.

### CI and acceptance boundary
- Exact PR HEAD `64cea5895d930b8621a7e31209676057d275fdb3` had GitHub status `ci/circleci: one-linux = FAILURE` (workflow `b7b0bbab-06dc-4cbc-88cf-f54624594d8f`, job `6e1d368b-8055-4cfe-a64e-858f9bd2fcdd`). Provider pipeline revision, job membership, logs, and root cause were not obtained; no cause is inferred.
- The newer commits `46c777e...` and `2600ec1...` intentionally skip CI and have no status contexts. No PASS is claimed for the new HEAD.
- Issue #104 and Issue #105 remain OPEN / NOT ACCEPTED until the required exact-SHA evidence is produced. This checkpoint records implementation/source state only; it does not promote ledger points, DoD, verifier pins, or release status.

### Next implementation work
Continue contract-to-source integration review of the existing v0.1 implementation scope and close any remaining production-code gaps before the broad test matrix. Do not rerun B-001 diagnostics, existing exact-SHA verifier suites, or historical gates without a relevant input change. Preserve fail-closed behavior and all protected state.


> The PR #113 HEAD in the table is the exact live HEAD read immediately before this register refresh. This documentation update itself advances the PR branch; refresh the branch HEAD before the next action. The code-bearing source SHA is unchanged by this documentation-only update.


## Scope reconciliation — fixed milestone ledger vs v0.1 release — 2026-11-21

The protected verifier's `LEDGER_SOURCE_COMMIT.txt` pins documentation SHA `d6c382a245c49a028bc60809f22ca956c7f894cc`, which contains the fixed 100-point milestone ledger and Definition of Done. Its 92/100 implementation subtotal (E1 7/10, E2 7/8, E3 8/8, E4 6/6, E5 4/4, E6 0/4) is PROVISIONAL and belongs to the separate milestone-100 workstream; it is not a completion score for PR #113's current v0.1 candidate. The current PR #113 source branch is `security/issue-104-root-confinement-20261118`, with code-bearing candidate `72eae103bc3304b0961c66a0cf61f716ddfa9ac4`. The separate `milestone/100-ecosystem-20261023` branch is not safe to merge as-is and is not the current PR #113 integration base.

The approved v0.1 package/build contract explicitly scopes the executable subset to an empty external dependency set and deterministic debug/intermediate `.oneir` / `.onepkg` artifacts; it does not claim a full external package registry or complete signed provenance envelope. Do not import future vNext/milestone-100 scope into v0.1 merely to chase ledger points, and do not alter the protected ledger, fixed weights, DoD, or release acceptance. Finish implementation against the approved v0.1 contract, then perform the required exact-SHA verification and gate reconciliation.


### Related Issue #81 resource-bound implementation refresh — 2026-11-21

The same current code-bearing source `cd95e5ec2234a2909ef79bf8b25fe65296fab5ed` explicitly enforces a 10-second process deadline, 1 MiB stdout cap, 1 MiB stderr cap, and 2 MiB aggregate capture ceiling. Existing source tests cover stdout overflow, stderr overflow, deadline, normal status/stream preservation, and cleanup. Issue #81 remains OPEN pending exact-SHA Linux/Windows execution; Issue #105 remains OPEN / NOT ACCEPTED pending Linux and real Android/Termux adversarial execution. Do not start broad tests before the implementation scope is complete.


## Linux FIFO blocking-read implementation fix — 2026-11-21

- Code commit `50923b545f2c804b24aa773ea5bb60129d1e0ff2` changes Linux `FilesystemAuthority::open_relative` to include `O_NONBLOCK` in the `openat2` read flags. Previously a FIFO target could block during open before the post-open regular-file check. Android/macOS read backends already use nonblocking open flags.
- Added Linux regression `authority_rejects_fifo_without_blocking`: creates a FIFO without a writer and expects `NotRegularFile`. It is NOT RUN.
- Documentation sync commits: `8f091944...`, `bfead1a4...`, `0bfd7f48...`, `24e6ba24...`, `d48dbf19...`, `ac3e6037...`, `f805752d...`, `6fc86c68...`. These record the implementation delta only; no test/build/CI evidence is claimed.
- PR #113 was OPEN / DRAFT / UNMERGED at the last live read. Base SHA remains `181ad350d8f0db53cda654c4d378425fbdd6ab24`. The table's HEAD is the exact read immediately before this register refresh; this register commit itself advances the branch. Refresh live HEAD before the next operation.
- Issue #104 remains OPEN / NOT ACCEPTED. The fix removes a blocking-I/O hazard but does not close race, mount, hard-link, platform, or acceptance requirements.


## Filesystem read allocation and resource-limit hardening — 2026-11-21

- Latest code-bearing source: `8792f7a1f707f444d00026d916fa8c18b7f3e7f1`.
- The Linux `openat2` read path includes `O_NONBLOCK` (commit `50923b545f2c804b24aa773ea5bb60129d1e0ff2`) so FIFO reads cannot block before regular-file rejection; regression `authority_rejects_fifo_without_blocking` is present, NOT RUN.
- Read-buffer capacity now uses checked `usize::try_from` and fallible `Vec::try_reserve_exact` rather than unchecked `u64 as usize`/infallible preallocation (commit `8792f7a1f707f444d00026d916fa8c18b7f3e7f1`). Added `authority_enforces_read_and_write_limits_without_partial_replacement`, NOT RUN.
- Commit `68d39100980474e71f78e4f9c079d8bc2c603fbe` also corrected the stale Linux nested-write comment to describe the detached-mount path and fail-closed fallback accurately.
- No build, test, CircleCI, verifier, or platform matrix has run on this candidate. Issue #104 remains OPEN / NOT ACCEPTED. These are implementation changes and test additions, not acceptance evidence.
- PR #113 remains OPEN / DRAFT / UNMERGED; base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`. The table HEAD is the exact pre-register read; this register update itself advances the branch, so refresh before next action.


## Linux nested-write implementation and verifier inventory alignment — 2026-11-21

- Latest code-bearing source: `858a5da09fe2ea74ebd17b0798f13fac848c7a28`.
- Commit `0fd434ad46e6b8e93c3b9ed887447baf10d0f697` maps Linux `EOPNOTSUPP` from required detached-mount primitives to `UnsupportedPlatform`, preserving fail-closed behavior for filesystems/kernels without the required operation.
- Commit `858a5da09fe2ea74ebd17b0798f13fac848c7a28` replaces the obsolete Linux nested-write test that unconditionally expected denial. The new `nested_write_uses_native_backend_or_fails_closed_without_side_effects` accepts native detached-mount success or explicit unsupported fallback and checks target integrity/temp cleanup.
- Current `adapter.rs` source inventory exactly matches the protected E4 workflow's nonzero test filters: 3 `descriptor_relative_read_rejects_`, 5 `descriptor_relative_write_`, 1 `descriptor_relative_walk_rejects_`. This was checked by source-name inventory only, not by running the tests.
- No build, test, CircleCI, verifier, or platform matrix was run. Issue #104 remains OPEN / NOT ACCEPTED. No acceptance, verifier pin, ledger, DoD, or release state was promoted.
- PR #113 remains OPEN / DRAFT / UNMERGED; base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`. The table HEAD is the exact pre-register read; this register update advances the branch, so refresh live HEAD before next action.


## Duplicate subprocess-test helper compile blocker removed — 2026-11-21

- Source commit: `0cceeb447fbe2ae59b8e3da78dee3ce8d9959830` (`crates/one-core/src/interop.rs`).
- Removed the second identical `cleanup_test_escaped_group` definition from the `interop.rs` test module. The duplicate would produce a Rust duplicate-definition error when compiling the test module; only one definition remains in source.
- This is a source-level compile-blocker correction, not a compile/test result. No build, test, CI, verifier, or platform matrix was run. Issue #105 remains OPEN / NOT ACCEPTED.

- PR #113 remains OPEN / DRAFT / UNMERGED; base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances the branch; refresh live HEAD before the next operation.


## Current source/documentation reconciliation — 2026-11-21

- Source compile-blocker correction: `0cceeb447fbe2ae59b8e3da78dee3ce8d9959830` removes the duplicate `cleanup_test_escaped_group` definition in `interop.rs` test module. Exactly one definition remains in the current source. No build/test has been run.
- Release contract reconciliation: `da5b007fe1dcfc7caa3c14c69b500866d5edbb75` corrects stale platform statements. Current source exposes filesystem adapters on Linux, Windows, Android, and macOS (with Android/macOS nested paths fail-closed; Linux nested writes require detached-mount support); process adapters on Linux, Android, and Windows (macOS fail-closed). These are source capabilities, not acceptance or support claims.
- PR #113 remains OPEN / DRAFT / UNMERGED, base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`. The table HEAD is the exact pre-register read; this register update itself advances branch HEAD. Refresh live PR metadata before the next action.
- Issue #104 and #105 remain OPEN / NOT ACCEPTED. Broad build/test/verification remains deferred until implementation scope is complete.


## Evidence/documentation sync after subprocess helper correction — 2026-11-21

- The evidence matrix and v0.1 completion addendum now record source commit `0cceeb447fbe2ae59b8e3da78dee3ce8d9959830` as a duplicate-test-helper compile-blocker correction only. They explicitly do not promote build/test/verification status.
- Release contract platform-boundary reconciliation is commit `da5b007fe1dcfc7caa3c14c69b500866d5edbb75`.
- PR #113 live HEAD at pre-register refresh: `ad29bda44f571f70106b1531ef1913534e3ce938`; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`; state OPEN / DRAFT / UNMERGED. This register update advances the branch; refresh before the next operation.
- Issue #104/#105 remain OPEN / NOT ACCEPTED. Implementation-first order unchanged.


## 2026-11-21 — Android/macOS helper repair and fail-closed nested-path boundary

- Latest code-bearing source: `5f98db9f70d28a0d5a206a3e6e37f986efc3574b` (`crates/one-core/src/filesystem.rs`).
- Removed duplicate Android `create_temp` definition and added missing macOS `create_temp` implementation. These are source-visible compile-blocker corrections; no build was run.
- Android/macOS retain direct-child no-follow read and atomic replacement. Android checks mandatory `statx(STATX_MNT_ID)`; macOS checks `fstatfs` filesystem ID + mount point; both enforce regular-file/single-link policy.
- Nested Android/macOS paths remain fail-closed. A descriptor-relative parent walk was not retained as supported because ordinary POSIX directory handles do not prevent an opened parent directory from being renamed outside the retained root. The current resolver rejects multi-component paths with `UnsupportedPlatform` until an anti-move primitive exists.
- Android/macOS tests now explicitly preserve nested-path denial and outside-sentinel integrity. They are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED. No build, test, CI, verifier, or platform matrix ran. Preserve acceptance criteria, fixed ledger/DoD, and protected verifier pins.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register refresh HEAD `6c4601d7930f23f8676a7d8f29b3b0f216b0daa1`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — Android/macOS filesystem source repair and confinement boundary

- Latest code-bearing source: `0f5859426273a61174d6d1b08ed8771a6f4b136a` (`crates/one-core/src/filesystem.rs`).
- Removed duplicate Android `create_temp` and added missing macOS `create_temp`; static inventory shows one helper definition in each native module. No compiler was run.
- Android/macOS direct-child no-follow reads and same-directory atomic replacement remain. Android requires `statx(STATX_MNT_ID)`; macOS checks `f_fsid` + mount point using `fstatfs`; both enforce regular-file/single-link policy.
- Nested Android/macOS paths remain fail-closed. Ordinary POSIX directory descriptors do not prevent parent rename-out; multi-component traversal is explicitly rejected with `UnsupportedPlatform` until an anti-move primitive is implemented. The attempted generic openat parent walk was not left enabled.
- Android/macOS regressions cover direct-child paths, symlink rejection, nested fail-closed behavior, root symlinks, and hard-link side effects. NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED. No build/test/CI/verifier/platform matrix ran; no acceptance or release state promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `69e863f9a1238650e14c367cb5036e38d473b3bb`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register commit advances the branch; refresh before next action.


## 2026-11-21 — HTTP response and filesystem payload resource bounds

- Latest relevant source candidate: `5fe958cad798b3d05c3ceb21f875f1f90074cb35` (NOT BUILT/TESTED).
- `FileWriteAdapter` no longer eagerly clones arbitrary text before the filesystem authority enforces `max_write_bytes`; it passes a borrowed slice to the authority and only makes the bounded response copy after successful write. Added oversized-content/no-side-effect regression; NOT RUN.
- HTTP/1.1 response ingestion now caps headers at 64 KiB and wire body at 32 MiB while receiving data, uses fallible vector growth, and retains a 16 MiB decoded-body cap. Parser rejects duplicate Content-Length, unsupported/duplicate Transfer-Encoding, conflicting Content-Length + chunked framing, and chunk-size overflow. Added bounded-reader/parser regressions; NOT RUN.
- No build/test/CI/verifier/platform matrix was run. These changes are implementation/resource-boundary hardening only; no exact-SHA evidence or release status is promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `39741b5b90f6326a7b3991ee9d48521ea52f74bd`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — bounded HTTP request serialization follow-up

- Latest source candidate: `198fbf4ccdbb7e219efbe3e296e98462a385c384` (NOT BUILT/TESTED).
- HTTP request bodies are limited to 16 MiB before serialization; request line/headers to 64 KiB with checked arithmetic and fallible preallocation. Content-Length must match body length; duplicate Content-Length and unsupported Transfer-Encoding are rejected. Added request-limit/framing regressions; NOT RUN.
- This complements the previously recorded bounded HTTP response ingestion (64 KiB headers, 32 MiB wire body, 16 MiB decoded body) and filesystem write payload allocation hardening.
- No build/test/CI/verifier/platform matrix ran. No exact-SHA evidence or release status promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register refresh HEAD `ff6b66088a0dadecb2adca1847f0642be544c04b`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — HTTP request syntax validation follow-up

- Latest source candidate: `b0a3379a1fb49feb18dcf61a2521446f199a8e2b` (NOT BUILT/TESTED).
- HTTP request method/header names now require valid token syntax; authority and request-target reject whitespace/control bytes and fragments; header values reject control bytes other than HTAB. This complements request/response resource limits and framing checks already recorded.
- No build/test/CI/verifier/platform matrix ran. No exact-SHA evidence or release status promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `bcfeadb61d79bfe09476cd47bdd9d9c20be92e28`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — HTTP exact framing and chunk trailer validation

- Latest source candidate: `eea3115ba5e70ff83f8ed78a01dc70d95e012ff0` (NOT BUILT/TESTED).
- Content-Length bodies require exact declared length; chunked parsing bounds size lines, validates trailer syntax and termination, rejects framing/Host trailers, and rejects trailing bytes. Regressions added for valid/forbidden trailers and extra bytes; NOT RUN.
- This extends the existing request/response size limits, token validation, ambiguous-framing rejection, and fallible buffer growth.
- No build/test/CI/verifier/platform matrix ran. No exact-SHA evidence or release status promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `4016691abe2ab3ba1d7b497b1c81b7102e8a2e56`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — HTTP header token/control validation

- Latest source candidate: `11bf5172ed060818dbd55d2ecee3cddf3cd99875` (NOT BUILT/TESTED).
- HTTP response header names are validated as tokens and values reject control bytes except HTAB. Request header names/values and request-target syntax have equivalent validation. Added regressions for invalid response header names/values and invalid request header names/targets; NOT RUN.
- This builds on bounded request/response bodies, strict Content-Length/Transfer-Encoding framing, chunk trailer validation, and checked/fallible allocations.
- No build/test/CI/verifier/platform matrix ran. No exact-SHA evidence or release status promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `c501032e86cf9487ecbeeaacf6328b6cf95d7af8`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — exact fallible HTTP buffer reservations

- Latest source candidate: `bbd4febbac3a4ba2a19195582e7e5992d1953b1c` (NOT BUILT/TESTED).
- HTTP request serialization, response ingestion and chunk-body decoding use `try_reserve_exact` under the explicit 16 MiB/64 KiB/32 MiB ceilings, avoiding geometric capacity growth beyond intended limits.
- No build/test/CI/verifier/platform matrix ran. No exact-SHA evidence or release status promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `a2b98ba20ab6473a69698825457031178391c93c`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — IPv6 HTTP authority and query-only target support

- Latest source candidate: `be5566435394cc27bbeb863b78c69b0fe52f774c` (NOT BUILT/TESTED).
- Shared HTTP authority parsing supports bracketed IPv6 with default/explicit ports; generic DNS/socket resolution now brackets IPv6 socket addresses correctly.
- HTTP GET target parsing supports bracketed IPv6 and query-only paths, and rejects malformed/whitespace paths. Source regressions added, NOT RUN.
- No build/test/CI/verifier/platform matrix ran. No exact-SHA evidence or release status promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `2ac38936eef9500cee6bf40f49778efd84ca9c54`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — process adapter smoke fixture correction

- Latest source/test candidate: `b4b4b3ee82e013eb3c001606820deaa76d50a9df` (NOT BUILT/TESTED).
- The process adapter no-argument smoke fixture now uses `whoami` instead of `rustc` (which can return nonzero without an input file); it is cfg-gated to Linux/Android/Windows. Android is included in process-platform metadata coverage.
- Test source only; NOT RUN. Issue #105 remains OPEN / NOT ACCEPTED.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `dda1428ecb4cfabdecc8eb419d98965fa1a2ac39`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — SQLite query resource boundaries

- Latest source/test candidate: `76f76c9a7a5884c08d851afc203cb1cae7a5827a` (NOT BUILT/TESTED).
- SQLite adapter limits SQL to 64 KiB, parameters to 128 / 1 MiB total, rows to 100,000, serialized output to 16 MiB; SQLite's own per-connection limits now cap value/row size, SQL size, columns, variables, compound SELECTs, expression depth, function args, LIKE patterns, attached databases, and worker threads.
- Output serialization uses bounded fallible exact allocation; BLOB hex output is size-checked before copy, and text conversion is streamed through the same cap. Oversized SQL/BLOB regressions added; NOT RUN.
- No build/test/CI/verifier/platform matrix ran. No exact-SHA evidence or release status promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `edd51a01b955ace5caeb993595d6e27bd8c940fe`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — SQLite execution deadline and busy timeout

- Latest source candidate: `91d2a6c3a49041284944a4dd12ae090bb783bd0d` (NOT BUILT/TESTED); rusqlite hooks feature enabled by `f6716d2c9dc83208de695c699804da7ccde9aec0`.
- Each SQLite query connection now has a 5-second VM progress-handler deadline and a 250 ms busy/lock wait timeout, in addition to the previously recorded SQL/parameter/row/output caps and SQLite engine limits.
- No build/test/CI/verifier/platform matrix ran. No exact-SHA evidence or release status promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `7260f992c298668ee6c83b253c016167cb7ea4ec`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — SQLite deadline includes result serialization

- Latest source candidate: `75bf347d7bbb9cce89a488b02cc285c7f605e984` (NOT BUILT/TESTED).
- The 5-second SQLite deadline now covers both SQLite VM execution through the progress handler and Rust-side row/result serialization; busy/lock waits remain capped at 250 ms.
- No build/test/CI/verifier/platform matrix ran. No exact-SHA evidence or release status promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `0039ee6cd4a969ee264dad365b819f187d919d38`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — additional SQLite parser and VM limits

- Latest source candidate: `4c01b6eb2b8ba2d75a316a1a792352657a74c73a` (NOT BUILT/TESTED).
- SQLite per-connection resource profile now also caps parser depth at 1,000, VDBE instructions at 1,000,000, and trigger recursion at 0, in addition to the previously recorded SQL/value/row/output/parameter and execution-time bounds.
- No build/test/CI/verifier/platform matrix ran. No exact-SHA evidence or release status promoted.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `20d16862bc7f51d0a5122884eb31223fc1b7bc7d`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — Android nested-read source implementation

- Latest source/test candidate: `52fcfb2706fcb48e9599d75f9086578279f1bbac` (`crates/one-core/src/filesystem.rs`, NOT BUILT/TESTED).
- Android `open_relative` now resolves the entire relative path through kernel `openat2` with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`; it then enforces regular-file, single-link, device, and `STATX_MNT_ID` checks. This enables nested reads without relying on a racy userspace parent walk.
- Android nested writes remain fail-closed until an anti-move atomic replacement backend exists. macOS nested reads and writes remain fail-closed; ordinary `openat` descriptors do not pin a parent beneath the root.
- The Android duplicate `create_temp` was already removed and macOS helper added; latest static inventory confirms one definition in each backend. No compiler was run.
- Added Android nested-read + symlink-parent escape regression; NOT RUN. No build/test/CI/verifier/platform matrix ran. Issue #104 remains OPEN / NOT ACCEPTED.
- PR #113 was OPEN / DRAFT / UNMERGED at pre-register read HEAD `95c2513d793a4c82440099771d457615c6a6531e`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. This register update advances branch HEAD; refresh before next action.


## 2026-11-21 — guarded Android nested writes and macOS nested reads

- Code-bearing source candidate: `9fe53815f83701a62536472f8a3c87cebac22e7a` (`crates/one-core/src/filesystem.rs`). NOT BUILT/TESTED.
- Android nested writes now call the shared detached-mount transaction used by Linux. It clones the retained root mount, applies recursive `MOUNT_ATTR_NOSYMFOLLOW`, resolves under the clone with `openat2` (`BENEATH | NO_SYMLINKS | NO_XDEV`), writes an exclusive temporary file, and atomically publishes with `renameat2`; unsupported/denied primitives fail closed. Android nested reads remain kernel-resolved through `openat2`.
- macOS nested reads now use XNU `O_RESOLVE_BENEATH | O_NOFOLLOW_ANY | O_UNIQUE` and post-open `fstatfs` identity checks. Unsupported kernels fail closed. macOS nested writes remain fail-closed pending a proven atomic no-cross-mount publication strategy.
- Source tests were updated for capability-gated Android nested write behavior and supported-kernel macOS nested reads; NOT RUN. No compiler, Android device test, macOS test, CI run, or E4 verifier result is claimed.
- PR #113 remains OPEN / DRAFT / UNMERGED. Latest PR HEAD before this register update: `70a3fa7239a93738c0d70358e4287692fc5e0e09`. Base remains `181ad350d8f0db53cda654c4d378425fbdd6ab24`. Issue #104 remains OPEN / NOT ACCEPTED.


## 2026-11-21 — filesystem implementation and first exact-SHA CI feedback

- Latest source candidate: `568d9360f0beec63c10fbeae9cac63f384bd082f` (`crates/one-core/src/filesystem.rs` plus Clippy corrections in `adapter.rs` and `database.rs`). NOT BUILT/TESTED on the updated candidate.
- Android nested read remains kernel `openat2`; nested write now uses the guarded detached-mount transaction with recursive `MOUNT_ATTR_NOSYMFOLLOW`, `openat2` and `renameat2`. Unsupported/SELinux-denied primitives fail closed. macOS direct-child read compatibility is preserved; nested reads use XNU `O_RESOLVE_BENEATH | O_NOFOLLOW_ANY | O_UNIQUE` plus `fstatfs` checks on supported kernels; nested writes remain fail-closed pending a proven atomic no-cross-mount publication path.
- Exact-SHA GitHub Actions run #2410 (`44845255637`) ran on earlier source SHA `9fe53815f83701a62536472f8a3c87cebac22e7a`: Ubuntu Build SUCCESS, Clippy FAILURE, Tests SKIPPED. Clippy reported a manual char-comparison lint in `adapter.rs` and ignored SQLite configuration `Result`s in `database.rs`. Descendant source commits `82a90b41fec74bd3829da89ff2b4cb872d451c83` and `c23bd0e09186254aade3f8665e622bc9d2846eab` correct these without weakening lint policy; macOS direct-child fallback was refined in `568d9360f0beec63c10fbeae9cac63f384bd082f`. These fixes have not yet been shown to pass CI.
- Windows leg of run #2410 was still IN_PROGRESS at `Install Android NDK (Windows)` on last readback; no Windows result is claimed. CircleCI for the earlier source candidate remained PENDING; no CircleCI success is claimed for the latest candidate.
- Android native tests, macOS native tests, focused platform builds, and adversarial nested-write race/mount tests remain outstanding. Issue #104 remains OPEN / NOT ACCEPTED. PR #113 remains OPEN / DRAFT / UNMERGED; no main, verifier pin, fixed ledger, or acceptance criteria changes.


## 2026-11-21 — Windows build blocker correction after run #2410

- Latest code-bearing source candidate: `74df3aa901732c8eb3adaac728162715fc3a4961` (NOT BUILT/TESTED).
- Exact-SHA run #2410 (`44845255637`) on parent source `9fe53815f83701a62536472f8a3c87cebac22e7a` ended with Ubuntu Build SUCCESS / Clippy FAILURE / Tests SKIPPED and Windows Build FAILURE. Windows errors showed Linux-only writer functions were compiled on Windows (missing Unix std APIs/libc/Linux helpers) and unstable `std::os::windows::fs::MetadataExt::{volume_serial_number,file_index}` calls.
- Corrected in source descendants: `74df3aa901732c8eb3adaac728162715fc3a4961` gates Linux writer functions to Linux only and replaces unstable Windows metadata APIs with a no-reparse NT handle reopen plus `GetFileInformationByHandle` volume/file-index comparison. Earlier Clippy corrections remain in `82a90b41...` and `c23bd0e0...`.
- No rerun on the corrected candidate is confirmed. Android native tests and macOS native tests remain outstanding; no platform support is promoted. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 remains OPEN / DRAFT / UNMERGED.


## 2026-11-21 — corrected candidate queued for fresh CI

- PR #113 HEAD before this register update: `8248d9d7ffd2dcf15b0708c6cf6383f943ec5e38`; base: `181ad350d8f0db53cda654c4d378425fbdd6ab24`; OPEN / DRAFT / UNMERGED.
- Latest code-bearing source candidate: `74df3aa901732c8eb3adaac728162715fc3a4961` (NOT BUILT/TESTED locally). The seven commits after it at this point are documentation-only.
- GitHub Actions run #2425 (`44845652963`) is IN_PROGRESS on the corrected source candidate; Ubuntu and Windows jobs were both still in progress at last readback. No result claimed.
- CircleCI context `ci/circleci: one-linux` for source SHA `74df3aa901732c8eb3adaac728162715fc3a4961` is PENDING. No CircleCI result claimed.
- User-confirmed Termux worktree last reached `1e8e19dcf3833aaff7169427350fa50b7df1f545`; it must safely fast-forward to the current remote before building. Do not overwrite changes.
- Issue #104 remains OPEN / NOT ACCEPTED. No broad verification or release acceptance is promoted.


## 2026-11-21 — Termux Android compile blockers on exact SHA and source correction

- User-confirmed clean $HOME/one-pr113 worktree was fast-forwarded to PR #113 HEAD 04143d1a9b11f55e1f2ae38960f35d32d2904a11; Rust 1.99.1, host aarch64-linux-android.
- On exact SHA 04143d1a9b11f55e1f2ae38960f35d32d2904a11, both cargo clippy --workspace --all-targets -- -D warnings and cargo test -p one-core filesystem::android_filesystem_tests -- --nocapture stopped at compilation with two errors: Android nested writer called File::write_all without importing std::io::Write for Android; Android Statx derived Default with [u8; 76], unsupported by this Android standard-library toolchain.
- Corrected source on PR branch in commit ed30f41fb9e69392c926a83dd9a8b6ac2e61efc4: scope std::io::Write to Linux + Android, and split the 76-byte Statx reserved ABI padding into [u8; 32] + [u8; 32] + [u8; 12]. The total padding and 256-byte Statx layout are preserved; compile-time size assertion remains.
- Commit ed30f41fb9e69392c926a83dd9a8b6ac2e61efc4 is NOT YET BUILT/TESTED. User must fast-forward the clean Termux worktree and rerun both commands on this exact new SHA. No test, Clippy, platform-support, or E4 acceptance claim is made.
- Issue #104 remains OPEN / NOT ACCEPTED. PR #113 remains OPEN / DRAFT / UNMERGED. No main, verifier pin, tests, or acceptance criteria changed.


## 2026-11-21 — Android Termux rerun on exact SHA 8b3cb521

- User-confirmed clean `$HOME/one-pr113` fast-forwarded to `8b3cb52168ed8d4961ce4e47bcc3410f9a0fd7ff`; rustc/Cargo 1.99.1, host `aarch64-linux-android`.
- Workspace Clippy progressed through `one-core` and failed in `crates/one-cli/tests/filesystem_write.rs`: helper `run` is unused on Android because its only caller is Linux/Windows cfg-gated. Corrected by cfg-gating the helper itself to Linux/Windows in commit `f9dd400b21670fd07c5498307714a9b7ea931d18`. NOT YET BUILT/TESTED.
- Focused Android test binary compiled successfully and started five tests. `android_root_final_symlink_is_rejected` and another test shown in output passed; `android_hard_link_targets_are_rejected_without_modifying_either_name` failed at fixture setup because `std::fs::hard_link` returned `PermissionDenied` (code 13). The test body never reached the authority assertions. The process was then terminated with `SIGSYS` (signal 31, bad system call), so remaining tests are NOT VERIFIED. Do not weaken or delete the hard-link test. Investigate Android/Termux syscall restrictions and rerun tests individually to identify which operation triggers SIGSYS.
- This demonstrates Android compilation is now possible on this toolchain, but the focused Android suite is NOT PASS. No claim of Android filesystem support or E4 acceptance.
- Next exact-SHA run after safe sync: workspace Clippy, then each Android test individually (especially hard-link fixture, nested read, nested write) to isolate permission and SIGSYS. Preserve test assertions and record capability limitations separately from implementation correctness.
- Issue #104 remains OPEN / NOT ACCEPTED; PR #113 remains OPEN / DRAFT / UNMERGED.


## 2026-11-21 — Termux exact-SHA Android test isolation and syscall investigation

User-provided Termux evidence on exact PR #113 SHA `4e831bf51e4c9f4e4075e30d3d1e4fc72eeded0b` (host previously identified as `aarch64-linux-android`):

- `cargo clippy --workspace --all-targets -- -D warnings`: PASS; completed successfully in 2.37s with no warnings.
- The test filters were corrected by removing `--exact`; each selected unit test then ran (1 test selected, not zero).
- `filesystem::android_filesystem_tests::android_root_final_symlink_is_rejected`: PASS (1 passed).
- `android_direct_child_read_and_atomic_write_work_with_root_handle`, `android_leaf_symlinks_and_nested_paths_fail_closed`, and `android_kernel_resolved_nested_reads_stay_beneath_root`: each process terminated with signal 31 / `SIGSYS: bad system call`. These are runtime terminations, not assertion failures; their security assertions did not complete.
- `android_hard_link_targets_are_rejected_without_modifying_either_name`: failed before authority assertions because fixture setup `std::fs::hard_link` returned `PermissionDenied` (errno 13). Do not weaken, skip, or delete the security test to manufacture a pass.

Source-path investigation at this exact branch source: successful `FilesystemAuthority::open` on Android calls `android_native::mount_id(&root)`, which directly invokes `SYS_statx` with `AT_EMPTY_PATH | STATX_MNT_ID`. The three SIGSYS tests all proceed through successful authority construction; the passing root-final-symlink test rejects the symlink before this call. This makes Android `statx` the leading syscall suspect, but the signal alone does not prove the kernel/seccomp cause. Obtain syscall-level evidence (e.g. `strace`, if available and permitted) before changing the backend.

Do not silently fall back to device/inode identity or omit mount identity: mount-boundary confinement is part of the security contract. Investigate a kernel-derived mount-ID source compatible with Android restrictions (for example, `/proc/self/fdinfo/<fd>` `mnt_id`, subject to explicit robustness/security review and tests) before proposing code. Any fallback must fail closed if the kernel source is unavailable or malformed.

Exact-SHA classification: Clippy PASS; one Android test PASS; three Android tests SIGSYS/NOT VERIFIED; hard-link test BLOCKED at fixture setup; Android filesystem backend NOT VERIFIED / NOT ACCEPTED. Issue #104 remains OPEN / NOT ACCEPTED. PR #113 remains OPEN / DRAFT / UNMERGED. No acceptance or platform support is promoted.


## 2026-11-21 — syscall trace correction: Android seccomp blocks openat2

This entry supersedes the earlier statx-suspect hypothesis recorded immediately above. User supplied a Termux ARM64 `strace` trace for exact test `filesystem::android_filesystem_tests::android_direct_child_read_and_atomic_write_work_with_root_handle` on source/test SHA `4e831bf51e4c9f4e4075e30d3d1e4fc72eeded0b`:

- `statx(3, "", AT_STATX_SYNC_AS_STAT|AT_EMPTY_PATH, STATX_MNT_ID, ...) = 0`: the kernel successfully returns `STATX_MNT_ID`; the prior hypothesis that `statx` caused SIGSYS is disproven for this invocation.
- `openat2(3, "data.txt", ..., resolve=RESOLVE_NO_XDEV|RESOLVE_NO_SYMLINKS|RESOLVE_BENEATH, ...) ` is invoked.
- Immediately afterward, the process receives `SIGSYS` with `si_code=SYS_SECCOMP`, `si_syscall=__NR_openat2`, `si_arch=AUDIT_ARCH_AARCH64`; the process is killed by SIGSYS.
- Confirmed cause for this traced test: Android/Termux seccomp policy blocks `openat2`. This is not a Rust assertion failure and not a `statx` failure.

Security constraints and next engineering action:
- Do not replace `openat2` with an unchecked `openat`, lexical path validation, device/inode-only checks, or omission of `RESOLVE_BENEATH` / symlink / mount-boundary protections.
- Perform a design review for a safe Android fallback using retained directory handles and component-by-component `openat` with no-follow semantics, validating every opened descriptor's kernel mount identity before reading or writing. Prove resistance to symlink swaps, mount-point crossings, directory replacement, and race/manipulation; fail closed if any required kernel identity or primitive is unavailable.
- The trace only directly confirms the first named test's `openat2` termination. The other previously SIGSYS-terminated Android tests remain unverified until their syscall paths are independently confirmed or the shared path is changed and retested.
- The hard-link regression remains blocked at fixture setup by `PermissionDenied`; do not weaken or skip it. Clippy PASS and the root-final-symlink test PASS remain evidence only for their recorded exact SHA.
- No implementation fix, Android support, E4 verification, or acceptance is claimed by this trace. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 remains OPEN / DRAFT / UNMERGED.


## 2026-11-21 — Android seccomp-safe capability boundary (code candidate)

- User-provided Termux ARM64 syscall trace proves `statx(STATX_MNT_ID)` succeeds and `openat2` is killed by seccomp with `SIGSYS / SYS_SECCOMP`. Do not probe `openat2` in-process on Android.
- Code candidate commit: `e5c8a7d5d2075902ef4f027322e2e615908bac14` (source `crates/one-core/src/filesystem.rs`). Android reads now use root-handle-relative `openat` only for a single direct child, with `O_NOFOLLOW | O_NONBLOCK`, regular-file/single-link checks, device identity, and `statx(STATX_MNT_ID)` equality. Nested reads return `UnsupportedPlatform` before any syscall that may trigger seccomp.
- Android nested writes now return `UnsupportedPlatform` before invoking the Linux detached-mount/`openat2` transaction. Direct-child atomic writes remain root-dirfd-relative and retain the existing no-follow, regular-file, hard-link, device, mount-ID, exclusive-temp, sync, and atomic-rename checks.
- Android tests were updated: direct-child read/write remains positive; nested reads (including symlink-parent paths) and nested writes assert fail-closed behavior; outside sentinel bytes remain unchanged. The old nested-read test name was replaced with `android_nested_reads_fail_closed_without_openat2`.
- Rationale: a component-by-component `openat` walk is not an acceptable nested fallback because an opened parent directory can be renamed outside the retained root before the leaf operation. This preserves the security boundary at the cost of nested Android functionality.
- **NOT YET BUILT OR TESTED** on this code SHA. Termux test execution, full target builds, CI, and independent verifier checks remain outstanding. Do not promote Android support or E4 acceptance. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 remains OPEN / DRAFT / UNMERGED.
