# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Latest exact-SHA executable verification

Canonical source revision: `55a8474ad5b19e20fcd51c3be79f39aaa374bbf3`.

Environment: Termux/Android.

The exact-SHA checkout passed the canonical `tools/verify_exact_sha.sh` harness. The run verified the exact checkout SHA, contract artifacts, workspace build, Clippy with `-D warnings`, full workspace tests, deterministic artifact output, DNS tests, orchestration tests, filesystem vertical slice and capability isolation, process vertical slice, database vertical slice and CLI tests, and HTTP GET vertical slice and CLI tests.

The full workspace test run passed all discovered test suites, including the current database and HTTP source-to-runtime paths. The database CLI suite passed 3/3 and the HTTP GET CLI suite passed 3/3. The database vertical slice passed 7/7 and the HTTP GET vertical slice passed 3/3.

This evidence is platform-bound to Termux/Android. It establishes exact-SHA executable evidence for canonical `main` on that platform only; it does not establish Ubuntu/Windows support for `55a8474ad5b19e20fcd51c3be79f39aaa374bbf3`.

## Latest cross-platform exact-SHA verification

Canonical source revision: `9038ac75f197753ff1ab6af7478e63e4831903a2`.

Independent verification mirror: `panglimanetizen01/one-windows-verification`, run `38929072800`.

The independent mirror checked out the exact recorded SHA and verified it on Ubuntu and Windows Server 2025. The run passed contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, deterministic artifact checks, orchestration runtime, and platform-specific offline verification.

That cross-platform evidence predates the database, HTTP, and later filesystem-write changes and must not be generalized to the current executable revision.

## Evidence boundary

`VERIFIED` is reserved for claims backed by executable evidence for the exact recorded source SHA and stated platform boundary. Evidence for one commit does not automatically verify later commits.

## Verification infrastructure status

The public verification mirror remains infrastructure only; GitHub `main` remains the canonical source of truth. Historical canonical hosted CI attempts for the affected work produced zero-step job failures, so those runs are classified as infrastructure evidence failure rather than source failure. The current canonical executable evidence is the independent Termux/Android exact-SHA run recorded above. No paid GitHub upgrade was used or required.
