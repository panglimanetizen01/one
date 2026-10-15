# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Current audit state

The repository has been audited after integration of the native HTTP/2 runtime. The contract-artifact checker was strengthened to require the concrete artifacts referenced by `docs/README.md`, including retained and derivative contracts.

The current canonical source has not yet received a fresh exact-SHA verification run after these audit/documentation changes. Therefore this record remains **VERIFICATION-PENDING** until the temporary public verification mirror checks out the exact current `main` commit and passes all stages.

## Required execution gate

The exact current canonical commit must pass, on both Ubuntu and Windows Server 2025:

- canonical SHA verification;
- contract-artifact consistency check;
- workspace build with all targets;
- Clippy with `-D warnings`;
- all workspace tests with all targets;
- real network runtime tests;
- platform-specific offline verification;
- deterministic `.oneir` and `.onepkg` smoke checks included by the offline verifiers.

## Contract-artifact gate

`tools/check_contract.py` now checks:

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

Previous independent verification runs remain historical evidence only and must not be relabeled as verification of the current canonical SHA. The latest fully green network verification before the current audit was run `38781871063`, against canonical implementation commit `c2fecc0a488d335c73ff3abcc37f2f5d28a59a1f`, with both Linux and Windows passing contract checks, build, Clippy, all workspace tests, real-network runtime tests, and offline verification.

## Result

**Current canonical verification: PENDING fresh exact-SHA audit run.**
