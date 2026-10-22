# ONE vNext Session Checkpoint

**Updated:** 2026-10-22  
**Workstream:** Deterministic human-instruction grammar vNext  
**Pull request:** [#78](https://github.com/panglimanetizen01/one/pull/78)  
**Branch:** `spec/human-instruction-grammar-vnext-20261022`

## Target milestone

Continue the accepted vNext human-instruction grammar implementation. The active request in this session did not specify which cumulative 30% / 60% / 100% gate applies, and no weighted completion ledger or exact weight formula was found in the repository. Therefore, a numeric project completion percentage is **NOT CALCULATED** rather than guessed. Do not silently invent weights or mark a milestone PASS.

## Code checkpoint

- Code revision before this checkpoint document: `42392c007905d99e6a8b56d60e700271ceaf86cd`
- Latest code change: `crates/one-core/src/human_vnext.rs`
- Change: preserve blank lines and a trailing newline in introduced indented text blocks; added regression test `preserves_blank_lines_and_trailing_newline_in_text_blocks`.
- Commit: [42392c0](https://github.com/panglimanetizen01/one/commit/42392c007905d99e6a8b56d60e700271ceaf86cd)

## Evidence and verification

- Repository contract read: `AGENTS.md`, `docs/ONE_CONSTITUTION.md`, `docs/DEVELOPMENT_PROTOCOL.md`, `docs/DECISIONS.md`.
- PR #78 head confirmed at code revision `42392c007905d99e6a8b56d60e700271ceaf86cd`; PR remains open and draft.
- CI run [39751590947](https://github.com/panglimanetizen01/one/actions/runs/39751590947) failed on Ubuntu and Windows with both jobs reporting **zero steps**. This is insufficient evidence of a source-code test failure.
- A retry of the prior infrastructure-shaped failure was requested, but the subsequent run still failed before any steps.
- Rust build, Clippy, and tests against the updated SHA: **NOT VERIFIED**. The GitHub connector does not provide a functioning execution environment for running the Rust test suite locally.

## Done

- [IMPLEMENTED] Text-block parser now reinserts blank source lines between meaningful lines and retains trailing blank lines/newline according to the current candidate contract.
- [IMPLEMENTED] Regression test added to assert exact text value for an internal blank line and trailing newline.
- [VERIFIED] The commit and changed source/test are present on the PR branch.
- [NOT VERIFIED] Compilation, test execution, Clippy, full CI, semantic integration, runtime behavior, and persistence.

## Remaining work

1. Diagnose why hosted CI jobs terminate before the first step; do not classify this as a Rust failure without logs/evidence.
2. Run targeted tests and formatting/lint/build checks on the exact head SHA in a functioning Rust environment.
3. Add tests for CRLF, relative indentation within text blocks, blank-only/empty text, and block-boundary edge cases.
4. Define and implement the versioned vNext compile/CLI path without changing normative v0.1 behavior.
5. Add deterministic semantic validation and stable diagnostics for all supported productions.
6. Implement the task-list acceptance application and durable storage adapter; prove persistence by starting it twice and observing retained data.
7. Add positive/negative fixtures and deterministic AST/IR evidence; update specs/status evidence to match actual behavior.
8. Establish or locate the canonical weighted completion ledger and its 30% / 60% / 100% formulas before asserting numeric completion.

## Git state

- Branch: `spec/human-instruction-grammar-vnext-20261022`
- Code SHA: `42392c007905d99e6a8b56d60e700271ceaf86cd`
- PR: #78, open, draft, not merged.
- Main was not modified; no merge was performed.

## Gate

**NOT PASSED** — code is committed, but build/test/Clippy/CI gates and end-to-end vNext acceptance requirements are not yet proven.
