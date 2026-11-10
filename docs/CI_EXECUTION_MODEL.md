## Active forensic continuation — 2026-11-11 — documentation synchronization candidate

- PR #94 remains OPEN / DRAFT / UNMERGED on `fix/windows-temp-path-canonicalization`; current source HEAD for this documentation update is `8c43a1533ee1e8f53d68984092873b9388442c98`; base `d75b2846465ba53281198693750f2e6ba39e2e33`.
- This commit is documentation/operating-contract synchronization after source SHA `6b5070e7c01aa8cf2885653fa1d5728ad8fcdfbd`. The CircleCI status and independent verifier matrix recorded for 6b are historical for this new HEAD; do not claim they verify `8c43a1533ee1e8f53d68984092873b9388442c98`.
- At last poll on source SHA 6b, CircleCI `ci/circleci: linux-verify` was SUCCESS at commit-status level only (workflow `d2f2171d-a60e-42df-8c72-d63d9f70a542`, job `61480bb4-6a12-4087-8566-3bf4da5b906c`); job detail, checkout, step logs/exit codes, and artifact hashes were not independently inspected in this session.
- The 6b verifier matrix reached 14 SUCCESS, 2 IN PROGRESS, 1 early ledger-gate FAILURE. Remaining 6b runs: Android API 35 native ARM64 runtime #42881168518 and candidate artifact integrity #42881168487. The gate ran before matrix completion and remains a stale NOT PASSED snapshot; 10 DoD unchecked, E1–E6 each 0 verified, and protected E2 production provenance absent. Do not transfer any 6b result to this new SHA.
- Verifier pin remains at 6b until CircleCI proves the new source SHA and the branch is stable. Keep `LOCAL_CI_CANONICAL_COMMIT.txt` unchanged for PR #92.
- Legacy Actions on 6b failed pre-run (runner_id 0, empty runner name, no steps); root cause UNKNOWN, not a source-test failure. CircleCI remains canonical; preserve legacy workflow files pending coverage-parity review.
- Acceptance remains NOT PASSED. No merge/release, protected E2 dispatch, secrets/trust-store mutation, branch-protection changes, or source Actions removal.
- Next: inspect CircleCI status for `8c43a1533ee1e8f53d68984092873b9388442c98`, retrieve authenticated job detail/logs/artifacts via read-only Termux helper, then synchronize the verifier pin and run only the exact-SHA scopes required for this new candidate. Continue full documentation/contract audit and preserve no-repeat evidence.

---

# ONE CI Execution Model and Repository Authority

**Status:** Accepted operational contract, effective 2026-11-11.  
**Applies to:** All technical work in `panglimanetizen01/one`, every contributor/agent session, and all repository documentation and CI changes.

## 1. Authority and source of truth

The GitHub repository is the single source of truth for One's source code, tests, documentation, workflow configuration, branches, commits, pull requests, and accepted project state.

- The live GitHub commit SHA is authoritative for a candidate change. An open PR branch is a candidate; merged `main` is the accepted baseline.
- GitHub files and history outrank local checkouts, copied files, shell history, cached reports, and prior chat summaries.
- CircleCI records whether a specific GitHub SHA passed configured execution. It does not change source authority or automatically satisfy every milestone/release gate.
- Notion stores reusable operating instructions and continuity checkpoints. It is not a substitute for GitHub source files or exact-SHA evidence.
- If local content differs from GitHub, local content is stale. Re-fetch the exact GitHub SHA; never push local divergence to make GitHub match Termux.

## 2. Canonical CI provider: CircleCI

CircleCI is the designated canonical and primary CI provider for the One source repository, replacing GitHub Actions as the normal source-repository CI path.

Normal workflow:
1. Make technical changes directly against the live GitHub branch/PR through GitHub-native repository operations.
2. Commit the change to GitHub and capture the resulting full SHA.
3. Validate that exact SHA through the configured CircleCI pipeline.
4. Inspect pipeline, workflow, job, every relevant step, logs, artifacts, and checkout revision.
5. Repair the earliest evidenced root cause on GitHub, then validate the new SHA in CircleCI.
6. Preserve evidence and update all affected documentation and Notion checkpoints.

