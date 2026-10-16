# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Latest independently executable exact-SHA verification

Canonical source revision: `f482c6b75505d9965dd33f4f25bb0cd04072cedb`.

Independent verification mirror: `panglimanetizen01/one-windows-verification`, run `38903950612`.

The independent mirror checked out the canonical One source at the exact recorded SHA and verified the source revision on Ubuntu and Windows Server 2025. The run passed the recorded contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, deterministic artifact checks, orchestration runtime, and platform-specific offline verification.

The verified orchestration boundary in this run includes the DNS source-level slice and the filesystem-read source-level slice. It does not include the later process-execution implementation.

## Evidence boundary

`VERIFIED` is reserved for claims backed by executable evidence for the exact recorded source SHA. The evidence above verifies `f482c6b75505d9965dd33f4f25bb0cd04072cedb`; it does not automatically verify later commits.

Canonical `main` is now `ab7df1e2a0dbcc8c3fab3b425137194d1ca0ccbe`. The documentation synchronization merge is documentation-only relative to the previous main baseline, but no new exact-SHA executable run has yet been recorded for `ab7df1e2a0dbcc8c3fab3b425137194d1ca0ccbe`.

The process/system vertical-slice branch contains source/runtime changes and therefore requires a fresh exact-SHA run before its process-execution capability can be labeled `EXPERIMENTAL`, `VERIFIED`, or `SUPPORTED`.

## Verification infrastructure status

The public verification mirror remains infrastructure only; GitHub `main` remains the canonical source of truth. The currently available mirror branches explicitly state that execution is not valid until their source tree exactly matches the canonical SHA recorded for the verification attempt. No paid GitHub upgrade was used or required.
