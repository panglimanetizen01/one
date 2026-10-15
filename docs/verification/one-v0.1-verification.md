# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Current audit state

The repository was audited after integration of the native HTTP/2 runtime. The contract-artifact checker was strengthened to require the concrete artifacts referenced by `docs/README.md`, including retained and derivative contracts.

## Exact-SHA evidence

Canonical commit `abf61aaebb6c3eddf198ab040dea6dd47db18b81` passed independent verification run `38783385413` on both Ubuntu and Windows Server 2025. All required stages passed: canonical SHA verification, contract-artifact consistency, workspace build, Clippy, all workspace tests, real network runtime tests, platform-specific offline verification, and deterministic artifact checks.

The release documentation was subsequently recorded in commit `cf0949ff6cfecf25b3ee12c5f11431aa4d81b0f9`. Because that documentation commit changes the canonical SHA, it requires one final exact-SHA verification run before it can be labeled VERIFIED.

## Contract-artifact gate

`tools/check_contract.py` checks:

- root release/security/contribution/CI artifacts;
- normative foundation, language, network, implementation, verification, and release contracts;
- retained v0.1 contract artifacts;
- executable conformance and CLI derivatives;
- offline verification scripts;
- every concrete artifact referenced by `docs/README.md`;
- valid/invalid/golden fixture minimums;
- normative grammar fixture markers;
- the no-GitHub-upgrade rule.

## Independent verification infrastructure

The temporary public repository `panglimanetizen01/one-windows-verification` is verification infrastructure only. It checks out the private canonical repository at an exact SHA. GitHub `main` remains the source of truth.

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution.

## Historical evidence

Previous fully green runs remain historical evidence for their exact commits. The latest pre-audit network verification was run `38781871063` against `c2fecc0a488d335c73ff3abcc37f2f5d28a59a1f`. The post-audit verification was run `38783385413` against `abf61aaebb6c3eddf198ab040dea6dd47db18b81`.

## Final gate

The current canonical SHA after the release-documentation update is `cf0949ff6cfecf25b3ee12c5f11431aa4d81b0f9`. This exact SHA must pass the same Ubuntu/Windows matrix before the release record can be closed as VERIFIED.

**Current canonical verification: PENDING final exact-SHA run.**
