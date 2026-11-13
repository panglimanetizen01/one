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


## 13. CircleCI exact-SHA discipline

CircleCI is the canonical CI execution provider. A CI configuration change is not verified by YAML syntax or file presence alone. Record the exact source SHA, pipeline ID, workflow/job names, executor platform, job result, logs, and artifacts. Preserve Linux and Windows coverage, Android NDK/Clang, MinGW-w64, GNU x86_64 linker, deterministic artifact checks, real-network release checks, offline verification, and dedicated Windows native PE validation/execution. Remove legacy GitHub Actions CI workflows from the migration branch; keep `main` unchanged until the reviewed migration PR passes exact-SHA CircleCI parity and is merged. Never report an unstarted pipeline or missing job steps as a source PASS/FAIL.



