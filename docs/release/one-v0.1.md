# One v0.1 Release Contract

Status: **PENDING FINAL EXACT-SHA VERIFICATION**.

## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

## Supported in v0.1

- DATA: declaration and assignment.
- ACTION: display, list creation, list mutation, and greeting through explicit productions.
- CONDITION: deterministic comparison phrases and `kalau tidak` branch.
- LOOP: `untuk setiap` and explicit boolean `selama` conditions.
- FUNCTION: parameters, explicit `hasilnya adalah` result, and calls using `dengan` / `dan`.
- EVENT: exact trigger registration and dispatch.
- Indonesian lexical and grammar contracts.
- Language-independent AST and Semantic IR.
- Reference interpreter.
- Structured diagnostics with source spans and stable error codes.
- Capability-aware process boundary.
- Cross-platform process API shape using argument vectors and preserved exit status/stdout/stderr.
- CLI commands `new`, `check`, `run`, `test`, `build`, `package`, `version`, and `ast`, with Indonesian conversational primary aliases.
- `one test` executes `src/main.one` and, when present, sorted deterministic `tests/conformance/valid/*.one` and `tests/conformance/invalid/*.one` fixtures.
- Deterministic `.oneir` and `.onepkg` textual artifacts.
- Strict v0.1 project manifest validation for `nama` and `versi`.
- Conformance fixtures and golden-output tests.
- Linux/Windows CI definition.
- Verified native network runtime slices: DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP, within the explicit runtime limits documented by the network contract.

## Explicit v0.1 limits

- Names are grammar-bounded and may contain multiple words. Structural words are interpreted according to the active production; the parser does not globally reserve them.
- The conversational loop form `selama masih ada ...` is not part of v0.1; only explicit boolean conditions are accepted.
- `build` currently emits a deterministic `.oneir` IR artifact; it is not a native executable compiler.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- Native C/C++ FFI is future work.
- Python, Node.js, and Java/JVM integration is currently process-boundary interoperability, not native language bindings.
- HTTP/3, broad discovery/routing/VPN/industrial/storage/mesh/DTN/telemetry/remote-access adapters, physical network drivers, complete network source grammar, and the remaining protocol inventory are future work.
- The verified application-protocol implementations are bounded runtime slices, not claims of complete RFC feature coverage.
- Package registry, full dependency registry, IDE/editor tooling, GUI framework integration, and additional locale grammars are future work.

## Release acceptance checklist

- [x] Linux compilation has passed on the post-audit source revision.
- [x] Windows compilation has passed on the post-audit source revision.
- [x] Linux unit/integration/conformance tests have passed on the post-audit source revision.
- [x] Windows unit/integration/conformance tests have passed on the post-audit source revision.
- [x] Linux real-network runtime tests have passed on the post-audit source revision.
- [x] Windows real-network runtime tests have passed on the post-audit source revision.
- [x] Linux offline verification has passed on the post-audit source revision.
- [x] Windows offline verification has passed on the post-audit source revision.
- [x] Contract-artifact closure has passed on the post-audit source revision.
- [x] Deterministic `.oneir` and `.onepkg` checks have passed on the post-audit source revision.
- [x] Capability/security tests have passed on the post-audit source revision.
- [x] Interoperability tests have passed on the post-audit source revision.
- [ ] The synchronized documentation SHA has passed one final independent Ubuntu/Windows matrix.

## Evidence

The latest post-audit independent verification run passed the then-current canonical source on both Ubuntu and Windows Server 2025. The temporary public repository `panglimanetizen01/one-windows-verification` is verification infrastructure only. GitHub `main` remains the source of truth.

No paid GitHub upgrade was used or is required. **Do not purchase an upgrade** merely to obtain hosted runner execution.

## Release labels

- **IMPLEMENTATION-COMPLETE:** all in-scope source/docs/tests are present in the canonical repository.
- **VERIFIED:** the exact current canonical SHA has actual cross-platform execution evidence.

Current state: **PENDING FINAL EXACT-SHA VERIFICATION**.
