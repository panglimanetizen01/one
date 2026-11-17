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


## 13. CircleCI and independent Windows exact-SHA discipline

CircleCI is the canonical CI execution provider for Project ONE's Linux product and Linux release checks. Keep the canonical CircleCI configuration Linux-hosted; Linux-host MinGW-w64 cross-compilation may remain as Windows-target backend regression coverage, but it is not Windows-host execution evidence.

Windows Server 2025 product/native verification belongs to the independent `panglimanetizen01/one-windows-verification` repository. Its workflow must check out the private canonical ONE repository at a full explicit source SHA, assert that checkout identity, record the verifier checkout SHA separately, run the required product/native checks, and preserve the manifest and executable artifact. Verify CircleCI Linux and independent Windows separately against the same exact canonical source SHA; neither result substitutes for the other.

Do not reintroduce Windows executor jobs into canonical CircleCI or restore the retired GitHub Actions product workflows in the ONE repository. Record exact source SHA, verifier SHA, pipeline/run/job IDs, host platform, terminal result, logs, and artifact digests. Never report an unstarted pipeline or missing job steps as PASS/FAIL. Keep `main` unchanged until the reviewed migration PR is eligible for an authorized merge. CI/tooling changes must not alter ONE grammar, semantics, compiler, interpreter, runtime, CLI, or user experience.
