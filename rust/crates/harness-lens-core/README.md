> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# harness-lens-core

Provider-neutral domain and analysis engine for Harness Lens. It owns source and
finding models, normalized evidence scores, category-aware aggregation,
deterministic statistical helpers, plugin contracts, report-sink contracts, and
failure-isolated orchestration.

Built-in text plugins report deterministic adjacent repetition and conservative,
explicitly heuristic opposite-modal instructions. Findings carry UTF-8 byte
spans so protocol adapters can convert positions without coupling core to LSP.

It does not read files, parse TOML, call networks, execute agents, import model
providers, or know about Python and editors. Hosts inject those behaviors.

## License

Early namespace-reservation versions used BSD-3-Clause. The official functional
implementation is licensed under MPL-2.0. See [LICENSING](../../../LICENSING.md),
[COPYRIGHT](../../../COPYRIGHT), and [TRADEMARKS](../../../TRADEMARKS).
