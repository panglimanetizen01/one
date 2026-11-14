# Project ONE CI Contract

**Canonical CI target:** CircleCI  
**Live migration state at 2026-11-14:** latest observed PR #95 test SHA `9c1873f109512002af5c09789e91dfe5f99615c7`; CircleCI `one-linux` FAILURE (job `09d65e6c-1081-4331-9528-a92b3915dd29`) and `one-windows` FAILURE (job `81733a94-be9e-42be-ad12-b052104f3863`), workflow `ee0e1c19-4899-4e13-bede-38345e931704`, failure report `558caa03-5580-4f31-b621-55a4e118369c`. The first failing step/root causes remain UNKNOWN because authenticated CircleCI logs are unavailable in this session. The docs-only reconciliation that produced this snapshot is followed by another exact-SHA run; PR #95 remains draft/unmerged and `main` unchanged.
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

## Latest exact-SHA status — 2026-11-14

- PR #95 branch `ci/circleci-migration-20261113`, exact tested SHA `9c1873f109512002af5c09789e91dfe5f99615c7`.
- Workflow `ee0e1c19-4899-4e13-bede-38345e931704`: `one-linux` job `09d65e6c-1081-4331-9528-a92b3915dd29` FAILURE; `one-windows` job `81733a94-be9e-42be-ad12-b052104f3863` FAILURE.
- CircleCI failure report `558caa03-5580-4f31-b621-55a4e118369c` is the shared diagnostic target. The first failing step and root causes remain UNKNOWN until authenticated logs/failure report are retrieved. Do not infer a cause or add blanket retries.
- Workflow `7acbf7b4-8dd6-4f90-bf92-5ad243da6845`: `one-windows` job `639baf29-477d-44e9-9287-7907aa241711` FAILURE; `one-linux` status has not yet been reported for this exact SHA.
- The Windows status links to failure report `0ad0f9f7-db81-4b2b-8477-c3e89b546fc1`; the first failing step and root cause remain UNKNOWN until the authenticated CircleCI failure report/log is inspected. Do not infer a cause from the status alone or add blanket retries.
- `.circleci/config.yml` is present on this migration branch; the three product GitHub Actions workflows are deleted on this branch. The source candidate on PR #92 predates this config and therefore reports “No configuration was found”; do not duplicate the config in PR #92.
- The current assistant session exposes GitHub status summaries but not authenticated CircleCI job-log actions/CLI. PR #95 stays OPEN/DRAFT/UNMERGED; main remains unchanged.

## Historical first pipeline observations

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
- Windows Server 2025 machine jobs declare `resource_class: windows.medium` at the job level; `image` and `shell` remain under `machine`. This follows CircleCI's v2.1 job schema and September 21, 2026 strict-compilation guidance: https://circleci.com/docs/reference/configuration-reference/ and https://discuss.circleci.com/t/breaking-changes-config-compilation-updates-september-21-2026/54719
- `tools/check_contract.py` enforces this exact Windows machine block for all three Windows jobs, plus explicit Rust/Cargo PATH prefixes in every Windows command block that invokes Rust/Cargo.


### Latest Linux + Windows job failure — 2026-11-13

On SHA `e44d16b2b5f4ef50c33674ccf711bd11b410effa`, both default CircleCI jobs transitioned from pending to failure:
- Workflow: `e5a0662d-023a-4e3d-9174-62a7aa55329a`.
- Linux job: `e87663d5-3843-4078-b155-efbb4c074a8d`.
- Windows job: `5a0d780e-0521-4a90-80da-9293718266de`.
- Failure-report pipeline ID for both: `ac6f1f05-1bbf-4d26-80ea-bf498ba29af3`.

This run used the corrected Android Gen1 image/resource-class pairing, explicit Windows Rust installation/PATH, and corrected job-level Windows resource classes. Both platforms failing in the same workflow suggests a shared setup/integration/contract issue may exist, but that is not a diagnosis. The exact failing steps are UNKNOWN because CircleCI logs/failure reports are not accessible through the current ChatGPT tools. Do not change more provisioning based on speculation; retrieve the failure report before the next targeted fix.


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


### Linux executor shell contract

The `one-linux` job runs commands in Bash. PowerShell environment syntax such as `$env:PATH` is invalid there and must not be copied from the Windows jobs. The static contract checker rejects PowerShell `$env:` syntax inside the Linux job. Linux uses the executor-provided PATH and the Rustup environment sourced by the setup command through `BASH_ENV`.

 
### Latest Linux shell correction and exact-SHA retry — 2026-11-13

