# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Last independently verified source baseline

`dba4fad49854952e256ae41744f57f93e39167a8` is the last independently verified canonical source revision recorded in this verification record. Independent run `38872070590` checked out that exact private-source SHA and executed the existing release gates on Ubuntu and Windows Server 2025.

The verified source baseline contains the general-purpose orchestration foundation: typed semantic capability requests, deterministic governance, governed typed adapters, deterministic adapter selection, normalized results/evidence, audit records, localized semantic-resolution fixtures, and the first real `network.dns.resolve` adapter. This foundation is implemented and verified as a runtime library boundary; language-level One-source orchestration is not yet claimed as supported.

## Exact-SHA evidence

Independent verification run `38872070590` in `panglimanetizen01/one-windows-verification` checked out canonical source commit `dba4fad49854952e256ae41744f57f93e39167a8` and passed on both Ubuntu and Windows Server 2025. Both jobs passed canonical SHA verification, contract-artifact consistency, workspace build, Clippy, all workspace tests, real-network runtime tests, and platform-specific offline verification. The offline verifiers also checked deterministic `.oneir` and `.onepkg` artifacts.

The canonical GitHub Actions run for the earlier pre-sync source remains unusable as source-failure evidence because its jobs terminated without runner IDs, executable steps, or logs. The independent mirror is the executable evidence path used for the verified source baseline.

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

## Verification boundary

`VERIFIED` is reserved for claims backed by executable evidence for the exact recorded source SHA. A later canonical SHA requires a fresh exact-SHA verification before that later SHA is called verified.

**Recorded evidence baseline: `dba4fad49854952e256ae41744f57f93e39167a8`, independently verified by run `38872070590`.**
