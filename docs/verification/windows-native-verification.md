# Independent Windows Native Verification


## Current exact-SHA CI candidate checkpoint — 2026-11-17

- Canonical candidate code/config snapshot: PR #95, branch `ci/circleci-migration-20261113`, exact source SHA `07d8cbf7422b8340f8efda3c2f9d4425f85b7537`; base `main` SHA `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI `ci/circleci: one-linux`: terminal SUCCESS for this exact SHA; workflow `0e03e28b-dfb9-4198-b897-3ce5d16dc2d8`, job `b2f536a1-6c87-49b5-9a3f-2db2f2908595`.
- Independent Windows Server 2025 native verification: [run #28](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43865524749), terminal SUCCESS; job `133292953068`, 32/32 main steps succeeded. Manifest source SHA exactly matches the candidate; target `x86_64-pc-windows-gnu`; host Windows Server 2025 AMD64; native exit code `0`, output `Budi`.
- Windows artifact ID `14225302303`, 15,868 bytes; ZIP SHA-256 `7dd4435e41bc8f0660f454768711c0fc460e2ca0fe38f2dee1034c59c0f767a5`; executable SHA-256 `9004e87f0c9d0357ee0197a1bce2a26474cb714aa8d71d54990236c9266dcfeb`; manifest SHA-256 `d95ae44ee24c5b793aeef59fb006e48d4c5b3e62de4cd5012b2b5a769cbf92a4`.
- Both required candidate platform gates therefore report PASS on the same exact code/config source SHA: CircleCI Linux and independent native Windows. The documentation-only PR #95 descendant is not falsely represented as a run at its later HEAD.
- CircleCI config defines Linux-host `one-linux` by default and opt-in Linux `release-linux`; it does not claim a CircleCI Windows executor. Linux MinGW cross-compilation is not Windows-host execution.
- PR #95 remains OPEN / DRAFT / UNMERGED. Canonical `main` remains `d35db95153f6ddd79885494c72f5aee16c899751` and still contains the three legacy GitHub Actions workflows; migration is not active on main until an authorized merge.
- Opt-in CircleCI `release-check` (`run_release_checks=true`) has not been executed for this candidate. CI PASS is not release approval or proof that the entire ONE project is 100% complete. No merge, tag, or release is authorized.


## Ownership and provider boundary

- Canonical source: `panglimanetizen01/one`; GitHub metadata currently reports the repository as PUBLIC. Treat source and public build logs as a disclosure boundary; never store credentials or private data in tracked files or logs.
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


## Latest post-split exact-SHA Windows verification — 2026-11-14

- Canonical source snapshot: `panglimanetizen01/one` SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858`, which was the PR #95 head when the run was triggered. The independent verifier PR #42 head is `09d5ce0c9653be91bceb6c99110802e138f7660a`; `WINDOWS_SOURCE_COMMIT.txt` pins the canonical source SHA exactly.
- Windows run [#11](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43592922789) completed terminal SUCCESS. Job `132441279904` ran on Windows Server 2025 / AMD64. The manifest records verifier checkout SHA `4435dbac04103c140b2ec14abc97d816ade28be9`, target `x86_64-pc-windows-gnu`, `job_status=success`, native exit code `0`, and output `Budi`.
- The workflow executed the canonical contract and user-document checks, including the new guard that reports `legacy GitHub Actions workflow files: 0`, then completed workspace build, Clippy, full workspace tests, deterministic artifacts, DNS/orchestration/database/HTTP checks, real-network tests, offline verification, Windows native build, PE inspection, and execution.
- Artifact ID: `14112773100`; ZIP size: `15869` bytes; ZIP SHA-256: `8ca9e99edd8ea7df80fd5cd1dc06d6d402bbddc13479bf0607bd8f2691fe1e56`; manifest SHA-256: `e9d9290dbfae95284b96435d3ad79f245d3d7fbe90b42a3e5994cdb352dff197`; executable SHA-256: `d2b0afa0f71fd2af831d75cdf02f1a8c7c5156654cf5a0b7fc6bcaa14cebdeaf`. The downloaded ZIP digest was independently recomputed and matched; the embedded manifest and executable hashes were independently checked. The 52,918-byte executable passed MZ, PE, AMD64 `0x8664`, and PE32+ `0x20B` checks.
- CircleCI on the same canonical source SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858` reports `ci/circleci: one-linux` FAILURE: workflow `ae7233e9-6146-417e-bef9-17b236a93716`, job `a8c73406-8277-4c6b-83d1-5d0ed85670a2`, failure report `47dd0f34-7bc4-43a7-8136-8a975b947fd6`. No Windows CircleCI job is expected under the split.
- The CircleCI job page and authenticated first-failure output are inaccessible through the current tools. The first failed Linux step/root cause remain UNKNOWN; do not infer a product-code defect or claim Linux PASS.
- Result: Windows PASS for canonical source SHA `4d22db6eae48739fd79dce732d99c9b6d86b5858` only. Overall cross-platform verification is NOT PASS because CircleCI Linux is failed. PR #95 and PR #42 remain OPEN / DRAFT / UNMERGED; `main` is unchanged.
- The subsequent PR #95 update containing this evidence is documentation-only; the tested source snapshot remains the exact SHA pinned above. The pin is not a floating branch name.


## Latest exact-SHA Windows verification — run #12 — 2026-11-14

- Canonical source snapshot: `panglimanetizen01/one` SHA `f85375e279257dcbb7d4ded9632800fcd1b78cc9`, which was PR #95's HEAD when run #12 was triggered. Independent verifier PR #42 head: `e765113b66eca514aa4759ddcc7215fc09254554`; `WINDOWS_SOURCE_COMMIT.txt` pins the canonical source SHA exactly.
- Windows run [#12](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43593490063) completed terminal SUCCESS. Job `132442986608` ran on Windows Server 2025 / AMD64. The manifest records verifier checkout SHA `0f1b22c6b92ece622d48218165d9d1b382a7ec82`, target `x86_64-pc-windows-gnu`, `job_status=success`, native exit code `0`, and output `Budi`.
- The workflow executed the canonical contract and user-document checks, including the guard output `legacy GitHub Actions workflow files: 0`, then completed workspace build, Clippy, full workspace tests, deterministic artifacts, DNS/orchestration/database/HTTP checks, real-network tests, offline verification, Windows native build, PE inspection, and execution.
- Artifact ID: `14113646474`; ZIP size: `15866` bytes; ZIP SHA-256: `551df64ac4ee00f1a2a7c9a65140b091cc956a2024d5ff23e12112cc3ade9201`; manifest SHA-256: `bfed07d536ce7d95a9b09140075c954877411922d5e85e12d79a69fd3f1dd638`; executable SHA-256: `ceaa2996f70a03cb69fb08b4b4d4edf63a2db453e5c13cfc7e356235fbb04e81`. The downloaded ZIP digest was independently recomputed and matched; the embedded manifest and executable hashes were independently checked. The 52,918-byte executable passed MZ, PE, AMD64 `0x8664`, and PE32+ `0x20B` checks.
- CircleCI on the same canonical source SHA `f85375e279257dcbb7d4ded9632800fcd1b78cc9` reports `ci/circleci: one-linux` FAILURE: workflow `1bc98a22-c403-4ba6-98dd-9143463a52b7`, job `588e3ce3-acc7-44cb-b453-c1d3787e03be`, failure report `53d7d9f0-a113-4728-8cf0-2a4729ae91dc`. No Windows CircleCI job is expected under the split.
- The CircleCI job page and authenticated first-failure output are inaccessible through the current tools. The first failed Linux step/root cause remain UNKNOWN; do not infer a product-code defect or claim Linux PASS.
- Result: Windows PASS for canonical source SHA `f85375e279257dcbb7d4ded9632800fcd1b78cc9` only. Overall cross-platform verification is NOT PASS because CircleCI Linux is failed. PR #95 and PR #42 remain OPEN / DRAFT / UNMERGED; `main` is unchanged.
- The subsequent PR #95 evidence-only documentation commit records this run; the tested source snapshot remains the exact SHA pinned above.


## Latest PR #95 source candidate — 2026-11-17

- Canonical candidate SHA: `e6bfb6bb0256dbd30710ceb544cc0d776e2927bf`.
- This candidate adds the workspace Cargo lockfile and matching target-specific platform dependencies; the CircleCI candidate now uses `--locked` for all build, Clippy, and test commands and its contract checker enforces that requirement.
- CircleCI Linux is PENDING for this exact SHA (workflow `ca520794-8456-4c2d-af0b-a7f85baaea83`, job `9cd8432e-89c8-4867-b661-64096ab6920f`).
- Windows run #32 remains PASS only for older source SHA `a10a69afdf8675e703f38f1620abc935d8361897`; no Windows PASS is claimed for `e6bfb6b...`. A fresh exact-SHA native Windows run is still required.
- Earlier CircleCI failure root cause remains NOT VERIFIED; do not infer it from the lockfile integration gap.
