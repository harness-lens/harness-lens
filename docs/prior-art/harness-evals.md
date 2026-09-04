> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Prior art: Harness Evals

Research date: 2026-09-04

Harness Lens studies evaluation-engine techniques from
[`harness/harness-evals`](https://github.com/harness/harness-evals) while applying
them to static and observed harness quality. Harness Evals targets LLM/agent
evaluation; Harness Lens targets technical trust in the configuration,
instructions, tools, policies, and traces surrounding agents.

This is design attribution, not a source-code copy. Harness Evals is Apache-2.0;
Harness Lens implementation remains MPL-2.0 and uses independently written Rust.

## Adopted now

| Idea | Harness Lens use | Source |
| --- | --- | --- |
| Normalized score with threshold-derived pass state | `Score::new` validates `[0,1]` and computes `passed` | [Score model](https://github.com/harness/harness-evals/blob/main/src/harness_evals/core/score.py) |
| Deterministic-first evaluation | Core and first-party inventory need no model or API key | [PLAN design principles](https://github.com/harness/harness-evals/blob/main/PLAN.md) |
| Metric failure isolation and timing | Plugin errors become failed executions; every execution records duration | [Runner](https://github.com/harness/harness-evals/blob/main/src/harness_evals/core/runner.py), [issue #14](https://github.com/harness/harness-evals/issues/14) |
| Sources, metrics, and sinks as distinct ports | TOML/filesystem are inbound adapters; `ReportSink` is outbound | [Architecture](https://github.com/harness/harness-evals/blob/main/docs/architecture.md), [issue #20](https://github.com/harness/harness-evals/issues/20) |
| Safety never hidden in a quality average | `ScoreSummary` reports safety violations separately | [ADR-003](https://github.com/harness/harness-evals/blob/main/docs/adr/003-safety-never-averaged.md), [issue #16](https://github.com/harness/harness-evals/issues/16) |
| Actionable evidence for security checks | Findings and scores carry source and safe structured evidence | [issue #25](https://github.com/harness/harness-evals/issues/25) |
| Reliability separate from capability | Outcome consistency can report repeatable failure without calling it success | [PLAN reliability section](https://github.com/harness/harness-evals/blob/main/PLAN.md) |
| Evidence-first heuristic labeling | `HL020` states its exact normalization assumption and stays distinct from deterministic `HL010` | [PLAN design principles](https://github.com/harness/harness-evals/blob/main/PLAN.md) |

## Statistical techniques retained

`harness_lens_core::statistics` provides deterministic, zero-randomness helpers:

- population mean, variance, standard deviation, and coefficient of variation;
- outcome consistency using normalized Bernoulli variance;
- resource consistency using an explicitly documented coefficient-of-variation
  heuristic;
- Brier score and its explicitly named higher-is-better complement for
  probability/outcome pairs;
- Beta-Binomial posterior mean with declared prior, sample size, and approximate
  interval.

These functions produce inputs for evidence-backed plugins. They do not create a
universal Harness Lens score. Each plugin must state why a formula fits its data.
The related Harness Evals concepts are documented in its
[reliability plan](https://github.com/harness/harness-evals/blob/main/PLAN.md).

## Useful later, deferred now

- Trajectory histogram cosine similarity and normalized longest-common-subsequence
  scoring from [issue #22](https://github.com/harness/harness-evals/issues/22):
  useful after Harness Lens has a generic action-trace schema.
- Deterministic fault perturbations from
  [issue #21](https://github.com/harness/harness-evals/issues/21): useful after
  adapters have replayable integration tests.
- Unauthorized tool use, privilege escalation, sensitive argument leakage,
  instruction hierarchy, and exfiltration indicators from
  [issue #25](https://github.com/harness/harness-evals/issues/25): strong fit for
  future trace/security plugins, but not inferred from static files without
  evidence.
- Dimension-level rendering from
  [issue #23](https://github.com/harness/harness-evals/issues/23): useful after
  score dimensions and enough real samples stabilize.

## Rejected for core

- Provider-specific LLM clients and LLM-as-judge metrics. They belong in optional
  interpretation adapters and cannot alter deterministic scores.
- Open-ended semantic contradiction judging for on-type editor diagnostics. Its
  latency, nondeterminism, disclosure risk, and uncertain evidence do not fit;
  the initial rule therefore recognizes only exact strong-modal pairs whose
  harness scopes overlap.
- Prompt optimizers and conversation simulators. Different product scope.
- One aggregate score mixing safety, quality, performance, and reliability.
  It hides assumptions and tail risk.
- Dynamic plugins loaded as arbitrary native libraries. Unsafe ABI and trust
  properties are unresolved; compiled registration is the first safe contract.
