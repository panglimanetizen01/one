# ONE v0.1 — CI and Verification Evidence Contract

**Status:** Normative development/tooling contract  
**Applies to:** CI configuration, runner provisioning, verification orchestration, evidence records, and documentation only.

## 1. Product boundary

CI and verification infrastructure are development-side tooling. They MUST NOT change ONE grammar, syntax, semantics, AST/IR meaning, compiler/runtime behavior, CLI contract, or beginner-facing user experience solely to satisfy a runner, package manager, hosted plan, CI provider, or external toolchain.

## 2. Outcome taxonomy

Every CI result MUST be classified using observed job-level evidence:

- **EXECUTED / PASS:** executable steps ran and the relevant checks passed for the recorded exact SHA.
- **EXECUTED / SOURCE-FAIL:** executable source or test steps ran and failed; include the failing command and diagnostic.
- **TOOLCHAIN-BOOTSTRAP-FAIL:** the runner started, but environment/toolchain provisioning failed before the relevant ONE checks.
- **RUNNER-NOT-STARTED / BLOCKED:** no executable steps, zero elapsed time, missing executor/resource assignment, or equivalent direct evidence. Root cause remains unknown unless provider evidence identifies it.
- **PLAN-CREDIT-BLOCKED:** explicit provider job/account evidence identifies unavailable credits.
- **INCONCLUSIVE:** evidence is incomplete or conflicting.

A generic failed status MUST NOT be relabeled as SOURCE-FAIL without executable source/test evidence.

## 3. Exact-SHA evidence

Each positive or negative verification claim MUST record:
- repository and branch;
- exact source commit SHA;
- verifier repository and verifier SHA, if applicable;
- provider/run/pipeline/job identifiers;
- OS, architecture, executor/resource class;
- commands and steps actually executed;
- outcome and artifact digests where applicable.

Evidence for one SHA MUST NOT be transferred to another SHA. Documentation-only commits may be described as not changing code only when the changed-file/commit evidence supports that statement.

## 4. Infrastructure incidents

Runner provisioning, hosted plan credits, package-manager keyrings, external toolchain installers, and transient provider failures are infrastructure/tooling findings. Fix the infrastructure only when it remains part of the current supported workflow. Historical or superseded workflows are recorded, not reintroduced solely to make old runs green.

Do not disable package signature verification or bypass trust checks to force toolchain installation.

## 5. Independent verification

When canonical hosted CI is unavailable, the independent verification path may provide evidence only for the exact source SHA it checks out and the platform/commands it actually executes. The result must be linked to the canonical source revision and must not be generalized beyond the recorded matrix.

## 6. Credits and account changes

A job-level no-credit message is evidence for that job only. Usage aggregates do not establish the current available balance. Plan usage is checked in the provider's authoritative account interface. No paid upgrade, purchase, secret change, or account/billing modification is authorized by this contract.

## 7. Documentation synchronization

An infrastructure finding that affects user-facing status or release interpretation MUST be reflected in:
- the user documentation's status/expectations page;
- the implementation status and verification evidence record;
- the evidence matrix and release contract where relevant;
- the documentation index and cross-session Notion Skill.

The user guide must explain only the practical impact: CI infrastructure may be blocked without indicating a defect in ONE programs. Internal runner details remain in the verification incident record.

## 8. Current recorded incident

See [CircleCI Infrastructure Incident Record — 2026-11-16](../verification/circleci-infrastructure-incident-2026-11-16.md). The historical Windows job #254 is diagnosed as an MSYS2 first-run/keyring initialization failure on a superseded workflow. Jobs #407 and #426 have explicit `free-plan-no-credits-available` messages and are PLAN-CREDIT-BLOCKED; #408–#409 remain runner-not-started with cause unknown; #424 is independently credit-blocked. Independent Windows run #21 is successful only for its pinned source SHA.

## 9. Diagnostic evidence retrieval

For CircleCI Cloud, prefer the official CLI failure report for completed failed runs: `circleci run get <run-uuid> --failure-report`. Inspect a specific job with `circleci job get <job-uuid> --json` and `circleci job output list <job-uuid> --json`. The CLI accepts the `CIRCLE_TOKEN` environment variable. A failure report may be empty when no failed executable steps are recorded or the run is not an eligible completed failure; inspect status and job metadata before drawing conclusions. CircleCI defines Failed as a job that failed or terminated for unknown reasons, so a generic failed status alone MUST NOT be treated as a source failure. See [the official failure-report guide](https://circleci.com/docs/guides/toolkit/diagnose-failed-runs-with-failure-report/) and [CLI reference](https://cli.circleci.com/reference/).


## 10. Correction — explicit CircleCI credit blocks (2026-11-16)

New user-supplied CircleCI v1.1 job responses establish `free-plan-no-credits-available` for jobs #407 (UUID `dde778de-e8d9-48ff-b2ac-18b9aa2aa04f`, pipeline #277) and #426 (UUID `16aedf05-149f-4c3e-9d25-888d1af8fe10`, pipeline #290). Both MUST be classified PLAN-CREDIT-BLOCKED, not root-cause-unknown runner failures and not source/test failures. Job #424 has its own explicit no-credit message. Jobs #408–#409 remain runner-not-started with cause unknown until their own job-level evidence is available. The #426 configuration snapshot is Linux-only (`one-linux`, plus parameter-gated `release-linux`); it has no Windows/MSYS2 job. No source-code change or CI rerun is justified by these blocked results.
