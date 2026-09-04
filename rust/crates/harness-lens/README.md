> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# harness-lens

Reusable Rust SDK for Harness Lens. It combines the generic engine with a safe
filesystem adapter, repository-local configuration resolution, and first-party
deterministic plugins.

Use `discover()` for content-free path discovery or `Scanner::scan()` for a
complete evidence-bearing report. Add analyzers through `Scanner::register_plugin`.
`discover_detailed()` exposes incomplete-scan reasons, and
`Scanner::scan_with_overrides()` safely analyzes unsaved editor buffers.

## License

Early namespace-reservation versions used BSD-3-Clause. The official functional
implementation is licensed under MPL-2.0. See [LICENSING](../../../LICENSING.md),
[COPYRIGHT](../../../COPYRIGHT), and [TRADEMARKS](../../../TRADEMARKS).
