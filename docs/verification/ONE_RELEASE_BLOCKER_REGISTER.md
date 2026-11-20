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
| PR #113 HEAD | `a0d94bc9a26cf9effd4b202f29af50f81c8e4ee5` |
| PR #113 base SHA | `181ad350d8f0db53cda654c4d378425fbdd6ab24` |
| Code-bearing source | `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` |
| Verifier PR #51 HEAD | `6c0979e62eb01e4b929c48b4224a3d4f742bc8af` |
| Verifier branch-local source pin | `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` |
| Protected verifier source pin | `722d662e7a81711ba3607d26571f4048a292b3ed` |

PR #113 and #51 were OPEN / DRAFT / UNMERGED at the last readback. Refresh all live refs, issue states, status contexts, and provider results before using this snapshot.

## 2. Active blockers

### B-001 — CircleCI pre-execution termination

**Priority:** P0 · **State:** OPEN / ROOT CAUSE UNKNOWN  
**Affected candidate:** PR #113 docs HEAD `a0d94bc9a26cf9effd4b202f29af50f81c8e4ee5`

| Run | Pipeline | Workflow | Job | Pipeline revision | Job start |
|---|---:|---|---|---|---|
| Current | #1199 | `916983e9-dbde-4db2-a967-438a666ebb19` | #834 / `a130c5d9-c078-4c18-8ae5-571470f46d60` | `a0d94bc...` | `started_at=null` |
| Previous | #1198 | `5ed6d9e7-976e-4322-b7ba-6deea915e1a9` | #833 / `8d2d756e-9297-4a00-9a7d-a0c9bd734cba` | `5daf333...` | `started_at=null` |

Facts:
- API v2 confirmed each job belongs to its listed workflow and each workflow maps to the listed pipeline revision.
- Both workflows stopped about 0.4–0.5 seconds after creation. Pipeline #1199 reported state `created`; workflow/job reported `failed`.
- Available step record: #101 CANCELED, `Task information unavailable`, stdout/stderr empty.
- Local Termux diagnostic script blob equals the remote PR #113 blob: `751c25e0b4a4227638c898043ce175d62d523834`.
- Local checkout HEAD is `70ae27cd9dab4429c528d6f01a42377f474c725c`, 76 commits behind the remote branch at inspection. Do not reset/pull/rebase or overwrite without reviewing the diff and permission.

**Classification:** pre-execution termination / no job start. This is NOT proven to be a test timeout, test failure, compiler failure, or source regression. Root cause is UNKNOWN.

**Next action:** obtain redacted provider-side event/failure details and pipeline configuration/compiled-configuration evidence for #1199; compare with #1198. Determine whether pipeline creation, config setup/compilation, workflow scheduling, or job provisioning failed. Change config only after evidence identifies the cause. Do not blindly rerun.

**Separate positive evidence:** CircleCI `ci/circleci: one-linux` SUCCESS on code-bearing SHA `fb5c8bc...` (workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`). Provider step logs were not inspected. This PASS does not transfer to docs HEAD `a0d94bc...`.

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
