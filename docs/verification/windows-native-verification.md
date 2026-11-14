# Independent Windows Native Verification

## Ownership and provider boundary

- Canonical source: private repository `panglimanetizen01/one`.
- Canonical Linux CI: CircleCI, configured in `.circleci/config.yml`.
- Windows host verification: independent repository `panglimanetizen01/one-windows-verification`.
- Workflow: `.github/workflows/windows-native-backend-runtime.yml`.
- Executor: GitHub-hosted Windows Server 2025 AMD64 (`runs-on: windows-2025`).
- The Windows workflow is not a CircleCI job and its result must never be described as CircleCI evidence.

The canonical ONE repository contains no CircleCI Windows executor jobs. Its Linux job retains MinGW-w64 only for Linux-hosted cross-compilation regression coverage of the Windows target. Actual Windows-host build and execution are performed by this independent verifier.

## Exact canonical source identity

The verifier reads `WINDOWS_SOURCE_COMMIT.txt`, requires exactly 40 lowercase hexadecimal characters, checks out `panglimanetizen01/one` at that exact commit using the repository-scoped `CANONICAL_READ_TOKEN` secret, and fails unless `git -C source rev-parse HEAD` equals the pin.

It separately asserts the verifier checkout SHA against the workflow's own commit identity. The manifest records both `source_sha` and `verifier_sha`; neither identity may be inferred from the other.

## Required Windows checks

1. Validate canonical source contracts and user documentation.
2. Provision stable Rust, MinGW-w64 Clang/linker, and pinned Android NDK r27c.
3. Build all workspace targets.
4. Run Clippy with warnings denied.
5. Run the complete workspace test suite.
6. Verify deterministic `.oneir`/`.onepkg` artifacts.
7. Run DNS, orchestration, database, HTTP GET, HTTP GET CLI, real-network, and offline verification checks.
8. Build the Windows GNU native artifact from `examples/hello.one`.
9. Inspect the DOS MZ header, PE signature, AMD64 machine field `0x8664`, and PE32+ optional-header magic `0x20B`.
10. Execute the Windows artifact and require successful exit plus the expected `Budi` output.
11. Confirm the source checkout remains at the pinned SHA and tracked source files/index remain unchanged.

The workflow intentionally does not use Cargo's `--locked` flag while the canonical repository does not commit `Cargo.lock`.

## Evidence artifact and acceptance

The workflow uploads:

- `one-windows-native.exe`
- `one-windows-native-runtime-manifest.txt`

The manifest records canonical source SHA, verifier SHA, workflow run ID/attempt, job status, Windows host/architecture, Rust/Cargo/Clang versions, target triple, binary SHA-256, and native runtime result.

A Windows PASS requires all of the following:

- The run is terminal SUCCESS on the exact canonical source SHA.
- The source and verifier identity assertions pass.
- Contract, build, Clippy, workspace tests, real-network/offline checks pass.
- Windows native build, PE32+ AMD64 inspection, and execution pass.
- The manifest's source SHA and verifier SHA match the workflow identities.
- The downloaded executable SHA-256 matches the manifest.

A workflow definition, pin file, queued run, successful Linux CircleCI job, or older source SHA is not a Windows PASS.

## Demonstrated verifier baseline — 2026-11-14

PR #42, `verify/windows-pr95-20261114`, introduced the independent exact-SHA Windows workflow. It remains draft/unmerged pending review.

Run [#9](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43589646641) completed SUCCESS for canonical source SHA `195755054bb733f44ff30acf8d3281b4efc820bf`. The manifest recorded verifier SHA `d2ee1902f343108a4b87f586921e7799608b1ff1`, Windows Server 2025 / AMD64, native exit code `0`, and output `Budi`.

- Artifact ID: `14111933646`
- ZIP SHA-256: `baebbed5fd822fdd35748e03a22dbbddbe7ad6c15d735ce3108718c9616055c2`
- Embedded manifest SHA-256: `adba50318a035da3842658e97b0bd5cc51fe3fcb50d558712eb66fccfa029b6e`
- Embedded executable SHA-256: `257aebe8f2bdd790064578faae1d41ece4d62fd73bda3d0ffd1c2fdff7bcaf3d`

The ZIP digest was independently recomputed from the downloaded artifact; the executable hash independently matched the manifest. This proves the verifier path is viable for source SHA `195755...` only. It does not verify later PR #95 commits. Every later source SHA requires its own run and artifact inspection; current run evidence is maintained in the Notion checkpoint and linked GitHub run.


## Current post-split exact-SHA validation — 2026-11-14

### Latest exact-SHA provider-split evidence — 2026-11-14

- Canonical ONE candidate: PR #95, branch `ci/circleci-migration-20261113`, exact source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642`, based on `main` `d35db95153f6ddd79885494c72f5aee16c899751`. PR #95 remains OPEN / DRAFT / UNMERGED.
- Independent verifier: PR #42, branch `verify/windows-pr95-20261114`, PR head `bc460d92d5e90e2808e1d835adf86bffad8a3463`. Windows run [#10](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43591754838) completed terminal SUCCESS on Windows Server 2025 / AMD64 for the exact canonical source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642`. Job: `132437775362`; manifest verifier checkout SHA: `07cd54d440c4f22910e4ed8417f4e6515339fb8d`.
- The manifest records run `43591754838`, attempt `1`, `job_status=success`, target `x86_64-pc-windows-gnu`, native exit code `0`, and output `Budi`. The workflow's contract/user-doc checks, workspace build, Clippy, complete tests, deterministic artifact checks, DNS/orchestration/database/HTTP checks, real-network tests, offline verification, native build, PE inspection, and execution all completed successfully.
- Artifact ID: `14113026885`; ZIP size: `15866` bytes; ZIP SHA-256: `9188dc0f81636e0e8dac536939eff9fcf0792e8d24b8b7f904beb74ff4fd1192`; manifest SHA-256: `b018c3719ff01fc58876250b32c9663f6f46b44dc1137e3f8e2b36f4d71a6ce8`; executable SHA-256: `22ef1752296a2cf1ce4ccc8a88e99274c73fd0eed65b5b27d34c9879eb6f3bc7`. The ZIP digest was independently recomputed; the manifest and executable hashes matched. The 52,918-byte executable independently passed MZ, PE, AMD64 `0x8664`, and PE32+ `0x20B` checks.
- CircleCI on the same canonical source SHA reports `ci/circleci: one-linux` FAILURE: workflow `bb9174a8-ac7f-42e1-84ed-1ce816267d64`, job `301edf07-748d-4adb-83b0-ba4dc8594ac9`, failure report `9c6078ca-186a-41e1-8d0e-386ac601952f`. No Windows CircleCI context is expected under the new ownership split.
- The authenticated CircleCI first-failure log is not accessible in this execution session; the failed step and root cause remain UNKNOWN. Do not infer a product-code defect or claim Linux PASS.
- Result: Windows is PASS for source SHA `fcbda49b586fb5cede2f7ff000e6e43969aaa642` only. Overall cross-platform verification is NOT PASS while CircleCI Linux is failed. Keep PR #95 draft/unmerged and `main` unchanged.
