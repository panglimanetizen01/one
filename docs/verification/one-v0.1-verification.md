# One v0.1 Verification Record

## Live CircleCI migration checkpoint — 2026-11-14

- CircleCI is the canonical Project ONE CI provider; GitHub remains repository, version-control, PR, and code-review host. The migration branch deletes the three legacy product GitHub Actions workflows; main is unchanged until reviewed merge.
- PR #95 exact HEAD `e36963a329d7f04e40278043d20f6cc86f314005`: `ci/circleci: one-linux` SUCCESS (job `204f0cc7-d0e7-472f-a4cc-3e92c53158e9`); `ci/circleci: one-windows` FAILURE (job `5128b82c-4b17-4670-8c47-7dbd7f4a124c`); workflow `62dcb9b5-814d-4303-b3e2-a9a0b99f19cf`.
- Windows failure report `6f4e8066-370f-49c3-9cc6-e9734a9a7412` has not been retrieved in this session. First failing step/root cause remain UNKNOWN. Do not claim parity PASS or make speculative changes.


## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Canonical main evidence

Canonical `main` verification evidence remains recorded at its exact SHA and stated platform boundary. This document must not silently replace that evidence with a feature-branch result.

## PR #64 native backend verification

Branch: `feature/native-backends-linux-windows`

Exact verification revision: `797c6fe75bd3fa9081329f73dd59e34278f27ddd`.

This is branch-bound evidence and becomes a canonical main verification record only after PR #64 is merged and the merged SHA is independently reverified.

### Linux AArch64

- Target: `aarch64-unknown-linux-gnu`.
- Native One CLI build succeeded.
- Artifact identified as ELF 64-bit AArch64.
- Artifact executed successfully on the AArch64/Linux verification host and printed `ONE_NATIVE_LINUX_AARCH64`.
- Status: **VERIFIED — exact PR SHA, AArch64/Linux host boundary**.

### Linux x86_64

- Target: `x86_64-unknown-linux-gnu`.
- Regression test `linux_x86_64_build_menggunakan_cross_linker_x86_64` passed.
- Native One CLI build succeeded.
- Artifact identified as ELF 64-bit x86-64 with `/lib64/ld-linux-x86-64.so.2`.
- No x86_64 executor was available.
- Status: **BUILD TERBUKTI — execution pending**.

### Windows x86_64

- Target: `x86_64-pc-windows-gnu`.
- Native artifact build is covered by backend/CLI verification.
- Artifact proven as PE32+ x86-64.
- No Windows/Wine executor was available.
- Status: **BUILD TERBUKTI — execution pending**.

### Native regression/static evidence

- Native backend integration suite: 9/9 passed.
- `cargo clippy -p one-core --all-targets -- -D warnings`: passed.
- Linux x86_64 native CLI build: passed; ELF x86-64 produced.
- `git diff --check`: passed before the fix commit.

## Full-suite boundary

A later full `cargo test` invocation in the Debian/proot verification environment failed in `android_target_explicit_mempertahankan_build_native` because the environment selected Debian host Clang and could not find `bits/libc-header-start.h`. This is environmental evidence outside the bounded Linux x86_64 linker fix and is not evidence that the fix itself failed. The targeted native suite, Clippy, native CLI build, and diff checks had already passed before commit.

## Latest cross-platform historical evidence

Independent mirror run `38929072800` verified revision `9038ac75f197753ff1ab6af7478e63e4831903a2` on Ubuntu and Windows Server 2025. That historical run predates the database, HTTP, filesystem-write, and native-backend revisions and must not be used for those later changes. The later exact-SHA matrix `39554661451` verifies canonical source `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` and includes the current source/core/CLI test suites for filesystem write, database query, and HTTP GET.

## Evidence boundary

`VERIFIED` is reserved for claims backed by executable evidence for the exact recorded source SHA and stated platform boundary. Evidence for one commit does not automatically verify later commits.

## Verification infrastructure status

The public verification mirror remains infrastructure only; GitHub `main` remains the canonical source of truth. Historical canonical hosted CI attempts for affected work produced zero-step job failures, so those runs are classified as infrastructure evidence failure rather than source failure. No paid GitHub upgrade was used or required.


