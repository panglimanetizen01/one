# One v0.1 Interoperability Contract

## Implemented boundary

One v0.1 exposes an explicit process boundary through `one-core::interop::run_process`.

A process request contains:

- program path/name;
- argument vector;
- optional working directory.

The result preserves:

- exit status;
- success/failure;
- stdout;
- stderr.

Arguments are passed as an argument vector, not through an implicit shell. This is the portability and security baseline for Windows and Linux.
The executable path must be non-empty. Embedded NUL bytes are rejected in the executable path, every argument, and the optional working directory before the host process API is called; this prevents platform-specific truncation of Windows command lines and path strings.

Each invocation enforces a 10-second deadline on the direct child, a 1 MiB capture limit for stdout, a 1 MiB capture limit for stderr, and an explicit 2 MiB aggregate capture ceiling. The aggregate ceiling is the sum of the two stream caps and is checked before returning a ProcessResult. Linux and Android create a dedicated process group and install an inherited seccomp filter before exec; the filter denies `setsid`, `setpgid`, `unshare`, `setns`, namespace-bearing `clone` flags, and x32 syscall entry, while returning `ENOSYS` for `clone3` so compatible runtimes can fall back to `clone`. The intended containment boundary is that descendants cannot leave the invocation process group; group cleanup runs on normal return, timeout, and output overflow. If `PR_SET_NO_NEW_PRIVS` or filter installation fails, process creation fails closed. This Linux/Android candidate has NOT been built or executed yet, so Issue #105 remains OPEN. Windows assigns the child to a Job Object during process creation and terminates remaining job members during cleanup, with kill-on-close as a final safeguard. Process execution is unavailable on macOS until equivalent non-escapable containment is implemented. Nonblocking Linux/Android readers retain a 100 ms bounded shutdown grace.

## Not yet claimed as complete

Native C/C++ ABI/FFI, Python embedding, Node.js module binding, Java/JVM binding, browser integration, database drivers, and network libraries require separate semantic/ABI contracts and conformance tests. They must not be represented as complete merely because process execution can invoke those tools.

## Issue #105 — Linux process-group containment implementation candidate — 2026-11-20

- Latest code-bearing candidate: `0a9685bded788ae46596227bd61457d10db1179c` (NOT BUILT OR TESTED).
- Linux subprocess startup now installs an inherited seccomp filter in the child before `exec`. It denies `setsid`, `setpgid`, `unshare`, `setns`, clone namespace flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` so compatible runtimes can fall back to `clone`. The existing dedicated process group is therefore intended to remain non-escapable for descendants; group cleanup still runs on normal return, timeout, and output overflow.
- If the kernel refuses `PR_SET_NO_NEW_PRIVS` or filter installation, process spawn fails closed. Windows Job Object containment remains unchanged.
- Process execution now uses the inherited Linux seccomp policy on Linux and Android; `ProcessAdapter` advertises Linux, Android, and Windows. macOS remains fail-closed until equivalent non-escapable containment exists.
- Replaced the old escaped-session reader-only probes with three Linux adversarial regressions for attempted `setsid` on normal completion, timeout, and output overflow. These tests have NOT been run.
- This is an implementation candidate, not Issue #105 acceptance. Exact-SHA build/execution and adversarial evidence are still required. Preserve bounded reader shutdown and all existing process limits.

## Android process containment regression coverage — 2026-11-20

- Latest code-bearing candidate: `418d9642778d975440b81df2bdb5e1bf9aa1d8a5` (NOT BUILT/TESTED).
- The inherited seccomp pre-exec filter is now installed on both Linux and Android. ProcessAdapter advertises Linux, Android, and Windows; macOS remains fail-closed. Android uses the same `setsid`/`setpgid`/namespace restrictions and fails closed if the child cannot install the filter.
- The three adversarial `setsid` regression tests and bounded-reader test are now target-gated for Linux and Android, so they can run in Termux as well as Linux CI. No Android/Termux execution evidence is claimed.
- Issue #105 remains OPEN / NOT ACCEPTED pending exact-SHA build and adversarial execution on Linux and a real Android/Termux target, plus Windows Job Object evidence reconciliation.


## Subprocess resource-bound implementation refresh — 2026-11-21

- Code-bearing candidate: `cd95e5ec2234a2909ef79bf8b25fe65296fab5ed` (NOT BUILT/TESTED).
- The implementation now names and enforces `MAX_OUTPUT_BYTES_TOTAL = 2 * MAX_OUTPUT_BYTES_PER_STREAM`, with a final aggregate-size guard on both Linux/Android and Windows before returning the process result. Per-stream limits remain 1 MiB each; deadline remains 10 seconds.
- Existing regression coverage includes stdout overflow, stderr overflow, deadline, normal exit/status/stream preservation, and Issue #105 direct-fork + `setsid` escape attempts for normal completion, timeout, and output overflow. These tests have NOT RUN on this candidate.
- Issue #81 remains OPEN until exact-SHA Linux/Windows execution confirms the timeout, per-stream, aggregate, and cleanup contract. Issue #105 remains OPEN / NOT ACCEPTED until Linux and real Android/Termux adversarial execution proves the containment boundary.
