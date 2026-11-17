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

Each invocation has a fixed 10-second execution deadline and a 1 MiB capture limit for stdout and stderr independently. Output beyond either limit triggers platform-specific cleanup and returns a bounded-resource error. On Windows, assignment to a kill-on-close Job Object occurs as part of CreateProcessW through PROC_THREAD_ATTRIBUTE_JOB_LIST, avoiding a spawn-then-assign race; cleanup terminates remaining job members and waits for the job to drain within a bounded deadline. On Unix, cleanup sends SIGKILL to the invocation's dedicated process group. This is best-effort only: descendants that create a new session or process group can escape. In this source revision, output-reader joins can still wait on inherited pipes held by escaped descendants; issue #105 tracks this unresolved guarantee.

## Not yet claimed as complete

Native C/C++ ABI/FFI, Python embedding, Node.js module binding, Java/JVM binding, browser integration, database drivers, and network libraries require separate semantic/ABI contracts and conformance tests. They must not be represented as complete merely because process execution can invoke those tools.
