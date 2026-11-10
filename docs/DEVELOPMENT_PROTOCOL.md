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
