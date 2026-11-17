from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]

required = [
    "README.md", "SECURITY.md", "CONTRIBUTING.md", "CHANGELOG.md", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml",
    ".circleci/config.yml",
    "docs/README.md",
    "docs/superpowers/specs/2026-10-15-one-language-foundation-design.md",
    "docs/superpowers/specs/2026-10-15-one-end-to-end-network-platform-design.md",
    "docs/superpowers/plans/2026-10-15-one-v0-1-implementation.md",
    "docs/superpowers/plans/2026-10-15-one-v0-1-completion-addendum.md",
    "docs/superpowers/plans/2026-10-15-one-network-foundation.md",
    "docs/superpowers/specs/2026-10-19-native-backends-linux-windows.md",
    "docs/superpowers/plans/2026-10-19-native-backend-linux-aarch64.md",
    "docs/superpowers/plans/2026-10-19-native-backend-linux-x86_64.md",
    "docs/superpowers/plans/2026-10-19-native-backend-windows-x86_64.md",
    "docs/spec/README.md", "docs/spec/one-v0.1-language-invariants.md", "docs/spec/one-v0.1-lexical.md",
    "docs/spec/one-v0.1-grammar.md", "docs/spec/one-v0.1-ast.md", "docs/spec/one-v0.1-localization-contract.md",
    "docs/spec/one-v0.1-localization.md", "docs/spec/one-v0.1-diagnostics.md", "docs/spec/one-v0.1-effect-capability.md",
    "docs/spec/one-v0.1-runtime-model.md", "docs/spec/one-v0.1-reference-interpreter.md",
    "docs/spec/one-v0.1-conformance-suite.md", "docs/spec/one-v0.1-platform-abstraction.md",
    "docs/spec/one-v0.1-module-system.md", "docs/spec/one-v0.1-package-build.md", "docs/spec/one-v0.1-standard-library.md",
    "docs/spec/one-v0.1-stdlib.md", "docs/spec/one-v0.1-cli-contract.md", "docs/spec/one-v0.1-cli.md",
    "docs/spec/one-v0.1-interop.md", "docs/spec/one-network-foundation.md",
    "docs/implementation/one-v0.1-core-implementation-gate.md", "docs/testing/conformance.md",
    "docs/verification/one-v0.1-implementation-status.md", "docs/verification/one-v0.1-verification.md",
    "docs/verification/one-v0.1-evidence-matrix.md", "docs/verification/one-v0.1-offline-evidence-template.md",
    "docs/verification/one-v0.1-linux-windows-2026-10-15.md", "docs/verification/native-backends-2026-10-19.md",
    "docs/release/one-v0.1.md", "tests/spec/grammar_examples.one", "crates/one-core/tests/conformance.rs",
    "crates/one-core/tests/native_backend_linux_windows.rs", "crates/one-cli/tests/native_cli.rs",
    "tools/check_contract.py", "tools/verify_offline.sh", "tools/verify_offline.ps1",
]

missing = [path for path in required if not (ROOT / path).is_file()]
if missing:
    raise SystemExit("MISSING CONTRACT ARTIFACTS:\n" + "\n".join(missing))

# Every Cargo build/test/run command used by CI or the portable offline verifiers
# must honor the committed lockfile. This prevents CI from silently resolving
# a different dependency graph than the one reviewed in the repository.
for script in (".circleci/config.yml", "tools/verify_offline.sh", "tools/verify_offline.ps1"):
    for line_number, line in enumerate((ROOT / script).read_text(encoding="utf-8").splitlines(), start=1):
        command = line.strip()
        if command.startswith("command:"):
            command = command[len("command:"):].strip()
        if re.match(r"^cargo\s+(?:build|clippy|test|run)\b", command) and "--locked" not in command:
            raise SystemExit(f"LOCKFILE CONTRACT: {script}:{line_number} Cargo command must include --locked")

release = (ROOT / "docs/release/one-v0.1.md").read_text(encoding="utf-8")
if "do not purchase an upgrade" not in release.lower():
    raise SystemExit("RELEASE CONTRACT: missing no-upgrade rule")

index = (ROOT / "docs/README.md").read_text(encoding="utf-8")
index_references = set()
for line in index.splitlines():
    stripped = line.strip()
    if not stripped.startswith("-") or "`" not in stripped:
        continue
    start = stripped.find("`") + 1
    end = stripped.find("`", start)
    if end > start:
        index_references.add(stripped[start:end])

for reference in sorted(index_references):
    if "*" in reference:
        continue
    path = ROOT / reference.removeprefix("../") if reference.startswith("../") else ROOT / "docs" / reference
    if not path.is_file():
        raise SystemExit(f"DOC INDEX: referenced artifact is missing: {reference}")

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
print(f"indexed concrete artifacts checked: {sum(1 for ref in index_references if '*' not in ref)}")
print(f"valid fixtures: {len(valid)}")
print(f"invalid fixtures: {len(invalid)}")
print(f"golden fixtures: {len(golden)}")
