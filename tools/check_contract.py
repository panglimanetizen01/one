from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

required = [
    "README.md", "SECURITY.md", "CONTRIBUTING.md", "CHANGELOG.md", "Cargo.toml", "rust-toolchain.toml",
    ".github/workflows/ci.yml", ".github/workflows/release-check.yml", ".github/workflows/windows-native-verification.yml",
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
    "docs/superpowers/specs/2026-10-22-one-human-instruction-grammar-vnext-draft.md",
    "docs/verification/one-completion-ledger.md", "docs/verification/human-vnext-task-list.md",
    "docs/verification/human-vnext-exact-sha-verification-contract.md", "tools/verify-human-vnext.sh",
    "docs/spec/human-instruction-vnext-starter-grammar.md",
    "examples/human-vnext/daftar-tugas.one",
    "tests/human-vnext/valid/minimal-data.one",
    "tests/human-vnext/valid/unicode-name.one",
    "tests/human-vnext/valid/task-list.one",
    "tests/human-vnext/valid/text-block-boundaries.one",
    "tests/human-vnext/valid/instruction-like-text.one",
    "tests/human-vnext/invalid/empty.one",
    "tests/human-vnext/invalid/orphan-instruction.one",
    "tests/human-vnext/invalid/unknown-instruction.one",
    "tests/human-vnext/invalid/unresolved-display.one",
    "tests/human-vnext/invalid/indented-application.one",
    "tests/human-vnext/invalid/missing-text-block.one",
    "tests/human-vnext/invalid/duplicate-data.one",
    "tests/human-vnext/invalid/duplicate-persistence.one",
    "tests/human-vnext/invalid/action-name-as-data.one",
    "tests/human-vnext/invalid/repeated-spaces.one",
    "tests/human-vnext/invalid/duplicate-data-action-name.one",
    "tests/human-vnext/invalid/second-application.one",
    "tests/human-vnext/invalid/empty-action.one",
    "tests/human-vnext/invalid/tab-indentation.one",
    "tests/human-vnext/invalid/noncanonical-indent.one",
    "tests/human-vnext/invalid/invalid-action-name.one",
    "crates/one-core/src/human_vnext.rs",
    "crates/one-cli/tests/cli.rs",
]

missing = [path for path in required if not (ROOT / path).is_file()]
if missing:
    raise SystemExit("MISSING CONTRACT ARTIFACTS:\n" + "\n".join(missing))

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

human_parser_tests = (ROOT / "crates/one-core/src/human_vnext.rs").read_text(encoding="utf-8")
for marker in (
    "semantic_errors_report_the_relevant_declaration_line",
    "rejects_noncanonical_identifier_edges_and_punctuation",
    "invalid_human_vnext_fixtures_are_rejected",
    "instruction_like_lines_remain_opaque_text_data",
):
    if marker not in human_parser_tests:
        raise SystemExit(f"HUMAN vNEXT PARSER TESTS: missing required test {marker!r}")

human_cli_tests = (ROOT / "crates/one-cli/tests/cli.rs").read_text(encoding="utf-8")
for marker in (
    "human_vnext_cli_runs_task_app_and_reopens_persistent_data",
    "human_vnext_cli_rejects_missing_source_argument",
    "human_vnext_cli_rejects_unknown_option_without_running_program",
    "human_vnext_cli_reports_missing_source_file",
    "human_vnext_cli_rejects_invalid_source_with_parser_diagnostic",
    "human_vnext_cli_rejects_task_app_without_persistence_contract",
):
    if marker not in human_cli_tests:
        raise SystemExit(f"HUMAN vNEXT CLI TESTS: missing required test {marker!r}")

human_vnext = (ROOT / "examples/human-vnext/daftar-tugas.one").read_text(encoding="utf-8")
for marker in (
    "Buat aplikasi daftar tugas",
    "Terima judul tugas",
    "Simpan tugas baru",
    "Tampilkan semua tugas",
    "Tandai tugas sebagai selesai",
    "Simpan semua tugas agar tidak hilang saat aplikasi ditutup",
):
    if marker not in human_vnext:
        raise SystemExit(f"HUMAN vNEXT EXAMPLE: missing required instruction {marker!r}")

ledger = (ROOT / "docs/verification/one-completion-ledger.md").read_text(encoding="utf-8")
if "30% GATE NOT PASSED" not in ledger or "TOTAL" not in ledger:
    raise SystemExit("COMPLETION LEDGER: missing explicit milestone status or weighted total")

verification_contract = (ROOT / "docs/verification/human-vnext-exact-sha-verification-contract.md").read_text(encoding="utf-8")
verification_script = (ROOT / "tools/verify-human-vnext.sh").read_text(encoding="utf-8")
for marker in (
    "git ls-remote origin",
    "cargo build --workspace --all-targets",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo test --workspace --all-targets",
    "does not manufacture CI results",
):
    if marker not in verification_contract + verification_script:
        raise SystemExit(f"EXACT-SHA VERIFICATION CONTRACT: missing required rule/command {marker!r}")

print("CONTRACT CHECK: PASS")
print(f"required artifacts: {len(required)}")
print(f"indexed concrete artifacts checked: {sum(1 for ref in index_references if '*' not in ref)}")
print(f"valid fixtures: {len(valid)}")
print(f"invalid fixtures: {len(invalid)}")
human_valid = sorted((ROOT / "tests/human-vnext/valid").glob("*.one"))
human_invalid = sorted((ROOT / "tests/human-vnext/invalid").glob("*.one"))
if len(human_valid) < 5 or len(human_invalid) < 16:
    raise SystemExit("HUMAN vNEXT FIXTURES: expected at least 5 valid and 16 invalid fixtures")

print(f"golden fixtures: {len(golden)}")
print(f"human-vNext valid fixtures: {len(human_valid)}")
print(f"human-vNext invalid fixtures: {len(human_invalid)}")
