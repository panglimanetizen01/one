# One Development Protocol

## 1. Working cycle

```text
DISCOVER
  -> EXECUTE
  -> VERIFY
  -> FIX
  -> REVERIFY
  -> DOCUMENT
  -> CONTINUE
```

Do not stop merely because one tool path is unavailable. Use another valid path when possible and record environmental limitations precisely.

## 2. Evidence law

```text
NO EVIDENCE -> NO CLAIM -> NO COMMAND -> NO PATCH
```

Evidence must identify the source, revision, command/test, environment, and result when those dimensions matter.

## 3. Change discipline

Prefer one logical change with one proof. Avoid unrelated refactors. Preserve user changes. Do not use destructive Git operations without explicit need and evidence.

## 4. Capability workflow

For a new runtime capability:

```text
SPECIFY
 -> RED TEST
 -> IMPLEMENT
 -> GREEN
 -> CLIPPY/LINT
 -> VERIFY
 -> DOCUMENT
 -> CONTINUE
```

## 5. Bug workflow

```text
ROOT CAUSE
 -> FAILING TEST
 -> FIX
 -> VERIFY
 -> DOCUMENT
 -> CONTINUE
```

Do not patch symptoms while the root cause remains unknown.

## 6. Status vocabulary

Use:

```text
Specified
Designed
Implemented
Tested
Verified
Supported
```

A later state requires evidence for the earlier states. `Implemented` is not `Verified`. `Verified` is not necessarily `Supported` if the support boundary has additional requirements.

## 7. Evidence labels

- `[P0 — VERIFIED]` fresh direct evidence.
- `[P1 — STRONG]` strong inference.
- `[P2 — HISTORICAL]` previous evidence, not current-head proof.
- `[P3 — HYPOTHESIS]` unverified.
- `[P4 — UNKNOWN]` insufficient evidence.
- `[P5 — INVALIDATED]` disproved.

## 8. CI interpretation

A workflow failure with no executable job steps is infrastructure/verification insufficiency evidence, not proof of a source-code failure. Inspect actual jobs and steps before classifying a CI failure.

## 9. Exact revision verification

When the project provides an exact-SHA verifier, use the exact commit under test. If documentation is changed after a verification run, the documentation commit itself must be considered a new revision for any final-head claim.

## 10. Documentation drift

If source, tests, contracts, status documents, or evidence matrices disagree:

```text
FIND ROOT CAUSE
 -> FIX SOURCE/TEST/CONTRACT AS APPROPRIATE
 -> VERIFY
 -> UPDATE DOCUMENTATION
 -> REVERIFY FINAL REVISION
```

## 11. Platform evidence

Do not generalize a Termux/Android result to Ubuntu/Windows, or a Windows result to Linux, without evidence. Record the environment boundary explicitly.

## 12. Completion

Do not report a milestone as complete until all required artifacts exist, references are consistent, repository state is verified, and the final change has a concrete Git revision.


## 13. Cross-session anti-duplication protocol

1. Load enabled Notion skills at session start and read the active ONE skill/checkpoint before doing work.
2. Resolve live repository, branch, HEAD, PR, and CI state. Treat old checkpoint identifiers as historical until compared with live state.
3. Build a task ledger with explicit statuses: NOT STARTED, IN PROGRESS, PASS, FAIL, BLOCKED, UNKNOWN. A PASS must include exact SHA, command/job, environment, and evidence location.
4. Resume the earliest unfinished dependency. Never repeat a PASS scope on the same SHA without a documented invalidating change. Never repeat a known deterministic failure without a changed input or corrective patch.
5. Inspect the first failed CI step and its complete log before editing. Preserve later skipped steps as NOT RUN, not failed. Fix the root cause, not the test or gate.
6. CircleCI is the primary CI for the active verification remediation. On Android/Termux, if the CircleCI CLI is incompatible, use the CircleCI API through a shell-local authenticated helper; do not expose or persist tokens in source files.
7. Update the active checkpoint, relevant verification docs, and Notion skill after each material milestone. Record exact commit and run IDs; distinguish implemented config from executed/verified behavior.
8. A new source or documentation commit creates a new exact-SHA verification target. Earlier results remain historical and are not silently promoted to the new HEAD.

## 14. Current CI remediation pointer

See [the CircleCI native-toolchain remediation checkpoint](verification/circleci-native-toolchain-remediation.md) for the active branch, exact commits, pipeline history, first failure, next action, and no-repeat rules. This pointer is operational context, not evidence of a passing build.


## 15. GitHub source authority, CircleCI canonical CI, and Termux boundary

- All source, test, workflow, build, and documentation changes are performed on GitHub branches through GitHub-native operations. GitHub is the single source of truth; the live full SHA is authoritative. Never use a local checkout as the content source for repository writes.
- CircleCI is the canonical CI provider for the One source repository, replacing GitHub Actions as the normal source CI path. Validate each material commit on its exact SHA and inspect pipeline, workflow, job, step, logs, checkout proof, and artifacts. Preserve old Actions workflows until their coverage is mapped and migration/removal is explicitly authorized.
- Termux is read-only and verification-only after the corresponding GitHub commit and CircleCI checks have succeeded. Allowed: inspect exact SHA, retrieve logs/artifacts, verify manifests/hashes, and run isolated independent validation. Forbidden: edit/patch source or docs, fix bugs, format, stage, commit, branch, rebase, reset, stash, push, or change CI/repository settings.
- A local discrepancy is a report to reproduce and fix on GitHub, not permission to patch locally. Local results are independent validation and never CircleCI evidence.
- At session start, load enabled Notion skills, read live `AGENTS.md`, this policy, and the active checkpoint; reconcile live GitHub refs/PR SHAs and CircleCI before resuming the earliest unfinished task.
- Every material change requires updates to all affected repository documentation and the existing Notion skill/checkpoint. Record exact SHA, CI evidence, hashes, unresolved gaps, and no-repeat scope. Do not merge or release without explicit authorization.


## 16. Forensic audit, repair, and documentation reconciliation

A forensic audit must establish a complete in-scope path inventory, then inspect execution routes and cross-document contracts rather than sampling only the latest failure. Include code, tests, scripts, dependency/toolchain setup, workflow triggers/path filters, OS/runtime variants, fallback routes, permissions/secrets references, artifact production/consumption, release/provenance paths, Markdown derivatives, specifications, plans, requirements, decisions, user guides, verification indexes, ledgers, and Notion continuity.

For each finding, record exact SHA/path/line or workflow/job/step, classification (BUG, SYNTAX, CONTRACT DRIFT, DOCUMENTATION DRIFT, WORKFLOW GAP, INFRASTRUCTURE, SECURITY REVIEW, UNKNOWN), evidence, severity, root cause, and regression proof. If actionable and authorized, fix it immediately on the GitHub branch, add/update regression coverage, run relevant CI, inspect logs/artifacts, read back the change, and update every affected document plus the Notion skill. Do not merely report fixable findings or stop after the first failure.

Continue independent work when one route is blocked. Do not misclassify pre-run failures as source failures. Do not weaken tests, security, or gates. Preserve historical evidence and mark it HISTORICAL; never transfer PASS across SHAs. A new source or documentation commit is a new exact-SHA candidate. No full completion claim while any required item is BLOCKED, UNKNOWN, NOT VERIFIED, or NOT RUN.
