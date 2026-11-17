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

CircleCI is the canonical CI provider for the ONE repository's Linux product and Linux release checks. Its configuration is `.circleci/config.yml`; the current migration contract is documented in `docs/CI.md`.

- The CircleCI `one-linux` job must pass on the exact canonical source SHA.
- Windows Server 2025 host verification must pass separately in `panglimanetizen01/one-windows-verification`, using the same exact source SHA in `WINDOWS_SOURCE_COMMIT.txt`.
- The independent Windows manifest must record matching source/verifier identities and a binary SHA-256 matching the uploaded executable.
- Linux-host MinGW-w64 cross-compilation tests may remain in CircleCI; they do not replace Windows-host execution evidence.
- The opt-in CircleCI release workflow is Linux-only. Release approval remains a separate project-level decision.
- The migration branch deletes the three legacy Project ONE GitHub Actions CI workflows. Do not reintroduce them as a parallel CI path. `main` remains unchanged until the migration PR is reviewed and merged.
- Do not purchase a GitHub upgrade solely to obtain hosted CI capacity. Independent offline and Windows verification are separate evidence paths and must never be mislabeled as CircleCI execution.

GitHub remains the canonical source repository and pull-request/code-review host. CircleCI owns canonical Linux CI; the independent verifier owns Windows-host verification.
