# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Verification state

**Implementation artifacts:** present in the canonical repository.

**Contract artifacts:** closed for the defined v0.1 implementation scope; the repository contains normative Markdown contracts, indexed derivatives, fixtures, executable conformance tests, and static/offline verification tooling.

**Hosted GitHub CI:** blocked by the GitHub-hosted runner environment at the time of this record.

Observed GitHub Actions evidence:

- One CI runs 39, 40, and 41: both Ubuntu and Windows jobs failed before usable step/log evidence was available.
- A temporary runner-only workflow was executed independently; its Ubuntu job also failed before usable step/log evidence was available. The temporary workflow was removed after diagnosis.
- One CI run `38623737388`: both platform jobs failed without usable step/log evidence.
- A retry of the failed jobs in run `38623737388` was attempted without changing source; the retried Linux and Windows jobs again failed without usable step data.
- After the contract-artifact updates, One CI run `38661412982` again produced completed Ubuntu and Windows jobs with `failure` conclusions and no usable step data. This reproduces the same runner-level evidence pattern.

This repeated failure pattern does not establish a source-code failure. The repository therefore does **not** claim hosted-CI green status.

## Alternative verification path

No paid GitHub upgrade is required by the project contract. The canonical repository contains:

- `tools/check_contract.py` for static contract-artifact consistency checks;
- `tools/verify_offline.sh` for Linux environments;
- `tools/verify_offline.ps1` for Windows environments;
- `.github/workflows/release-check.yml` as an optional hosted release gate when runners are available.

The offline verifiers first validate the contract artifact set, then run build, clippy, workspace tests, and CLI smoke checks with Cargo's offline mode. Independent execution evidence must record the exact commit SHA, OS, Rust/Cargo versions, command, and result. Independent evidence is not relabeled as hosted GitHub CI evidence.

## Local verification policy

The GitHub repository remains canonical. A clean local/Termux checkout may be used only as an independent verification environment after the canonical source/docs/tests are in place. Local changes are not treated as source-of-truth changes.

## Required release evidence

Before declaring One v0.1 fully verified:

1. Linux compilation and test evidence is green.
2. Windows compilation and test evidence is green.
3. CLI `new`, `check`, `run`, `build`, and `ast` smoke tests are green.
4. Core DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT tests are green.
5. Capability/interoperability tests are green.
6. No known contradictory grammar/implementation contract remains.
7. Conformance and golden tests are green.
8. The contract-artifact consistency checker passes on the same commit being verified.

Until these conditions are evidenced, the release remains **implementation-complete but verification-pending** rather than falsely marked complete.
