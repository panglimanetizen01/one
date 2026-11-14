# Verification artifacts and CI workflows

## Live CircleCI migration checkpoint — 2026-11-14

- CircleCI is the canonical Project ONE product CI; GitHub remains source control and PR/review host. No parallel GitHub Actions product CI is intended after migration.
- PR #95 exact tested SHA `f4d08cb7c13ca132e7553610bf62a504a32ec65b`: Linux job `a92345e7-96ce-4d86-8a15-3e0734a0bb55` FAIL; Windows job `2f95ae9e-005e-4c69-9ff1-1c69e8f7d806` FAIL; workflow `599c3f0d-a97c-4386-a4d3-0762fb39fd82`; shared failure report `079b7551-fef5-4a3e-81b2-65cd9c85cf7b`.
- First failing steps/root causes are UNKNOWN pending authenticated CircleCI logs. Keep PR #95 draft/unmerged and `main` unchanged; release/native evidence remains pending.

The canonical Project ONE CI configuration belongs in the private source repository at `.circleci/config.yml`. CircleCI job logs and stored artifacts are the primary CI execution evidence.

Independent verification mirrors and their runner-specific workflow artifacts remain outside the canonical private repository unless explicitly approved. They are supplementary verification infrastructure, not the source repository and not a substitute for recording exact-SHA CircleCI results.

The migration branch deletes the three Project ONE `.github/workflows/*.yml` CI files to avoid a parallel CI path. The `main` branch remains unchanged until the migration PR is reviewed and eligible to merge. Keep the PR draft/unmerged until equivalent CircleCI Linux/Windows, release, and native-verification evidence passes on exact SHAs.
