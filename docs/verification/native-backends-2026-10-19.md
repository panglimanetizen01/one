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


## Latest post-split exact-SHA Windows verification — 2026-11-14

- Canonical source snapshot: `panglimanetizen01/one` SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858`, which was the PR #95 head when the run was triggered. The independent verifier PR #42 head is `09d5ce0c9653be91bceb6c99110802e138f7660a`; `WINDOWS_SOURCE_COMMIT.txt` pins the canonical source SHA exactly.
- Windows run [#11](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43592922789) completed terminal SUCCESS. Job `132441279904` ran on Windows Server 2025 / AMD64. The manifest records verifier checkout SHA `4435dbac04103c140b2ec14abc97d816ade28be9`, target `x86_64-pc-windows-gnu`, `job_status=success`, native exit code `0`, and output `Budi`.
- The workflow executed the canonical contract and user-document checks, including the new guard that reports `legacy GitHub Actions workflow files: 0`, then completed workspace build, Clippy, full workspace tests, deterministic artifacts, DNS/orchestration/database/HTTP checks, real-network tests, offline verification, Windows native build, PE inspection, and execution.
- Artifact ID: `14112773100`; ZIP size: `15869` bytes; ZIP SHA-256: `8ca9e99edd8ea7df80fd5cd1dc06d6d402bbddc13479bf0607bd8f2691fe1e56`; manifest SHA-256: `e9d9290dbfae95284b96435d3ad79f245d3d7fbe90b42a3e5994cdb352dff197`; executable SHA-256: `d2b0afa0f71fd2af831d75cdf02f1a8c7c5156654cf5a0b7fc6bcaa14cebdeaf`. The downloaded ZIP digest was independently recomputed and matched; the embedded manifest and executable hashes were independently checked. The 52,918-byte executable passed MZ, PE, AMD64 `0x8664`, and PE32+ `0x20B` checks.
- CircleCI on the same canonical source SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858` reports `ci/circleci: one-linux` FAILURE: workflow `ae7233e9-6146-417e-bef9-17b236a93716`, job `a8c73406-8277-4c6b-83d1-5d0ed85670a2`, failure report `47dd0f34-7bc4-43a7-8136-8a975b947fd6`. No Windows CircleCI job is expected under the split.
- The CircleCI job page and authenticated first-failure output are inaccessible through the current tools. The first failed Linux step/root cause remain UNKNOWN; do not infer a product-code defect or claim Linux PASS.
- Result: Windows PASS for canonical source SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858` only. Overall cross-platform verification is NOT PASS because CircleCI Linux is failed. PR #95 and PR #42 remain OPEN / DRAFT / UNMERGED; `main` is unchanged.
- The subsequent PR #95 update containing this evidence is documentation-only; the tested source snapshot remains the exact SHA pinned above. The pin is not a floating branch name.
