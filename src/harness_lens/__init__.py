# SPDX-License-Identifier: MPL-2.0
# Copyright © 2026 Cristian Camargo Filho

"""Public package interface for HarnessLens."""

from importlib.metadata import PackageNotFoundError, version

from harness_lens.core import discover, native_available, scan

try:
    __version__ = version("harness-lens")
except PackageNotFoundError:  # pragma: no cover - source tree without installation
    __version__ = "0.0.0"

__all__ = ["__version__", "discover", "native_available", "scan"]
