#!/usr/bin/env bash
set -euo pipefail

EXPECTED_SHA="${1:-}"
if [[ -z "$EXPECTED_SHA" ]]; then
  echo "usage: $0 <expected-commit-sha>" >&2
  exit 2
fi

actual_sha="$(git rev-parse HEAD)"
if [[ "$actual_sha" != "$EXPECTED_SHA" ]]; then
  echo "ERROR: checkout SHA mismatch" >&2
  echo "expected: $EXPECTED_SHA" >&2
  echo "actual:   $actual_sha" >&2
  exit 1
fi

if [[ -n "$(git status --porcelain)" ]]; then
  echo "ERROR: working tree is not clean; refusing to verify an uncommitted state" >&2
  git status --short >&2
  exit 1
fi

echo "=== EXACT SHA ==="
git rev-parse HEAD

echo "=== TOOLCHAIN ==="
rustc --version
cargo --version
python --version

echo "=== CONTRACT ==="
python tools/check_contract.py

echo "=== BUILD ==="
cargo build --workspace --all-targets

echo "=== CLIPPY ==="
cargo clippy --workspace --all-targets -- -D warnings

echo "=== TESTS ==="
cargo test --workspace --all-targets

echo "=== DETERMINISTIC ARTIFACTS ==="
cargo test -p one-cli --test cli build_and_package_artifacts_are_byte_deterministic

echo "=== DNS ==="
cargo test -p one-core --all-targets dns_adapter

echo "=== ORCHESTRATION ==="
cargo test -p one-core --all-targets orchestrator

echo "=== FILESYSTEM VERTICAL SLICE ==="
cargo test -p one-core --test filesystem_vertical_slice

echo "=== FILESYSTEM CAPABILITY ISOLATION ==="
cargo test -p one-core --test filesystem_capability_isolation

echo "=== PROCESS VERTICAL SLICE ==="
cargo test -p one-core --test process_vertical_slice

echo "=== DATABASE VERTICAL SLICE ==="
cargo test -p one-core --test database_vertical_slice

echo "=== DATABASE CLI ==="
cargo test -p one-cli --test database_orchestration

echo "=== HTTP GET VERTICAL SLICE ==="
cargo test -p one-core --test http_get_vertical_slice

echo "=== HTTP GET CLI ==="
cargo test -p one-cli --test http_get_orchestration

echo "=== FINAL STATUS ==="
git status --short
printf 'VERIFIED EXACT SHA: %s\n' "$EXPECTED_SHA"