GitHub Actions workflow files already in the repository are legacy/transition assets until their coverage is mapped and migrated. Do not use source-repository GitHub Actions as the default CI route. Do not delete, disable, or rewrite existing workflows merely to make the migration look complete. Removal requires a coverage-parity review and explicit authorization. Keep the independent verifier repository and its workflows separate from source-repository CircleCI.

### Coverage parity is not assumed

At the 2026-11-11 checkpoint, `.circleci/config.yml` defines one `linux-verify` job. The latest inspected exact-SHA run is workflow `f392e32f-564a-4756-a97c-87db69376cd0`, job #20 (`a28b1024-b9f1-424c-b92f-05f275366f98`), SUCCESS on source SHA `912c10fe86cee10294a5e09b96a99f00477d788a`. Contract/user-doc checks, health scanner regression tests, locked workspace build, health scan, Clippy, full workspace tests, deterministic package test, and database/HTTP vertical slices all reported success. Artifact metadata listed JSON and Markdown health reports, but both download URLs returned HTTP 404; payload contents and checksums are NOT VERIFIED. This run is historical for any subsequent source commit. It ran on Linux and performed Windows PE32+ and Android AArch64 compiler probes only; those probes are not native Windows or Android runtime tests.

Therefore:
- CircleCI is the canonical CI path by policy.
- Full parity with all former source-repository Actions coverage is still **NOT VERIFIED**.
- Do not claim Windows Server 2025 native execution, Android API 35 native runtime, macOS, distro breadth, or release/provenance coverage unless dedicated exact-SHA CircleCI jobs or an explicitly accepted independent verifier prove those scopes.
- Maintain a coverage matrix listing each required OS/architecture/runtime/test/artifact/security/release scope, its CircleCI job, and evidence. Gaps remain BLOCKED or NOT VERIFIED, not silently waived.

## 3. Termux is verification-only

Termux on Android is **not** an implementation, authoring, repair, or CI administration environment for the One repository.

Allowed only after GitHub + CircleCI proof exists for the exact SHA:
- Read-only inspection of GitHub files, commit metadata, PR state, and CircleCI results.
- Authenticated read-only API requests to retrieve CircleCI job logs and artifacts.
- Exact-SHA comparison, artifact inspection, manifest checks, file-format checks, and SHA-256 verification.
- Optional independent validation against a clean, isolated checkout of the already-proven SHA, when it adds evidence. Keep generated output outside tracked repository paths.
- Reporting discrepancies with the exact SHA, command, environment, output, and evidence.

Forbidden in Termux:
- Editing, creating, deleting, patching, formatting, or regenerating source, tests, workflows, build scripts, or documentation.
- Implementing features, fixing bugs, or making any repository change in a local checkout.
- Creating/changing branches, staging, committing, rebasing, cherry-picking, resetting, stashing, or pushing One repository changes.
- Treating local build/test results as CircleCI evidence or as a substitute for hosted CI.
- Using local files as the content source for GitHub writes.
- Mutating GitHub/CircleCI settings, secrets, environments, permissions, branch protection, or release configuration.

If a local check discovers a defect, do not patch locally. Reproduce and repair it on a GitHub branch, then validate the resulting commit in CircleCI. A local PASS is independent validation only and never upgrades a CircleCI or acceptance status.

## 4. Mandatory session-start sequence

Every new session must do these steps before technical work:
1. Load enabled Notion skills with `notion-get-enabled-skills`; load this Project ONE skill and any other relevant enabled skill.
2. Read the live `AGENTS.md` from GitHub.
3. Read this CI execution model, the current Notion checkpoint, and the active repository CI/remediation checkpoint.
4. Resolve live GitHub repository metadata, branch refs, PR state, base/head SHAs, and commit URLs. Treat saved SHAs as historical until compared with GitHub.
5. Inspect CircleCI state for the exact active SHA. Reuse successful exact-SHA evidence; do not rerun it without a changed SHA or invalidating input.
6. Identify the earliest unfinished dependency and continue execution directly. Do not restart completed audits, inventories, diagnoses, or fixes.
7. If a required tool is unavailable, use another supported GitHub/CircleCI route. Do not fall back to local implementation.

