from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

required = [
    "README.md",
    "SECURITY.md",
    "CONTRIBUTING.md",
    "CHANGELOG.md",
    "Cargo.toml",
    "rust-toolchain.toml",
    ".github/workflows/ci.yml",
    ".github/workflows/release-check.yml",
    "docs/README.md",
    "docs/superpowers/specs/2026-10-15-one-language-foundation-design.md",
    "docs/superpowers/plans/2026-10-15-one-v0-1-implementation.md",
    "docs/superpowers/plans/2026-10-15-one-v0-1-completion-addendum.md",
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
    "tests/spec/grammar_examples.one",
    "crates/one-core/tests/conformance.rs",
    "tools/verify_offline.sh",
    "tools/verify_offline.ps1",
]

missing = [path for path in required if not (ROOT / path).is_file()]
if missing:
    raise SystemExit("MISSING CONTRACT ARTIFACTS:\n" + "\n".join(missing))

release = (ROOT / "docs/release/one-v0.1.md").read_text(encoding="utf-8")
if "do not purchase an upgrade" not in release.lower():
    raise SystemExit("RELEASE CONTRACT: missing no-upgrade rule")

index = (ROOT / "docs/README.md").read_text(encoding="utf-8")
for path in required[8:27]:
    if path not in index:
        raise SystemExit(f"DOC INDEX: missing reference to {path}")

valid = sorted((ROOT / "tests/conformance/valid").glob("*.one"))
invalid = sorted((ROOT / "tests/conformance/invalid").glob("*.one"))
golden = sorted((ROOT / "tests/golden").glob("*.snap"))
if len(valid) < 5 or len(invalid) < 4 or len(golden) < 3:
    raise SystemExit("CONFORMANCE ARTIFACTS: expected valid/invalid/golden fixtures are missing")

fixture = (ROOT / "tests/spec/grammar_examples.one").read_text(encoding="utf-8")
for marker in ("kalau ", "untuk setiap ", "selama ", "buat pekerjaan ", "ketika "):
    if marker not in fixture:
        raise SystemExit(f"GRAMMAR FIXTURE: missing normative construct marker {marker!r}")

print("CONTRACT CHECK: PASS")
print(f"required artifacts: {len(required)}")
print(f"valid fixtures: {len(valid)}")
print(f"invalid fixtures: {len(invalid)}")
print(f"golden fixtures: {len(golden)}")
