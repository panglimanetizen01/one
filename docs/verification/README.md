# Verification artifacts and CI workflows

The canonical Project ONE CI configuration belongs in the private source repository at `.circleci/config.yml`. CircleCI job logs and stored artifacts are the primary CI execution evidence.

Independent verification mirrors and their runner-specific workflow artifacts remain outside the canonical private repository unless explicitly approved. They are supplementary verification infrastructure, not the source repository and not a substitute for recording exact-SHA CircleCI results.

The legacy `.github/workflows/*.yml` files are retained temporarily during the CircleCI migration. They may be retired only after the equivalent CircleCI workflows have passed on the same exact source SHA and the migration evidence has been reviewed.
