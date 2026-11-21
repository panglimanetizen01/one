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


- Instrumentation refinement commit: `5c0b45a9dd8cbab4814f7935eacd08cecea17c88`. The live process snapshot now filters on the process `comm` field (avoids matching the enclosing shell's embedded script text) and emits a real newline after each captured command line. This is a logging-quality correction only; no package, timeout, or compiler behavior changed.


## Exact-HEAD CircleCI refresh — 2026-11-19 16:14Z

- PR #113 exact HEAD `519f0f6856e0ae6242746581b9579cce33fca8cd` has GitHub commit status `ci/circleci: one-linux` SUCCESS (workflow `a3e98250-83bd-4c4a-a72c-cc15b596267b`, job `8ef8576f-999c-4b09-954f-b52aa82f0dd3`). This is terminal job-level evidence for this SHA only.
- CircleCI provider logs are not exposed in this session. No claim is made about individual step logs, timings, or a separately observed unit-test count. The configured job includes `python3 -m unittest discover -s tools/ci -p 'test_*.py' -v`; overall job SUCCESS indicates the job completed without a reported failure.
- `circleci config validate .circleci/config.yml` remains NOT VERIFIED as a separate command. The job-level SUCCESS does not independently establish that this CLI validation was executed.
- This does not resolve protected verifier gate #44519577312, which is terminal FAILURE on its own protected run. Do not transfer the PR #113 CircleCI status to the protected verifier.


## Current checkpoint correction — 2026-11-19 16:44Z

- PR #113 exact HEAD `54b92ab9703b9c3450cb33f5eb4f34861aae107b` now has CircleCI `ci/circleci: one-linux` terminal SUCCESS (workflow `f9bd8ac7-a741-4bf6-8536-37af4798f3a1`, job `77951c98-1242-4c47-b45b-90011ecebfd6`). This supersedes the earlier PENDING observation for the same SHA. CircleCI provider logs are not exposed; this is job-level exact-SHA evidence only. No separate CLI config-validation or per-step test-count claim is made.
- Protected verifier branch `verification` remains `b14abf0020472dc45842be30854e0a09317d6cb4`, canonical source pin `722d662e7a81711ba3607d26571f4048a292b3ed`. All 12 required exact-source verifier workflow names plus Android runtime alternative have terminal SUCCESS.
- Draft ONE ledger/DoD PR #103 head `36bba2e45a629c4fa39677b0ea07a2489d37bc99` records E3 VERIFIED 8/8; ledger totals 92/100 implementation and 8/100 verified; DoD 4 checked / 9 unchecked / 13. This is still a draft source.
- Draft verifier PR #50 head `622637766b9fe7b673182e21faf6b2d8ce0a8767` proposes the exact-source evidence index and ledger source pointer to PR #103. It remains OPEN / DRAFT / UNMERGED. Protected `LEDGER_SOURCE_COMMIT.txt` remains `d6c382a245c49a028bc60809f22ca956c7f894cc`; the proposed pointer has not been applied.
- Historical fixed-weight gate #44519577312 remains FAILURE on the old protected ledger source: 92/100 implementation, 0/100 verified, 18 rows checked, 10 DoD unchecked, and no protected source-bound ONE Package Provenance Release. No new gate run has been triggered. Do not alter fixed weights/DoD or merge PR #50 without explicit authorization.
- Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes and unsupported filesystem adapters remain fail-closed. Issue #82 protected Android runtime #44519577666 succeeded (API 35 arm64-v8a, ELF exit 0), but PR #49 metadata runtime job #135708475823 remains IN_PROGRESS. Issue #105 escaped-descendant containment remains unresolved.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`; PR #113 remains OPEN / DRAFT / UNMERGED.


## Live exact-SHA refresh — 20 November 2026

- ONE PR #113 is OPEN / DRAFT / UNMERGED at exact HEAD `b0d854124191587882705c832e1825ad2a46cc8f`; base remains `181ad350d8f0db53cda654c4d378425fbdd6ab24`. ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI `ci/circleci: one-linux` is terminal SUCCESS on this exact PR HEAD: workflow `ecd5d877-e5f5-429c-884c-426196fc7e78`, job `cc77e7cb-8daf-4d9c-845d-df991e0f68bf`. This updates the prior PENDING observation for the same SHA. Only the combined-status/job result is verified here; no provider step-log or test-count claim is made.
- The last code-bearing candidate remains `722d662e7a81711ba3607d26571f4048a292b3ed`. GitHub's net tree comparison from that source pin to PR #113 HEAD shows no changed source or test paths; the net changes are CI configuration, diagnostics tooling, and documentation. Keep E4 and other independent results attributed to the exact source pin `722d662...`, not relabeled as runs on `b0d854124191587882705c832e1825ad2a46cc8f`.
- Protected verifier branch `verification` is `b14abf0020472dc45842be30854e0a09317d6cb4`; `CANONICAL_COMMIT.txt` remains pinned to `722d662...`, and `LEDGER_SOURCE_COMMIT.txt` remains `d6c382a245c49a028bc60809f22ca956c7f894cc`.
- Protected E4 run [#44519577693](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44519577693) is SUCCESS for its pinned source. The historical fixed-weight gate [#44519577312](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44519577312) remains FAILURE; it is not overwritten by E4 PASS.
- Verifier PR #50 remains OPEN / DRAFT at `0c0d8563d28e1f70734ecf4d906be91f912472f3`, with no status contexts/check runs reported. PR #49 remains OPEN / DRAFT for Android runtime metadata. ONE PR #103 remains OPEN / DRAFT at `5f616ae08e303081666a5e0d6a4b8b63b9d69c16`; its CircleCI status is a separate config-fetch error and does not transfer to PR #113.
- Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes and unsupported platform adapters remain fail-closed. No main update, merge, protected pin change, ledger promotion, or release action occurred.


## Exact-SHA continuation update — 20 November 2026

- Latest CircleCI-confirmed PR #113 HEAD at this checkpoint: `051c7815cc5cf3dbca840ac8cbb38e3e663da6b7`; `ci/circleci: one-linux` terminal SUCCESS, workflow `3b89115a-ff07-41ac-940e-fb78baccd7ac`, job `241f1d78-0aaf-4f47-8ea2-57fbd3b35a1c`. This is job-level evidence; provider step logs were not separately inspected.
- PR #113 remains OPEN / DRAFT / UNMERGED; ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`. This successful candidate result does not mean CircleCI has been integrated into `main`.
- Android API 35 AArch64 runtime metadata evidence is now complete: verifier run #44565956781 SUCCESS on PR #49 exact head `fa0b5cffe52bf31fedef81ed945d6cc3a6dc2e19`; build job `135707610069` and runtime job `135708475823` SUCCESS. Inspected artifact #14553867982 records source pin `722d662e7a81711ba3607d26571f4048a292b3ed`, API 35, ABI `arm64-v8a`, host `aarch64`, exact ADB invocation, exit 0 and `ONE_NATIVE_BACKEND_OK`. ELF SHA-256 matched the metadata. ONE Issue #82 is CLOSED / COMPLETED for this runtime-evidence scope only.
- Protected verifier branch `verification` remains `b14abf0020472dc45842be30854e0a09317d6cb4`; canonical source pin remains `722d662...`; protected ledger source pin remains `d6c382a245c49a028bc60809f22ca956c7f894cc`. No protected pin or ledger weight changed.
- The fixed-weight gate #44519577312 remains historical FAILURE. Issue #104 remains OPEN / NOT ACCEPTED; Linux nested writes and unsupported platform filesystem adapters remain fail-closed. Issue #105 and the broader E1/E2/E4/E5/E6 release work remain unresolved.
- This documentation synchronization itself creates a new PR #113 HEAD. Do not transfer the SUCCESS above to the new SHA; inspect the exact resulting commit status before any subsequent CircleCI claim.


## CircleCI exact-HEAD terminal confirmation — 20 November 2026

- Current PR #113 HEAD at this checkpoint: `26a34efb6d0908bd02738a7b05090f96051a6dc7`.
- GitHub combined status: `ci/circleci: one-linux` = terminal SUCCESS.
- Workflow: `a7016ead-ed2d-467f-9f1b-4d20c60a966f`; job: `6cc9778c-307c-48fc-9111-39c97d288393`.
- This is a direct exact-SHA status read for `26a34efb6d0908bd02738a7b05090f96051a6dc7`; it is not transferred from parent commits. Provider step logs were not inspected, so no per-step or test-count claim is made.
- PR #113 remains OPEN / DRAFT / UNMERGED; ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`. The CircleCI candidate has not been integrated into `main`.
- Issue #82 is CLOSED / COMPLETED for Android API 35 AArch64 runtime evidence; artifact #14553867982 was downloaded and its metadata/ELF digest inspected. This does not establish Android filesystem adapter support.
- Issue #104 remains OPEN / NOT ACCEPTED; fixed-weight gate #44519577312 remains historical FAILURE. Linux nested writes and unsupported platform filesystem adapters remain fail-closed.
- This documentation-only synchronization creates a new PR #113 HEAD; check that resulting SHA independently before transferring any CI status.


## Exact-SHA platform metadata and host-selection verification — 20 November 2026

- Candidate source: `c7f460a4e38be61de635ad5af9f7ff987a941f01`. The code/test delta from protected E4 pin `722d662e7a81711ba3607d26571f4048a292b3ed` includes `adapter.rs`, `interpreter.rs`, and `filesystem_vertical_slice.rs`.
- Filesystem adapter metadata now advertises Linux only on Linux and Windows only on Windows; Android/macOS/other unsupported targets advertise no filesystem backend. `PlatformKind::current_host()` supplies actual host identity to orchestration.
- E4 run #44585270928 SUCCESS; the metadata regression explicitly passed on Linux and Windows. Linux distro run #44585282690 SUCCESS on Ubuntu 24.04 and Debian 12, including the corrected CLI integration test. E1 Windows acceptance #44585282432 SUCCESS.
- Aggregate cross-platform, Windows native runtime, Android runtime, SEC-05, E1 milestone, and artifact integrity workflows remained in progress/pending. CircleCI on exact source `c7f460a...` remained PENDING; last terminal CircleCI SUCCESS was on parent docs SHA `26a34ef...` and does not transfer.
- PR #113 remains OPEN / DRAFT / UNMERGED. Verifier PR #51 remains OPEN / DRAFT with branch-local pin `c7f460a...`; protected verifier branch and pins remain unchanged. Issue #104 remains OPEN / NOT ACCEPTED; Issue #82 is CLOSED only for Android runtime evidence. No merge, main update, pin promotion, or release.
