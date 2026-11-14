# Independent Windows Native Verification

## Ownership and provider boundary

- Canonical source: private repository `panglimanetizen01/one`.
- Canonical Linux CI: CircleCI, configured in `.circleci/config.yml`.
- Windows host verification: independent repository `panglimanetizen01/one-windows-verification`.
- Workflow: `.github/workflows/windows-native-backend-runtime.yml`.
- Executor: GitHub-hosted Windows Server 2025 AMD64.
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
