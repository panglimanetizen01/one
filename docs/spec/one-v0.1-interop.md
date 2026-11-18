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

Each invocation enforces a 10-second deadline on the direct child and a 1 MiB capture limit for stdout and stderr independently. Output-limit violations terminate the direct child and trigger platform-specific cleanup. On Unix, cleanup kills the original process group, but a descendant can escape by creating a new session or process group; such a descendant may remain alive after the invocation returns. After shutdown is requested, nonblocking readers continue draining only until the pipe becomes idle, the 1 MiB per-stream output cap is exceeded, or a 100 ms shutdown grace expires. A continuously active writer therefore produces an explicit output-limit or non-idle error rather than keeping the caller blocked or being silently accepted. This is bounded reader shutdown, not descendant containment. Strong Unix process-tree containment remains unresolved (Issue #105). On Windows, the child is assigned to a Job Object during process creation, and remaining job members are explicitly terminated during cleanup, with kill-on-close as a final safeguard.

## Not yet claimed as complete

Native C/C++ ABI/FFI, Python embedding, Node.js module binding, Java/JVM binding, browser integration, database drivers, and network libraries require separate semantic/ABI contracts and conformance tests. They must not be represented as complete merely because process execution can invoke those tools.
