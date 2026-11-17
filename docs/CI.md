# Project ONE CI Contract


## Current exact-SHA CI candidate checkpoint — 2026-11-17

- PR #95 branch `ci/circleci-migration-20261113` is OPEN / DRAFT / UNMERGED. Current source HEAD at this checkpoint: `9b233dfe75da8c0752c203af3711bf3da319dc02`; base `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`.
- The executable CI configuration candidate, including the APT mirror mitigation and corrected shell escaping, is exact SHA `4db76fbaf651dd069c7000fb46a4dea61b11fbd8`. CircleCI `ci/circleci: one-linux` is terminal SUCCESS on that exact SHA (workflow `d436555e-6a03-4167-a844-acc2ac5e50d6`, job `b8478a26-15f6-457c-b8e4-f30e461e17e9`). This verifies the APT mitigation on that code/config snapshot.
- The current PR HEAD `9b233dfe75da8c0752c203af3711bf3da319dc02` is a documentation-only descendant of `4db76fb...`; compare reports exactly one changed path, `docs/CI.md` (7 additions, 0 deletions). CircleCI on this current HEAD is terminal FAILURE (workflow `91d704b2-516e-4736-9fb2-d4c8504e001b`, job `948e41bd-2090-4708-8b1b-1dc142672580`). The authenticated job page/logs are not accessible through the current integrations, so the first failing step and root cause remain UNKNOWN. Do not attribute this failure to APT, docs, or product code without the job output. Current PR status is not green.
- PR #95 now tracks `Cargo.lock`, includes the target-specific Unix `libc` and Windows `windows-sys` dependencies, uses `--locked` for all CircleCI Cargo build/Clippy/test commands, and enforces those requirements in `tools/check_contract.py`. The package set and product-language behavior are unchanged by the APT fix.
- Independent Windows verifier PR #42 is OPEN / DRAFT / UNMERGED and its canonical source pin is `9b233d...` (pin commit `e88b2be4e0845f932cd978e26305d25f9730f777`; docs commit `af52f1ab4897f8a464b787fa895ba2051f311372`). No Windows run has been observed for this exact source pin. Historical Windows run #32 applies only to source SHA `a10a69...` and is not reused.
- CircleCI Linux PASS on `4db76fb...` and current-head FAILURE on `9b233d...` are distinct exact-SHA outcomes. Do not combine them into a current PR PASS. Migration acceptance remains NOT PASSED until the latest candidate has terminal CircleCI success and the independent Windows verifier succeeds on the same exact source SHA.
- The opt-in `release-check` workflow has not been executed. `main` remains unchanged with its existing GitHub Actions workflows; the CircleCI migration is not active on `main`. No merge, tag, or release is authorized.

## Provider boundary

- GitHub remains the canonical source repository, version-control host, pull-request system, and code-review surface.
- CircleCI is the canonical CI execution provider for the ONE repository's Linux product checks and Linux release checks.
- Windows-host product/native verification is owned by the independent `panglimanetizen01/one-windows-verification` repository. It checks out the canonical ONE repository at a full, explicit source SHA and records the verifier SHA separately.
- The CircleCI `one-windows`, `release-windows`, and `windows-native-verification` jobs, their Windows provisioning command, and the `run_windows_native_verification` parameter are retired from the canonical configuration.
- The Linux CircleCI job retains MinGW-w64 as a Linux-host cross-toolchain for the existing Windows-target backend build regression. That is cross-compilation on Linux, not execution or host verification on Windows.
- The standalone `ci-local` extraction is a separate completed project and is not imported, invoked, or recreated by this configuration.
- The migration remains on PR #95 and must remain unmerged until the exact-SHA Linux CircleCI result and the independent Windows exact-SHA result are both inspected. The `main` branch is unchanged.

## Default product workflow

The `one-ci` workflow runs only `one-linux` on repository-triggered pipelines. It:

