# Termux Git worktree map — Project ONE

## Why this file exists

Do not rediscover the repository layout or try to switch to a branch that is already checked out in another worktree. Git permits a branch to be checked out in only one worktree at a time.

## Known worktree map

These paths were confirmed from the user's Termux output on 2026-11-20:

| Termux path | Checked-out branch | Purpose / handling |
| --- | --- | --- |
| `$HOME/one` | `docs/system-architecture-diagram-20261115` | Separate architecture-documentation checkout. Do not use it to switch to the PR #113 branch while that branch is active elsewhere. |
| `$HOME/one-pr113` | `security/issue-104-root-confinement-20261118` | Canonical existing worktree for ONE PR #113 and B-001 continuation. Enter this directory directly. |
| `$HOME/one-ci-worktree` | `tooling/python-local-ci` | Separate worktree for local CI tooling. Preserve it; do not repurpose it. |

The branch-to-worktree association was confirmed by `git branch -vv`. In particular, the fatal message from `git switch security/issue-104-root-confinement-20261118` inside `$HOME/one` is expected because that branch is already checked out at `$HOME/one-pr113`; it is not a repository corruption or a reason to create another worktree.

## Mandatory continuation procedure

Start with the existing PR #113 worktree:

```bash
cd "$HOME/one-pr113"
pwd
git status -sb
git branch -vv
git rev-parse HEAD
git rev-parse origin/security/issue-104-root-confinement-20261118
```

Then verify the diagnostic utility and tests exist:

```bash
ls -l tools/ci/diagnose_circleci.py
ls -l tools/ci/test_diagnose_circleci.py
```

Interpretation and safety:

1. Confirm `pwd` is `$HOME/one-pr113` and the current branch is `security/issue-104-root-confinement-20261118`.
2. Review `git status -sb` before any synchronization. Preserve all user changes and untracked evidence.
3. Compare local HEAD with the remote-tracking SHA. If the worktree is clean and local HEAD is an ancestor of the fetched remote-tracking branch, a fast-forward-only update is the only permitted sync:
   ```bash
   git fetch origin
   git merge --ff-only origin/security/issue-104-root-confinement-20261118
   ```
   Run the ancestry/status checks again after fetch and before the merge. If the worktree is dirty, diverged, or the ancestry is unclear, stop before modifying it and inspect/preserve the changes.
4. Never use `git reset --hard`, force-push, branch deletion, or worktree removal as a shortcut. Do not create a duplicate worktree for this already-checked-out branch.
5. Do not switch branches in `$HOME/one` to access PR #113. Use `cd "$HOME/one-pr113"`.
6. Keep `$HOME/one`'s untracked evidence files and the separate `$HOME/one-ci-worktree` intact.

## Recorded checkpoint — 2026-11-20

Observed from the user's Termux session:

- PR #113 worktree: `$HOME/one-pr113`.
- Local HEAD: `70ae27cd9dab4429c528d6f01a42377f474c725c`.
- Remote-tracking branch SHA at readback: `2341fdabef712f400e5bd13288ef058996d4d36c`.
- Local PR #113 worktree was clean and reported `[behind 138]`.
- Both `tools/ci/diagnose_circleci.py` and `tools/ci/test_diagnose_circleci.py` existed in that worktree.
- The remote-tracking SHA is a historical checkpoint, not a permanent pin. Always fetch and read the current remote SHA before syncing.

This checkpoint does not authorize an automatic sync. Follow the clean-status and fast-forward-only checks above.

## B-001 continuation handoff

After safely synchronizing the existing worktree, continue the current B-001 evidence task; do not restart repository discovery or repeat already-proven work. Use the exact diagnostic target and instructions in [CIRCLECI_DIAGNOSTICS.md](CIRCLECI_DIAGNOSTICS.md). Keep B-001 `PARTIAL / NOT CLOSED` until the exact pipeline → workflow → job → `vcs.revision` evidence and provider diagnostics required by that document are available. Never request, paste, or print the CircleCI token.
