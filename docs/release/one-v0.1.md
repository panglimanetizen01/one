# One v0.1 Release Contract

Status: **VERIFIED BASELINE WITH EXPERIMENTAL DNS, FILESYSTEM-READ, PROCESS-EXECUTION, AND FILESYSTEM-WRITE ORCHESTRATION**.

## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

## Current exact-SHA evidence boundary

The latest cross-platform source/runtime revision independently verified is `9038ac75f197753ff1ab6af7478e63e4831903a2` via mirror run `38929072800` on Ubuntu and Windows Server 2025.

The current canonical `main` is `a332dc5e58760460c26ebebb33f32f910bfdf4b3`. This newer SHA has executable verification on Termux/Android: contract check, build, Clippy, targeted filesystem-write CLI test, full workspace tests, and offline verification all passed. That evidence does not establish Ubuntu/Windows support for the newer SHA.

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
- Experimental source-level DNS orchestration through `cari alamat`.
- Experimental source-level filesystem-read orchestration through `baca berkas`.
- Experimental source-level process-execution orchestration through `jalankan proses`, limited to one explicitly targeted program without arguments and backed by cross-platform evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2`.
- Experimental source-level filesystem-write orchestration through `tulis ke berkas`, limited to one text payload and one explicitly targeted file, with `filesystem.write` permission. The current exact-SHA evidence is Termux/Android only.

## Current experimental filesystem-write boundary

```text
tulis ke berkas "data/hasil.txt" dengan "halo dari One"
→ parser
→ semantic validation
→ typed orchestration IR
→ explicit target-scoped governance
→ filesystem.write adapter
→ CLI output
```

Explicit permission:

```text
one jalankan main.one --izinkan filesystem.write --target data/hasil.txt
```

Exact-SHA evidence: `a332dc5e58760460c26ebebb33f32f910bfdf4b3`, Termux/Android. The targeted CLI regression test, filesystem-write runtime test, full workspace tests, and offline verification pass. Cross-platform verification for this SHA is still pending.

## Explicit v0.1 limits

- `build` emits a deterministic `.oneir` IR/debug artifact; it is not a native executable compiler.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- The language-level orchestration catalog is not complete.
- Filesystem write is experimental only within the single-text-payload/single-target boundary and the currently verified Android/Termux platform boundary. It is not a claim of general filesystem management or sandbox/path-confinement guarantees.
- Process execution is experimental only within the explicitly targeted single-program/no-arguments boundary; this is not a claim of general process management, service management, shell scripting, or arbitrary command orchestration support.
- Native C/C++ FFI, Python/Node/JVM native bindings, package registry, IDE/editor, GUI framework integration, additional locales, and additional protocol/capability inventory beyond the verified boundary remain future work.

## Experimental orchestration

Filesystem-write source:

```text
tulis ke berkas "data/hasil.txt" dengan "halo dari One"
```

Explicit CLI permission:

```text
one jalankan main.one --izinkan filesystem.write --target data/hasil.txt
```

For the filesystem-write slice, no explicit permission means denial and a different target is outside the granted scope.

## Release labels

- **IMPLEMENTATION-COMPLETE:** in-scope source/docs/tests are present.
- **VERIFIED:** exact source revision has executable evidence for the stated platform boundary.
- **EXPERIMENTAL:** the declared vertical slice is verified but intentionally limited in coverage or platform boundary.
- **SUPPORTED:** reserved for a stronger platform/support claim.

## Verification infrastructure rule

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution. **Do not purchase an upgrade** merely to obtain hosted runner execution; use the independent verification path when the canonical hosted runner is unavailable.
