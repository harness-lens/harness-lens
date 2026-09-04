# SPDX-License-Identifier: MPL-2.0
# Copyright © 2026 Cristian Camargo Filho

"""Print a Harness Lens report using the Python facade over the Rust core."""

from __future__ import annotations

import json
import sys

from harness_lens import scan


root = sys.argv[1] if len(sys.argv) > 1 else "."
print(json.dumps(scan(root), indent=2))
