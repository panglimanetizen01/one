# One v0.1 Release Contract

Status: **VERIFIED BASELINE `dba4fad49854952e256ae41744f57f93e39167a8`**.

## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

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

## Explicit v0.1 limits

- `build` emits a deterministic `.oneir` IR/debug artifact; it is not a native executable compiler.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- The orchestration foundation is verified at the runtime-library boundary, but **language-level One-source orchestration is not yet supported**. The current CLI still follows the established parse → analyze → lower → interpreter path without a verified source-to-Orchestrator bridge.
- Native C/C++ FFI, Python/Node/JVM native bindings, package registry, IDE/editor, GUI framework integration, additional locales, and the remaining protocol inventory are future work.

## Release acceptance evidence

Independent verification run `38872070590` in `panglimanetizen01/one-windows-verification` checked out canonical source baseline `dba4fad49854952e256ae41744f57f93e39167a8` and passed on Ubuntu and Windows Server 2025. Both jobs passed canonical SHA verification, contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, and platform-specific offline verification. The offline verifiers included deterministic `.oneir` and `.onepkg` comparisons.

The later documentation-only synchronization revision is intentionally treated as a separate canonical revision and receives its own fresh exact-SHA verification before it is called the verified current head.

## Release labels

- **IMPLEMENTATION-COMPLETE:** in-scope source/docs/tests are present.
- **VERIFIED:** exact source revision has executable cross-platform evidence.
- **SUPPORTED:** reserved for a stronger platform/support claim.

## Verification infrastructure rule

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution. **Do not purchase an upgrade** merely to obtain hosted runner execution; use the independent verification path when the canonical hosted runner is unavailable.
