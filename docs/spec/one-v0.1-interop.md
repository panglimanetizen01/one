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

Each invocation has a fixed 10-second execution deadline and a 1 MiB capture limit for stdout and stderr independently. Output beyond either limit terminates the process tree and returns a bounded-resource error. Descendants must not outlive the invocation: Unix execution uses a dedicated process group, and Windows execution assigns the process to a kill-on-close Job Object during process creation.

## Not yet claimed as complete

Native C/C++ ABI/FFI, Python embedding, Node.js module binding, Java/JVM binding, browser integration, database drivers, and network libraries require separate semantic/ABI contracts and conformance tests. They must not be represented as complete merely because process execution can invoke those tools.
