# One v0.1 Release Contract

Status: **VERIFICATION-PENDING** for the current canonical source while the post-HTTP/2 contract-artifact audit is reverified at an exact SHA.

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

A fresh exact-SHA verification run is required after the current audit changes.

- [ ] Linux compilation succeeds on the current canonical SHA.
- [ ] Windows compilation succeeds on the current canonical SHA.
- [ ] Linux unit/integration/conformance tests succeed on the current canonical SHA.
- [ ] Windows unit/integration/conformance tests succeed on the current canonical SHA.
- [ ] Linux real-network runtime tests succeed on the current canonical SHA.
- [ ] Windows real-network runtime tests succeed on the current canonical SHA.
- [ ] Linux offline verification succeeds on the current canonical SHA.
- [ ] Windows offline verification succeeds on the current canonical SHA.
- [ ] Contract-artifact closure check succeeds on the current canonical SHA.
- [ ] Deterministic `.oneir` and `.onepkg` checks succeed.
- [ ] Capability/security tests succeed.
- [ ] Interoperability tests preserve exit status/stdout/stderr and reject denied capabilities.
- [ ] Documentation and grammar are aligned with the implemented language behavior and verified network boundary.
- [ ] No known failing test remains in the fresh current-SHA verification run.

## Evidence

Previous independent verification run `38781871063` verified canonical implementation commit `c2fecc0a488d335c73ff3abcc37f2f5d28a59a1f`, including HTTP/2. It is retained as historical evidence and is not used as proof for the current post-audit SHA.

The current release gate will use `panglimanetizen01/one-windows-verification` to execute the exact current canonical commit on Ubuntu and Windows Server 2025.

The private canonical GitHub Actions environment may remain blocked if its hosted jobs fail before usable workflow-step evidence. **Do not purchase an upgrade** merely to obtain hosted runner execution. The portable independent verification path is the project contract's fallback.

## Release labels

- **IMPLEMENTATION-COMPLETE:** all in-scope source/docs/tests are present in the canonical repository.
- **VERIFICATION-PENDING:** required execution evidence is incomplete for the current canonical SHA.
- **VERIFIED:** all acceptance checklist items have actual current-SHA evidence.

Current state: **VERIFICATION-PENDING**.
