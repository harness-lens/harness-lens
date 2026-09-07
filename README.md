> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

![Harness Lens](assets/harness-lens-banner.png)

# Harness Lens

Harness Lens is an ecosystem for evidence-backed reports, observability, and
editor feedback about coding-agent harness files. This repository is the project
hub: it owns architecture, prior-art decisions, examples, governance, and a
reproducible composition of the implementation repositories.

## Repositories

| Repository | Ownership | Primary artifacts |
| --- | --- | --- |
| [core](https://github.com/harness-lens/core) | provider-neutral contracts, deterministic/heuristic analysis, statistics, plugins, report ports | `harness-lens-core`, `@harness-lens/core` |
| [sdk](https://github.com/harness-lens/sdk) | filesystem discovery, TOML configuration, Python bindings, Harness Score adapter | `harness-lens`, Python `harness-lens`, `@harness-lens/sdk` |
| [cli](https://github.com/harness-lens/cli) | terminal behavior and native executable | `harness-lens` binary, `@harness-lens/cli` |
| [language-server](https://github.com/harness-lens/language-server) | LSP lifecycle, workspace overlays, UTF-16 diagnostics | `harness-lens-lsp`, `@harness-lens/language-server` |
| [harness-lens-vscode](https://github.com/harness-lens/harness-lens-vscode) | VS Code process lifecycle, discovery UX, packaging | `harness-lens.harness-lens`, `@harness-lens/vscode` |

The [`modules/`](modules/) entries are Git submodules pinned to commits tested as
one ecosystem revision. Cargo dependencies additionally pin their upstream Git
commits. The `branch = main` hints in `.gitmodules` make deliberate future
updates possible without weakening the committed pins.

```text
core <- sdk/adapters <- cli
                    <- language-server <- VS Code

project hub --pins--> every repository
```

No core contract depends on OpenAI, Anthropic, Ollama, another model provider,
or an agent framework. Optional integrations consume generic reports through
adapters.

## Get the composed source

```bash
git clone --recurse-submodules https://github.com/harness-lens/harness-lens.git
cd harness-lens
git submodule status
```

For an existing checkout:

```bash
git submodule update --init --recursive
```

Build and installation commands live with the owning repository. Start with the
[SDK](https://github.com/harness-lens/sdk) for Rust/Python embedding, the
[CLI](https://github.com/harness-lens/cli) for terminal use, or the
[VS Code extension](https://github.com/harness-lens/harness-lens-vscode) for
inline diagnostics.

## What the proof currently covers

- deterministic adjacent-word repetition (`HL010`);
- deliberately conservative, evidence-labeled instruction incongruence (`HL020`);
- normalized deterministic, statistical, heuristic, and probabilistic result
  types with explicit sample/uncertainty metadata;
- failure-isolated plugins and observable execution records;
- safe, root-bounded harness-file discovery derived from Harness Score ideas;
- PyO3/Maturin Python acceleration;
- standard LSP diagnostics compatible with Problems and Error Lens;
- a transport-neutral Harness Score report-mapping seam.

See [architecture](docs/architecture.md), [repository split](docs/repository-split.md),
[validation rules](docs/validation-rules.md), [metrics](docs/metrics.md),
[integrations](docs/integrations.md), and [prior-art notes](docs/prior-art/).

## Configuration and examples

[`harness-lens.toml`](harness-lens.toml) is this repository's own configuration.
The reusable example lives at [`examples/harness-lens.toml`](examples/harness-lens.toml).
Plugin and integration options stay opaque to core and are resolved by the SDK.

The Python example assumes the package from the SDK repository is installed:

```bash
python examples/python_scan.py .
```

## Project status

Version `0.0.x` remains pre-alpha. Each implementation repository runs and
publishes its own tests, security checks, packages, and release artifacts. This
hub does not publish a duplicate crate, wheel, npm package, or extension.

## License

Early namespace-reservation versions used BSD-3-Clause. The official functional
implementation is licensed under MPL-2.0. When Covered Software is distributed,
modified MPL-covered files must remain available in Source Code Form under the
license. See [LICENSING](LICENSING.md), [COPYRIGHT](COPYRIGHT), and
[TRADEMARKS](TRADEMARKS).

## Authors

Harness Lens was created by [cristiancmrg](https://github.com/cristiancmrg) and
is maintained with contributions from the Harness Lens community.
