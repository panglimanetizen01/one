# ONE vNext Session Checkpoint

**Updated:** 2026-10-23  
**Active target:** Milestone 60% — core language/runtime  
**Repository:** [panglimanetizen01/one](https://github.com/panglimanetizen01/one)  
**Implementation PR:** [#79](https://github.com/panglimanetizen01/one/pull/78)  
**Foundation PR:** [#78](https://github.com/panglimanetizen01/one/pull/78)  
**Active branch:** `spec/human-instruction-grammar-vnext-20261022`

## Completion and weighted method

The canonical weighted ledger is [one-completion-ledger.md](one-completion-ledger.md). Weights are fixed; provisional implementation points do not count as verified completion.

- **Implementation subtotal:** 60/100 points are now represented by code/tests/docs for F1–F6 and the C1–C5 candidate.
- **Verified completion:** 30/100 points. The 30-point Foundation scope remains supported only by its recorded exact implementation SHA `26199c0114ea0da5f9784ea4d000ab185799e4fc` and evidence in the ledger.
- **Milestone 60 gate:** **NOT PASSED** until the current candidate passes required exact-SHA Linux and Windows verification. C1–C5 currently earn 0 additional verified points.
- The adapter does not claim a complete native vNext grammar. Map values and broader vNext-specific syntax remain unimplemented.

## Current Git state

- PR #78 is open, draft, and not merged. Base is `main` at `d86c3be75c81a27d6d4f040f21718f6248413e47`.
- Latest code-bearing implementation SHA: `9f03d55a38a7b4da1ddb92328dc716a439d4a278`. The checkpoint/ledger updates that follow are documentation-only.
- PR #78 remains open/draft and unmerged; no branch was merged into `main`.
- Do not change `main`, merge either PR, or touch `~/one-termux` without explicit authorization.

## Implemented candidate scope (not yet verified)

1. `crates/one-core/src/human_compiler.rs`: validates a `Buat program <nama>` header and passes the body through canonical parse → semantic analysis → IR lowering.
2. `one jalankan-human-core <program.one>` / `one run-human-core`: executes through the canonical reference interpreter; no second evaluator was added.
3. Relative `Gunakan modul "file.one"` resolution for modules declared with `Buat modul <nama>`, with explicit rejection of path traversal, duplicate imports and import cycles.
4. CLI tests cover function parameters/results, arithmetic, conditions, collection operations, loops, assignment, type errors, scope errors, malformed headers, and module resolution failures.
5. `crates/one-core/src/persistence.rs`: versioned `ONE-KV-1` key/value storage, deterministic encoding, size/count bounds, corruption/duplicate-key rejection, restart tests, rollback tests and bulk replacement.
6. Calculator, score-tracker, and multi-file module examples use the same canonical parser/analyzer/IR/interpreter path.
7. Existing v0.1 command routing and existing human-vNext task-list runner remain separate. The new reusable key/value store has not replaced the task app's legacy storage file format.

## Exact-SHA verification history

- Canonical repository Actions on earlier commits have repeatedly terminated before steps started (empty steps and runner ID 0). Those runs are neither a source pass nor evidence of a code failure.
- Independent mirror run [39931638753](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39931638753) verified checkout and build on candidate SHA `6efb3aaac6a3559314d5913e5188bac3f5a2fb03`, but Ubuntu Clippy failed due to `items_after_test_module` and strict-Clippy findings in the new persistence file. Those findings were fixed in later commits; that run does not verify the current SHA.
- Independent mirror run [39931908944](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39931908944) tested SHA `df5439c484cda6cfd3da333a6cabf3856a82579e`; Ubuntu Clippy exposed one additional collapsible-if warning, fixed in `c4b502f3aabea95486f4a506234edde1945da538`. The old run does not verify the fix.
- The mirror's `CANONICAL_COMMIT.txt` must target code-bearing SHA `9f03d55a38a7b4da1ddb92328dc716a439d4a278`; later checkpoint/ledger-only commits do not alter code, tests, or workflows. Accept only a completed run with both `ubuntu-latest` and `windows-2025` successful and all required steps completed on the exact selected source SHA.
- Canonical repository Actions status is not described as green based on the mirror; report the two separately.

## Next actions

1. Set the independent verification mirror's `CANONICAL_COMMIT.txt` to the final PR #78 HEAD after all code, ledger, and checkpoint edits.
2. Inspect Linux/Windows build, strict Clippy, workspace tests, real-network tests and offline verification. Fix every source finding and rerun the exact-SHA gate after any code change.
3. If all required checks pass, update C1–C5 verified points from 0 to their fixed weights, record the final exact SHA and mirror run evidence, and set milestone 60 to PASS GREEN LOCKED.
4. If any required check is pending or fails, keep milestone 60 NOT PASSED and continue from the exact reported blocker.

## Gate

**NOT PASSED** — milestone 60 is implemented as a candidate but has not yet passed the required exact-SHA cross-platform acceptance checks. The previous 30% Foundation gate is a separate, bounded claim tied to its own recorded implementation SHA; it is not evidence that the current 60% milestone passes.
