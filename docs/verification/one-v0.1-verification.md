# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Current audit state

The repository was audited after integration of the native HTTP/2 runtime. The contract-artifact checker was strengthened to require the concrete artifacts referenced by `docs/README.md`, including retained and derivative contracts. The FTP/SMTP/POP3/IMAP application-runtime boundary was also hardened for command-field injection and bounded line/body framing.

## Exact-SHA evidence

Independent verification run `38786708863` checked out canonical source commit `abee462cb3941dbad4bddca1737027263c040d6e` and passed on both Ubuntu and Windows Server 2025. All required stages passed: canonical SHA verification, contract-artifact consistency, workspace build, Clippy, all workspace tests, real network runtime tests, platform-specific offline verification, and deterministic artifact checks.

The documentation synchronization following that run is deliberately treated as a new source revision. The final release gate therefore requires one fresh independent Ubuntu/Windows matrix against the resulting canonical SHA; no earlier run is substituted for that gate.

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

Previous fully green runs remain historical evidence for their exact commits. The latest completed pre-synchronization run was `38786708863`. The final release decision must use the fresh matrix against the documentation-synchronized SHA.

## Final gate

The final gate is the same independent Ubuntu/Windows matrix against the exact canonical SHA produced by the documentation synchronization. The gate must pass contract artifacts, build, Clippy, tests, real network runtime, and platform-specific offline verification on both platforms before this record is interpreted as final verification evidence.

**Current canonical verification: FINAL VERIFICATION GATE.**
