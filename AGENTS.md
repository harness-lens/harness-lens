> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Harness Lens contributor instructions

## Mission

Harness Lens produces evidence-backed, local-first reports about coding-agent
harnesses. Optimize for technical trust: deterministic behavior, explicit
assumptions, observable execution, safe defaults, and provider neutrality.

## Architecture boundaries

- `rust/crates/harness-lens-core` owns domain models, score/statistical helpers,
  plugin contracts, report-sink contracts, and deterministic orchestration.
- Core must not depend on a model provider, agent framework, filesystem format,
  network client, Python, editor, or product SDK.
- `rust/crates/harness-lens-config` adapts TOML into core configuration.
- `rust/crates/harness-lens` is the Rust SDK and filesystem adapter.
- `rust/crates/harness-lens-python` is the PyO3 boundary used by Python.
- `rust/crates/harness-lens-cli` is a terminal adapter.
- `rust/crates/harness-lens-lsp` maps generic findings to standard LSP
  diagnostics and owns editor position conversion.
- `rust/crates/harness-lens-adapter-*` contains external product integrations.
- Dependencies point inward. Core never imports an adapter.

## Analysis rules

- Prefer deterministic checks. Optional AI interpretation consumes completed
  reports and must not alter deterministic findings or scores.
- Every score is normalized to `[0.0, 1.0]`; pass/fail derives from its threshold.
- Mark score method as deterministic, heuristic, statistical, or probabilistic.
- Heuristics must include evidence and assumptions.
- Statistical results must expose sample size; probabilistic results must expose
  their prior/interval method.
- Safety failures remain separate from quality averages.
- Plugin failures become observable execution results, not process-wide crashes.
- Never put raw source contents, secrets, or credentials in serialized reports.
- Text findings use UTF-8 byte spans in core. Editor adapters convert them to
  the negotiated protocol encoding; LSP currently advertises UTF-16.

## Verification

Run before submitting changes:

```bash
cd rust && cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings
cd rust && cargo test --workspace
python3 -m pytest
python3 -m build
```

Add focused tests for every public behavior and statistical edge case. Keep
source files under MPL-2.0 headers. External ideas and algorithms need a note in
`docs/prior-art/` explaining adoption, rejection, and source links.
