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

Each invocation enforces a 10-second deadline on the direct child and a 1 MiB capture limit for stdout and stderr independently. Linux creates a dedicated process group and installs an inherited seccomp filter before exec; the filter denies `setsid`, `setpgid`, `unshare`, `setns`, namespace-bearing `clone` flags, and x32 syscall entry, while returning `ENOSYS` for `clone3` so compatible runtimes can fall back to `clone`. The intended containment boundary is that descendants cannot leave the invocation process group; group cleanup runs on normal return, timeout, and output overflow. If `PR_SET_NO_NEW_PRIVS` or filter installation fails, process creation fails closed. This Linux candidate has NOT been built or executed yet, so Issue #105 remains OPEN. Windows assigns the child to a Job Object during process creation and terminates remaining job members during cleanup, with kill-on-close as a final safeguard. Process execution is unavailable on Android and macOS until equivalent non-escapable containment is implemented. Nonblocking Linux readers retain a 100 ms bounded shutdown grace.

## Not yet claimed as complete

Native C/C++ ABI/FFI, Python embedding, Node.js module binding, Java/JVM binding, browser integration, database drivers, and network libraries require separate semantic/ABI contracts and conformance tests. They must not be represented as complete merely because process execution can invoke those tools.

## Issue #105 — Linux process-group containment implementation candidate — 2026-11-20

- Latest code-bearing candidate: `0a9685bded788ae46596227bd61457d10db1179c` (NOT BUILT OR TESTED).
- Linux subprocess startup now installs an inherited seccomp filter in the child before `exec`. It denies `setsid`, `setpgid`, `unshare`, `setns`, clone namespace flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` so compatible runtimes can fall back to `clone`. The existing dedicated process group is therefore intended to remain non-escapable for descendants; group cleanup still runs on normal return, timeout, and output overflow.
- If the kernel refuses `PR_SET_NO_NEW_PRIVS` or filter installation, process spawn fails closed. Windows Job Object containment remains unchanged.
- Process execution is now explicitly unavailable on Android and macOS until equivalent non-escapable containment exists; `ProcessAdapter` advertises only Linux and Windows.
- Replaced the old escaped-session reader-only probes with three Linux adversarial regressions for attempted `setsid` on normal completion, timeout, and output overflow. These tests have NOT been run.
- This is an implementation candidate, not Issue #105 acceptance. Exact-SHA build/execution and adversarial evidence are still required. Preserve bounded reader shutdown and all existing process limits.
