# CircleCI failure report — cross-toolchain provisioning timeout

## Exact execution identity

- Repository: `panglimanetizen01/one`
- Branch: `security/issue-104-root-confinement-20261118`
- Source SHA: `69e66846a67f5d79044329ccf407838f454d24bf`
- PR #113: OPEN / DRAFT / UNMERGED at the diagnostic readback
- CircleCI pipeline UUID: `7a83d247-df67-4a07-a844-2b3c907f5f64`
- Workflow: `one-ci`
- Job: `one-linux` #740
- Job UUID: `5c42627c-b911-45d1-a32a-7cb72826af53`
- Failed step: #105, `Install Linux and Windows-target cross-toolchains (Linux host)`

## Observed evidence

The redacted exact-SHA diagnostic report identifies source SHA equality and failed step #105. The final stderr line is:

```text
Exited with code exit status 124
```

The source configuration at this SHA wraps the package installation in:

```bash
timeout 40m sudo apt-get "${apt_opts[@]}" install -y --no-install-recommends clang gcc-mingw-w64-x86-64-posix binutils-x86-64-linux-gnu
```

The same CircleCI step has `no_output_timeout: 45m`, so the explicit 40-minute command deadline precedes the inactivity guard.

The supplied stdout shows:

- `apt-get update` fetched 41.2 MB and completed package-list processing.
- The install fetched 66.1 MB of archives.
- APT selected 11 new packages, including Clang 18 and x86_64 POSIX MinGW GCC.
- All listed packages reached their `Setting up` phase.
- The log reached man-db and libc-bin trigger processing.
- No package download, dependency-resolution, compiler, or test error is shown in the supplied output.

## Evidence-based conclusion

**High confidence:** the explicit GNU `timeout 40m` deadline expired while the APT install command had not returned successfully. Exit status 124 is the expected timeout utility status. The 45-minute CircleCI no-output guard is not the first configured deadline.

**Not established:** which package post-install script, dpkg trigger, apt hook, or other child process consumed the remaining time. The captured output has no per-package timestamps or process-state snapshot. The fact that package setup messages appear does not prove that the parent APT process completed.

This is a repeated provisioning timeout: the same step previously exhausted 25 minutes (job #734, source `bb35f1e4777a1a2f078c4f88ad7f16d3dbad9dc2`), after an earlier 15-minute timeout on source `72bd068e33045c95d042c225c0bdfd27ea4df57f`. Raising the deadline from 25 to 40 minutes did not resolve the failure.

## Decision

Do **not** blindly increase the timeout again or add retries. That would extend a repeatedly failing stage without identifying the process that stalls.

The next config change should be diagnostic and narrowly scoped:

1. Add UTC start/end markers and elapsed-time reporting around each provisioning substage.
2. Separate Clang installation from MinGW installation so the slow package group is identifiable.
3. On a nonzero install result, capture a bounded process snapshot, `dpkg --audit`, and the tail of `/var/log/dpkg.log` before exiting with the original status.
4. Keep the existing compiler packages and platform capability unchanged until actual consumers are verified.
5. After evidence identifies the blocking package or hook, apply the smallest targeted correction.

No CircleCI configuration change, retry, or test rerun is made by this report. The diagnostic unit tests and `circleci config validate` remain NOT VERIFIED on source SHA `69e668...`.

## Safety and acceptance

- PR #113 remains OPEN / DRAFT / UNMERGED.
- Do not modify `main`, merge the PR, force-push, or change the protected verifier pin.
- This CircleCI provisioning failure does not change independent verifier results and does not establish Issue #104 acceptance.
