# Human-vNext exact-SHA local verification contract

**Contract ID:** `human-vnext-local-verification/1`  
**Purpose:** reproducibly verify the human-vNext foundation when GitHub-hosted Actions cannot start jobs.  
**Scope:** independent Linux/Termux-Debian execution of source fetched from the GitHub feature branch. This contract does not claim Windows execution or hosted CI passed.

**Authority:** this procedure implements the user-directed rule that implementation changes stay on GitHub and local Termux/Debian is used only to fetch GitHub source and validate/test it.

## Source-of-truth rules

1. GitHub branch `spec/human-instruction-grammar-vnext-20261022` is the only source of implementation changes for this workstream.
2. The local verification worktree must be clean and detached at the current GitHub branch HEAD. Local edits, local commits, and local pushes are prohibited as implementation inputs.
3. The verifier compares local `HEAD` with `git ls-remote` before and after the checks. A mismatch fails the run rather than producing evidence.
4. Evidence applies only to the exact printed commit SHA and platform/toolchain printed by the script.
5. A skipped or ignored test is not a pass. The verifier output and test summary must retain ignored-test counts.
6. A local pass is not represented as a GitHub Actions pass, Windows pass, Android native execution pass, or release certification.

## Required commands

Run `bash tools/verify-human-vnext.sh` from a clean worktree fetched from GitHub. The script runs, in order:

1. source cleanliness and exact remote-HEAD checks;
2. `bash -n tools/verify-human-vnext.sh`;
3. `python3 tools/check_contract.py`;
4. `git diff --check`;
5. `cargo build --workspace --all-targets`;
6. `cargo clippy --workspace --all-targets -- -D warnings`;
7. `cargo test --workspace --all-targets`;
8. final worktree cleanliness and a second exact-HEAD check to detect local changes or branch movement during verification.

If the Android AArch64 native execution test is intended to run, set `ONE_ANDROID_CLANG` to a compatible Android AArch64 Clang executable before running the script. If unavailable, the test may be ignored and must be recorded as such.

## Evidence recording

For each run, retain the full terminal output and record:
- exact GitHub source SHA;
- platform and Rust/Cargo/Python versions;
- contract-checker result;
- build, Clippy, and test results including passed/failed/ignored counts;
- any tests not executed and the specific reason;
- whether GitHub-hosted Actions and Windows verification actually ran.

A valid record is invalidated if the source SHA does not match the recorded SHA or the remote branch moved during the run.

## Formatting boundary

Workspace-wide `cargo fmt --all -- --check` is not an acceptance gate for this milestone: the current repository baseline reports formatting differences across numerous pre-existing Rust modules, including code outside this tranche. Do not bulk-reformat unrelated modules solely to manufacture a green result. The exact-SHA gate remains contract checks, shell syntax, diff cleanliness, workspace build, Clippy with warnings denied, and workspace tests. Formatting cleanup is separate work and is not represented as passed.

## Independent hosted cross-platform verification fallback

When canonical `one` repository Actions jobs terminate before executing any steps, the existing free verification mirror may provide hosted exact-SHA evidence without upgrading the canonical repository plan:

- Mirror repository: `panglimanetizen01/one-windows-verification`.
- Workflow: `.github/workflows/windows-verification.yml`, branch `verification`.
- Source selector: `CANONICAL_COMMIT.txt` must contain the complete 40-character source SHA from the canonical ONE feature branch.
- The workflow checks out the canonical private source at that exact SHA and independently verifies the checked-out HEAD before running checks.
- Replacement evidence is valid only if **both** `ubuntu-latest` and `windows-2025` jobs finish successfully and the run's source SHA matches the recorded canonical SHA. Required successful steps include contract artifacts, build, Clippy, all workspace tests, real-network tests, and platform-appropriate offline verification.
- Record the mirror run URL, mirror commit, exact canonical source SHA, both job conclusions, and ignored/skipped test counts in the ledger.
- This is **independent hosted cross-platform verification**, not a claim that canonical `one` repository Actions passed. Do not credit a pending, failed, zero-step, wrong-SHA, or partial matrix run. Do not upgrade the repository plan to obtain a runner.

This fallback preserves the platform-evidence requirement rather than waiving it: the mirror must actually execute both Linux and Windows jobs on the exact source snapshot.

## Reviewed secondary fallback when the mirror's Ubuntu setup is blocked

**Review status:** accepted for the 30% foundation gate by repository-internal technical review; no external sign-off is claimed.

If the mirror's `ubuntu-latest` job remains in dependency installation for at least 15 minutes without reaching contract/build/test steps, or fails before those steps for runner/package infrastructure reasons, use this alternative without changing the acceptance criteria:

1. The mirror's `windows-2025` job must finish successfully on the exact canonical source SHA, including contract artifacts, build, Clippy, workspace tests, real-network tests, and Windows offline verification.
2. A clean GitHub-fetched Termux/Debian worktree must run `bash tools/verify-human-vnext.sh` successfully on that **same canonical source SHA**. This executes the contract checker, build, Clippy, and workspace tests locally.
3. Record the Ubuntu job as **BLOCKED/INFRASTRUCTURE**, never as passed; record the successful Windows job and the exact-SHA local Debian output separately. Do not call the overall mirror matrix green when its Ubuntu job is still blocked.
4. Do not upgrade the canonical repository plan, suppress tests, reclassify ignored tests as passed, or change the point weights to use this route.

This is a location-of-execution fallback, not a test waiver: both Linux and Windows must actually execute the required checks against the identical source SHA. It is valid only for this bounded 30% gate and does not certify Android native execution, release readiness, or later 60%/100% milestones.

## Windows offline verifier failure propagation

The Windows offline verifier must set `$PSNativeCommandUseErrorActionPreference = $true` so a non-zero exit from `cargo`, `python`, or another native executable fails the script instead of allowing a false PASS. Its workspace test command excludes only `android_target_explicit_mempertahankan_build_native`, which requires an Android AArch64 toolchain/runtime not supplied by the Windows runner. This exclusion is a skip, never a pass; Android-native execution remains separately unverified unless run in an appropriate environment.

## Replacement boundary

This is a controlled replacement for the **local build/test evidence collection procedure only** when hosted Actions cannot start. It does not manufacture CI results and does not independently waive any acceptance criterion that requires Windows execution, external review, or a platform-specific runtime result. The milestone ledger may credit points only after its full acceptance criteria are met and the evidence is explicitly reviewed.
