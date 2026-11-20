# ONE package release provenance

The `.github/workflows/package-provenance-release.yml` workflow builds a committed ONE project, creates its final `.onepkg` artifact, signs that artifact with source-bound Ed25519 V2 provenance, verifies the signature and source snapshot, and uploads only the package, provenance sidecar, and SHA-256 digest file.

## Required GitHub environment

Configure a protected GitHub Actions environment named `one-release` before using the workflow:

- Add required reviewers / deployment protection rules appropriate for release signing.
- Add `ONE_RELEASE_SIGNING_SEED` as an environment secret. It must be the 32-byte Ed25519 seed encoded as exactly 64 hexadecimal characters. Do not store the seed in the repository, workflow inputs, logs, or release artifacts.
- Add `ONE_TRUSTED_PUBLISHERS` as an environment secret containing the publisher trust-store lines (`publisher-id=64-hex-character-public-key`). Provision this trust store from an independently authenticated publisher-key distribution process; do not derive it from the signing seed during the release job.
- Ensure the requested publisher ID is present in the trusted-publisher store and the corresponding public key is the intended release identity.

The workflow intentionally fails closed if either secret is absent, the publisher is not in the trust store, the project path escapes the checkout, or package signing / verification fails. The private seed and trust store are written to runner-temporary files with restrictive permissions and are not uploaded.

## Run a package release

1. Confirm the selected project is committed at the workflow's source revision and contains `one.toml` and `src/main.one`.
2. Review the exact commit and the project changes before approving the protected `one-release` environment.
3. Run **Actions → ONE Package Provenance Release → Run workflow**.
4. Set `project_path` to the repository-relative project directory and `publisher_id` to the trusted publisher ID.
5. Confirm the job's `one verify-package` step succeeds with `--source-root`. That step verifies the signed artifact, trusted publisher mapping, source-tree digest, and Git revision.
6. Retain the uploaded `.onepkg`, `.onepkg.provenance`, and SHA-256 digest file together. Distribute the trusted-publisher store separately from the artifact.

## Acceptance boundary

A successful workflow run is evidence for the exact source SHA, project, package artifact, publisher trust-store configuration, and source-bound V2 attestation used by that run. It does not authenticate a trust store whose public key was provisioned through an untrusted channel, and it does not by itself approve a repository-wide release. Final milestone acceptance still requires all mandatory platform, standard-library, security, compatibility, documentation, and Definition-of-Done checks on the final source SHA.

## Repository fixture

The repository includes a small committed package project at `examples/package-release` for validating the release path. After the protected environment and its two secrets have been provisioned, use `examples/package-release` as the `project_path` input and the matching trusted `publisher_id`. The workflow still signs and verifies the package built from the exact selected repository revision; the fixture is not a substitute for reviewing a real application package before release.
