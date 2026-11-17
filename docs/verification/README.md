# Verification artifacts and CI workflows

GitHub metadata currently reports the canonical ONE repository as **PUBLIC**. No visibility change was made during this documentation work. Treat public source and build logs as a disclosure boundary; never place credentials or private data in tracked files or logs.

## Historical CI ownership snapshot — 2026-11-16

- Canonical source: `panglimanetizen01/one`; candidate PR #95 branch `ci/circleci-migration-20261113`, exact SHA `07d8cbf7422b8340f8efda3c2f9d4425f85b7537`.
- Canonical Linux product CI: CircleCI `one-linux`, terminal SUCCESS on this exact SHA (workflow `0e03e28b-dfb9-4198-b897-3ce5d16dc2d8`, job `b2f536a1-6c87-49b5-9a3f-2db2f2908595`, status ID `—`).
- Independent Windows host verification: [run #28](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43865524749), terminal SUCCESS on Windows Server 2025 AMD64; 32/32 steps, native exit 0, output `Budi`; artifact and manifest hashes independently checked.
- Both results bind to the same exact canonical source SHA. Linux-host MinGW cross-compilation is not Windows-host execution evidence.
- PR #95 remains OPEN / DRAFT / UNMERGED; canonical `main` remains unchanged at `d35db95153f6ddd79885494c72f5aee16c899751`. The migration is not live on main until separately authorized merge.
- The opt-in Linux release-check has not run. No merge, tag, or release is authorized by this CI checkpoint.




## Current PR #95 migration candidate — 2026-11-17

- PR #95 remains OPEN / DRAFT / UNMERGED on branch `ci/circleci-migration-20261113`. Latest branch head is `de71a54762b72f3347002cbe848467dc2d9b1f7a` (documentation-only evidence correction); the latest tested product/config candidate remains `e50748a1b25922d9090eba1f5fb90548d7c7d003`.
- CircleCI `ci/circleci: one-linux` was terminal SUCCESS on exact source/config SHA `e50748a...` (workflow `dc6f3660-328f-4016-8c01-72c3565d0321`, job `64a34e41-9759-4a5c-8e50-b77296730a54`).
- Independent Windows native run #31 on exact `e50748a...` failed before compilation because the source did not track `Cargo.lock` while the verifier used `--locked`; no Windows artifact/runtime PASS exists for that SHA. Earlier run #28 PASS on `07d8cbf...` is historical and cannot replace run #31 on `e50748a...`.
- Lockfile-only candidate `a10a69afdf8675e703f38f1620abc935d8361897` adds only `Cargo.lock`. GitHub Actions One CI run #43998641019 passed Linux+Windows on that SHA, and independent Windows native runtime run #32 passed (exit 0, output `Budi`). This is not CircleCI evidence and is not the same SHA as PR #95's CircleCI PASS; do not combine into same-SHA parity.
- CircleCI on docs-only PR heads `2f1c57e95c0d51725aa8aace3c00583b681025ad` and `de71a54762b72f3347002cbe848467dc2d9b1f7a` both reported FAILURE. Latest workflow `ca8b9376-9ded-4ccc-9540-7ab9f9011c07`, job `aba2c931-3fb7-43c7-b152-67eaebf2dc7c`. Logs are not accessible through the current integration, so the failing step/root cause is NOT VERIFIED. Latest PR #95 branch is not green.
- PR #102 is a separate lockfile and Windows subprocess-containment change: latest source HEAD `87c80e2f21ebdc7dd3e4bd02744b30c4efd1a068`; exact-SHA GitHub Actions run #44008584799 remains IN_PROGRESS (Ubuntu and Windows jobs at toolchain setup at last check). This is not release evidence.
- Integration requirement: after authorized PR sequencing brings the lockfile and CircleCI config together, add `--locked` to every CircleCI Cargo build/Clippy/test command and require it in `tools/check_contract.py` when `Cargo.lock` is tracked. Current PR #95 config does not yet use `--locked`.
- CircleCI is not active on `main` while PR #95 remains unmerged. Optional `release-check` remains unproven; no merge/release is authorized.


## Independent verifier security hardening — 2026-11-17

PR #39, #40 and #42 now remove the unnecessary `CANONICAL_READ_TOKEN` secret from exact-source checkout steps because the canonical ONE repository is public. All harness and source `actions/checkout` steps set `persist-credentials: false`, preventing checkout credentials from remaining in the worktree while build/test code runs. The PRs remain OPEN / DRAFT / UNMERGED; these security changes still require exact workflow execution and review.

## Canonical evidence locations

- CircleCI job logs/artifacts: Linux product and Linux release jobs for the exact ONE source SHA.
- Independent Windows logs/artifacts: exact source pin, verifier SHA, native executable, and runtime manifest in `one-windows-verification`.
- Historical cross-platform evidence remains recorded in `one-v0.1-verification.md`, `one-v0.1-evidence-matrix.md`, and `native-backends-2026-10-19.md`.

## Evidence rules

- Source presence and workflow configuration are not execution evidence.
- A failed or unstarted job does not establish a source-code failure without executable steps.
- Every platform result is bound to its exact canonical source SHA and stated host boundary.
- Windows PASS requires terminal-success Windows workflow plus independent manifest and binary-hash verification.
- A CircleCI Linux PASS does not imply Windows PASS; a Windows verifier PASS does not imply CircleCI Linux PASS.


## Completion ledger and release gate — 2026-11-17

The fixed-weight completion ledger and Definition of Done are restored in the canonical documentation surface by [PR #103](https://github.com/panglimanetizen01/one/pull/103):

- [Fixed-weight completion ledger](https://github.com/panglimanetizen01/one/blob/docs/restore-completion-ledger-dod-20261117/docs/verification/one-completion-ledger.md) — F1–F6, C1–C6, and E1–E6; weights remain fixed at 100 points.
- [Definition of Done](https://github.com/panglimanetizen01/one/blob/docs/restore-completion-ledger-dod-20261117/docs/verification/DEFINITION_OF_DONE.md) — release acceptance checklist.

Current disposition is deliberately fail-closed:
- Historical implementation inventory: 92/100 provisional; current exact-SHA verified points: 0/100.
- Historical DoD state: 3 checked / 10 unchecked; the checked items require revalidation against a frozen final source SHA.
- Lockfile candidate `a10a69afdf8675e703f38f1620abc935d8361897` passed GitHub Actions Ubuntu+Windows workspace CI and independent Windows native PE32+ runtime (exit 0, output `Budi`). This is prior-SHA evidence and does not verify the newer process-hardening HEAD.
- PR #102 latest HEAD: `4f9b8b08a550b368b8588855239c35948ad53990`; exact-SHA run #44009296614 is IN_PROGRESS (Windows cross-compiler installation; Ubuntu Build). Current HEAD NOT VERIFIED. The new implementation explicitly calls `TerminateJobObject` when members remain after the direct process exits, then waits for job accounting to reach zero with a bounded five-second deadline; `KILL_ON_JOB_CLOSE` remains the fallback.
- Previous SHA `87c80e2f21ebdc7dd3e4bd02744b30c4efd1a068`, run #44008584799: Ubuntu SUCCESS; Windows build/Clippy SUCCESS; tests 119 passed / 1 failed. Descendant PID `8208` survived (`WAIT_TIMEOUT=258`) even though the exact invocation-job membership assertion passed.
- Previous SHA `b088bc49839ea093f2e1fa234baa23dcfe05465d`, run #44008055855: Ubuntu SUCCESS; Windows build failed before tests due windows-sys API import paths; corrected on `87c80e2...`.
- Previous SHA `54a68a7673247c0fe23aba24c5e86f7f77372c60`, run #44007422002: Ubuntu SUCCESS; Windows build/Clippy SUCCESS; tests 119 passed / 1 failed because descendant PID `9876` survived; exact invocation-job membership had not yet been probed.
- Earlier source SHA `a472b3bd9e410f1d42f37f31cfcb4e5c11f399a8`, run #44002493159: Ubuntu SUCCESS; Windows FAILURE (119 passed / 1 failed), descendant PID `8864` survived (`WAIT_TIMEOUT=258`).
- CircleCI migration PR #95 remains unmerged; `.circleci/config.yml` is absent from `main`. CircleCI ERROR contexts on product-fix branches are not proof that source tests failed.
- CircleCI status on PR #95 exact HEAD `e50748a...` is SUCCESS (`ci/circleci: one-linux`, workflow `dc6f3660-328f-4016-8c01-72c3565d0321`, job `64a34e41-9759-4a5c-8e50-b77296730a54`). `.circleci/config.yml` is absent from `main` and PR #102 because PR #95 remains unmerged; generic CircleCI Pipeline ERROR contexts on those refs are consistent with missing migration config, not product-test failure. PR #102 adds `Cargo.lock` and expects `--locked`; the PR #95 config does not yet pass `--locked`. After authorized integration, update all CircleCI Cargo commands and make the contract checker require lockfile enforcement.
- No final candidate SHA is frozen. Release gate remains **NOT PASSED**. No merge or release is authorized by this checkpoint.



### Health scanner verifier ownership and workflow correction — 2026-11-17

- The scanner workflow is not stale: it belongs to independent verifier PR #39 and is pinned through `HEALTH_SCANNER_SOURCE_COMMIT.txt` to exact ONE PR #91 source SHA `9b38c246f0ef18f6a39a79ff2be81c4ba06b63db`. At that SHA, `tools/one_health.py`, `tools/test_one_health.py`, `tools/test_collect_ci_health.py`, and `tools/test_health_quality.py` are present. The same scanner path is absent from current ONE `main` and the PR #95 code/config candidate `e50748a...`; that absence is consistent with PR #91 remaining OPEN / DRAFT / UNMERGED and the scanner being historical/developer-side tooling, not a current product dependency.
- PR #39's dedicated pin separates this scanner verification from the shared `CANONICAL_COMMIT.txt` used by other verifier workflows. Do not delete or redirect the workflow merely because current ONE main lacks the scanner.
- Differential review found duplicate YAML mapping keys: `persist-credentials: false` appeared twice in each canonical-source checkout block. Removed the duplicate lines in PR #39 commit `6e2ce12eeb36ec2ffa0db38c9fce7e98faf2fbdd`; the workflow now has one credential-persistence setting per each of its three checkout steps. This is a static correction only; exact workflow execution remains NOT VERIFIED because the workflow triggers on pushes to the verifier base branch or manual dispatch, not pushes to the draft PR branch.
- PR #39 remains OPEN / DRAFT / UNMERGED. Do not merge it just to trigger a run. The health scanner remains outside current ONE main and must not be reintroduced into the product for verifier convenience.


### PR #102 — latest Windows Clippy correction — 2026-11-17

- PR #102 source `eaa7cf77bd9971f1fd9b5205cef975e6acad270f`, run #44010448799: Ubuntu SUCCESS; Windows FAILURE. Windows contract check and build passed; strict Clippy failed on redundant `.into()` at `crates/one-core/src/interop.rs:894`. Windows tests and all later gates were skipped; stable process-handle containment remains NOT VERIFIED.
- Removed only the redundant conversion in commit `da740c4f7f9c72e9b76e573d23651334272582c5` on `fix/commit-cargo-lockfile-20261117`. The workflow-run lookup returned no run for this commit at the time of the checkpoint. Do not reuse run #44010448799 as evidence for the new SHA; poll for its exact-SHA run and inspect the retained process-handle wait result.


### PR #102 — retained process object survives invocation — 2026-11-17

- Exact run #44012805081 on source `da740c4f7f9c72e9b76e573d23651334272582c5`: Ubuntu SUCCESS; Windows build SUCCESS; Windows Clippy SUCCESS; Windows tests 119 passed / 1 failed. The stable descendant handle returned `WAIT_TIMEOUT=258` after `run_process` completed. This proves the original descendant process object remained active; PID reuse is no longer a plausible explanation.
- The test cleanup terminates the test-owned ping process through the same retained handle after capturing the failure.
- Diagnostic follow-up commit `efc5422845927c2a46e08e61aeb6a497e2f9980b` adds `GetProcessId` and `GetExitCodeProcess` observations on that retained handle to validate identity and report its state.
- Exact-SHA run #44018063886 on `efc5422...` is IN_PROGRESS. Do not call containment fixed or verified; inspect the diagnostic failure before changing the Job Object lifecycle.

## Live CircleCI migration checkpoint — 2026-11-17 12:51 WIB

This addendum supersedes older PR #95 current-status statements above where they conflict; all earlier run records remain historical.

- PR #95 branch `ci/circleci-migration-20261113` current HEAD: `cef1f41c97585b6e25bf91af6e8b4281efe299c4`; PR remains OPEN / DRAFT / UNMERGED.
- Exact current HEAD commit status `ci/circleci: one-linux` is SUCCESS. CircleCI workflow `930378d8-49bb-416e-a42c-63e1797b991c`, job `0e2642b0-bd94-4892-baca-df5b6c8fa4ed`, timestamp 2026-11-17 05:49:29 UTC. The status is terminal and exact-SHA-bound; step-level CircleCI logs are not exposed by the current connected integration.
- Independent Windows verifier PR #42 remains pinned to `9b233dfe75da8c0752c203af3711bf3da319dc02`, not the current PR #95 source SHA. No independent Windows run for `cef1f41...` is evidenced; classify that gate NOT VERIFIED.
- PR #102 current HEAD `7e10e758d7e1ca70f450bfbb6cf397d805500e53` has exact-SHA GitHub Actions run #44019833034 terminal SUCCESS on Ubuntu job `133826415064` and Windows job `133826414765`. This is a separate source branch and must not be substituted for PR #95's Windows verifier gate.
- Main remains PUBLIC and unchanged at `d35db95153f6ddd79885494c72f5aee16c899751`; PR #95 is not merged. CircleCI migration is not yet active on main; no merge or release has occurred.


## Latest exact-SHA status — 2026-11-17

- PR #102 source SHA `7e10e758d7e1ca70f450bfbb6cf397d805500e53`: GitHub Actions One CI run #44019833034 terminal SUCCESS on Ubuntu job `133826415064` and Windows job `133826414765`; the Windows descendant Job Object regression passed.
- PR #95 source SHA `cef1f41c97585b6e25bf91af6e8b4281efe299c4`: CircleCI `ci/circleci: one-linux` terminal SUCCESS, workflow `930378d8-49bb-416e-a42c-63e1797b991c`, job `0e2642b0-bd94-4892-baca-df5b6c8fa4ed`. This status is exact-SHA-bound; step-level logs are not exposed by the current integration.
- PR #42 now pins `cef1f41...` in commit `f5113c084fa978f275bc8426a4ca1e4d74376549`; docs/pin branch HEAD is `44b36a1894382cc2508edf0166f60b06c9434d7a`. No independent Windows run exists for this exact source SHA. Same-SHA Windows verification remains NOT VERIFIED because the PR branch update does not trigger the base `verification` workflow and no workflow-dispatch action is available in the connected integration.
- PR #95, #97, #102, #103 and verifier PR #42 remain OPEN / DRAFT / UNMERGED. CircleCI is not active on main until the migration PR is separately authorized and merged. No merge/release is authorized.
- Health scanner workflow provenance: verifier PR #39 targets ONE PR #91 SHA `9b38c246f0ef18f6a39a79ff2be81c4ba06b63db`; verifier PR #40 targets ONE PR #89 SHA `242b88e4c8b6f2c2c2039f2bc95ebeab20ae96de`. Both exact source revisions contain the scanner files; current ONE main does not. These are branch-scoped historical/developer-tooling verifiers, not current main product dependencies. Preserve them unless their source PRs are explicitly superseded.
- E6 remains NOT IMPLEMENTED / final release gate NOT PASSED. No completion points are promoted by these CI results alone.


## Latest cross-PR verification checkpoint — 2026-11-17

- Main remains PUBLIC at exact SHA `d35db95153f6ddd79885494c72f5aee16c899751`; no merge/release occurred.
- PR #102 exact source `7e10e758d7e1ca70f450bfbb6cf397d805500e53`: GitHub Actions run #44019833034 terminal SUCCESS on Windows job `133826414765` and Ubuntu job `133826415064`. The Windows Job Object descendant-termination regression passed. This is not CircleCI migration evidence for PR #95.
- PR #95 current source `5ba239ac3964dc147d1568dac2b819459f31b463` is a documentation-only descendant. CircleCI `ci/circleci: one-linux` is PENDING (workflow `5ec5c4c5-e6ab-4518-8d46-1178ba75333f`, job `59fcdba6-1647-4352-ad02-1add7eebae22`). Previous exact-SHA SUCCESS on `cef1f41...` is not automatically transferred.
- Independent verifier PR #42 pin is synchronized to `5ba239...` (pin commit `764d197864b19181ba7a814bf93c56c596bf8600`; current verifier PR head `6fd76ed85189923a1c0f661638f8077f290955a1`). No Windows run for `5ba239...` is observed; status NOT VERIFIED. Historical run #32 applies only to `a10a69...`.
- PR #97 remains OPEN / DRAFT / UNMERGED. PR #103 ledger and DoD were refreshed in commits `5b68887d4de4644c7e730171af2ff60c64ad0bd6` and `0930e5ad3aee496e88f1a76adf4cafaface3e440` respectively; its PR remains OPEN / DRAFT / UNMERGED.
- The optional CircleCI `release-check` with `run_release_checks=true` is NOT VERIFIED. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; release acceptance remains NOT PASSED.
- No merge, release, default-branch mutation, or source-language behavior change occurred.


## Latest locked-dependency CI checkpoint — 2026-11-17

- PR #95 current source SHA: `d42689718870236694f6cab841f8b7be1afccfcc`. CircleCI `ci/circleci: one-linux` is PENDING (workflow `6af2386b-dd8d-47c8-b414-4237e92130fd`, job `6cf4f8c5-7b2d-4876-a448-89dc0c596c68`). No status from an ancestor SHA is transferred.
- Offline scripts `tools/verify_offline.sh` and `tools/verify_offline.ps1` now use `--locked` for every Cargo build, Clippy, test, and run command; `tools/check_contract.py` rejects any omission.
- Independent verifier PR #42 pin matches `d426897...`; verifier workflow enforces `--locked` on all Cargo commands and includes a self-check for the rule. Verifier docs/current PR head: `9475956ae0ddccb969a5d14d1c9e803c7e44d097`.
- Windows exact-SHA run for `d426897...` is NOT VERIFIED. Optional CircleCI `release-check` remains NOT VERIFIED.
- No implementation score, verified score, DoD checkbox, or E6 status is promoted. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; release acceptance remains NOT PASSED.


## Superseding exact-SHA evidence boundary — 2026-11-17

- PR #95 code/tooling candidate `d42689718870236694f6cab841f8b7be1afccfcc` has CircleCI `ci/circleci: one-linux` terminal SUCCESS (workflow `6af2386b-dd8d-47c8-b414-4237e92130fd`, job `6cf4f8c5-7b2d-4876-a448-89dc0c596c68`). It includes `--locked` enforcement in both offline scripts and the source contract checker.
- Current PR #95 HEAD `c207fb15bc19889a8c8e60923581ef1b1f7a6374` is a documentation-only descendant and its CircleCI status is PENDING (workflow `15aa40bb-0225-4ad6-a2ea-fdaa0f364b0b`, job `cdcc88bd-abdc-43b0-9885-1e696bd07ac8`). Do not transfer the prior SUCCESS automatically.
- Independent verifier PR #42 pin matches `c207fb...` (pin commit `8db417749753009ad5dbc69f8b4d2bb00c5d1642`; verifier docs commit/current head `b45f9a259ea072168a1efc3300300438e78fbbf6`). No matching Windows run is observed; status NOT VERIFIED. The verifier workflow now uses `--locked` for all Cargo commands and self-checks the invariant.
- PR #102 exact source `7e10e758d7e1ca70f450bfbb6cf397d805500e53` remains GitHub Actions PASS on Ubuntu and Windows (run #44019833034); the Windows descendant Job Object regression passed.
- Optional CircleCI `release-check` with `run_release_checks=true` remains NOT VERIFIED. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; no score or DoD checkbox is promoted.


## Current PR #95 docs-only descendant and Windows rerun — 2026-11-17

- Previous PR #95 source `c207fb15bc19889a8c8e60923581ef1b1f7a6374` has CircleCI Linux terminal SUCCESS and independent Windows run #34 terminal SUCCESS (run `44030597595`, verifier SHA `3f8e9b662805a89fd5145e8691a221218b098e11`, job `133861337521`). Artifact ID `14305081962`; ZIP SHA-256 `7cbbf251efe2f56ba24965a76cd420ccddae8e84e8f0dcd727b09943014f3b8f`; executable SHA-256 `0c79f06f37f793061884acf43c9ba419bb0fc4ccf48c4a487f511e462cf55af5`; manifest SHA-256 `7ac05a71f1f5e5149f1dd721d2ca9c610bfe7504acb6b164938a51255c60b1d6`; native exit 0 / output `Budi`.
- Current PR #95 HEAD `048f87f5e9c14fdda33c7fa7a3d6dd77f469b9d4` is a docs-only descendant recording the run #34 evidence. CircleCI on this exact SHA is PENDING (workflow `875e39ae-399d-4803-81f7-407c7dc3f0d3`, job `40fbbc41-4a32-4796-b817-d144f8186e46`).
- Independent verifier PR #42 pin matches `048f87...`; Windows run #35 / ID `44031205160` is IN_PROGRESS with verifier SHA `62f401db17e5bc2ff3ae4c53e02cc721ed1d191e`. Run #34 remains bound only to `c207fb...`.
- Optional CircleCI `release-check` remains NOT VERIFIED. E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; no score/DoD promotion or release acceptance.


## Latest PR #95 exact-SHA candidate — 2026-11-17

- Previous source `048f87f5e9c14fdda33c7fa7a3d6dd77f469b9d4` has CircleCI Linux PASS and independent Windows run #35 PASS; artifact ID `14304459543`, ZIP SHA-256 `0238718f261ba9f9840d26eb243ce3196931f1f9927c98e438b3077f7cf1a58b`, executable SHA-256 `7d9eaa745e561bafe4203404c9d6b0601b584ff93eec46a379e5fa1d0e9afd0a`, manifest SHA-256 `eace9930574e939e94a3f5d18c75901ffe883a43852ff43c281b236ece22543e`; exit 0 / `Budi`.
- Current PR #95 HEAD `54bf0973f59c2705decd91a8457cf08d6bbb7f8e` is a docs-only descendant recording run #35. CircleCI is PENDING (workflow `4e04be28-3664-4842-a4c8-6844d1d45817`, job `cc67ef7a-1df1-4a31-a694-05725f702652`).
- Verifier PR #42 pin matches `54bf097...`; Windows run #36 / `44031839506` is IN_PROGRESS with verifier SHA `c40752bd134c83a671e928a0a2aab25953e4ad6e`. Do not transfer run #35 to this descendant.
- Optional CircleCI release-check remains NOT VERIFIED. E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED.


## Differential open-PR verification checkpoint — 2026-11-17

### PR #95 / PR #42 — latest exact-SHA PASS

- PR #95 source `54bf0973f59c2705decd91a8457cf08d6bbb7f8e`: CircleCI Linux SUCCESS (workflow `4e04be28-3664-4842-a4c8-6844d1d45817`, job `cc67ef7a-1df1-4a31-a694-05725f702652`) and independent Windows run #36 / `44031839506` SUCCESS (verifier `c40752bd134c83a671e928a0a2aab25953e4ad6e`, job `133865371299`). Artifact `14304669534`; ZIP SHA-256 `b281dc52a781063fdda0570e357d5f1cee30a86fc6b77fa4c287b52514c8adb2`; executable SHA-256 `515bb17d95223108934dbedce435d93ab823f231a3d6ce1be95ab2c532e51dd4`; manifest SHA-256 `73472141307457975dcc10dc9e923afa5be7e6d1b83e796cba05168de02f86f4`; runtime exit 0 / `Budi`.
- The optional CircleCI `release-check` remains NOT VERIFIED. PR #95 and verifier PR #42 remain OPEN / DRAFT / UNMERGED.

### PR #98, #99, #100 — exact-SHA legacy GHA PASS

- PR #98 SHA `a83321bc41b3d0d28378841e29a6169abed8ac15`: GHA run #2018 / `43923604400`, Ubuntu job `133497053339` and Windows job `133497052786`, both SUCCESS including build, Clippy, tests and vertical slices.
- PR #99 SHA `8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9`: GHA run #2019 / `43923948205`, Ubuntu job `133498292797` and Windows job `133498293201`, both SUCCESS.
- PR #100 SHA `6375356c9c275ace5fd35cf4a81799a9546e1e1d`: GHA run #2020 / `43924465030`, Ubuntu job `133500117683` and Windows job `133500117243`, both SUCCESS.
- Their CircleCI Pipeline contexts are ERROR because main has no `.circleci/config.yml` before PR #95 migration. This is not evidence of Rust test failure. Their GHA evidence remains exact-SHA historical PASS; reassess CircleCI after the authorized migration.

### PR #101 — compile failure fixed; new CI running

- Old SHA `bbcc86876553ffd9a97af12c11bf99bdabae2792`, GHA run #2021 / `43924632960`: Ubuntu and Windows both failed during Build, before Clippy/tests. Root cause: two new filesystem denial/approval regression tests were appended outside the closing brace of `#[cfg(test)] mod tests`, so `SemanticCapabilityId`, `ExecutionScope`, and `EffectClass` were out of scope.
- Fixed in commit `cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f` by moving the premature module-closing brace after the new tests. No test was removed or weakened; production behavior unchanged.
- New GHA run #2138 / `44032422058` is IN_PROGRESS on exact SHA `cdf7bd...`; Ubuntu job `133867259381` and Windows job `133867259061`. No PASS claim until both finish.

### PR #96 — superseded CircleCI Windows patch

- PR #96 only fixes MSYS2 keyring initialization in the historical CircleCI Windows executor path. Current PR #95 architecture intentionally uses CircleCI Linux plus the independent Windows verifier; the current CircleCI config has no Windows executor. Merging #96 would reintroduce the superseded Windows job. Its disposition is historical/superseded; leave open/unmerged, preserve original failure evidence.

### Completion controls

- PR #102 remains GHA PASS on exact source `7e10e758d7e1ca70f450bfbb6cf397d805500e53`, including Windows Job Object descendant termination.
- E6 remains `NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED`; no score or DoD checkbox is promoted. Optional release-check, clean installation/use acceptance and release authorization remain outstanding.
- Main remains PUBLIC and unchanged at `d35db95153f6ddd79885494c72f5aee16c899751`. No merge or release.


## Product-PR reconciliation — terminal update — 2026-11-17

### PR #101 is now PASS

- Exact source `cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f`; GHA run #2138 / `44032422058` terminal SUCCESS on Ubuntu job `133867259381` and Windows job `133867259061`.
- Root cause was the two filesystem-inheritance tests being outside the existing `#[cfg(test)] mod tests`; commit `cdf7bd5...` moved the module-closing brace after the tests. Build, Clippy, full tests and all listed vertical slices passed on both platforms. No tests weakened/deleted.
- PR #101 remains OPEN / DRAFT / UNMERGED.

### PR #98–#100 historical exact-SHA PASS

- #98 `a83321bc41b3d0d28378841e29a6169abed8ac15`: GHA #2018 / `43923604400`, Ubuntu `133497053339` and Windows `133497052786` SUCCESS.
- #99 `8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9`: GHA #2019 / `43923948205`, Ubuntu `133498292797` and Windows `133498293201` SUCCESS.
- #100 `6375356c9c275ace5fd35cf4a81799a9546e1e1d`: GHA #2020 / `43924465030`, Ubuntu `133500117683` and Windows `133500117243` SUCCESS.
- Their CircleCI Pipeline errors reflect that main has not yet received the CircleCI config; GHA results remain valid historical evidence for those exact SHAs. Recheck CircleCI only when the migration is authorized and active.

### Additional open-PR inventory and disposition

- #96 MSYS2 keyring fix targets the retired CircleCI Windows executor path; current architecture is CircleCI Linux + independent Windows verifier. Superseded; keep open/unmerged, do not reintroduce the old Windows executor.
- #80 is a 153-file milestone branch, head `d75b2846465ba53281198693750f2e6ba39e2e33`, with no exact-head executable CI evidence. It includes nine legacy GHA workflows, health scanner/developer tooling, experimental human-vNext code and broad product/runtime changes. Its own historical ledger records E1/E2/E4/E6 gaps and a NOT PASSED gate. It is not safe to merge as-is; decompose and compare product code against accepted v0.1 contracts before promotion.
- #94 is based on #80 and contains repository-health scanner/tooling; out of current product scope. Do not reintroduce scanner files into ONE.
- #92 main was scanned: no paths match `ci-local`/`ci_local`, and code search returned no content matches. However PR #92 is not a pure cleanup: its diff also changes product code and re-adds `.github/workflows/ci.yml`. Treat the extraction cleanup as already satisfied on main; do not merge #92 as a cleanup PR or reintroduce GitHub Actions. Review its product changes separately if still needed.
- #93 and #84 are older exact-SHA evidence documentation corrections, now superseded by PR #103's completion ledger/DoD and current evidence index; preserve history, do not merge stale snapshots.
- #85 adds legacy GitHub Actions user-documentation checks; current CircleCI source contract and docs checker supersede the old workflow path. Keep unmerged unless a missing check is proven.
- #83 remains a WIP SEC-05 process-launch experiment on a non-main base; no current exact-SHA CI evidence. PR #102 is a separate, main-targeted Windows Job Object/resource-containment fix with its own PASS evidence; do not conflate the two.
- #77 is a language-surface audit/proposal, not normative grammar acceptance. #78 is an older human-vNext implementation branch; PR #80 contains a broader milestone snapshot but is unverified and vNext remains non-normative until separately accepted.
- #76 HTTP IPv6 authority parsing remains an open non-draft PR on a stale base (5 ahead / 3 behind main). Its only GHA run #39606316737 ended with zero steps on both platforms; status NOT VERIFIED, not a code failure. It has parser regression tests, but requires a current-base update and executable verification before any merge decision. PR #80 contains a more extensive parser implementation, but that branch is also unverified; do not treat it as evidence.

### Current completion guard

- PR #95 source `54bf097...`: CircleCI Linux + independent Windows run #36 PASS exact SHA; PR #102 exact SHA `7e10e...` GHA PASS; PR #101 exact SHA `cdf7bd5...` GHA PASS; PR #98–#100 exact SHA GHA PASS.
- E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED. Optional CircleCI release-check, clean installation/use acceptance and final release authorization remain outstanding.
- No PR merge, branch deletion, release, visibility change, secret change, or history rewrite performed.


## Source installation and release CLI smoke evidence — 2026-11-17

- PR #102 exact source SHA `cb2748c5bb7143d62818068bd2a7128520ae2c69`; GitHub Actions run #44043930974 terminal SUCCESS on Ubuntu job `133904876367` and Windows Server 2025 job `133904877181`.
- Both platforms built the release CLI with `cargo build --release --locked -p one-cli --bin one` and passed their platform-specific smoke test: version output `one 0.1.0`, execution of a temporary One source program, expected output `Budi`.
- `docs/panduan-pengguna/instalasi.md` on the PR #102 candidate now documents source build prerequisites (Rust/Cargo 1.97+), clone/build commands, Linux `target/release/one` and Windows `target/release/one.exe`, version verification, and first program. It explicitly does not claim a prebuilt installer or official binary release; `.oneir`/`.onepkg` are not installers.
- All Cargo commands in the PR #102 GitHub Actions workflow now use `--locked`.
- CircleCI's separate error context on this main-based PR #102 branch is not the result of the PR #102 GitHub Actions workflow and does not negate the exact-SHA Ubuntu/Windows PASS. CircleCI migration remains a separate PR #95 candidate.
- This is implementation/verification evidence only; no release acceptance or DoD completion is inferred.


## Latest source-install and documentation-check gates — 2026-11-17

- PR #102 current exact source SHA `5ee645507199bdd94955099f6633aa67bb697ee5`; GHA run #44044795819 terminal SUCCESS on Ubuntu job `133907717583` and Windows Server 2025 job `133907717844`.
- Both platforms passed user documentation consistency and checker unit tests; both built the release CLI with `cargo build --release --locked -p one-cli --bin one`; both passed version and sample execution smoke tests (`one 0.1.0`, output `Budi`). Full workflow jobs were terminal SUCCESS.
- PR #102 installation guide now documents a truthful source-build path (Rust/Cargo 1.97+, clone, locked release build, platform binary path, version check, and first program); no installer or official binary release is claimed.
- PR #102 CI workflow includes user-doc consistency tests and Linux/Windows release CLI smoke. All Cargo build/Clippy/test commands use `--locked`.
- This is exact-SHA implementation/verification evidence only. It does not promote E6, final release acceptance, or DoD checkboxes.


## Latest PR #95 exact-SHA verification checkpoint — 2026-11-17

- ONE source `876b83170af2e77adcc43e408901e99ea37f6b0f` passed full independent Windows Server 2025 verifier run #38 / `44045612644`, verifier SHA `564fd1e0853dd424eb8d451678faaa77aaa59d11`, job `133910422388`. All 31 primary steps passed, including locked Cargo guard, contract/docs checks, workspace build/Clippy/tests, release CLI, product vertical slices, real-network/offline, Windows native backend, PE32+ inspection, native execution, manifest and artifact upload.
- Run #38 artifact ID `14310959518`, ZIP SHA-256 `8e10e35d04709eee8429f143ef8bde0ca358941b51a4785f49e934e94f9f163c`; native EXE SHA `11ed652bd05e09145c8ca304762b72391de952eb2f441344dd259efda38221c7`; CLI EXE SHA `61a5e8166831153c2b98f69e09ceeda84759cee53e8d060f406c5d8a7ea12dba`; manifest SHA `9844c92e89ed1ac805adbf757c8a111ba66163acf265f00f428ba587f090ba23`. ZIP readback, manifest hashes and PE32+ AMD64 headers independently matched.
- PR #95 docs-only descendant/current HEAD is `ed2382eed9022a8c67a13d72a5de6a4d9134f631`; verifier PR #42 pin matches it. New exact-source verifier runs #39 / `44046670869` (IN_PROGRESS) and #40 / `44046678525` (PENDING) were triggered by successive verifier branch pushes. Do not transfer run #38 to this descendant.
- CircleCI Pipeline #558 reported pipeline-level ERROR on the previous candidate without exposing a `one-linux` job context/log. This is not enough to diagnose a source test failure. Current integrations do not expose authenticated CircleCI pipeline details; optional `release-check` remains NOT VERIFIED.
- PR #95/#42/#97/#103 remain OPEN / DRAFT / UNMERGED. Main remains unchanged. E6 and release acceptance remain NOT PASSED.


## Active product remediation PR exact-SHA CI matrix — 2026-11-17

The following open product PRs were rechecked against live GitHub Actions run metadata. Each listed run is terminal SUCCESS on the exact current PR source SHA, with both Windows Server 2025 and Ubuntu jobs SUCCESS. These are implementation/test evidence only; the PRs remain open/draft/unmerged and do not independently complete release acceptance.

| PR | Exact source SHA | GitHub Actions run | Windows job | Ubuntu job | Scope |
|---|---|---:|---:|---:|---|
| #98 | `a83321bc41b3d0d28378841e29a6169abed8ac15` | `43923604400` | `133497052786` PASS | `133497053339` PASS | Validate project manifest in `one test`; reject unsafe package filename components and path traversal |
| #99 | `8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9` | `43923948205` | `133498293201` PASS | `133498292797` PASS | Open existing SQLite database files read-only; never create missing DB on query; preserve `:memory:` fixture behavior |
| #100 | `6375356c9c275ace5fd35cf4a81799a9546e1e1d` | `43924465030` | `133500117243` PASS | `133500117683` PASS | Restore lexical scope frames after nested block errors and failed event dispatch |
| #101 | `cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f` | `44032422058` | `133867259061` PASS | `133867259381` PASS | Apply coarse Filesystem parent coverage consistently to grants, scopes, denials and approvals; explicit parent denial precedes inherited grants |

- All four changes are confined to CLI/runtime/governance contract behavior and tests; no changes to the language grammar or beginner-facing syntax are claimed.
- CircleCI pipeline statuses for these PR branches are separately ERROR (pipeline numbers #369, #373, #377 and #550 respectively) while GitHub Actions is green. These are pipeline-level CircleCI statuses and are not enough to claim Rust test failures. The current PR #95 CircleCI migration candidate is separately being diagnosed; do not conflate CI providers.
- PR #96 (`25ea36051eb6fb4cefc40c1867f5a6fc3e777bca`) modifies the historical CircleCI Windows/MSYS2 setup, but it is superseded by the current PR #95 design (CircleCI Linux + independent Windows verifier). Its exact status shows CircleCI Linux PASS and Windows FAIL on the old workflow; do not merge/reintroduce the retired CircleCI Windows job. Preserve as historical evidence.
- These PRs remain implementation candidates until integrated through authorized review. No DoD checkbox, E6, verified score, or release acceptance is promoted solely by CI PASS.


### Superseding PR #95 / Windows evidence correction — 2026-11-17
- PR #95 current source SHA `993f76351c7a119bea2e66f7d9e5bca07f392fbc`; PR #42 verifier pin matches it. New Windows run #41 / `44047540578` is PENDING at last poll.
- Previous exact source `ed2382...` passed full Windows run #39 / `44046670869`, artifact `14312311019`, ZIP SHA-256 `aa56f8275042969b18c12283eb152e0572b8970159e3df7db62e9aad9f109da1`; native and CLI output `Budi`, exit 0; CLI version `one 0.1.0`.
- Correction: CircleCI Pipeline #558 belongs to PR #102 SHA `5ee645...`, not PR #95. PR #95 SHA `ed2382...` had no CircleCI status context visible in GitHub; current SHA `993f763...` likewise has no CircleCI status visible at last read. PR #102 GHA PASS and generic CircleCI ERROR are separate; root cause NOT VERIFIED. Optional release-check NOT VERIFIED.
- PR #97 remains OPEN / DRAFT / UNMERGED; no merge/release.


## PR #102 process-spec input validation review — 2026-11-17

- Current PR #102 source candidate `fe3e4d0ae55a90a0b07deec124def54f7c79f041` adds cross-platform rejection of empty executable paths and embedded NUL in executable, args, and optional working directory before process spawn; capability authorization remains first. Four regression tests and `docs/spec/one-v0.1-interop.md` update included.
- The prior exact-SHA GHA PASS `44044795819` applies only to predecessor `5ee645...`; it does not transfer to `fe3e4d0...`. Current exact-SHA GHA run #2159 / `44048756331` is IN_PROGRESS; intermediate run #2157 `790dd...` and #2158 `ac2ee...` also in progress at last poll. The `790dd...` intermediate contained an over-escaped literal, corrected in `ac2ee...`; current candidate is `fe3e4d0...`. Do not claim PASS until the exact current run is terminal on both platforms.
- This is process interop input validation only; no grammar/semantics/CLI UX change. E4/DoD score remains unchanged until exact-SHA verification.


## PR #95 exact-source Windows verification — run #42 — 2026-11-17

- Current PR #95 source `993f76351c7a119bea2e66f7d9e5bca07f392fbc` passed independent Windows Server 2025 run #42 / GitHub Actions ID `44048174334`, verifier SHA `6070e2ec839cc10b49068a84733d301ec6375b58`, job `133919144866`; all 31 primary steps terminal SUCCESS.
- Artifact `14312034494`, ZIP SHA-256 `0d86f4876414d032d4062554958f23d7308ae38c76ff3a4cf491292757df7ef0`; native EXE SHA `45ebf54262ab9d77ba14f63b79f7d0e80987df27659ea65311b078453dacdf9a`; release CLI EXE SHA `a53e5f5e41e493a3c98123a592321242f8efa2883f3d2ee8b538d4c6385c0a88`; manifest SHA `62297b0ea101d53fdcf93e610503d02e5ae803a2e1ec19f4680e6a7635f83834`. Artifact and manifest hashes match; both executables parse as PE32+ AMD64; native and CLI outputs `Budi`, exit 0; CLI `one 0.1.0`.
- Verifier PR #42 docs current commit `56271be82645357c2feaa1134ae2dc1d2ceae734` records run #42; documentation-only verifier run #43 / `44049102193` is IN_PROGRESS on the same exact source pin. No product source change.
- CircleCI status for PR #95 source `993f763...` is not visible through GitHub; optional `release-check` remains NOT VERIFIED. CircleCI Pipeline #558 belongs to PR #102 SHA `5ee645...`, not PR #95. No merge/release or DoD promotion.


## User installation prerequisites correction — PR #102 — 2026-11-17

- Review of workspace dependency `rusqlite = { version = "0.40.2", features = ["bundled"] }` confirmed the build compiles bundled SQLite C source. The installation guide previously listed Git/Rust only and omitted the native compiler toolchain, which could cause first-time builds to fail on clean Linux/Windows machines.
- Updated `docs/panduan-pengguna/instalasi.md`: Debian/Ubuntu installs `build-essential`; other Linux distributions need GCC/Clang and `make`; Windows requires Visual Studio Build Tools 2022+ with Desktop development with C++, MSVC x64/x86 build tools and Windows SDK, using Developer PowerShell. Added `cc`, `make`, `cl`, and `link` checks. The doc still accurately states no installer or official prebuilt release exists.
- Current PR #102 source HEAD `ac1bb214bc028395367cce7edc234a1909971e2a` is a docs-only descendant of exact-SHA GHA PASS `fe3e4d0...`. Fresh current-SHA GHA run has not yet appeared; no PASS is transferred. PR remains OPEN / DRAFT / UNMERGED.
