# One Official User Documentation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an official, beginner-oriented, versioned, editable user-documentation surface to the One repository without presenting unfinished capabilities as supported.

**Architecture:** Keep normative language contracts and engineering evidence under the existing `docs/spec`, `docs/verification`, and `docs/superpowers` surfaces. Add a separate `docs/panduan-pengguna/` surface for task-oriented user guidance, with an explicit documentation status contract and a single entry point from the repository README. User documentation is derived from verified implementation/contracts; it never becomes semantic authority over the language implementation.

**Tech Stack:** Markdown, existing One CLI, existing conformance fixtures, existing contract/evidence checker, GitHub `main` as canonical source.

**Spec:** Approved in-chat design from 2026-10-16; existing normative sources include `docs/spec/one-v0.1-cli-contract.md`, `docs/spec/one-v0.1-grammar.md`, `docs/verification/one-v0.1-implementation-status.md`, and `docs/README.md`.

## Global Constraints

- GitHub `main` is the canonical source of truth; local/Termux copies are verification environments only.
- Documentation MUST distinguish verified/implemented capabilities from experimental, designed, future, or unavailable capabilities.
- Documentation MUST NOT claim a feature is supported merely because it appears in an architecture, registry, or roadmap.
- User-facing documentation MUST prioritize plain Indonesian and progressive disclosure for beginners.
- Normative language semantics remain defined by the existing language contracts and implementation, not by the user guide.
- Existing deterministic grammar rules remain authoritative; documentation MUST NOT teach arbitrary natural-language intent as executable One syntax.
- Existing CLI names and aliases must remain aligned with the CLI contract.
- Examples presented as currently usable MUST be executable or otherwise independently verified against the recorded source revision.
- No GitHub paid upgrade is required for this documentation work.
- Documentation must remain editable by normal Git changes, pull requests, additions, removals, and corrections as One evolves.

## Review Focus

- A beginner follows the first-time path without needing to understand compiler internals — verify the guide reaches a first successful program.
- A documented command does exactly what the CLI contract says — verify each current command example against the CLI contract and available executable evidence.
- An unfinished feature is not accidentally presented as available — verify every capability-status claim against implementation/evidence inventory.
- A changed or removed language feature cannot silently leave stale instructions — verify cross-document links and feature references are discoverable.
- A user can recover from common mistakes without guessing — verify diagnostics, indentation, project layout, and offline limitations are explained accurately.

---

### Task 1: Establish the editable user-documentation contract

**Files:**
- Create: `docs/panduan-pengguna/README.md`
- Create: `docs/panduan-pengguna/status-fitur.md`
- Modify: `docs/README.md`
- Modify: `README.md`

**Interfaces:**
- Consumes: existing normative contracts and verification inventory.
- Produces: stable documentation entry point, status vocabulary, and navigation boundary for all later user guides.

- [ ] **Step 1: Write the documentation contract content**

Define the purpose, audience, source-of-truth rule, status vocabulary (`Didukung`, `Eksperimental`, `Dalam pengembangan`, `Dirancang`, `Belum tersedia`), correction/removal workflow, and rule that unsupported claims are forbidden.

- [ ] **Step 2: Add the user-documentation map**

Create a concise navigation page that points users to the beginner path first and separates user guidance from normative engineering documents.

- [ ] **Step 3: Link the new surface from repository documentation indexes**

Update the root README and `docs/README.md` so a new user can reach the guide without searching the repository.

- [ ] **Step 4: Review links and terminology**

Verify every referenced path exists on the branch and that status terms do not contradict the implementation-status definitions.

- [ ] **Step 5: Commit**

```bash
git add README.md docs/README.md docs/panduan-pengguna
git commit -m "docs: establish official user documentation contract"
```

### Task 2: Build the beginner first-run path

**Files:**
- Create: `docs/panduan-pengguna/mulai-dari-nol.md`
- Create: `docs/panduan-pengguna/instalasi.md`
- Create: `docs/panduan-pengguna/program-pertama.md`

**Interfaces:**
- Consumes: current CLI contract, executable command surface, current valid grammar examples.
- Produces: an end-to-end beginner path from installation/environment check to a first checked and executed One program.

- [ ] **Step 1: Document prerequisites and platform scope**

State only currently evidenced Windows/Linux requirements and clearly identify any environment step that depends on a future installer or distribution mechanism.

- [ ] **Step 2: Document the smallest valid program**

Use currently valid Indonesian syntax and explain each line in ordinary language.

