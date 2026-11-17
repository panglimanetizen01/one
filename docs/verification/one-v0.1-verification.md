# One v0.1 Verification Record

## Current exact-SHA verification checkpoint — 2026-11-17

- PR #98–#101 each passed GitHub Actions on both Ubuntu and Windows Server 2025 on their own exact candidate SHAs: runs #2018, #2019, #2020, and #2138 respectively. These changes remain unmerged and are not yet one integrated source revision.
- PR #102 exact SHA `6c72d8e548bbbc28d85be39b8de427614989a038` passed run #2229 / `44086429416` on Ubuntu and Windows Server 2025.
- PR #107 exact SHA `21e393157e52728d9510699ac6c9120987e3852c` passed run #2230 / `44088328030` on Ubuntu and Windows Server 2025.
- PR #103 ledger exact SHA `041b359b8504551c2c1705093050d9c467368bac` passed run #2231 / `44089449162` on Ubuntu and Windows Server 2025.
- PR #108 is the current CircleCI migration candidate. CircleCI has not returned a status context or job-level execution evidence for its current head. Its config and static integration fixes do not constitute CircleCI PASS.
- GitHub `main` remains at `d35db95153f6ddd79885494c72f5aee16c899751`, still has GitHub Actions workflows, and has no CircleCI config.
- Issues #104 (filesystem root confinement), #105 (Unix escaped-descendant containment), and #82 (Android AArch64 runtime execution) remain OPEN. E6 remains NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED.

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Canonical main evidence

Canonical `main` verification evidence remains recorded at its exact SHA and stated platform boundary. This document must not silently replace that evidence with a feature-branch result.

## PR #64 native backend verification

Branch: `feature/native-backends-linux-windows`

Exact verification revision: `797c6fe75bd3fa9081329f73dd59e34278f27ddd`.

This is branch-bound evidence and becomes a canonical main verification record only after PR #64 is merged and the merged SHA is independently reverified.

### Linux AArch64

- Target: `aarch64-unknown-linux-gnu`.
- Native One CLI build succeeded.
- Artifact identified as ELF 64-bit AArch64.
- Artifact executed successfully on the AArch64/Linux verification host and printed `ONE_NATIVE_LINUX_AARCH64`.
- Status: **VERIFIED — exact PR SHA, AArch64/Linux host boundary**.

### Linux x86_64

- Target: `x86_64-unknown-linux-gnu`.
- Regression test `linux_x86_64_build_menggunakan_cross_linker_x86_64` passed.
- Native One CLI build succeeded.
- Artifact identified as ELF 64-bit x86-64 with `/lib64/ld-linux-x86-64.so.2`.
- No x86_64 executor was available.
- Status: **BUILD TERBUKTI — execution pending**.

### Windows x86_64

- Target: `x86_64-pc-windows-gnu`.
- Native artifact build is covered by backend/CLI verification.
- Artifact proven as PE32+ x86-64.
- No Windows/Wine executor was available.
- Status: **BUILD TERBUKTI — execution pending**.

### Native regression/static evidence

- Native backend integration suite: 9/9 passed.
- `cargo clippy -p one-core --all-targets -- -D warnings`: passed.
- Linux x86_64 native CLI build: passed; ELF x86-64 produced.
- `git diff --check`: passed before the fix commit.

## Full-suite boundary

A later full `cargo test` invocation in the Debian/proot verification environment failed in `android_target_explicit_mempertahankan_build_native` because the environment selected Debian host Clang and could not find `bits/libc-header-start.h`. This is environmental evidence outside the bounded Linux x86_64 linker fix and is not evidence that the fix itself failed. The targeted native suite, Clippy, native CLI build, and diff checks had already passed before commit.

## Latest cross-platform historical evidence

Independent mirror run `38929072800` verified revision `9038ac75f197753ff1ab6af7478e63e4831903a2` on Ubuntu and Windows Server 2025. That historical run predates the database, HTTP, filesystem-write, and native-backend revisions and must not be used for those later changes. The later exact-SHA matrix `39554661451` verifies canonical source `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` and includes the current source/core/CLI test suites for filesystem write, database query, and HTTP GET.

## Evidence boundary

`VERIFIED` is reserved for claims backed by executable evidence for the exact recorded source SHA and stated platform boundary. Evidence for one commit does not automatically verify later commits.

## Verification infrastructure status

The public verification mirror remains infrastructure only; GitHub `main` remains the canonical source of truth. Historical canonical hosted CI attempts for affected work produced zero-step job failures, so those runs are classified as infrastructure evidence failure rather than source failure. No paid GitHub upgrade was used or required.
