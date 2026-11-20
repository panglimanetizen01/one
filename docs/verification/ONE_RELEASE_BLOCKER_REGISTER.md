# ONE RELEASE BLOCKER REGISTER

**Status:** ACTIVE — single release-sprint work queue  
**Last verified:** 2026-11-20  
**ONE PR:** [#113](https://github.com/panglimanetizen01/one/pull/113)  
**Verifier PR:** [#51](https://github.com/panglimanetizen01/one-windows-verification/pull/51)

> This register controls priority only. It does not replace or modify the release contract, specifications, fixed acceptance ledger, Definition of Done, or evidence matrix. Those remain authoritative for acceptance. Update this register instead of creating another blocker list or duplicate roadmap.

## 1. Last verified identity — refresh before acting

| Item | Exact value |
|---|---|
| ONE canonical `main` | `d35db95153f6ddd79885494c72f5aee16c899751` |
| PR #113 branch | `security/issue-104-root-confinement-20261118` |
| PR #113 HEAD | `509b7b197115adb1ba80d730342e9f6fbf981645` |
| PR #113 base SHA | `181ad350d8f0db53cda654c4d378425fbdd6ab24` |
| Code-bearing source | `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` |
| Verifier PR #51 HEAD | `6c0979e62eb01e4b929c48b4224a3d4f742bc8af` |
| Verifier branch-local source pin | `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` |
| Protected verifier source pin | `722d662e7a81711ba3607d26571f4048a292b3ed` |

PR #113 and #51 were OPEN / DRAFT / UNMERGED at the last readback. Refresh all live refs, issue states, status contexts, and provider results before using this snapshot.

## 2. Active blockers

### B-001 — CircleCI pre-execution termination

**Priority:** P2 · **State:** OPERATIONALLY RESOLVED ON SHA `979973cd...`; HISTORICAL ROOT CAUSE UNKNOWN  
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
**Latest code-bearing candidate:** `8dd92aebc033a94abb14a8d7ec498f6933d4ba8e` (NOT BUILT/TESTED); latest PR #113 HEAD is `202f3ca0ced5499e3b9db2a54b94e48ad4463c41`.

Exact-source verifier evidence already recorded in the evidence matrix includes E4 adversarial filesystem, Linux distro, Windows native runtime, SEC-05, resource guard, artifact integrity, focused standard-library/database, E1 Windows, and Android API 35 native-runtime evidence. Preserve these results; do not rerun without a changed source/verifier input or a documented technical reason.

Remaining boundaries:
- Linux nested writes remain fail-closed.
- Android/macOS filesystem adapters remain unavailable.
- Android native-backend runtime evidence does not prove Android filesystem adapter support.
- Verifier success does not itself constitute Issue #104 acceptance.

**Next action:** reconcile every fixed Issue #104 acceptance criterion against exact candidate source and terminal evidence. Implement only demonstrated gaps; keep unsupported behavior fail-closed. Do not alter criteria, fixed ledger weights, protected verifier pin, or DoD checkboxes.

### B-003 — Formal v0.1 release gates

**Priority:** P1 · **State:** OPEN / NOT RELEASE-APPROVED**

The v0.1 release contract, completion addendum, fixed ledger/DoD, and E1–E6 evidence remain authoritative. Reconcile every remaining gate against its exact-SHA evidence. Do not infer release readiness from a subset of green workflows or documentation completion. Issue #105 remains an open engineering item; classify it as release-blocking only if the normative contract/ledger says so.

**Next action:** produce a gate-by-gate closeout from existing records. Do not change gate weights or acceptance rules.

### B-004 — Architecture and documentation traceability

**Priority:** P2 · **State:** PARTIALLY VERIFIED**

- Existing Mermaid source: [filesystem-root-confinement.md](../architecture/filesystem-root-confinement.md).
- Full overview image remains in draft [PR #97](https://github.com/panglimanetizen01/one/blob/docs/system-architecture-diagram-20261115/docs/architecture/one-full-architecture-map.png); its binary has not been visually inspected in this session.
- Do not recreate/re-upload a duplicate diagram. Update existing Mermaid source only if architecture changes, then synchronize links and evidence.

## 3. Release sprint execution order

1. **P0:** Diagnose CircleCI pre-execution termination using provider evidence; no speculative config patch or blind rerun.
2. **P1:** Close Issue #104 against its unchanged acceptance contract; reuse valid exact-source evidence and verify any new code on a new exact SHA.
3. **P1:** Reconcile every remaining formal v0.1 release gate from the existing contract and fixed ledger.
4. **P2:** Freeze one release-candidate SHA and run the required integrated platform matrix. Inspect terminal job results/logs. Parent-SHA evidence never transfers.
5. **P2:** Synchronize only affected existing docs: specs, roadmap/addendum, implementation status, evidence matrix, verification index/checkpoint, user feature status, architecture source, changelog, docs map, and Notion Skill.
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
