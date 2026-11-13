# Project ONE CI Contract

**Canonical CI target:** CircleCI  
**Migration state at 2026-11-13:** configuration introduced on the migration branch; exact-SHA CircleCI execution and parity are still pending.  
**Canonical source repository:** `panglimanetizen01/one`  
**Configuration:** `.circleci/config.yml`

## Provider boundary

- GitHub remains the canonical source repository, version-control host, pull-request system, and code-review surface.
- CircleCI is the selected canonical CI execution provider for Project ONE.
- GitHub Actions workflows are retained temporarily as a migration safety net. They must not be removed or disabled until CircleCI has passed the equivalent checks against the same exact source revision and required Windows/Linux evidence is available.
- The standalone `ci-local` extraction is a separate completed project and is not imported, invoked, or recreated by this configuration.

## Default CI workflow

The `one-ci` workflow runs the product CI suite on Linux and Windows for repository-triggered pipelines:

1. Verify Rust, Cargo, and Python toolchains.
2. Provision the Android NDK r27c, Android AArch64 Clang, and MinGW-w64 cross-toolchains.
3. Run `python tools/check_contract.py`.
4. Build all workspace targets.
5. Run Clippy with warnings denied.
6. Run all workspace/all-target tests.
7. Explicitly retain deterministic `.oneir`/`.onepkg`, DNS, orchestration, database, HTTP GET, and HTTP GET CLI regression gates.

The Linux executor uses CircleCI's Android Gen1 machine image tag `android:default` with the `large` resource class. It supplies the Android SDK/NDK tooling required by the existing native CLI tests. The job bootstraps the stable Rust toolchain through rustup because the Android image does not guarantee Rust is preinstalled. The pinned NDK revision remains `27.2.12479018`; do not silently change the toolchain revision without updating the existing product CI contract and evidence.

## Opt-in verification workflows

The following pipeline parameters default to `false` and are intended for manual/API-triggered pipelines:

- `run_release_checks=true`: runs the Linux and Windows release checks, including real-network runtime tests and the repository's platform-specific offline verification scripts.
- `run_windows_native_verification=true`: runs the dedicated Windows native verification: host architecture, Rust/Clippy, locked tests, Windows GNU native build, PE32+ AMD64 validation, execution smoke test, and artifact upload.

The current canonical `main` tree does not contain a committed `Cargo.lock`. Accordingly, the CircleCI cache key hashes the root/workspace manifests and Rust toolchain file, and the dedicated Windows job does not pass Cargo's `--locked` flag. The contract checker rejects reintroducing `--locked` until a lockfile is committed.

The Windows cross-toolchain setup installs MSYS2/MinGW-w64 and the exact Android NDK r27c package. The downloaded Android NDK archive is SHA-256 checked before extraction. The expected digest is `27e49f11e0cee5800983d8af8f4acd5bf09987aa6f790d4439dda9f3643d2494`.

## First pipeline observations

CircleCI accepted the configuration and created Linux jobs, but the first observed Linux jobs failed. GitHub status contexts were `ci/circleci: one-linux` on `a213165cb175355c73412fe0787d11e6e0e9807f` and `17b81146dc610a44a369e29b36551f46e717ea9e`. The associated CircleCI workflow/job IDs are recorded in the verification record. Job logs were not accessible through the current ChatGPT integrations, so the failure root cause is UNKNOWN. The Android executor was changed from the Gen2-oriented `current` tag to the Gen1 `default` tag to match `resource_class: large`; this is a configuration correction, not a proven diagnosis. Recheck the latest PR SHA and inspect fresh job logs before claiming the issue is resolved.

## Exact-SHA evidence requirements

A config file existing or parsing is not a CI pass. A migration is complete only after:

- CircleCI project integration is enabled for this repository.
- The default workflow has executed on a specific commit SHA.
- Linux and Windows jobs are both green at that SHA.
- Release and Windows-native workflows have been triggered and inspected at the intended SHA where their gates are required.
- Job logs and produced artifacts have been inspected.
- Product check parity with the retired GitHub Actions workflows is documented.
- The GitHub Actions workflows have then been retired in a reviewed change, with no parallel CI path remaining.

Do not record a CircleCI PASS based on a successful local YAML parser, a configuration file upload, or an unexecuted pipeline.

## Failure handling

- Preserve CircleCI job logs and artifacts as the primary execution evidence.
- Treat a job that never starts or has no executable steps as infrastructure failure/inconclusive, not as proof of a product source failure.
- A failing product step must be reproduced or isolated, fixed at its root cause, covered by regression tests where appropriate, and rerun at the resulting exact SHA.
- Do not weaken language semantics, security/capability rules, or product tests to make CI green.

## Migration rollback boundary

Until CircleCI parity is proven, leave the existing GitHub Actions workflows unchanged. If CircleCI cannot start, keep the migration branch unmerged and continue using the existing documented independent exact-SHA verification route. No force-push, history rewrite, merge, release, or paid GitHub upgrade is authorized by this CI migration.
