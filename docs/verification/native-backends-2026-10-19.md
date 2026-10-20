# Native Backend Verification Record — 2026-10-19

PR #64 native backend evidence is anchored to implementation/fix SHA `797c6fe75bd3fa9081329f73dd59e34278f27ddd` on `feature/native-backends-linux-windows`.

- Linux AArch64 (`aarch64-unknown-linux-gnu`): ELF AArch64 built and executed successfully on the AArch64/Linux verification host. **VERIFIED** for that exact SHA and boundary.
- Linux x86_64 (`x86_64-unknown-linux-gnu`): ELF x86-64 built successfully using `--ld-path=/usr/bin/x86_64-linux-gnu-ld`; regression test passes. No x86_64 executor was available. **BUILD TERBUKTI**.
- Windows x86_64 (`x86_64-pc-windows-gnu`): PE32+ x86-64 artifact proven. No Windows/Wine executor was available. **BUILD TERBUKTI**.
- Native backend integration suite: **9/9 passed**.
- `cargo clippy -p one-core --all-targets -- -D warnings`: **passed**.
- Linux x86_64 native CLI build: **passed**.

A later full `cargo test` in the Debian/proot environment failed in the Android CLI regression because the environment selected Debian host Clang and could not find `bits/libc-header-start.h`. This is environmental evidence outside the bounded Linux x86_64 linker fix.

This evidence is branch-bound until PR #64 is merged and the merged `main` SHA is reverified. Linux x86_64 and Windows x86_64 require real target execution evidence before promotion from build-proven to executable-verified.
