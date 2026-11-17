from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

required = [
    "README.md", "SECURITY.md", "CONTRIBUTING.md", "CHANGELOG.md", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml",
    ".circleci/config.yml", "docs/CI.md", "docs/README.md",
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
    "docs/verification/one-v0.1-evidence-matrix.md", "docs/verification/windows-native-verification.md", "docs/verification/README.md", "docs/verification/repository-context-milestone.md", "docs/verification/forensic-audit-2026-10-21.md", "docs/verification/spec-implementation-gap-audit-2026-10-21.md", "docs/verification/one-v0.1-offline-evidence-template.md",
    "docs/verification/one-v0.1-linux-windows-2026-10-15.md", "docs/verification/native-backends-2026-10-19.md",
    "docs/release/one-v0.1.md", "tests/spec/grammar_examples.one", "crates/one-core/tests/conformance.rs",
    "crates/one-core/tests/native_backend_linux_windows.rs", "crates/one-cli/tests/native_cli.rs",
    "tools/check_contract.py", "tools/verify_offline.sh", "tools/verify_offline.ps1",
]

ci_config = (ROOT / ".circleci/config.yml").read_text(encoding="utf-8")
ci_contract_markers = [
    "one-linux:",
    "release-linux:",
    "setup-linux-cross-toolchains:",
    "run_release_checks:",
    "cargo build --locked --workspace --all-targets",
    "cargo clippy --locked --workspace --all-targets -- -D warnings",
    "cargo test --locked --workspace --all-targets",
    "tools/check_contract.py",
    "--features real-network",
    "tools/verify_offline.sh",
    "android:default",
    "resource_class: large",
    "27.2.12479018",
    "mingw-w64",
    "ONE_WINDOWS_CLANG=clang",
]
if not (ROOT / "Cargo.lock").is_file():
    raise SystemExit("CIRCLECI CONTRACT: canonical CircleCI candidate must track Cargo.lock")

cargo_commands = [
    line.strip()
    for line in ci_config.splitlines()
    if line.strip().startswith(("cargo build ", "cargo clippy ", "cargo test "))
]
if not cargo_commands:
    raise SystemExit("CIRCLECI CONTRACT: no explicit Cargo build, Clippy, or test commands found")
unlocked_commands = [command for command in cargo_commands if "--locked" not in command.split()]
if unlocked_commands:
    raise SystemExit("CIRCLECI CONTRACT: every Cargo command must use --locked: " + "; ".join(unlocked_commands))

forbidden_circleci_windows = (
    "run_windows_native_verification",
    "setup-windows-cross-toolchains",
    "windows-server-2025-gui:current",
    "windows.medium",
    "Install MSYS2 and MinGW-w64",
    "no_output_timeout: 30m",
    "verify_offline.ps1",
)
for marker in forbidden_circleci_windows:
    if marker in ci_config:
        raise SystemExit(f"CIRCLECI CONTRACT: Windows host job remains in canonical CircleCI config: {marker}")

for forbidden_job in ("  one-windows:", "  release-windows:", "  windows-native-verification:"):
    if forbidden_job in ci_config.splitlines():
        raise SystemExit(f"CIRCLECI CONTRACT: Windows job remains in canonical CircleCI config: {forbidden_job.strip()}")

legacy_workflow_dir = ROOT / ".github" / "workflows"
if legacy_workflow_dir.exists() and any(path.is_file() for path in legacy_workflow_dir.rglob("*")):
    raise SystemExit(
        "CI CONTRACT: canonical ONE must not retain GitHub Actions workflows; "
        "Windows host verification belongs to panglimanetizen01/one-windows-verification"
    )

workflows_start = ci_config.find("workflows:")
if workflows_start < 0:
    raise SystemExit("CIRCLECI CONTRACT: missing workflows section")
workflow_config = ci_config[workflows_start:]
if "      - one-linux" not in workflow_config or "      - release-linux" not in workflow_config:
    raise SystemExit("CIRCLECI CONTRACT: Linux product and release jobs must remain wired")
