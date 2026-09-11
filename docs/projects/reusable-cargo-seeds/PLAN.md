<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Reusable Cargo seeds

## Decision

Create two small, useful Rust libraries in their existing owning repositories:

- `harness-lens-terminal` in CLI owns deterministic human and JSON rendering of
  completed Core reports;
- `harness-lens-store` in SDK owns a backend-neutral report-store contract and
  a bounded immutable JSON directory implementation.

Both remain below their current owners. Core gains no filesystem or terminal
dependency. GUIs continue to consume report or protocol contracts.

## Matrix record

| Slice | Ratings | Value | Confidence | Effort | Decision |
| --- | --- | ---: | ---: | ---: | --- |
| Terminal renderer seed | `4/4/5/3/4/5/5` | 84 | 1.00 | 2 | Adopt and publish useful `0.0.1` |
| Bounded report-store seed | `4/5/5/3/4/5/3` | 84 | 1.00 | 2 | Adopt and publish useful `0.0.1` |

Ratings use the governance matrix order
`developer/trust/architecture/unlock/reuse/evidence/time`. Evidence is the CLI
consumer, immutable content-safe report schema, owner tests, clean package
verification, and explicit user need for reusable terminal and storage seams.

### Terminal decision record

- `decision_id`: `cargo-seed-terminal-2026-09-11`
- `owner_repository`: `harness-lens/cli`
- `scorer`: project maintainer with Codex research/implementation evidence
- alternatives: keep rendering in the binary; publish an empty reservation;
  create a separate repository
- gates: ownership, no duplication, technical trust, data safety, and legitimate
  artifact all pass through the small renderer API, CLI consumer, and tests
- criterion evidence: removes duplicated host formatting risk; preserves Core
  reports; fits CLI ownership; enables later terminal hosts; is implemented and
  registry-package verified; name availability made delay costly
- dependencies: `harness-lens-core 0.0.2`
- approval: explicit user adoption and Cargo publication authorization
- execution index: `84 * 1.00 / 2 = 42`
- decision: adopt; review on any proposed TTY/TUI/process scope

### Store decision record

- `decision_id`: `cargo-seed-store-2026-09-11`
- `owner_repository`: `harness-lens/sdk`
- `scorer`: project maintainer with Codex research/implementation evidence
- alternatives: duplicate storage in each host; put filesystem code in Core;
  publish an empty reservation; add SQLite immediately
- gates: ownership, no duplication, technical trust, data safety, and legitimate
  artifact all pass through the content-safe report contract, immutable writes,
  portable keys, explicit bounds, and tests
- criterion evidence: centralizes a reusable seam; enforces safe local bounds;
  keeps Core pure; enables CLI/Desktop reuse; is implemented and
  registry-package verified
- dependencies: `harness-lens-core 0.0.2`
- approval: explicit user adoption and Cargo publication authorization
- execution index: `84 * 1.00 / 2 = 42`
- decision: adopt; review on any new backend, history, query, or retention scope

## Included work

1. Add and test both libraries in their owner repositories.
2. Make the native CLI consume the terminal renderer without changing its
   command or release contract.
3. Package each crate with registry dependencies only.
4. Merge green owner pull requests and publish version `0.0.1`.
5. Pin accepted owner commits in this hub and record the artifacts.

## Excluded work

- TTY detection, color, terminal sizing, process control, interactive TUI state,
  shell completion, or GUI rendering;
- SQLite, network/object storage, migrations, retention policy, history UI, or
  query indexing;
- CLI `0.0.5`, npm, PyPI, desktop, or editor releases.

## Acceptance

- owner formatting, lint, tests, packaging, and CI pass;
- rendering is deterministic and serializes no raw source;
- store keys are portable and validated, writes are immutable, reads reject
  unsupported files, and size/listing limits are explicit;
- `harness-lens-terminal 0.0.1` and `harness-lens-store 0.0.1` are available on
  crates.io from merged owner commits;
- the CLI package verifies against the published terminal dependency; and
- this hub records the immutable SDK and CLI pins last.

## Completion boundary

This plan ends when every acceptance item above is satisfied. TUI behavior,
additional store backends, query/history behavior, and new consumers are future
work. Each requires a separate matrix decision and bounded plan; they do not
extend this seed plan.
