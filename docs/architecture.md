> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Architecture

Harness Lens uses multiple implementation repositories because its analysis
model serves more than one language and ecosystem. Separation makes ownership,
release cadence, and dependency direction explicit. The umbrella repository
keeps design material and pins known-compatible revisions as submodules.

```text
                         harness-lens (hub)
                         /   /   |   \   \
                        pins exact component revisions

core <- sdk + adapters <- CLI
                      <- language server <- VS Code
```

| Layer | Repository | Owns | Must not own |
| --- | --- | --- | --- |
| Domain | [core](https://github.com/harness-lens/core) | models, evidence, normalized scores, statistical helpers, plugin/report ports, analysis rules | filesystem, transport, editor, model vendor |
| Application/adapters | [sdk](https://github.com/harness-lens/sdk) | safe discovery, TOML, Rust/Python/TypeScript facades, integration mappings | terminal or editor presentation |
| Terminal | [cli](https://github.com/harness-lens/cli) | arguments, output, exit behavior | analysis rules |
| Protocol | [language-server](https://github.com/harness-lens/language-server) | LSP state, overlays, UTF-8-to-UTF-16 conversion | analysis rules or VS Code APIs |
| Editor | [harness-lens-vscode](https://github.com/harness-lens/harness-lens-vscode) | server lifecycle, selectors, commands, views, packaging | duplicate analysis |

Lower layers never import upper layers. Model providers and agent frameworks
belong in optional adapter repositories/packages. They receive generic reports;
core never receives OpenAI, Anthropic, Ollama, LangChain, or similar types.

## Core contracts

`HarnessSource` contains loaded UTF-8 input for in-process analysis.
`SourceRecord` is the content-free report form. `Finding` carries rule, severity,
location, a protocol-neutral UTF-8 byte span, and safe evidence. `Metric`
preserves a raw measurement. `Score` normalizes interpretation to `[0.0, 1.0]`,
derives pass/fail from a threshold, and records method, sample size, uncertainty,
evidence, and producer.

`quality_mean` includes only quality scores. Safety, reliability, and performance
remain separate dimensions, so a severe safety failure or unstable execution
cannot disappear inside a broad average.

## Plugins

Rust plugins implement the generic `Plugin` contract and are registered by a
host. Core provides immutable config and sources, collects output, measures
execution time, and records disabled, unavailable, failed, or completed status.
Configured but unavailable plugins do not abort other analysis.

Initial plugins compile in-process. Dynamic library loading is deferred until a
stable ABI, trust policy, signatures, permissions, and crash isolation exist.
Future out-of-process plugins can preserve the contract through a versioned
protocol.

## Configuration and Python

`harness-lens.toml` is versioned. Core owns generic typed values; the SDK owns
TOML parsing and resolution: explicit caller path, repository-local file, then
built-in defaults. Plugin and integration option maps remain opaque to core.

The SDK's Maturin build produces `harness_lens._native` through PyO3 and the
Python 3.10 stable ABI. A small discovery fallback keeps source-tree Python use
available before native compilation; installed wheels use Rust.

## Discovery and editor boundary

The SDK traverses in deterministic order, skips configured generated/dependency
directories, applies a file-count fuse, and prevents followed symlinks from
escaping the canonical workspace. Incomplete scans are explicit, content-free
report data.

The language server overlays open editor buffers and maps core byte spans to
UTF-16 LSP ranges. It publishes `source = harness-lens` and stable `HL...` codes.
The VS Code extension launches the server; Problems, Error Lens, and any other
diagnostic consumer can render results without a private integration.

## Cross-repository reproducibility

Cargo manifests pin upstream Git revisions and retain registry version
constraints. This lets dependent feature branches build before release while
preserving publishable dependency metadata. The hub's submodule gitlinks pin the
full set of repositories independently of `.gitmodules` branch hints.

See [repository split](repository-split.md), [Harness Evals prior art](prior-art/harness-evals.md),
and [integration boundaries](integrations.md).
