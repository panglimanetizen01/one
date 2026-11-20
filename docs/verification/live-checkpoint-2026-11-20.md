# Live continuation checkpoint — 2026-11-20

Snapshot readback: 2026-11-20 01:56 UTC (08:56 WIB). This checkpoint supersedes older live-status pointers where they conflict; historical evidence remains preserved.

## Repository and exact revision identity

### ONE

- Repository: `panglimanetizen01/one`
- PR #113: OPEN / DRAFT / UNMERGED
- Branch: `security/issue-104-root-confinement-20261118`
- Documentation HEAD at readback: `71d518ae4b9aa3b1bf05c738e9d6e862883ffcd0`
- Code-bearing source: `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`
- Base SHA: `181ad350d8f0db53cda654c4d378425fbdd6ab24`
- Canonical `main`: `d35db95153f6ddd79885494c72f5aee16c899751` (confirmed unchanged)

A GitHub compare from source `fb5c8bc...` to documentation HEAD `71d518ae...` reports 12 commits ahead, 0 behind; the net changed paths are documentation-only. The code-bearing source and its tests/config are unchanged across this documentation-only range.

### Independent verifier

- Repository: `panglimanetizen01/one-windows-verification`
- PR #51: OPEN / DRAFT / UNMERGED
- Branch: `probe/issue-104-platform-metadata-20261120`
- Current verifier HEAD: `e5632e75c473546527c7e9a404262631d827f429`
- Branch-local `CANONICAL_COMMIT.txt`: `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`
- Branch `verification`: `b14abf0020472dc45842be30854e0a09317d6cb4`
- Protected-source pin file: `722d662e7a81711ba3607d26571f4048a292b3ed`
- Protected ledger source pin: `d6c382a245c49a028bc60809f22ca956c7f894cc`

**Branch-protection caveat:** GitHub branch metadata currently reports `protected=false` for branch `verification`; the repository rulesets endpoint returned an empty list. The classic branch-protection endpoint returned 403 to this integration, so actual protection configuration is NOT VERIFIED. Do not describe GitHub-side protection as confirmed and do not change branch settings without explicit authorization. The branch SHA and pin values above are read-only observations.

## CircleCI exact-SHA reconciliation

- Source SHA `fb5c8bc...`: `ci/circleci: one-linux` SUCCESS; workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`; job `c4faf162-80e9-4f17-a83a-a72634bb592f`. Provider step logs were not inspected.
- Documentation HEAD `71d518ae...`: `ci/circleci: one-linux` FAILURE; workflow `980d47d2-704a-426f-87b4-b0687657f42d`; job `a288a97c-38ff-448b-b74d-8abcea612342`.
- Failed step and root cause for the documentation HEAD are UNKNOWN. The job page is inaccessible through the connected browser, and no CircleCI CLI/MCP log action is exposed in this session. Do not guess, rerun blindly, or change configuration based on a hypothesis.
- Current `.circleci/config.yml` blob at PR #113 is `ec1bdd5277d193b2e0fcf88f0557897517f5687a`. Diagnostic utility: `tools/ci/diagnose_circleci.py`; it redacts token-like values and supports exact-SHA discovery.
- Local redacted diagnostic command, run from the ONE repository root after setting `CIRCLECI_TOKEN` in the local environment (never paste the token into chat):

```bash
python3 tools/ci/diagnose_circleci.py \
  --sha 71d518ae4b9aa3b1bf05c738e9d6e862883ffcd0 \
  --branch security/issue-104-root-confinement-20261118 \
  --output "$HOME/one-circleci-71d518ae.txt"