- [ ] **Step 3: Document the first verification cycle**

Show the appropriate `one periksa` and `one jalankan` flow, expected success/failure distinction, and how to read diagnostics.

- [ ] **Step 4: Verify every command and example**

Cross-check commands against `one-v0.1-cli-contract.md` and executable fixtures. Do not invent installation commands that are not implemented.

- [ ] **Step 5: Commit**

```bash
git add docs/panduan-pengguna/mulai-dari-nol.md docs/panduan-pengguna/instalasi.md docs/panduan-pengguna/program-pertama.md
git commit -m "docs: add One beginner first-run guide"
```

### Task 3: Document currently supported language concepts

**Files:**
- Create: `docs/panduan-pengguna/konsep-dasar.md`
- Create: `docs/panduan-pengguna/data-dan-nama.md`
- Create: `docs/panduan-pengguna/kondisi-dan-perulangan.md`
- Create: `docs/panduan-pengguna/fungsi-dan-event.md`
- Create: `docs/panduan-pengguna/kesalahan-dan-debugging.md`

**Interfaces:**
- Consumes: `one-v0.1-grammar.md`, lexical/AST/diagnostic contracts, current conformance fixtures.
- Produces: beginner-readable explanations of the currently implemented language surface without weakening deterministic grammar rules.

- [ ] **Step 1: Inventory only verified constructs**

Map each chapter to an implemented grammar production and fixture. Exclude syntax that exists only in design documents or future plans.

- [ ] **Step 2: Write progressive explanations**

Start with plain-language concepts, then show exact syntax, then show valid and invalid examples.

- [ ] **Step 3: Explain ambiguity and structural boundaries simply**

Teach users that One looks natural but remains formal; structural words and unsupported conversational variants must not be guessed.

- [ ] **Step 4: Cross-check examples against conformance tests**

Every example labelled valid must correspond to current accepted syntax; invalid examples must be rejected for the documented reason.

- [ ] **Step 5: Commit**

```bash
git add docs/panduan-pengguna/konsep-dasar.md docs/panduan-pengguna/data-dan-nama.md docs/panduan-pengguna/kondisi-dan-perulangan.md docs/panduan-pengguna/fungsi-dan-event.md docs/panduan-pengguna/kesalahan-dan-debugging.md
git commit -m "docs: explain current One language basics"
```

### Task 4: Document the project lifecycle and CLI

**Files:**
- Create: `docs/panduan-pengguna/membuat-project.md`
- Create: `docs/panduan-pengguna/perintah-cli.md`
- Create: `docs/panduan-pengguna/test-build-package.md`
- Modify: `docs/panduan-pengguna/README.md`

**Interfaces:**
- Consumes: `one-v0.1-cli-contract.md`, project scaffolding behavior, deterministic artifact contracts.
- Produces: task-oriented instructions for creating, checking, running, testing, building, packaging, inspecting, and querying version/help.

- [ ] **Step 1: Document `one buat` and project layout**

Explain the generated `one.toml` and `src/main.one` structure and the refusal to overwrite an existing folder.

- [ ] **Step 2: Document each current CLI command**

For each command, state purpose, required argument, whether it executes code, and its current output/artifact boundary.

- [ ] **Step 3: Explain `.oneir` and `.onepkg` accurately**

Explicitly state that current `bangun`/`package` do not constitute a native executable compiler or platform installer.

- [ ] **Step 4: Verify command examples and artifact claims**

Use existing CLI tests and current contract evidence; remove any example that cannot be verified.

- [ ] **Step 5: Commit**

```bash
git add docs/panduan-pengguna/membuat-project.md docs/panduan-pengguna/perintah-cli.md docs/panduan-pengguna/test-build-package.md docs/panduan-pengguna/README.md
git commit -m "docs: document One project lifecycle and CLI"
```

### Task 5: Document networking and interoperability only to the verified boundary

**Files:**
- Create: `docs/panduan-pengguna/jaringan.md`
- Create: `docs/panduan-pengguna/interoperabilitas.md`

**Interfaces:**
- Consumes: network contracts, implementation-status evidence, interoperability contract, verified runtime inventory.
- Produces: user-facing guidance that clearly separates implemented/verified network and interoperability capabilities from future bindings.

- [ ] **Step 1: Build a capability/status matrix from evidence**

Use the current verification inventory as the sole source for claims of implemented or verified runtime behavior.

- [ ] **Step 2: Explain capability boundaries in beginner terms**

