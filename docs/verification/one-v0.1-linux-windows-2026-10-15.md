# One v0.1 Linux/Windows Verification Evidence — 2026-10-15

## Canonical source

Repository: `panglimanetizen01/one`

This document records verification evidence. It must never claim a source SHA that was not actually checked out by the verification runner.

## Latest fully green independent verification

Temporary verification mirror: `panglimanetizen01/one-windows-verification`

Independent run `38848143359` checked out the exact canonical source revision `38dc98fcee49fa04c589d5a3c825b0d4006d6d61` and passed on both Ubuntu and Windows Server 2025.

Both jobs completed:

- canonical SHA verification;
- contract-artifact check;
- build;
- Clippy;
- all workspace tests;
- real network runtime tests;
- platform-specific offline verification.

The real-network stage included the verified HTTP/2 loopback runtime tests and the hardened FTP/SMTP/POP3/IMAP tests.

## Current result

The exact current canonical SHA has now passed the independent Ubuntu/Windows matrix. The release documentation is therefore allowed to identify this SHA as verified.

Any later canonical commit requires another exact-SHA verification before it can be labeled verified.

## Interpretation

These are independent GitHub-hosted executions in a temporary public verification repository, not executions of the private canonical repository's hosted workflow. The canonical repository remains the source of truth.

The private canonical GitHub Actions environment may remain blocked if its runner jobs fail before usable step/log evidence. No paid GitHub upgrade is required or authorized for this verification path.

## Result

Current canonical source revision `38dc98fcee49fa04c589d5a3c825b0d4006d6d61`: **GREEN / VERIFIED** by independent run `38848143359` on Ubuntu and Windows Server 2025.
