# One v0.1 Verification Record


## Current exact-SHA CI candidate — 2026-11-16

- PR #95 candidate: `panglimanetizen01/one`, branch `ci/circleci-migration-20261113`, exact source SHA `07d8cbf7422b8340f8efda3c2f9d4425f85b7537`; base `main` `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI `ci/circleci: one-linux`: PASS; workflow `0e03e28b-dfb9-4198-b897-3ce5d16dc2d8`; job `b2f536a1-6c87-49b5-9a3f-2db2f2908595`.
- Independent Windows Server 2025 run [#28](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43865524749): PASS, 32/32 steps; native exit code 0; output `Budi`; verifier SHA `a83e05437a7d0612f1bf6f5dcc0c7b39483a8173`.
- Windows artifact ZIP SHA-256 `7dd4435e41bc8f0660f454768711c0fc460e2ca0fe38f2dee1034c59c0f767a5`; executable SHA-256 `9004e87f0c9d0357ee0197a1bce2a26474cb714aa8d71d54990236c9266dcfeb`; manifest SHA-256 `d95ae44ee24c5b793aeef59fb006e48d4c5b3e62de4cd5012b2b5a769cbf92a4`.
- Both platform gates PASS for this exact source SHA. The optional `release-check` has not run. PR #95 remains OPEN / DRAFT / UNMERGED; `main` is unchanged and still contains the legacy GitHub Actions workflows. This is candidate evidence, not merge/release approval or proof of 100% project completion.

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

## CircleCI infrastructure incident record — 2026-11-16

The historical Windows CircleCI job #254 at source SHA e44d16b2b5f4ef50c33674ccf711bd11b410effa failed during MSYS2 first-run/keyring initialization after the Windows runner and Rust installation had succeeded. It did not reach ONE source verification. This workflow is superseded; current ONE CircleCI configuration does not require reintroducing MSYS2. Windows host verification is handled separately.

CircleCI jobs #409, #424, and #426 have explicit `free-plan-no-credits-available` messages and are PLAN-CREDIT-BLOCKED, not runner-not-started failures. Jobs #407 and #408 remain pre-run blockers with unknown root cause. Do not conflate these records. Independent Windows verifier run #21 passed only for pinned source SHA f85375e279257dcbb7d4ded9632800fcd1b78cc9. See [full incident evidence](circleci-infrastructure-incident-2026-11-16.md) and [normative CI evidence contract](../spec/one-v0.1-ci-verification-contract.md).
