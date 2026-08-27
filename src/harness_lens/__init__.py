"""Public package interface for HarnessLens."""

from importlib.metadata import PackageNotFoundError, version

from harness_lens.core import discover

try:
    __version__ = version("harness-lens")
except PackageNotFoundError:  # pragma: no cover - source tree without installation
    __version__ = "0.0.0"

__all__ = ["__version__", "discover"]
