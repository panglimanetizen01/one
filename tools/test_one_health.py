import json
import shutil
import tempfile
import unittest
from pathlib import Path

from one_health import scan_repository, apply_baseline, _safe_fix_missing_markdown_newline


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

    def test_missing_internal_markdown_heading_anchor_is_reported(self):
        self.write("guide.md", "# Current Heading\n")
        self.write("README.md", "[section](guide.md#removed-heading)\n")
        report = scan_repository(self.root)
        finding = next(f for f in report["findings"] if f["rule_id"] == "broken-heading-anchor")
        self.assertEqual("README.md", finding["path"])
        self.assertEqual(1, finding["line"])
        self.assertEqual("medium", finding["severity"])

    def test_valid_internal_heading_and_explicit_html_anchors_are_accepted(self):
        self.write("guide.md", "# Current Heading\n## Repeat\n## Repeat\n<div id=\"custom-target\"></div>\n")
        self.write("README.md", "[one](guide.md#current-heading) [two](guide.md#repeat-1) [three](guide.md#custom-target)\n")
        report = scan_repository(self.root)
        self.assertFalse(any(f["rule_id"] == "broken-heading-anchor" for f in report["findings"]))

    def test_unclosed_markdown_fence_is_reported(self):
        fence = chr(96) * 3
        self.write("guide.md", "Example:\n" + fence + "one\nnama adalah 1\n")
        report = scan_repository(self.root)
        self.assertTrue(any(f["rule_id"] == "unclosed-code-fence" for f in report["findings"]))

    def test_repeated_workflow_command_is_a_review_hint_not_a_failure(self):
        self.write(".github/workflows/a.yml", "name: A\njobs:\n  test:\n    steps:\n      - run: cargo test --locked\n")
        self.write(".github/workflows/b.yml", "name: B\njobs:\n  test:\n    steps:\n      - run: cargo test --locked\n")
        report = scan_repository(self.root)
        finding = next(f for f in report["findings"] if f["detector_id"] == "workflow-overlap")
        self.assertEqual("info", finding["severity"])
        self.assertEqual("low", finding["confidence"])
        self.assertIn("a.yml", finding["evidence"])
        self.assertIn("b.yml", finding["evidence"])

    def test_markdown_link_escaping_repository_root_is_high_severity(self):
        self.write("README.md", "[escape](../../outside.md)\n")
        report = scan_repository(self.root)
        finding = next(f for f in report["findings"] if f["rule_id"] == "path-escape")
        self.assertEqual("high", finding["severity"])

    def test_missing_workflow_script_is_reported(self):
        self.write(".github/workflows/ci.yml", "name: CI\njobs:\n  test:\n    steps:\n      - run: python tools/missing.py\n")
        report = scan_repository(self.root)
        self.assertTrue(any(f["rule_id"] == "missing-command-file" for f in report["findings"]))

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

    def test_live_ci_collection_outage_is_blocked_not_a_source_failure(self):
        self.write("ci-health-evidence.json", json.dumps({
            "schema_version": "one-ci-health-evidence/1", "collector_status": "blocked",
            "collection_errors": ["Actions API unavailable"], "jobs": []
        }))
        report = scan_repository(self.root)
        finding = next(f for f in report["findings"] if f["rule_id"] == "live-ci-metadata-blocked")
        self.assertEqual("blocked", finding["status"])
        self.assertEqual("info", finding["severity"])
        self.assertIn("bukan bukti kegagalan source", finding["summary"])

    def test_expired_ci_artifact_is_reported_and_missing_digest_is_not_integrity_pass(self):
        self.write("ci-health-evidence.json", json.dumps({
            "collector_status": "complete",
            "artifacts": [
                {"run_id": 12, "name": "old-report", "expired": True, "created_at": "2026-01-01"},
                {"run_id": 13, "name": "digest-missing", "expired": False}
            ]
        }))
        report = scan_repository(self.root)
        rules = {f["rule_id"]: f for f in report["findings"]}
        self.assertEqual("expired-ci-artifact", rules["expired-ci-artifact"]["rule_id"])
        self.assertEqual("medium", rules["expired-ci-artifact"]["severity"])
        self.assertEqual("artifact-digest-unavailable", rules["artifact-digest-unavailable"]["rule_id"])
        self.assertEqual("info", rules["artifact-digest-unavailable"]["severity"])

    def test_duplicate_artifact_names_in_same_run_are_review_hints(self):
        self.write("ci-health-evidence.json", json.dumps({
            "artifacts": [
                {"run_id": 12, "name": "report"},
                {"run_id": 12, "name": "report"}
            ]
        }))
        report = scan_repository(self.root)
        finding = next(f for f in report["findings"] if f["rule_id"] == "duplicate-artifact-name")
        self.assertEqual("medium", finding["severity"])

    def test_failed_ci_job_without_steps_is_blocked_not_misclassified(self):
        self.write("run.json", json.dumps({"jobs": [{"name": "One CI", "conclusion": "failure", "steps": [], "runner_id": 0}]}))
        report = scan_repository(self.root)
        finding = next(f for f in report["findings"] if f["rule_id"] == "failed-job-without-visible-steps")
        self.assertEqual("blocked", finding["status"])
        self.assertIn("penyebab belum diketahui", finding["summary"])
        self.assertIn("runner_id=0", finding["evidence"])

    def test_failed_ci_step_requires_log_review(self):
        self.write("run.json", json.dumps({"jobs": [{"name": "One CI", "conclusion": "failure",
            "steps": [{"name": "Tests", "conclusion": "failure"}]}]}))
        report = scan_repository(self.root)
        finding = next(f for f in report["findings"] if f["rule_id"] == "failed-step-needs-log-review")
        self.assertEqual("triaged", finding["status"])
        self.assertIn("Tests", finding["evidence"])

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
        self.assertTrue(any(line.startswith("@@") for line in proposals))
        self.assertEqual(before, path.read_bytes())

    def test_optional_one_source_check_reports_blocked_when_cli_is_missing(self):
        report = scan_repository(self.root, check_one_source=True)
        finding = next(f for f in report["findings"] if f["rule_id"] == "one-cli-unavailable")
        self.assertEqual("blocked", finding["status"])
        self.assertEqual("info", finding["severity"])

    def test_empty_normative_fixture_set_is_blocked_not_passed(self):
        binary = self.root / "target" / "debug" / "one.exe"
        binary.parent.mkdir(parents=True, exist_ok=True)
        binary.write_bytes(b"placeholder")
        report = scan_repository(self.root, check_one_source=True)
        finding = next(f for f in report["findings"] if f["rule_id"] == "no-normative-fixtures-found")
        self.assertEqual("blocked", finding["status"])

    def test_safe_fix_apply_requires_an_explicit_path(self):
        path = self.write("docs/without-newline.md", "# heading")
        report = scan_repository(self.root)
        with self.assertRaisesRegex(RuntimeError, "specify each exact"):
            _safe_fix_missing_markdown_newline(self.root, report, apply=True)
        self.assertEqual(b"# heading", path.read_bytes())

    def test_safe_fix_rejects_path_traversal_and_non_markdown(self):
        report = scan_repository(self.root)
        with self.assertRaisesRegex(RuntimeError, "cannot traverse"):
            _safe_fix_missing_markdown_newline(self.root, report, apply=True, repair_paths=["../outside.md"])
        with self.assertRaisesRegex(RuntimeError, "only Markdown"):
            _safe_fix_missing_markdown_newline(self.root, report, apply=True, repair_paths=["tools/one_health.py"])

    def test_safe_fix_apply_refuses_non_isolated_checkout(self):
        path = self.write("docs/without-newline.md", "# heading")
        before = path.read_bytes()
        report = scan_repository(self.root)
        with self.assertRaisesRegex(RuntimeError, "branch must start with"):
            _safe_fix_missing_markdown_newline(self.root, report, apply=True, repair_paths=["docs/without-newline.md"])
        self.assertEqual(before, path.read_bytes())

    def test_report_records_scanner_and_runtime_environment(self):
        report = scan_repository(self.root)
        self.assertEqual("0.1.0", report["scanner_version"])
        self.assertTrue(report["environment"]["python"])
        self.assertIn(report["environment"]["platform"], {"Linux", "Windows", "Darwin", "FreeBSD", "OpenBSD", "NetBSD"})
        self.assertIn(report["working_tree"]["state"], {"clean", "dirty_or_unavailable"})

    def test_report_exposes_detector_coverage_and_limitations(self):
        self.write("tools/example.py", "value = 1\n")
        report = scan_repository(self.root)
        self.assertIn("syntax", report["detectors_run"])
        self.assertIn("contract-drift", report["detectors_run"])
        self.assertTrue(any("YAML" in item for item in report["limitations"]))

    def test_normative_contract_drift_is_reported(self):
        self.write("docs/spec/one-v0.1-grammar.md", "Grammar without explicit ambiguity rule\n")
        report = scan_repository(self.root)
        self.assertTrue(any(f["rule_id"] == "no-generic-action-production" for f in report["findings"]))
        self.assertTrue(any(f["rule_id"] == "ambiguous-parse-rejection" for f in report["findings"]))

    def test_contract_detector_is_reported_even_when_docs_directory_is_missing(self):
        shutil.rmtree(self.root / "docs")
        report = scan_repository(self.root)
        self.assertIn("contract-drift", report["detectors_run"])
        self.assertTrue(any(f["rule_id"] == "normative-contract-missing" for f in report["findings"]))

    def test_contract_documents_missing_are_blocked_not_passed(self):
        report = scan_repository(self.root)
        missing = [f for f in report["findings"] if f["rule_id"] == "normative-contract-missing"]
        self.assertGreaterEqual(len(missing), 5)
        self.assertTrue(all(f["status"] == "blocked" for f in missing))

    def test_cli_contract_command_missing_from_user_guide_is_reported(self):
        self.write("docs/spec/one-v0.1-cli-contract.md", "# CLI contract\n")
        self.write("docs/panduan-pengguna/perintah-cli.md", "one help\n")
        report = scan_repository(self.root)
        self.assertTrue(any(f["rule_id"] == "missing-core-cli-command" for f in report["findings"]))

    def test_json_report_schema_and_counts_are_present(self):
        self.write("README.md", "Healthy.\n")
        report = scan_repository(self.root)
        self.assertEqual("one-health-report/1", report["schema_version"])
        self.assertEqual(sum(report["severity_counts"].values()), report["finding_count"])
        self.assertIsInstance(report["findings"], list)

    def test_baseline_marks_existing_findings_recurring_and_prioritizes_them(self):
        self.write("README.md", "[missing](absent.md)\n")
        current = scan_repository(self.root)
        finding = current["findings"][0]
        baseline = {"schema_version": current["schema_version"], "findings": [dict(finding, status="open")]}
        compared = apply_baseline(current, baseline)
        item = next(f for f in compared["findings"] if f["finding_id"] == finding["finding_id"])
        self.assertEqual("recurring", item["change_status"])
        self.assertGreater(item["priority_score"], 0)
        self.assertIn("acceptance_blocker_factor", item["priority_inputs"])

    def test_baseline_keeps_findings_not_checked_when_detector_input_is_absent(self):
        current = scan_repository(self.root)
        old = {"finding_id": "OH-ci-old", "detector_id": "ci-evidence", "status": "open", "summary": "Old CI issue"}
        compared = apply_baseline(current, {"schema_version": "one-health-report/1", "findings": [old]})
        self.assertEqual("not_checked", compared["not_checked_findings"][0]["change_status"])
        self.assertEqual("blocked", compared["not_checked_findings"][0]["status"])

    def test_baseline_marks_disappeared_findings_as_reviewable_resolutions(self):
        current = scan_repository(self.root)
        old = {"finding_id": "OH-previous", "detector_id": "markdown-links", "status": "open", "summary": "Old issue"}
        compared = apply_baseline(current, {"schema_version": "one-health-report/1", "findings": [old]})
        self.assertEqual("resolved", compared["resolved_findings"][0]["change_status"])
        self.assertIn("review detector coverage", compared["resolved_findings"][0]["resolution_basis"])

    def test_baseline_does_not_resolve_findings_from_detectors_not_run(self):
        current = scan_repository(self.root, check_one_source=False)
        old = {"finding_id": "OH-old-one-source", "detector_id": "one-source-contract",
               "status": "open", "summary": "One source check was blocked"}
        compared = apply_baseline(current, {"schema_version": "one-health-report/1", "findings": [old]})
        self.assertEqual("not_checked", compared["not_checked_findings"][0]["change_status"])
        self.assertEqual(1, compared["change_counts"]["not_checked"])

    def test_baseline_reopens_previously_resolved_finding(self):
        self.write("README.md", "[missing](absent.md)\n")
        current = scan_repository(self.root)
        finding = current["findings"][0]
        baseline = {"schema_version": current["schema_version"],
                    "findings": [dict(finding, status="resolved", change_status="resolved")]}
        compared = apply_baseline(current, baseline)
        item = next(f for f in compared["findings"] if f["finding_id"] == finding["finding_id"])
        self.assertEqual("reopened", item["change_status"])


if __name__ == "__main__":
    unittest.main()
