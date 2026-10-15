#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

export CARGO_NET_OFFLINE=true

echo "=== ONE OFFLINE VERIFICATION ==="
echo "commit: $(git rev-parse HEAD 2>/dev/null || echo unknown)"
echo "rust: $(rustc --version)"
echo "cargo: $(cargo --version)"

echo "=== CONTRACT ARTIFACTS ==="
python3 tools/check_contract.py

echo "=== BUILD ==="
cargo build --workspace --all-targets

echo "=== CLIPPY ==="
cargo clippy --workspace --all-targets -- -D warnings

echo "=== TESTS ==="
cargo test --workspace --all-targets

echo "=== CLI SMOKE ==="
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
cargo run --quiet --package one-cli -- new "$TMP_DIR/sample"
printf 'nama adalah "Budi"\ntampilkan nama\n' > "$TMP_DIR/sample/src/main.one"
cargo run --quiet --package one-cli -- check "$TMP_DIR/sample/src/main.one"
cargo run --quiet --package one-cli -- run "$TMP_DIR/sample/src/main.one"
cargo run --quiet --package one-cli -- ast "$TMP_DIR/sample/src/main.one" >/dev/null
cargo run --quiet --package one-cli -- build "$TMP_DIR/sample/src/main.one"

echo "OFFLINE VERIFICATION: PASS"
