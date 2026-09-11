<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# North star

Harness Lens helps a developer understand whether a coding-agent harness is
getting cheaper, more stable, and more effective over time.

Every useful statement should answer four questions:

- What happened: finding, cost, error, retry, latency, or trend.
- How certain: deterministic, heuristic, statistical, or probabilistic method.
- What caused the context: harness, skill, config, referenced file, or tool.
- Where to inspect: exact safe file and source range when available.

## Product shape

GitLens-like navigation shows workspace assets, skills, references, findings,
context consumption, runtime history, and comparisons. A focused dashboard
shows trends and distributions. Tree navigation remains useful without runtime
data; runtime views state when evidence is unavailable.

## Boundary

Static analysis is local-first and deterministic. Runtime capture is optional,
consent-controlled, and never changes deterministic findings or scores. Raw
source, tool arguments, outputs, transcripts, stderr, secrets, and credentials
never enter reports.

## Required completion boundary

This plan has an end. Required implementation is complete when:

- steps 00 through 08 in [`04-step-order.md`](04-step-order.md) satisfy their
  acceptance checks in order;
- applicable acceptance gates in the
  [unified plan](../harness-governance-suite/UNIFIED-PLAN.md) pass;
- owning repository changes are merged and downstream immutable pins are
  independently verified; and
- `state/now.md` records no unfinished required slice.

At that point, stop this implementation plan and mark it complete. Step 09 is
explicitly deferred and does not block completion. New packages, views,
providers, integrations, or speculative refinements do not become required work
because they are discovered during implementation.

## Transition after completion

After completion, transition this plan to maintenance: correctness, security,
compatibility, release integrity, and measured user problems. Any new product
scope requires its own decision record under the
[decision matrix](../harness-governance-suite/DECISION-MATRIX.md), explicit
adoption, and a separate bounded plan. New work does not reopen this completed
plan automatically.
