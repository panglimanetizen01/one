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
- Job `one-linux` #836 status `failed`; `started_at=null`, `stopped_at=09:47:03.544Z`, no duration, no dependencies.
- API v3 exposes step #101 `CANCELED` / `Error: Task information unavailable`; stdout/stderr are empty.
- Pipeline config endpoint returns source and compiled configuration strings; setup-config fields are absent. Source config SHA-256: `b7827cd95925851482b833a5f45033e1b26ef0f731857b29e2df0821e08fd155`; compiled config SHA-256: `9f3f77d48351a10c3222a4a1b79634d30de29038c562c9c0e548046026b85aee`.
- Compiled config includes `one-linux`, machine image `android:default`, resource class `large`. Config presence/compilation payload does not prove executor provisioning or job start.
- Diagnostic script on the same target SHA contains a classifier false positive: its report text says no `failed/timed_out` step was exposed, then its classifier scans the word `timed_out` in that explanatory sentence and labels the report `TIMEOUT / NO-OUTPUT`. This is not timeout evidence; preserve the actual canceled-step/no-start facts.

**CircleCI account/settings evidence — 2026-11-20**
- Project Advanced screenshot: **Free and Open Source ON**, **Block all new work OFF**, **Only build pull requests OFF**, dynamic config ON.
- Plan screenshot: Free plan; usage snapshot shows 110,089 of 30,000 regular credits used. The page explicitly states OS project contributions are not subject to the 30,000-credit limit and advertises up to 400,000 monthly credits for Linux open-source builds.
- The screenshot does not show remaining/consumed balance for the separate 400,000 OSS allocation. An earlier provider error was recorded as `free-plan-no-credits-available`; the screenshots do not independently prove whether that OSS allocation was exhausted or whether the error still applies.
- Personal API token authentication is valid (`/api/v2/me` HTTP 200); project metadata read is HTTP 200. Project settings API returns HTTP 403 Permission denied for both `gh` and `github` slugs. Do not interpret that as OSS OFF.
- Do not buy credits, upgrade, or toggle settings. Do not retry CircleCI while the recorded no-credit provider block remains unresolved.

**Classification:** confirmed pre-execution termination / no job start. It is NOT a test timeout, test assertion failure, compiler failure, or source regression. Root cause remains UNKNOWN; credit eligibility/allocation remains unresolved.

**Next action:** obtain provider-side failure detail for #1201 and resolve the OSS allocation/eligibility block using authenticated CircleCI access. Do not patch CircleCI config or rerun blindly.

**Separate GitHub Actions bootstrap route**
- PR #115 `ci/bootstrap-actions-on-fix-windows-verifier-20261120` is OPEN / NOT MERGED; base remains `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Initial run #2243 failed on both OS jobs because the PR #113 contract checker correctly rejected the newly introduced `.github/workflows/ci.yml`; Build/Clippy/Tests did not run.
- Corrective workflow commit on PR #115: `3b684acd1f3e68f1b40dd6d600fad0e013044495`, blob `35015ded4f7635ea37ec2b035f2ba0c33d7d4bf5`. It preserves merge-result checkout for normal PRs, uses exact PR head checkout for the stacked base branch, and skips only the contract check on the bootstrap PR itself. Full contract check remains enabled for PR #113.
- New GitHub Actions run #2244 (`44748865610`) is IN_PROGRESS on the corrective commit; Ubuntu and Windows jobs have started and were installing cross-toolchains at latest readback. This is not a PASS and does not resolve CircleCI B-001.
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
