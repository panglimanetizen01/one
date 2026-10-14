# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Verification state

**Implementation:** present in the canonical repository.

**Automated CI:** blocked by the GitHub-hosted runner environment at the time of this record.

Observed GitHub Actions evidence:

- One CI run 39: both Ubuntu and Windows jobs failed before usable step/log evidence was available.
- One CI run 40: both Ubuntu and Windows jobs failed before usable step/log evidence was available.
- One CI run 41: both Ubuntu and Windows jobs failed before usable step/log evidence was available after removing the third-party Rust toolchain action.
- A temporary runner-only workflow was also executed; its Ubuntu job failed before usable step/log evidence was available. It was removed after diagnosis.

This means the repository must **not** claim that the current Rust implementation is CI-green yet. The code is implemented, but the external runner failure prevents an evidence-based green-build claim.

## Local verification policy

No user local/Termux synchronization is part of this implementation stage. Once the GitHub implementation is complete and the external CI blocker is resolved, a clean local verification checkout may be used as an independent validation environment.

## Required release evidence

Before declaring One v0.1 fully verified:

1. Linux CI build, clippy, formatting, unit tests, integration tests, and conformance tests are green.
2. Windows CI equivalents are green.
3. CLI `new`, `check`, `run`, `build`, and `ast` smoke tests are green.
4. Core DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT tests are green.
5. Capability/interoperability tests are green.
6. No known contradictory grammar/implementation contract remains.

Until these conditions are evidenced, the release remains **implementation-complete but verification-pending** rather than falsely marked complete.
