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
- Next external dependency: run the local diagnostic utility with `CIRCLECI_TOKEN` and the exact job UUID above; verify the job's pipeline revision equals `5f21f24603410dedbe51b11a62ca196ebfdddf26` before attributing the report. The previous failed job for parent SHA `7d4268d...` is a separate run.
