# Forensic Audit — Contract, Documentation, and Verification State (2026-10-21)

## Scope

Reviewed the canonical `main` repository artifacts for specification gate status, lexical/AST source-span guarantees, verification evidence wording, implementation status, CI behavior, and unresolved project state. This audit distinguishes repository-source defects from hosted-runner/infrastructure failures.

## Findings and dispositions

| ID | Severity | Finding | Disposition |
|---|---|---|---|
| FA-01 | High | Language invariants remained marked as a draft even though lexical and grammar were normative and AST was accepted; the AST also depended on “approved language invariants.” | Resolved by ADR-0016 and synchronized specification-index/status wording. |
| FA-02 | High | Lexical specification required preserving a complete multi-word-name span after parsing, while the accepted AST explicitly normalizes names and does not retain component/name spans. | Resolved: lexical contract now guarantees token-level word spans only and explicitly excludes a full name range from the v0.1 AST. |
| FA-03 | High | Evidence matrix still said cross-platform support for current `main` was unverified until a merged revision was independently executed, contradicting the recorded exact-SHA Ubuntu/Windows matrix and its later documentation-only commits. | Resolved: evidence now identifies the last code-bearing source SHA and preserves the target-execution/release-approval limits. |
| FA-04 | High / external blocker | Canonical hosted CI run [39558774194](https://github.com/panglimanetizen01/one/actions/runs/39558774194) failed on both Ubuntu and Windows jobs with no usable step summaries. Fetching the job logs returned `BlobNotFound`, so the failure cannot be attributed to a source step. Earlier runs 39372356906 and 39381125667 had the same zero-step pattern. | Not repairable by changing source based on current evidence. Reopen/track as a hosted CI infrastructure blocker; retry and inspect the run from GitHub Actions when the runner service yields usable logs. Do not classify it as a Rust test failure. |
| FA-05 | Medium | The implementation status said the foundation was complete by reference to approved architecture/spec docs while the language-invariants document still claimed draft status. | Resolved by recording the invariant contract as accepted and separating contract acceptance from implementation conformance. |
| FA-06 | Medium / still a gate | The detailed localization contract remains a draft and additional locales are future work. | Correctly retained as draft/future scope; do not claim multi-locale conformance. |
| FA-07 | Medium / still a gate | Host-level Ubuntu/Windows source verification does not by itself prove execution of every cross-compiled native target. | Correctly retained as a target-specific evidence boundary; AArch64 and x86_64 claims must cite their actual target executor/artifact evidence. |

## Evidence boundary

The last code-bearing source revision with a recorded successful independent Ubuntu/Windows exact-SHA matrix is `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`, run [39554661451](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451). The subsequent evidence-sync commits are documentation-only. The matrix covered contract artifacts, build, AST regression tests, Clippy, workspace tests, real-network tests, and platform-appropriate offline verification.

That result is not a blanket release approval and does not imply that every cross-compiled target was executed. The latest canonical hosted CI failure is inconclusive because no steps/logs were available.

## Next actions

1. Resolve the canonical hosted CI zero-step runner issue and obtain a usable successful run on a code-bearing source SHA.
2. Keep native target execution claims tied to actual target execution, not only cross-compilation or host test matrices.
3. Complete review and approval of the detailed localization contract before claiming additional locale support.
4. Expand conformance coverage for the invariant edge-case classes and retain exact-SHA evidence for any source changes.
5. Periodically compare documentation status tables against the canonical verification records; do not promote draft contracts or target capabilities based on file presence alone.
