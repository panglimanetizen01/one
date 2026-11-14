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

PR #64 was merged into `main` at merge commit `ec844aa652a4d35c3891fc810d344c9c9db2f5cf` on 2026-10-20. The last code-bearing canonical `main` revision independently verified is `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`. Subsequent evidence-sync commits change documentation only, not source or tests.

Independent exact-SHA verification run [`39554661451`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451) checked out this exact canonical revision and passed on Ubuntu and Windows Server 2025. The jobs passed contract artifacts, build, AST contract regression tests, Clippy, workspace tests, real-network runtime tests, and platform-appropriate offline verification. Linux cross-compilers and Windows/Android cross-toolchains were provisioned successfully.

Canonical hosted `One CI` runs `39372356906` and `39381125667` failed before executable steps (`steps: []`). This is infrastructure evidence rather than source-test evidence; it does not negate the successful independent exact-SHA matrix.

Target execution evidence remains target-specific:
- Linux AArch64: verified for the recorded AArch64/Linux host and earlier implementation SHA.
- Linux x86_64: the exact-SHA source build and test matrix passed; claim actual target execution only where the recorded native backend test ran the produced artifact.
- Windows x86_64: the exact-SHA Windows build and test matrix passed; claim actual target execution only where the recorded native backend test ran the produced artifact.


## Independent Windows-host verification path — 2026-11-14

Windows host verification is now performed outside canonical CircleCI by `panglimanetizen01/one-windows-verification`, workflow `.github/workflows/windows-native-backend-runtime.yml`. The workflow pins a full canonical source SHA in `WINDOWS_SOURCE_COMMIT.txt`, checks the source checkout and verifier checkout identities independently, builds and executes the Windows GNU artifact on Windows Server 2025 AMD64, and uploads a manifest plus executable.

Baseline run [#9](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43589646641) passed for source SHA `195755054bb733f44ff30acf8d3281b4efc820bf`; the PE32+ AMD64 artifact executed with exit code `0` and output `Budi`. See `windows-native-verification.md` for the independently verified ZIP, manifest, and binary digests. This result is source-SHA-specific and does not transfer to later commits.


## Post-split Windows-host execution evidence — 2026-11-14

### Latest exact-SHA provider-split evidence — 2026-11-14

- Canonical ONE candidate: PR #95, branch `ci/circleci-migration-20261113`, exact source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642`, based on `main` `d35db95153f6ddd79885494c72f5aee16c899751`. PR #95 remains OPEN / DRAFT / UNMERGED.
- Independent verifier: PR #42, branch `verify/windows-pr95-20261114`, PR head `bc460d92d5e90e2808e1d835adf86bffad8a3463`. Windows run [#10](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43591754838) completed terminal SUCCESS on Windows Server 2025 / AMD64 for the exact canonical source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642`. Job: `132437775362`; manifest verifier checkout SHA: `07cd54d440c4f22910e4ed8417f4e6515339fb8d`.
- The manifest records run `43591754838`, attempt `1`, `job_status=success`, target `x86_64-pc-windows-gnu`, native exit code `0`, and output `Budi`. The workflow's contract/user-doc checks, workspace build, Clippy, complete tests, deterministic artifact checks, DNS/orchestration/database/HTTP checks, real-network tests, offline verification, native build, PE inspection, and execution all completed successfully.
- Artifact ID: `14113026885`; ZIP size: `15866` bytes; ZIP SHA-256: `9188dc0f81636e0e8dac536939eff9fcf0792e8d24b8b7f904beb74ff4fd1192`; manifest SHA-256: `b018c3719ff01fc58876250b32c9663f6f46b44dc1137e3f8e2b36f4d71a6ce8`; executable SHA-256: `22ef1752296a2cf1ce4ccc8a88e99274c73fd0eed65b5b27d34c9879eb6f3bc7`. The ZIP digest was independently recomputed; the manifest and executable hashes matched. The 52,918-byte executable independently passed MZ, PE, AMD64 `0x8664`, and PE32+ `0x20B` checks.
- CircleCI on the same canonical source SHA reports `ci/circleci: one-linux` FAILURE: workflow `bb9174a8-ac7f-42e1-84ed-1ce816267d64`, job `301edf07-748d-4adb-83b0-ba4dc8594ac9`, failure report `9c6078ca-186a-41e1-8d0e-386ac601952f`. No Windows CircleCI context is expected under the new ownership split.
- The authenticated CircleCI first-failure log is not accessible in this execution session; the failed step and root cause remain UNKNOWN. Do not infer a product-code defect or claim Linux PASS.
- Result: Windows is PASS for source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642` only. Overall cross-platform verification is NOT PASS while CircleCI Linux is failed. Keep PR #95 draft/unmerged and `main` unchanged.