## CircleCI migration checkpoint — 2026-11-13

- Migration branch: `ci/circleci-migration-20261113`.
- CircleCI configuration introduction commit: `3157333c8618cb211ffebd00f983f9c49900d0cb`.
- Linux cross-toolchain hardening commit: `1b110a8aeb76b5c836657a96ee7454d4b165e475`.
- Cargo cache/lockfile correction commit: `a17ade8c2808c17ca67749e59d0ff01e61869890`.
- The verified `main` tree does not track `Cargo.lock`; CircleCI cache keys therefore hash the root/workspace Cargo manifests and `rust-toolchain.toml`, and native verification omits `--locked` to avoid a guaranteed lockfile precondition failure.
- The config defines default Linux and Windows product CI, opt-in Linux/Windows release checks, and opt-in Windows native PE build/inspection/execution with stored artifact.
- `tools/check_contract.py` now requires `.circleci/config.yml` and `docs/CI.md`, and statically checks that the config retains the named product/release/native gates.
- CircleCI project integration, remote config validation, pipeline/job execution, exact-SHA PASS results, logs, and artifacts remain **NOT VERIFIED** in this checkpoint. The ChatGPT tool session currently exposes no authenticated CircleCI API/CLI action.
- The three legacy GitHub Actions CI workflows are deleted from the migration branch; `main` remains unchanged until CircleCI exact-SHA parity is proven and the reviewed PR is merged. GitHub remains the source and PR host.


### CircleCI first-run failure evidence

- Commit `a213165cb175355c73412fe0787d11e6e0e9807f`: GitHub status `ci/circleci: one-linux` = FAILURE; CircleCI workflow `c1a9d9ef-305d-4347-8031-53f59cb48244`, job `7e50636b-ebbd-4bb2-8680-be056a091ce7`.
- Commit `17b81146dc610a44a369e29b36551f46e717ea9e`: GitHub status `ci/circleci: one-linux` = FAILURE; CircleCI workflow `e02c1358-fb93-498c-a6b6-39e3a515231a`, job `539c5580-e078-4f76-8e8d-79ed9bfcffda`.
- CircleCI supplied the CLI failure-report hint, but job logs were inaccessible through the available integrations. The exact failure step and root cause therefore remain UNKNOWN; do not label either failure as a Rust/product defect without logs.
- Follow-up changes bootstrap Rust on the Android executor, install the GNU x86_64 linker explicitly, use the Android Gen1 `default` image with the `large` resource class, and avoid Cargo `--locked` while the repository has no lockfile.
- Latest migration branch SHA at this update: `6af3ff690fe85a47dd94cb4deb6b749f7b8da4c9`. No PASS is claimed for this SHA; the GitHub status endpoint had not yet returned a CircleCI status for it.


### Latest structural audit — 2026-11-13

At migration branch SHA `94534c4e5ba6bba3acbf55f2838660df67ea2b81`:
- Recursive Git tree contains 235 entries.
- All 55 paths in the `tools/check_contract.py` required-artifact list exist.
- All 74 concrete documentation-index references resolve to repository paths.
- All 15 required CircleCI job/gate markers are present in `.circleci/config.yml`.
- `Cargo.lock` is absent, and `--locked` is absent from CircleCI config.
- The Python checker was not executed locally; this is a structural source/tree audit only.

Temporary GitHub Actions run `43414203198` for SHA `94534c4e5ba6bba3acbf55f2838660df67ea2b81` failed on Ubuntu and Windows with zero executable steps. Treat it as runner/infrastructure evidence only. CircleCI had no reported status for this latest SHA at the time of the audit.


### CircleCI latest default matrix — pending

At SHA `9776d7bdab72397cb533b052f7d5dfa2c0bad106`, CircleCI workflow `cc29f03b-c0f9-45f4-99aa-ab5a40b814c6` started both default jobs:
- Linux: `9148fb7a-4c9b-49aa-8ab1-3e4997cf08f8` — PENDING/RUNNING at last poll.
- Windows: `b18d9d6a-b2c4-4878-bed0-c1f5380ffe02` — PENDING/RUNNING at last poll.

