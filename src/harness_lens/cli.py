"""Command-line interface for the first HarnessLens preview."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Sequence

from harness_lens import __version__
from harness_lens.core import discover


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="harness-lens",
        description="Discover coding-agent harness files in a repository.",
    )
    parser.add_argument("path", nargs="?", default=".", help="repository to inspect")
    parser.add_argument("--json", action="store_true", help="emit machine-readable output")
    parser.add_argument("--version", action="version", version=f"%(prog)s {__version__}")
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    root = Path(args.path).expanduser()

    try:
        files = discover(root)
    except (FileNotFoundError, NotADirectoryError) as exc:
        parser.error(str(exc))

    payload = {
        "root": str(root.resolve()),
        "count": len(files),
        "files": [path.as_posix() for path in files],
    }

    if args.json:
        print(json.dumps(payload, indent=2))
        return 0

    print(f"HarnessLens found {len(files)} harness file(s) under {payload['root']}")
    for path in files:
        print(f"- {path.as_posix()}")
    return 0


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(main())
