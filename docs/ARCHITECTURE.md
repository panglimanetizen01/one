# One Architecture Contract

## 1. Authoritative pipeline

```text
Localized Source
 -> Lexer
 -> Parser
 -> AST
 -> Name/Scope Analysis
 -> Type/Semantic Analysis
 -> One Semantic IR
 -> Reference Interpreter / Compiler Backend
 -> Runtime
 -> Platform Abstraction
 -> Host OS / Device
```

## 2. Semantic authority

The parser, semantic analyzer, and One IR are authoritative for program meaning. Runtime adapters implement approved effects; they do not reinterpret source language.

## 3. Host abstraction

Operating systems, distributions, terminals, editors, IDEs, containers, and Android applications are hosts or integration surfaces. They must not become semantic dependencies of the language.

## 4. CLI boundary

The One CLI is a stable host-facing interface. It should expose predictable commands for checking, running, testing, building, formatting, packaging, project creation, diagnostics, and tooling integration as those capabilities mature.

## 5. Runtime boundary

The runtime provides execution semantics, effects/capabilities, normalized values, diagnostics, audit records, and platform abstractions. Platform-specific code stays behind explicit interfaces.

## 6. Compiler/backend boundary

The compiler/toolchain may target native platforms, portable runtimes, or other approved backends. Backend selection must not alter language semantics.

## 7. Cross-distro strategy

Distribution support should be achieved through portable artifacts, source builds, dependency contracts, packaging metadata, and automated compatibility testing. The core language must not contain Ubuntu/Kali/Alpine-specific semantics.

## 8. Cross-terminal strategy

Terminal compatibility is primarily a CLI and process/IO contract. The language source format remains independent of shell quoting and shell syntax. One programs should not require users to translate One semantics into Bash/Zsh/Fish constructs.

## 9. Editor/IDE strategy

Editor support should be based on standard or well-defined interfaces such as:

- CLI;
- formatter;
- language server / LSP;
- diagnostics;
- project metadata;
- build/test commands;
- debugger interface where implemented.

## 10. Android strategy

Android support is layered. Termux can host a native One CLI when the architecture and dependencies permit. UserLAnd and similar environments can host One through their Linux environment. Other applications are supported when their execution model can invoke the One toolchain. The language itself remains unaware of which host application launched it.

## 11. Interoperability strategy

Interop belongs below or beside the semantic core through explicit FFI, process, library, API, or generated-artifact boundaries. Initial planned ecosystems include C/C++, Python, Java/JVM, JavaScript/Node.js, HTML/CSS/web, and OS APIs.

## 12. Localization strategy

Localized grammars produce the same semantic identity. Locale-specific parser rules must map to common AST/semantic constructs rather than create separate language implementations.

## 13. Capability architecture

External operations follow:

```text
source intent
 -> semantic operation
 -> typed capability request
 -> target/resource scope
 -> policy evaluation
 -> deterministic adapter selection
 -> adapter execution
 -> normalized result
 -> audit/diagnostic
```

## 14. Progressive precision

The same semantic operation may have a beginner-friendly surface and an expert surface with explicit parameters and controls. Both must lower to compatible semantic representations.

## 15. Scalability

The architecture must support small scripts and large projects without forcing the beginner surface to expose every advanced mechanism. Modules, packages, dependency management, build systems, testing, debugging, deployment, and interop are planned layers rather than ad hoc syntax additions.

## 16. Security boundary

Capability checks occur before privileged external execution. Wording, localization, aliases, generated code, and adapters cannot grant authority by themselves.

## 17. Current orchestration pattern

The current experimental orchestration implementation follows:

```text
One source
 -> parser / semantic validation
 -> typed orchestration IR
 -> capability + scope governance
 -> registered adapter
 -> normalized result
 -> CLI/runtime output
```

This pattern is the preferred template for future real-world capabilities.


## 18. Repository integrity engine

Repository integrity checks are a first-party toolchain capability, separate from ONE source-language semantics. The target architecture is:

```text
ONE CLI / tooling interface
 -> Integrity Engine API
 -> deterministic detector registry and scope selection
 -> normalized findings + coverage metadata
 -> terminal diagnostics / JSON or Markdown report
```

The engine must make the selected scope, executed detectors, skipped detectors, finding counts, and evidence revision explicit. “Not requested” is not “passed.” It must not autonomously edit source, merge changes, or release artifacts. Any future repair operation requires a separate allowlist, preview, scope, and approval contract.

The current Python scanner remains an independent developer/CI reference while native implementation and parity are built. The first-party command must not require Python at runtime. Implementation should reuse the existing ONE toolchain architecture; the current implementation language is Rust, so “native” does not imply that ONE is already self-hosting. Cross-platform support and default-command status require exact-revision executable evidence.


### Current implementation status

The first read-only Rust CLI preview is implemented at `crates/one-cli/src/integrity.rs` and is exposed as `one pindai`. It currently covers a subset of the target detectors only; it does not yet emit JSON/Markdown reports or provide full security, YAML, artifact-byte, CI-log, or baseline checks. The Python scanner remains the reference developer/CI tool until parity and exact-SHA cross-platform evidence are accepted. This preview is not yet a release or CI gate.
