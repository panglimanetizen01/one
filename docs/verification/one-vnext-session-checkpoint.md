# ONE vNext Session Checkpoint

**Updated:** 2026-10-23  
**Active target:** Milestone 60% — core language/runtime  
**Repository:** [panglimanetizen01/one](https://github.com/panglimanetizen01/one)  
**PR:** [#78](https://github.com/panglimanetizen01/one/pull/78)  
**Canonical branch:** `spec/human-instruction-grammar-vnext-20261022`

## Completion

The canonical weighted ledger is [one-completion-ledger.md](one-completion-ledger.md). Weights are unchanged.

- **Implementation:** 60/100 points.
- **Verified completion:** 60/100 points.
- **Gate:** **PASS GREEN LOCKED — milestone 60% only.**
- **Code-bearing source SHA:** `3027e43b9e2481d73bdb67dbbbbb2e27c3419639`.
- **Exact-SHA verification:** [run 39936123376](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39936123376) — Ubuntu and Windows jobs both successful.

## Verified acceptance evidence

On exact code SHA `3027e43b9e2481d73bdb67dbbbbb2e27c3419639`, the documented independent mirror completed:
- Contract artifacts and compatibility checks.
- Build and AST contract regression tests.
- Strict Clippy.
- Workspace tests, including human-core integration tests for function calls/results, values and arithmetic, conditions, loops, lists/maps, type errors, scope errors, source-located diagnostics, duplicate/cyclic/path-unsafe module imports, persistence behavior, and actual checked-in example programs.
- Real-network runtime tests.
- Platform-appropriate offline verification.

Linux core test suite: 160 passed, 0 failed; Windows core test suite: 156 passed, 0 failed. One Android AArch64 native execution test is ignored on each OS because the hosted environment lacks the required Android execution-capable Clang setup. It is explicitly not counted as passed and is not a required milestone-60 gate.

## Implemented and integrated scope

1. Explicit `Buat program <nama>` frontend and `one jalankan-human-core <program.one>` / `one run-human-core` route through the canonical parser, semantic analyzer, IR lowering, and reference interpreter.
2. Values and types include text, numbers, booleans, empty values, lists, and deterministic text-keyed maps. Map create/insert/read/delete operations have type/runtime errors; experimental map grammar is opt-in and does not change default v0.1 parser acceptance.
3. Conditions, loops, assignment, functions/parameters/results, name resolution, block scope, and source-located semantic diagnostics have positive and negative tests.
4. Relative multi-file modules use `Buat modul` / `Gunakan modul`, reject absolute/traversal paths, external symlinks, duplicate imports and cycles, and share one top-level semantic namespace. Per-module private/export syntax and original-file source maps for imported-file diagnostics remain future work.
5. `one_core::persistence::KeyValueStore` provides versioned `ONE-KV-1` storage, deterministic encoding, size/entry limits, corruption rejection, restart, rollback, and bounded replacement tests. The existing task-list app continues using its prior storage format; no silent migration was performed.
6. Checked-in calculator, score-tracker, module and mapping examples execute through the same canonical compiler/runtime.
7. The completion ledger and ADR-0017 record the fixed weights, design choices and compatibility limits.

## Git and CI state

- PR #78 remains open/draft and unmerged.
- `main` was not changed; no merge was performed.
- `~/one-termux` was not accessed or modified.
- Canonical repository Actions run [39936118585](https://github.com/panglimanetizen01/one/actions/runs/39936118585) failed before executing steps (`runner_id=0`, zero steps on both OS jobs). This is reported separately and is not claimed as a green canonical run.
- The repository's [exact-SHA verification contract](human-vnext-exact-sha-verification-contract.md) permits the independent mirror fallback. Run 39936123376 passed both required OS jobs on exact source SHA `3027e43b9e2481d73bdb67dbbbbb2e27c3419639`.

## Next

Continue to milestone 100% using the same fixed-weight ledger. Focus on E1–E6 ecosystem/release requirements, complete native vNext grammar and source-map/visibility work, persistence integration/migration only with explicit compatibility tests, and final forensic audit. Do not change the 60% weights or claim the full ONE ecosystem is complete.

## Gate

**PASS GREEN LOCKED — milestone 60% only**, at code SHA `3027e43b9e2481d73bdb67dbbbbb2e27c3419639`, supported by exact-SHA cross-platform run 39936123376.
