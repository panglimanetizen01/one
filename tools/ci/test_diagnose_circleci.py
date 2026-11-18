import unittest
from unittest.mock import patch

from diagnose_circleci import (
    DiagnosticError,
    classify_failure,
    extract_failed_steps,
    paged_v2_items,
    redact,
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

    def test_pagination_fails_instead_of_silently_truncating(self):
        response = {"items": [{"id": "page"}], "next_page_token": "next"}
        with patch("diagnose_circleci.get_json", return_value=response) as get_json:
            with self.assertRaisesRegex(DiagnosticError, "results are incomplete"):
                list(paged_v2_items("https://circleci.example/pipelines", {}))
        self.assertEqual(get_json.call_count, 10)

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
