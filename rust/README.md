> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Harness Lens Rust workspace

This workspace contains the provider-neutral core and its adapters:

- `harness-lens-core`: models, evidence, scoring, statistics, plugins, sinks;
- `harness-lens-config`: TOML adapter;
- `harness-lens`: Rust SDK and filesystem adapter;
- `harness-lens-python`: private PyO3 extension built by Maturin;
- `harness-lens-cli`: terminal adapter;
- `harness-lens-lsp`: standard-diagnostic language server adapter;
- `harness-lens-adapter-harness-score`: transport-neutral product bridge.

Dependencies point toward `harness-lens-core`. Core does not depend on Python,
model providers, agent frameworks, transports, or presentation layers.

The Rust workspace follows the repository-wide MPL-2.0 policy. See the root
[LICENSING](../LICENSING.md), [COPYRIGHT](../COPYRIGHT), and
[TRADEMARKS](../TRADEMARKS) files.
