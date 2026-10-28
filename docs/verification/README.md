# Verification artifacts and workflows

Canonical project-owned verification workflows belong in this repository under `.github/workflows/` and are governed by the repository's normal review and branch-protection rules.

Independent verifier implementations, external runner evidence, and generated verification artifacts may live in the separate verifier repository when the verification contract calls for separation. External evidence must identify the exact canonical source commit SHA and must not be represented as canonical repository CI.

A workflow run, matching source pin, or artifact's presence is not a PASS by itself. Acceptance requires inspection of the actual job result, checkout SHA, steps/logs, and required artifacts for the documented scope. Historical-SHA evidence remains historical after the source SHA changes.