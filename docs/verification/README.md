# Verification artifacts and CI workflows

## Live CircleCI migration checkpoint — 2026-11-14

- CircleCI is the canonical Project ONE product CI; GitHub remains source control and PR/review host. No parallel GitHub Actions product CI is intended after migration.
- PR #95 exact tested SHA `2e4832c3d7a761a9bc00d6888bb6a7a98cb5de9b`: Linux job `3dbeadff-6151-4346-8b9b-d088f8fca33a` FAIL; Windows job `0c0cc6dc-a795-4917-a588-ada5f48d80ee` FAIL; workflow `8de847a4-d996-4f6a-9b92-1a3e0120dc53`; shared failure report `5c65f369-082d-4288-8e16-8d5e6aee95bf`.
- First failing steps/root causes are UNKNOWN pending authenticated CircleCI logs. Keep PR #95 draft/unmerged and `main` unchanged; release/native evidence remains pending.

The canonical Project ONE CI configuration belongs in the private source repository at `.circleci/config.yml`. CircleCI job logs and stored artifacts are the primary CI execution evidence.

Independent verification mirrors and their runner-specific workflow artifacts remain outside the canonical private repository unless explicitly approved. They are supplementary verification infrastructure, not the source repository and not a substitute for recording exact-SHA CircleCI results.

The migration branch deletes the three Project ONE `.github/workflows/*.yml` CI files to avoid a parallel CI path. The `main` branch remains unchanged until the migration PR is reviewed and eligible to merge. Keep the PR draft/unmerged until equivalent CircleCI Linux/Windows, release, and native-verification evidence passes on exact SHAs.
