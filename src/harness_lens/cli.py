# SPDX-License-Identifier: MPL-2.0
# Copyright © 2026 Cristian Camargo Filho

"""Command-line interface for the first HarnessLens preview."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Sequence

from harness_lens import __version__
from harness_lens.core import scan


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="harness-lens",
        description="Discover coding-agent harness files in a repository.",
    )
    parser.add_argument("path", nargs="?", default=".", help="repository to inspect")
    parser.add_argument("--config", help="explicit harness-lens.toml path")
    parser.add_argument("--json", action="store_true", help="emit machine-readable output")
    parser.add_argument("--version", action="version", version=f"%(prog)s {__version__}")
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    root = Path(args.path).expanduser()

    try:
        report = scan(root, config=args.config)
    except (FileNotFoundError, NotADirectoryError, RuntimeError, ValueError) as exc:
        parser.error(str(exc))

    if args.json:
        print(json.dumps(report, indent=2))
        return 0

    sources = report["sources"]
    print(f"Harness Lens found {len(sources)} harness source(s) under {report['root']}")
    for source in sources:
        print(f"- {source['path']}")
    return 0


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(main())
