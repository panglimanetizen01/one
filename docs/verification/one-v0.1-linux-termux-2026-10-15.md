# One v0.1 Independent Linux Verification Evidence

Date: 2026-10-15

## Canonical source

- Repository: `panglimanetizen01/one`
- Branch: `main`
- Verified commit: `b4b60c82656a2564894cf7398e47984cd5d9e388`
- Local verification checkout: `/storage/emulated/0/master one/one`
- Local `HEAD` matched `origin/main` before verification.
- Working tree was clean before verification.

## Verification environment

- Environment: Android/Termux Linux userspace
- Rust: `rustc 1.97.1 (8bab26f4f 2026-07-14)`
- Cargo: `cargo 1.97.1 (c980f4866 2026-06-30)`
- Cargo target directory: `$HOME/.one-target`

The source checkout is on Android shared storage. The Cargo target directory was moved to the Termux home filesystem because executables under `/storage/emulated/0` cannot be executed there. This is a verification-environment constraint and does not modify the canonical source tree.

## Exact verification path

```text
cd "/storage/emulated/0/master one/one"
git fetch origin main
git reset --hard origin/main
git clean -fd
test "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)"
test -z "$(git status --porcelain)"
rm -rf "$HOME/.one-target"
mkdir -p "$HOME/.one-target"
CARGO_TARGET_DIR="$HOME/.one-target" bash tools/verify_offline.sh
```

## Recorded results

- Contract consistency: PASS
  - required artifacts: 30
  - valid fixtures: 6
  - invalid fixtures: 5
  - golden fixtures: 3
- Workspace build: PASS
- Clippy with `-D warnings`: PASS
- Workspace/all-target tests: PASS
- CLI integration tests: 11 passed, 0 failed
- Project tests: 2 passed, 0 failed
- Core library tests: 5 passed, 0 failed
- Conformance tests: 5 passed, 0 failed
- Core pipeline tests: 5 passed, 0 failed
- Function/event tests: 2 passed, 0 failed
- Lexer tests: 2 passed, 0 failed
- Name-boundary tests: 2 passed, 0 failed
- Security tests: 2 passed, 0 failed
- CLI smoke path: PASS
- Deterministic `.oneir` comparison: PASS
- Deterministic `.onepkg` comparison: PASS

The regression covering list mutation followed by `untuk setiap` also passed: `executes_list_loop ... ok`.

Final verifier result:

```text
OFFLINE VERIFICATION: PASS
```

## Evidence boundary

This record establishes independent Linux/Termux execution evidence for the recorded commit. It does **not** establish Windows execution evidence and does not relabel the previously blocked GitHub-hosted CI runs as green.
