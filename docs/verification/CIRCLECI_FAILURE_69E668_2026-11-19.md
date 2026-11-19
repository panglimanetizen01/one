# CircleCI failure report — cross-toolchain provisioning timeout

## Exact execution identity

- Repository: `panglimanetizen01/one`
- Branch: `security/issue-104-root-confinement-20261118`
- Failed source SHA: `69e66846a67f5d79044329ccf407838f454d24bf`
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

The source configuration at failed SHA `69e668...` wrapped the package installation in:

```bash
timeout 40m sudo apt-get "${apt_opts[@]}" install -y --no-install-recommends clang gcc-mingw-w64-x86-64-posix binutils-x86-64-linux-gnu
```

The same CircleCI step had `no_output_timeout: 45m`, so the explicit 40-minute command deadline preceded the inactivity guard.

The supplied stdout shows:

- `apt-get update` fetched 41.2 MB and completed package-list processing.
- The install fetched 66.1 MB of archives.
- APT selected 11 new packages, including Clang 18 and x86_64 POSIX MinGW GCC.
- All listed packages reached their `Setting up` phase.
- The log reached man-db and libc-bin trigger processing.
- No package download, dependency-resolution, compiler, or test error is shown in the supplied output.

## Evidence-based conclusion

**High confidence:** the explicit GNU `timeout 40m` deadline expired while the APT install command had not returned successfully. Exit status 124 is the expected timeout utility status. The 45-minute CircleCI no-output guard was not the first configured deadline.

**Not established:** which package post-install script, dpkg trigger, apt hook, or other child process consumed the remaining time. The captured output has no per-package timestamps or process-state snapshot. Package setup messages do not prove that the parent APT process completed.

This is a repeated provisioning timeout: the same step previously exhausted 25 minutes (job #734, source `bb35f1e4777a1a2f078c4f88ad7f16d3dbad9dc2`), after an earlier 15-minute timeout on source `72bd068e33045c95d042c225c0bdfd27ea4df57f`. Raising the deadline from 25 to 40 minutes did not resolve the failure.

## Implemented diagnostic remediation — 2026-11-19

Config commit: `84028a01fff08325db365a131a26a34c32690d62`.

The combined package command was replaced with two independently timed package groups:

1. Clang + Linux binutils.
2. x86_64 POSIX MinGW GCC.

Each group has a 25-minute explicit command deadline, with a 30-minute CircleCI no-output guard for the step. The command now sets `DEBIAN_FRONTEND=noninteractive`, emits UTC start/completion markers and elapsed seconds, and on failure captures:

- `dpkg --audit`
- a bounded process listing for apt/dpkg/debconf
- the last 100 lines of `/var/log/dpkg.log`

The package set and required Linux-host Windows cross-compilation capability are preserved. This is an instrumentation/isolation change, not a claim that the underlying package hook has been fixed.

## Validation status

- The new config commit requires exact-SHA CircleCI validation; its run must be inspected independently.
- `circleci config validate .circleci/config.yml`: NOT VERIFIED.
- Diagnostic Python unit tests: NOT VERIFIED on config commit `84028a0...`.
- No package-specific root cause has yet been established. Use the next run's substage markers and failure snapshot to identify the blocking group/process before changing packages or deadlines.

## Safety and acceptance

- PR #113 remains OPEN / DRAFT / UNMERGED.
- Do not modify `main`, merge the PR, force-push, or change the protected verifier pin.
- This CircleCI provisioning failure does not change independent verifier results and does not establish Issue #104 acceptance.


## Exact-SHA follow-up — instrumented Clang group timeout — 2026-11-19

- Failed source SHA: `26fcd96d53e19b4218e5b1e5926020a8326fb09b`.
- CircleCI pipeline: `2c819857-b2b8-4be5-ba75-6b9ad6b4b62b`; workflow `one-ci`; job `one-linux` #747 / `9666c56b-20cc-4be8-b08c-6547c4a467ab`; failed step #105.
- The report confirms the first group, **Clang and Linux binutils**, started at `08:47:02 UTC`. All five Clang packages reached installed state by `08:47:06 UTC`; man-db and libc-bin triggers reached installed state by `08:47:09 UTC`.
- At `09:12:02 UTC`, the wrapper recorded `APT INSTALL FAILED: Clang and Linux binutils; exit 124; elapsed 1500s`. This is the configured 25-minute GNU timeout, not the 30-minute CircleCI no-output guard.
- The MinGW group did **not** start. The failure handler ran `dpkg --audit` (no pending package output), captured a post-timeout process snapshot, and showed no active apt/dpkg/debconf process. The snapshot cannot reveal the process state before timeout termination.
- Evidence-based conclusion: the Clang install command remained alive for 25 minutes after its final visible dpkg trigger output, then was terminated by `timeout`. The packages being installed does not establish that the parent `apt-get` returned. The precise wait point / hook / child process remains UNKNOWN. Do not blame Clang packages, MinGW, or increase the timeout blindly.
- Follow-up config commit: `60a8a20ca92dd4d55ce062f08708250eb1cb6317`. It retains the same package groups and 25-minute deadline, but records live apt/dpkg/debconf/needrestart/sudo/timeout process state every 120 seconds while the install runs, including process wait-channel data. On failure it also captures APT terminal/history logs. This is additional diagnosis, not a claimed fix.
- The config change has not yet been validated by CircleCI or `circleci config validate`. The 14 diagnostic Python unit tests passed on parent SHA `26fcd96...`; that result does not transfer to this new config commit.
