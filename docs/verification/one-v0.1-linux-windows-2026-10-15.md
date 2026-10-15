# One v0.1 Linux/Windows Verification Evidence — 2026-10-15

## Canonical source

Repository: `panglimanetizen01/one`

This document records verification evidence. It must never claim a source SHA that was not actually checked out by the verification runner.

## Latest fully green independent verification before documentation synchronization

Temporary verification mirror: `panglimanetizen01/one-windows-verification`

The post-audit hardening verification passed on both Ubuntu and Windows Server 2025 for the exact canonical source revision then recorded by `CANONICAL_COMMIT.txt`.

Both jobs completed:

- canonical SHA verification;
- contract-artifact check;
- build;
- Clippy;
- all workspace tests;
- real network runtime tests;
- platform-specific offline verification.

The real-network stage included the verified HTTP/2 loopback runtime tests and the hardened FTP/SMTP/POP3/IMAP tests.

## Current final gate

The canonical documentation set has now been synchronized with the post-audit runtime boundary. That synchronization changes the canonical SHA, so the resulting exact SHA requires one final independent Ubuntu/Windows verification run before the release record can be closed.

Required final stages are:

- exact canonical SHA verification;
- contract-artifact closure;
- workspace build with all targets;
- Clippy with `-D warnings`;
- all workspace tests with all targets;
- real network runtime tests;
- Linux offline verification;
- Windows offline verification;
- deterministic `.oneir` and `.onepkg` smoke checks included by the offline scripts.

## Interpretation

These are independent GitHub-hosted executions in a temporary public verification repository, not executions of the private canonical repository's hosted workflow. The canonical repository remains the source of truth.

The private canonical GitHub Actions environment may remain blocked if its runner jobs fail before usable step/log evidence. No paid GitHub upgrade is required or authorized for this verification path.

## Result

Historical and post-audit verification: **GREEN for the exact source revisions explicitly checked out by their respective runs**.

Current synchronized-documentation canonical verification: **PENDING final exact-SHA rerun**.
