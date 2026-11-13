# Verification artifacts and CI workflows

## Live CircleCI migration checkpoint — 2026-11-14

- CircleCI is the canonical Project ONE product CI; GitHub remains source control and PR/review host. No parallel GitHub Actions product CI is intended after migration.
- PR #95 exact HEAD `e36963a329d7f04e40278043d20f6cc86f314005`: Linux job `204f0cc7-d0e7-472f-a4cc-3e92c53158e9` PASS; Windows job `5128b82c-4b17-4670-8c47-7dbd7f4a124c` FAIL; workflow `62dcb9b5-814d-4303-b3e2-a9a0b99f19cf`.
- Windows first failing step is not yet known; failure report identifier `6f4e8066-370f-49c3-9cc6-e9734a9a7412` must be read before any fix. Keep PR #95 draft/unmerged and main unchanged until both OS and required opt-in release/native evidence pass.


The canonical Project ONE CI configuration belongs in the private source repository at `.circleci/config.yml`. CircleCI job logs and stored artifacts are the primary CI execution evidence.

Independent verification mirrors and their runner-specific workflow artifacts remain outside the canonical private repository unless explicitly approved. They are supplementary verification infrastructure, not the source repository and not a substitute for recording exact-SHA CircleCI results.

The migration branch deletes the three Project ONE `.github/workflows/*.yml` CI files to avoid a parallel CI path. The `main` branch remains unchanged until the migration PR is reviewed and eligible to merge. Keep the PR draft/unmerged until equivalent CircleCI Linux/Windows, release, and native-verification evidence passes on exact SHAs.
