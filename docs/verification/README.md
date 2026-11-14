# Verification artifacts and CI workflows

## Live CircleCI migration checkpoint — 2026-11-14

- CircleCI is the canonical Project ONE product CI; GitHub remains source control and PR/review host. No parallel GitHub Actions product CI is intended after migration.
- PR #95 latest observed tested SHA `e38d561387896723510530630e9842d89b7fc94a`: Linux job `68b6e45a-7fd6-4d66-bac5-c12fd6467215` FAIL; Windows job `280d2b50-cbd4-42da-a82c-2342e8ec4daa` FAIL; workflow `2a133b0a-f3db-4980-a35a-652670d7897e`; failure report `b3c73393-a3a7-4689-af86-6480bf80cc29`.
- First failing steps/root causes are not yet known; authenticated CircleCI report/log access is required before any fix. Keep PR #95 draft/unmerged and main unchanged until both OS and required opt-in release/native evidence pass.
- First failing steps/root causes are not yet known; authenticated CircleCI report/log access is required before any fix. Keep PR #95 draft/unmerged and main unchanged until both OS and required opt-in release/native evidence pass.
- Windows first failing step is not yet known; failure report identifier `0ad0f9f7-db81-4b2b-8477-c3e89b546fc1` must be read before any fix. Keep PR #95 draft/unmerged and main unchanged until both OS and required opt-in release/native evidence pass.


The canonical Project ONE CI configuration belongs in the private source repository at `.circleci/config.yml`. CircleCI job logs and stored artifacts are the primary CI execution evidence.

Independent verification mirrors and their runner-specific workflow artifacts remain outside the canonical private repository unless explicitly approved. They are supplementary verification infrastructure, not the source repository and not a substitute for recording exact-SHA CircleCI results.

The migration branch deletes the three Project ONE `.github/workflows/*.yml` CI files to avoid a parallel CI path. The `main` branch remains unchanged until the migration PR is reviewed and eligible to merge. Keep the PR draft/unmerged until equivalent CircleCI Linux/Windows, release, and native-verification evidence passes on exact SHAs.
