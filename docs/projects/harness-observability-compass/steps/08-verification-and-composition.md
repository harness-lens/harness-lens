<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step 08 — verification and composition

Run last, after owning repositories merge and their checks are green.

1. Verify Core.
2. Verify SDK against immutable Core revision.
3. Verify CLI and language server independently.
4. Verify VS Code package and editor tests.
5. Update hub submodule pins and split documentation last.

At hub:

```bash
git submodule status --recursive
git diff --check
git diff --exit-code --submodule=short
```

Record exact merge SHAs, commands, and known limitations. Keep implementation
out of hub. Do not claim remote merge from stale local checkout.
