# Native Backend Verification Record — 2026-10-19

PR #64 native backend evidence is anchored to implementation/fix SHA `797c6fe75bd3fa9081329f73dd59e34278f27ddd` on `feature/native-backends-linux-windows`.

- Linux AArch64 (`aarch64-unknown-linux-gnu`): ELF AArch64 built and executed successfully on the AArch64/Linux verification host. **VERIFIED** for that exact SHA and boundary.
- Linux x86_64 (`x86_64-unknown-linux-gnu`): ELF x86-64 built successfully using `--ld-path=/usr/bin/x86_64-linux-gnu-ld`; regression test passes. No x86_64 executor was available. **BUILD TERBUKTI**.
- Windows x86_64 (`x86_64-pc-windows-gnu`): PE32+ x86-64 artifact proven. No Windows/Wine executor was available. **BUILD TERBUKTI**.
- Native backend integration suite: **9/9 passed**.
- `cargo clippy -p one-core --all-targets -- -D warnings`: **passed**.
- Linux x86_64 native CLI build: **passed**.

A later full `cargo test` in the Debian/proot environment failed in the Android CLI regression because the environment selected Debian host Clang and could not find `bits/libc-header-start.h`. This is environmental evidence outside the bounded Linux x86_64 linker fix.

## Post-merge canonical state

PR #64 was merged into `main` at merge commit `ec844aa652a4d35c3891fc810d344c9c9db2f5cf` on 2026-10-20. The last production-implementation revision of canonical `main` independently verified is `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`. Subsequent evidence-sync commits also added test-only standard-library regression coverage in `crates/one-core/src/stdlib.rs`; they did not change production implementation, and those tests were not part of run `39554661451`.

Independent exact-SHA verification run [`39554661451`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451) checked out this exact canonical revision and passed on Ubuntu and Windows Server 2025. The jobs passed contract artifacts, build, AST contract regression tests, Clippy, workspace tests, real-network runtime tests, and platform-appropriate offline verification. Linux cross-compilers and Windows/Android cross-toolchains were provisioned successfully.

Canonical hosted `One CI` runs `39372356906` and `39381125667` failed before executable steps (`steps: []`). This is infrastructure evidence rather than source-test evidence; it does not negate the successful independent exact-SHA matrix.

Target execution evidence remains target-specific:
- Linux AArch64: verified for the recorded AArch64/Linux host and earlier implementation SHA.
- Linux x86_64: the exact-SHA source build and test matrix passed; claim actual target execution only where the recorded native backend test ran the produced artifact.
- Windows x86_64: the exact-SHA Windows build and test matrix passed; claim actual target execution only where the recorded native backend test ran the produced artifact.
