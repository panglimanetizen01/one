# ONE vNext Session Checkpoint

**Updated:** 2026-10-24  
**Active target:** Milestone 100% — ecosystem and release quality  
**Repository:** [panglimanetizen01/one](https://github.com/panglimanetizen01/one)  
**Active PR:** [#80](https://github.com/panglimanetizen01/one/pull/80)  
**Working branch:** `milestone/100-ecosystem-20261023`  
**Base branch:** `main` — unchanged; no merge performed.

## Completion ledger (fixed weights; no reweighting)

Canonical ledger: [one-completion-ledger.md](one-completion-ledger.md).

- **Implementation subtotal:** 82/100 provisional points.
- **Verified completion:** 68/100 points.
- **E1–E6 verified points:** 8/40 (E3 cross-platform/conformance row verified); E1, E2, E4, E5, and E6 remain unverified. The 14 remaining provisional ecosystem implementation points are not verified completion.
- **Milestone 100% gate:** **NOT PASSED**.
- The previously earned 60% milestone remains historical and scoped to its exact source SHA and evidence. It does not make the current 100% candidate green.

## Current Git and CI evidence

- PR #80 is **open, draft, and unmerged**; no merge is authorized by this checkpoint.
- Latest code-bearing source SHA for this implementation slice: `5da1f4dfcfba7bc64186eaa6a4ab018e6703c8a6`. This candidate adds checked integer remainder coverage, bounded JSON serialization, explicit JSON control-character escaping tests, and retries for interrupted FTP/SMTP protocol reads. This checkpoint commit is documentation-only and is tracked separately from the tested code SHA.
- Canonical workflow run [39994484534](https://github.com/panglimanetizen01/one/actions/runs/39994484534), “One CI”, completed with **failure**. The two reported jobs (`test (windows-2025)` and `test (ubuntu-latest)`) failed; job-step details/logs were not returned by the available connector. Root cause is **UNKNOWN** from available evidence; do not infer a source failure or a pass.
- The associated Linux Distribution Conformance run [39994484625](https://github.com/panglimanetizen01/one/actions/runs/39994484625) and Standard Library Focused Verification run [39994484574](https://github.com/panglimanetizen01/one/actions/runs/39994484574) were **cancelled**. Subsequent runs on the integer-remainder candidate also report failure: [One CI 39995112864](https://github.com/panglimanetizen01/one/actions/runs/39995112864), [Linux distro 39995112888](https://github.com/panglimanetizen01/one/actions/runs/39995112888), and [focused stdlib 39995112905](https://github.com/panglimanetizen01/one/actions/runs/39995112905). The available job-step endpoint returned empty step lists for these jobs, so the underlying failure cause is **UNKNOWN**; this does not establish that the source tests failed or passed. The latest canonical runs on this branch have repeatedly failed before steps start: jobs report `runner_id: 0` and `steps: []`; retry attempt 2 also failed identically. This is an Actions runner provisioning failure, not evidence that source tests failed. The independent mirror fallback runs the required source checks.
- Older independent-mirror PASS results only apply to their explicitly recorded older source SHAs. They do not verify the current PR head.
- Exact-SHA independent verification is established for code SHA `07780078c2563f3622d036f483735b262eb298d6`: focused stdlib, Ubuntu/Debian distro, and Ubuntu/Windows cross-platform runs passed. This evidence applies to that SHA only; after this checkpoint-only commit, the new branch tip must be re-verified before claiming it is the final tested SHA.

## Implemented scope and remaining acceptance

The current milestone branch contains ecosystem work including standard-library helpers and UTC formatting, JSON limits, HTTP framing/size bounds, filesystem adapter checks, package/build tooling and SHA-256 module lock v2. Source presence alone is not acceptance evidence.

Remaining mandatory requirements include, at minimum:

- Complete and tested standard-library APIs, including the still-missing/incomplete time/date, concurrency, logging/configuration and persistence integration contracts.
- External dependency declarations and registry resolution; authenticated package identity/provenance; target-specific artifact/release workflow.
- Full current-SHA conformance and compatibility matrix on required platforms, with no substituted historical evidence.
- Race-resistant filesystem confinement, general resource quotas, and complete threat/recovery review.
- Complete, tested documentation/examples/developer workflow and compatibility review.
- Final forensic Definition-of-Done audit and release evidence on the exact release SHA.

## Actions taken in this session

- Read `AGENTS.md`, `docs/ONE_CONSTITUTION.md`, `docs/ARCHITECTURE.md`, `docs/REQUIREMENTS.md`, `docs/COMPATIBILITY.md`, `docs/INTEROPERABILITY.md`, `docs/SECURITY.md`, ADR-0017, the weighted completion ledger, the prior checkpoint, the stdlib implementation and relevant CI workflow definitions.
- Confirmed PR #80 is the active open/draft/unmerged milestone-100 branch; no merge or main-branch change was performed.
- Implemented `one_core::stdlib::number_modulo`: checked signed-integer remainder rejects wrong value types, divisor zero, and the `i64::MIN % -1` overflow case.
- Integrated the source-level builtin `angka sisa bagi` through runtime dispatch and semantic arity/type/result handling.
- Added unit regression coverage and a canonical-pipeline integration test for positive and negative remainder, wrong argument type, and division-by-zero handling.
- Updated `docs/spec/one-v0.1-stdlib.md` with the exact operation/error contract.
- Relevant commits:
  - `17a27f667d7178298b29d58ae1e308be9b9d81f3` — checked stdlib helper and unit tests.
  - `961e3354f6fd655640ff60f82f919a2a0f08247f` — runtime builtin integration.
  - `7f1eab3feaa23cadaaeffc9eb746f0f96b46f6a7` — semantic type/arity integration.
  - `f93be7ea2e98a577abb3b764e7f0b72ab4d0fd08` — end-to-end regression test (follow-up test-fixture cleanup).
  - `0f826510595472777cceab887cf2e36bb77d015b` — standard-library contract documentation.
  - `cac099caa28bd9eeed3c72ded101a70ccec39b5a` — checkpoint update recording implementation and verification boundaries.
- Verification evidence for source SHA `809a26d12a411b492f0888610dc9ebabf1a6992b` from the independent mirror:
  - Focused stdlib run [39995613378](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39995613378): **PASS** — exact-SHA checkout succeeded; UTC unit/boundary test, UTC pipeline test, prefix/suffix pipeline test, checked integer remainder pipeline test, and strict one-core Clippy all passed.
  - Linux distro run [39995613439](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39995613439): **SUCCESS** — Ubuntu 24.04 and Debian 12 both passed contract checks, build, Clippy, native workspace tests and real-network tests.
  - Full cross-platform run [39995613393](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39995613393): **IN PROGRESS** at the last inspection. Ubuntu had passed build, AST contract tests, Clippy and workspace tests; real-network runtime tests were still running. Windows was still provisioning the cross-compiler. No full cross-platform PASS is claimed yet.
  - The first focused run [39995473148](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39995473148) correctly caught the invalid arithmetic-in-argument fixture (E1014). The fixture was fixed in `809a26d12a411b492f0888610dc9ebabf1a6992b` and the focused workflow then passed.
  - Local clone/test execution is unavailable because this environment cannot resolve `github.com`; verification evidence above comes from the independent GitHub Actions mirror.
- No verified ledger points are added for this narrow stdlib slice: E1 acceptance remains partial and E1–E6 verified points remain 0/40. The 100% gate remains NOT PASSED.

## Next execution sequence

1. Re-pin the mirror selector to the checkpoint commit SHA and rerun focused stdlib, Linux distro, and Ubuntu/Windows cross-platform workflows on that exact final checkpoint SHA.
2. Continue E1, E2, E4, and E5 implementation against the fixed acceptance criteria; each slice must receive focused and full-matrix verification before ledger credit.
3. Finish required standard-library breadth, dependency/package provenance, target artifacts, race-resistant filesystem confinement/resource quotas, security review, compatibility and developer workflow gaps.
4. Resolve the main-repository Actions runner provisioning failure if the repository's runner service becomes available; do not fabricate or manually mark checks green.
5. Perform E6 forensic audit only after every mandatory gate is satisfied.

## Gate

**NOT PASSED — Milestone 100%.** The current PR is draft/unmerged; the latest reported canonical CI failed, companion verification workflows were cancelled, and no current-head exact-SHA green matrix was established in this session.
