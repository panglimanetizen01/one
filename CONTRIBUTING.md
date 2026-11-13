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

## CI and verification

CircleCI is the selected canonical CI provider. Its repository configuration is `.circleci/config.yml`; the required jobs and opt-in verification workflows are documented in `docs/CI.md`.

- Linux and Windows product checks must both pass on the exact commit SHA.
- The release workflow retains real-network tests and platform-specific offline verification.
- The dedicated Windows native workflow retains PE32+ validation, target execution, and artifact upload.
- Do not retire GitHub Actions until CircleCI parity is proven at the same exact SHA. The legacy workflows remain a temporary migration safety net.
- Do not purchase a GitHub upgrade solely to obtain hosted CI capacity. Independent offline verification remains a separate evidence path and must never be mislabeled as CircleCI execution.

GitHub remains the canonical source repository and pull-request/code-review host. CircleCI is the intended CI execution authority.
