> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Architecture

Harness Lens is a monorepo because one analysis model must serve Rust, Python,
CLI, language-server, editor, CI, and reporting ecosystems without giving any
adapter ownership of the domain.

```text
rust/crates/
├── harness-lens-core
│   ├── domain models and evidence
│   ├── normalized score model
│   ├── deterministic statistics
│   ├── Plugin input port
│   └── ReportSink output port
├── harness-lens-config                TOML adapter
├── harness-lens                       Rust SDK + filesystem adapter
├── harness-lens-python                PyO3 adapter
├── harness-lens-cli                   terminal adapter
├── harness-lens-lsp                   LSP diagnostic adapter
└── harness-lens-adapter-harness-score outbound integration seam

src/harness_lens/                      Python SDK and source-tree fallback
```

Dependency rule:

```text
CLI / Python / LSP / integrations
                 SDK
                Core
```

Lower layers never import upper layers. Model providers and agent frameworks
belong in optional adapter crates or packages. They receive generic source and
report types; core never receives OpenAI, Anthropic, Ollama, LangChain, or other
vendor/framework types.

## Core contracts

`HarnessSource` contains loaded UTF-8 input for in-process analysis.
`SourceRecord` is the content-free report form. `Finding` carries rule, severity,
location, a protocol-neutral UTF-8 byte span, and safe evidence. `Metric`
preserves a raw measurement. `Score`
normalizes interpretation to `[0.0, 1.0]`, derives pass/fail from a threshold,
and records method, sample size, uncertainty, evidence, and producer.

`quality_mean` contains only `ScoreCategory::Quality`. Safety, reliability, and
performance remain separate dimensions, so a severe safety failure or unstable
execution cannot disappear inside a broad average.

## Plugins

Rust plugins implement `Plugin` and are registered by a host. Core gives each
plugin immutable config and sources, collects output, measures execution time,
and records disabled, unavailable, failed, or completed status. Configured but
unregistered plugins do not abort other analysis.

Initial plugins are compiled in-process. Dynamic shared-library loading is
deferred because a stable Rust ABI, trust policy, signature model, capability
permissions, and crash isolation must be designed first. Future out-of-process
plugins can implement the same conceptual contract over a versioned protocol.

## Configuration

Repository-local `harness-lens.toml` is versioned. Core owns typed config;
`harness-lens-config` owns TOML parsing. Plugin and integration options remain
opaque string maps so adapters evolve without adding vendor concepts to core.

Resolution order is explicit CLI/API path, repository-local config, built-in
defaults.

## Python boundary

Maturin builds `harness_lens._native` from `harness-lens-python`. PyO3 uses the
Python 3.10 stable ABI feature. Python exposes `discover()` and `scan()` and
retains a small discovery fallback so source-tree tests and documentation remain
usable before compiling Rust. Installed wheels use Rust.

## Discovery and editor boundary

The SDK filesystem adapter traverses in deterministic order, skips configured
dependency/generated directories, applies a file-count fuse, and never follows
a symlink outside the canonical workspace root. Conditions that prevent full
coverage become content-free `ScanCompleteness` reasons rather than an
authoritative-looking partial scan.

`harness-lens-lsp` holds open editor buffers as in-memory overlays. It maps core
byte spans to UTF-16 LSP ranges and publishes stable `source = harness-lens` and
`code = HL...` diagnostics. It does not depend on VS Code or Error Lens.

## Planned adapters

- Language Server Protocol: diagnostics are implemented; code actions remain planned.
- VS Code: thin client for the language server; no analysis logic.
- Model interpretation: optional report consumer, never a scorer of record.
- OTLP: plugin timing, finding counts, and score dimensions.
- Harness Score: map the stable report through its adapter and injected transport.

See [Harness Evals prior art](prior-art/harness-evals.md) for adopted and deferred
evaluation techniques.
