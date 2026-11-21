# Issue #104 current evidence reconciliation — 2026-11-19

## Exact branch and CircleCI state

- Repository: `panglimanetizen01/one`.
- PR #113: OPEN / DRAFT / UNMERGED.
- Historical exact PR HEAD readback: `5386e1048615524e6b6a6c079267b35a2367c484` (superseded by the 2026-11-21 checkpoint appended below).
- Base branch: `fix/windows-process-verifier-timeouts-20261118`; exact base SHA: `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- GitHub combined status on exact HEAD `5386e1048615524e6b6a6c079267b35a2367c484`: `ci/circleci: one-linux` = PENDING at `2026-11-19T13:05:38Z`.
- CircleCI workflow: `dfae6ca1-099d-4ea9-8fd1-1f9dd32c1a1c`; job: `0f9dc451-d530-434d-98bb-b3e28f5b3c8f`.
- Job URL: https://app.circleci.com/workflow/dfae6ca1-099d-4ea9-8fd1-1f9dd32c1a1c/job/0f9dc451-d530-434d-98bb-b3e28f5b3c8f
- The prior exact SHA `3c77d753281ce117cb0c1951ca368c1ade575994` had CircleCI SUCCESS, but that result does not transfer to later documentation commits.
- The 14/14 diagnostic utility unit tests were run on `70ae27c...`; do not claim they were rerun on later documentation SHAs.
- The earlier CircleCI APT timeout root cause remains undetermined. It is distinct from the newly classified GitHub Actions SEC-05 Ubuntu mirror/network failure documented below.
- Any further commit to PR #113 creates a new exact SHA requiring its own status check.

## Runtime-source identity and independent E4 evidence

- The independent diagnostic verifier branch `probe/windows-obj-dont-reparse-20261118` pins code-bearing source SHA `722d662e7a81711ba3607d26571f4048a292b3ed` in its branch-local `CANONICAL_COMMIT.txt`.
- GitHub compare `722d662...` → `70ae27c...` reports 103 commits ahead, 0 behind. The complete changed-path list contains only `.circleci/config.yml`, documentation files, and `tools/ci/diagnose_circleci.py` plus its tests. No runtime implementation or runtime test file changed in that range. Therefore the filesystem runtime/test source at PR HEAD remains byte-for-byte the code-bearing source pinned by the diagnostic verifier.
- E4 Unix + Windows adversarial matrix: all three jobs SUCCESS — [run #44335797344](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797344). Jobs: Unix no-follow/resource bounds; Windows reparse/handle checks; Windows user-mode `NtCreateFile + OBJ_DONT_REPARSE` probe.
- Cross-platform Ubuntu + Windows Server 2025 matrix: both jobs SUCCESS — [run #44335797200](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797200).
- Debian 12 + Ubuntu 24.04: both jobs SUCCESS — [run #44335797294](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797294).
- Android AArch64 artifact build: SUCCESS — [run #44335797320](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797320); artifact `14442665831`, SHA-256 `3bc37c20a92087464abb087be1fe174fac314dbddd635116656e34ad1cee4123`.
- Android API 35 ARM64 runtime: both build and runtime jobs SUCCESS — [run #44335797240](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797240).
- SEC-05 Linux + Windows: both jobs SUCCESS — [run #44335797365](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797365).
- Subprocess resource guard: SUCCESS — [run #44335797191](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797191).
- Windows native probe evidence is limited to Windows Server 2025 build 26100. Do not generalize it to every Windows version.

## E4 filter inventory readback

The current PR HEAD still contains the exact named tests required by the diagnostic verifier workflow:
- Unix: three `descriptor_relative_read_rejects_*` cases, five `descriptor_relative_write_*` cases, one `descriptor_relative_walk_rejects_*` case, the exact swapped-parent-symlink race test, and the exact final-component mount-boundary no-truncate test.
- Windows: all seven exact tests in the verifier workflow are present, including directory-junction escape, absolute in-root read, outside-root no-side-effect confinement, successful atomic write, oversized write rejection, denied atomic replacement preserving bytes/cleaning the temp sibling, and parent traversal rejection.
- The verifier jobs were executed on the pinned source SHA and their GitHub job records are terminal SUCCESS, not filtered-only or pending.

## Remaining boundaries — do not promote prematurely

- Issue #104 remains OPEN / NOT ACCEPTED. The evidence above materially advances implementation and verification, but does not authorize changing the protected verifier pin or closing the issue.
- PR #44 was subsequently marked ready and merged with explicit user authorization; the exact merge commit and post-merge pin are recorded below.
- At the time of the initial evidence snapshot, protected verifier pin was `e50748a1b25922d9090eba1f5fb90548d7c7d003`; it was subsequently promoted through the explicitly authorized PR #44 merge. Current protected pin is recorded in the promotion section below.
- Linux nested writes remain deliberately fail-closed; do not add a path-based fallback.
- macOS and other unsupported platforms remain fail-closed. Android runtime evidence is API 35 ARM64 only.
- Keep PR #113 OPEN / DRAFT / UNMERGED. Do not merge, modify `main`, force-push, delete branches, or change verifier pins.
- A successful CircleCI run is not a substitute for the independent E4 checks; conversely, the independent verifier runs do not replace the CircleCI exact-HEAD status.

## Next safe gate

1. Reconcile the confirmed protected-branch E4 success into the remaining Issue #104 acceptance artifacts and check whether any acceptance gates beyond E4 remain.
2. Investigate the separate SEC-05 Linux cross-compiler-install failure from the same protected-branch push using its exact failed-step logs; do not conflate it with the passing E4 Linux/Windows jobs.
3. Keep PR #113 OPEN / DRAFT / UNMERGED and Issue #104 OPEN / NOT ACCEPTED until all remaining acceptance criteria are satisfied. Unsupported cases remain fail-closed.


## Protected-verifier promotion gap — historical snapshot (superseded 2026-11-19)

The following records the pre-authorization state only. PR #44 was subsequently authorized, merged, and its protected post-merge E4 run has now been confirmed; do not use the old OPEN/DRAFT or old-pin statements below as current state.



The diagnostic PR #44 and protected `verification` branch are intentionally different:

- Protected branch pin `CANONICAL_COMMIT.txt` remains `e50748a1b25922d9090eba1f5fb90548d7c7d003`.
- Protected branch E4 workflow still has the older inventory: 2 Linux read-rejection matches, 1 Linux write-rejection match, 1 walk-rejection match, and older database/governance/network test counts. It does not yet contain the added Windows atomic-replacement-denial test or the separate native `OBJ_DONT_REPARSE` contract-probe job.
- PR #44's three-file diff updates the diagnostic branch's E4 workflow to the current source inventory (3 read-rejection matches, 5 write matches, 1 walk match, exact race + mount no-truncate tests, current supporting test counts), adds the Windows denied-atomic-replacement regression, adds the Windows native contract-probe job, enables the diagnostic `probe/**` push trigger, and updates only the diagnostic branch's `CANONICAL_COMMIT.txt` to `722d662...`.
- PR #44 remains OPEN / DRAFT / UNMERGED. Its diagnostic branch runs have all completed successfully, but none of this changes the protected `verification` branch.
- This is the remaining verifier-governance/acceptance gate, not an unverified runtime code claim. Do not merge PR #44, alter the protected pin, or close Issue #104 without explicit authorization.
- PR #44 conversation comment ID: `7008358258`; PR #113 conversation comment ID: `7008342294`.




## PROTECTED-BRANCH POST-MERGE E4 EXECUTION CONFIRMED — 2026-11-19

- Protected repository: `panglimanetizen01/one-windows-verification`; branch: `verification`.
- Exact push commit: `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd` (merge PR #44).
- Protected canonical source pin readback: `722d662e7a81711ba3607d26571f4048a292b3ed`.
- Protected E4 workflow run #44519577693: terminal `completed / success` — [run details](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44519577693).
- Exact jobs from that run: Windows user-mode `NtCreateFile OBJ_DONT_REPARSE` contract probe (job 135544437734) SUCCESS; Windows reparse-point and handle checks (job 135544438225) SUCCESS; Unix no-follow and SQLite resource bounds (job 135544438313) SUCCESS.
- This is direct post-merge protected-branch evidence and supersedes the earlier note that post-merge run confirmation was outstanding.
- Separate same-push workflows are not all green. In particular SEC-05 run #44519577571: Windows job SUCCESS; Linux job FAILED at step `Install Linux native cross-compilers` (step 5). This is a toolchain-install failure, not an E4 job failure. Other E1/stdlib/provenance/health/artifact workflows also reported failures and require separate scope/triage; do not aggregate them into the E4 result.
- E4 acceptance evidence is now confirmed on the protected branch, but this alone does not establish that every Issue #104 acceptance gate is complete. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 remains OPEN / DRAFT / UNMERGED.
- PR #113 exact HEAD at time of this reconciliation: `3c77d753281ce117cb0c1951ca368c1ade575994`, CircleCI `ci/circleci: one-linux` SUCCESS (workflow `ca61ca33-e0ef-4df1-b048-821c26dcbb81`, job `586c97ac-1d4e-4d14-a9cf-c092f942023d`).


## Protected verifier non-E4 failure log classification — 2026-11-19

The full GitHub Actions job logs were retrieved after the initial run-summary review. This section supersedes the earlier statement that the SEC-05 and E1/stdlib failure causes were not yet classified.

### SEC-05 Linux toolchain installation

- Run #44519577571, Linux job `135544437757`, failed in `Install Linux native cross-compilers`.
- `apt-get update` completed and fetched 13.2 MB. The install requested 277 MB of archives; 194 MB downloaded over 11 minutes before package fetches failed.
- The failed package URLs were Ubuntu MinGW/LLD packages; the runner repeatedly failed to connect to Azure's Ubuntu mirror backend `azure.archive.ubuntu.com`, IP `52.252.75.106:80`. Final APT exit code: 100.
- This is an observed mirror/network retrieval failure, not a Rust compile/test failure and not an E4 failure. It is consistent with the mirror connectivity instability previously seen in CircleCI, but do not assume the remediation is identical without validating the GitHub Actions runner's apt mirror configuration.
- SEC-05 Windows job passed. No SEC-05 Linux test suite result exists because setup failed before tests ran.

### E1 / standard-library workflow harness failures

- E1 Windows run #44519577311, job `135544437394`, failed before tests executed: Cargo reported `no test target named stdlib_builtins in one-core package`, exit code 1.
- Milestone 1 E1 Linux run #44519577315, job `135544437616`, failed with the same missing target, exit code 101.
- Focused stdlib/database run #44519577640, job `135544438572`, failed at its first source-level integration step with the same missing target, exit code 101.
- Exact candidate source tree `722d662...` contains no `crates/one-core/tests/stdlib_builtins.rs`; available integration test targets include `conformance`, `core_pipeline`, `security`, `target_backend_foundation`, and `target_toolchain_foundation`.
- These are verifier-command/acceptance-fixture alignment failures: the named test target is absent, so the requested source-level acceptance tests did not run. Do not substitute a different test target or mark E1 passed without mapping/adding the intended test cases and validating their exact names.

### Scope and next action

- Protected E4 run #44519577693 remains terminal SUCCESS for all three E4 jobs; the failures above belong to separate workflows and do not invalidate that exact E4 result.
- Do not call the full protected verifier push green. Keep Issue #104 OPEN / NOT ACCEPTED and PR #113 OPEN / DRAFT / UNMERGED.
- Next safe work: inspect apt-mirror configuration/remediation for the SEC-05 runner; separately reconcile E1 intended test cases against actual source test targets before changing workflow commands.


## E4 exact test execution audit — protected run #44519577693 — 2026-11-19

The full Unix and Windows E4 job logs and workflow helper were inspected. The "0 passed" Cargo summaries emitted by non-matching test binaries are expected when a filter is applied across multiple binaries; they are not evidence that the required test filter was empty. The workflow explicitly guards against filtered-only false positives:

- Unix helper run_test(expected, filter) counts exact passing test-output lines matching the filter and fails unless the count equals the expected value. Expected counts are 3 read-rejection, 5 write, 1 walk, 1 parent-symlink race, and 1 mount-boundary no-truncate.
- Unix job log contains all 11 required passing tests, including actual mount-boundary, symlink, parent-traversal, atomic-write, race, and no-truncate cases.
- Windows helper requires the requested test name, "running 1 test", and "test result: ok. 1 passed; 0 failed" for every invocation.
- Windows job log contains all seven required passing tests: directory-junction escape; absolute in-root read; outside-root absolute confinement/no-side-effect; successful write; oversized write rejection; denied atomic replacement preserving target; parent-directory traversal rejection.
- Native Windows job log records ordinary-file open success and both direct/intermediate junction opens rejected with STATUS_REPARSE_POINT_ENCOUNTERED (0xC000050B), ending with OBJ_DONT_REPARSE_ROOT_DIRECTORY_JUNCTION_DENIAL_OK.
- Therefore protected E4 run #44519577693 is supported by actual exact test execution, not merely green job conclusions or filtered-only output. Platform limits remain as stated above.


## Fixed-weight final release ledger gate — pending — 2026-11-19

- Protected verifier run #44519577312 is still IN_PROGRESS on verifier merge commit `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`; job `135544436703` is executing `Enforce fixed weights, exact-SHA workflow evidence, protected release, and Definition of Done`.
- The workflow pins the implementation source separately (`CANONICAL_COMMIT.txt` = `722d662...`) and the ledger/DoD document source separately (`LEDGER_SOURCE_COMMIT.txt` = `d6c382a245c49a028bc60809f22ca956c7f894cc`).
- The ledger at the pinned documentation SHA records implementation 92/100 provisional, verified 0/100, and E6 0/4; the Definition of Done has 10 unchecked items. The gate's verifier requires all positive-weight rows fully verified, zero unchecked DoD items, all required exact-SHA mirror workflows terminal-success, and a protected package-provenance release run for the exact source SHA.
- This is a final Project ONE release gate, broader than the Issue #104 E4 subtask. Do not treat its current IN_PROGRESS state as PASS or FAIL, and do not update ledger scores/checkboxes to manufacture acceptance.
- The gate is expected to report its own terminal evaluation after its live-workflow polling. Keep Issue #104 OPEN / NOT ACCEPTED and PR #113 OPEN / DRAFT / UNMERGED meanwhile.


## SEC-05 LINUX FAILURE LOG ROOT CAUSE — 2026-11-19

Exact evidence: protected-verifier workflow run #44519577571, attempt 1; Linux job `135544437757`, [workflow run](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44519577571). Runner: GitHub-hosted Ubuntu 24.04.5 LTS / `ubuntu-24.04`. Candidate checkout: `722d662e7a81711ba3607d26571f4048a292b3ed`. Failed step: `Install Linux native cross-compilers` (step 5).

### Observed failure

- Workflow command was `sudo apt-get update` followed by `sudo apt-get install -y clang lld mingw-w64 binutils-x86-64-linux-gnu`.
- APT update completed successfully, using the runner's Azure Ubuntu mirror (`azure.archive.ubuntu.com`) and Microsoft package source.
- Package install downloaded 194 MB over 11 minutes 9 seconds (reported average 290 kB/s), but several remaining package archives could not be fetched.
- Repeated errors report connection failures/timeouts to `azure.archive.ubuntu.com`, IP `52.252.75.106:80`; APT's final fetch URLs included `security.ubuntu.com` and `archive.ubuntu.com`, while the transport error identifies the Azure mirror backend.
- Examples: `mingw-w64-i686-dev`, `g++-mingw-w64-x86-64-posix`, several `gcc-mingw-w64` variants, `lld`, and meta-packages. The final APT message was `Unable to fetch some archives`; process exit code **100** at `2026-11-19T12:09:56Z`.
- No evidence indicates a Rust compiler failure, SEC-05 source/test failure, package dependency conflict, dpkg configuration error, or compiler invocation. Because the install step failed, the Linux Rust toolchain and subprocess/one-core tests were skipped.
- The Windows SEC-05 job in the same run passed, including existing subprocess contract tests and the complete `one-core` suite.
- Later `if: always()` checks ran: the SEC-05 static marker check passed. Whole-repository `cargo fmt --check --all` printed existing formatting diffs but is explicitly non-blocking; it is not the cause of the job failure.

### Classification and corrective experiment

**Root cause classified:** GitHub-hosted runner's Azure-backed Ubuntu mirror/network endpoint was intermittently unreachable while APT fetched package archives. This is an infrastructure/package-download failure, not an application or SEC-05 test failure. It is distinct from the separate CircleCI APT timeout incident whose root cause remains undetermined.

- A rerun of only the failed Linux job was initiated (workflow attempt 2) to test whether the mirror failure is transient, without changing source or protected branch configuration.
- Rerun Linux job: `135581237093`; at last readback it was still executing `Install Linux native cross-compilers`. Do not claim rerun success until terminal completion.
- No protected workflow or source change was made as part of this diagnosis. If the same failure recurs, a narrowly scoped remediation should first make the package source deterministic (avoid the failing Azure mirror path where possible), add bounded retries for transient archive fetch failures, and emit mirror/source diagnostics. Do not blindly reduce the compiler set or remove x86_64 target capability without inspecting actual downstream compiler requirements.
- Keep the Windows job's successful result distinct from Linux. E4 run #44519577693 remains independently successful; this SEC-05 failure does not negate E4.


## Exact live checkpoint refresh — 2026-11-19 13:20Z

- PR #113 was re-read from GitHub: OPEN / DRAFT / UNMERGED; HEAD `bc0070cbe6ea23b97ff814c145d68c24585a83c5`; base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- GitHub commit status for that exact HEAD now reports `ci/circleci: one-linux` = SUCCESS at `2026-11-19T13:11:51Z`; workflow `3d609b9a-fadc-4451-97fd-daf371ae0d4f`; job `8973d4dd-f38f-4bd1-9aa5-f098cfa3c7b0`. This result applies only to SHA `bc0070c...`.
- Protected verifier SEC-05 rerun attempt 2 remains IN_PROGRESS at the latest API read. Linux job `135581237093` is still in step 5, `Install Linux native cross-compilers`; steps 6–15 remain pending. Windows job `135581239271` is SUCCESS. Job logs for the in-progress Linux attempt are not yet available (GitHub log endpoint returned 404 BlobNotFound).
- No evidence yet that the Azure mirror failure is transient or resolved. Do not alter protected `verification` branch, reduce compiler targets, or claim SEC-05 Linux pass until terminal job evidence is available.
- PR #113 remains OPEN / DRAFT / UNMERGED; Issue #104 remains OPEN / NOT ACCEPTED; `main` remains untouched. Protected E4 run #44519577693 remains independently SUCCESS; unrelated verifier workflow failures remain separate.


## Protected verifier multi-workflow failure reconciliation — 2026-11-19 13:30Z

### PR #113 exact-SHA CircleCI

- PR #113 remains OPEN / DRAFT / UNMERGED; exact HEAD at this checkpoint: `d3924152f2536492a3117e288f60572f3e97814a`; base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Exact-HEAD CircleCI `ci/circleci: one-linux` = SUCCESS (workflow `21bd7a8f-d992-4b62-8250-a46295ca2ea6`, job `328ed531-0e4a-4160-a0bf-3fb50bbf2478`). This success applies only to `d3924152...`.

### SEC-05 Linux rerun attempt 2 — failure reproduced

- Protected verifier run #44519577571 attempt 2; Linux job `135581237093` FAILED at step 5 `Install Linux native cross-compilers`; Windows job `135581239271` SUCCESS.
- Attempt 2 downloaded 277 MB in 15m19s, then received HTTP 503 Service Unavailable from `security.ubuntu.com` for two remaining MinGW packages. APT ended with exit code 100. This differs from attempt 1's connection/timeouts to Azure backend `azure.archive.ubuntu.com` / `52.252.75.106:80`, but both are package-fetch failures before Rust setup or Linux tests.
- Classification remains hosted Ubuntu mirror/package-service availability failure, not a SEC-05 source/test failure. Two attempts now reproduce the package-fetch problem across distinct endpoints; no Linux test result exists for either attempt.
- Draft PR #46: [SEC-05 bounded APT retry mitigation](https://github.com/panglimanetizen01/one-windows-verification/pull/46), head `8a880ad00225ed8a78a3fd58adfbc329f0df5b11`, base protected merge `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`. It records configured APT sources, applies bounded `Acquire::Retries=3` and 30-second HTTP/HTTPS timeouts to update/install, records package policy, and raises Linux job timeout 30→45 minutes. Full compiler set remains unchanged. No protected branch modification or merge.
- PR #46 pull-request workflow run #44533999931 is IN_PROGRESS: Linux job `135594520678` at bounded APT installation step 5; Windows job `135594521270` at Windows toolchain installation step 2. Do not claim validation until terminal results and logs are read.

### Package provenance verifier — false-positive probe corrected in PR #45

- Original run #44519577562 / job `135544438096`: exact-SHA provenance/CLI tests passed 19/19, then the stale-lockfile probe failed because its `sed -i '/^sha2 = /a regex = "1"'` insertion matched no line in the pinned `crates/one-cli/Cargo.toml` (the manifest has only `one-core` under dependencies). Cargo therefore correctly accepted the unchanged lockfile; the probe had not mutated the manifest.
- Draft PR #45: [fix package provenance and artifact verifier assumptions](https://github.com/panglimanetizen01/one-windows-verification/pull/45), head `dabccd46bb6549e36761a4fa3b02da5d7c84a42e`. The probe now inserts `regex = "1"` after the `[dependencies]` header and asserts that the mutation occurred. Workflow execution is NOT RUN; the workflow is push-triggered on protected `verification`, so PR remains draft pending review/authorized promotion.

### Artifact integrity verifier — stale Rust version expectation corrected in PR #45

- Original run #44519577533 / job `135544437263`: artifact discovery and archive digest verification passed, then embedded metadata validation rejected Rust/Cargo `1.100.0` because the verifier hardcoded `1.99.0`.
- Exact pinned ONE source SHA `722d662e7a81711ba3607d26571f4048a292b3ed` has `rust-toolchain.toml` channel `1.100.0`; actual artifact metadata reports Rust/Cargo `1.100.0`. PR #45 updates the expectation to match the exact source toolchain. Workflow execution remains NOT RUN until authorized promotion to protected branch.

### Health scanner workflow — implementation absent, not a scanner defect proven

- Run #44519577289: Linux job `135544437333` and Windows job `135544437821` both failed at final outcome aggregation.
- Each of three unittest discovery commands reported `Ran 0 tests` / `NO TESTS RAN` (exit code 5). The self-scan command could not open `one-source/tools/one_health.py`; recursive source-tree inspection at the pinned SHA found no health/scanner/collector/quality implementation paths.
- The verifier repository's `docs/CONTINUOUS_REPOSITORY_HEALTH_AND_REPAIR_SYSTEM.md` explicitly labels the system a proposal / implementation plan, not an implemented tool. Do not fabricate implementation, change the workflow to green, or claim scanner tests passed. This is an unimplemented-scope / workflow-fixture alignment blocker requiring a separate design and implementation decision.

### Final release gate

- Fixed-weight ledger gate run #44519577312 / job `135544436703` remains IN_PROGRESS at the exact-SHA ledger/DoD enforcement step. It is not PASS or FAIL. Preserve ledger scores and unchecked DoD items; do not manufacture acceptance.
- Protected E4 run #44519577693 remains independently SUCCESS. Do not call the entire protected verifier push green; E1/stdlib, SEC-05 Linux, provenance, artifact, health scanner, and release gate are distinct workflows/findings.
- Issue #104 remains OPEN / NOT ACCEPTED. PR #113 remains OPEN / DRAFT / UNMERGED. Protected branch `verification` and `main` remain unchanged absent explicit authorization.


## Exact-SHA continuation refresh — 2026-11-19 13:40Z



### PR #113 and CircleCI



- PR #113 remains OPEN / DRAFT / UNMERGED. Exact HEAD before this documentation refresh: 09f043d44099c77ed2cb2d3496190571ab47795e; base 181ad350d8f0db53cda654c4d378425fbdd6ab24.

- CircleCI ci/circleci: one-linux = SUCCESS for exact SHA 09f043d; workflow cffa57d9-b339-432c-8d3f-61a592fc531c; job 25e62c16-b0e2-4282-ae75-eb1f6810c234. This does not transfer to the next docs SHA.



### Draft verifier remediation PRs



- PR #45 — https://github.com/panglimanetizen01/one-windows-verification/pull/45, head dabccd46bb6549e36761a4fa3b02da5d7c84a42e, OPEN / DRAFT. Static diff/readback only; workflow execution NOT RUN. Corrects stale-lock probe that previously inserted after nonexistent sha2 dependency line, and aligns artifact Rust/Cargo expectation from 1.99.0 to exact source pin 1.100.0.

- PR #46 — https://github.com/panglimanetizen01/one-windows-verification/pull/46, head 8a880ad00225ed8a78a3fd58adfbc329f0df5b11, OPEN / DRAFT. Pull-request run #44533999931 COMPLETED / SUCCESS on exact PR head. Linux job 135594520678 SUCCESS; Windows job 135594521270 SUCCESS. Linux APT source diagnostics, bounded retries, complete compiler set, subprocess contract tests, full one-core suite, and static SEC-05 guards passed. Windows toolchain, exact source checkout, Rust 1.100.0, 10 subprocess contract tests, full one-core suite, and Windows descendant guard passed. This validates the mitigation on that runner run only; no permanent upstream mirror guarantee. Whole-repo fmt remains a non-blocking diagnostic with pre-existing diffs.

- PR #47 — https://github.com/panglimanetizen01/one-windows-verification/pull/47, head bd100d10c12c7b0849f01e9481111b0b0c859dd1, OPEN / DRAFT. Pull-request run #44535174043 COMPLETED / SUCCESS. Windows Server 2025 job 135598544938 checked out exact source SHA 722d662e7a81711ba3607d26571f4048a292b3ed, read Rust 1.100.0 from source rust-toolchain.toml, installed MinGW clang, passed PE compiler probe, built and inspected PE32+ AMD64, executed with exit code 0 / output Budi, and wrote exact-SHA manifest. PR branch run is not yet protected-branch evidence.

- All three PRs target protected branch verification but remain draft/unmerged. No protected branch mutation or merge was performed.



### E1 / standard-library failure classification



- E1 Windows run #44519577311, job 135544437394: failed at cargo --locked test -p one-core --test stdlib_builtins; Cargo reports no test target named stdlib_builtins and lists actual targets (conformance, core_pipeline, database_vertical_slice, etc.).

- E1 cross-platform run #44519577315: Linux job 135544437616 and Windows job 135544437995 failed at the same nonexistent stdlib_builtins test target.

- Focused standard-library/database run #44519577640, job 135544438572: several filtered --lib invocations returned success with 0 passed / 148 filtered out (no matching tests), then the UTC integration step failed because stdlib_builtins does not exist.

- Exact source tree at 722d662 has no crates/one-core/tests/stdlib_builtins.rs target. crates/one-core/src/stdlib.rs currently implements only display/list append/list remove; expected UTC/JSON/text/integer standard-library tests and features are not present in that file/tree.

- Do not change these workflows to --lib merely to make them green; that would mask missing acceptance coverage. The correct next step is to implement the required E1 APIs and tests in a separately scoped ONE source change, then update verifier tests to target real, named tests. Until then E1 is NOT accepted.



### Windows native runtime exact-SHA gate blocker



- Fixed-weight ledger gate run #44519577312 remains IN_PROGRESS at job 135544436703, timeout budget 180 minutes; its live evidence poll waits up to 10,200 seconds (170 minutes).

- The required workflow ONE Windows Native Backend Runtime — Windows Server 2025 had no run for canonical source SHA 722d662: its old workflow read WINDOWS_SOURCE_COMMIT.txt = a10a69afdf8675e703f38f1620abc935d8361897 and triggered only on that file or its own workflow. The gate resolves evidence through CANONICAL_COMMIT.txt = 722d662, so this workflow could not satisfy the exact-SHA requirement and remained pending.

- PR #47 changes the workflow to trigger/read CANONICAL_COMMIT.txt, adds PR validation, and derives Rust toolchain from the exact checked-out source. Its PR-run SUCCESS validates the fix but is not included in the protected branch's branch=verification run list. After authorized merge, the protected push should generate the exact-SHA run; then the gate must be rerun/evaluated on terminal evidence.

- Other required exact-SHA mirror workflows on verifier commit a44c32e are terminal: E1 Windows #44519577311 FAILURE, E1 cross-platform #44519577315 FAILURE, focused stdlib #44519577640 FAILURE, artifact #44519577533 FAILURE, SEC-05 #44519577571 FAILURE; Linux distro #44519577570, Cargo lockfile #44519577555, subprocess resource guard #44519577526, E4 #44519577693, cross-platform #44519577496, and native ARM64 runtime #44519577666 SUCCESS. Thus even after the missing Windows native run becomes terminal, the gate cannot pass while the above failures and other acceptance requirements remain unresolved.



### Unchanged acceptance boundaries



- Protected E4 run #44519577693 remains SUCCESS, scoped to its stated threat boundary.

- Issue #104 remains OPEN / NOT ACCEPTED. PR #113 remains OPEN / DRAFT / UNMERGED. Main and protected verifier branch remain unchanged absent explicit authorization.

- Preserve exact-SHA claims, fixed ledger weights, DoD unchecked items, fail-closed platform behavior, and full compiler architecture coverage. Do not transfer PR-branch success into protected-branch acceptance.


## Correction and final scope reconciliation — 2026-11-19 13:52Z

### Corrected E1 classification

- The prior note that expected UTC/JSON/text/integer standard-library APIs were missing required E1 features is superseded. The normative docs/spec/one-v0.1-stdlib.md explicitly scopes v0.1 to display, greeting, empty-list creation, list append/remove, arithmetic, and comparison phrases; it warns against adding operations based only on helper signatures. UTC/JSON/text-prefix/suffix/checked-remainder are not required v0.1 APIs.
- E1 verifier failures were stale test assumptions: stdlib_builtins target does not exist; focused --lib filters returned 0 tests matched. Correct action is align workflows to documented v0.1 behavior and existing test targets, not implement unapproved language features.
- Draft PR #48 https://github.com/panglimanetizen01/one-windows-verification/pull/48 head 0fcbbd60e7c97a64cdcb1931b50e16c7cc00ff79. Focused Linux run #44536641377 SUCCESS: stdlib 6/6, core pipeline 6/6, conformance 5/5, DB unit 1/1, DB vertical slice 9/9, full one-core lib 148/148, contract checker PASS. E1 Windows run #44536641371 SUCCESS: stdlib 6/6, core pipeline 6/6, conformance 5/5, full one-core lib 133/133, contract checker and Clippy PASS. Cross-platform run #44536641375: Windows job 135603697996 SUCCESS; Ubuntu job 135603697243 still IN_PROGRESS at native cross-compiler installation. Keep PR draft until Linux terminal result.

### Corrected PR #45 scope

- PR #45 https://github.com/panglimanetizen01/one-windows-verification/pull/45 head 9a72e04b7e2992d655f8e3fc4e478892e91b7ae5 is artifact-integrity-only; current diff changes only candidate-artifact-inspection.yml. Exact-head run #44538012071 SUCCESS; job 135608513172 verified source SHA 722d662, Linux/Windows archive digests, embedded Rust/Cargo 1.100.0, ELF64 x86-64 and PE32+ x86-64.
- The package-provenance signing lab is explicitly excluded from PR #45. The normative v0.1 package/build spec states the full provenance envelope is not implemented. The lab references nonexistent examples/package-release fixture and unsupported sign-package/verify-package commands; it is a future/experimental workflow and not an Issue #104 gate. The attempted stale-lock probe modification was reverted from PR #45. Do not claim package signing/provenance is implemented or validated.

### PR #46 and PR #47 validation

- PR #46 https://github.com/panglimanetizen01/one-windows-verification/pull/46 head 8a880ad00225ed8a78a3fd58adfbc329f0df5b11: run #44533999931 SUCCESS on Linux and Windows; bounded APT retries, full compiler set, subprocess contract tests, full one-core suites, and static guards passed. Draft/unmerged.
- PR #47 https://github.com/panglimanetizen01/one-windows-verification/pull/47 head bd100d10c12c7b0849f01e9481111b0b0c859dd1: run #44535174043 SUCCESS; exact canonical source SHA 722d662, Rust 1.100.0, MinGW clang, PE32+ AMD64, native Windows runtime exit 0/output Budi. Draft/unmerged; PR-branch run is not protected-branch evidence.

### PR #113 and ledger gate

- PR #113 remains OPEN / DRAFT / UNMERGED at pre-refresh HEAD f68246eefed45a45f78f0c43a29d9a55ca7dce99; CircleCI was PENDING immediately after that docs SHA and must be polled again after this documentation update.
- Fixed-weight gate #44519577312 remains IN_PROGRESS waiting for a protected exact-SHA Windows Native Backend run. PR #47 fixes the trigger/source-pin mismatch, but its PR-branch SUCCESS is not visible to the gate's branch=verification query until authorized protected promotion. Other required workflows still include the SEC-05 and artifact/E1 failures addressed by PRs #45/#46/#48.
- Issue #104 remains OPEN / NOT ACCEPTED. Protected E4 #44519577693 is independently SUCCESS; do not claim the entire verifier push is green.

### Current safe boundary

- PRs #45–#48 remain OPEN / DRAFT / UNMERGED. No protected verification branch or main change has been made. No merge authorization inferred.
- Preserve exact-SHA evidence, fixed ledger weights, unchecked DoD, fail-closed behavior, and full compiler target coverage.


## Final validation checkpoint — 2026-11-19 14:03Z



### Exact PR #113 / CircleCI state before this documentation refresh



- PR #113 OPEN / DRAFT / UNMERGED; exact HEAD 2adb4308585e3815a5596e98feb2231a4dfaf80d; base 181ad350d8f0db53cda654c4d378425fbdd6ab24.

- CircleCI ci/circleci: one-linux SUCCESS for exact SHA 2adb4308585e3815a5596e98feb2231a4dfaf80d; workflow 8a3f0ff9-a54b-4624-8b9a-d073d2c01c07; job 752530ca-1f13-413e-aed0-05f730fe0d54. This result does not transfer to the next docs SHA.



### Remediation PRs current final validation



- PR #45 artifact-only: https://github.com/panglimanetizen01/one-windows-verification/pull/45, head 9a72e04b7e2992d655f8e3fc4e478892e91b7ae5, OPEN / DRAFT. Exact-head run #44538012071 SUCCESS, job 135608513172; verified source SHA 722d662, archive digests, Rust/Cargo 1.100.0, Linux ELF64 x86-64 and Windows PE32+ x86-64. Package-provenance signing lab excluded; v0.1 package spec explicitly says full provenance envelope not implemented.

- PR #46 SEC-05 APT mitigation: https://github.com/panglimanetizen01/one-windows-verification/pull/46, head 8a880ad00225ed8a78a3fd58adfbc329f0df5b11, OPEN / DRAFT. Run #44533999931 SUCCESS on Linux and Windows. Bounded APT retries/source diagnostics and complete compiler set passed; subprocess contracts, full one-core suites, and static guards passed.

- PR #47 Windows native canonical pin: https://github.com/panglimanetizen01/one-windows-verification/pull/47, head bd100d10c12c7b0849f01e9481111b0b0c859dd1, OPEN / DRAFT. Run #44535174043 SUCCESS on Windows Server 2025; exact source SHA 722d662; Rust 1.100.0; PE32+ AMD64; native execution exit 0/output Budi. This is PR-branch evidence, not protected-branch evidence.

- PR #48 E1 workflow alignment: https://github.com/panglimanetizen01/one-windows-verification/pull/48, current head 06bf1d7cbd10adadb74c79d7296f951244ae9839, OPEN / DRAFT. Latest cross-platform run #44538540682 COMPLETED / SUCCESS: Linux job 135610376664 and Windows job 135610376991 SUCCESS. Linux bounded APT retries, full compiler set, Android API 35 clang, Rust 1.100.0, stdlib 6/6, core pipeline 6/6, conformance 5/5, one-core lib 148/148, workspace integration suites, contract checker and Clippy passed. Windows toolchain/Android NDK, Rust 1.100.0, stdlib 6/6, core pipeline 6/6, conformance 5/5, one-core lib 133/133, workspace integration suites, contract checker and Clippy passed. E1 Windows run #44538540631 also SUCCESS. Focused Linux run #44536641377 on prior head 0fcbbd60... passed stdlib/database non-zero tests; its workflow file and source pin were unchanged by the later APT-only commit.



### E1 / standard-library scope correction



- Normative docs/spec/one-v0.1-stdlib.md defines v0.1 operations as display, greeting, empty-list creation, list append/remove, arithmetic, and comparisons. UTC/JSON/text-prefix/suffix/checked-remainder are not required v0.1 APIs. Old E1 workflows incorrectly targeted nonexistent stdlib_builtins and had filters that matched zero tests.

- PR #48 corrects the workflows to execute existing, documented targets. Do not implement unapproved future APIs just to satisfy stale verifier commands.



### Package provenance lab scope



- The package-provenance synthetic signing lab is not an Issue #104 gate and is outside executable v0.1 package/build scope. docs/spec/one-v0.1-package-build.md states the full provenance envelope is not implemented. The lab references missing examples/package-release and unsupported sign-package/verify-package commands. PR #45 restores this lab workflow to base content and only changes artifact integrity metadata validation.



### Protected ledger gate / acceptance boundary



- Fixed-weight gate #44519577312 remains IN_PROGRESS. It waits for a protected exact-SHA Windows Native Backend run; PR #47 has a successful PR-branch run but is not included in branch=verification until authorized promotion. Other protected workflow failures have candidate fixes in PRs #45, #46, and #48, but none has been merged.

- Protected E4 #44519577693 remains independently SUCCESS. Issue #104 remains OPEN / NOT ACCEPTED. PR #113 remains OPEN / DRAFT / UNMERGED. Main and protected verification remain unchanged.

- All PRs #45–#48 remain OPEN / DRAFT / UNMERGED. No merge or protected-branch promotion was performed; no authorization inferred.

- Preserve exact-SHA evidence, fixed ledger weights, unchecked DoD, fail-closed platform behavior, and full compiler target coverage.


## Live continuation reconciliation — 2026-11-19 16:14Z

### Exact source and CircleCI
- PR #113 remains OPEN / DRAFT / UNMERGED at exact HEAD `519f0f6856e0ae6242746581b9579cce33fca8cd`, base `181ad350d8f0db53cda654c4d378425fbdd6ab24`, branch `security/issue-104-root-confinement-20261118`.
- GitHub combined status on this exact HEAD: `ci/circleci: one-linux` = SUCCESS; workflow `a3e98250-83bd-4c4a-a72c-cc15b596267b`, job `8ef8576f-999c-4b09-954f-b52aa82f0dd3`. Job-level status only: provider logs are not exposed here, so no individual step output is claimed. Separate `circleci config validate .circleci/config.yml` remains NOT VERIFIED.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`, unchanged.

### Protected verifier reconciliation
- PR #44 merged at 2026-11-19T11:58:30Z as `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`. Protected `verification` now points to that merge commit; `CANONICAL_COMMIT.txt` pins `722d662e7a81711ba3607d26571f4048a292b3ed`. This supersedes older notes claiming PR #44 was draft and pin `e50748a...` was protected; preserve those as historical only.
- Protected E4 run #44519577693 on source `722d662...` SUCCESS: Windows native OBJ_DONT_REPARSE probe, Windows reparse/handle checks, and Unix no-follow/resource-bound jobs all passed. Native Windows probe evidence is limited to Windows Server 2025 build 26100.

### Draft verifier remediation PRs
All remain OPEN / DRAFT / UNMERGED; their successful branch workflows are not protected-branch acceptance evidence:
- PR #45 artifact integrity, head `9a72e04b7e2992d655f8e3fc4e478892e91b7ae5`, run #44538012071 SUCCESS.
- PR #46 SEC-05 APT retry/diagnostics, head `8a880ad00225ed8a78a3fd58adfbc329f0df5b11`, run #44533999931 Linux + Windows SUCCESS.
- PR #47 Windows native runtime pin alignment, head `bd100d10c12c7b0849f01e9481111b0b0c859dd1`, run #44535174043 SUCCESS; source `722d662...`, Rust 1.100.0, PE32+ AMD64, native output Budi / exit 0.
- PR #48 E1 workflow alignment, head `06bf1d7cbd10adadb74c79d7296f951244ae9839`, run #44538540682 Ubuntu + Windows SUCCESS; stdlib 6/6, core pipeline 6/6, conformance 5/5, one-core 148/148 Linux and 133/133 Windows, workspace tests, Clippy and contract checker passed.

### Protected release gate
- Fixed-weight gate #44519577312 is terminal FAILURE (job `135544436703`): timed out waiting 10,200 seconds for the exact-SHA verifier matrix and listed five failed protected workflows: SEC-05 #44519577571, artifact #44519577533, E1 Windows #44519577311, focused stdlib/database #44519577640, and E1 cross-platform #44519577315.
- SEC-05 retry #44519577571 attempt 2: Linux APT install failed after 277 MB / 15m19s with HTTP 503 from `security.ubuntu.com`; Windows passed. Linux failed before Rust setup/tests. PR #46 is the separately validated branch mitigation, not yet promoted.
- PR #45/#46/#48 provide branch-validated candidate corrections for the artifact, SEC-05 and E1 failures; PR #47 fixes the Windows native runtime canonical-pin trigger. Protected revalidation is still required after authorized integration.
- Issue #104 remains OPEN / NOT ACCEPTED. Protected E4 SUCCESS does not make the fixed-weight release gate pass.

### Safety / next action
Do not merge PR #113 or PR #45–#48, change ONE `main`, force-push, or alter protected verification without explicit authorization. After authorized verifier integration, rerun the protected exact-SHA matrix and fixed-weight gate. Until then preserve the current fail-closed behavior and all evidence boundaries.


## Fixed-weight gate log detail — 2026-11-19

The complete GitHub Actions job log for run #44519577312 / job 135544436703 has now been retrieved. The failure is not an E4 regression. The verifier reported:

- Exact candidate source SHA: 722d662e7a81711ba3607d26571f4048a292b3ed.
- Fixed weights: 100 total; implementation 92/100; verified 0/100; 18 ledger rows checked.
- Definition of Done: 10 unchecked acceptance items.
- The independent evidence index did not contain the exact candidate SHA.
- Every non-zero fixed-weight ledger row was still recorded as 0 verified points (F1–F6, C1–C5, E1–E6); no row acceptance points were promoted.
- Exact-SHA matrix wait timed out after 10,200 seconds. The protected run list had failed SEC-05 #44519577571, artifact #44519577533, E1 Windows #44519577311, focused stdlib/database #44519577640, and E1 cross-platform #44519577315; no exact-source protected Windows Native Backend Runtime run was found.
- No protected ONE Package Provenance Release run existed for the exact source SHA, so E2 could not be accepted from candidate bundles.

This is the actual logged gate output, not a hypothesis. PR #45–#48 each validate a narrow fix on its own draft branch, but none is merged into protected verification; their successes cannot retroactively satisfy the failed protected run. Do not edit fixed weights, mark rows VERIFIED, check DoD items, or dispatch protected release merely to make the gate green. Issue #104 remains OPEN / NOT ACCEPTED and the overall release gate remains NOT PASSED.

## Current PR #113 status after documentation sync

- PR #113 HEAD: 3ebb877c5e2f5b97e2154a7b4643b478af940879, OPEN / DRAFT / UNMERGED.
- CircleCI ci/circleci: one-linux on this exact SHA: PENDING at 2026-11-19T16:21:13Z; workflow abe252f3-c181-441c-a5f8-732d3bf9b785; job b233706c-6421-491b-8730-603443e4b196. Parent SHA 519f0f6856e0ae6242746581b9579cce33fca8cd had SUCCESS, which does not transfer.
- ONE main remains d35db95153f6ddd79885494c72f5aee16c899751. No merge or protected-branch change was made.

## Verifier remediation integration and latest exact-SHA evidence — 2026-11-19 16:28Z

The user authorized integration of independent verifier PRs #45–#48. They were squash-merged into panglimanetizen01/one-windows-verification protected branch verification; the protected branch now points to b14abf0020472dc45842be30854e0a09317d6cb4. Exact merge commits:

- PR #45 candidate artifact integrity; tested PR head 9a72e04b7e2992d655f8e3fc4e478892e91b7ae5; merge 42feff8bc11d9b17d04ac9cff4abea4e80a7a832.
- PR #46 SEC-05 bounded APT retries; tested PR head 8a880ad00225ed8a78a3fd58adfbc329f0df5b11; merge d539a162aeed9d22988c7ffb0c7a893e8d33f571.
- PR #47 Windows native runtime exact canonical pin/toolchain; tested PR head bd100d10c12c7b0849f01e9481111b0b0c859dd1; merge 4ae84bad84ddacf169b9f25fa4c78581ab2b4bc4.
- PR #48 E1 v0.1 test-target alignment; tested PR head 06bf1d7cbd10adadb74c79d7296f951244ae9839; merge b14abf0020472dc45842be30854e0a09317d6cb4.

Canonical source pin remains 722d662e7a81711ba3607d26571f4048a292b3ed. No pin, fixed-weight ledger, DoD checkbox, package-release dispatch, or source implementation was changed.

### Protected-branch push runs (do not conflate with PR-branch runs)

- Artifact integrity run #44566172841 on merge 42feff8: SUCCESS. Log verified Linux and Windows candidate metadata, including Rust/Cargo 1.100.0.
- SEC-05 run #44566187162 on merge d539a16: IN PROGRESS; Linux job 135708430394 SUCCESS, Windows job 135708430883 IN PROGRESS.
- Windows Native Backend Runtime #44566197814 on merge 4ae84ba: IN PROGRESS, Windows Server 2025 job 135708473420.
- E1 cross-platform #44566212975 on final protected HEAD b14abf0: IN PROGRESS, Ubuntu job 135708526288 and Windows job 135708526901.
- Focused stdlib/database #44566212982 on b14abf0: SUCCESS. Retrieved Linux log confirms source-pinned Rust/Cargo 1.100.0; stdlib 6/6, core pipeline 6/6, conformance 5/5, database unit 1/1, database vertical slice 9/9, full one-core library 148/148, and warnings-denied Clippy PASS.
- E1 Windows #44566212997 on b14abf0: SUCCESS. Retrieved Windows log confirms Rust/Cargo 1.100.0; stdlib 6/6, core pipeline 6/6, conformance 5/5, full one-core library 133/133, and warnings-denied Clippy PASS.

Only terminal runs above may be classified SUCCESS. In-progress scopes remain NOT VERIFIED until jobs finish and logs are inspected. Protected E4 run #44519577693 remains SUCCESS for its original declared scope and was not rerun because E4 source/test inputs did not change.

### CircleCI exact-SHA correction

ONE PR #113 HEAD 64c490e9045aa29506772f74e3697a63a66b9695 has now reached CircleCI ci/circleci: one-linux SUCCESS (workflow b16121d3-51f0-4297-bb05-16e15b0434ea, job 148c47ea-a1ac-4574-8a61-1980d6c5a55a, status updated 2026-11-19T16:28:21Z). This is exact-SHA status evidence; CircleCI provider step logs were not retrieved in this session. Any subsequent docs commit creates a new PR head and requires its own CircleCI result; this SUCCESS must not transfer.

### Release/acceptance boundary

The prior fixed-weight gate #44519577312 remains FAILURE with its retrieved log: 92/100 implementation, 0/100 verified, 18 ledger rows checked, 10 unchecked DoD items, exact-SHA wait timeout, missing protected workflow evidence and no protected ONE Package Provenance Release. Newly merged fixes do not make that historical gate green. Wait for protected runs above to become terminal, reconcile their exact-source evidence, then consider a new gate evaluation only when relevant prerequisites changed. Do not change weights or mark DoD items without their own evidence.

Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes and unsupported platform adapters remain fail-closed. No universal filesystem sandbox or full release acceptance is claimed. ONE main remains d35db95153f6ddd79885494c72f5aee16c899751. PR #113 remains OPEN / DRAFT / UNMERGED; no merge to ONE main is authorized.

Verifier documentation follow-up PR #50 is OPEN / DRAFT at head 8c1d4dad927397219be5c8803e214105fa99e30f, targeting protected verification; it records the integration and exact status boundaries.

## Final protected verifier integration result — 2026-11-19 16:33Z

All six protected-branch push workflows resulting from the authorized verifier PR #45–#48 integration are now terminal SUCCESS:

- Artifact integrity #44566172841 on verifier merge 42feff8bc11d9b17d04ac9cff4abea4e80a7a832: SUCCESS; exact job log confirms Linux and Windows artifact metadata with Rust/Cargo 1.100.0.
- SEC-05 #44566187162 on verifier merge d539a162aeed9d22988c7ffb0c7a893e8d33f571: SUCCESS; Linux and Windows jobs passed bounded APT setup, interop tests, full one-core suites (148 Linux / 133 Windows), and fail-closed static guards.
- Windows Native Backend Runtime #44566197814 on verifier merge 4ae84bad84ddacf169b9f25fa4c78581ab2b4bc4: SUCCESS; Windows Server 2025 checked out canonical ONE source 722d662, Rust/Cargo 1.100.0, built PE32+ AMD64, executed with exit 0 and output Budi, and wrote exact-SHA manifest.
- E1 cross-platform #44566212975 on verifier protected HEAD b14abf0020472dc45842be30854e0a09317d6cb4: SUCCESS; Ubuntu and Windows both terminal SUCCESS. Ubuntu: stdlib 6/6, core pipeline 6/6, conformance 5/5, one-core 148/148, workspace tests, Clippy warnings denied, repository contract checker. Windows: stdlib 6/6, core pipeline 6/6, conformance 5/5, one-core 133/133, workspace tests, Clippy warnings denied, repository contract checker.
- Focused stdlib/database #44566212982 on b14abf0: SUCCESS; Linux stdlib 6/6, core pipeline 6/6, conformance 5/5, database unit 1/1, database vertical slice 9/9, one-core 148/148, Clippy PASS.
- E1 Windows #44566212997 on b14abf0: SUCCESS; Windows stdlib 6/6, core pipeline 6/6, conformance 5/5, one-core 133/133, Clippy PASS.

The protected verification branch remains at b14abf0020472dc45842be30854e0a09317d6cb4. Canonical source pin remains 722d662e7a81711ba3607d26571f4048a292b3ed. No ledger weight, verified-point score, DoD checkbox, or protected release state was changed. Protected E4 #44519577693 remains SUCCESS for its original declared scope and was not rerun.

The fixed-weight gate #44519577312 remains historical FAILURE: 92/100 implementation, 0/100 verified, 18 rows checked, 10 unchecked DoD items, exact-SHA wait timeout, and missing protected source-bound Package Provenance Release. Do not treat the six remediation workflow successes as overall release acceptance. A new gate evaluation is deferred until the evidence index and all row-level/DoD requirements are legitimately reconciled. Do not change fixed weights or dispatch a release to force PASS.

Issue #104 remains OPEN / NOT ACCEPTED. Linux nested writes and unsupported platform filesystem adapters remain fail-closed. No universal filesystem sandbox, complete E1–E6 acceptance, or overall release is claimed.

ONE CircleCI status on previous PR #113 HEAD d20036c0ad7b00caf6b68e4807102c6ee3b895a4 remains PENDING at 2026-11-19T16:30:42Z (workflow fc7039a6-8cde-464b-8c06-387e291d6d67, job 1a030227-696e-490d-bb16-dd288947976d). No CircleCI CLI/provider action is exposed in this session; the direct CircleCI app URL is inaccessible here. The previous HEAD 64c490e... had SUCCESS, but status does not transfer to later docs commits. This new docs commit will require its own exact-SHA CircleCI result.

Verifier docs follow-up PR #50 is OPEN / DRAFT, targeting verification, with latest docs head 046d356527040f0375d91fabcc9678c683d969e0. ONE PR #113 remains OPEN / DRAFT / UNMERGED; ONE main remains d35db95153f6ddd79885494c72f5aee16c899751.

## CircleCI exact-HEAD terminal result — 2026-11-19 16:39Z

The latest preceding ONE PR #113 documentation HEAD eeafcab579cebb762af34d1daf4830f95cba87e7 has now reached CircleCI ci/circleci: one-linux SUCCESS. GitHub commit status was updated 2026-11-19T16:38:52Z; workflow 0621c480-6054-4fc1-8c82-b9028f105735; job 49178c86-d311-49ac-835a-6fa49985a980. This is exact-SHA status evidence for eeafcab only. CircleCI provider step logs were not separately retrieved, so no detailed step/test-count claim is made from CircleCI. Any later commit must have its own status checked.

The six protected verifier remediation workflows remain terminal SUCCESS on their exact verifier commits and pinned source SHA 722d662. E3 is VERIFIED 8/8 in draft ONE PR #103 ledger source 36bba2e45a629c4fa39677b0ea07a2489d37bc99; total is 92/100 implementation and 8/100 verified; DoD is 4 checked / 9 unchecked / 13. The corresponding verifier draft PR #50 updates LEDGER_SOURCE_COMMIT.txt to the exact ledger commit, but neither documentation PR has been merged to its base branch.

Issue #104 remains OPEN / NOT ACCEPTED. E1, E2, E4, E5 and E6 are not fully verified; protected source-bound Package Provenance Release is still absent. The historical fixed-weight gate #44519577312 remains FAILURE and is not overwritten. No fixed weights, canonical source pin, E4 acceptance pin, ONE main, or protected release state changed.


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

- Code-bearing candidate source: `c7f460a4e38be61de635ad5af9f7ff987a941f01`. Net source/test changes from protected E4 pin `722d662e7a81711ba3607d26571f4048a292b3ed` include `crates/one-core/src/adapter.rs`, `crates/one-core/src/interpreter.rs`, and `crates/one-core/tests/filesystem_vertical_slice.rs`.
- Filesystem adapters advertise only the native backend compiled for the host: Linux on Linux, Windows on Windows, and no filesystem backend on Android/macOS/other unsupported targets. `PlatformKind::current_host()` now passes the actual host identity to orchestration.
- E4 run [#44585270928](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44585270928) SUCCESS: Unix no-follow/resource-bound and Windows reparse/handle tests passed; Windows native `NtCreateFile + OBJ_DONT_REPARSE` probe passed on Windows Server 2025 build 26100. The metadata regression `filesystem_adapters_advertise_only_platforms_with_native_backends` was explicitly executed on Linux and Windows (1 passed / 0 failed each).
- Linux distro run [#44585282690](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44585282690) SUCCESS on Ubuntu 24.04 and Debian 12, including workspace tests and the corrected filesystem vertical-slice CLI regression. E1 Windows acceptance [#44585282432](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44585282432) SUCCESS.
- Remaining candidate workflows (aggregate cross-platform, Windows native backend runtime, Android runtime, SEC-05, E1 milestone, artifact integrity) were still running/pending. CircleCI `ci/circleci: one-linux` on exact source `c7f460a...` remained PENDING; last terminal SUCCESS was on parent docs SHA `26a34ef...` and does not transfer. Provider step logs were not available.
- Verifier diagnostic PR #51 is OPEN / DRAFT at `e9dd717f36bda85bb63e716490e692651fa06126`, branch-local canonical pin `c7f460a...`. Protected verifier branch remains `b14abf0020472dc45842be30854e0a09317d6cb4`; protected canonical pin remains `722d662...`; protected ledger source pin remains `d6c382a245c49a028bc60809f22ca956c7f894cc`.
- Issue #82 is CLOSED / COMPLETED for Android API 35 AArch64 runtime evidence only. This does not imply Android filesystem adapter support.
- Issue #104 remains OPEN / NOT ACCEPTED; Linux nested writes and unsupported platform filesystem adapters remain fail-closed. Fixed-weight gate #44519577312 remains historical FAILURE. No main update, merge, protected pin promotion, ledger weight change, or release action occurred.
- This documentation synchronization creates a new PR #113 HEAD. Keep test evidence attributed to code source `c7f460a...` and check the resulting documentation HEAD independently for CI.

## LIVE EXACT-SHA CHECKPOINT — 2026-11-20

This checkpoint supersedes older live-status statements above where they conflict. Historical run outcomes remain preserved.

### Exact source and repository state

- ONE PR #113 remains OPEN / DRAFT / UNMERGED on `security/issue-104-root-confinement-20261118`.
- Exact source HEAD: `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`; it was not modified.
- Independent verifier PR #51 remains OPEN / DRAFT / UNMERGED; current verifier HEAD: `27207b85ed205d2cf12e6fd66b918193a215d8fd`.
- Verifier PR #51 branch-local `CANONICAL_COMMIT.txt` is `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. Protected `verification` branch and its canonical source pin `722d662e7a81711ba3607d26571f4048a292b3ed` remain unchanged.

### Exact-source verification

- CircleCI `ci/circleci: one-linux`: SUCCESS on source `fb5c8bc...`; workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`. This is status-context evidence; provider step logs were not inspected.
- E4 adversarial filesystem run [#44651796986](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796986): SUCCESS on verifier HEAD `27207b85...`, pinned source `fb5c8bc...`. Windows job #136008224371 explicitly passed the seven named E4 Windows filesystem regressions, the atomic-replacement failure-preservation regression, the hard-link regression, and the platform-metadata regression (9 selected tests, 0 failed). Unix job #136008224711 passed the no-follow, mount-boundary, traversal, race, direct-child write, hard-link, platform metadata, policy, and related resource-bound selected tests.
- Linux distro run [#44651796952](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796952): SUCCESS on Ubuntu 24.04 and Debian 12, including contract checks, build, Clippy, workspace tests, and real-network tests.
- Windows native backend runtime [#44651796852](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796852): SUCCESS.
- E1 Windows acceptance [#44651797000](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651797000): SUCCESS.
- Focused standard-library/database [#44651796941](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796941): SUCCESS.
- SEC-05 subprocess timeout/output-limit [#44651796634](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796634): SUCCESS.
- E4 subprocess resource guard [#44651796650](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796650): SUCCESS.
- Artifact integrity [#44651796712](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796712): SUCCESS. Linux artifact #14563276002 SHA-256 `59aa04887ffa5c2cb66d65e4ef7936622c6e0a4f6164ea8ea458457d10b8a940`; Windows artifact #14561664789 SHA-256 `7d3c51123f73c398032ef0c636c018c12ea501340b822b8e3b4edc9777645a3a`. Embedded manifests both match source SHA `fb5c8bc...`; ELF64 x86-64 and PE32+ x86-64 checks passed.
- Android AArch64 build artifact [#44651796635](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796635): SUCCESS. Android API 35 ARM64 runtime [#44651796976](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796976) remained IN_PROGRESS at this checkpoint.
- Cross-platform aggregate [#44651796654](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796654) remained PENDING; E1 Milestone 1 aggregate [#44651796950](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44651796950) remained IN_PROGRESS. Do not infer completion from individual platform jobs.

### Artifact-integrity historical failure reconciliation

- Earlier artifact-integrity run [#44589012615](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44589012615) FAILED while waiting for a successful cross-platform bundle run for source `fb5c8bc...`; its log did not identify archive corruption.
- Later exact-source run #44651796712 successfully discovered both artifacts, checked GitHub SHA-256 digests, verified embedded source manifests and toolchain metadata, and validated ELF/PE executable formats. Preserve the old failure as historical evidence; the later success is a separate terminal verification, not a deletion or rewrite of that failure.

### Acceptance boundary

- The hard-link test closes a previously missing regression-evidence gap: an in-root file with an outside hard link is rejected for both read and write, and both names retain their original bytes.
- Linux mount-boundary, symlink, traversal, race, and direct-child write controls are exercised by the exact-source Unix E4 run. Linux nested writes remain fail-closed.
- Windows junction/reparse-point and handle-relative checks are exercised by the exact-source Windows E4 run and native backend runtime.
- Filesystem adapter metadata remains Linux-only on Linux and Windows-only on Windows; Android/macOS filesystem adapters remain unavailable. Android native runtime evidence does not imply Android filesystem adapter support.
- Issue #104 remains OPEN / NOT ACCEPTED. Do not promote protected pins, ledger weights, or DoD items; do not merge PR #113 or PR #51; do not change `main` or protected `verification`.


## Android API 35 ARM64 runtime — exact-source verification refresh (2026-11-20)

- Independent verifier PR #51 HEAD: `226360d00bc26f858128d3127591fce13ecd52d9`; exact ONE source pin: `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`.
- Workflow [#44652693606](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44652693606) completed SUCCESS. Build job #136011228119 and API 35 ARM64 runtime job #136012787863 both completed SUCCESS.
- Runtime job log confirms an ARM64 ELF with interpreter `/system/bin/linker64` was pushed to the Android API 35 ARM64 emulator, executed with exit code 0, and printed `ONE_NATIVE_BACKEND_OK`.
- Runtime evidence artifact #14592959506 (`one-api35-arm64-runtime-evidence-fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`) uploaded successfully; SHA-256 `ab24bf6244652ccc6892dabcbbd3571cb4c4dda3bd2bf5ad902c922e3faf152d`.
- This verifies the native backend runtime on the specified Android API 35 ARM64 environment for this exact source. It does **not** establish Android filesystem adapter support: filesystem adapter metadata remains Linux-only on Linux and Windows-only on Windows; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.
- Do not repeat this Android runtime workflow for the same source and verifier inputs unless a relevant input changes.


## Live exact-SHA refresh — 2026-11-20 (B-001 / B-002 / release-gate reconciliation)

This is the newest readback. Preserve all earlier snapshots as history; this block supersedes only conflicting live-state claims.

### Repository and pull requests
- ONE PR #113: OPEN / DRAFT / UNMERGED; exact HEAD `7296c573cbd7eab4a1d7ff08bced2b397a5c555f`; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`. Compare from code-bearing source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` shows 48 commits and documentation-only changes across 18 existing docs paths; no runtime source, test, or CI workflow path changed.
- PR #115 is MERGED at `3adace165890bb69de346880e3c612c5cd6c2ff8`.
- Verifier PR #51: OPEN / DRAFT / UNMERGED; HEAD `6c0979e62eb01e4b929c48b4224a3d4f742bc8af`; branch-local `CANONICAL_COMMIT.txt` pins ONE source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. Compare from verifier HEAD `226360d00bc26f858128d3127591fce13ecd52d9` is three documentation-only commits (CANDIDATE_VERIFICATION.md, MIRROR_STATUS.md, README.md, Android evidence note); no workflow/source implementation change.
- Protected verifier branch remains `verification` at `b14abf0020472dc45842be30854e0a09317d6cb4`; protected canonical source pin remains `722d662e7a81711ba3607d26571f4048a292b3ed`. Protected `LEDGER_SOURCE_COMMIT.txt` remains `d6c382a245c49a028bc60809f22ca956c7f894cc`. No protected pin was changed.
- ONE `main` remains `d35db95153f6ddd79885494c72f5aee16c899751`; no main mutation.

### B-001 — CircleCI pre-execution termination
- Historical pipeline #1201 on `509b7b197115adb1ba80d730342e9f6fbf981645` remains a confirmed pre-execution termination; provider root cause UNKNOWN.
- Code-bearing ONE source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` has GitHub combined status `ci/circleci: one-linux = SUCCESS`, workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`. Provider step logs were not inspected; claim only exact-SHA status success.
- Current PR #113 HEAD `7296c573...` has no combined-status entries. Its predecessor docs SHA `a0d94bc9a26cf9effd4b202f29af50f81c8e4ee5` had FAILURE at workflow `916983e9-dbde-4db2-a967-438a666ebb19`, job `a130c5d9-c078-4c18-8ae5-571470f46d60`; available step #101 was CANCELED with unavailable task information and empty logs. Do not call it a test timeout or source failure.
- Exact main SHA `d35db951...` separately shows context `CircleCI Pipeline = ERROR`, target pipeline #1202. Pipeline-to-workflow/job/revision attribution and provider logs remain unavailable. Do not conflate this with the PR context or the source-SHA SUCCESS.
- B-001 is operationally resolved only for the tested code-bearing source/workflow state; provider root cause remains UNKNOWN and the latest docs HEAD has no new CircleCI result.

### B-002 — Issue #104 exact-source verification and remaining product gap
- Twelve PR-triggered verifier workflows associated with verifier HEAD `6c0979e62eb01e4b929c48b4224a3d4f742bc8af` are terminal SUCCESS, all using branch-local source pin `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`:
  - E4 adversarial #44658361321: Unix job #136030099861; Windows reparse/handle job #136030099812; Windows native probe #136030099584 — all SUCCESS.
  - Linux distro #44658361778: Ubuntu 24.04 job #136030102055; Debian 12 job #136030102321 — SUCCESS.
  - Windows native runtime #44658361296: job #136030100371 — SUCCESS.
  - E1 Windows #44658361571: job #136030101715 — SUCCESS.
  - E1 Milestone 1 #44658361518: Windows job #136030101394 and Ubuntu job #136030101588 — SUCCESS.
  - SEC-05 #44658361512: Linux job #136030100674 and Windows job #136030100987 — SUCCESS.
  - Cross-platform #44658361514: Windows job #136030101518 and Ubuntu job #136030101784 — SUCCESS.
  - Artifact integrity #44658361320: job #136030099580 — SUCCESS.
  - Resource guard #44658361457: job #136030100840 — SUCCESS.
  - Android AArch64 build #44658361365: job #136030100171 — SUCCESS.
  - Focused stdlib/database #44658361557: job #136030101561 — SUCCESS.
  - Android API 35 ARM64 runtime #44658361350: build job #136030099698 and runtime job #136030558684 — SUCCESS.
- E4 job logs were inspected, not just run summaries. Linux executed 31 named tests (3 read, 5 write, 1 walk, race, mount-boundary, hard-link, database, 11 governance, 5 network, policy-denial, platform metadata), 0 failed. Windows executed 9 named regressions, 0 failed. The Linux write suite explicitly includes `descriptor_relative_write_fails_closed_for_nested_parent`; this is proof of fail-closed behavior, not positive nested-write support.
- Android API 35 ARM64 runtime is native-backend execution evidence only; it does not establish an Android filesystem adapter.
- Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Do not promote full filesystem support from the scoped E4 pass.

### Release gate / B-003 boundary
- The protected ledger document pin remains `d6c382a245c49a028bc60809f22ca956c7f894cc`; the protected ledger gate #44519577312 remains historical FAILURE (92/100 provisional implementation, 0/100 verified at that gate's ledger source, 18 rows checked, 10 DoD items unchecked, no protected exact-source ONE Package Provenance Release).
- ONE PR #103 remains OPEN / DRAFT / UNMERGED at HEAD `5f616ae08e303081666a5e0d6a4b8b63b9d69c16`. Its branch does not replace the protected ledger source pin. Keep fixed weights, verified points, DoD checkboxes, canonical pins, and release criteria unchanged. No new gate evaluation is justified until relevant acceptance/evidence inputs change.
- No source code, CI configuration, main branch, protected verifier branch/pin, ledger weights, DoD checkbox, merge, or release was changed in this refresh.

### Next executable work
1. Continue B-002 from the remaining Linux nested-write acceptance gap. Keep the current fail-closed behavior; do not weaken it. Any implementation proposal must prove parent-directory move/race confinement with real Linux and Windows runners before changing security-sensitive code.
2. Keep unsupported Android/macOS filesystem adapters explicitly unavailable unless native backends and their own adversarial matrices are implemented and verified.
3. Continue B-003 only after the exact release-candidate SHA, complete E1–E6 evidence, source-bound package provenance, and ledger/DoD reconciliation are genuinely available. Do not rerun the unchanged historical gate or promote scores.


## Exact-SHA Windows parent-rename regression — 2026-11-20

- New code-bearing test SHA: `c4f3800bc12b70a4d59d72fbc5a80029cb053cea`; the only source change relative to the prior candidate is a Windows-only test in `crates/one-core/src/filesystem.rs`, not production behavior.
- Verifier PR #51 exact HEAD: `7c3ded0d811fe71bd14034880b24b0f8e396349e`; branch-local canonical source pin: `c4f3800...`. Protected verifier branch and canonical pin remain unchanged.
- New test: `filesystem::windows_native::tests::retained_parent_handle_blocks_directory_rename`. It opens a nested parent with the same `FILE_SHARE_READ | FILE_SHARE_WRITE` share flags used by `write_relative` (no `FILE_SHARE_DELETE`), proves rename fails while the handle is retained, then proves rename succeeds after handle close.
- E4 push run #44760172674 is terminal SUCCESS on exact source `c4f3800...` / verifier `7c3ded0...`: Unix job #136370280796 SUCCESS; Windows reparse/handle job #136370280317 SUCCESS; Windows native probe job #136370280733 SUCCESS. Windows log shows the new test executed exactly once and passed (1 passed, 0 failed).
- Duplicate E4 pull-request run #44760180858 is also terminal SUCCESS on the same exact source/verifier pair; Unix job #136370312121, Windows reparse/handle job #136370312012, and native probe job #136370311602 all SUCCESS. Count the duplicate event as confirmation, not a second acceptance point.
- Other exact-verifier-SHA PR workflows: SUCCESS — E4 resource guard #44760181208, focused stdlib/database #44760180912, Android AArch64 artifact build #44760180867, Linux distro #44760180826, E1 Windows #44760181124. IN_PROGRESS — Windows native runtime #44760181204, candidate artifact integrity #44760180923, SEC-05 #44760180951, E1 Milestone 1 #44760180919 (Ubuntu job #136370312623 SUCCESS; Windows job #136370313083 IN_PROGRESS), cross-platform #44760181947, Android API35 runtime #44760180953. Do not call the complete verifier matrix green yet.
- ONE source PR #113 GitHub Actions run #44760138732 is IN_PROGRESS on exact source `c4f3800...`: Ubuntu job #136370164396 SUCCESS; Windows job #136370165192 IN_PROGRESS. CircleCI `ci/circleci: one-linux` is PENDING on this exact SHA, workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Pipeline UUID/number, `vcs.revision`, and job→workflow→pipeline membership remain unverified.
- B-001 remains PARTIAL / NOT CLOSED pending CircleCI pipeline mapping/provider logs. Issue #104 remains OPEN / NOT ACCEPTED: Linux nested writes fail-closed; Android/macOS filesystem adapters unavailable. This test verifies one Windows parent-handle exclusion primitive, not complete E4/Issue #104 acceptance.
- No production runtime code, CI configuration, main, protected verifier branch/pin, ledger/DoD, acceptance criteria, merge, or release changed.


## Exact-SHA verification refresh — 2026-11-20 15:14Z

### Candidate identity
- Code-bearing source: `c4f3800bc12b70a4d59d72fbc5a80029cb053cea` (only source delta from `fb5c8bc...` is the Windows-only parent-handle rename regression test).
- Verifier PR #51 HEAD: `7c3ded0d811fe71bd14034880b24b0f8e396349e`; branch-local source pin: `c4f3800...`.
- ONE PR #113 current docs-only HEAD before this sync: `043390cf0673247524ffb44c444cbbe4745be91f`. This sync changes existing docs only and uses `[skip ci]`; no CI result transfers to the resulting docs HEAD.

### Terminal results on exact source/verifier pair
- ONE GitHub Actions run #44760138732 on source `c4f3800...`: Ubuntu job #136370164396 SUCCESS; Windows Server 2025 job #136370165192 SUCCESS. Windows log explicitly ran and passed `interop::tests::subprocess_execution_is_bounded_by_a_deadline`, `subprocess_stdout_capture_has_a_hard_limit`, `subprocess_stderr_capture_has_a_hard_limit`, and `windows_job_object_terminates_descendants_when_parent_exits`.
- Verifier PR matrix on verifier SHA `7c3ded0...` / source pin `c4f3800...`: 11 of 12 named workflows are terminal SUCCESS:
  - E4 adversarial #44760180858: Unix #136370312121, Windows #136370312012, native probe #136370311602 — SUCCESS. New `retained_parent_handle_blocks_directory_rename` test executed once and passed.
  - E4 subprocess resource guard #44760181208 / job #136370313217 — SUCCESS.
  - Focused stdlib/database #44760180912 / job #136370312551 — SUCCESS.
  - Android AArch64 artifact build #44760180867 / job #136370311618 — SUCCESS.
  - Linux distro #44760180826: Debian #136370312102 and Ubuntu #136370312874 — SUCCESS.
  - Windows native runtime #44760181204 / job #136370312887 — SUCCESS; Windows Server 2025, Rust 1.100.0, runtime exit 0, output `Budi`.
  - SEC-05 #44760180951: Linux #136370311990 and Windows #136370312357 — SUCCESS. Source PR Windows GHA also proves the dynamic deadline/stdout/stderr/Job Object tests executed and passed.
  - E1 Windows #44760181124 / job #136370313677 — SUCCESS.
  - E1 Milestone 1 #44760180919: Ubuntu #136370312623 and Windows #136370313083 — SUCCESS.
  - Cross-platform #44760181947: Windows #136370329101 and Ubuntu #136370329521 — SUCCESS.
  - Candidate artifact integrity #44760180923 / job #136370311819 — SUCCESS.
- E4 push event #44760172674 is also SUCCESS on the same exact source/verifier pair (Unix #136370280796, Windows #136370280317, native probe #136370280733); it is a duplicate trigger and is not a second acceptance point.

### Artifact provenance and remaining runtime
- Artifact integrity selected cross-platform run #44760181947 at verifier SHA `7c3ded0...` for exact source `c4f3800...`.
- Linux artifact #14640868829, SHA-256 `7e8e443e2ae7c736f18243043f355840801afebdddca46a5efe41c159bac5732`; metadata target `x86_64-unknown-linux-gnu`, Rust/Cargo 1.100.0, ONE CLI 0.1.0, ELF64 little-endian EM_X86_64 PASS.
- Windows artifact #14641018247, SHA-256 `f68c7325ff39ae33b2cfadf0b896896c0ba4c3eeb581b033f8e1a75974900846`; metadata target `x86_64-pc-windows-msvc`, Rust/Cargo 1.100.0, ONE CLI 0.1.0, PE32+ x86-64 PASS.
- The only verifier workflow not terminal is Android API 35 ARM64 runtime #44760180953. ELF build job #136370311867 is SUCCESS; runtime job #136371143664 is still executing the “Boot Android API 35 AArch64 on ARM64 host” step. Workflow timeout is 150 minutes; no timeout/failure is claimed.
- Do not claim Android filesystem adapter support from this native-backend runtime. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

### CircleCI and release gate
- CircleCI `ci/circleci: one-linux` on source `c4f3800...` remains PENDING; workflow `cdb2cc27-ced0-4b88-ab5a-3b656e173151`, job `bbb3abb8-7272-483a-9708-18f5573ee2fd`. Pipeline ID/`vcs.revision` mapping and provider logs remain unavailable. B-001 remains PARTIAL / NOT CLOSED.
- Protected verifier branch/pin, protected ledger pin `d6c382a...`, fixed weights, DoD, main, acceptance criteria, merge, and release state remain unchanged. Historical fixed-weight gate #44519577312 remains FAILURE.

## Implementation-first continuation — 2026-11-20

Owner direction: complete the actual v0.1 product implementation before spending another cycle on broad test/verification runs. Add or update necessary regression tests as part of implementation, but defer the full matrix and repetitive audits until the implementation scope is complete. Do not claim a feature complete merely because tests or docs are green.

### Current Issue #104 implementation delta

- PR #113 exact HEAD at this checkpoint: `7cbcb1bf575defe03a2d9991dc5094569de95f03` (OPEN / DRAFT / UNMERGED; base `181ad350d8f0db53cda654c4d378425fbdd6ab24`).
- CLI filesystem authority now derives only from the host process current working directory; source-file location and program-controlled `--target` no longer select or widen the root.
- Linux root acquisition now opens the supplied directory with `O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC` and checks canonical-path device/inode identity against the retained handle; a mismatch fails closed.
- CLI regression fixture updated to place the source file outside the working directory while targeting an in-root file.
- Implementation commits: `8b1d13e02c181fc06b97fb7f147e1ccd013d3aa1`, `5e39430a341629eb9fda11ee4e11b0f6f40487ab`, `7cbcb1bf575defe03a2d9991dc5094569de95f03`.
- These changes are NOT BUILT OR TESTED yet. Existing prior-SHA results do not transfer. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

### B-001 operational status

- Exact SHA `979973cd6fb6f76802e13dbbbe21530d68b4cf53` has a terminal successful CircleCI workflow/job mapping: pipeline #1233 revision matches the SHA; workflow `934cd811-3a44-4737-bb62-943e677d979b` SUCCESS; job #865 `4b9419f9-9b1a-4823-902e-dcd872073e39` SUCCESS, started `2026-11-20T15:49:44.969Z`.
- The diagnostic classifier correction is locally 20/20 on that source. Historical pipeline #1201 cause remains UNKNOWN, but no longer blocks the current implementation queue. Do not rerun it without a new relevant failure.

### Next implementation priority

Continue Issue #104 production work and reconcile the normative contract against the current source. Preserve fail-closed behavior where a race-resistant primitive is not available; do not weaken the boundary to make nested Linux writes appear supported. Then continue remaining v0.1 product implementation gaps. Only after implementation is complete should the integrated final platform/security/release matrix run.


## Implementation-first source update — 2026-11-21

The latest code-bearing candidate reviewed is `72eae103bc3304b0961c66a0cf61f716ddfa9ac4`. Source includes retained-root Linux `openat2` reads, atomic direct-child writes, guarded detached-mount nested writes with fail-closed fallback; Windows native handle-relative operations; Android direct-child reads/writes with no-follow and mandatory `statx(STATX_MNT_ID)`; macOS direct-child reads/writes with no-follow and filesystem identity checks; hard-link rejection; and atomic temporary-file replacement/sync paths. Android/macOS nested paths remain fail-closed. CLI integration fixtures were corrected in `46c777e...` and `2600ec1...` to use the host working directory as authority and to reflect conditional Linux nested-write support. No build, test, or verifier run was performed. Issue #104 remains OPEN / NOT ACCEPTED; implementation source presence is not security acceptance. Preserve all existing acceptance criteria and protected verifier pins.


## Linux special-file read hardening — 2026-11-21

Code change: `50923b545f2c804b24aa773ea5bb60129d1e0ff2` updates the Linux `openat2` read flags to include `O_NONBLOCK` before the opened object is checked with `fstat` for regular-file type. This prevents opening a FIFO with blocking semantics while retaining the existing handle-relative `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV` policy. A Linux regression `authority_rejects_fifo_without_blocking` creates a FIFO with no writer and requires `FilesystemError::NotRegularFile`. The regression has NOT RUN; this is source/test presence only, not execution evidence or Issue #104 acceptance.


## Filesystem read-allocation and resource-limit hardening — 2026-11-21

Code commit: `8792f7a1f707f444d00026d916fa8c18b7f3e7f1`.

- Replaced the unchecked `u64 as usize` conversion used to preallocate filesystem read buffers with `usize::try_from` and `Vec::try_reserve_exact`. Oversized capacities now fail with a defined filesystem error instead of truncating on 32-bit hosts or panicking on reservation failure.
- Added focused Linux regression `authority_enforces_read_and_write_limits_without_partial_replacement`; it verifies an oversized read is rejected, an oversized write is rejected, and the existing target bytes remain unchanged.
- This regression is NOT RUN. No build, test, CI, verifier, or platform matrix is claimed. Issue #104 remains OPEN / NOT ACCEPTED.


## Linux nested-write fallback and verifier-inventory alignment — 2026-11-21

Code commits:
- `0fd434ad46e6b8e93c3b9ed887447baf10d0f697`: classify Linux `EOPNOTSUPP` from required detached-mount primitives as `FilesystemError::UnsupportedPlatform`, preserving fail-closed behavior when the backing filesystem/kernel cannot provide the required operation.
- `858a5da09fe2ea74ebd17b0798f13fac848c7a28`: replace the stale test that required every Linux nested write to fail with `nested_write_uses_native_backend_or_fails_closed_without_side_effects`. It accepts a successful native detached-mount write or an explicit `UnsupportedPlatform` fallback, and checks target integrity and temp-file cleanup.

The independent verifier's expected source test-filter inventory is now structurally matched in `adapter.rs`: 3 `descriptor_relative_read_rejects_`, 5 `descriptor_relative_write_`, and 1 `descriptor_relative_walk_rejects_` tests. This is a source inventory check only; no tests were run and no verifier PASS is claimed. Issue #104 remains OPEN / NOT ACCEPTED.


## Android/macOS filesystem helper repair and nested-path security boundary — 2026-11-21

Latest code-bearing source: `5f98db9f70d28a0d5a206a3e6e37f986efc3574b` (`crates/one-core/src/filesystem.rs`).

- Removed the duplicate Android `create_temp` definition and supplied the missing macOS `create_temp` helper. These were source-visible compile blockers; no build was run.
- Android/macOS direct-child read/write backends remain. Their root-relative direct-child operations retain no-follow checks, regular-file/single-link checks, Android `statx(STATX_MNT_ID)` or macOS `fstatfs` identity checks, and same-directory atomic replacement.
- Nested Android/macOS paths remain fail-closed. A proposed generic openat parent walk was deliberately not enabled: ordinary POSIX directory handles do not pin an opened parent beneath the retained root, so an attacker may rename that directory outside the root before the leaf operation. The current resolver rejects multi-component paths with `UnsupportedPlatform` until an anti-move primitive is implemented.
- Android/macOS regression fixtures now cover nested-path fail-closed behavior, leaf symlinks, and an outside sentinel. These tests are NOT RUN.
- This is source implementation hardening and a precise capability boundary, not acceptance. Issue #104 remains OPEN / NOT ACCEPTED. No build/test/CI/verifier/platform matrix was run and no status was promoted.


## 2026-11-21 — Android/macOS filesystem source repair and confinement boundary

Latest code-bearing source: `0f5859426273a61174d6d1b08ed8771a6f4b136a` (`crates/one-core/src/filesystem.rs`).

- Android's duplicate `create_temp` definition has been removed; macOS now has its missing exclusive `create_temp` helper. Static source inventory shows one helper definition in each native module. No compiler was run.
- Android/macOS direct-child operations retain root-handle-relative no-follow reads and same-directory exclusive-temp atomic replacement; Android requires `statx(STATX_MNT_ID)`, while macOS compares `f_fsid` and mount point with `fstatfs`. Regular-file and hard-link checks remain.
- Nested paths on both POSIX backends explicitly return `UnsupportedPlatform`. Ordinary POSIX directory descriptors do not prevent a parent directory from being renamed outside the retained root, so a multi-component openat walk would not satisfy the root-confinement race contract. No such unsafe nested support is enabled.
- Android/macOS source regressions cover direct-child operation, leaf symlink rejection, nested-path fail-closed behavior, root symlink rejection, and hard-link side-effect preservation. These tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED. This source correction is not a build/test/verifier result; no platform support or release approval is promoted.


## 2026-11-21 — Android nested-read implementation with kernel-enforced confinement

Latest source candidate: `52fcfb2706fcb48e9599d75f9086578279f1bbac` (`crates/one-core/src/filesystem.rs`, NOT BUILT/TESTED).

- Static source inventory at this candidate finds exactly one `create_temp` definition in `android_native` and exactly one in `macos_native`. The earlier duplicate Android helper was already removed; macOS's missing helper was added. This is source inspection only, not Android/macOS compile evidence.
- Android nested reads now use Linux/Android `openat2` with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`. The kernel performs the complete path resolution relative to the retained root descriptor and rejects symlink, mount-crossing, escape, and unsafe rename-race outcomes. The opened leaf is still checked for regular-file type, single hard-link, device identity, and `STATX_MNT_ID` equality.
- Android nested writes remain fail-closed (`UnsupportedPlatform`) until an anti-move atomic replacement backend is implemented. Direct-child atomic writes remain unchanged.
- macOS nested reads and writes remain fail-closed. Ordinary `openat` directory-descriptor walks cannot prevent a parent directory from being renamed outside the root; no unsafe traversal has been enabled. Direct-child operations remain unchanged.
- Added Android source regression for a valid nested read and a symlinked-parent escape with outside-sentinel preservation. Existing Android expectations now distinguish missing nested files (`NotFound`) from symlink-parent denial (`ResolutionFailed`). Tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED. No build, test, CI, verifier, or platform matrix ran; no platform support or release approval is promoted.


## Issue #104 source implementation update — 2026-11-21

- The current PR #113 code-bearing candidate is `9fe53815f83701a62536472f8a3c87cebac22e7a` (NOT BUILT/TESTED). Android nested reads use `openat2`; Android nested writes now use the detached-mount + recursive `MOUNT_ATTR_NOSYMFOLLOW` + `openat2`/`renameat2` transaction shared with Linux and fail closed when required kernel/SELinux primitives are unavailable. macOS nested reads now use XNU `O_RESOLVE_BENEATH | O_NOFOLLOW_ANY | O_UNIQUE` plus post-open `fstatfs` mount identity checks; unsupported kernels fail closed. macOS nested writes remain fail-closed pending a proven atomic no-cross-mount publication primitive. Source regressions are NOT RUN. Issue #104 remains OPEN / NOT ACCEPTED.
