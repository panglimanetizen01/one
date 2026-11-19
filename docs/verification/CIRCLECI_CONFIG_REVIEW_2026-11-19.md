# CircleCI Configuration Review — 2026-11-19

## Scope and evidence

- Repository: `panglimanetizen01/one`
- Reviewed branch: `security/issue-104-root-confinement-20261118`
- Reviewed source HEAD: `7d4268d8cf713a6d6c3bd5ac533b7396d22b7361`
- PR: [#113](https://github.com/panglimanetizen01/one/pull/113), OPEN / DRAFT / UNMERGED
- Config path: `.circleci/config.yml`
- Config blob SHA: `165c78c2f47722913d3fff97b5d928806f7ad42e`
- Exact-HEAD GitHub status: `ci/circleci: one-linux` = FAILURE
- Job: [CircleCI one-linux](https://app.circleci.com/workflow/7a53f3ea-99f3-4bf8-827b-1fd203236b8b/job/0adc6a90-9706-4c90-bd02-650edd74ab90)

The CircleCI job page/log endpoint is not accessible through the connected tools in this session. Therefore the first failing step and root cause for the reviewed HEAD are UNKNOWN. No config change is claimed, and no provider-side validation or rerun was performed.

## Current configuration strengths

- Uses `version: 2.1` and a named reusable command for Linux cross-toolchain setup.
- Pins the Rust release through `rust-toolchain.toml` and installs the matching rustfmt/clippy components.
- Uses `Cargo.lock` and `--locked` in build/test/release commands.
- Cargo cache key includes architecture, workspace manifests, lockfile, and Rust toolchain manifest.
- Diagnostic unit tests run before expensive toolchain provisioning.
- Release-only network and offline checks are gated by a pipeline parameter.
- Focused diagnostics run only after an earlier failure rather than duplicating the full suite on every successful build.
- The APT install and NDK install have explicit command deadlines and CircleCI no-output guards.

## Improvement recommendations

### P1 — Resolve the exact current failure before changing timeouts or retries

The latest exact-HEAD check is FAILURE, but the current provider log is unavailable. Earlier diagnostic notes record an APT install timeout on older SHAs; that history is not proof that the current HEAD failed at the same step. Retrieve the current job's failed-step metadata and stdout/stderr, verify its pipeline revision equals the reviewed SHA, then patch only the proven cause. Do not add blanket retries.

### P1 — Make the Android machine image reproducible

The job currently uses mutable tag `android:default`. Pin a dated CircleCI Android machine image after confirming it contains the required base tools, then keep the project-required NDK version explicitly pinned. This avoids silent base-image/toolchain drift. CircleCI publishes dated Android image tags and documents the image's preinstalled SDK/NDK contents:
- https://circleci.com/developer/machine/image/android
- https://circleci.com/docs/guides/execute/android-machine-image/

Do not silently replace the project's explicit NDK 27.2.12479018 requirement with the image's preinstalled NDK version.

### P2 — Avoid repeating Rust toolchain installation on every cold job

The current cache stores Cargo registry and Git dependency data, but not the pinned Rust toolchain installed by rustup. Measure toolchain-install duration, then consider caching only the exact toolchain directory under a key containing architecture and the exact `rust-toolchain.toml` checksum. Keep the install-and-version checks as a cache-miss fallback. Do not cache mutable rustup configuration or rely on the cache for correctness.

### P2 — Improve dependency-cache population on failed builds

The current `save_cache` is the final step. A job that fails during build/test never reaches it, so a cold cache can remain cold across repeated diagnostic attempts. If logs confirm dependency download is material, add an explicit `cargo fetch --locked` after cache restore and save the dependency cache immediately after that command succeeds. Keep cache keys immutable and lockfile-based. CircleCI recommends restoring before install and saving after successful dependency installation:
- https://circleci.com/docs/guides/optimize/caching/

### P2 — Measure whether all cross-toolchains belong in the default job

The job installs MinGW and NDK before the normal build/test suite, while the visible standard commands are host `cargo build/test --workspace --all-targets`. Confirm from build scripts, target-specific tests, and the failed logs which commands actually consume `ONE_WINDOWS_CLANG`, MinGW, and `ONE_ANDROID_CLANG`. If a toolchain is only needed by a distinct cross-target check, isolate that check into an explicit job or conditional stage without reducing platform coverage. Do not remove either toolchain until its consumers are proven absent.

### P3 — Add stage timing and toolchain provenance

Emit elapsed time and exact versions for APT provisioning, NDK installation, Rust installation, and Cargo dependency acquisition. Record `rustc --version`, `cargo --version`, `clang --version`, MinGW version, and the resolved NDK path/version in job output. This will make future bottlenecks evidence-based rather than timeout-driven guesses.

### P3 — Keep the APT mirror workaround narrow and observable

The setup command rewrites matching mirror hostnames across all APT source files. Add a pre/post diagnostic that lists the active source entries (without secrets), and constrain any rewrite to the known affected source format/host. Preserve the current fallback behavior until the actual runner image and source-file format are confirmed.

## Anti-duplication and validation

- Existing diagnostic test count documented in `docs/verification/CIRCLECI_DIAGNOSTICS.md`: 14 methods. Their execution on this reviewed HEAD is NOT VERIFIED.
- `circleci config validate .circleci/config.yml`: NOT VERIFIED on this reviewed HEAD.
- Current CircleCI job failure: VERIFIED as a GitHub status context for the exact reviewed HEAD; failed step/root cause: NOT VERIFIED.
- Older exact-SHA successes or failures do not transfer to this HEAD.
- No test was rerun merely to recreate historical evidence.

## Next action

Obtain the redacted failed-step report for job `0adc6a90-9706-4c90-bd02-650edd74ab90` and independently confirm the pipeline revision is `7d4268d8cf713a6d6c3bd5ac533b7396d22b7361`. Use the existing `tools/ci/diagnose_circleci.py` utility with a locally configured `CIRCLECI_TOKEN`; never paste the token into chat. Once the first failing step is known, apply a minimal fix, validate the config and diagnostic tests on the resulting exact SHA, then inspect the new CircleCI run.

## Follow-up status refresh — 2026-11-19

- Latest PR #113 HEAD after adding this review artifact: `5f21f24603410dedbe51b11a62ca196ebfdddf26`.
- Exact-HEAD GitHub combined status: `ci/circleci: one-linux` = PENDING; workflow `2ff63399-02db-41b6-a780-2a20e25dce2c`; job `c0a8d112-0c76-4b55-ac17-7020a8281c27` ([exact-HEAD job](https://app.circleci.com/workflow/2ff63399-02db-41b6-a780-2a20e25dce2c/job/c0a8d112-0c76-4b55-ac17-7020a8281c27)).
- This PENDING result supersedes the prior statement that the documentation commit had no status contexts. It is not PASS and not FAIL. Do not transfer the FAILURE status from parent source SHA `7d4268d8cf713a6d6c3bd5ac533b7396d22b7361`.
- PR #113 remains OPEN / DRAFT / UNMERGED; base branch remains `fix/windows-process-verifier-timeouts-20261118`; no merge or main change.
- The diagnostic script and 13-test suite were read from this exact HEAD. No execution environment or CircleCI provider log reader is available through the connected tools in this session. Diagnostic tests and `circleci config validate` remain NOT VERIFIED.
- The local diagnostic utility requires the user's local `CIRCLECI_TOKEN`; never paste it into chat. Verify the job's pipeline revision before attributing direct job-ID output. The previous failed job for parent SHA `7d4268d...` is a separate run.

## Exact-SHA failed-step evidence — 2026-11-19

- The user supplied the redacted diagnostic report for source SHA `69e66846a67f5d79044329ccf407838f454d24bf`. The report identifies pipeline `7a83d247-df67-4a07-a844-2b3c907f5f64`, workflow `one-ci`, job `one-linux` #740 / `5c42627c-b911-45d1-a32a-7cb72826af53`, and failed step #105.
- The failed step is `Install Linux and Windows-target cross-toolchains (Linux host)`. The exact config uses `timeout 40m sudo apt-get ... install` and `no_output_timeout: 45m`. The final stderr line is `Exited with code exit status 124`, consistent with the explicit 40-minute GNU timeout expiring before APT returned.
- Stdout shows APT update completed, 66.1 MB of packages fetched, all 11 selected packages unpacked/configured, and man-db/libc-bin triggers reached. No download, dependency-resolution, compiler, or test error appears in the supplied output.
- This is the third observed timeout at this same provisioning stage: 15m on source `72bd068...`, 25m on `bb35f1e...`, and now 40m on `69e668...`. Raising the timeout again without identifying the slow/hung post-install operation is not justified.
- Created [the exact-SHA failure report](CIRCLECI_FAILURE_69E668_2026-11-19.md) with evidence, confidence, and a diagnostic remediation plan.
- No CircleCI config change, retry, or test rerun was made. Diagnostic unit tests and `circleci config validate` remain NOT VERIFIED on source SHA `69e668...`.
- PR #113 was OPEN / DRAFT / UNMERGED at readback; base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`. No merge, main modification, force-push, or protected verifier pin change.
- Next safe change: instrument and split the provisioning stage, capture dpkg/process state on failure, and retain the same required compiler capabilities. Apply a package-specific fix only after the instrumentation identifies the blocking operation.


## Instrumented provisioning change — 2026-11-19

- Implemented in `.circleci/config.yml`, commit `84028a01fff08325db365a131a26a34c32690d62`.
- The combined apt install is split into independently timed Clang/Linux-binutils and x86_64 POSIX MinGW groups. Each has a 25-minute GNU timeout; the CircleCI step no-output guard is 30 minutes.
- Added `DEBIAN_FRONTEND=noninteractive`, UTC start/completion markers, elapsed seconds, and failure diagnostics (`dpkg --audit`, apt/dpkg/debconf process snapshot, and the last 100 lines of `/var/log/dpkg.log`).
- Compiler package selection and required Linux-host Windows cross-compilation capability are preserved.
- This is diagnostic isolation, not proof of a package-specific fix. The config has not yet been validated by `circleci config validate`; diagnostic tests have not yet been executed on the new config commit.
- The next CircleCI run must show which package group completes or times out. Do not transfer the failed status from source SHA `69e668...`.


## Exact-SHA follow-up — Clang group reaches timeout — 2026-11-19

- Source SHA `26fcd96d53e19b4218e5b1e5926020a8326fb09b`; CircleCI job #747 / `9666c56b-20cc-4be8-b08c-6547c4a467ab`; failed step #105.
- The first package group (Clang + Linux binutils) started at 08:47:02 UTC. All five Clang packages and the man-db/libc-bin triggers reached installed state by 08:47:09 UTC. The group wrapper did not return and was terminated at 09:12:02 UTC by its 25-minute GNU timeout (exit 124).
- The MinGW group never started. The post-timeout dpkg audit showed no pending package output. Because the process snapshot was taken after timeout cleanup, the blocking process/wait state remains UNKNOWN.
- Follow-up config commit `60a8a20ca92dd4d55ce062f08708250eb1cb6317` preserves package selection and deadline but records live process state every 120 seconds and captures APT terminal/history logs on failure. This is additional instrumentation, not a proven root-cause fix.
- Validation: CircleCI config validation and the diagnostic unit tests on the new commit remain NOT VERIFIED. The 14/14 diagnostic test result on parent SHA `26fcd96...` does not transfer.
- Detailed incident record: [CIRCLECI_FAILURE_69E668_2026-11-19.md](CIRCLECI_FAILURE_69E668_2026-11-19.md).
