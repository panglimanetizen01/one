#!/usr/bin/env python3

import importlib.util
import pathlib
import tempfile
import unittest


ROOT = pathlib.Path(__file__).resolve().parents[1]
CHECKER = ROOT / "tools" / "check_user_docs.py"


def load_checker():
    spec = importlib.util.spec_from_file_location("check_user_docs", CHECKER)
    if spec is None or spec.loader is None:
        raise AssertionError(f"cannot load checker: {CHECKER}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class UserDocsCheckerTests(unittest.TestCase):
    def test_broken_internal_link_is_rejected(self):
        checker = load_checker()
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            (root / "guide.md").write_text(
                "# Guide\n\n[Lihat](missing.md)\n",
                encoding="utf-8",
            )

            errors = checker.check_user_docs(root)

            self.assertTrue(
                any("missing.md" in error for error in errors),
                errors,
            )

    def test_unknown_feature_status_is_rejected(self):
        checker = load_checker()
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            (root / "status-fitur.md").write_text(
                "# Status\n\n"
                "## Fitur A\n"
                "Status: Tersedia aja\n",
                encoding="utf-8",
            )

            errors = checker.check_user_docs(root)

            self.assertTrue(
                any("Tersedia aja" in error for error in errors),
                errors,
            )

    def test_valid_guide_tree_is_accepted(self):
        checker = load_checker()
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            (root / "README.md").write_text(
                "# Panduan Pengguna\n\n[Status](status-fitur.md)\n",
                encoding="utf-8",
            )
            (root / "status-fitur.md").write_text(
                "# Status Fitur\n\n"
                "Status: Didukung\n"
                "Status: Eksperimental\n"
                "Status: Dalam pengembangan\n"
                "Status: Dirancang\n"
                "Status: Belum tersedia\n",
                encoding="utf-8",
            )

            errors = checker.check_user_docs(root)

            self.assertEqual([], errors)


if __name__ == "__main__":
    unittest.main()
