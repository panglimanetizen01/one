import unittest
from unittest.mock import patch

from diagnose_circleci import (
    DiagnosticError,
    classify_failure,
    classify_job_evidence,
    discover_failed_jobs,
    extract_failed_steps,
    paged_v2_items,
    redact,
    source_identity_line,
)


class CircleCIDiagnosticTests(unittest.TestCase):
    def test_extracts_only_failed_steps_and_parallel_execution(self):
        payload = {
            "data": {
                "attributes": {
                    "parallel_executions": [
                        {
                            "steps": [
                                {"num": 1, "name": "Checkout", "outcome": "succeeded"},
                                {"num": 103, "name": "Install toolchains", "outcome": "timed_out"},
                            ]
                        },
                        {
                            "steps": [
                                {"num": 7, "name": "Windows probe", "outcome": "failed"}
                            ]
                        },
                    ]
                }
            }
        }
        self.assertEqual(
            extract_failed_steps(payload),
            [
                {
                    "execution": 0,
                    "num": 103,
                    "name": "Install toolchains",
                    "outcome": "timed_out",
                },
                {
                    "execution": 1,
                    "num": 7,
                    "name": "Windows probe",
                    "outcome": "failed",
                },
            ],
        )

    def test_ignores_successful_steps(self):
        payload = {
            "data": {
                "attributes": {
                    "parallel_executions": [
                        {"steps": [{"num": 1, "outcome": "succeeded"}]}
                    ]
                }
            }
        }
        self.assertEqual(extract_failed_steps(payload), [])

    def test_classifies_no_output_timeout(self):
        self.assertEqual(
            classify_failure("Too long with no output: context deadline exceeded"),
            "TIMEOUT / NO-OUTPUT",
        )

    def test_classifier_does_not_match_its_own_explanatory_phrase(self):
        self.assertEqual(
            classify_failure("No step with failed/timed_out outcome was exposed by API v3."),
            "UNCLASSIFIED — inspect the evidence below",
        )

    def test_canceled_step_without_start_or_logs_is_pre_execution_unknown(self):
        self.assertEqual(
            classify_job_evidence(["canceled"], None, ""),
            "PRE-EXECUTION TERMINATION / ROOT CAUSE UNKNOWN",
        )

    def test_canceled_step_after_start_without_logs_is_not_timeout(self):
        self.assertEqual(
            classify_job_evidence(["canceled"], "2026-11-20T10:00:00Z", ""),
            "NO-OUTPUT / ROOT CAUSE UNKNOWN",
        )

    def test_missing_start_time_metadata_is_not_assumed_pre_execution(self):
        self.assertEqual(
            classify_job_evidence(["canceled"], None, "", start_time_known=False),
            "NO-OUTPUT / START TIME UNKNOWN",
        )

    def test_timed_out_provider_outcome_is_classified_as_timeout(self):
        self.assertEqual(
            classify_job_evidence(["timed_out"], None, ""),
            "TIMEOUT / NO-OUTPUT",
        )

    def test_classifies_rust_compiler_failure(self):
        self.assertEqual(
            classify_failure("error[E0308]: mismatched types"),
            "COMPILER / LINKER",
        )

    def test_classifies_test_failure(self):
        self.assertEqual(
            classify_failure("test result: FAILED. 1 failed"),
            "TEST FAILURE",
        )


    def test_pagination_reads_all_pages(self):
        responses = [
            {"items": [{"id": "first"}], "next_page_token": "page-2"},
            {"items": [{"id": "second"}], "next_page_token": None},
        ]
        with patch("diagnose_circleci.get_json", side_effect=responses) as get_json:
            self.assertEqual(
                list(paged_v2_items("https://circleci.example/pipelines", {})),
                [{"id": "first"}, {"id": "second"}],
            )
        self.assertEqual(get_json.call_count, 2)

    def test_pagination_reads_more_than_ten_pages(self):
        responses = [
            {
                "items": [{"id": "page-{}".format(index)}],
                "next_page_token": "page-{}".format(index + 1),
            }
            for index in range(1, 12)
        ]
        responses.append({"items": [{"id": "page-12"}], "next_page_token": None})
        with patch("diagnose_circleci.get_json", side_effect=responses) as get_json:
            items = list(paged_v2_items("https://circleci.example/pipelines", {}))
        self.assertEqual(len(items), 12)
        self.assertEqual(items[-1], {"id": "page-12"})
        self.assertEqual(get_json.call_count, 12)

    def test_pagination_rejects_repeated_page_token(self):
        responses = [
            {"items": [{"id": "first"}], "next_page_token": "repeat"},
            {"items": [{"id": "second"}], "next_page_token": "repeat"},
        ]
        with patch("diagnose_circleci.get_json", side_effect=responses) as get_json:
            with self.assertRaisesRegex(DiagnosticError, "repeated a next_page_token"):
                list(paged_v2_items("https://circleci.example/pipelines", {}))
        self.assertEqual(get_json.call_count, 2)

    def test_pagination_rejects_missing_items_array(self):
        with patch("diagnose_circleci.get_json", return_value={"next_page_token": None}):
            with self.assertRaisesRegex(DiagnosticError, "no valid 'items' array"):
                list(paged_v2_items("https://circleci.example/pipelines", {}))

    def test_pagination_rejects_non_object_items(self):
        with patch(
            "diagnose_circleci.get_json",
            return_value={"items": ["unexpected"], "next_page_token": None},
        ):
            with self.assertRaisesRegex(DiagnosticError, "non-object item"):
                list(paged_v2_items("https://circleci.example/pipelines", {}))

    def test_exact_sha_pipeline_without_failed_jobs_is_not_an_error(self):
        responses = [
            {
                "items": [
                    {
                        "id": "pipeline-uuid",
                        "vcs": {"revision": "a" * 40},
                        "created_at": "2026-11-19T00:00:00Z",
                    }
                ],
                "next_page_token": None,
            },
            {
                "items": [
                    {"id": "workflow-uuid", "name": "one-ci", "status": "running"}
                ],
                "next_page_token": None,
            },
            {
                "items": [
                    {"id": "job-uuid", "name": "one-linux", "status": "running"}
                ],
                "next_page_token": None,
            },
        ]
        with patch("diagnose_circleci.get_json", side_effect=responses):
            self.assertEqual(
                discover_failed_jobs("token", "gh/example/one", "feature", "a" * 40),
                [],
            )

    def test_exact_sha_discovery_checks_every_matching_pipeline(self):
        sha = "a" * 40
        responses = [
            {
                "items": [
                    {"id": "older-pipeline", "number": 10, "state": "failed",
                     "vcs": {"revision": sha}, "created_at": "2026-11-18T00:00:00Z"},
                    {"id": "newer-pipeline", "number": 11, "state": "created",
                     "vcs": {"revision": sha}, "created_at": "2026-11-19T00:00:00Z"},
                ],
                "next_page_token": None,
            },
            {"items": [{"id": "older-workflow", "name": "one-ci", "status": "failed"}],
             "next_page_token": None},
            {"items": [{"id": "older-job", "name": "one-linux", "status": "failed",
                        "job_number": 836, "started_at": None}],
             "next_page_token": None},
            {"items": [{"id": "newer-workflow", "name": "one-ci", "status": "running"}],
             "next_page_token": None},
            {"items": [{"id": "newer-job", "name": "one-linux", "status": "running"}],
             "next_page_token": None},
        ]
        with patch("diagnose_circleci.get_json", side_effect=responses) as get_json:
            failures = discover_failed_jobs("token", "gh/example/one", "feature", sha)
        self.assertEqual(len(failures), 1)
        self.assertEqual(failures[0]["pipeline_id"], "older-pipeline")
        self.assertEqual(failures[0]["pipeline_revision"], sha)
        self.assertEqual(failures[0]["workflow_id"], "older-workflow")
        self.assertEqual(failures[0]["job_started_at"], None)
        self.assertEqual(get_json.call_count, 5)

    def test_direct_job_lookup_marks_source_sha_unverified(self):
        line = source_identity_line({"id": "job-uuid"})
        self.assertIn("SOURCE SHA: UNVERIFIED", line)
        self.assertIn("bypasses exact-SHA", line)

    def test_exact_sha_discovery_prints_source_sha(self):
        line = source_identity_line({"commit_sha": "a" * 40})
        self.assertEqual(line, "SOURCE SHA: " + "a" * 40)

    def test_redacts_tokens_and_credentials(self):
        source = (
            "Authorization: Bearer abc.def_123 "
            "CIRCLECI_TOKEN=secret-value password=hunter2 "
            "ghp_abcdefghijklmnopqrstuvwxyz123456"
        )
        result = redact(source)
        self.assertNotIn("abc.def_123", result)
        self.assertNotIn("secret-value", result)
        self.assertNotIn("hunter2", result)
        self.assertNotIn("ghp_abcdefghijklmnopqrstuvwxyz123456", result)
        self.assertIn("[REDACTED]", result)


if __name__ == "__main__":
    unittest.main()
