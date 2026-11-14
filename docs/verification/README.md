# Verification artifacts and CI workflows

## Live CircleCI migration checkpoint — 2026-11-14

- CircleCI is the canonical Project ONE product CI; GitHub remains source control and PR/review host. No parallel GitHub Actions product CI is intended after migration.
- PR #95 latest observed tested SHA `9c1873f109512002af5c09789e91dfe5f99615c7`: Linux job `09d65e6c-1081-4331-9528-a92b3915dd29` FAIL; Windows job `81733a94-be9e-42be-ad12-b052104f3863` FAIL; workflow `ee0e1c19-4899-4e13-bede-38345e931704`; failure report `558caa03-5580-4f31-b621-55a4e118369c`.
- First failing steps/root causes are not yet known; authenticated CircleCI report/log access is required before any fix. Keep PR #95 draft/unmerged and main unchanged until both OS and required opt-in release/native evidence pass.
- Windows first failing step is not yet known; failure report identifier `0ad0f9f7-db81-4b2b-8477-c3e89b546fc1` must be read before any fix. Keep PR #95 draft/unmerged and main unchanged until both OS and required opt-in release/native evidence pass.


The canonical Project ONE CI configuration belongs in the private source repository at `.circleci/config.yml`. CircleCI job logs and stored artifacts are the primary CI execution evidence.

Independent verification mirrors and their runner-specific workflow artifacts remain outside the canonical private repository unless explicitly approved. They are supplementary verification infrastructure, not the source repository and not a substitute for recording exact-SHA CircleCI results.

The migration branch deletes the three Project ONE `.github/workflows/*.yml` CI files to avoid a parallel CI path. The `main` branch remains unchanged until the migration PR is reviewed and eligible to merge. Keep the PR draft/unmerged until equivalent CircleCI Linux/Windows, release, and native-verification evidence passes on exact SHAs.
