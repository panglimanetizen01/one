# MASTER CONTINUATION PROMPT — PROJECT ONE

**CONTINUOUS EXECUTION · NOTION-FIRST · IMPLEMENTATION-FIRST · GITHUB/CIRCLECI-FIRST · CONTRACT-FIRST · DIAGRAM-AWARE · EVIDENCE-FIRST · EXACT-SHA · ZERO-REWORK · NO-PREMATURE-STOP**

## Mission

Continue **Project ONE — Beginner-Friendly Programming Language** until the product implementation required by the v0.1 release contract is complete and usable. Execute real code, integration, documentation, and repository changes. Do not let the project loop endlessly through audits and repeated tests while required product work remains unfinished.

## Authoritative workspaces

- Main repository: https://github.com/panglimanetizen01/one
- Independent Windows verifier: https://github.com/panglimanetizen01/one-windows-verification
- Notion Skill: **PROJECT ONE — Cross-Session Memory & Anti-Duplication Skill**, page `407a9d23-afaa-813c-b942-d4d6a4d1c5ce`
- Single work queue: [ONE_RELEASE_BLOCKER_REGISTER.md](ONE_RELEASE_BLOCKER_REGISTER.md). Do not create a competing blocker register, roadmap, evidence ledger, or checkpoint document.

Only modify the two repositories above and their authorized CircleCI configuration/workflows. Never modify `main` directly.

## Mandatory start-of-session sequence

1. Read the latest Notion Skill checkpoint first. Newer live evidence supersedes older notes; retain history.
2. Read repository instructions (`AGENTS.md`), README/docs maps, release contract, relevant specs/contracts/acceptance criteria, fixed ledger/DoD, architecture diagrams, and the single blocker register.
3. Refresh GitHub state: PRs, issues, branches, full commit SHAs, workflow/job results, and verifier pins. Refresh CircleCI pipeline → workflow → job → `.vcs.revision` mapping. Never assume historical state is current.
4. Before any Termux Git command, read `docs/verification/TERMUX_WORKTREE_MAP.md`. Check the intended worktree, branch, and clean/dirty state first.
5. Continue the highest-priority unfinished implementation item in the blocker register. Do not ask the user to repeat known project context.

## Execution policy

- Work in this order: **DISCOVER → IMPLEMENT → INTEGRATE → DOCUMENT → TARGETED TEST → FINAL VERIFICATION → RELEASE**.
- Product implementation is the priority. Add focused regression tests alongside security-sensitive changes, but defer broad matrix reruns and repeated audits until implementation scope is complete.
- Run a test again only when its source, configuration, environment, or required evidence has materially changed, or a new failure makes it necessary. Never repeat a proven same-SHA audit merely to produce another report.
- Fix actionable findings directly when safe. Do not stop after describing a defect or proposing a plan.
- Distinguish FACT, HYPOTHESIS, ROOT CAUSE, HISTORICAL, and UNKNOWN. Do not promote an assumption into evidence.
- Every PASS claim must state the full source SHA, verifier SHA where applicable, platform/runner, workflow and job, and terminal result. Evidence from a parent SHA does not prove a child SHA.
- Keep the complete existing documentation ecosystem synchronized when implementation changes: source contracts/specs, acceptance links, blocker register, evidence matrix, implementation status, architecture/Mermaid diagrams, developer/user docs, changelog/docs map, Notion Skill, and this master prompt. Update existing documents; do not create duplicates.
- Keep the one work queue current. Record implemented, built, tested, verified, accepted, and unsupported/fail-closed states separately. No claim of completion without exact-SHA evidence and synchronized documentation.

## Security and authorization boundaries

Without explicit user authorization, never:
- merge a PR, modify `main`, force-push, delete branches/worktrees, publish/tag/release, or change repository visibility;
- change protected verifier source pins, fixed acceptance ledger, DoD, release contract, or acceptance criteria;
- weaken symlink/reparse, hard-link, mount/volume, root-confinement, process-containment, cleanup, or resource-limit checks;
- remove tests or historical failure evidence, bypass a failing gate, or claim support for an untested platform;
- expose/request tokens, credentials, private keys, or secrets; or purchase plans, credits, or runners.

Unsupported platform primitives must fail closed. Do not replace kernel-enforced beneath/no-follow operations with unsafe path-based fallbacks merely to make a test pass.

## Live continuation checkpoint — 2026-11-21

