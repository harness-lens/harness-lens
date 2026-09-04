<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# harness-lens-lsp

Editor-neutral Language Server Protocol adapter for Harness Lens. It scans the
workspace with the Rust SDK, overlays unsaved open documents, and publishes
standard diagnostics with stable Harness Lens rule codes.

Run over standard input/output:

```bash
cargo run -p harness-lens-lsp
```

The server keeps presentation out of the core. VS Code extensions—including
Error Lens—can render the published diagnostics without Harness Lens depending
on their APIs.
