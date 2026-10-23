# Human-vNext exact-SHA verification fallback contract

**Contract:** `human-vnext-exact-sha-local/1.0`  
**Scope:** F6 evidence for the 30% human-vNext foundation milestone only  
**Status:** Repository-internal review; no external CI or cross-platform certification claimed  
**Canonical runner:** `tools/verify-human-vnext.sh`

## Why a fallback exists

The repository's GitHub Actions runs inspected for this branch terminate before executing any job steps. Such runs provide neither a passing result nor evidence of a source-code failure. The local fallback exists to produce reproducible Linux evidence for the exact GitHub branch commit without misrepresenting hosted CI as passing.

## Required procedure

Run the verifier only in a clean worktree freshly fetched from the canonical GitHub repository and branch. The verifier itself must confirm that:
1. The current directory is the repository root and the worktree is clean.
2. `origin` points to the canonical ONE repository.
3. `HEAD` exactly matches the current remote branch head before and after checks.
4. The contract checker, shell syntax check, and `git diff --check` pass.
5. `cargo build --workspace --all-targets` and strict Clippy pass.
6. The complete workspace test suite passes, except that the Android cross-build integration test may be explicitly skipped when `ONE_ANDROID_CLANG` is unavailable. If skipped, that limitation must be recorded; it must never be reported as passed or as Android execution evidence.
7. The verification command exits successfully and reports `VERIFICATION RESULT: PASS` plus the exact source SHA.

When `ONE_ANDROID_CLANG` is configured, the verifier passes it to the test process and runs the full test suite. Without it, only `android_target_explicit_mempertahankan_build_native` is excluded; all other tests still run.

## Evidence and non-claims

- A successful run establishes Linux exact-SHA contract/build/Clippy/test evidence only.
- It does not establish GitHub-hosted CI success, Windows test execution, Android native execution, or release readiness.
- If the remote branch advances during verification, the run fails and must be repeated against the new exact head.
- Logs must be retained in the conversation or another reviewable record, with the exact SHA, toolchain versions, skipped tests, and all test-suite summaries.
- No completion points are awarded for this document alone. F6 may receive its two points only when the exact-SHA procedure passes and the grammar, CLI, task app, persistence, documentation, and regression evidence for F1–F5 are also complete.

## Review decision

The fallback is accepted as a bounded replacement for the unavailable hosted CI *for the Linux exact-SHA evidence component of the 30% foundation gate only*. It is not an equivalent replacement for cross-platform CI and does not waive any F1–F5 acceptance criteria. The review is repository-internal; no external reviewer approval is claimed.
