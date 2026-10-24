#!/usr/bin/env bash
set -euo pipefail

BRANCH="spec/human-instruction-grammar-vnext-20261022"

if [[ ! -f Cargo.toml || ! -f tools/check_contract.py ]]; then
  echo "VERIFICATION ERROR: run this script from the repository root." >&2
  exit 2
fi
if [[ -n "$(git status --porcelain --untracked-files=normal)" ]]; then
  echo "VERIFICATION ERROR: worktree is not clean; use a fresh GitHub-fetched worktree." >&2
  git status --short >&2
  exit 2
fi
REMOTE_URL="$(git remote get-url origin)"
case "$REMOTE_URL" in
  https://github.com/panglimanetizen01/one.git|https://github.com/panglimanetizen01/one|git@github.com:panglimanetizen01/one.git) ;;
  *)
    echo "VERIFICATION ERROR: origin is not the canonical ONE GitHub repository." >&2
    exit 2
    ;;
esac
LOCAL_SHA="$(git rev-parse HEAD)"
REMOTE_SHA="$(git ls-remote origin "refs/heads/$BRANCH" | awk 'NR == 1 {print $1}')"
if [[ -z "$REMOTE_SHA" || "$LOCAL_SHA" != "$REMOTE_SHA" ]]; then
  echo "VERIFICATION ERROR: local HEAD ($LOCAL_SHA) is not the current GitHub branch HEAD ($REMOTE_SHA)." >&2
  exit 2
fi

echo "ONE HUMAN-VNEXT EXACT-SHA VERIFICATION"
echo "repository: $(git remote get-url origin)"
echo "branch: $BRANCH"
echo "commit: $LOCAL_SHA"
echo "platform: $(uname -srm)"
rustc --version
cargo --version
python3 --version
if [[ -z "${ONE_ANDROID_CLANG:-}" && -x /tmp/one-android-clang-wrapper ]]; then
  export ONE_ANDROID_CLANG=/tmp/one-android-clang-wrapper
fi
if [[ -n "${ONE_ANDROID_CLANG:-}" ]]; then
  echo "ONE_ANDROID_CLANG: configured"
else
  echo "ONE_ANDROID_CLANG: not configured (Android native build test will be explicitly skipped)"
fi

bash -n tools/verify-human-vnext.sh
python3 tools/check_contract.py
git diff --check
cargo --locked build --workspace --all-targets
cargo --locked clippy --workspace --all-targets -- -D warnings
if [[ -n "${ONE_ANDROID_CLANG:-}" ]]; then
  ONE_ANDROID_CLANG="$ONE_ANDROID_CLANG" cargo --locked test --workspace --all-targets
else
  echo "NOTE: skipping android_target_explicit_mempertahankan_build_native; ONE_ANDROID_CLANG is not configured."
  cargo --locked test --workspace --all-targets -- --skip android_target_explicit_mempertahankan_build_native
fi

if [[ -n "$(git status --porcelain --untracked-files=normal)" ]]; then
  echo "VERIFICATION ERROR: checks changed the worktree." >&2
  git status --short >&2
  exit 2
fi
FINAL_SHA="$(git rev-parse HEAD)"
FINAL_REMOTE_SHA="$(git ls-remote origin "refs/heads/$BRANCH" | awk 'NR == 1 {print $1}')"
if [[ "$FINAL_SHA" != "$LOCAL_SHA" || "$FINAL_REMOTE_SHA" != "$LOCAL_SHA" ]]; then
  echo "VERIFICATION ERROR: source SHA or remote branch HEAD changed during verification." >&2
  exit 2
fi
echo "VERIFICATION RESULT: PASS"
echo "VERIFIED SOURCE SHA: $LOCAL_SHA"
echo "NOTE: this is local execution evidence only; it does not claim GitHub-hosted CI or Windows execution passed."
