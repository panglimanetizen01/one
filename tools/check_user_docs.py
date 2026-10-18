#!/usr/bin/env python3
"""Deterministic offline consistency checks for the official One user docs."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit


ALLOWED_STATUSES = {
    "Didukung",
    "Eksperimental",
    "Dalam pengembangan",
    "Dirancang",
    "Belum tersedia",
}

MARKDOWN_LINK_RE = re.compile(r"!?(?:\[[^\]]*\])\(([^)]+)\)")
STATUS_RE = re.compile(r"^\s*Status\s*:\s*(.*?)\s*$", re.MULTILINE)


def _internal_link_target(raw_target: str) -> str | None:
    target = raw_target.strip().strip("<>")
    if not target:
        return None

    parsed = urlsplit(target)
    if parsed.scheme or parsed.netloc:
        return None

    path = unquote(parsed.path)
    if not path:
        return None
    return path


def _check_links(root: Path) -> list[str]:
    errors: list[str] = []
    for document in sorted(root.rglob("*.md")):
        text = document.read_text(encoding="utf-8")
        for match in MARKDOWN_LINK_RE.finditer(text):
            target = _internal_link_target(match.group(1))
            if target is None:
                continue
            resolved = (document.parent / target).resolve()
            try:
                resolved.relative_to(root.resolve())
            except ValueError:
                errors.append(
                    f"{document.relative_to(root)}: internal link escapes docs root: {target}"
                )
                continue
            if not resolved.exists():
                errors.append(
                    f"{document.relative_to(root)}: broken internal link: {target}"
                )
    return errors


def _check_statuses(root: Path) -> list[str]:
    errors: list[str] = []
    for document in sorted(root.rglob("*.md")):
        text = document.read_text(encoding="utf-8")
        for status in STATUS_RE.findall(text):
            if status not in ALLOWED_STATUSES:
                errors.append(
                    f"{document.relative_to(root)}: unknown feature status: {status}"
                )
    return errors


def check_user_docs(root: Path) -> list[str]:
    """Return deterministic consistency errors for a user-docs directory."""
    root = Path(root)
    if not root.is_dir():
        return [f"user docs root does not exist: {root}"]

    return _check_links(root) + _check_statuses(root)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "root",
        nargs="?",
        type=Path,
        default=Path("docs/panduan-pengguna"),
    )
    args = parser.parse_args(argv)

    errors = check_user_docs(args.root)
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1

    print(f"OK: user documentation is consistent: {args.root}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
