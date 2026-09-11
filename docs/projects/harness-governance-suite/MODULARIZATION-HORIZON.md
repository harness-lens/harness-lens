<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Reusable module and package horizon

## Decision

Harness Lens already has the correct large ownership boundaries. Further reuse
should come from small libraries inside those owning repositories before it
creates more repositories or global package names.

Three implemented Rust package names received legitimate publication on
2026-09-11:

- `harness-lens-config`;
- `harness-lens-adapter-harness-score`; and
- `harness-lens-lsp`.

They were published only after the existing Rust dependency chain was updated,
merged owner-first, and verified from clean packages against crates.io.

## Current reusable surfaces

| Capability | Owner | Existing reusable surface | Decision |
| --- | --- | --- | --- |
| Domain models, reports, scores, statistics, plugins, report sinks | Core | `harness-lens-core`, `@harness-lens/core` | Keep one contract authority |
| TOML parsing and resolution | SDK | `harness-lens-config` crate | Published `0.0.2`; keep inside SDK release train |
| Filesystem discovery and embeddable scan facade | SDK | `harness-lens` crate, Python `harness-lens`, `@harness-lens/sdk` | Keep as main embedding facade |
| Harness Score mapping | SDK | `harness-lens-adapter-harness-score` crate | Published `0.0.2`; keep transport-neutral |
| Native command, output, exit behavior, TUI | CLI | `harness-lens-cli` binary and `@harness-lens/cli` | Extract internal terminal library before considering another public crate |
| Editor-neutral protocol | Language server | `harness-lens-lsp` binary/crate and `@harness-lens/language-server` | Published Rust `0.0.1`; retain protocol position conversion here |
| Runtime aggregate correlation | Harness Metrics | `harness-metrics` crate | Preserve name, add ownership redundancy, expose restored source |
| Go report consumption | Go repository | `github.com/harness-lens/go` | Make public and tag reviewed `v0.1.0` after approval |
| Editor presentation | VS Code and Visual Studio | LSP clients; internal TypeScript view state | Share protocol data, never rule code |
| Desktop presentation | Desktop repository | Planned offline Tauri report viewer | Consume completed reports; no second scanner |

## Registry audit and release outcome

Current crates.io artifacts verified on 2026-09-11:

- `harness-lens-core 0.0.2`;
- `harness-lens-config 0.0.2`;
- `harness-lens-adapter-harness-score 0.0.2`;
- `harness-lens 0.0.2`;
- `harness-lens-cli 0.0.1`; and
- `harness-lens-lsp 0.0.1`; and
- `harness-metrics 0.0.4`.

All seven have one named owner, so continuity needs a trusted backup owner/team.
The new Core, SDK, config, adapter, and LSP archives carry MPL-2.0 and their
owning-repository metadata. Older immutable archives retain their original
license and metadata and are not rewritten.

The release started because workspace tests used immutable Git revisions while
clean package verification resolved obsolete crates.io dependencies:

- `harness-lens-config 0.0.1` fails because registry
  `harness-lens-core 0.0.1` lacks current `HarnessLensConfig`;
- `harness-lens-adapter-harness-score 0.0.1` fails because that Core archive
  lacks current report, score, completeness, and report-sink APIs; and
- `harness-lens-lsp 0.0.1` fails because registry `harness-lens 0.0.1` lacks
  current scanner, finding, span, path, and config APIs.

The completed owner-first release order was:

