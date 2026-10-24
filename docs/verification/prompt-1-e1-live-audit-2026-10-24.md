# Prompt 1 — E1 live audit checkpoint (2026-10-24)

## Scope and exact source revision

- Repository: https://github.com/panglimanetizen01/one
- Work branch: `milestone/100-ecosystem-20261023`
- Initial code-audit HEAD: `091e01b2fde3d860aa6d74fa866c53d03ad6f187`.
- Latest test-change commit before this checkpoint refresh: `a996653bc638f202713a7d6169ea112096238e27`.
- Historical checkpoint SHA supplied by the task: `54f3de667e05075805fcc5cd5dc621b653075667` (not HEAD; docs-only DoD update).
- PR: https://github.com/panglimanetizen01/one/pull/80 (open, draft, unmerged).
- `main` HEAD at audit: `d86c3be75c81a27d6d4f040f21718f6248413e47`. No change made to `main`.

## Repository contract read

Read `AGENTS.md`, `docs/ONE_CONSTITUTION.md`, `docs/DECISIONS.md`, `docs/spec/one-v0.1-standard-library.md`, `docs/spec/one-v0.1-stdlib.md`, `docs/verification/DEFINITION_OF_DONE.md`, `docs/verification/one-completion-ledger.md`, `docs/verification/one-milestone-100-session-checkpoint.md`, `docs/verification/one-v0.1-evidence-matrix.md`, and `.github/workflows/stdlib-focused.yml`. `ARCHITECTURE.md` is not present at the audited branch root (404); architecture decisions are in `docs/DECISIONS.md` and domain specs.

## E1 contract inventory

Normative source-level call surface listed in `docs/spec/one-v0.1-stdlib.md`:
- Text: `panjang teks`, `teks memuat`, `teks diawali`, `teks diakhiri`, `huruf kecil`, `huruf besar`, `pangkas teks`, `ganti teks`, `pisah teks`, `ubah teks ke angka`.
- Collections: `panjang daftar`, `gabung teks`, `jumlah daftar angka`, `ambil indeks`, `nilai pemetaan`, `atur pemetaan`.
- Integers: `nilai mutlak`, `bagi bulat`, `angka sisa bagi`, `angka minimum`, `angka maksimum`.
- JSON: `ubah nilai ke json`, `nilai dari json`.
- UTC: `waktu utc dari unix`, `unix dari waktu utc`.

The Rust helper module additionally exposes `display_value`, `list_append`, `list_remove`, `text_length`, `text_contains`, `text_starts_with`, `text_ends_with`, `text_lowercase`, `text_uppercase`, `text_trim`, `text_replace`, `text_split`, `text_to_integer`, `list_length`, `list_get`, `list_join_text`, `list_sum_numbers`, `map_get`, `map_set`, `number_abs`, `number_divide`, `number_modulo`, `number_min`, `number_max`, `value_to_json`, `json_to_value`, `unix_seconds_to_utc_text`, and `utc_text_to_unix_seconds`. Mapping helpers and `display_value` are Rust-level helpers, not source-level built-ins. Reserved standard-library domains are not automatically stable APIs.

## Live hosted workflow evidence

- Initial code-audit runs: https://github.com/panglimanetizen01/one/actions/runs/40099732249 and https://github.com/panglimanetizen01/one/actions/runs/40099732245.
- Test-change commit: `a996653bc638f202713a7d6169ea112096238e27`.
- Focused stdlib run on that exact SHA: https://github.com/panglimanetizen01/one/actions/runs/40100658290 (attempt 2). The run failed before executing steps: `steps: []`, `runner_id: 0`, `runner_name: ""`, start/completion only seconds apart; job logs endpoint returned `BlobNotFound`.
- The Linux distribution workflow on the same SHA also failed before steps: https://github.com/panglimanetizen01/one/actions/runs/40100658294. The retry did not restore a runner.
- Artifact endpoint for the earlier canonical run returned zero artifacts.
- Classification: **hosted runner/infrastructure failure; no source test result**. This is not a test failure diagnosis and not a pass.
- Earlier exact-SHA independent mirror runs listed in PR #80/ledger are historical and cannot verify current HEAD.

## Code and test review notes

