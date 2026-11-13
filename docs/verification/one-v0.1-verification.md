# One v0.1 Verification Record

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
- Legacy GitHub Actions workflows remain unchanged as a temporary safety net; their retirement is blocked on exact-SHA CircleCI parity. GitHub remains the source and PR host.


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

CircleCI's published Windows Server 2025 machine-image example nests `resource_class: windows.medium` inside `machine`; all three Windows jobs now match that structure. Android Gen1 remains `android:default` + job-level `resource_class: large`. The static contract checker now enforces the Windows block shape and Rust/Cargo PATH setup.


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
