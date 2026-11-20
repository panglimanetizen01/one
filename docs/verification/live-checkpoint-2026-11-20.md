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

