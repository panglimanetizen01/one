# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Current exact-SHA verification

Canonical source revision: `43cf7337ac963a0e18512aacf56dbfd7ab6bcd3a`.

Independent verification mirror: `panglimanetizen01/one-windows-verification`, run `38896533761`, branch `verification`.

The mirror read `CANONICAL_COMMIT.txt` containing the exact SHA above, checked out the canonical One source at that SHA, and asserted `git rev-parse HEAD` against it on both Ubuntu and Windows Server 2025.

## P0 evidence

### Ubuntu — job `116958230147`

- Exact canonical SHA assertion: PASS.
- Contract verification: `python3 tools/check_contract.py` → PASS.
- Build: `cargo build --workspace --all-targets` → PASS.
- Clippy: `cargo clippy --workspace --all-targets -- -D warnings` → PASS.
- Workspace tests: `cargo test --workspace --all-targets` → PASS.
- Real-network tests: `cargo test -p one-core --features real-network --all-targets` → PASS.
- Offline verification: `bash tools/verify_offline.sh` → PASS.
- Deterministic `.oneir`/`.onepkg`: deterministic artifact checks passed.
- DNS success and DNS resolution-failure classification: PASS.
- Orchestration runtime: allow/deny/incompatible-adapter tests and source-level DNS path: PASS.

### Windows — job `116958229777`

- Exact canonical SHA assertion: PASS.
- Contract verification: `python tools/check_contract.py` → PASS.
- Build: `cargo build --workspace --all-targets` → PASS.
- Clippy: `cargo clippy --workspace --all-targets -- -D warnings` → PASS.
- Workspace tests: `cargo test --workspace --all-targets` → PASS.
- Real-network tests: `cargo test -p one-core --features real-network --all-targets` → PASS.
- Offline verification: `.\tools\verify_offline.ps1` → PASS.
- Deterministic `.oneir`/`.onepkg`: deterministic artifact checks passed.
- DNS success and DNS resolution-failure classification: PASS.
- Orchestration runtime: allow/deny/incompatible-adapter tests and source-level DNS path: PASS.

## Verification boundary

`VERIFIED` is reserved for claims backed by executable evidence for the exact recorded source SHA. The evidence above verifies `43cf7337ac963a0e18512aacf56dbfd7ab6bcd3a`; it does not automatically verify later commits.

The independent verification mirror is verification infrastructure only. GitHub `main` remains the canonical source of truth. No paid GitHub upgrade was used or required.