- Previous candidate SHA `ebf6a41f2ab0c71ac2441f41fac96da21375b0b6`: Linux job `4e13ae26-bab1-4850-b513-ee15a69846a6` failed; CircleCI status exposed failure-report pipeline `0cb3e736-20ac-452c-a566-7e7b5a2f33d6`. Windows job `cd106523-748c-4ef7-9702-a8ef81caf06f` was still pending at last poll.
- Static inspection found seven PowerShell `$env:PATH` assignments inside the Bash `one-linux` job. Those assignments were removed. Rustup already adds `$HOME/.cargo/bin` to `BASH_ENV` in the Linux setup command.
- `tools/check_contract.py` now rejects any `$env:` syntax inside the isolated `one-linux` job to prevent this shell mismatch from returning.
- Corrected config commit: `940fa17b6c19ec050a8b8e720fc3046d438247dc`; contract guard: `dc58507b466c803c2d62358566350b370027c10d`.
- Documentation reconciliation commits: `dfa5571f54aefa58d352d41222bac5f5390d05c1`, `6f81120ff75488c2c9edb66e8c4b32294b014a25`, `2095098ea0e97da1c06c3f260f1f95218a06a339`.
- Latest observed candidate SHA before this documentation addendum: `2095098ea0e97da1c06c3f260f1f95218a06a339`; workflow `05a7e587-645d-4809-95a6-97d528c33507`; Linux `54a95e18-180c-40e9-b572-83baedfd4bce` and Windows `e44eb79c-605d-4dd6-8a5e-5ccaad91b06f` were PENDING at last poll.
- The shell mismatch is a proven configuration defect, but the exact failing step for the previous Linux job remains unconfirmed until the failure report is retrieved. Do not claim this correction as the proven root cause without the log.
- No product source, grammar, semantics, compiler, interpreter, runtime, or CLI files changed. The migration branch has no `.github/` files; `main` remains unchanged. Keep PR #95 draft/unmerged until default Linux/Windows, opt-in release/native checks, and artifact inspection pass.


### Windows PE32+ verification boundary

The native Windows job must validate the DOS MZ header, PE signature, AMD64 machine value `0x8664`, and PE32+ optional-header magic `0x20B` before executing the produced binary. The static contract checker requires the PE32+ optional-header guard so the log message cannot overstate what the test actually inspected.

 
### PE32+ native verification hardening — latest source revision

The Windows native artifact inspection now validates the DOS MZ header, PE signature, AMD64 machine field `0x8664`, and PE32+ optional-header magic `0x20B` before execution. The bounds check now requires the optional-header magic bytes to be present. `tools/check_contract.py` requires the optional-header validation guard, preventing the job from printing a PE32+ claim after checking only the machine field.


### Candidate after shell and PE32+ guard corrections

- Current migration candidate before this checkpoint: `5dcb7088910d0520252deffbdacf15caeffe907e`.
- Config includes the Bash-only Linux command correction and a Windows PE32+ optional-header (`0x20B`) validation.
- Static structural audit: 60 required contract paths exist; all 78 concrete docs-index references resolve; migration tree has zero `.github/` paths; Linux job contains no PowerShell `$env:`; Linux shell guard and PE32+ guard are present; all three Windows executor blocks use job-level `windows.medium` resource classes.
- The GitHub status endpoint returned no CircleCI statuses for this candidate at the last poll. This is NOT a PASS or a confirmed trigger; inspect the next exact-SHA status before treating the run as started.

 
### Current exact-SHA retry after shell and PE32+ corrections

At candidate SHA `36a4051913c0063f0237bf41cf4b25ebc250e493`, CircleCI workflow `e42b0304-1241-4327-807d-03926d7d0422` started:
- Linux job `65187706-c4ac-4142-89b0-d1c93d7fa36c` — PENDING.
- Windows job `d51c952e-6484-4f6a-9375-bc57576110c6` — PENDING.

This candidate contains the Bash-only Linux command fix, Linux shell syntax regression guard, and PE32+ optional-header validation plus checker guard. The prior candidate `2095098ea0e97da1c06c3f260f1f95218a06a339` failed on both platforms; failure-report pipeline `991dcbb1-49e9-4b65-acb1-9083caf1f24f`. The exact failed steps remain unknown without the report. Do not claim the latest corrections fixed those failures until the current jobs are terminal and logs are inspected.

 
### Latest CircleCI retry — structural audit and current run

At SHA `54386f415062d4c21a7acb5508d90fea4ba5cdef`, workflow `ca11e9e9-d870-43d8-8ea0-88949a1f3f62`:
- Linux job `d5947530-f1bb-4136-b137-f037a72b99a3` — PENDING.
- Windows job `1739d1cc-b23c-40a6-924b-201616b4af6a` — PENDING.