No terminal result or PASS is recorded yet.


### CircleCI Windows failure and latest retry

- SHA `9776d7bdab72397cb533b052f7d5dfa2c0bad106`: Windows job `b18d9d6a-b2c4-4878-bed0-c1f5380ffe02` = FAILURE; failure-report pipeline ID `f84b18b4-80e2-4c21-8a04-ff36deacba81`. Job logs are inaccessible through the current integrations; exact failing step/root cause UNKNOWN. Linux remained pending at last poll.
- Latest branch SHA at this update: `a1dd2b53a257a4a977bdf501c276e64f3046bf91`, workflow `94c0cd5e-b92a-45c4-9866-e10707ed7f24`: Linux job `3f57923e-2155-45e0-9227-b9a2829a4b32` and Windows job `560f8b62-e133-4534-a892-b85e024bad4b` were pending at last poll.
- Do not infer the cause; obtain the CircleCI failure report before changing Windows provisioning.


### Current branch head status — 2026-11-13

At SHA `12d4006ecc239e9ae3fb6b3fc191a1d50dd17317`, CircleCI workflow `5f028c06-b3ff-4caf-9f96-b9b422640152` reports:
- Linux job `0515fcdb-b3d5-47c6-8c90-d007fe7dac6b` — PENDING.
- Windows job `d7e7de1d-0eb0-4485-957b-9882422701b3` — PENDING.

No terminal result/PASS is recorded for the current branch head.


### Windows Rust provisioning correction

- SHA `06a8d54654d671c9b642547eec1607f087434c6f`: Windows job `03c291bc-d501-4ac9-972b-7962d30ae50a` failed; failure-report pipeline `50b8afe4-3e92-4ae3-827a-89e2097056f8`. Job logs remain inaccessible; exact failing step UNKNOWN.
- CircleCI's published Windows Server 2025 image inventory does not list Rust/Cargo. The job had assumed `rustc`/ `cargo` were preinstalled. The configuration now bootstraps stable Rust via rustup and prepends `%USERPROFILE%\\.cargo\\bin` for each Windows Cargo invocation.
- This is a proven missing prerequisite in the configuration, but not a proven diagnosis of the earlier job failure without its log. Verify the fix with a fresh exact-SHA CircleCI run.


### Windows PATH contract regression guard

`tools/check_contract.py` now inspects the Windows provisioning, default CI, release, and native-verification command blocks. Any `cargo`, `rustc`, or `rustup` invocation without the explicit user Cargo-bin PATH prefix is a contract-check failure. This guards against future Windows jobs silently depending on an image-level Rust installation.


### CircleCI executor schema correction

CircleCI's v2.1 job schema requires `resource_class: windows.medium` at the job level, outside `machine`; all three Windows jobs now follow that structure. The September 21, 2026 strict-compilation guidance explicitly rejects nesting `resource_class` inside `machine`. Android Gen1 remains `android:default` + job-level `resource_class: large`. The static contract checker now enforces the Windows block shape and Rust/Cargo PATH setup.


### Latest dual-platform CircleCI failure — 2026-11-13

SHA `e44d16b2b5f4ef50c33674ccf711bd11b410effa`, workflow `e5a0662d-023a-4e3d-9174-62a7aa55329a`:
- Linux job `e87663d5-3843-4078-b155-efbb4c074a8d` — FAILURE.
- Windows job `5a0d780e-0521-4a90-80da-9293718266de` — FAILURE.
- Failure-report pipeline ID: `ac6f1f05-1bbf-4d26-80ea-bf498ba29af3`.

Both jobs failed after the Windows Rust bootstrap and executor-schema corrections were present. The exact failing step/root cause is UNKNOWN because the job logs are inaccessible. No product source failure is inferred. Further changes must be evidence-led from the failure report.


### Current CircleCI retry — 2026-11-13

