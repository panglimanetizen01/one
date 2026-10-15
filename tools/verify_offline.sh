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
cargo run --quiet --package one-cli -- buat "$TMP_DIR/sample"
printf 'nama adalah "Budi"\ntampilkan nama\n' > "$TMP_DIR/sample/src/main.one"
cargo run --quiet --package one-cli -- periksa "$TMP_DIR/sample/src/main.one"
cargo run --quiet --package one-cli -- jalankan "$TMP_DIR/sample/src/main.one"
cargo run --quiet --package one-cli -- lihat struktur "$TMP_DIR/sample/src/main.one" >/dev/null
cargo run --quiet --package one-cli -- bangun "$TMP_DIR/sample/src/main.one"
cp "$TMP_DIR/sample/src/target/main.oneir" "$TMP_DIR/first.oneir"
cargo run --quiet --package one-cli -- bangun "$TMP_DIR/sample/src/main.one"
cmp "$TMP_DIR/first.oneir" "$TMP_DIR/sample/src/target/main.oneir"
cargo run --quiet --package one-cli -- test "$TMP_DIR/sample"
cargo run --quiet --package one-cli -- package "$TMP_DIR/sample"
cp "$TMP_DIR/sample/target/proyek-one-0.1.0.onepkg" "$TMP_DIR/first.onepkg"
cargo run --quiet --package one-cli -- package "$TMP_DIR/sample"
cmp "$TMP_DIR/first.onepkg" "$TMP_DIR/sample/target/proyek-one-0.1.0.onepkg"

echo "OFFLINE VERIFICATION: PASS"
