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


## ADR-0017 — CircleCI is the canonical Linux CI execution provider

**Status:** Accepted

CircleCI is the canonical CI execution provider for Project ONE's Linux product and Linux release checks. GitHub remains the canonical source repository, version-control host, pull-request system, and code-review surface. Windows Server 2025 host/product/native verification is executed by the independent `panglimanetizen01/one-windows-verification` repository, which checks out the private canonical ONE repository at a full explicit source SHA and records the source SHA separately from the verifier SHA.

The canonical CircleCI configuration must retain Linux product/release gates and must not define Windows-host executor jobs. Linux-host MinGW-w64 cross-compilation may remain for Windows-target backend regression coverage, but it is not Windows-host verification. The independent Windows result and CircleCI Linux result must both bind to the same exact source SHA before cross-platform verification is claimed.

The migration branch removes the three Project ONE GitHub Actions CI workflows to avoid a parallel product-CI path in the canonical repository. The default branch remains unchanged until the migration PR is reviewed. No CI migration may alter One grammar, semantics, compiler, interpreter, runtime, CLI, or user experience merely to accommodate tooling.