Explain that One can have explicit capabilities/effects and that documentation does not grant authority.

- [ ] **Step 3: Document current interoperability boundary**

Clearly separate current process interoperability from future native C/C++, Python, Node.js, and JVM bindings when those are not verified.

- [ ] **Step 4: Verify all protocol/binding claims**

Do not infer support from registry entries alone; require current evidence for each `verified` claim.

- [ ] **Step 5: Commit**

```bash
git add docs/panduan-pengguna/jaringan.md docs/panduan-pengguna/interoperabilitas.md
git commit -m "docs: document verified network and interoperability boundaries"
```

### Task 6: Add FAQ and documentation maintenance workflow

**Files:**
- Create: `docs/panduan-pengguna/faq.md`
- Create: `docs/panduan-pengguna/memperbarui-dokumentasi.md`
- Modify: `docs/panduan-pengguna/README.md`

**Interfaces:**
- Consumes: documentation contract and current verification workflow.
- Produces: user troubleshooting and contributor-facing instructions for safely adding, editing, or deleting documentation as One evolves.

- [ ] **Step 1: Write the FAQ from current behavior**

Cover common beginner questions: installation status, command vocabulary, syntax strictness, errors, project structure, offline use, and unfinished features.

- [ ] **Step 2: Define the documentation update loop**

Document: implementation change → verification → update/add/remove affected guide → cross-check links/examples → commit/PR.

- [ ] **Step 3: Define stale-documentation handling**

Require incorrect instructions to be corrected or removed rather than preserved merely for historical continuity.

- [ ] **Step 4: Commit**

```bash
git add docs/panduan-pengguna/faq.md docs/panduan-pengguna/memperbarui-dokumentasi.md docs/panduan-pengguna/README.md
git commit -m "docs: add FAQ and documentation maintenance workflow"
```

### Task 7: Add documentation consistency verification

**Files:**
- Create: `tools/check_user_docs.py`
- Create: `tools/check_user_docs_test.py`
- Modify: `docs/panduan-pengguna/status-fitur.md`

**Interfaces:**
- Consumes: user-guide Markdown files, status markers, repository paths, and existing contract-checking conventions.
- Produces: deterministic offline checks for broken internal links, unknown status labels, and references to paths that do not exist.

- [ ] **Step 1: Write failing checker tests**

Cover a broken internal link, an unknown feature-status marker, and a valid guide tree.

- [ ] **Step 2: Run the checker tests and verify failure**

Run the repository's Python test/check mechanism used for existing contract tooling. Expected: new tests fail before implementation.

- [ ] **Step 3: Implement the minimal deterministic checker**

Keep the checker offline and repository-local. It must not fetch web pages, invoke an LLM, or infer feature support.

- [ ] **Step 4: Run checker tests and full documentation checks**

Expected: all user-documentation consistency checks pass and no existing contract checks regress.

- [ ] **Step 5: Commit**

```bash
git add tools/check_user_docs.py tools/check_user_docs_test.py docs/panduan-pengguna/status-fitur.md
git commit -m "test: verify One user documentation consistency"
```

### Task 8: Final cross-contract review and verification

**Files:**
- Modify only where evidence finds stale or contradictory documentation.
- Review: `README.md`, `docs/README.md`, `docs/panduan-pengguna/**`, relevant `docs/spec/**`, `docs/verification/**`.

**Interfaces:**
- Consumes: all preceding documentation artifacts and existing repository verification tooling.
- Produces: a documentation tree that is navigable, internally consistent, and explicitly scoped to current One capability evidence.

- [ ] **Step 1: Run documentation consistency checks**

Expected: no broken internal links, unknown status markers, or missing referenced paths.

- [ ] **Step 2: Run existing contract checks**

Expected: no regression in existing contract-artifact consistency.

- [ ] **Step 3: Run the relevant One tests/fixtures for documented syntax**

Expected: every currently supported example used as a normative user instruction remains executable or accepted as documented.

- [ ] **Step 4: Perform a manual contradiction audit**

Compare user documentation against the current CLI, grammar, implementation-status, release, and verification documents. Correct or remove stale claims rather than rationalizing them.

- [ ] **Step 5: Record exact verification evidence**

Record the tested source SHA and commands in the appropriate verification artifact; do not claim cross-platform support without actual evidence.

- [ ] **Step 6: Commit final corrections**

```bash
git add README.md docs/README.md docs/panduan-pengguna tools
git commit -m "docs: verify official One user documentation"
```
