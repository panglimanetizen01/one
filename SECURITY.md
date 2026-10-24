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

The SQLite query adapter accepts read-only `SELECT` requests and applies layered resource limits: at most 100,000 rows, 16 MiB of rendered UTF-8 output, 1 MiB per text value, 32 result columns, 1 MiB SQL text, 256 bound parameters, a 100,000-opcode SQLite VM limit, and SQLite limits on expression depth, compound selects, function arguments, attached databases, and LIKE patterns. A 2-second progress-handler deadline and 250 ms database-lock timeout bound query execution/waiting. Blob values are rendered as hexadecimal only when the encoded value fits within the output budget. Resource-limit and SQLite size/interruption errors map to the typed `ResourceLimitExceeded` adapter error; partial results are not returned as success. These controls bound query execution and result materialization but do not establish a complete runtime-wide quota policy or a project-root sandbox.