- The normative contract explicitly rejects empty split separators, wrong input types, invalid signed-i64 conversion, negative/out-of-range list indices, non-text join members, non-integer/overflowing list sums, missing map keys, checked integer overflow, division/remainder edge cases, JSON floats/exponents, duplicate JSON keys, malformed escapes/surrogate pairs, excessive nesting, oversized JSON input/output, invalid UTC dates, offsets/fractions/leap seconds, and non-canonical UTC forms.
- Current `stdlib.rs` implements checked index conversion via `usize::try_from`, checked arithmetic, ordered `BTreeMap` serialization, bounded JSON parsing/serialization, strict integer parsing, and strict UTC helpers. Existing integration tests cover a meaningful subset through parse → semantic analysis → IR → interpreter.
- This inspection did not establish a newly failing source assertion or a specific production bug. No production-code change is justified without a reproducible failing test/root cause.
- The available GitHub connector can inspect and commit repository contents but does not provide a shell/checkout execution environment. Therefore I did **not** claim to have run Cargo, Clippy, contract checker, or workspace tests locally. No test pass is asserted here.

## Follow-up changes on 2026-10-24

- Added three source-level regression-test groups in `crates/one-core/tests/stdlib_builtins.rs` at commit `a996653bc638f202713a7d6169ea112096238e27`: Unicode scalar/empty-text/i64 boundaries; empty-list join, invalid indices and wrong join member type; UTC parser year-zero and upper-year boundary.
- These tests were committed but **not executed successfully**. Local clone/test execution is unavailable in this session because the container cannot resolve `github.com`; hosted CI retry on the exact test SHA again had no assigned runner or steps. No claim is made that the new assertions compile or pass until a runner executes them.

## Acceptance status and required continuation

- E1 remains **PARTIAL / NOT VERIFIED (0/10 ledger points)**. The repository's own Definition of Done explicitly says focused UTC/JSON/text/list/map/integer tests are partial evidence and E1 requires the full scope-defined API contract plus valid/invalid, deterministic, diagnostic, and cross-platform tests.
- No ledger weights or acceptance boxes were changed.
- Before E1 can close: restore a functioning runner or an approved independent execution environment; execute the focused suite on the exact source SHA, `cargo --locked test --workspace`, `cargo --locked clippy --workspace --all-targets -- -D warnings`, and `python tools/check_contract.py`; triage any actual failing assertion, fix only a reproducible root cause, then rerun the complete matrix on the resulting exact SHA. Independent verification must remain labelled as independent, never hosted CI.
- Prompt 2 should begin only after E1 has a reproducible exact-SHA test record and this checkpoint is superseded with actual results. E2 provenance/protected-release acceptance, E3 Android AArch64 runtime, E4 open security/root-confinement/resource-bound review, and E6 final forensic reconciliation remain open.
- Release gate remains **NOT PASSED**. PR #80 remains open/unmerged; no release was authorized or published.

## Alternative E1 test lab added on 2026-10-24

- Dedicated workflow: `.github/workflows/milestone-1-e1-lab.yml`.
- Portable local entry point: `bash tools/milestone1-e1-lab.sh`.
- Lab guide: `docs/verification/milestone-1-e1-test-lab.md`.
- The lab executes the entire `one-core` stdlib integration test target, `one-core` library unit tests, workspace tests, workspace Clippy with warnings denied, and `tools/check_contract.py`; it prints the exact tested SHA and toolchain.
- The dedicated hosted workflow was actually triggered on candidate SHA `266d51340b78f15bb52b742fd2e4c80d7353dccc`: https://github.com/panglimanetizen01/one/actions/runs/40100991584. It hit the same infrastructure symptom (`runner_id: 0`, empty runner name, `steps: []`), so it yielded no test result.
- Attempted local execution is not possible in this session: the container has no Rust toolchain and cannot resolve `github.com` for a clone. No local pass is claimed.
- To use a separate functioning runner pool, updated the independent verifier repository's `verification/CANONICAL_COMMIT.txt` to the current candidate revision. Pin update commit: https://github.com/panglimanetizen01/one-windows-verification/commit/29bfea16e6e630fffa9100d6e44c3ce36551013e. Its workflows did not start automatically after this API-driven commit, so trigger **Run workflow** manually on the verifier branch for the Linux distro, focused standard-library, and cross-platform workflows. Those new runs must show the exact candidate SHA before their results count.
- Current candidate before this checkpoint refresh: `5aca770bb69f266577be8a2514635de09efa15be`. The checkpoint refresh itself will be the new candidate SHA; update the verifier pin to that resulting SHA before running its tests.

**Gate remains NOT PASSED** until the dedicated or independent lab completes every required check successfully against one exact candidate SHA.

