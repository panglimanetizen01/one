# One v0.1 Release Contract

Status: **VERIFIED FOR CANONICAL SHA `be2e65852e9a4be833e659524a12c6359769665b`**.

## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

## Verified in v0.1

- DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT core language constructs.
- Indonesian lexical and grammar contracts.
- Language-independent AST and Semantic IR.
- Reference interpreter.
- Structured diagnostics with source spans and stable error codes.
- Capability-aware process boundary.
- Cross-platform process API shape using argument vectors and preserved exit status/stdout/stderr.
- CLI commands `new`, `check`, `run`, `test`, `build`, `package`, `version`, and `ast`, with Indonesian conversational primary aliases.
- Deterministic `.oneir` and `.onepkg` textual artifacts.
- Strict v0.1 project manifest validation.
- Conformance fixtures and golden-output tests.
- Verified native network runtime slices: DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP, within explicit runtime limits.
- General-purpose orchestration runtime foundation: typed capability requests, deterministic governance, governed adapters, normalized results/evidence, audit records, localized semantic-resolution fixtures, and `network.dns.resolve`.

## Explicit v0.1 limits

- `build` currently emits a deterministic `.oneir` IR artifact; it is not a native executable compiler.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- The orchestration foundation is verified at the runtime-library boundary, but **language-level One-source orchestration is not yet supported**. The current CLI still executes the established parse → analyze → lower → interpreter path without a verified source-to-Orchestrator bridge.
- Native C/C++ FFI is future work.
- Python, Node.js, and Java/JVM integration is currently process-boundary interoperability, not native language bindings.
- HTTP/3, broad discovery/routing/VPN/industrial/storage/mesh/DTN/telemetry/remote-access adapters, physical network drivers, complete network source grammar, and remaining protocol inventory are future work.
- Package registry, full dependency registry, IDE/editor tooling, GUI framework integration, and additional locale grammars are future work.

## Release acceptance checklist

- [x] Linux compilation passed on canonical SHA `be2e65852e9a4be833e659524a12c6359769665b`.
- [x] Windows compilation passed on the same canonical SHA.
- [x] Linux unit/integration/conformance tests passed.
- [x] Windows unit/integration/conformance tests passed.
- [x] Linux real-network runtime tests passed.
- [x] Windows real-network runtime tests passed.
- [x] Linux offline verification passed.
- [x] Windows offline verification passed.
- [x] Contract-artifact closure passed.
- [x] Deterministic `.oneir` and `.onepkg` checks passed on both offline paths.
- [x] Capability/security and interoperability tests passed as part of the workspace verification gates.
- [x] The orchestration runtime foundation passed the same independent matrix.
- [ ] Language-level One-source orchestration is not a v0.1 release claim yet.
- [ ] Documentation changes in this synchronization branch require one final fresh exact-SHA verification after merge.

## Evidence

Independent verification run `38871435579` in `panglimanetizen01/one-windows-verification` checked out canonical source commit `be2e65852e9a4be833e659524a12c6359769665b` and passed on Ubuntu and Windows Server 2025. The real-network stage and platform-specific offline stage passed on both platforms. The offline scripts included deterministic `.oneir` and `.onepkg` comparisons.

The temporary public repository `panglimanetizen01/one-windows-verification` is verification infrastructure only. GitHub `main` remains the source of truth.

No paid GitHub upgrade was used or is required. **Do not purchase an upgrade** merely to obtain hosted runner execution.

## Release labels

- **IMPLEMENTATION-COMPLETE:** all in-scope source/docs/tests are present in the canonical repository.
- **VERIFIED:** the exact canonical SHA has actual cross-platform execution evidence from an independent matrix.
- **SUPPORTED:** reserved for a stronger platform/support claim; not inferred merely from implementation or registry metadata.

Current verified source: **`be2e65852e9a4be833e659524a12c6359769665b`**. Any later canonical SHA requires fresh exact-SHA verification.
