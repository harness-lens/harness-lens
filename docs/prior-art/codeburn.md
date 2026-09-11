<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Prior art: CodeBurn

Research date: 2026-09-04

[CodeBurn](https://github.com/getagentseal/codeburn) reads local coding-agent
session stores and reports token usage, cost, cache behavior, model efficiency,
skills, and optimization opportunities. Its documented JSON surfaces include
[`report --format json`](https://codeburn.app/docs/json-output), `models`, and
[`optimize --json`](https://codeburn.app/docs/cli-options).

Harness Lens adopts a composition boundary, not CodeBurn algorithms:

- CodeBurn owns session discovery/parsing, model aliases, pricing, efficiency,
  and optimization semantics.
- Harness Metrics consumes aggregate JSON, accepts unknown fields, labels
  deterministic/heuristic/statistical methods, and correlates model or skill
  identifiers with UTF-8 document spans.
- The language server captures one snapshot at startup or explicit refresh,
  then converts spans to LSP positions for diagnostics, hover, and code lenses.
- Harness Lens core remains independent from runtime stores, subprocesses,
  pricing databases, providers, and editor protocols.

Rejected choices:

- copying CodeBurn parsers, price tables, or optimization detectors;
- reading raw transcripts in Harness Metrics or the language server;
- treating optional AI/runtime interpretation as deterministic score input;
- using MCP as the canonical replay/CI boundary when JSON snapshots are easier
  to cache, inspect, and test.

Assumptions and safeguards:

- `report` and `models` are required; missing `optimize` output produces a safe,
  observable warning and omits optimization insights.
- Some provider costs use estimated tokens. Estimated cost stays labeled, and
  generic savings remain “CodeBurn-reported” unless CodeBurn marks their basis.
- Statistical text exposes calls, sessions, turns, or edit-turn sample sizes.
- CodeBurn recommendation `savingsPct` is already scaled from 0 to 100.
- Snapshots contain aggregates and may contain project paths/model names; they
  never include Harness Lens source text and should not be committed blindly.

CodeBurn is MIT-licensed. Harness Lens and Harness Metrics are MPL-2.0. No
CodeBurn source was copied.
