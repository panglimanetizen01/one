# One Requirements Baseline

This document records the accepted direction that must be considered when designing or implementing One. It does not imply that every requirement is already implemented.

## R1 — Human-oriented source

One source should be understandable to ordinary users and resemble clear instructions rather than conventional punctuation-heavy programming syntax.

## R2 — Formal semantics

Human-oriented syntax must map to formal grammar, AST, semantic analysis, and One IR. Ambiguous wording must not become arbitrary execution.

## R3 — General-purpose language

One must be capable of growing beyond scripting into serious application, systems, networking, data, web, developer-tooling, security, and other legitimate computing domains.

## R4 — Indonesian-first localization

Bahasa Indonesia is the initial language. The architecture must allow additional human-language grammars without forking semantic meaning.

## R5 — LLM independence

Programs must execute without an LLM. AI can assist authoring but cannot be the authoritative compiler, interpreter, permission system, or runtime.

## R6 — Portable execution

The language/toolchain must support multiple operating systems and environments through a stable semantic core, runtime/compiler abstraction, and platform adapters.

## R7 — Linux distribution breadth

The project must have a path to Ubuntu, Debian, Kali, Alpine, Arch, Fedora, and other Linux distributions. Distribution-specific behavior belongs in packaging/build/runtime integration layers, not the language semantics.

## R8 — Terminal breadth

The CLI must be usable from major shells and technical terminal environments, including desktop Linux terminals, SSH, CI, containers, and Android terminal environments such as Termux.

## R9 — Editor/IDE breadth

One must expose stable tooling interfaces suitable for VS Code, Acode, SuperCode, terminal editors, and future IDEs. Editor integrations must not redefine language semantics.

## R10 — Android breadth

One should support Android-hosted development where the host can provide the required runtime/toolchain, including Termux and Linux-container-style environments such as UserLAnd. Pydroid and similar applications are integration targets where their execution model permits One tooling.

## R11 — Architecture portability

The implementation should allow multiple CPU architectures, beginning with practical mainstream targets and expanding through backend/toolchain support rather than embedding architecture assumptions in the language.

## R12 — Interoperability

One must interoperate with existing languages, libraries, operating-system APIs, web technologies, and command/tool ecosystems through explicit interfaces.

## R13 — Security governance

External effects require explicit capability, target/resource scope, policy, adapter, and audit semantics as applicable. Security tooling is a valid domain; authority must remain explicit and controlled.

## R14 — Deterministic diagnostics

Language and governance diagnostics must be stable, structured, human-readable, and testable within their contracts.

## R15 — Progressive complexity

A beginner must be able to start simply while an advanced user can use precise types, effects, capabilities, modules, packages, interop, and platform controls.

## R16 — Test-first evolution

Every new capability must have a failing test before implementation where practical, followed by passing tests, linting, exact-revision verification, and documentation.

## R17 — Evidence-backed support status

The project must distinguish Specified, Designed, Implemented, Verified, and Supported. Source presence alone never establishes support.

## R18 — No hidden privilege

Aliases, localization, generated source, macros, libraries, adapters, or LLM assistance must not bypass capability or scope checks.

## R19 — Existing ecosystem compatibility

One should complement and integrate with existing ecosystems rather than requiring users to abandon established tools.

## R20 — Repository continuity

The repository documentation must preserve project intent, accepted decisions, requirements, and evidence boundaries so a new development session can resume without reconstructing prior conversation history.
