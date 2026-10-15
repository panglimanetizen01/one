# One v0.1 Linux/Windows Verification Evidence — 2026-10-15

## Canonical source

Repository: `panglimanetizen01/one`

This document records historical and current verification evidence. It must never claim a source SHA that was not actually checked out by the verification runner.

## Latest fully green independent network verification

Temporary verification mirror: `panglimanetizen01/one-windows-verification`

Workflow run: `38781871063`

Canonical implementation commit checked out: `c2fecc0a488d335c73ff3abcc37f2f5d28a59a1f`

Ubuntu job: `116583878787` — success.
Windows Server 2025 job: `116583878563` — success.

Both jobs completed:

- canonical SHA verification;
- contract-artifact check;
- build;
- Clippy;
- all workspace tests;
- real network runtime tests;
- platform-specific offline verification.

The real-network stage included the verified HTTP/2 loopback runtime tests. This run is the latest fully green evidence before the current post-HTTP/2 contract-audit changes.

## Current audit gate

After that green run, the canonical repository received a contract-artifact closure audit and documentation corrections. The current `main` commit therefore requires a fresh exact-SHA verification run before the release documents may return to `VERIFIED`.

Required current-SHA stages are:

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

Historical HTTP/2 verification: **GREEN** at `c2fecc0a488d335c73ff3abcc37f2f5d28a59a1f`.

Current post-audit canonical verification: **PENDING exact-SHA rerun**.
