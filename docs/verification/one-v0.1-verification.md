# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Verification state

**Implementation artifacts:** present in the canonical repository.

**Contract artifacts:** closed for the defined v0.1 implementation scope; the repository contains normative Markdown contracts, indexed derivatives, fixtures, executable conformance tests, and static/offline verification tooling.

**Independent Linux/Termux execution evidence:** recorded for commit `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` in `docs/verification/one-v0.1-linux-termux-2026-10-15.md`. That run passed contract consistency, build, clippy, workspace tests, CLI smoke, and deterministic `.oneir`/`.onepkg` checks.

**Hosted GitHub CI:** blocked by the GitHub-hosted runner environment at the time of this record.

Observed GitHub Actions evidence:

- One CI run `38715889895` targeted canonical commit `95e01d2b2fcee6af935e52b55867fdaf7088ed7e` using `windows-2022` and `ubuntu-latest`.
- Both jobs completed with `failure` conclusions and no usable workflow steps or logs.
- The Windows job was explicitly rerun; the rerun again completed with `failure` and no usable workflow steps or logs.
- The same pre-step failure pattern had already been reproduced on earlier One CI runs and after a runner-label change.

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

1. Linux compilation and test evidence is green for the final canonical commit being released.
2. Windows compilation and test evidence is green for the final canonical commit being released.
3. CLI `new`, `check`, `run`, `build`, and `ast` smoke tests are green.
4. Core DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT tests are green.
5. Capability/interoperability tests are green.
6. No known contradictory grammar/implementation contract remains.
7. Conformance and golden tests are green.
8. The contract-artifact consistency checker passes on the same commit being verified.

The recorded Linux evidence is evidence for its exact recorded commit. After canonical changes, the final release candidate must be synchronized and independently re-verified.

Until all release conditions are evidenced on the final canonical commit, the release remains **implementation-complete but verification-pending** rather than falsely marked complete.
