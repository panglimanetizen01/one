# Security Policy

## Scope

One is designed to expose filesystem, process, network, and native capabilities only through explicit runtime boundaries.

## Rules

- Foreign process execution must not implicitly invoke a shell.
- Programs receive no privileged capability by default in the reference interpreter.
- Process output, stderr, and exit status are preserved by interoperability APIs.
- Native FFI is not enabled by v0.1 unless an explicit capability/ABI contract is implemented.
- Security-sensitive behavior must have regression tests before being advertised as supported.

## Reporting

Do not publish an exploitable vulnerability before a fix is available. Use a private GitHub security report for vulnerabilities when the repository's security reporting facility is enabled.


## Bounded database query output

The SQLite query adapter accepts read-only `SELECT` requests and applies explicit result limits: at most 100,000 rows and 16 MiB of rendered UTF-8 output. Blob values are rendered as hexadecimal only when the encoded value fits within the same output budget. Exceeding either limit returns the typed `ResourceLimitExceeded` adapter error; partial results are not returned as success. These limits bound result materialization but do not replace filesystem path confinement, query timeouts, or a complete runtime-wide resource quota policy.
