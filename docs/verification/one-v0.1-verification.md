# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Latest exact-SHA executable verification

Canonical source revision: `a332dc5e58760460c26ebebb33f32f910bfdf4b3`.

Environment: Termux/Android.

The exact-SHA checkout passed contract checks, build, Clippy with `-D warnings`, targeted filesystem-write CLI regression test, full workspace tests, and offline verification. The filesystem-write CLI test executed a One source program with `--izinkan filesystem.write --target <path>` and verified the target file contained the expected text.

This evidence is platform-bound to Termux/Android. It does not establish Ubuntu/Windows support for `a332dc5e58760460c26ebebb33f32f910bfdf4b3`.

## Latest cross-platform exact-SHA verification

Canonical source revision: `9038ac75f197753ff1ab6af7478e63e4831903a2`.

Independent verification mirror: `panglimanetizen01/one-windows-verification`, run `38929072800`.

The independent mirror checked out the exact recorded SHA and verified it on Ubuntu and Windows Server 2025. The run passed contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, deterministic artifact checks, orchestration runtime, and platform-specific offline verification.

That cross-platform evidence predates the filesystem-write commits and must not be generalized to the current `main` SHA.

## Evidence boundary

`VERIFIED` is reserved for claims backed by executable evidence for the exact recorded source SHA and stated platform boundary. Evidence for one commit does not automatically verify later commits.

## Verification infrastructure status

The public verification mirror remains infrastructure only; GitHub `main` remains the canonical source of truth. The canonical hosted CI path for the filesystem-write CLI work produced zero-step job failures and retries produced no executable jobs, so it is classified as infrastructure evidence failure rather than source failure. No paid GitHub upgrade was used or required.
