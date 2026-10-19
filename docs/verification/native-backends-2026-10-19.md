# Native Backend Verification Record — 2026-10-19

## Scope

PR #64 adds native hosted backends for Linux AArch64, Linux x86_64, and Windows x86_64 while retaining Android AArch64. The semantic pipeline remains target-independent:

`One Source → Parser → Analyzer → One Semantic IR → Target Backend → Toolchain → Native Artifact`

## Exact revision

PR branch: `feature/native-backends-linux-windows`

Verification revision: `797c6fe75bd3fa9081329f73dd59e34278f27ddd`

This is branch-bound evidence. It is not a verification claim for `main` until the PR is merged and the merged SHA is reverified.

## Linux AArch64

- Target: `aarch64-unknown-linux-gnu`
- Artifact: ELF 64-bit AArch64 executable.
- One CLI native build succeeded.
- Artifact executed successfully on the AArch64 verification host and printed `ONE_NATIVE_LINUX_AARCH64`.
- Status: **VERIFIED — exact PR SHA, AArch64/Linux host boundary**.

## Linux x86_64

- Target: `x86_64-unknown-linux-gnu`
- The backend uses Clang with the verified cross-linker `--ld-path=/usr/bin/x86_64-linux-gnu-ld` in the current verification environment.
- Regression test `linux_x86_64_build_menggunakan_cross_linker_x86_64` passes.
- One CLI native build succeeded.
- `file` identified the artifact as ELF 64-bit x86-64 with `/lib64/ld-linux-x86-64.so.2`.
- No x86_64 executor was available in the verification environment; execution is therefore not claimed.
- Status: **BUILD TERBUKTI — execution pending**.

## Windows x86_64

- Target: `x86_64-pc-windows-gnu`
- Native artifact build is covered by the backend/CLI tests.
- Artifact was previously proven as PE32+ x86-64.
- No Windows/Wine executor was available in the current Termux verification environment; execution is therefore not claimed.
- Status: **BUILD TERBUKTI — execution pending**.

## Regression and static verification

- Native backend integration suite: **9/9 passed**.
- `cargo clippy -p one-core --all-targets -- -D warnings`: **passed**.
- Linux x86_64 CLI native build: **passed**, ELF x86-64 artifact produced.
- `git diff --check`: **passed** before commit.
- Commit `797c6fe` contains only the bounded native linker fix and its regression test: 2 files, 36 insertions, 9 deletions.

## Full-suite boundary

A later `cargo test` run on the Debian/proot verification environment failed in the pre-existing Android CLI regression because the environment selected the Debian host Clang and could not find the Android/Linux libc header `bits/libc-header-start.h`. The failure is environmental and outside the Linux x86_64 linker fix scope; it must not be converted into a claim that the native linker fix failed.

## Next gate

Merge/review of PR #64 must not be treated as final native support completion. After merge, rerun exact-SHA verification on the merged `main` revision. Linux x86_64 and Windows x86_64 require real target execution evidence before their status can be promoted from build-proven to executable-verified.
