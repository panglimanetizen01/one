# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Current exact-SHA verification

Canonical source revision: `e43b31fe9d7a5f62de4d3863b358aa28241fbf9c`.

Independent verification mirror: `panglimanetizen01/one-windows-verification`, run `38896135846`, branch `verification`.

The mirror checked out the verification branch at `1ae3aae1276248349b6906913321db3da0b14599`, read `CANONICAL_COMMIT.txt`, checked out the canonical One source at the exact SHA above, and asserted `git rev-parse HEAD` against that SHA on both Ubuntu and Windows Server 2025.

## P0 evidence

### Ubuntu — job `116956935554`

- OS: Ubuntu 24.04.5 LTS / `ubuntu-24.04`.
- Exact canonical SHA assertion: PASS.
- Toolchain: `rustc 1.99.0`, `cargo 1.99.0`, Python 3.12.3.
- Contract verification: `python3 tools/check_contract.py` → `CONTRACT CHECK: PASS`.
- Build: `cargo build --workspace --all-targets` → PASS.
- Clippy: `cargo clippy --workspace --all-targets -- -D warnings` → PASS.
- Workspace tests: `cargo test --workspace --all-targets` → PASS.
- Real-network tests: `cargo test -p one-core --features real-network --all-targets` → PASS.
- Offline verification: `bash tools/verify_offline.sh` → `OFFLINE VERIFICATION: PASS`.
- Deterministic `.oneir` and `.onepkg`: the offline verifier passed two-build/two-package byte-determinism checks; the workspace CLI test `build_and_package_artifacts_are_byte_deterministic` also passed.
- DNS success: `network_dns_adapter::tests::real_dns_adapter_resolves_loopback_without_external_command_text` → PASS.
- DNS failure classification: `network_dns_adapter::tests::dns_adapter_reports_resolution_failure_as_resolution_failure` → PASS.
- Orchestration runtime: `orchestrator::tests::approved_request_selects_compatible_adapter_and_invokes_once`, `denied_request_never_reaches_adapter_invocation`, and source-level DNS integration tests → PASS.

### Windows — job `116956935967`

- OS: Microsoft Windows Server 2025, `windows-2025`.
- Exact canonical SHA assertion: PASS.
- Toolchain: `rustc 1.99.0`, `cargo 1.99.0`, Python 3.12.10.
- Contract verification: `python tools/check_contract.py` → `CONTRACT CHECK: PASS`.
- Build: `cargo build --workspace --all-targets` → PASS.
- Clippy: `cargo clippy --workspace --all-targets -- -D warnings` → PASS.
- Workspace tests: `cargo test --workspace --all-targets` → PASS.
- Real-network tests: `cargo test -p one-core --features real-network --all-targets` → PASS.
- Offline verification: `.\tools\verify_offline.ps1` → `OFFLINE VERIFICATION: PASS`.
- Deterministic `.oneir` and `.onepkg`: the offline verifier passed the deterministic artifact checks; the workspace CLI test `build_and_package_artifacts_are_byte_deterministic` also passed.
- DNS success and failure classification: both DNS adapter tests passed.
- Orchestration runtime: approved, denied, and incompatible-adapter orchestration runtime tests passed; source-level DNS integration tests passed.

## Verification boundary

`VERIFIED` is reserved for claims backed by executable evidence for the exact recorded source SHA. The evidence above verifies `e43b31fe9d7a5f62de4d3863b358aa28241fbf9c`; it does not automatically verify later commits.

The independent public mirror is verification infrastructure only. GitHub `main` remains the canonical source of truth. No paid GitHub upgrade was used or required.
