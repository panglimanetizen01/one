# ONE vNext Session Checkpoint

**Updated:** 2026-10-22  
**Active target:** Milestone 30% — foundation and first runnable reference application  
**Repository:** [panglimanetizen01/one](https://github.com/panglimanetizen01/one)  
**Pull request:** [#78](https://github.com/panglimanetizen01/one/pull/78)  
**Branch:** `spec/human-instruction-grammar-vnext-20261022`

## Completion

The canonical weighted ledger is [one-completion-ledger.md](one-completion-ledger.md). Its weights are fixed for future sessions.

- **Implementation progress:** 21/100 points (provisional, includes work not yet verified).
- **Progress inside the 30-point foundation tranche:** 21/30 points implemented (70% of that tranche).
- **Verified completion:** 0/100 points for this newly introduced vNext tranche at the current code revision because no Rust build/test/Clippy run has completed successfully on the exact revision.
- **Milestone 30% gate:** **NOT PASSED**. Do not report the overall project as 30% complete or PASS GREEN LOCKED.

## Latest code revision

- **Last code-changing SHA:** `8c4f7c4706365972797446382989b81335434d92`
- **Source:** [human_vnext.rs](https://github.com/panglimanetizen01/one/blob/8c4f7c4706365972797446382989b81335434d92/crates/one-core/src/human_vnext.rs)
- **CLI:** [main.rs](https://github.com/panglimanetizen01/one/blob/8c4f7c4706365972797446382989b81335434d92/crates/one-cli/src/main.rs)
- **CLI integration test:** [cli.rs](https://github.com/panglimanetizen01/one/blob/8c4f7c4706365972797446382989b81335434d92/crates/one-cli/tests/cli.rs)
- **Example:** [daftar-tugas.one](https://github.com/panglimanetizen01/one/blob/spec/human-instruction-grammar-vnext-20261022/examples/human-vnext/daftar-tugas.one)
- **Grammar contract:** [grammar draft](https://github.com/panglimanetizen01/one/blob/spec/human-instruction-grammar-vnext-20261022/docs/superpowers/specs/2026-10-22-one-human-instruction-grammar-vnext-draft.md)
- **User instructions:** [task-list guide](human-vnext-task-list.md)

## Implemented in this session

- Added preservation and tests for blank lines, trailing newlines, CRLF, relative indentation and Unicode text in text blocks.
- Defined stricter deterministic name syntax and duplicate-persistence rejection.
- Added task-list action instructions to the human-vNext parser.
- Added an experimental task-list runner supporting add, list, mark-complete, input validation, durable file storage, reload on restart, corrupt-record errors, and temporary-file/rename writes.
- Added explicit `one jalankan-human-vnext <program.one> [--data <file>]` CLI entry; the default v0.1 command path remains separate.
- Added a runnable human-vNext source example and usage documentation.
- Added core runtime tests and a CLI subprocess test for persistence across process restarts.
- Formalized the starter subset grammar and versioning/boundary rules.
- Added a weighted completion ledger and static contract checks for the example and ledger.

## Verification and failures

- **Source presence:** VERIFIED through GitHub file reads and commit metadata.
- **Rust compile, format, Clippy and unit/integration tests:** **NOT VERIFIED**. This environment has no functioning Rust execution path for the private checkout.
- **CI:** [Exact-code-SHA run](https://github.com/panglimanetizen01/one/actions/runs/39752818112) failed on Ubuntu and Windows with zero steps in both jobs. The logs endpoint returned 404 and no runner IDs were allocated. This is a runner/workflow execution blocker, not evidence that the Rust source failed, and not a passing result.
- **Exact current code SHA CI:** run [39752818112](https://github.com/panglimanetizen01/one/actions/runs/39752818112) is attached to `8c4f7c4706365972797446382989b81335434d92`; both matrix jobs failed before any steps started.
- **Canonical ONE AST/IR integration:** NOT IMPLEMENTED. The human-vNext task runner is a dedicated experimental interpreter for the supported task-list action shapes; it does not yet lower into the canonical ONE v0.1 IR.
- **General-purpose vNext execution:** NOT IMPLEMENTED. Unsupported constructs must not be presented as working.
- **Persistence end-to-end:** implementation and regression tests are present, but actual test execution is NOT VERIFIED.

## Git state

- PR #78 remains **open, draft, not merged**.
- Base branch: `main`; base SHA observed: `d86c3be75c81a27d6d4f040f21718f6248413e47`.
- Last code-changing SHA: `8c4f7c4706365972797446382989b81335434d92`.
- Main was not modified and no merge was performed.
- Documentation-only checkpoint/ledger commits may follow the code SHA; refresh the actual PR head before further work.

## Remaining work to reach 30%

1. Get a functioning exact-SHA Linux/Windows CI run. Diagnose repository Actions availability/runner setup outside source code if jobs still terminate before their first step.
2. Run `cargo fmt --check`, `cargo build --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, and `python tools/check_contract.py`; fix actual failures and rerun.
3. Review storage atomic replacement on Windows, invalid/corrupt data handling, Unicode/control-character cases, empty text and block-boundary cases.
4. Verify default v0.1 parser/CLI compatibility on the same SHA.
5. Resolve the frontend integration gate: either lower supported vNext nodes into the canonical AST/IR/runtime or explicitly revise the milestone contract only through reviewed governance. Do not call the current task-specific runner the general compiler pipeline.
6. Once all F1–F6 criteria earn their full 30 verified points, update the ledger and checkpoint with exact SHA and successful CI evidence.

## Next action

First, refresh PR #78 head and workflow state. Then restore a functioning Rust/Actions verification path and run the exact-SHA suite. Fix code defects revealed by real diagnostics; do not weaken or disable gates to get green.

## Gate

**NOT PASSED** — substantial implementation is committed, but exact-SHA tests/CI and canonical execution-pipeline acceptance are not proven.
