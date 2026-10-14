# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Verification state

**Implementation:** present in the canonical repository.

**Automated CI:** blocked by the GitHub-hosted runner environment at the time of this record.

Observed GitHub Actions evidence:

- One CI runs 39, 40, and 41: both Ubuntu and Windows jobs failed before usable step/log evidence was available.
- A temporary runner-only workflow was executed independently; its Ubuntu job also failed before usable step/log evidence was available. The temporary workflow was removed after diagnosis.
- One CI run 49, the latest recorded CI run after the temporary workflow was removed, again failed on both platform jobs without usable step/log evidence.

The failure pattern also affects the runner-only smoke workflow, so the evidence does not currently identify a source-code failure. The repository must nevertheless **not** claim that the Rust implementation is CI-green until an actual successful build/test run is observed.

## Local verification policy

No user local/Termux synchronization is part of this implementation stage. Once the GitHub implementation is complete and the external CI blocker is resolved, a clean local verification checkout may be used as an independent validation environment.

## Required release evidence

Before declaring One v0.1 fully verified:

1. Linux CI build, clippy, unit tests, integration tests, and conformance tests are green.
2. Windows CI equivalents are green.
3. CLI `new`, `check`, `run`, `build`, and `ast` smoke tests are green.
4. Core DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT tests are green.
5. Capability/interoperability tests are green.
6. No known contradictory grammar/implementation contract remains.

Until these conditions are evidenced, the release remains **implementation-complete but verification-pending** rather than falsely marked complete.
