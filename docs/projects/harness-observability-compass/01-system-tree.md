<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# System tree

```text
Harness Lens hub
├── core
│   └── contracts, findings, scores, statistics, provenance, orchestration
├── sdk
│   └── bounded discovery, config adapters, snapshots, runtime facades
├── cli
│   └── terminal commands and local report rendering
├── language-server
│   └── workspace reports, diagnostics, hover, code lenses, JSON-RPC transport
└── harness-lens-vscode
    └── Activity Bar, trees, dashboard, settings, source navigation

Optional runtime path
CodeBurn ──> harness-metrics ──> language-server ──> VS Code
```

Dependencies point inward: `core <- sdk <- CLI/LSP <- editor`.
The optional runtime path is separate from deterministic path.

## Main data flow

```text
workspace files
  -> SDK discovery and bounded inclusion graph
    -> Core report with per-file measurements and safe provenance
      -> LSP protocol conversion and diagnostics
        -> CLI and VS Code trees/dashboard
```

Per-file records are the join point for bytes, estimated tokens, configured
cost, findings, references, and later attributable runtime evidence.