- PR #113: branch `security/issue-104-root-confinement-20261118`, OPEN / DRAFT / UNMERGED. Base SHA: `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- Latest PR/documentation HEAD at this checkpoint: `9421a4f7a47dc004ef5d96667a23bf08a4439a33`.
- Latest code/test commit: `6403592796b9827e4bffe26a982dfc66afc75d9d`.
- macOS native workflow run #44906889694 SUCCESS on exact source SHA `6403592796b9827e4bffe26a982dfc66afc75d9d`. Both macOS 15 and 26 passed 15 macOS-filter tests (0 failed, 4 ignored) and 8 nested-write-filter tests (0 failed). Nested reads are required on both; nested writes are required on macOS 26. macOS 15 returns UnsupportedPlatform for nested atomic write and must remain fail-closed.
- macOS nested writes now use an exclusive same-directory temporary sibling in a parent validated with Darwin fstatfs filesystem-ID and mountpoint checks. The final rename resolves source and target from the retained root handle with RENAME_RESOLVE_BENEATH and RENAME_NOFOLLOW_ANY. Parent-move, mount-crossing-before-staging, cleanup-stage, and cleanup-failure regressions exist. Do not weaken these protections.
- Windows native filesystem code retains handle-relative traversal, no-delete-share parent handles, reparse/volume/link checks, atomic sibling replacement, and handle-bound delete. New Windows tests cover real directory-junction write escape rejection and positive absolute in-root writes; they need terminal exact-SHA Windows CI/E4 evidence.
- Linux reads use openat2 with BENEATH/NO_SYMLINKS/NO_XDEV. Nested writes depend on detached-mount primitives and fail closed when unavailable.
- Android currently supports direct-child filesystem operations; nested paths remain fail-closed under the observed Termux seccomp/openat2 constraint.
- POSIX delete remains unsupported/fail-closed; Windows is the only platform advertising handle-bound delete.
- E4 Unix source inventory: 3 descriptor-relative read-rejection tests, 5 write tests, and 1 walk/traversal test. Windows E4 named tests exist; two new Windows tests still need to be included in the independent verifier's exact filter.
- Independent verifier PR #51 is OPEN / DRAFT / UNMERGED and currently pins historical source `c4f3800bc12b70a4d59d72fbc5a80029cb053cea`. Protected verifier pin: `722d662e7a81711ba3607d26571f4048a292b3ed`. Do not move either pin until implementation scope is complete and the final E4 candidate is frozen.
- One CI run #44906889708 on exact SHA `6403592796b9827e4bffe26a982dfc66afc75d9d`: Ubuntu job SUCCESS; Windows 2025 job still in Build at last read. CircleCI context `ci/circleci: one-linux` on that SHA is PENDING (workflow `2ba5af51-82f5-4bee-9865-fa842200c9a2`, job `39275e15-8d19-4c6e-ac0d-9ad9b03332d9`); provider page/logs are inaccessible. Refresh both terminal states before relying on them.
- Issue #104 Filesystem Root Confinement remains OPEN / NOT ACCEPTED. Issue #105 Process Containment remains OPEN / NOT ACCEPTED. Project ONE is not complete or release-ready.

## Immediate continuation

1. Refresh the terminal result of One CI and the current PR #113 HEAD.
2. Refresh Windows 2025 terminal result on One CI #44906889708 and CircleCI status on the exact source SHA.
3. Ask the user to run the Android statx mount-ID regression and full `android_` test group on Termux after syncing the exact branch head; do not infer device runtime success from cross-platform CI.
4. Continue implementing the remaining Issue #104 platform/E4 gaps; include the Windows junction-write, absolute-write, volume-ID, and Android statx regressions in the independent verifier when the implementation candidate is ready.
5. Continue Android/Linux/macOS compatibility work without unsafe fallbacks, then Issue #105 and remaining v0.1 requirements. Only after implementation is complete, freeze one exact source SHA, update the independent verifier pin with authorization, run the required platform matrix once, reconcile every failure, and synchronize final evidence/docs.

## Definition of done

Project ONE is complete only when all required v0.1 product capabilities are implemented, integrated, and usable; every official release/security gate passes on one frozen release-candidate SHA; platform support and fail-closed limitations are explicit; exact-SHA verifier evidence is terminal; contracts, tests, diagrams, docs, blocker register, and Notion Skill agree; and the user explicitly authorizes release actions. A passing partial workflow, a complete document set, or an open PR is not release approval.

**Do the work. Do not merely report what should be done. Do not stop while safe, authorized implementation work remains.**
