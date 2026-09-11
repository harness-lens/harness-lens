<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Prior art: weighted decision and execution matrices

Research date: 2026-09-11

The [NASA Systems Engineering Handbook](https://www.nasa.gov/wp-content/uploads/2018/09/nasa_systems_engineering_handbook_0.pdf)
describes decision analysis using established criteria, weighted trade-off
matrices, engineering judgment, and explicit treatment of uncertainty when it
could change the ranking. The UK government's
[Multi-criteria analysis manual](https://www.gov.uk/government/publications/multi-criteria-analysis-manual-for-making-government-policy)
documents decision context, alternatives, criteria, scores, weights, aggregate
scores, and sensitivity analysis. The Scaled Agile Framework describes
[Weighted Shortest Job First](https://framework.scaledagile.com/blog/glossary_term/weighted-shortest-job-first-wsjf-2)
as relative value or cost of delay divided by relative job duration.

Harness Lens adopts:

- explicit alternatives, including doing nothing;
- named criteria and weights that total 100;
- evidence beside every rating;
- sensitivity review when uncertainty may change a result; and
- a value-per-effort signal for sequencing comparable ready work.

Harness Lens adapts these ideas by separating three concerns:

- non-negotiable architecture, safety, privacy, and publication gates;
- a normalized strategic-value score; and
- dependency-aware execution priority adjusted by evidence confidence and
  effort.

Rejected choices:

- allowing a weighted benefit to compensate for a safety or ownership failure;
- mixing safety failures into a quality average;
- treating the highest aggregate score as automatic approval;
- scoring blocked work as if it were executable;
- comparing an open-ended product with a small delivery slice; and
- changing weights after seeing results without recording a changed objective.

Assumptions and limits:

- Ratings remain judgments unless backed by product telemetry, tests, fixtures,
  incidents, or repeated user evidence.
- Linear weighted sums assume criteria and rating scales are sufficiently
  independent for the decision; overlapping criteria must be merged or their
  weights reduced.
- Execution indices are relative within one capacity pool and planning window.
- A one-point sensitivity check is a practical warning, not a probabilistic
  confidence interval.
- Mandatory security, correctness, legal, and incident work bypasses optional
  feature ranking.

No source code or proprietary worksheet was copied. Harness Lens uses an
independently written, project-specific process.
