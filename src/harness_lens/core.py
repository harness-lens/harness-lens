# SPDX-License-Identifier: MPL-2.0
# Copyright © 2026 Cristian Camargo Filho

"""Python SDK facade with an optional Rust acceleration layer."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

try:
    from harness_lens import _native
except ImportError:  # pragma: no cover - expected in an unbuilt source tree
    _native = None

_STANDARD_NAMES = frozenset({"AGENTS.md", "CLAUDE.md", "GEMINI.md"})
_IGNORED_DIRECTORIES = frozenset(
    {
        ".git",
        ".mypy_cache",
        ".pytest_cache",
        ".ruff_cache",
        ".venv",
        "__pycache__",
        "build",
        "dist",
        "node_modules",
        "target",
        "venv",
    }
)


def _is_harness_file(path: Path) -> bool:
    if path.name in _STANDARD_NAMES:
        return True

    parts = path.parts
    if len(parts) >= 2 and parts[-2:] == (".github", "copilot-instructions.md"):
        return True

    return any(
        parts[index : index + 2] == (".cursor", "rules")
        for index in range(len(parts) - 2)
    )


def _source_kind(path: Path) -> str:
    if path.name == "AGENTS.md":
        return "agents"
    if any(
        path.parts[index : index + 2] == (".cursor", "rules")
        for index in range(len(path.parts) - 2)
    ):
        return "rules"
    return "instructions"


def _discover_python(root: str | Path = ".") -> tuple[Path, ...]:
    """Source-tree fallback for discovery before the extension is built."""

    root_path = Path(root).expanduser()
    if not root_path.exists():
        raise FileNotFoundError(f"path does not exist: {root_path}")
    if not root_path.is_dir():
        raise NotADirectoryError(f"path is not a directory: {root_path}")

    matches: list[Path] = []
    for candidate in root_path.rglob("*"):
        relative = candidate.relative_to(root_path)
        if any(part in _IGNORED_DIRECTORIES for part in relative.parts[:-1]):
            continue
        if candidate.is_file() and _is_harness_file(relative):
            matches.append(relative)

    return tuple(sorted(matches, key=lambda path: path.as_posix()))


def native_available() -> bool:
    """Return whether the PyO3 extension is loaded."""

    return _native is not None


def discover(
    root: str | Path = ".", *, config: str | Path | None = None
) -> tuple[Path, ...]:
    """Return recognized harness files as paths relative to *root*."""

    if _native is None:
        if config is not None:
            raise RuntimeError("custom configuration requires the native Rust extension")
        return _discover_python(root)

    config_path = None if config is None else str(Path(config).expanduser())
    return tuple(Path(path) for path in _native.discover(str(root), config_path))


def scan(
    root: str | Path = ".", *, config: str | Path | None = None
) -> dict[str, Any]:
    """Return a provider-neutral, evidence-bearing analysis report."""

    root_path = Path(root).expanduser()
    if _native is not None:
        config_path = None if config is None else str(Path(config).expanduser())
        return json.loads(_native.scan_json(str(root_path), config_path))

    if config is not None:
        raise RuntimeError("custom configuration requires the native Rust extension")

    paths = _discover_python(root_path)
    resolved_root = root_path.resolve()
    sources = [
        {
            "path": path.as_posix(),
            "kind": _source_kind(path),
            "scope": path.parent.as_posix() if path.parent != Path(".") else "",
            "bytes": (resolved_root / path).stat().st_size,
        }
        for path in paths
    ]
    found = bool(sources)
    score = {
        "id": "harness.source_presence",
        "category": "quality",
        "method": "deterministic",
        "value": 1.0 if found else 0.0,
        "threshold": 1.0,
        "passed": found,
        "sample_size": len(sources),
        "reason": (
            f"Found {len(sources)} harness source(s)"
            if found
            else "No harness sources found"
        ),
        "evidence": {},
        "source": "harness-lens.inventory",
    }
    return {
        "schema_version": 1,
        "root": str(resolved_root),
        "completeness": {"complete": True, "reasons": []},
        "sources": sources,
        "findings": [],
        "metrics": [
            {
                "name": "harness.sources",
                "value": float(len(sources)),
                "unit": "count",
                "source": "harness-lens.inventory",
            }
        ],
        "scores": [score],
        "score_summary": {
            "quality_mean": score["value"],
            "safety_violations": 0,
            "by_category": {"quality": score["value"]},
        },
        "plugin_executions": [
            {
                "id": "harness-lens.inventory",
                "status": "completed",
                "duration_micros": 0,
            }
        ],
    }
