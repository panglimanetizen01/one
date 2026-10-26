# ONE v0.1 — Definition of Done and Release Gate

**Status:** In progress — not a release authorization  
**Candidate branch:** `milestone/100-ecosystem-20261023`  
**Candidate source SHA:** see the synchronized source pin and evidence index; this reusable checklist intentionally does not embed a self-referential commit SHA.  
**Fixed-weight ledger:** [one-completion-ledger.md](one-completion-ledger.md)  
**Independent evidence index:** [CANDIDATE_VERIFICATION.md](https://github.com/panglimanetizen01/one-windows-verification/blob/verification/CANDIDATE_VERIFICATION.md)  
**PR:** [#80](https://github.com/panglimanetizen01/one/pull/80)

This checklist is an acceptance control, not a claim that the release is complete. A checked item requires direct evidence tied to the exact source SHA. Do not adjust ledger weights or award credit because a workflow was started, a bundle exists, or a test on another SHA passed.

## 1. Immutable evidence rules

- [x] Candidate source SHA is recorded explicitly in the PR and independent verification selector; the ledger records the freeze rule without embedding a self-referential commit SHA.
- [x] Independent Linux/Windows build and test evidence is tied to the candidate source SHA.
- [x] Candidate Linux and Windows bundles were inspected; embedded checksum manifests verify and build metadata pins the source SHA.
- [ ] Final candidate SHA is frozen after all remaining source changes; all evidence is rerun against that exact SHA.
- [ ] All evidence links, ledger rows, PR body, workflow pins, and artifact metadata agree on the final SHA.
- [ ] Any failed workflow is classified from logs/job metadata; no infrastructure failure is described as a passing test.
- [ ] No hosted-CI gate is silently replaced by a local or independent run. Any approved alternate evidence must execute equivalent checks on the exact SHA and state the remaining hosted-CI limitation.

## 2. Fixed-weight ledger acceptance

Keep the existing weights unchanged. Mark a row verified only after its acceptance criteria are demonstrated by executable tests and/or review artifacts.

- [ ] **E1 — standard library (10 points):** complete the scope-defined public reusable API contracts; tests cover valid/invalid inputs, deterministic behavior, diagnostics, and cross-platform behavior. Existing focused UTC/JSON/text/list/map/integer tests are partial evidence, not automatic full-row credit.
- [ ] **E2 — packaging and provenance (8 points):** deterministic package/lock behavior and stale-lock rejection pass; v2 source-bound provenance signature verifies against an independently configured trusted-publisher store; source tree is rechecked; protected `one-release` release path is exercised without exposing signing seeds or trust secrets.
- [ ] **E3 — conformance and platforms (8 points):** exact-SHA Linux/Windows suites and required Ubuntu 24.04/Debian 12 jobs passed on source SHA `85ded41bea3d7e2f64f0758e1bfacb376f0cb2cd`: [Linux distro run 40217709918](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40217709918) and [cross-platform run 40217709951](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40217709951). Android ELF build/header passed in [run 40217710176](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40217710176), but API 35 `arm64-v8a` runtime remains OPEN. Latest run [40221042296](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40221042296) failed with `HV_UNSUPPORTED`; the ELF execution step was skipped. Build/header evidence is not runtime acceptance.
- [ ] **E4 — security, resource bounds, recovery (6 points):** exact-source adversarial tests passed for Unix descriptor-relative no-follow operations, SQLite limits, capability denial without adapter invocation, Windows reparse/junction handling, and bounded subprocess behavior: [security suite 40251746218](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40251746218), [subprocess matrix 40251750642](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40251750642). The candidate uses `PROC_THREAD_ATTRIBUTE_JOB_LIST` during `CreateProcessW`, and the Windows process-creation guard/test suite passed. Do not accept E4 yet: explicit root descriptor anchoring, mount-boundary enforcement, runtime-wide quotas, fallback-platform guarantees, and the full threat review remain open. No complete sandbox claim.
- [x] **E5 — docs and developer tooling (4 points):** contract checker, documentation consistency checks, examples, CLI create/check/run/build/test/package smoke tests, and developer workflow instructions passed on candidate source SHA `533744e33e8e95ab83e239c2751846701b2604ea` in [cross-platform run 40095539284](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40095539284), [focused run 40095539311](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40095539311), [Linux distro run 40095539287](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40095539287), and [lockfile run 40095539351](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40095539351); candidate artifact integrity passed in [run 40095539261](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40095539261). The ledger evidence row was subsequently aligned to these results; this DoD update creates a new source SHA, so a fresh exact-SHA matrix is mandatory before release acceptance.
- [ ] **E6 — final forensic audit (4 points):** E1–E5 acceptance is reconciled; required docs, specs, plans, completion ledger, checkpoints, CI evidence, PR metadata, and release provenance are mutually consistent; no mandatory gate remains open.

## 3. Mandatory release gate

The release gate is **NOT PASSED** while any required checklist item or fixed-weight ledger acceptance row remains unchecked, while the final SHA is not frozen, or while a required platform/provenance/security gate lacks direct evidence.

Before authorizing a release:

1. Run the complete required verification matrix against the final SHA.
2. Verify artifact checksums and signed provenance independently.
3. Verify the trusted-publisher configuration and protected release environment without disclosing secrets.
4. Review the final diff and security limitations; ensure no unrelated changes were included.
5. Reconcile the completion ledger and PR description to the same evidence.
6. Record the exact final SHA and the URLs of every passing required workflow.
7. Only then may the ledger and release report say **PASS GREEN LOCKED — FINAL RELEASE**.

## 4. Current known open items

- Canonical-repository Actions job-step details remain unavailable through the current connector permission scope. Empty/unavailable step data is not a source-test result and cannot be counted as PASS; independent lab workflows executed actual steps and recorded logs on the exact candidate SHA.
- The exact-SHA Android API 35 runtime attempt is recorded in the Prompt 3 evidence section below. Emulator boot failed on the hosted macOS runner with `HV_UNSUPPORTED`; the Linux ARM64 host could not install a compatible emulator package. No Android ELF runtime PASS exists yet.
- Explicit project-root descriptor anchoring, mount-boundary behavior, runtime-wide quotas, fallback-platform guarantees, and the full threat review remain incomplete. The E3/E4 evidence below is partial and does not close E1–E6 or authorize a release.
- This document does not authorize merging PR #80 or publishing a release.

## 5. Prompt 3 E3/E4 execution record — code-bearing SHA `5751aff3404f000545549908a00ea322cc96e277`

This is an evidence record for the code-bearing SHA, not acceptance of the full E3/E4 rows. The source/documentation branch advances after this update; the independent verification selector must be pinned to the resulting exact branch HEAD and the required matrix rerun.

- [Linux distro matrix](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771630) — Ubuntu 24.04 and Debian 12 passed build, Clippy, workspace tests, and real-network tests.
- [Cross-platform matrix](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771649) — Windows Server 2025 passed the full Windows build/test/Clippy/network/offline/bundle steps. The Ubuntu job was still running at checkpoint time; its final status must be rechecked.
- [SEC-05 subprocess verification](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771660) — Linux and Windows focused/full one-core tests and static guards passed; scoped `rustfmt --check crates/one-core/src/interop.rs` passed. Whole-repository `cargo fmt --check --all` reported unrelated pre-existing formatting drift and is retained as a separate non-blocking diagnostic.
- [E4 adversarial regressions](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40213093522) — Unix descriptor-relative/no-follow (3 tests), SQLite resource/query limits (11 tests), governance policy (10 tests), and Windows directory-junction/reparse escape (1 test) passed. The full Windows workspace suite passed `orchestrator::tests::denied_request_never_reaches_adapter_invocation`.
- [E4 subprocess resource guard](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771671) — passed.
- [Android AArch64 device artifact build](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40213093523) — passed on the exact source SHA. The artifact is ELF64 little-endian AArch64 PIE, interpreter `/system/bin/linker64`, with SHA-256 `b3a52ce990c157332ab39f25264fbdab3de9e796c26f827c497551237855967c`. This does not prove runtime execution.
- [Android API 35 macOS runtime attempt](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771680) — API 35 arm64-v8a image and pre-boot contract checks passed, but emulator boot failed with `HV_UNSUPPORTED`; the ELF execution step was skipped.
- [Android Linux ARM64-host attempt](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40212771636) — failed before runtime because a compatible Android emulator package was unavailable. Neither Android attempt is a PASS.

**Disposition:** E3 remains unverified because no API 35 ELF execution/output assertion was captured. E4 remains unverified because project-root descriptor anchoring, mount-boundary enforcement, runtime-wide quotas, fallback-platform guarantees, and the complete threat review remain open. The fixed-weight ledger remains **91/100 provisional implementation, 60/100 verified**; release gate remains **NOT PASSED**. PR #80 remains unmerged; no release is authorized.
## 6. Exact-SHA follow-up — Windows output-flood stress tests and Android AVD setup

The first final-SHA matrix attempt on `26d3d30be773cfc403daa002f7116c176ef31fc8` failed two Windows output-flood tests: `stderr_flood_is_bounded` and `aggregate_output_limit_is_enforced`. Logs showed the test's 3-second deadline fired before the process reached the byte cap. Commit `c339da876f6ceb18084e8551cc08d4edbabc0f27` changes only the test timeout for stdout/stderr/aggregate flood cases to 15 seconds; the assertion still requires an output-limit error and still fails if a timeout occurs. Production timeout and output limits are unchanged. A fresh exact-SHA Windows matrix is mandatory.

The first software-acceleration fallback attempt also revealed that the lab workflow had not created the `one-api35-arm64` AVD. The workflow now explicitly creates and verifies the API 35 `arm64-v8a` AVD before attempting hardware acceleration and the bounded `-accel off` fallback. The failed launch is a harness failure, not runtime acceptance; only actual ELF execution and `ONE_NATIVE_BACKEND_OK` on API 35 count.

This follow-up and the remaining checkpoint documentation create a new candidate branch HEAD. Pin the independent lab to that final exact SHA and rerun the matrix before acceptance. E3 and E4 remain unchecked, and the release gate remains NOT PASSED.