1. Verifies Rust, Cargo, and Python toolchains.
2. Provisions Android NDK r27c, Android AArch64 Clang, and the Linux-host MinGW-w64 cross-toolchain used by Windows-target backend tests.
3. Runs `python3 tools/check_contract.py`.
4. Builds all workspace targets.
5. Runs Clippy with warnings denied.
6. Runs all workspace/all-target tests.
7. Retains deterministic `.oneir`/`.onepkg`, DNS, orchestration, database, HTTP GET, and HTTP GET CLI regression gates.

The Linux executor uses CircleCI's Android Gen1 machine image tag `android:default` with the `large` resource class. It bootstraps stable Rust through rustup and installs the pinned NDK revision `27.2.12479018`. The Linux job also installs MinGW-w64 so its Windows-target native-backend regression can cross-compile from Linux; it does not replace the independent Windows-host verification.

## Opt-in Linux release workflow

The `run_release_checks` parameter defaults to `false`. When enabled, `release-check` runs only `release-linux`, including workspace build/tests, real-network runtime tests, and `tools/verify_offline.sh`.

There is no Windows release job in CircleCI. Windows-host build/runtime evidence is recorded by the independent verifier. The Windows-specific Rust/MinGW/NDK provisioning formerly performed by CircleCI has moved to that verifier workflow.

## Independent Windows exact-SHA verification

- Repository: `panglimanetizen01/one-windows-verification`.
- Review branch: `verify/windows-pr95-20261114`, PR #42 (draft/unmerged while under review).
- Source pin: `WINDOWS_SOURCE_COMMIT.txt`, containing a full 40-character canonical ONE commit SHA.
- Workflow: `.github/workflows/windows-native-backend-runtime.yml`, on Windows Server 2025 AMD64.
- Read access: the canonical ONE repository is PUBLIC, so no separate `CANONICAL_READ_TOKEN` secret is needed. Exact-source and verifier-harness checkouts set `persist-credentials: false`; source build/test code does not receive a persisted checkout credential.
- Identity assertions: the workflow checks `git -C source rev-parse HEAD` against the source pin and independently checks its own verifier checkout SHA.
- Required checks: source contracts/user documentation, stable Rust, MinGW-w64 Clang/linker, pinned Android NDK r27c, full workspace build, Clippy, full workspace tests, deterministic artifacts, DNS/orchestration/database/HTTP vertical slices, real-network tests, offline verification, Windows native build, PE32+ AMD64 inspection, and native execution.
- Evidence: the workflow writes `one-windows-native-runtime-manifest.txt` with `source_sha`, `verifier_sha`, run identity, host/toolchain details, binary SHA-256, and native runtime output; it uploads that manifest and the executable.

A Windows PASS requires a terminal-success run for the exact source SHA, successful native build/PE32+ inspection/execution, matching source and verifier identities in the manifest, and an independently matching executable SHA-256. A queued workflow, pin file, successful Linux CircleCI job, or older source SHA is not a Windows PASS.

### Previous exact-SHA provider-split evidence — run #10, 2026-11-14

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

### Demonstrated independent verifier baseline — before this CircleCI split

