import json
import tempfile
import unittest
from pathlib import Path

from one_health import scan_repository, _safe_fix_missing_markdown_newline


class OneHealthScannerTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / "docs").mkdir()
        (self.root / "docs" / "target.md").write_text("# Target\n", encoding="utf-8")

    def tearDown(self):
        self.tmp.cleanup()

    def write(self, relative, content):
        path = self.root / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")
        return path

    def test_clean_markdown_link_has_no_broken_link_finding(self):
        self.write("README.md", "[target](docs/target.md)\n")
        report = scan_repository(self.root)
        self.assertFalse(any(f["rule_id"] == "broken-relative-link" for f in report["findings"]))

    def test_missing_relative_link_is_reported_with_location(self):
        self.write("README.md", "See [missing](docs/nope.md).\n")
        report = scan_repository(self.root)
        finding = next(f for f in report["findings"] if f["rule_id"] == "broken-relative-link")
        self.assertEqual("README.md", finding["path"])
        self.assertEqual(1, finding["line"])
        self.assertEqual("high", finding["confidence"])

    def test_unclosed_markdown_fence_is_reported(self):
        fence = chr(96) * 3
        self.write("guide.md", "Example:\n" + fence + "one\nnama adalah 1\n")
        report = scan_repository(self.root)
        self.assertTrue(any(f["rule_id"] == "unclosed-code-fence" for f in report["findings"]))

    def test_invalid_python_is_reported(self):
        self.write("tools/bad.py", "def missing(:\n    pass\n")
        report = scan_repository(self.root)
        self.assertTrue(any(f["rule_id"] == "invalid-python" and f["path"] == "tools/bad.py" for f in report["findings"]))

    def test_invalid_toml_is_reported(self):
        self.write("Cargo.toml", "[package\nname = 'one'\n")
        report = scan_repository(self.root)
        self.assertTrue(any(f["rule_id"] == "invalid-toml" for f in report["findings"]))

    def test_invalid_json_is_reported(self):
        self.write("config.json", '{"enabled": true,,}')
        report = scan_repository(self.root)
        self.assertTrue(any(f["rule_id"] == "invalid-json" for f in report["findings"]))

    def test_evidence_source_and_checkout_sha_mismatch_is_reported(self):
        self.write("evidence.json", json.dumps({"source_sha": "a" * 40, "checked_out_sha": "b" * 40}))
        report = scan_repository(self.root)
        finding = next(f for f in report["findings"] if f["rule_id"] == "source-checkout-sha-mismatch")
        self.assertEqual("high", finding["severity"])

    def test_invalid_expected_sha_is_reported_not_passed(self):
        report = scan_repository(self.root, expected_sha="not-a-commit")
        self.assertTrue(any(f["rule_id"] == "invalid-expected-sha" for f in report["findings"]))

    def test_finding_id_is_stable_between_identical_runs(self):
        self.write("README.md", "[missing](absent.md)\n")
        first = scan_repository(self.root)
        second = scan_repository(self.root)
        ids1 = [f["finding_id"] for f in first["findings"]]
        ids2 = [f["finding_id"] for f in second["findings"]]
        self.assertEqual(ids1, ids2)

    def test_safe_fix_proposal_does_not_modify_files(self):
        path = self.write("docs/without-newline.md", "# heading")
        before = path.read_bytes()
        report = scan_repository(self.root)
        proposals = _safe_fix_missing_markdown_newline(self.root, report, apply=False)
        self.assertTrue(any("docs/without-newline.md" in item for item in proposals))
        self.assertEqual(before, path.read_bytes())

    def test_json_report_schema_and_counts_are_present(self):
        self.write("README.md", "Healthy.\n")
        report = scan_repository(self.root)
        self.assertEqual("one-health-report/1", report["schema_version"])
        self.assertEqual(sum(report["severity_counts"].values()), report["finding_count"])
        self.assertIsInstance(report["findings"], list)


if __name__ == "__main__":
    unittest.main()
