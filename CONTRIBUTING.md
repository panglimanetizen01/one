# Contributing to One

One is specification-driven. Contributions must preserve the approved semantic contracts.

## Required workflow

1. Read the foundation and relevant specification.
2. Follow the dependency-ordered implementation plan.
3. Add a failing test before production behavior for every new feature or bug fix.
4. Implement the smallest behavior that makes the test pass.
5. Run formatting, linting, unit, integration, and conformance checks.
6. Do not silently change language semantics to make an example pass.
7. Update the specification when behavior intentionally changes.
8. Record verification evidence with the exact commit, toolchain, platform, command, and result.

## Verification without paid hosted CI

If GitHub-hosted runners are unavailable or GitHub presents a paid-upgrade requirement, do not purchase capacity merely to validate One. Run the canonical offline verifier in an independent environment:

- Linux: `bash tools/verify_offline.sh`
- Windows PowerShell: `./tools/verify_offline.ps1`

Independent results are execution evidence, but they must not be reported as GitHub-hosted CI results. Keep the hosted-CI blocker documented until it is actually resolved.

The workspace `Cargo.lock` is committed and must remain in version control. Dependency changes must update the lockfile intentionally; CI uses `--locked` so an unrecorded dependency resolution fails closed.

GitHub `main` is the canonical source of truth. Local environments are for verification and validation.

## Canonical CI and platform verification

CircleCI is the intended canonical Linux CI provider. Until the migration PR is merged with explicit owner authorization and a terminal exact-SHA CircleCI PASS, GitHub `main` remains on its existing GitHub Actions workflow. Do not describe the migration as active merely because a draft branch contains `.circleci/config.yml`.

Windows Server 2025 native execution is verified separately in [panglimanetizen01/one-windows-verification](https://github.com/panglimanetizen01/one-windows-verification). Record both the exact ONE source SHA and verifier SHA. Linux-host cross-compilation is not Windows-host execution evidence.


### Repository integrity preview (developer tooling)

The read-only repository integrity preview is a development tool, not part of the ONE source language or beginner-facing CLI guide:

~~~sh
cargo test --locked -p one-cli --bin one integrity::tests
cargo run --locked -p one-cli --bin one -- pindai --scope all --root . --expected-sha <40-character-git-sha>
python3 -m unittest discover -s tools -p "test_one_health.py" -v
~~~

The native Rust preview and Python reference scanner are intentionally separate implementations. Do not claim parity until both run on the same exact checkout and their differences are reviewed. Findings are advisory at this preview stage; execution errors and exact-SHA mismatches are not advisory.
