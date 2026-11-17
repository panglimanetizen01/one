# One Decisions

This file records accepted architectural decisions so future sessions do not reopen settled questions without evidence that a decision must change.

## ADR-0001 — Indonesian-first, global localization

**Status:** Accepted

One starts with Bahasa Indonesia while sharing a common semantic core with future locales.

## ADR-0002 — General-purpose language

**Status:** Accepted

One is not restricted to education, scripting, automation, web, or one technical domain. Its architecture must support serious general-purpose programming.

## ADR-0003 — Human-oriented but formally deterministic

**Status:** Accepted

The surface may resemble conversational instructions, but grammar and semantics are formal and deterministic. Ambiguity is not delegated to an LLM at runtime.

## ADR-0004 — LLM-independent execution

**Status:** Accepted

An LLM may assist users or developers, but One program meaning and execution belong to the language toolchain and runtime.

## ADR-0005 — Common semantic IR

**Status:** Accepted

Localized syntax and future frontends converge on a shared semantic representation before runtime/backend execution.

## ADR-0006 — Capability-governed external effects

**Status:** Accepted

External effects require explicit capability and scope governance. Adapters are implementation backends, not semantic authorities.

## ADR-0007 — Cross-platform and cross-environment

**Status:** Accepted

One must not be tied to a specific OS, Linux distro, shell, terminal, editor, IDE, or Android application. Hosts integrate with One through stable tooling/runtime interfaces.

## ADR-0008 — Linux distribution breadth

**Status:** Accepted

Ubuntu, Debian, Kali, Alpine, Arch, Fedora, and other Linux distributions are compatibility targets. Distribution-specific differences belong in packaging/build/platform layers.

## ADR-0009 — Terminal independence

**Status:** Accepted

One runs from technical terminals but is not a shell language. Bash, Zsh, Fish, BusyBox, SSH, CI, and Android terminals are host surfaces.

## ADR-0010 — Editor/IDE integration through tooling contracts

**Status:** Accepted

VS Code, Acode, SuperCode, and other editors/IDEs integrate through CLI, LSP, formatter, diagnostics, debugger, project metadata, and related interfaces rather than semantic forks.

## ADR-0011 — Security as a general-purpose domain

**Status:** Accepted

One may support legitimate security engineering and authorized penetration testing. Security-sensitive effects remain governed by capability, scope, policy, and audit controls.

## ADR-0012 — Repository as persistent project memory

**Status:** Accepted

Project intent, requirements, decisions, architecture, compatibility goals, and evidence boundaries must be stored in the repository so a new session can recover the project context without relying on prior chat history.

## ADR-0013 — Evidence before support claims

**Status:** Accepted

A feature or platform is not called Verified or Supported without executable evidence at the relevant source revision and environment boundary.

## ADR-0014 — Originality framing

**Status:** Accepted

One does not claim absolute historical novelty for every individual idea. Its intended distinctiveness is the concrete combination of human-oriented syntax, formal deterministic semantics, shared IR, native localization, progressive complexity, general-purpose scope, portability, interoperability, and governance.

## ADR-0015 — v0.1 AST schema is implementation-aligned and explicitly bounded

**Status:** Accepted

For v0.1, the normative AST schema is the public structure in `crates/one-core/src/ast.rs`: statement variants, normalized string names, expression enums, generic-but-grammar-validated action nodes, and statement-introducer span anchors. The AST contract documents the limitations directly, including missing expression/full-statement spans and missing component spans for multi-word names.

This decision avoids treating an aspirational draft schema as if it were already implemented or silently changing the public AST to a much larger shape without tests. It does not remove the requirement for deterministic parsing, formal grammar boundaries, stable diagnostics, or LLM-independent semantics.

Any future addition of structured `Name` nodes, component spans, full node ranges, or action-specific variants requires fail-first tests and a coordinated compatibility review across parser, semantic analysis, IR lowering, diagnostics, and tooling. It must not silently alter v0.1 meaning.


## ADR-0018 — Native repository integrity engine; Python scanner remains an adapter

**Status:** Accepted architectural direction; initial Rust CLI preview implemented, parity and acceptance pending.

ONE's repository-integrity capability is a first-party toolchain responsibility, but `tools/one_health.py` is not itself part of the ONE language runtime or source-language semantics. Keep it as an independent developer/CI reference scanner during migration; do not make Python a runtime prerequisite for using ONE or for the eventual first-party integrity command.

The target is a native integrity engine exposed through the ONE toolchain/CLI and implemented against ONE's existing architecture and contracts. It must provide deterministic, auditable checks for revision/SHA identity, repository paths and structure, workflow configuration, contracts, documentation/Markdown, and build/evidence artifacts. Findings must have stable rule identifiers, severity, confidence, location, evidence, and remediation guidance. Scope selection must report what was checked and what was not checked; an unrequested detector must never be represented as passing.

Migration is evidence-gated:

1. Specify the report and detector contracts independently of Python implementation details.
2. Implement the native engine and fail-first regression tests in the current ONE toolchain (currently Rust-based).
3. Run native and Python scanners against the same repository revision and compare results, explaining intentional differences and adjudicating false positives/misses.
4. Verify supported host platforms and exact source revision in CI before making the native command the default.
5. Retain or retire the Python adapter only after equivalent coverage and operational evidence are established.

“Native” here means first-party and free of a Python runtime dependency, not that the current compiler is already self-hosted in the ONE language. Rust is an implementation technology of the present toolchain and may be replaced only through a separately verified migration. JSON/Markdown may remain report interchange formats; their use does not make them the language or semantic authority of ONE.

This decision does not authorize merging the scanner-hardening PR, bypassing repository protections, or treating existing verification evidence as current for a changed SHA.

**Implementation status (2026-11-17):** the initial read-only Rust preview is exposed as `one pindai` and `one pindai keamanan`. The Python scanner remains the independent developer/CI reference. The native preview is not a parity replacement, release gate, or default gate. CircleCI is the canonical CI provider; GitHub Actions metadata collection, where retained, is historical-only. Promotion requires same-checkout parity review and exact-SHA supported-platform evidence.
