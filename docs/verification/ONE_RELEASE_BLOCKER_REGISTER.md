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

**Priority:** P0 · **State:** OPEN / ROOT CAUSE UNKNOWN  
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

**Next action:** obtain provider-side failure detail for #1201 and resolve OSS allocation/eligibility using authenticated CircleCI access. Do not patch CircleCI config or rerun blindly.

**Separate GitHub Actions bootstrap route**
- PR #115 `ci/bootstrap-actions-on-fix-windows-verifier-20261120` is OPEN / NOT MERGED; base remains `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Initial run #2243 failed on both OS jobs because the PR #113 contract checker rejected the newly introduced `.github/workflows/ci.yml`; Build/Clippy/Tests did not run.
- Corrective workflow commit on PR #115: `3b684acd1f3e68f1b40dd6d600fad0e013044495`, blob `35015ded4f7635ea37ec2b035f2ba0c33d7d4bf5`. It preserves merge-result checkout for normal PRs, uses exact PR head checkout for the stacked base branch, and skips only the contract check on the bootstrap PR itself. Full contract check remains enabled for PR #113.
- GitHub Actions run #2244 (`44748865610`) is terminal SUCCESS on exact PR #115 HEAD `3b684acd1f3e68f1b40dd6d600fad0e013044495`. Ubuntu and Windows jobs both completed SUCCESS; Build, Clippy, workspace tests, and all focused tests passed. The bootstrap PR's Contract artifacts step was intentionally skipped; it is not claimed as executed on this bootstrap branch. [Run #2244](https://github.com/panglimanetizen01/one/actions/runs/44748865610).
- CircleCI status on the same PR #115 SHA is FAILURE at workflow `e6e6a97c-2cda-4a7a-a5ff-3b0c9129d9ec`, job `667d82b1-c817-41f3-91f2-5bd09b55e879`; provider logs are inaccessible in this session, so root cause is not inferred. This does not change the separate B-001 mapping for PR #113 pipeline #1201.
- PR #113 remains OPEN / DRAFT / UNMERGED at `509b7b197115adb1ba80d730342e9f6fbf981645`; base remains `181ad350d8f0db53cda654c4d378425fbdd6ab24`.

### B-002 — Issue #104 security acceptance

**Priority:** P1 · **State:** OPEN / NOT ACCEPTED  
**Candidate code source:** `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`

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
