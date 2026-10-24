#!/usr/bin/env bash
# Portable E1 test-lab entry point. Run from any clean checkout of the candidate branch:
#   bash tools/milestone1-e1-lab.sh
set -Eeuo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if ! command -v cargo >/dev/null 2>&1; then
  echo "BLOCKED: cargo is not installed or not on PATH." >&2
  exit 2
fi
if ! command -v python3 >/dev/null 2>&1; then
  echo "BLOCKED: python3 is not installed or not on PATH." >&2
  exit 2
fi

echo "=== E1 test lab: source revision ==="
git rev-parse HEAD
rustc --version
cargo --version

run() {
  printf '\n=== RUN:'
  printf ' %q' "$@"
  printf ' ===\n'
  "$@"
}

run cargo --locked test -p one-core --test stdlib_builtins
run cargo --locked test -p one-core --lib
run cargo --locked test --workspace --all-targets
run cargo --locked clippy --workspace --all-targets -- -D warnings
run python3 tools/check_contract.py

echo
echo "E1 TEST LAB: ALL COMMANDS PASSED at $(git rev-parse HEAD)"
