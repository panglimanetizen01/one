# One v0.1 Release Contract

Status: **VERIFIED BASELINE WITH EXPERIMENTAL DNS AND FILESYSTEM-READ ORCHESTRATION**.

## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

## Current exact-SHA evidence boundary

The latest independently verified implementation revision is `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b`.

Independent mirror run: `38901366447` in `panglimanetizen01/one-windows-verification`.

Ubuntu and Windows Server 2025 both checked out that exact SHA and passed canonical SHA assertion, contract checks, build, Clippy, workspace tests, real-network runtime tests, deterministic artifact checks, DNS success/failure classification, filesystem-read orchestration runtime, and platform-specific offline verification.

The canonical `main` is newer than that evidence revision. Evidence for one commit does not automatically verify a later commit. The current post-verification commits must therefore not be described as independently exact-SHA verified until a new verification run records the later SHA.

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

## Explicit v0.1 limits

- `build` emits a deterministic `.oneir` IR/debug artifact; it is not a native executable compiler.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- The language-level orchestration catalog is not complete. Only the DNS discovery and filesystem-read vertical slices are currently experimental and independently verified through the source/CLI boundary.
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

For both slices, no explicit permission means denial and a different target is outside the granted scope.

## Release labels

- **IMPLEMENTATION-COMPLETE:** in-scope source/docs/tests are present.
- **VERIFIED:** exact source revision has executable cross-platform evidence.
- **EXPERIMENTAL:** the declared vertical slice is verified but intentionally limited in coverage.
- **SUPPORTED:** reserved for a stronger platform/support claim.

## Verification infrastructure rule

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution. **Do not purchase an upgrade** merely to obtain hosted runner execution; use the independent verification path when the canonical hosted runner is unavailable.
