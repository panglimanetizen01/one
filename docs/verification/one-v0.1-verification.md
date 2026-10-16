# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Current audit state

The repository was audited after integration of the native HTTP/2 runtime. The contract-artifact checker was strengthened to require the concrete artifacts referenced by `docs/README.md`, including retained and derivative contracts. The FTP/SMTP/POP3/IMAP application-runtime boundary was also hardened for command-field injection and bounded line/body framing. The HTTPS loopback fixture was then made address-family deterministic by using IPv4 loopback and a certificate containing both `localhost` and `127.0.0.1` identities.

## Exact-SHA evidence

Independent verification run `38846301365` checked out canonical source commit `e58da6393e53094f29a5121cb440c94973f71b6a` and passed on both Ubuntu and Windows Server 2025. Both jobs passed canonical SHA verification, contract-artifact consistency, workspace build, Clippy, all workspace tests, real network runtime tests, platform-specific offline verification, and deterministic artifact checks. The real-network suite reported `38 passed; 0 failed` on both platforms, including the HTTPS/TLS loopback regression and HTTP/2 runtime tests.

This documentation synchronization records that evidence. The source revision itself was independently verified before this documentation commit; any later documentation-only revision must not be represented as tested unless a fresh exact-SHA matrix checks it.

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

Previous fully green runs remain historical evidence for their exact commits. Run `38846301365` is the latest completed independent matrix and directly verifies `e58da6393e53094f29a5121cb440c94973f71b6a` on Ubuntu and Windows Server 2025.

## Final gate

A documentation-only revision after a verified source revision is not silently treated as verified. If this file or the release contract changes again, the resulting canonical SHA requires a fresh independent Ubuntu/Windows matrix before that new SHA is called verified.

**Current evidence status: source commit `e58da6393e53094f29a5121cb440c94973f71b6a` has a completed independent Ubuntu/Windows verification matrix.**