## 5. Exact-SHA evidence contract

For each relevant CircleCI run, retain repository, branch, PR, full source SHA and commit URL; pipeline/workflow/job IDs and conclusions; executor/environment; direct checkout proof; each required step's status, exit code and log; test totals where exposed; artifact path, download result, embedded source SHA and SHA-256 where applicable.

Use honest states: `PASS`, `FAIL`, `BLOCKED`, `NOT RUN`, `NOT VERIFIED`, `UNKNOWN`, and `N/A`. A missing log is not proof of source failure. A green workflow does not prove every platform. A compiler probe does not prove native runtime execution. Informational scanner findings are review leads, not confirmed defects.

A PASS belongs to the exact SHA and scope. A later commit is a new candidate. Preserve earlier results as historical and rerun only checks invalidated by the change.

## 6. Documentation and Notion synchronization

Every material change to source, tests, build/tooling, CI, workflow, architecture, security, API contract, release process, or acceptance decision requires a documentation impact review.

Update every affected ONE document, as applicable: `AGENTS.md`, root `README.md`, `docs/README.md`, this CI execution model, `docs/DEVELOPMENT_PROTOCOL.md`, `docs/DECISIONS.md`, affected constitution/architecture/requirements/compatibility/security/specification contracts, Indonesian user guides/examples for user-visible behavior, verification README, CI remediation record, active checkpoint, evidence matrix, completion ledger, Definition of Done, and release/provenance docs.

Do not mechanically rewrite unrelated historical evidence. Preserve dated records, label superseded procedures, and link them to the current canonical policy. Documentation changes are GitHub commits and must pass CircleCI's documentation/contract checks.

After every major change, update this Notion skill and the existing project checkpoint (no duplicates); record exact GitHub SHA, PR, CircleCI IDs/statuses, test counts, logs/artifacts, hashes, unresolved gaps and next action; read back the saved GitHub and Notion changes; and add a no-repeat guard.

## 7. Authorization and safety

No merge, release, tag, protected production dispatch, secret/trust-store mutation, branch-protection change, or repository-setting change without explicit authorization. Do not change acceptance weights or promote ledger points without exact evidence and the documented gate. Never skip tests or weaken a gate to manufacture a green result.

## 8. Current evidence snapshot — 2026-11-11

- PR #94: OPEN / DRAFT / UNMERGED, branch `fix/windows-temp-path-canonicalization`.
- Last live pre-documentation HEAD: `e19d0129c6a3169a36e18a9ce7cd13f4a677d348`; base `milestone/100-ecosystem-20261023` at `d75b2846465ba53281198693750f2e6ba39e2e33`; `main` at last check: `d35db95153f6ddd79885494c72f5aee16c899751`. Resolve refs again next session.
- CircleCI pipeline #34, workflow `one-linux-verification`, job #19 `linux-verify`, job ID `1ef51523-a767-4ccf-bae8-9f806dc13ca7`: SUCCESS on exact SHA `e19d012...`.
- Health report `expected_sha == head_sha == e19d012...`; working tree clean; 27 findings, all informational low-confidence `workflow-overlap`; zero critical/high/medium/low.
- JSON artifact SHA-256 `76b77bcc94e066ad8f2b270318ab5a12db76c73b3ea2106d4cf3bcd1c5664359`; Markdown artifact SHA-256 `a69a2db6806a1bd4347211db5b31097154023c19fb3df6d955e3cea8724e1baf`.
- Authenticated CircleCI API access was required to download artifacts redirected to a private endpoint. Never expose or commit the token.
- This job does not establish native Windows or Android runtime acceptance. Overall project acceptance remains NOT PASSED; independent verifier pin reconciliation and CI coverage parity remain unfinished.
