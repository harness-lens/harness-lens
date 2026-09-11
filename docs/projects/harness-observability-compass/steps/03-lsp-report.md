<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step 03 — LSP report transport

Owner: `modules/language-server`.

Expose same Core report through bounded workspace requests. Preserve UTF-8 byte
spans in Core; convert to UTF-16 only at protocol boundaries.

Add explicit bounds or pagination for large workspaces. Keep loading, partial,
missing, cycle, failure, and empty states observable. Unsaved editor buffers
overlay the report without changing disk history.

## Done when

Request/response fixtures, position conversion tests, and native JSON-RPC smoke
test pass. Report remains useful with runtime integration unavailable.