Verifier run [#9](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43589646641) completed SUCCESS on 2026-11-14 for canonical source SHA `195755054bb733f44ff30acf8d3281b4efc820bf`. Its manifest recorded verifier checkout SHA `d2ee1902f343108a4b87f586921e7799608b1ff1`, Windows Server 2025 / AMD64, native exit code `0`, and output `Budi`. Artifact ID `14111933646` had GitHub ZIP digest `sha256:baebbed5fd822fdd35748e03a22dbbddbe7ad6c15d735ce3108718c9616055c2`; the downloaded ZIP independently matched that digest, and the embedded executable SHA-256 matched the manifest (`257aebe8f2bdd790064578faae1d41ece4d62fd73bda3d0ffd1c2fdff7bcaf3d`).

This is baseline evidence for source SHA `195755...` only. It does not verify any later PR #95 commit. Each later source SHA requires a new independent Windows run and its own manifest/artifact inspection.

## Exact-SHA evidence requirements

- CircleCI must reach terminal SUCCESS for the exact candidate SHA on `one-linux`; inspect its executable steps and artifacts.
- The independent Windows workflow must reach terminal SUCCESS for the same exact source SHA; inspect its manifest and binary hash.
- Release approval remains a separate project-level decision. A successful Linux release check or Windows verification does not automatically authorize a release.
- Keep source SHA, verifier SHA, run ID/attempt, platform, command/check set, artifact digest, and terminal result bound together.
- Evidence for one commit never transfers automatically to another commit.

## Failure handling

- Preserve CircleCI logs/artifacts and independent Windows logs/manifests/artifacts.
- Treat a job that never starts or has no executable steps as infrastructure failure/inconclusive, not proof of a source failure.
- Fix only evidenced failures and add focused regression checks.
- Do not weaken language semantics, security/capability rules, or product tests to make CI green.

## Migration safety boundary

The PR #95 migration branch must not reintroduce a Windows executor into canonical CircleCI or restore the retired GitHub Actions CI workflows. If CircleCI Linux cannot start or pass, keep PR #95 unmerged; `main` remains unchanged. If the independent Windows workflow cannot run, do not claim Windows verification. No force-push, history rewrite, merge, release, paid GitHub upgrade, or repository visibility change is authorized by this migration.

> The historical event log below includes old CircleCI Windows jobs. Those entries describe earlier candidates only; the current provider contract above supersedes them.

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


### Latest independent Windows exact-SHA evidence — run #19 — 2026-11-15

- Windows run [#19](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43666673348) completed terminal SUCCESS on Windows Server 2025 / AMD64; job `132668616929`.
- Exact canonical source SHA: `f85375e279257dcbb7d4ded9632800fcd1b78cc9`; verifier branch HEAD at trigger: `89700258033c16bf6bad7ef983b0f442703d5a4c`; manifest verifier merge-ref SHA: `40b7087bbd30dbf9c30cfec2c1d4af94e2796505`.
- All 28 substantive checks succeeded: source/verifier identity, Rust/Clang/NDK provisioning, contracts/docs, workspace build, Clippy, full tests, deterministic package artifacts, DNS/orchestration/database/HTTP, real-network/offline behavior, Windows native build, PE32+ AMD64 inspection, execution, manifest and artifact upload.
- Manifest: target `x86_64-pc-windows-gnu`, exit code `0`, output `Budi`; executable SHA-256 `ef996ba9a1229afca36fcf9767042a1eb0032fc44b279588dfb5de59d74e3d34`.
- Artifact ID `14142329450`; ZIP SHA-256 `1c51ca84a1396d25585d30a65295821ce5c8ac545cf8e29787e16bae64f53a0e`. Downloaded artifact digest independently matched GitHub's digest; embedded manifest and executable hash were independently inspected and matched.
- This result verifies source SHA `f85375e...` only. The later PR #95 HEAD `5d50f42...` remains unverified on Windows by this run.
- CircleCI Linux on PR #95 HEAD `5d50f42...` remains FAILURE; workflow `d98d94fa-cb59-4487-84e9-582d889f6143`, job `b4427cc0-8253-4d95-8b5d-51f2f2778ca6`. Root cause remains unknown until the first-failure output is retrieved. Overall cross-platform result remains NOT PASS.


### Latest independent Windows exact-SHA verification — run #21 — 2026-11-15

- Windows run [#21](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43667599852) completed terminal SUCCESS on Windows Server 2025 / AMD64; job `132671502711`.
- Exact canonical source SHA tested: `f85375e279257dcbb7d4ded9632800fcd1b78cc9`; verifier branch HEAD at trigger: `9c40fffdaca6125561320a16475515e82923e9e8`; manifest verifier SHA: `32c5b29e36f3b3e07c1eabf5df20874d397eb048`.
- All 28 substantive checks succeeded, including workspace build, Clippy with warnings denied, complete workspace tests, deterministic artifacts, DNS/orchestration/database/HTTP/CLI HTTP, real-network/offline behavior, Windows native build, PE32+ AMD64 inspection, native execution, manifest, and upload.
- Native execution returned exit code `0`, output `Budi`, target `x86_64-pc-windows-gnu`. Artifact ID `14143565311`; ZIP SHA-256 `1f9a6d8143cbde469efd39d8fb3173188ea9adc2d8e6272d14a3aa3110865242`; executable SHA-256 `72c0e947d9c9250b6fff2af611b2e368ce3e543e1aed9679224418df7d767491`; manifest SHA-256 `c4605e9da570d73a9b501ee8d9ff5f6680434b43128e4889c11b0baa20baf61a`. The downloaded ZIP digest was independently recomputed; the executable header is PE32+ AMD64.
- This run tests source SHA `f85375e...`. Current PR #95 HEAD `cab441c45ade081c429476a5a432b91f35246399` is two commits ahead, and the compare shows only documentation paths changed; no product source or CircleCI config changed between those SHAs. The verifier run therefore exercises the same code/config tree, but it does not validate the later documentation tree at `cab441c...`.
- CircleCI on current PR #95 HEAD `cab441c...` remains `ci/circleci: one-linux = FAILURE`; workflow `5b7d5721-7975-46d5-bec0-f01d90a8196a`, job `5b937529-aa87-4228-9fd4-79e4bf7507cc`. The user-retrieved API response exposed only `Task information unavailable` and a pre-signed output URL returning HTTP 404. First failing step/root cause remain UNKNOWN. No speculative code/config fix is justified.
- Result: Windows exact-SHA PASS for `f85375e...`; Linux FAILURE for current PR head; overall cross-platform verification remains NOT PASS. PR #95 and PR #42 remain OPEN / DRAFT / UNMERGED; `main` unchanged.


## Current CircleCI lockfile integration candidate — 2026-11-17

- PR #95 latest source candidate: `e6bfb6bb0256dbd30710ceb544cc0d776e2927bf` (OPEN / DRAFT / UNMERGED).
- The candidate now tracks the workspace `Cargo.lock`, includes the target-specific `libc` and `windows-sys` dependencies required by `one-core`, and invokes every CircleCI Cargo build, Clippy, and test command with `--locked`.
- `tools/check_contract.py` now requires `Cargo.lock` and checks inline as well as multiline Cargo invocations for the `--locked` flag.
- CircleCI `ci/circleci: one-linux` for this exact SHA is PENDING (workflow `ca520794-8456-4c2d-af0b-a7f85baaea83`, job `9cd8432e-89c8-4867-b661-64096ab6920f`). The failing step from earlier CircleCI attempts remains NOT VERIFIED because job logs are not accessible in the current integration context. Do not infer that this integration change is the earlier failure's root cause.
- Independent Windows native verification run #32 is PASS only for source SHA `a10a69afdf8675e703f38f1620abc935d8361897`; it does not verify this new candidate. A fresh Windows exact-SHA run is required.
- No local Cargo execution is claimed. Revalidate this exact candidate with CircleCI and the independent Windows verifier before promoting any migration gate.


## CircleCI terminal result for lockfile candidate — 2026-11-17

- PR #95 source SHA `c71d81a60f7792ed8536ba2b70464b9922f33c18`: CircleCI `ci/circleci: one-linux` is terminal FAILURE, workflow `07e89069-4b33-49f6-b32c-8e70519f5221`, job `781e65c4-e43d-4e55-89d3-beaa61623629`.
- The current session has no CircleCI CLI installed, no CircleCI token in its environment, no CLI auth config, and no connected CircleCI job-log integration. The CircleCI job page is also inaccessible through the available web reader. Therefore the first failing step and root cause remain NOT VERIFIED.
- Do not infer that the lockfile/platform-dependency integration caused this failure. It is a confirmed integration correction, but failure attribution requires the job log.
- The next candidate must be diagnosed from authenticated CircleCI logs before any speculative config/code change or retry. No broad retries are introduced.


### CircleCI Ubuntu APT mirror 404 diagnosis and mitigation — 2026-11-17

- User-retrieved authenticated CircleCI job output for PR #95 code/config candidate `c71d81a60f7792ed8536ba2b70464b9922f33c18` identifies the first failing step as cross-toolchain provisioning (step index 103): `apt-get install` exited 100 after multiple Ubuntu Noble package URLs on `us-east1.gce.archive.ubuntu.com` returned HTTP 404. The failing package set includes MinGW i686/x86_64 runtime/compiler packages and MinGW binutils; other packages downloaded successfully. This is an APT mirror/index-to-pool inconsistency, not a Rust build/test failure.
- The shared `setup-linux-cross-toolchains` CircleCI command now rewrites the affected regional GCE Ubuntu archive hostname to `archive.ubuntu.com` in APT `.sources`/`.list` files, cleans cached APT metadata, refreshes indexes with `Acquire::Retries=3`, and installs the same required toolchains with transport retries. The package set and product code/tests are unchanged.
- Config fix commit: `481873f277cb8601fc8881f8e5cc3fb0383f9115`. This is a targeted mitigation based on the observed 404 evidence; it is not yet a verified fix until the CircleCI run for the resulting exact SHA completes successfully.
- Keep migration acceptance NOT PASSED until CircleCI terminal success and independent Windows exact-SHA verification are both recorded.


### APT mirror rewrite shell-quoting correction — 2026-11-17

- Static review of the first APT mirror mitigation found that the generated CircleCI YAML contained doubled backslashes around the `find` expression and line continuation. The shell command was corrected to use single shell escapes; this is a configuration syntax correction, not a new product/toolchain change.
- Correction commit: `0dbc23dd47243a7b2ae5736c32943e3014353d4f`. The APT mirror rewrite now uses `find ... \( ... \) \\ ` with valid single-character shell escapes and the existing canonical archive, cache refresh, and retry behavior.
- PR #95 exact HEAD is now `0dbc23dd47243a7b2ae5736c32943e3014353d4f`. No CircleCI status had yet been published at the time of this documentation update; do not claim the correction or mirror mitigation has passed until the exact-SHA run completes.


### CircleCI rerun after APT shell correction — exact-SHA pending — 2026-11-17

- Exact PR #95 HEAD: `4db76fbaf651dd069c7000fb46a4dea61b11fbd8`.
- CircleCI `ci/circleci: one-linux` has started and is PENDING for this exact SHA: workflow `d436555e-6a03-4167-a844-acc2ac5e50d6`, job `b8478a26-15f6-457c-b8e4-f30e461e17e9`.
- This run is the first verification attempt after correcting the doubled shell escapes in the APT source-rewrite command. No terminal outcome is available yet; do not claim the mirror mitigation passed.


## Superseding live migration evidence — 2026-11-17

This addendum supersedes older candidate-status statements above where they conflict; historical run records remain unchanged.

- PR #95 source HEAD at this evidence boundary: `cef1f41c97585b6e25bf91af6e8b4281efe299c4` (OPEN / DRAFT / UNMERGED).
- CircleCI context `ci/circleci: one-linux` is terminal SUCCESS for that exact SHA: workflow `930378d8-49bb-416e-a42c-63e1797b991c`, job `0e2642b0-bd94-4892-baca-df5b6c8fa4ed`. The connected integration exposes the exact-SHA commit status but not step-level CircleCI logs; no unobserved step details are claimed.
- Independent verifier PR #42 has synchronized `WINDOWS_SOURCE_COMMIT.txt` to `cef1f41...` (pin commit `f5113c084fa978f275bc8426a4ca1e4d74376549`; verifier PR head `44b36a1894382cc2508edf0166f60b06c9434d7a`). No workflow run is evidenced for this exact source SHA. Windows verification is therefore NOT VERIFIED. Historical run #32 is PASS only for source `a10a69afdf8675e703f38f1620abc935d8361897` and must not be reused.
- The optional CircleCI `release-check` workflow with `run_release_checks=true` has not been proven to have run. Release acceptance remains NOT PASSED.
- Main remains PUBLIC at exact SHA `d35db95153f6ddd79885494c72f5aee16c899751`; PR #95 is not merged, so CircleCI migration is not active on main.
- No merge or release is authorized or performed.

A later documentation-only commit changes the PR head SHA. Treat the evidence above as bound to `cef1f41...`; do not silently transfer the status to a descendant commit. If this addendum is committed as a new PR head, refresh CircleCI status for that new head and keep Windows exact-SHA status NOT VERIFIED until a matching run exists.
