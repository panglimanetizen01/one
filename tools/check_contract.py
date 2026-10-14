from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

required = [
    "README.md",
    "SECURITY.md",
    "Cargo.toml",
    ".github/workflows/ci.yml",
    ".github/workflows/release-check.yml",
    "docs/spec/one-v0.1-lexical.md",
    "docs/spec/one-v0.1-grammar.md",
    "docs/spec/one-v0.1-ast.md",
    "docs/spec/one-v0.1-localization.md",
    "docs/spec/one-v0.1-diagnostics.md",
    "docs/spec/one-v0.1-stdlib.md",
    "docs/spec/one-v0.1-interop.md",
    "docs/spec/one-v0.1-cli.md",
    "docs/testing/conformance.md",
    "docs/release/one-v0.1.md",
    "docs/verification/one-v0.1-implementation-status.md",
    "docs/verification/one-v0.1-verification.md",
    "docs/verification/one-v0.1-evidence-matrix.md",
    "docs/verification/one-v0.1-offline-evidence-template.md",
    "docs/superpowers/plans/2026-10-15-one-v0-1-completion-addendum.md",
    "tools/verify_offline.sh",
    "tools/verify_offline.ps1",
    "crates/one-core/tests/conformance.rs",
]

missing = [path for path in required if not (ROOT / path).is_file()]
if missing:
    raise SystemExit("MISSING CONTRACT ARTIFACTS:\n" + "\n".join(missing))

release = (ROOT / "docs/release/one-v0.1.md").read_text(encoding="utf-8")
if "do not purchase an upgrade" not in release.lower():
    raise SystemExit("RELEASE CONTRACT: missing no-upgrade rule")

readme = (ROOT / "docs/README.md").read_text(encoding="utf-8")
for path in required[5:20]:
    if path not in readme:
        raise SystemExit(f"DOC INDEX: missing reference to {path}")

valid = sorted((ROOT / "tests/conformance/valid").glob("*.one"))
invalid = sorted((ROOT / "tests/conformance/invalid").glob("*.one"))
golden = sorted((ROOT / "tests/golden").glob("*.snap"))
if len(valid) < 5 or len(invalid) < 4 or len(golden) < 3:
    raise SystemExit("CONFORMANCE ARTIFACTS: expected valid/invalid/golden fixtures are missing")

print("CONTRACT CHECK: PASS")
print(f"valid fixtures: {len(valid)}")
print(f"invalid fixtures: {len(invalid)}")
print(f"golden fixtures: {len(golden)}")
