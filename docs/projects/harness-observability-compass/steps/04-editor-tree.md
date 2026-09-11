<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step 04 — editor tree and dashboard

Owner: `modules/harness-lens-vscode`.

Provide a small Activity Bar surface:

```text
Harness Lens
├── Workspace assets
├── Skills and references
├── Findings
├── Context consumption
└── Runtime history
```

Every file row can show bytes, estimated tokens, configured cost, findings,
provenance, and evidence method. Clicking safe provenance item opens exact
source location. Dashboard graphs show distributions and trends only when
sample requirements are met.

Runtime-unavailable, loading, empty, partial, and failure states need explicit
copy. Do not fake runtime history from static estimates.

## Done when

Tree navigation, source links, refresh, and static dashboard work with runtime
mode off. TypeScript, extension tests, and package inspection pass.
