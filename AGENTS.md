# One Agent Operating Contract

> **Read this file before making changes to the One repository.**

## 1. Purpose

This file is the entry point for AI agents, automated coding agents, and human contributors working on One. It prevents loss of project context between sessions by making the repository itself the persistent working contract.

## 2. Required reading order

Before modifying code, tests, specifications, or documentation:

1. Read `AGENTS.md`.
2. Read `docs/ONE_CONSTITUTION.md`.
3. Read the relevant domain document(s), especially `ARCHITECTURE.md`, `REQUIREMENTS.md`, `COMPATIBILITY.md`, `SECURITY.md`, `LOCALIZATION.md`, or `INTEROPERABILITY.md` as applicable.
4. Read applicable decisions in `docs/DECISIONS.md`.
5. Read current verification evidence before making claims about implementation status.
6. Inspect the repository state, branch, HEAD, and relevant source/tests.

Do not reconstruct project intent from memory when the repository contract already records it.

## 3. Source-of-truth hierarchy

When sources disagree, resolve them in this order:

1. Executable source and tests for what the current implementation actually does.
2. `docs/ONE_CONSTITUTION.md` for accepted project invariants and non-negotiable direction.
3. `docs/DECISIONS.md` for accepted architectural decisions.
4. Domain specifications and plans for scoped design intent.
5. Verification evidence for what has actually been executed and proven.
6. README and user-facing summaries.

A documentation statement must not be used to claim an implementation that source/tests do not establish.

## 4. Operating law

```text
DISCOVER
  -> EXECUTE
  -> VERIFY
  -> FIX
  -> REVERIFY
  -> DOCUMENT
  -> CONTINUE
```

Use:

```text
NO EVIDENCE -> NO CLAIM -> NO COMMAND -> NO PATCH
```

For bugs or anomalies:

```text
ROOT CAUSE
  -> FAILING TEST
  -> FIX
  -> VERIFY
  -> DOCUMENT
  -> CONTINUE
```

## 5. Non-negotiable engineering rules

- Build One; do not merely audit it.
- Preserve deterministic semantics.
- Do not make an LLM the semantic or execution authority.
- Do not replace formal grammar with unconstrained natural-language guessing.
- Do not weaken capability, scope, or security boundaries to make a test pass.
- Prefer minimal changes with direct evidence.
- Every new capability follows `SPECIFY -> RED TEST -> IMPLEMENT -> GREEN -> CLIPPY -> VERIFY -> DOCUMENT -> CONTINUE`.
- Do not call a capability Supported until executable evidence satisfies its declared boundary.
- Distinguish FACT, HYPOTHESIS, ROOT CAUSE, and UNKNOWN.
- Never invent test results, CI results, SHAs, merge state, platform support, or runtime behavior.
- Never treat source presence as execution evidence.
- Do not destructively reset, clean, checkout, or rewrite user work without explicit evidence and authorization.
- If documentation and implementation drift, fix the root cause and reverify rather than silently choosing one.

## 6. Cross-environment rule

One is a language and toolchain, not a terminal, shell, distro, editor, or Android application. Linux distributions, terminals, IDEs, editors, containers, CI systems, and Android Linux environments are hosts/integration targets.

Compatibility must be established through stable CLI/toolchain, runtime, platform abstraction, and editor integration contracts rather than special-casing the language around individual products.

## 7. Security rule

One is general-purpose and may support legitimate security engineering and authorized penetration testing. Security capabilities must remain governed by explicit capability, scope, policy, and audit contracts. A natural-language request must never silently become privilege.

## 8. Evidence rule

Use evidence labels consistently:

- `[P0 — VERIFIED]` direct fresh executable/repository evidence.
- `[P1 — STRONG]` strong inference explicitly based on evidence.
- `[P2 — HISTORICAL]` prior evidence that does not prove the current HEAD.
- `[P3 — HYPOTHESIS]` unverified assumption.
- `[P4 — UNKNOWN]` insufficient evidence.
- `[P5 — INVALIDATED]` previously believed claim disproved by evidence.

## 9. Completion rule

A milestone is complete only when its source capability or documentation contract is implemented, tested where applicable, verified, and documented. A documentation-only milestone is complete only after repository content, links, consistency, and the resulting commit are verified.

## 10. Session continuity

A new session must start from this repository contract instead of assuming that prior chat history is available. The repository is the persistent memory of One's accepted direction, decisions, requirements, and evidence boundaries.

## 11. Mandatory cross-session continuation and Notion skill sync

- At the start of every session, load the user's enabled Notion skills with the Notion `notion-get-enabled-skills` capability; load the relevant skill before project work. The Project ONE Cross-Session Memory & Anti-Duplication Skill is a required workflow reference.
- Read this file, the current project checkpoint, and the active CI remediation record before running commands or editing files. Resolve the live branch HEAD and compare it with the recorded SHA; the recorded SHA is a checkpoint, not permission to assume the repository is unchanged.
- Maintain a durable no-repeat ledger: each task/scope is tagged NOT STARTED, IN PROGRESS, PASS (exact SHA + evidence), FAIL (first failing step + log), BLOCKED (reason), or UNKNOWN. Never rerun a PASS scope on the same SHA unless its inputs/environment changed or a later change invalidated it.
- Continue at the first unfinished dependency. Do not restart inventories, successful tests, completed audits, or prior toolchain diagnosis. If the source SHA changes, rerun only checks invalidated by that change, while preserving prior evidence as historical.
- For CI, inspect workflow -> job -> step -> log before patching. Fix the earliest deterministic root cause; do not skip tests or weaken acceptance gates. CircleCI is the primary CI for the active remediation; use the authenticated API fallback from Termux when the CircleCI CLI is incompatible.
- After every material code/config/documentation change, update the active checkpoint and Notion skill/reference with the exact commit, evidence, pending next action, and no-repeat guard. Do not claim a result until readback and executable evidence support it.