if any(marker in workflow_config for marker in ("one-windows", "release-windows", "windows-native-verification")):
    raise SystemExit("CIRCLECI CONTRACT: Windows jobs must not be wired into CircleCI workflows")

linux_job_start = ci_config.find("  one-linux:")
linux_job_end = ci_config.find("  release-linux:", linux_job_start)
if linux_job_start < 0 or linux_job_end < 0:
    raise SystemExit("CIRCLECI CONTRACT: cannot isolate the Linux product job")
linux_job = ci_config[linux_job_start:linux_job_end]
if "$env:" in linux_job:
    raise SystemExit("CIRCLECI CONTRACT: PowerShell environment syntax is forbidden in the Bash one-linux job")

linux_toolchains_start = ci_config.find("  setup-linux-cross-toolchains:")
linux_toolchains_end = ci_config.find("jobs:", linux_toolchains_start)
if linux_toolchains_start < 0 or linux_toolchains_end < 0:
    raise SystemExit("CIRCLECI CONTRACT: cannot isolate Linux-host cross-toolchain setup")
linux_toolchains = ci_config[linux_toolchains_start:linux_toolchains_end]
if "mingw-w64" not in linux_toolchains or "ONE_WINDOWS_CLANG=clang" not in linux_toolchains:
    raise SystemExit("CIRCLECI CONTRACT: Linux-host Windows-target cross-compilation coverage must remain explicit")

windows_verification = (ROOT / "docs/verification/windows-native-verification.md").read_text(encoding="utf-8")
windows_verification_markers = (
    "panglimanetizen01/one-windows-verification",
    "WINDOWS_SOURCE_COMMIT.txt",
    ".github/workflows/windows-native-backend-runtime.yml",
    "windows-2025",
    "source_sha",
    "verifier_sha",
    "PE32+",
    "07d8cbf7422b8340f8efda3c2f9d4425f85b7537",
    "43865524749",
    "14225302303",
    "GitHub metadata currently reports the repository as PUBLIC",
)

missing_windows_contract = [marker for marker in windows_verification_markers if marker not in windows_verification]
if missing_windows_contract:
    raise SystemExit("WINDOWS VERIFICATION CONTRACT: missing independent exact-SHA evidence markers: " + ", ".join(missing_windows_contract))

if "run #28" not in windows_verification:
    raise SystemExit("WINDOWS VERIFICATION CONTRACT: current candidate evidence must point to independent Windows run #28")
if "private repository" in windows_verification.lower() or "repository remains private" in windows_verification.lower():
    raise SystemExit("WINDOWS VERIFICATION CONTRACT: stale private-repository visibility claim remains")

status_features = (ROOT / "docs/panduan-pengguna/status-fitur.md").read_text(encoding="utf-8")
current_status_section = status_features.split("## Kondisi One saat ini", 1)[0]
status_markers = (
    "07d8cbf7422b8340f8efda3c2f9d4425f85b7537",
    "43865524749",
    "14225302303",
    "[#28]",
)
missing_status_markers = [marker for marker in status_markers if marker not in current_status_section]
if missing_status_markers:
    raise SystemExit("USER STATUS CONTRACT: current exact-SHA evidence checkpoint is stale: " + ", ".join(missing_status_markers))
if "bf37ccd3af924c95594b8ff3226b720f71dfdbb7" in current_status_section or "run #27" in current_status_section:
    raise SystemExit("USER STATUS CONTRACT: superseded Windows run #27 is still presented as current")

missing_ci_contract = [marker for marker in ci_contract_markers if marker not in ci_config]
if missing_ci_contract:
    raise SystemExit("CIRCLECI CONTRACT: missing required Linux CI gates/configuration: " + ", ".join(missing_ci_contract))

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

print("CONTRACT CHECK: PASS")
print("legacy GitHub Actions workflow files: 0")
print(f"required artifacts: {len(required)}")
print(f"indexed concrete artifacts checked: {sum(1 for ref in index_references if '*' not in ref)}")
print(f"valid fixtures: {len(valid)}")
print(f"invalid fixtures: {len(invalid)}")
print(f"golden fixtures: {len(golden)}")
