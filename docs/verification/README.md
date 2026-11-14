# Verification artifacts and CI workflows

## Live CircleCI migration checkpoint — 2026-11-14

- CircleCI is the canonical Project ONE product CI; GitHub remains source control and PR/review host. No parallel GitHub Actions product CI is intended after migration.
- PR #95 exact HEAD `6b130c7db9faca5ed6b6cd044c49e582fcafddf9`: Windows job `639baf29-477d-44e9-9287-7907aa241711` FAIL; Linux has not yet reported a status for this exact SHA; workflow `7acbf7b4-8dd6-4f90-bf92-5ad243da6845`.
- Windows first failing step is not yet known; failure report identifier `0ad0f9f7-db81-4b2b-8477-c3e89b546fc1` must be read before any fix. Keep PR #95 draft/unmerged and main unchanged until both OS and required opt-in release/native evidence pass.


The canonical Project ONE CI configuration belongs in the private source repository at `.circleci/config.yml`. CircleCI job logs and stored artifacts are the primary CI execution evidence.

Independent verification mirrors and their runner-specific workflow artifacts remain outside the canonical private repository unless explicitly approved. They are supplementary verification infrastructure, not the source repository and not a substitute for recording exact-SHA CircleCI results.

The migration branch deletes the three Project ONE `.github/workflows/*.yml` CI files to avoid a parallel CI path. The `main` branch remains unchanged until the migration PR is reviewed and eligible to merge. Keep the PR draft/unmerged until equivalent CircleCI Linux/Windows, release, and native-verification evidence passes on exact SHAs.
