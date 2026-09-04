> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Harness Lens

Harness Lens produces evidence-backed reports about coding-agent harness files,
configuration, policies, and eventually observed executions. Rust owns the fast,
deterministic core; Python, CLI, language servers, editors, model providers, and
reporting products remain adapters.

## Installation

```bash
python -m pip install harness-lens
```

## Usage

Inspect the current repository:

```bash
harness-lens .
```

Emit JSON for scripts:

```bash
harness-lens . --json
```

Use the Python SDK:

```python
from harness_lens import discover, scan

files = discover(".")
report = scan(".")
print(report["score_summary"])
```

The preview recognizes:

- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`
- `.github/copilot-instructions.md`
- files under `.cursor/rules/`

It searches nested repository directories while ignoring common generated and dependency directories.

The Rust core also reports adjacent word repetition (`HL010`) and narrowly
defined opposite strong instructions (`HL020`). Markdown fenced code is excluded
from both checks. The language server publishes those findings as standard LSP
diagnostics:

```bash
cd rust
cargo run -p harness-lens-lsp
```

A thin editor client can launch that process. VS Code's Problems view and
extensions such as Error Lens can render the diagnostics without analysis logic
inside the editor extension. See [language server](docs/language-server.md).

## Architecture

```text
CLI / Python / LSP / integrations
                 SDK
                Core
```

`harness-lens-core` exposes generic models, normalized evidence scores,
deterministic statistics, plugin inputs, and report outputs. It has no model
provider or agent-framework dependency. Maturin builds the PyO3 module
`harness_lens._native` against the Python 3.10 stable ABI. A pure-Python discovery
fallback keeps source-tree workflows usable before Rust compilation.

See [architecture](docs/architecture.md), [integration boundaries](docs/integrations.md),
[language server](docs/language-server.md), and the [prior-art notes](docs/prior-art/).

Current proof and operating boundaries are documented in [metrics](docs/metrics.md),
[validation rules](docs/validation-rules.md), [security policy](SECURITY.md), and
the runnable [examples](examples/README.md).

## Configuration

Repository-local [`harness-lens.toml`](harness-lens.toml) controls discovery,
plugins, and integrations. Explicit API/CLI paths override the repository-local
file. Plugin and integration options stay opaque to core.

## Project status

Version `0.0.1` remains pre-alpha. Rust now implements discovery, safe loading,
provider-neutral reports, plugin execution observability, normalized scoring,
deterministic statistical helpers, text diagnostics, and an LSP adapter. History,
the VS Code launcher, broader validation, and optional AI interpretation remain
planned.

## Development

```bash
python3 -m pip install -e ".[dev]"
python3 -m pytest
cd rust && cargo test --workspace
cd rust && cargo fmt --all --check
cd rust && cargo clippy --workspace --all-targets -- -D warnings
python3 -m build
python3 -m twine check --strict dist/*
```

Release setup lives in [publishing documentation](docs/publishing.md). The
cross-registry ownership and administration inventory lives in the [registry
and administration map](docs/registry-and-administration.md).

## License

Early namespace-reservation versions used BSD-3-Clause. The official functional
implementation is licensed under MPL-2.0. When Covered Software is distributed,
modified MPL-covered files must remain available in Source Code Form under the
license. See [LICENSING](LICENSING.md), [COPYRIGHT](COPYRIGHT), and
[TRADEMARKS](TRADEMARKS).

## Authors

HarnessLens was created by [cristiancmrg](https://github.com/cristiancmrg) and is maintained with contributions from the HarnessLens community.
