# One v0.1 Release Contract

Status: **VERIFIED BASELINE WITH EXPERIMENTAL DNS, FILESYSTEM-READ, AND PROCESS-EXECUTION ORCHESTRATION**.

## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

## Current exact-SHA evidence boundary

The process-execution source/runtime revision independently verified is `9038ac75f197753ff1ab6af7478e63e4831903a2`.

Independent mirror run: `38929072800` in `panglimanetizen01/one-windows-verification`.

Ubuntu and Windows Server 2025 both checked out that exact SHA and passed canonical SHA assertion, contract checks, build, Clippy, workspace tests, real-network runtime tests, and platform-specific offline verification.

Canonical `main` is now at documentation checkpoint `6d5a3ee7900a0705ebdcc9a4df4bfb3794ba7f6d`. The documentation checkpoint was independently verified in mirror run `38929461255` on Ubuntu and Windows Server 2025.

## Verified baseline

- DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT core language constructs.
- Indonesian lexical and grammar contracts.
- Language-independent AST and Semantic IR.
- Reference interpreter and structured diagnostics.
- Capability-aware process boundary and cross-platform process API shape.
- CLI commands `new`, `check`, `run`, `test`, `build`, `package`, `version`, and `ast`, with Indonesian conversational primary aliases.
- Deterministic `.oneir` and `.onepkg` textual artifacts.
- Strict v0.1 project manifest validation.
- Conformance fixtures and golden-output tests.
- Verified native network runtime slices: DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit runtime limits.
- General-purpose orchestration runtime foundation: typed capability requests, deterministic governance, governed adapters, normalized results/evidence, audit records, localized semantic-resolution fixtures, and `network.dns.resolve`.
- Experimental source-level DNS orchestration through `cari alamat`, explicit `network.discover` permission, exact target scope, governed adapter execution, normalized result, and CLI output.
- Experimental source-level filesystem-read orchestration through `baca berkas`, explicit `filesystem.read` permission, exact target scope, governed adapter execution, normalized UTF-8 result, and CLI output.
- Experimental source-level process-execution orchestration through `jalankan proses`, explicit `process.execute` permission, exact target scope, governed adapter execution, normalized process output, and CLI output, within the boundary stated below.

## Current experimental process boundary

```text
jalankan proses "rustc"
→ parser
→ semantic validation
→ typed orchestration IR
→ explicit target-scoped governance
→ process.execute adapter
→ normalized process output
→ CLI output
```

Exact-SHA evidence: source `9038ac75f197753ff1ab6af7478e63e4831903a2`, mirror run `38929072800`, Ubuntu + Windows Server 2025. The current implementation executes one explicitly targeted program without additional arguments and has TDD coverage for success, default denial, CLI permission, and missing-program backend failure.

## Explicit v0.1 limits

- `build` emits a deterministic `.oneir` IR/debug artifact; it is not a native executable compiler.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- The language-level orchestration catalog is not complete. DNS discovery, filesystem-read, and the limited process-execution vertical slice are currently experimental and independently verified through the source/CLI boundary.
- Process execution is experimental only within the explicitly targeted single-program/no-arguments boundary; this is not a claim of general process management, service management, shell scripting, or arbitrary command orchestration support.
- Native C/C++ FFI, Python/Node/JVM native bindings, package registry, IDE/editor, GUI framework integration, additional locales, and additional protocol/capability inventory beyond the verified boundary remain future work.

## Experimental orchestration

DNS source:

```text
cari alamat "127.0.0.1"
```

Explicit CLI permission:

```text
one jalankan main.one --izinkan network.discover --target 127.0.0.1
```

Filesystem-read source:

```text
baca berkas "data/contoh.txt"
```

Explicit CLI permission:

```text
one jalankan main.one --izinkan filesystem.read --target data/contoh.txt
```

Process source:

```text
jalankan proses "rustc"
```

Explicit CLI permission:

```text
one jalankan main.one --izinkan process.execute --target rustc
```

For all verified slices, no explicit permission means denial and a different target is outside the granted scope.

## Release labels

- **IMPLEMENTATION-COMPLETE:** in-scope source/docs/tests are present.
- **VERIFIED:** exact source revision has executable cross-platform evidence.
- **EXPERIMENTAL:** the declared vertical slice is verified but intentionally limited in coverage.
- **SUPPORTED:** reserved for a stronger platform/support claim.

## Verification infrastructure rule

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution. **Do not purchase an upgrade** merely to obtain hosted runner execution; use the independent verification path when the canonical hosted runner is unavailable.
