> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Metrics and scores

Harness Lens keeps raw measurements, normalized scores, and findings distinct.
A metric preserves a value in its natural unit. A score is always in `[0, 1]`,
records its method and producer, and derives pass/fail from its threshold.

## Implemented signals

| Signal | Type | Meaning |
| --- | --- | --- |
| `harness.sources` | metric, count | safely loaded recognized sources |
| `harness.source_presence` | deterministic quality score | `1` when at least one source is present, otherwise `0` |
| `duration_micros` | plugin execution field | observed wall-clock plugin duration |

`quality_mean` averages only quality-category scores. Reliability, performance,
and safety stay separately visible in `by_category`; failed safety constraints
also increment `safety_violations`.

## Statistical building blocks

The Rust core exposes deterministic functions for population summaries,
Bernoulli outcome consistency, resource consistency, Brier score/probability
accuracy, and a Beta-Binomial probability estimate. Callers must retain raw
measurements, sample size, and declared assumptions. The Beta interval is a
normal approximation and is explicitly provisional for small or skewed samples.

These helpers do not imply one universal project score. See the
[Harness Evals prior-art note](prior-art/harness-evals.md) for provenance and
the [architecture](architecture.md) for aggregation boundaries.