Structural audit at this exact SHA: all 60 required contract paths present; all 78 concrete docs-index references resolve; 6 valid/6 invalid/3 golden fixtures; all five grammar markers present; all 21 CircleCI markers present; no `$env:` in Linux; Linux shell guard and PE32+ guard present; three Windows machine executor blocks retain image/shell under machine while resource_class remains job-level; no Cargo.lock/`--locked`; no `.github/` paths; release no-upgrade rule preserved. This is a structural mirror, not execution of the Python checker. Prior candidate `2095098...` failed both platforms with failure-report pipeline `991dcbb1-49e9-4b65-acb1-9083caf1f24f`; exact failing steps remain unknown without logs.


### Exact-SHA status refresh — Windows failed, Linux pending — 2026-11-13

At PR #95 HEAD `0d33533fd2c1fef2a4ec648549dbd1fb18b6ba0c`, GitHub now reports:
- Windows job `a5fa0f85-5bdf-49f3-8c1f-6410ee5272d5` — FAILURE. CircleCI status exposes failure-report run ID `24b70353-4030-4a3a-86d2-842ed4dc6bb3`.
- Linux job `5bc064d5-4ee5-4919-a9c5-ea5649064433` — PENDING at the same poll.
- Workflow: `c6d194ab-b1c0-4a70-8361-e7af0ace3650`.

The Windows failure is now terminal, but the first failed step and diagnostic output have not been retrieved. The current agent session exposes GitHub/Notion operations but no authenticated CircleCI CLI, CircleCI MCP connector, or CircleCI API action. CircleCI API endpoints are inaccessible through the available web retrieval channel. Do not infer a cause from the status description, do not make speculative edits, and do not add blanket retries.

Required next step: run `circleci run get 24b70353-4030-4a3a-86d2-842ed4dc6bb3 --failure-report` using an authenticated CircleCI CLI or connected CircleCI CLI MCP, identify the first failing Windows step and its evidence, then make the smallest confirmed fix. Re-poll both platforms against the resulting exact SHA. Keep PR #95 OPEN / DRAFT / UNMERGED; `main` remains unchanged. No CI PASS is claimed.


### MSYS2 Windows CI provisioning — 2026-11-13

The Windows cross-toolchain bootstrap now follows the official MSYS2 CI sequence: launch the extracted base environment once, run two complete `pacman --noconfirm -Syuu` passes (core update followed by the remaining full update), then install the MinGW-w64 Clang/toolchain packages. The two-pass update is required because core package updates can invalidate the running MSYS2 shell; the second invocation completes the upgrade after the first pass. The contract checker requires the shell-initialization command and the full-upgrade command marker so the setup does not regress to a single `pacman -Syu` pass.

This is a source-backed provisioning hardening based on the MSYS2 CI guide. It is not yet proven to be the root cause of the prior Windows job failure because that job's failure log remains unavailable. Verify the next exact-SHA Windows run and inspect its first failed step if it still fails.


### MSYS2 keyring initialization correction — 2026-11-13

The Windows MSYS2 base is provisioned from the self-extracting archive, which unpacks files but does not run the full installer keyring initialization. Initialize the pacman trust database before any repository synchronization by running `pacman-key --init` followed by `pacman-key --populate msys2`, checking each exit code, then run `pacman -Syu --noconfirm` and install the MinGW-w64 toolchain. This addresses the observed failure: missing public keyring, unknown package signing key, and non-writable keyring. The earlier proposed shell-only initialization and two-pass update sequence was not the evidenced fix and has been removed. The precise failure and correction are documented in PR #96, based on the failed Windows job log.

### Windows MSYS2 package-install timeout correction — 2026-11-13

The latest failed `one-windows` log confirms that MSYS2 keyring initialization and package-signature verification succeeded. The `mingw-w64-x86_64-clang` plus `mingw-w64-x86_64-toolchain` transaction then began installing 47 packages (213.49 MiB download; approximately 1.45 GiB installed size) but produced no output for the CircleCI default 10-minute no-output window. CircleCI terminated the step with `Too long with no output (exceeded 10m0s): context deadline exceeded`; the log does not show a package-manager error or a completed transaction.

The MSYS2 installation run step now sets `no_output_timeout: 30m` so the large first-time package transaction can finish without being mistaken for a hung command. The contract checker scopes a regression guard to this exact MSYS2 step and requires the 30-minute setting. This is a targeted timeout correction based on the attached log; it does not claim that the Windows job has passed. The next exact-SHA run must confirm whether the package transaction completes and whether subsequent NDK/build/test steps pass.
