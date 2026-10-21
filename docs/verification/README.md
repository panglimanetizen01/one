# Verification Records and Artifacts

Keep durable verification records, evidence matrices, and status documents in this directory when they preserve project continuity and identify the exact source revision, environment, command, and result. A record must state its evidence boundary and must not promote historical evidence to a newer revision.

Generated binaries, bulky logs/archives, secrets, and external verification-workflow artifacts should remain outside the canonical repository unless explicitly approved. Canonical GitHub Actions workflow definitions belong under `.github/workflows/`.

A record stored here is documentation, not proof that a command ran. Claims of `PASS`, `VERIFIED`, or `SUPPORTED` still require the executable evidence and exact-revision boundary described in `docs/DEVELOPMENT_PROTOCOL.md`.
