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


## ADR-0016 — Human instruction grammar acceptance invariants

**Status:** Accepted

See [ADR-0016](decisions/ADR-0016-accept-language-invariants.md) for explicit grammar, ambiguity rejection, and v0.1 isolation invariants.

## ADR-0017 — Human frontend integration, modules, mappings, and persistence

**Status:** Accepted for the experimental core/runtime slice

See [ADR-0017](decisions/ADR-0017-human-vnext-core-runtime.md) for the canonical-pipeline adapter, module visibility, mapping semantics, persistence format, and compatibility boundaries.


## ADR-0018 — CircleCI-first verification for the active remediation

**Status:** Accepted for the active verification remediation.

The active branch `fix/windows-temp-path-canonicalization` uses CircleCI as its primary CI execution and evidence source. Authenticated Termux API access is limited to read-only retrieval and validation of hosted evidence; it is not an implementation or CI administration path. This is an operational CI-provider decision, not a change to One's platform support contract or release gate. Exact-SHA evidence remains mandatory; a successful CircleCI job does not by itself authorize merge or release.


## ADR-0019 — GitHub repository as source of truth, CircleCI as canonical CI, Termux verification-only

**Status:** Accepted — 2026-11-11

**Decision**
- The live GitHub repository `panglimanetizen01/one` is the single source of truth for source code, tests, workflows, documentation, branches, commits, PRs, and accepted state.
- CircleCI is the canonical CI provider for the One source repository and replaces GitHub Actions as the normal source-repository CI path.
- Termux is restricted to read-only, post-CircleCI verification and independent validation of an exact GitHub SHA already proven by the hosted pipeline. It is not an implementation, repair, authoring, commit, push, or CI administration environment.
- Every material change must update all affected repository documentation and the existing Notion Project ONE skill/checkpoint. New sessions must load enabled Notion skills, read live `AGENTS.md` and checkpoints, resolve live GitHub refs/SHAs, inspect CircleCI, and continue without repeating successful unchanged scopes.

**Rationale**

GitHub must remain the authoritative, reviewable record of changes. Hosted CircleCI evidence must be tied to the exact GitHub commit. Local Termux is useful for independent validation and evidence inspection but must not become a second, divergent implementation source.

**Consequences**
- Existing source-repository GitHub Actions workflow files remain preserved during migration until their coverage is mapped and CircleCI parity is demonstrated; removal/disablement requires explicit authorization.
- The current CircleCI `linux-verify` job passing does not establish native Windows/Android runtime or full matrix parity.
- This decision does not authorize merge, release, protected environment changes, secret changes, branch-protection changes, or changes to fixed acceptance weights.
- Earlier local-verification contracts remain historical evidence/proposals and do not supersede this decision.
