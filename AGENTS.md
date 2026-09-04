> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Harness Lens contributor instructions

## Mission

Harness Lens produces evidence-backed, local-first reports about coding-agent
harnesses. Optimize for technical trust: deterministic behavior, explicit
assumptions, observable execution, safe defaults, and provider neutrality.

## Repository role

This is the architecture and composition hub. Do not add implementation modules
back to this repository. Change code in its owning repository, merge that pull
request, then update the corresponding `modules/*` Git submodule pin here.

- [`modules/core`](modules/core) — domain models, score/statistical helpers,
  plugin/report contracts, deterministic orchestration.
- [`modules/sdk`](modules/sdk) — filesystem discovery, config/PyO3 adapters,
  Harness Score mapping, embedding facades.
- [`modules/cli`](modules/cli) — terminal adapters.
- [`modules/language-server`](modules/language-server) — LSP diagnostics and
  protocol position conversion.
- [`modules/harness-lens-vscode`](modules/harness-lens-vscode) — editor process
  lifecycle, presentation, and packaging.

Dependencies point inward: `core <- sdk <- CLI/LSP <- editor`. Core must not
depend on a model provider, agent framework, filesystem format, network client,
Python, editor, or product SDK.

## Analysis rules

- Prefer deterministic checks. Optional AI interpretation consumes completed
  reports and must not alter deterministic findings or scores.
- Normalize every score to `[0.0, 1.0]`; derive pass/fail from its threshold.
- Mark methods as deterministic, heuristic, statistical, or probabilistic.
- Heuristics include evidence and assumptions. Statistical results expose sample
  size; probabilistic results expose prior and interval method.
- Keep safety failures separate from quality averages.
- Record plugin failure as observable execution output, not a process-wide crash.
- Never serialize raw source contents, secrets, or credentials into reports.
- Core text findings use UTF-8 byte spans; protocol adapters perform position
  conversion. The language server currently publishes UTF-16 positions.

## Cross-repository changes

1. Modify and verify the lowest-level owning repository first.
2. Commit it and pin its immutable SHA in downstream Cargo manifests.
3. Verify downstream repositories independently.
4. Update this hub's Git submodule pins and `docs/repository-split.md` last.

Use normal GitHub links for navigation and immutable SHAs for builds. Do not make
the submodules a Cargo or npm monorepo workspace; they are a reproducible source
composition only.

## Verification

Run the commands documented in every changed component repository. At the hub,
verify composition with:

```bash
git submodule status --recursive
git diff --exit-code --submodule=short
```

External ideas and algorithms need a note in `docs/prior-art/` explaining
adoption, rejection, assumptions, and source links.