1. merge [core#26](https://github.com/harness-lens/core/pull/26) and publish
   `harness-lens-core 0.0.2` from `2a8e916f`;
2. merge [sdk#32](https://github.com/harness-lens/sdk/pull/32), pin Core, and
   publish config, adapter, and facade `0.0.2` from `c11b8683`;
3. merge [language-server#32](https://github.com/harness-lens/language-server/pull/32),
   pin SDK, and publish `harness-lens-lsp 0.0.1` from `be825fd4`; and
4. update CLI's immutable SDK pin and registry requirement, while leaving CLI
   publication to its supervised release runbook.

Each registry candidate must be built again from a clean checkout using only
registry dependencies. Workspace success against Git pins is insufficient.

## Next reusable module

The strongest additional boundary is a cross-language conformance suite. Begin
with synthetic, content-safe schema fixtures and black-box expected results. It
may later become `harness-lens-conformance`, but only after a useful suite exists.

It owns:

- canonical synthetic schema-version fixtures;
- valid, invalid, incomplete, truncated, Unicode-span, and forward-field cases;
- deterministic ordering and round-trip expectations;
- black-box CLI/LSP process probes;
- compatibility matrices for Rust, TypeScript, Python, Go, and later C/C++;
- fixture checksums and provenance; and
- helpers that assert reports without serializing real source content.

It does not own report models, analysis rules, graph derivation, or protocol
implementation. Core remains the schema authority. Start fixtures in Core and
export a versioned conformance bundle. Split a repository/package only after at
least two independent consumers use the same bundle; Go interoperability already
provides the first external consumer signal.

## Internal-first modules

These modules improve reuse but do not justify global package publication yet.

### Terminal library

Add a library target inside the CLI repository for:

- argument-independent presentation models;
- stable text/JSON rendering;
- terminal width, color, and accessibility policy;
- TUI state transitions; and
- snapshot-driven renderer tests.

The CLI binary, non-interactive terminal output, and TUI consume it. A GUI does
not: GUIs consume report/protocol contracts. Consider publishing
`harness-lens-terminal` only after another terminal host needs the API and its
semver can differ from the CLI release.

### Shared GUI renderer

Keep a framework-neutral internal TypeScript package in a presentation
repository for view state, filtering, graph projection, accessibility tables,
and offline assets. VS Code, Visual Studio, or Desktop must prove a second real
consumer before publishing `@harness-lens/renderer`. The controlled npm scope
already protects that future name.

### History and storage

Keep snapshot comparison and storage ports in Core; keep filesystem or SQLite
implementations in SDK. Split a `harness-lens-store` crate only after profiling
shows a real backend boundary used by CLI and Desktop. Storage never becomes a
Core filesystem dependency.

### Plugin protocol

Keep the current in-process Rust `Plugin` and `ReportSink` contracts in Core.
Do not publish `harness-lens-plugin-api` while Rust ABI, trust, signature,
permission, and crash-isolation rules are unresolved. A future out-of-process
protocol can become a small language-neutral package after one external plugin
works through versioned content-safe messages.

## Deferred package names

| Candidate | Current decision | Promotion evidence |
| --- | --- | --- |
| `harness-lens-conformance` | Build useful fixtures first | Two consumers, versioned bundle, clean black-box runner |
| `harness-lens-terminal` | Internal CLI library | Second terminal host and stable renderer API |
| `@harness-lens/renderer` | Internal presentation package | Two GUI hosts consuming same code and release contract |
| `harness-lens-store` | SDK module | Measured history/query workload and two consumers |
| `harness-lens-plugin-api` | Core module | External plugin, stable protocol, trust/crash boundary |
| `harness-lens-wasm` | Defer | Browser host that cannot use LSP or native CLI |
| .NET or Java SDK | Defer | Real non-LSP embedding consumer |

Do not create separate repositories for TUI, shell completions, report models,
graph schema, individual GUI views, or provider-specific Core logic. Those names
would divide existing owners without adding an independent release contract.

## Expansion tests

Every new reusable surface should pass four layers:

1. owner unit tests for its pure behavior;
2. content-safe conformance fixtures shared across languages;
3. clean-package tests resolving only published dependencies; and
4. black-box consumer tests for CLI, LSP, terminal, or GUI lifecycle.

Fuzz/property testing belongs at parser, config, report decoder, span, and graph
bounds. Golden tests belong at stable JSON/text contracts. GUI snapshots test
presentation state and accessibility, never duplicate analysis rules.