```

The diagnostic unit tests and `circleci config validate` have NOT been executed in this connected session on the latest documentation HEAD. The user-documentation tree was statically inspected for the checker's internal-link and status-label rules, but that is not a substitute for running the repository checker or CircleCI job.

## Independent verifier — fresh run status on verifier HEAD e5632e...

All listed workflows pin the exact ONE source `fb5c8bc...`. These results do not transfer to a different source SHA or to `main`.

| Scope | Run | Current result |
|---|---:|---|
| E4 adversarial filesystem | [44656893733](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893733) | SUCCESS — Unix no-follow/resource bounds; Windows reparse/handle; native `OBJ_DONT_REPARSE` probe |
| Linux distro | [44656893931](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893931) | SUCCESS — Debian 12 and Ubuntu 24.04 |
| Windows native backend runtime | [44656893882](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893882) | SUCCESS |
| SEC-05 subprocess timeout/output | [44656893697](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893697) | SUCCESS — Linux and Windows |
| E4 subprocess resource guard | [44656893627](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893627) | SUCCESS |
| Candidate artifact integrity | [44656893650](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893650) | SUCCESS |
| E1 Windows acceptance | [44656893639](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893639) | SUCCESS |
| Focused standard-library/database | [44656893788](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893788) | SUCCESS |
| Cross-platform aggregate | [44656893832](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893832) | SUCCESS — Ubuntu and Windows jobs terminal |
| Android AArch64 build | [44656893715](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893715) | SUCCESS |
| Android API 35 ARM64 runtime | [44656893597](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893597) | IN PROGRESS — build job passed; runtime job `136025758614` still running |
| E1 Milestone 1 aggregate | [44656893655](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44656893655) | IN PROGRESS — Windows passed; Ubuntu job `136025210968` still running |

Earlier Android API 35 runtime run [#44652693606](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44652693606) completed SUCCESS for the same ONE source `fb5c8bc...` on the prior verifier HEAD `226360d...`; its runtime log recorded ARM64 ELF execution on API 35, exit 0, and `ONE_NATIVE_BACKEND_OK`. The fresh run on verifier HEAD `e5632e...` is still in progress and must not be prematurely marked complete.

## Issue #104 acceptance boundary

- Hard-link rejection is IMPLEMENTED / TESTED / VERIFIED on Linux and Windows for source `fb5c8bc...`; the test checks read and write rejection and unchanged bytes through both in-root and outside hard-link names.
- E4 evidence includes real Linux mount-boundary, no-follow, traversal, parent-symlink race, direct-child write, and hard-link cases; Windows junction/reparse-point, absolute in-root read, outside-root no-side-effect, atomic write, size-limit, replacement-preservation, parent traversal, and hard-link cases.
- Linux nested writes remain fail-closed. Android/macOS filesystem adapters remain unavailable. Android native runtime evidence does not imply Android filesystem adapter support.
- Windows native reparse evidence is limited to Windows Server 2025 build 26100.
- Issue #104 remains OPEN / NOT ACCEPTED. No fixed-weight ledger, DoD checkbox, protected source pin, release status, or merge state is changed.

## Architecture and documentation inventory

- ONE recursive tree at PR #113 HEAD contained 247 tracked entries and no standalone SVG/PNG/draw.io/PlantUML diagram assets. The focused Mermaid source is `docs/architecture/filesystem-root-confinement.md`; it was read and mapped to `filesystem.rs`, `adapter.rs`, `interpreter.rs`, and CLI host-root selection.
- Verifier recursive tree at PR #51 HEAD contained 90 entries and no standalone diagram assets.
- Full architecture image remains in draft PR #97, HEAD `029471e0396504c030db60833fcda5b5baff24dd`. Its binary image could not be visually inspected in this session; no visual-audit PASS is claimed.
- The active Issue #104 documentation set, repository operating contract, top-level architecture/security/platform contracts, user-facing filesystem status, verifier evidence, and CI diagnostic/config documents were reviewed. This is not a claim that every unrelated historical language spec and plan in the repository received a full semantic review.

## Anti-duplication matrix

| Area | Classification | Action |
|---|---|---|
| E4 Linux/Windows adversarial suite on source `fb5c8bc...` | VERIFIED | Do not rerun without a relevant source/workflow input change |
| Linux distro, Windows runtime, SEC-05, resource guard, artifact integrity, focused stdlib/database | VERIFIED | Preserve exact run evidence; no duplicate run |
| Cross-platform aggregate on source `fb5c8bc...` | VERIFIED | Preserve run #44656893832 |
| Android API 35 runtime on fresh verifier HEAD `e5632e...` | IN PROGRESS | Wait for terminal result; inspect runtime output once |
| E1 Milestone 1 on fresh verifier HEAD `e5632e...` | IN PROGRESS | Wait for Ubuntu job terminal result |
| CircleCI on docs HEAD `71d518ae...` | FAILED / ROOT CAUSE UNKNOWN | Obtain redacted exact-SHA failure report before any config/source fix |
| Issue #104 acceptance | OPEN / NOT ACCEPTED | Keep nested writes/platform gaps fail-closed |
| Full architecture image visual inspection | NOT VERIFIED | Do not claim diagram audit complete |
| GitHub branch protection on verifier `verification` | NOT VERIFIED | Branch metadata says `protected=false`; rulesets empty; classic endpoint denied |

## Safety

- Keep PR #113 and PR #51 OPEN / DRAFT / UNMERGED.
- Do not modify ONE `main`, merge PRs, force-push, delete branches, release, promote the verifier source pin, or change ledger weights/DoD.
- Preserve historical failures and exact-SHA evidence.
- Do not rerun completed workflows for the same source and verifier inputs without a specific technical reason.
- Any documentation-only commit creates a new PR HEAD; read its own CircleCI status before making a current-head claim.

## Live exact-SHA status refresh — 2026-11-20

This is a later GitHub status readback than earlier dated entries in this document. Historical checkpoints remain unchanged.

- ONE PR #113 is OPEN / DRAFT / UNMERGED. Exact HEAD: `5daf3337b4bcc93fa3ee6072f9633915d5fea85b`; base: `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Code-bearing source: `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. The 16 commits from this source to current PR HEAD are documentation-only; no runtime implementation or test paths changed in that range.
- CircleCI `ci/circleci: one-linux` on exact code source `fb5c8bc...`: SUCCESS (workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`). This is GitHub status-context evidence; provider step logs were not inspected.
- CircleCI `ci/circleci: one-linux` on exact PR HEAD `5daf333...`: FAILURE (workflow `5ed6d9e7-976e-4322-b7ba-6deea915e1a9`, job `8d2d756e-9297-4a00-9a7d-a0c9bd734cba`, job #833). Available diagnostic: step #101 CANCELED, `Task information unavailable`, stdout/stderr empty. Root cause UNKNOWN; do not infer a timeout, infrastructure fault, or source/config regression without provider logs.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`. Its separate `CircleCI Pipeline` status is ERROR at pipeline #399 (`https://app.circleci.com/pipelines/gh/panglimanetizen01/one/399`). Provider logs and cause are UNKNOWN; this status must not be conflated with PR #113's `ci/circleci: one-linux` context.
- Verifier PR #51 remains OPEN / DRAFT / UNMERGED at `6c0979e62eb01e4b929c48b4224a3d4f742bc8af`. Its branch-local source pin is `fb5c8bc...`; 12 associated PR-triggered workflows are terminal SUCCESS. Protected `verification` pin remains `722d662e7a81711ba3607d26571f4048a292b3ed` and is unchanged.
- Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes and unavailable Android/macOS filesystem adapters remain fail-closed. Verifier successes do not constitute Issue #104 acceptance.
- Provider-side CircleCI logs are not accessible through the connected session. Collect job #833 logs by direct job UUID and pipeline #399 logs using the local diagnostic script; verify each run's source revision before attribution. Never expose `CIRCLECI_TOKEN`.
- No merge, force-push, `main` update, protected pin promotion, ledger-weight/DoD change, or blind retry is authorized by this status refresh.

## Release-sprint live status — 2026-11-20

The single prioritized work queue is [ONE_RELEASE_BLOCKER_REGISTER.md](ONE_RELEASE_BLOCKER_REGISTER.md); new sessions start from [MASTER_CONTINUATION_PROMPT.md](MASTER_CONTINUATION_PROMPT.md).

- PR #113 docs HEAD `a0d94bc9a26cf9effd4b202f29af50f81c8e4ee5`: pipeline #1199 / workflow `916983e9-dbde-4db2-a967-438a666ebb19` / job #834. Job `started_at=null`; step #101 CANCELED / `Task information unavailable`, no stdout/stderr. Root cause UNKNOWN.
- Prior docs HEAD `5daf333...`: pipeline #1198 / job #833 has the same pre-execution/no-output signature.
- Code-bearing source `fb5c8bc...` has separate CircleCI SUCCESS; it does not verify the documentation HEAD.
- Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable.
- No acceptance ledger, DoD, protected pin, merge, main, or release state changed.


## Latest execution checkpoint — 2026-11-20

- Exact PR #113 identity remains OPEN / DRAFT / UNMERGED: HEAD `509b7b197115adb1ba80d730342e9f6fbf981645`; base `fix/windows-process-verifier-timeouts-20261118` at `181ad350d8f0db53cda654c4d378425fbdd6ab24`. ONE `main` remains unchanged at `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI pipeline #1201 is now mapped exactly to workflow `9a87f37e-ebc9-4574-a966-14dc1300e7db`, job #836 `9dbc05b4-a601-4fbc-b6a4-61f64ecc8e9a`, revision `509b7b197115adb1ba80d730342e9f6fbf981645`. Workflow/job stop within about 0.5s; job `started_at=null`; API v3 step #101 is CANCELED / `Task information unavailable`, with empty stdout/stderr. Root cause remains UNKNOWN.
- User-provided CircleCI screenshots confirm Free and Open Source ON and Block all new work OFF. Plan usage is 110,089 / 30,000 regular credits, but the page explicitly exempts OS project contributions from the 30,000 limit and advertises 400,000 monthly OSS credits. The screenshots do not expose the remaining OSS balance. An earlier `free-plan-no-credits-available` provider error remains unresolved; do not buy/upgrade/toggle or retry blindly.
- CircleCI settings API returns 403 while account `/api/v2/me` and project metadata endpoints return 200. This is an endpoint permission gap, not evidence OSS is disabled.
- PR #115 bootstrap workflow initial run #2243 failed both jobs at Contract artifacts because the candidate's CircleCI-only contract rejected the workflow file. Corrective commit `3b684acd1f3e68f1b40dd6d600fad0e013044495` narrows checkout to exact PR head only for the stacked base branch and skips only the contract step on the bootstrap PR itself. Full contract checking remains enabled on PR #113.
- GitHub Actions run #2244 (`44748865610`) on the corrective PR #115 commit is IN_PROGRESS; Ubuntu and Windows were installing cross-toolchains at latest readback. No PASS is claimed.
- No merge, main update, verifier pin change, fixed ledger/DoD change, or Issue #104 acceptance change was made.


## Terminal bootstrap workflow result — 2026-11-20

- Corrective PR #115 HEAD `3b684acd1f3e68f1b40dd6d600fad0e013044495` completed GitHub Actions run #2244 / `44748865610` with terminal SUCCESS. Both Ubuntu and Windows jobs passed Build, Clippy, workspace tests, and all focused tests. The bootstrap-only Contract artifacts step was intentionally skipped; this result does not claim that the contract checker passed on the bootstrap branch.
- PR #115 remains OPEN / UNMERGED. CircleCI status on the same SHA is FAILURE (workflow `e6e6a97c-2cda-4a7a-a5ff-3b0c9129d9ec`, job `667d82b1-c817-41f3-91f2-5bd09b55e879`); provider logs are inaccessible, so cause is UNKNOWN.
- PR #113 remains OPEN / DRAFT / UNMERGED at latest docs HEAD `2cbf87ed05277fc6d66088a645f2aea8d0d8ce16`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. Its CircleCI context was PENDING at last readback (workflow `d924efb4-6927-43e7-85c7-ad38cab7f425`, job `192e4737-7e01-426d-b2f4-80ee80fa48c3`). No GitHub Actions run exists for PR #113 yet because PR #115 is not merged.
- No merge, main change, verifier pin change, acceptance-ledger/DoD change, or Issue #104 acceptance occurred.


## Authorized bootstrap merge — 2026-11-20

- PR #115 was explicitly authorized by the user and merged into `fix/windows-process-verifier-timeouts-20261118` using expected head SHA `3b684acd1f3e68f1b40dd6d600fad0e013044495`.
- Merge commit: `3adace165890bb69de346880e3c612c5cd6c2ff8`. Diff from the prior base SHA contains only `.github/workflows/ci.yml` (87 additions); exactly three commits ahead including the merge.
- PR #115 is now merged. PR #113 remains OPEN / DRAFT / UNMERGED. Its head before this follow-up docs update was `673c057421ad3a3293dab990563e7f83cd7f3681`.
- GitHub Actions run for PR #113 was not yet returned by the immediate post-merge lookup. Updating existing docs on the PR #113 head is the next low-risk trigger for its newly installed `pull_request` workflow. The full contract check must run there (unlike bootstrap PR #115).
- CircleCI on merge commit is PENDING: workflow `509828d0-3d7d-434a-8f00-a58645eabe9e`, job `83e5ce02-f0ee-4d30-a0df-5dbfe5c77fc6`. PR #113 CircleCI context also PENDING. B-001 remains OPEN / ROOT CAUSE UNKNOWN.
- No `main` change, PR #113 merge, protected verifier pin update, acceptance criteria or fixed ledger/DoD modification.


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


## B-001 operational resolution — 2026-11-20

- Historical CircleCI pipeline #1201 on SHA `509b7b197115adb1ba80d730342e9f6fbf981645` remains a confirmed pre-execution termination; provider root cause remains unknown.
- Exact SHA `6ef875291b599ef144dba5333e9a40c455f3e013` subsequently received `ci/circleci: one-linux = success`, workflow `c8cddd6a-caa3-49df-81c5-c5c52cc9d578`, job `7f12cc56-4a76-4255-8992-cd7ed984cf9a`. Compare from original failed SHA to tested SHA shows only the three existing verification docs changed; runtime source and CI workflow are unchanged.
- B-001 is RESOLVED OPERATIONALLY for this candidate code/workflow state; historical provider root cause UNKNOWN. Latest PR #113 HEAD `7a9ac7ae720eba3b8f64c1bfa46feda3927dd410` adds docs-only commits beyond tested SHA and has no CircleCI status entry.
- GitHub Actions run #2247: Windows SUCCESS; Ubuntu IN_PROGRESS at Ubuntu cross-compiler installation, later steps pending. Overall matrix is not yet PASS.
- Free and Open Source is ON per user-confirmed CircleCI UI. Settings API 403 is only a permission limitation.


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

## Android filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `7dd7b0049a7f5d71ac8a3d59570c2975fed835ec` (NOT BUILT OR TESTED).
- Android now has a native `FilesystemAuthority` root open and a handle-relative direct-child read/atomic-replace backend. Root and leaf are opened without following final symlinks; `statx(STATX_MNT_ID)` is required to reject mount crossings; reads reject non-regular and hard-linked files; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The backend deliberately avoids `openat2` because Android seccomp may terminate the process with `SIGSYS` for that syscall. It requires kernel `statx` mount-ID support; if unavailable it fails closed.
- Scope is intentionally limited to direct children of the trusted root. Nested Android paths return `UnsupportedPlatform`; no claim of full Android filesystem support or Issue #104 acceptance is made.
- Three Android-targeted regression tests cover direct-child read/write, leaf-symlink and nested-path denial, and final-component root-symlink rejection. They have NOT been run. Actual Termux/device execution is still required.
- Linux nested writes remain fail-closed; macOS filesystem adapter remains unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

## macOS filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `4be1b8fd579092d8195c3fb775b7c765f6221814` (NOT BUILT OR TESTED).
- macOS now has a native retained-root open and direct-child read/atomic-replace backend. Root and leaf operations use directory/file descriptors with `O_NOFOLLOW`; file and root device identity are compared; hard-linked and non-regular targets are rejected; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The macOS adapter is deliberately limited to direct children of the trusted root. Nested paths return `UnsupportedPlatform`. This implementation has no target-host execution evidence yet; its mount/device and race behavior still require platform-specific review.
- Three macOS-only regression tests cover direct-child read/write, symlink/nested-path denial, and final-component root-symlink rejection. They have NOT been run.
- Android direct-child backend remains statx-mount-ID dependent; Linux nested writes remain fail-closed. Issue #104 remains OPEN / NOT ACCEPTED.

## Issue #105 — Linux process-group containment implementation candidate — 2026-11-20

- Latest code-bearing candidate: `0a9685bded788ae46596227bd61457d10db1179c` (NOT BUILT OR TESTED).
- Linux subprocess startup now installs an inherited seccomp filter in the child before `exec`. It denies `setsid`, `setpgid`, `unshare`, `setns`, clone namespace flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` so compatible runtimes can fall back to `clone`. The existing dedicated process group is therefore intended to remain non-escapable for descendants; group cleanup still runs on normal return, timeout, and output overflow.
- If the kernel refuses `PR_SET_NO_NEW_PRIVS` or filter installation, process spawn fails closed. Windows Job Object containment remains unchanged.
- Process execution is now explicitly unavailable on Android and macOS until equivalent non-escapable containment exists; `ProcessAdapter` advertises only Linux and Windows.
- Replaced the old escaped-session reader-only probes with three Linux adversarial regressions for attempted `setsid` on normal completion, timeout, and output overflow. These tests have NOT been run.
- This is an implementation candidate, not Issue #105 acceptance. Exact-SHA build/execution and adversarial evidence are still required. Preserve bounded reader shutdown and all existing process limits.

## Android process containment extension — 2026-11-20

- Code candidate `58ae7831c7d17e60e53fa6a933dc2f170196c585` enables the same inherited Linux seccomp process-group filter on Android as on Linux and advertises `ProcessAdapter` on Android. It denies session/group changes and namespace escapes; filter setup failure is fail-closed.
- macOS process execution remains unavailable until equivalent containment exists. Windows Job Object behavior is unchanged.
- This Android extension is NOT BUILT OR TESTED. Android seccomp policy compatibility and the three adversarial process tests still require execution on an actual Android/Termux target. Issue #105 remains OPEN / NOT ACCEPTED.

## Android process containment regression coverage — 2026-11-20

- Latest code-bearing candidate: `418d9642778d975440b81df2bdb5e1bf9aa1d8a5` (NOT BUILT/TESTED).
- The inherited seccomp pre-exec filter is now installed on both Linux and Android. ProcessAdapter advertises Linux, Android, and Windows; macOS remains fail-closed. Android uses the same `setsid`/`setpgid`/namespace restrictions and fails closed if the child cannot install the filter.
- The three adversarial `setsid` regression tests and bounded-reader test are now target-gated for Linux and Android, so they can run in Termux as well as Linux CI. No Android/Termux execution evidence is claimed.
- Issue #105 remains OPEN / NOT ACCEPTED pending exact-SHA build and adversarial execution on Linux and a real Android/Termux target, plus Windows Job Object evidence reconciliation.

## IMPLEMENTATION CHECKPOINT — 2026-11-21 (supersedes older platform-status bullets)

Code-bearing source/test candidate: `4c46a41fe45baa7627557a2a25b9c71812840479`. PR #113 was OPEN / DRAFT / UNMERGED at this exact SHA before documentation synchronization. This candidate has NOT been built or tested; no previous exact-SHA result transfers.

### Issue #104 — filesystem implementation
- Linux: retained root handle, `openat2` no-follow/beneath/no-cross-device reads; atomic direct-child replacement; nested writes use the detached-mount + `mount_setattr(MOUNT_ATTR_NOSYMFOLLOW)` + `openat2` path and fail closed when the kernel/security profile cannot provide those primitives.
- Windows: existing native handle-relative/reparse-point backend and retained-parent rename protection remain unchanged.
- Android: direct-child reads and atomic replacement use retained root `dirfd`, `O_NOFOLLOW`, mandatory `statx(STATX_MNT_ID)` equality, regular-file/single-link checks, exclusive temporary sibling, file sync, root-directory sync, and `renameat`. Nested paths remain fail-closed.
- macOS: direct-child reads and atomic replacement use retained root `dirfd`, `O_NOFOLLOW`, regular-file/single-link checks, `fstatfs` filesystem-ID plus mount-point comparison, exclusive temporary sibling, file sync, root-directory sync, and `renameat`. Nested paths remain fail-closed.
- Android/macOS hard-link side-effect regressions were added. Existing Android/macOS direct-child, symlink, and root-symlink regressions remain in source. These tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED until the fixed acceptance criteria have exact-SHA Unix and Windows execution evidence, including race/side-effect proof.

### Issue #105 — process containment implementation
- Linux and Android subprocess startup install an inherited seccomp filter before `exec`; it denies `setsid`, `setpgid`, `unshare`, `setns`, namespace-bearing `clone` flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` for compatible fallback. Filter setup failure is fail-closed.
- Windows Job Object containment is unchanged. macOS process execution remains fail-closed until equivalent non-escapable containment exists.
- Three adversarial `setsid` regressions cover normal completion, timeout, and output overflow on Linux/Android; they are NOT RUN. Issue #105 remains OPEN / NOT ACCEPTED pending Linux + real Android/Termux execution and reconciliation of Windows Job Object evidence.

### Execution priority
- Continue production implementation against the existing v0.1 contract and release blockers. Add regression coverage with each code change, but do not launch broad matrix/audit cycles until the implementation scope is complete.
- Preserve B-001's prior exact-SHA success; do not repeat its investigation without a new relevant provider failure.
- Preserve `main`, protected verifier pins, fixed ledger weights, DoD, acceptance criteria, and release authorization boundaries.


## Implementation continuation refresh — 2026-11-21

Latest code-bearing candidate and PR #113 HEAD at this checkpoint: `fd1ccc5ca56f48a306ac8f8c551189f46b8fe44c` (OPEN / DRAFT / UNMERGED; NOT BUILT/TESTED). This supersedes the previous 2026-11-21 candidate pointer.

Issue #105 regression coverage now uses ignored helper tests in the same Rust test binary. The helper forks a descendant and calls `setsid(2)` directly, avoiding dependence on an external `setsid` utility on Android/Termux. The parent tests cover normal completion, timeout, and output overflow, record any escaped PID, and clean up the escaped group before reporting failure. The tests are NOT RUN.

Issue #104 implementation changes from the previous checkpoint remain: Linux guarded detached-mount nested writer; Android direct-child `statx(STATX_MNT_ID)` backend; macOS direct-child `fstatfs` filesystem-ID/mount-point backend; exclusive temp replacement with file and directory sync; hard-link regressions. Windows native backend remains unchanged. Nested Android/macOS operations fail closed.

Issues #104 and #105 remain OPEN / NOT ACCEPTED. No exact-SHA build/test or broad verification is claimed. Continue implementation-first and preserve all release/security gates.

## Process-group cleanup hardening — 2026-11-21

Latest code-bearing source/test candidate: `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` (NOT BUILT/TESTED). Linux/Android process supervision now observes direct-child exit using `waitid(P_PID, WEXITED | WNOHANG | WNOWAIT)` rather than reaping via `try_wait()` before process-group cleanup. The child remains a zombie (its PID/PGID cannot be recycled) while the wrapper sends SIGKILL to the invocation process group; only then does `Child::wait()` reap it. The redundant post-reap group kill was removed. All timeout, output-overflow, wait-error and normal-completion paths terminate the group before reaping.

Process escape regressions use in-binary ignored helpers with direct `fork` + `setsid(2)` syscalls, record any escaped PID and clean up before failing. Three paths: normal completion, timeout, output overflow. NOT RUN. Issue #105 remains OPEN / NOT ACCEPTED.

## IMPLEMENTATION-FIRST CHECKPOINT — 2026-11-21

PR #113 remains OPEN / DRAFT / UNMERGED on `security/issue-104-root-confinement-20261118`, base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`. Latest code-bearing source is `72eae103bc3304b0961c66a0cf61f716ddfa9ac4`; current PR HEAD after test-fixture corrections is `2600ec18d4e3fb05558ca50d7cf466638e1ab015`. The two new commits change only `crates/one-cli/tests/filesystem_write.rs` and intentionally use `[skip ci]`. They align CLI fixture working directories with the host-owned root contract, allow Linux nested writes only when the detached-mount backend works (otherwise fail closed with no side effects), and stop treating Android/macOS as unsupported native backends. No tests or workflows were run. See the single work queue `ONE_RELEASE_BLOCKER_REGISTER.md` for the full source inventory and exact status boundaries. Issue #104/#105 remain OPEN / NOT ACCEPTED; no release/ledger/protected-pin state changed.


## Subprocess resource-bound implementation refresh — 2026-11-21

The latest code-bearing source for Issue #81/#105 is `cd95e5ec2234a2909ef79bf8b25fe65296fab5ed`. It explicitly enforces a 10-second deadline, 1 MiB stdout cap, 1 MiB stderr cap, and 2 MiB aggregate output ceiling before returning a process result. Source tests cover stdout overflow, stderr overflow, timeout, normal exit/status/stream preservation, and Linux/Android direct-fork + `setsid` escape attempts for normal completion, timeout, and output overflow. No build or test was run on this source. Issues #81 and #105 remain OPEN; no security or release acceptance is claimed.
