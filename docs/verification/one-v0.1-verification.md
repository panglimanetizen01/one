# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Verification state

**Implementation artifacts:** complete for the defined v0.1 scope.

**Contract artifacts:** closed for the defined v0.1 implementation scope; the repository contains normative Markdown contracts, indexed derivatives, fixtures, executable conformance tests, and static/offline verification tooling.

**Final canonical source commit verified:** `69c4918e6845fdbbd79495b3fa2c59798831bce0`.

**Linux execution evidence:** green on `ubuntu-latest` in public verification run `38747320152`, job `116464264565`, against the canonical commit above. The job passed canonical SHA verification, contract-artifact checks, build, clippy with `-D warnings`, workspace tests, and `tools/verify_offline.sh`.

**Windows execution evidence:** green on Windows Server 2025 (`windows-2025`) in public verification run `38744988440`, job `116460500173`, against the canonical commit above. The job passed canonical SHA verification, contract-artifact checks, build, clippy with `-D warnings`, workspace tests, and `tools/verify_offline.ps1`.

The complete evidence record is `docs/verification/one-v0.1-linux-windows-2026-10-15.md`.

**Hosted GitHub CI:** blocked in the private canonical repository because GitHub-hosted runner jobs fail before producing usable workflow-step evidence. This historical condition is not relabeled as green.

## Alternative verification path

No paid GitHub upgrade is required by the project contract. The canonical repository contains:

- `tools/check_contract.py` for static contract-artifact consistency checks;
- `tools/verify_offline.sh` for Linux environments;
- `tools/verify_offline.ps1` for Windows environments;
- `.github/workflows/release-check.yml` as an optional hosted release gate when runners are available.

Independent verification was executed in a temporary public repository which checked out the private canonical repository at the exact commit being verified. The public mirror is verification infrastructure only; it is not the source of truth.

## Local verification policy

The GitHub repository remains canonical. A clean local/Termux checkout may be used only as an independent verification environment after the canonical source/docs/tests are in place. Local changes are not treated as source-of-truth changes.

## Release evidence

For canonical commit `69c4918e6845fdbbd79495b3fa2c59798831bce0`, the required execution evidence is recorded for both Linux and Windows. The same verification jobs also exercised the contract-artifact checker, build, clippy, workspace tests, and platform-specific offline verification scripts.

The release remains honest about the distinction between independent execution evidence and private hosted-CI status: the former is green; the latter remains blocked by runner provisioning before usable steps.

## Result

**One v0.1 execution verification is COMPLETE for the recorded canonical commit.**
