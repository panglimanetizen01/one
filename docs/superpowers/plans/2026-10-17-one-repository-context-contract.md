# Repository Context Contract Implementation Plan

> **For agentic workers:** This plan implements the accepted repository-context design. Read `AGENTS.md` and `docs/ONE_CONSTITUTION.md` before further work.

**Goal:** Make the One repository a persistent source of project intent, decisions, requirements, compatibility targets, and development rules so new sessions can resume work without reconstructing prior chat history.

**Architecture:** Use Markdown contracts with `AGENTS.md` as the agent entry point and `docs/ONE_CONSTITUTION.md` as the highest-level project contract. Split stable concerns into focused documents and keep executable implementation/evidence as the authority for actual status.

**Tech Stack:** Markdown, Git/GitHub, existing One Rust toolchain and verification infrastructure.

**Spec:** `docs/ONE_CONSTITUTION.md`

## Global Constraints

- `AGENTS.md` is the first-read operational entry point.
- One remains Indonesian-first and globally localizable.
- One remains a formal deterministic general-purpose language.
- One remains LLM-independent for execution semantics.
- Cross-platform and cross-environment compatibility is an architectural requirement.
- Security-sensitive effects require explicit governance.
- Documentation must not claim support without evidence.
- Repository history and accepted decisions must remain reviewable through Git.

## Review Focus

- New sessions must have an unambiguous first-read path — covered by `AGENTS.md`.
- Constitution must not overclaim implementation status — covered by evidence/status rules in the constitution and development protocol.
- Cross-distro/editor/terminal goals must remain host integration requirements rather than semantic forks — covered by compatibility and architecture contracts.
- Accepted decisions must not be silently reopened — covered by `docs/DECISIONS.md`.
- Documentation drift must be detectable and correctable — covered by `docs/DEVELOPMENT_PROTOCOL.md`.

### Task 1: Establish agent entry point

**Files:**
- Create: `AGENTS.md`

- [x] Define mandatory reading order and source-of-truth hierarchy.
- [x] Encode evidence, change, security, portability, and completion rules.

### Task 2: Establish project constitution

**Files:**
- Create: `docs/ONE_CONSTITUTION.md`

- [x] Record identity, vision, mission, language philosophy, architecture invariants, general-purpose scope, portability, localization, interoperability, security, and evidence principles.

### Task 3: Establish focused contracts

**Files:**
- Create: `docs/REQUIREMENTS.md`
- Create: `docs/ARCHITECTURE.md`
- Create: `docs/COMPATIBILITY.md`
- Create: `docs/SECURITY.md`
- Create: `docs/LOCALIZATION.md`
- Create: `docs/INTEROPERABILITY.md`
- Create: `docs/DECISIONS.md`
- Create: `docs/DEVELOPMENT_PROTOCOL.md`
- Create: `docs/verification/repository-context-milestone.md`

- [x] Separate stable concerns so future sessions can load only the relevant contract while preserving the common constitution.

### Task 4: Connect the entry point from README

**Files:**
- Modify: `README.md`

- [x] Add a persistent project-context section pointing agents and contributors to the repository contract.

### Task 5: Verify repository consistency

- [x] Verify all newly referenced paths exist in the same commit.
- [x] Verify links and terminology are internally consistent.
- [x] Verify the resulting commit is on the intended branch.
- [x] Treat the commit SHA as the durable checkpoint for future sessions.
