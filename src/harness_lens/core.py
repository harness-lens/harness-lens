"""Minimal harness-file discovery."""

from __future__ import annotations

from pathlib import Path

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
        "venv",
    }
)


def _is_harness_file(path: Path) -> bool:
    if path.name in _STANDARD_NAMES:
        return True

    parts = path.parts
    if len(parts) >= 2 and parts[-2:] == (".github", "copilot-instructions.md"):
        return True

    return len(parts) >= 3 and parts[-3:-1] == (".cursor", "rules")


def discover(root: str | Path = ".") -> tuple[Path, ...]:
    """Return recognized harness files as paths relative to *root*."""

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
