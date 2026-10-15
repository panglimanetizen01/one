# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Current audit state

The repository was audited after integration of the native HTTP/2 runtime. The contract-artifact checker was strengthened to require the concrete artifacts referenced by `docs/README.md`, including retained and derivative contracts. The FTP/SMTP/POP3/IMAP application-runtime boundary was also hardened for command-field injection and bounded line/body framing.

## Exact-SHA evidence

Independent verification run `38785876520` checked out the then-current canonical source commit and passed on both Ubuntu and Windows Server 2025. All required stages passed: canonical SHA verification, contract-artifact consistency, workspace build, Clippy, all workspace tests, real network runtime tests, platform-specific offline verification, and deterministic artifact checks.

Because subsequent documentation updates change the canonical SHA, one final exact-SHA verification run is required after the documentation set is synchronized. The release record must not be interpreted as evidence for a later SHA that has not itself been executed.

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

Previous fully green runs remain historical evidence for their exact commits. The latest pre-hardening network verification was run `38781871063`. The post-audit hardening verification was run `38785876520`.

## Final gate

The synchronized documentation set changes the canonical SHA again. The final gate is the same independent Ubuntu/Windows matrix against the resulting exact `main` SHA. No final VERIFIED claim is made until that exact SHA passes all required stages.

**Current canonical verification: PENDING final exact-SHA run after documentation synchronization.**