SHA `637ab548210a948f177186eb3a76381330f401f0`, workflow `b8e52e70-281f-4ac7-a898-a21d45ceedd9`:
- Linux job `a26c2b6d-c66e-43b1-8337-108220e5dd4d` — PENDING.
- Windows job `437e36cd-b267-407e-8919-92ce2e1a1a90` — PENDING.

The config did not change after the preceding dual-platform failure; this retry follows documentation/verification record updates only.


### CircleCI matrix after GitHub Actions removal

At SHA `e728f965165b21c72810c80dd64734912a916324`, the migration branch contains no `.github/workflows/` files. CircleCI workflow `1c06b7ae-3464-4c6c-b835-ed0c050af262`:
- Linux `55307f51-6de4-4b73-bc26-13f3a40c2589` — PENDING.
- Windows `417b51de-5b84-43ac-a566-2e229464a4ff` — PENDING.

This is the first default Linux/Windows matrix observed after workflow retirement in the migration branch. No PASS is claimed.

 
### Linux shell mismatch remediation and retry — 2026-11-13

- SHA `ebf6a41f2ab0c71ac2441f41fac96da21375b0b6`: Linux job `4e13ae26-bab1-4850-b513-ee15a69846a6` failed with CircleCI failure-report pipeline `0cb3e736-20ac-452c-a566-7e7b5a2f33d6`; Windows job `cd106523-748c-4ef7-9702-a8ef81caf06f` was pending at last poll.
- Static inspection found seven PowerShell `$env:PATH` statements in the Bash Linux job. They were removed. The contract checker now rejects PowerShell `$env:` syntax in the Linux job.
- Corrected candidate `2095098ea0e97da1c06c3f260f1f95218a06a339`, workflow `05a7e587-645d-4809-95a6-97d528c33507`: Linux `54a95e18-180c-40e9-b572-83baedfd4bce` and Windows `e44eb79c-605d-4dd6-8a5e-5ccaad91b06f` were pending at last poll. No PASS is claimed.
- The syntax defect is independently proven from the config; its status as the actual failed CircleCI step is not confirmed without the failure report.

 
### PE32+ native artifact inspection hardening — 2026-11-13

The Windows native verification now checks the DOS MZ header, PE signature, AMD64 machine field `0x8664`, and PE32+ optional-header magic `0x20B`; the bounds check ensures both optional-header bytes are present. The static contract checker requires this PE32+ guard. This strengthens the test contract but is not execution evidence.

 
### Exact-SHA retry after Linux shell and PE32+ corrections — 2026-11-13

Candidate `36a4051913c0063f0237bf41cf4b25ebc250e493`, workflow `e42b0304-1241-4327-807d-03926d7d0422`:
- Linux `65187706-c4ac-4142-89b0-d1c93d7fa36c` — PENDING.
- Windows `d51c952e-6484-4f6a-9375-bc57576110c6` — PENDING.

The candidate contains the Linux Bash shell fix and the PE32+ optional-header guard. The preceding candidate `2095098ea0e97da1c06c3f260f1f95218a06a339` failed on both platforms; failure-report pipeline `991dcbb1-49e9-4b65-acb1-9083caf1f24f`. Exact failing steps remain unknown without logs. No PASS is claimed.

 
### Latest exact-SHA CircleCI retry — 2026-11-13

SHA `54386f415062d4c21a7acb5508d90fea4ba5cdef`, workflow `ca11e9e9-d870-43d8-8ea0-88949a1f3f62`:
- Linux `d5947530-f1bb-4136-b137-f037a72b99a3` — PENDING.
- Windows `1739d1cc-b23c-40a6-924b-201616b4af6a` — PENDING.

Structural audit: 60 required paths and 78 docs-index refs resolve; fixtures 6/6/3; five grammar markers; all 21 CI markers; Linux Bash syntax guard; Windows PE32+ optional-header guard; three correct Windows executor blocks; zero `.github/` files. This is not a CircleCI PASS. Prior candidate `2095098...` failed both platforms; failure-report pipeline `991dcbb1-49e9-4b65-acb1-9083caf1f24f`, exact failed steps unknown.
