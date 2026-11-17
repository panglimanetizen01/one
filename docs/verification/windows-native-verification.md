# Windows Native Verification Contract

Windows Server 2025 native build and execution evidence is maintained in the independent repository:

https://github.com/panglimanetizen01/one-windows-verification

The ONE repository is currently PUBLIC. Repository visibility remains an owner-controlled privacy decision and must not be changed as a routine CI workaround.

## Evidence identity

Every Windows verification record must pin:
- the exact ONE source commit SHA;
- the exact verifier repository commit SHA;
- workflow run and job identifiers;
- Windows Server version and architecture;
- native artifact format/architecture inspection;
- artifact checksum and manifest;
- actual process exit code and output.

A Linux-host MinGW cross-compile, PE structural inspection, or Windows-target build does not by itself prove Windows-host execution.

## Current state

- The verifier repository remains the designated Windows Server 2025 execution path.
- PR #102 exact source SHA `6c72d8e548bbbc28d85be39b8de427614989a038` passed the ONE GitHub Actions Windows Server 2025 test matrix (run #2229).
- PR #107 exact source SHA `21e393157e52728d9510699ac6c9120987e3852c` passed the ONE GitHub Actions Windows Server 2025 test matrix (run #2230).
- The independent verifier's historical run #45 pinned source `3f0ddb83b70d91b73aad3875d9eb1e4f5637064c`, not the current PR #102/#107 candidate. Do not transfer that artifact evidence to later source SHAs.
- Refresh the verifier's dedicated source pin and run it only when the exact source SHA and required Windows-native artifact gate are in scope. Do not repeat a successful exact-SHA run without a source/toolchain/verifier change.

No Windows verifier run is claimed for the latest PR #108 migration candidate until its current source SHA is pinned and the verifier run reaches terminal success.
