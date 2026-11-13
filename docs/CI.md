# Project ONE CI Contract

**Canonical CI target:** CircleCI  
**Migration state at 2026-11-13:** configuration introduced on the migration branch; exact-SHA jobs have started, but successful CircleCI verification and provider parity remain pending.  
**Canonical source repository:** `panglimanetizen01/one`  
**Configuration:** `.circleci/config.yml`

## Provider boundary

- GitHub remains the canonical source repository, version-control host, pull-request system, and code-review surface.
- CircleCI is the selected canonical CI execution provider for Project ONE.
- The migration branch deletes the three Project ONE GitHub Actions CI workflows so it has no parallel CI path. The `main` branch remains unchanged until the migration PR is reviewed and merged. The PR must remain draft/unmerged until CircleCI passes equivalent checks on the same exact source revision and required Windows/Linux/release/native evidence is available.
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

The current canonical `main` tree does not contain a committed `Cargo.lock`. Accordingly, the CircleCI cache key hashes the root/workspace manifests and Rust toolchain file, and the dedicated Windows job does not pass Cargo's `--locked` flag. The contract checker rejects reintroducing `--locked` until a lockfile is committed. It also inspects each Windows provisioning/default/release/native command block and fails if a Rust/Cargo invocation lacks the explicit user Cargo-bin PATH prefix.

The Windows setup explicitly installs Rust stable through rustup because the CircleCI Windows Server 2025 image's published software inventory does not list Rust/Cargo; every PowerShell step that invokes Cargo prepends `%USERPROFILE%\\.cargo\\bin` to PATH. It also installs MSYS2/MinGW-w64 and the exact Android NDK r27c package. Image inventory: https://circleci.com/developer/machine/image/windows-server-2025-gui. The downloaded Android NDK archive is SHA-256 checked before extraction. The expected digest is `27e49f11e0cee5800983d8af8f4acd5bf09987aa6f790d4439dda9f3643d2494`.

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

The migration branch must not retain or reintroduce GitHub Actions CI workflows. If CircleCI cannot start or pass, keep the migration branch unmerged; `main` remains unchanged and its current workflows remain available until an approved merge. Continue using the documented independent exact-SHA verification route. No force-push, history rewrite, merge, release, or paid GitHub upgrade is authorized by this CI migration.


### Latest structural audit and legacy runner observation

At migration branch SHA `94534c4e5ba6bba3acbf55f2838660df67ea2b81`, the repository tree audit found all 55 paths required by `tools/check_contract.py`, all 74 concrete documentation-index references resolved, and all CircleCI gate markers present. The Python checker itself has not been executed in this session.

The temporary GitHub Actions safety-net run `43414203198` for the same SHA failed on both Ubuntu and Windows with zero executable steps (`steps: []`). This is runner/infrastructure evidence, not a product test result. CircleCI has no reported status yet for this latest SHA. Do not treat either provider's unstarted jobs as source PASS/FAIL.


### Latest exact-SHA pipeline — 2026-11-13

CircleCI has now started both default product jobs for migration SHA `9776d7bdab72397cb533b052f7d5dfa2c0bad106`:

- Workflow: `cc29f03b-c0f9-45f4-99aa-ab5a40b814c6`.
- Linux job: `9148fb7a-4c9b-49aa-8ab1-3e4997cf08f8`, status PENDING/RUNNING at last poll.
- Windows job: `b18d9d6a-b2c4-4878-bed0-c1f5380ffe02`, status PENDING/RUNNING at last poll.

This is a valid exact-SHA execution trigger, not a PASS. Wait for terminal job states and inspect logs/artifacts before proceeding to release/native opt-in workflows or merging the branch that retires GitHub Actions.


### Windows first-run failure and latest retry

On SHA `9776d7bdab72397cb533b052f7d5dfa2c0bad106`, CircleCI Windows job `b18d9d6a-b2c4-4878-bed0-c1f5380ffe02` failed. The status description supplied failure-report pipeline ID `f84b18b4-80e2-4c21-8a04-ff36deacba81`. The job log is not accessible through the current ChatGPT integrations, so the failed step and root cause remain UNKNOWN. Linux was still pending at last poll for that SHA.

A subsequent documentation/contract-checker update triggered both default jobs again on SHA `a1dd2b53a257a4a977bdf501c276e64f3046bf91`:
- Workflow `94c0cd5e-b92a-45c4-9866-e10707ed7f24`
- Linux job `3f57923e-2155-45e0-9227-b9a2829a4b32` — PENDING at last poll
- Windows job `560f8b62-e133-4534-a892-b85e024bad4b` — PENDING at last poll

