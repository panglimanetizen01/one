# Human-vNext exact-SHA local verification contract

**Contract ID:** `human-vnext-local-verification/1`  
**Purpose:** reproducibly verify the human-vNext foundation when GitHub-hosted Actions cannot start jobs.  
**Scope:** independent Linux/Termux-Debian execution of source fetched from the GitHub feature branch. This contract does not claim Windows execution or hosted CI passed.

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
2. `python3 tools/check_contract.py`;
3. `git diff --check`;
4. `cargo build --workspace --all-targets`;
5. `cargo clippy --workspace --all-targets -- -D warnings`;
6. `cargo test --workspace --all-targets`;
7. a second exact-HEAD check to detect branch movement during verification.

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

## Replacement boundary

This is a controlled replacement for the **local build/test evidence collection procedure only** when hosted Actions cannot start. It does not manufacture CI results and does not independently waive any acceptance criterion that requires Windows execution, external review, or a platform-specific runtime result. The milestone ledger may credit points only after its full acceptance criteria are met and the evidence is explicitly reviewed.
