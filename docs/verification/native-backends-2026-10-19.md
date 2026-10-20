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

PR #64 was merged into `main` at merge commit `ec844aa652a4d35c3891fc810d344c9c9db2f5cf` on 2026-10-20.

PR #65 subsequently repaired the canonical documentation state and was merged into `main` at `100d0f9553be6352b8dbc76b3e6dad67f5a7a9e2`.

The merge itself establishes source presence on canonical `main`; it does **not** promote the branch-bound execution/build evidence above to current-head verification. A post-merge exact-SHA verification is still required before any native backend is documented as verified for the merged `main` revision.

At the implementation merge SHA `100d0f9553be6352b8dbc76b3e6dad67f5a7a9e2`, the post-merge `One CI` run was `39372356906` (run attempt 2 after retry). GitHub reported both matrix jobs as `failure`, but both jobs had `steps: []`. This is **CI infrastructure/runner evidence**, not source-code failure evidence: the jobs never reached checkout, toolchain, build, Clippy, or test steps. The retry reproduced the same no-step failure on both Ubuntu and Windows, with no job logs or artifacts produced.

After the documentation repair commit `5e618c2514a99093c8a7c1d40faf5af7f0baf2de`, `One CI` run `39381125667` independently reproduced the same failure on both matrix jobs, again with `steps: []`. This confirms the runner/infrastructure condition persisted independently of the native-backend implementation.

Until a post-merge executable verification is recorded, retain these boundaries:
- Linux AArch64: verified only for the recorded AArch64/Linux verification host and implementation SHA above.
- Linux x86_64: build-proven only for the recorded verification environment and implementation SHA above.
- Windows x86_64: build-proven only for the recorded verification environment and implementation SHA above.