Do not infer the Windows failure cause from timing or setup commands. Obtain the CircleCI failure report before changing the Windows provisioning path.


### Current branch head — latest status poll

At SHA `12d4006ecc239e9ae3fb6b3fc191a1d50dd17317`, CircleCI workflow `5f028c06-b3ff-4caf-9f96-b9b422640152` has both default jobs PENDING:
- Linux job `0515fcdb-b3d5-47c6-8c90-d007fe7dac6b`.
- Windows job `d7e7de1d-0eb0-4485-957b-9882422701b3`.

No terminal results are available for this latest SHA. The preceding Windows failure and its inaccessible failure report remain an open blocker.


### Windows executor provisioning finding — 2026-11-13

The Windows job failed on SHA `06a8d54654d671c9b642547eec1607f087434c6f` (workflow `42d15225-5ab1-4ab8-bb64-dbbf208e057b`, job `03c291bc-d501-4ac9-972b-7962d30ae50a`; failure-report pipeline `50b8afe4-3e92-4ae3-827a-89e2097056f8`). The log remains inaccessible, so this is not a proven root-cause diagnosis.

Independent inspection of CircleCI's published Windows image inventory shows Rust/Cargo are not listed as preinstalled. This was a guaranteed provisioning defect in the original job definition, which assumed `rustc` and `cargo` were already on PATH. The config now installs Rust stable explicitly and prefixes the Cargo bin directory in every Windows Cargo step. This correction is justified by the image inventory; do not claim it explains the prior failure until a fresh exact-SHA job passes.


### Executor schema references and guardrails

- Android Gen1 uses `android:default` with job-level `resource_class: large`; CircleCI documents that Gen2 uses `android:current` with a `.gen2` resource class: https://circleci.com/docs/guides/execute/android-machine-image/
- Windows Server 2025 machine executor nests `resource_class: windows.medium` inside the `machine` block, alongside `image` and `shell`, matching CircleCI's published example: https://circleci.com/developer/machine/image/windows-server-2025-gui
- `tools/check_contract.py` enforces this exact Windows machine block for all three Windows jobs, plus explicit Rust/Cargo PATH prefixes in every Windows command block that invokes Rust/Cargo.


### Latest Linux + Windows job failure — 2026-11-13

On SHA `e44d16b2b5f4ef50c33674ccf711bd11b410effa`, both default CircleCI jobs transitioned from pending to failure:
- Workflow: `e5a0662d-023a-4e3d-9174-62a7aa55329a`.
- Linux job: `e87663d5-3843-4078-b155-efbb4c074a8d`.
- Windows job: `5a0d780e-0521-4a90-80da-9293718266de`.
- Failure-report pipeline ID for both: `ac6f1f05-1bbf-4d26-80ea-bf498ba29af3`.

This run used the corrected Android Gen1 image/resource-class pairing, explicit Windows Rust installation/PATH, and nested Windows machine resource classes. Both platforms failing in the same workflow suggests a shared setup/integration/contract issue may exist, but that is not a diagnosis. The exact failing steps are UNKNOWN because CircleCI logs/failure reports are not accessible through the current ChatGPT tools. Do not change more provisioning based on speculation; retrieve the failure report before the next targeted fix.


### Current retry after dual-platform failure

The documentation-only checkpoint at SHA `637ab548210a948f177186eb3a76381330f401f0` triggered another default CircleCI workflow `b8e52e70-281f-4ac7-a898-a21d45ceedd9`:
- Linux job `a26c2b6d-c66e-43b1-8337-108220e5dd4d` — PENDING.
- Windows job `437e36cd-b267-407e-8919-92ce2e1a1a90` — PENDING.

The config itself was unchanged from the preceding dual-platform failure. The static structural audit passes, but the actual Python contract checker and product tests have not yet returned a terminal result. Do not infer the prior shared failure root cause without its CircleCI report.


### CircleCI run after GitHub Actions workflow removal

At migration branch SHA `e728f965165b21c72810c80dd64734912a916324`, the migration branch has no `.github/workflows/` files and CircleCI has triggered the default Linux/Windows matrix:
- Workflow: `1c06b7ae-3464-4c6c-b835-ed0c050af262`.
- Linux job: `55307f51-6de4-4b73-bc26-13f3a40c2589` — PENDING.
- Windows job: `417b51de-5b84-43ac-a566-2e229464a4ff` — PENDING.

This is the first observed default matrix on the branch with GitHub Actions CI removed. It is not a PASS; keep the PR draft until both jobs and the opt-in release/native workflows have terminal green results and their logs/artifacts are inspected.
