# Issue #104 current evidence reconciliation — 2026-11-19

## Exact branch and CircleCI state

- Repository: `panglimanetizen01/one`.
- PR #113: OPEN / DRAFT / UNMERGED.
- Latest exact PR HEAD readback: `8829ca5e9fcb8d7412bdaee0714d48e6b7a894e3`.
- Base branch: `fix/windows-process-verifier-timeouts-20261118`; exact base SHA: `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- GitHub combined status on exact HEAD `8829ca5...`: `ci/circleci: one-linux` = PENDING at `2026-11-19T12:59:38Z`.
- CircleCI workflow: `be01c3a0-3088-454d-b687-0fc71dc02a49`; job: `289e257e-c387-402b-a277-cee762af3493`.
- Job URL: https://app.circleci.com/workflow/be01c3a0-3088-454d-b687-0fc71dc02a49/job/289e257e-c387-402b-a277-cee762af3493
- Immediately preceding exact SHA `3c77d753281ce117cb0c1951ca368c1ade575994` had CircleCI `ci/circleci: one-linux` SUCCESS (workflow `ca61ca33-e0ef-4df1-b048-821c26dcbb81`, job `586c97ac-1d4e-4d14-a9cf-c092f942023d`). Do not transfer that success to the later documentation SHA.
- The 14/14 diagnostic utility unit tests were run on `70ae27c...`; do not claim they were rerun on later documentation SHAs.
- APT timeout was not reproduced in the successful run; root cause remains UNDETERMINED. Do not claim a root-cause fix.
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
