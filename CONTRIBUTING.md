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
