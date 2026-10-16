# One v0.1 Release Contract

Status: **VERIFIED BASELINE WITH EXPERIMENTAL DNS ORCHESTRATION**.

## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

## Current exact-SHA evidence

Canonical source revision independently verified: `e43b31fe9d7a5f62de4d3863b358aa28241fbf9c`.

Independent mirror run: `38896135846` in `panglimanetizen01/one-windows-verification`.

Ubuntu job `116956935554` and Windows Server 2025 job `116956935967` both checked out that exact SHA and passed canonical SHA assertion, contract checks, build, Clippy, workspace tests, real-network runtime tests, and platform-specific offline verification. Deterministic `.oneir` and `.onepkg` checks, DNS success/failure classification, and orchestration runtime/source-level DNS tests also passed.

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

## Explicit v0.1 limits

- `build` emits a deterministic `.oneir` IR/debug artifact; it is not a native executable compiler.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- The language-level orchestration catalog is not complete. Only the DNS discovery vertical slice is currently experimental and verified through the source/CLI boundary.
- Native C/C++ FFI, Python/Node/JVM native bindings, package registry, IDE/editor, GUI framework integration, additional locales, and the remaining protocol inventory are future work.

## Experimental DNS orchestration

Source:

```text
cari alamat "127.0.0.1"
```

Explicit CLI permission:

```text
one jalankan main.one --izinkan network.discover --target 127.0.0.1
```

No explicit permission means denial. A different target is outside the granted scope.

## Release labels

- **IMPLEMENTATION-COMPLETE:** in-scope source/docs/tests are present.
- **VERIFIED:** exact source revision has executable cross-platform evidence.
- **EXPERIMENTAL:** the declared vertical slice is verified but intentionally limited in coverage.
- **SUPPORTED:** reserved for a stronger platform/support claim.

## Verification infrastructure rule

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution. **Do not purchase an upgrade** merely to obtain hosted runner execution; use the independent verification path when the canonical hosted runner is unavailable.
