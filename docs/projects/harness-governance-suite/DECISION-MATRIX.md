<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Decision value and execution matrix

## Purpose

Use this matrix to compare proposed product, architecture, package, and delivery
work. It makes value judgments explicit, removes low-value work, and sequences
accepted work without treating one weighted sum as automatic authority.

The process has four stages:

1. reject or redesign proposals that fail a non-negotiable gate;
2. calculate strategic value for proposals that pass;
3. block accepted work whose dependency, evidence, or approval is missing; and
4. sequence ready work by value, confidence, effort, and dependency order.

Security fixes, correctness regressions, release incidents, and legal duties use
a mandatory lane. They do not compete with optional product work for a score.

## Non-negotiable gates

| Gate | Pass condition | Failure result |
| --- | --- | --- |
| Ownership | One repository owns the behavior at the lowest valid layer | Move it to the correct owner or reject the split |
| No duplication | The proposal does not create a second rules, scan, graph, provider, or presentation engine | Merge into the existing owner or reject it |
| Technical trust | Determinism, method labels, completeness, score bounds, and safe failure behavior remain intact | Redesign before scoring |
| Data safety | Reports and APIs exclude raw source, prompts, transcripts, tool arguments/output, credentials, and secrets | Reject until the boundary is content-safe |
| Legitimate artifact | A public package or repository has useful behavior and is not a name-only placeholder | Keep private, defer, or reject publication |

A failed gate cannot be compensated by a high value rating. Record the failed
gate and rationale instead of assigning a strategic score.

## Strategic value

Rate each criterion from 0 to 5 using evidence recorded beside the decision.
Weights total 100.

| Criterion | Weight | Rating question |
| --- | ---: | --- |
| Developer value | 20 | How much real developer pain, time, or uncertainty does this remove? |
| Trust and safety | 20 | How much does this improve correctness, evidence, privacy, or safe failure? |
| Architecture fit | 15 | How cleanly does this respect ownership and inward dependencies? |
| Dependency unlock | 15 | How much accepted downstream work becomes possible? |
| Reuse and reach | 10 | How many real consumers or platforms share the result? |
| Evidence and readiness | 10 | How strong are fixtures, demand, contracts, tests, and implementation evidence? |
| Time criticality | 10 | What concrete cost, breakage, or lost opportunity comes from delay? |

```text
strategic_value = sum(weight * rating / 5)
```

The result is normalized to `[0, 100]`:

| Value | Default decision |
| ---: | --- |
| 85–100 | Critical investment; reserve capacity |
| 70–84 | Adopt and schedule when gates are ready |
| 55–69 | Validate with a bounded experiment or thin slice |
| 40–54 | Defer until evidence or dependencies change |
| 0–39 | Reject unless mandatory evidence appears |

Thresholds guide review. They do not override a failed gate, a dependency, or a
mandatory incident.

## Execution priority

Assign confidence from the evidence behind the ratings:

| Confidence | Meaning |
| ---: | --- |
| `1.00` | Implemented or directly measured |
| `0.75` | Strong contract, fixtures, or repeated user evidence |
| `0.50` | Plausible design with partial evidence |
| `0.25` | Speculative or based mainly on analogy |

Estimate one independently reviewable delivery slice with effort points
`1, 2, 3, 5, 8, 13`. Do not score an entire open-ended product against a small
package change.

```text
execution_index = strategic_value * confidence / effort_points
```

Calculate the execution index only when required contracts, external approval,
and owner capacity are available. First respect dependency order, then compare
the index among ready slices competing for the same capacity. Higher means more
evidence-adjusted value per unit of effort. Strategic value remains visible so
large foundational work is not starved by many small tasks.

## Required decision record

Each scored proposal records:

```text
decision_id
title
owner_repository
decision_date
scorer
alternatives_including_do_nothing
gate_results_and_evidence
criterion_ratings_and_evidence
strategic_value
confidence
effort_points
required_dependencies
required_approval
execution_index_if_ready
decision                    adopt | experiment | defer | reject | mandatory
review_trigger
```

Re-score when evidence, scope, dependencies, effort, or user need changes. If a
one-point change in any uncertain rating changes the decision band or order,
mark the result unstable and gather evidence before committing major effort.

## Current baseline

Ratings use criterion order `developer/trust/architecture/unlock/reuse/evidence/time`.
They are provisional planning judgments, not product telemetry.
This table is a triage summary; create the full decision record above before a
score authorizes material implementation or publication.

| Initiative or slice | Ratings | Value | Confidence | Effort | Scheduling state | Decision |
| --- | --- | ---: | ---: | ---: | --- | --- |
| Repair and publish compatible Rust Core/SDK/LSP dependency train | Mandatory release blocker | — | 1.00 | 5 | Completed 2026-09-11 through merged owner PRs and clean registry-only package verification | Complete |
| Publish implemented `harness-lens-config`, Harness Score adapter, and `harness-lens-lsp` names | `5/5/5/5/5/5/5` | 100 | 1.00 | 2 | Published 2026-09-11 as useful packages after the dependency train | Complete |
| Publish verified Go report facade `e4c5f9b` and tag `v0.1.0` | `4/4/5/3/4/5/5` | 84 | 1.00 | 1 | Blocked only on public-visibility approval; index 84 when unblocked | Adopt now |
| Secure `harness-metrics` continuity and make restored source `07445e2` public, without a retroactive tag | `3/5/5/3/3/5/5` | 82 | 1.00 | 2 | Blocked on public-visibility approval and selection of a trusted backup owner/team; index 41 when unblocked | Adopt now |
| Core graph-contract slice | `5/5/5/5/5/3/5` | 96 | 0.75 | 5 | Ready after baseline/worktree audit; index 14.4 | Critical next foundation |
| Core policy-decision/completeness slice | `4/5/5/3/4/3/3` | 80 | 0.75 | 3 | Ready after baseline/worktree audit; index 20.0 | Adopt; unlock Inspector |
| Cross-language conformance bundle and black-box runner | `5/5/5/5/5/2/4` | 92 | 0.50 | 8 | Begin fixtures in Core; split only after two consumers; provisional index 5.75 | Critical testing investment, thin slice first |
| Internal CLI terminal/TUI library target | `4/4/5/3/3/3/2` | 72 | 0.75 | 3 | Ready as internal extraction; index 18.0 | Adopt internally; no public package yet |
| Shared internal GUI renderer | `4/4/4/4/4/2/2` | 72 | 0.50 | 5 | Wait for second implemented GUI consumer; no index | Retain internal package boundary |
| Versioned out-of-process plugin protocol | `4/5/4/4/5/1/1` | 74 | 0.25 | 8 | Blocked by external plugin and trust/crash model; no index | Defer package publication |
| Desktop offline report-viewer proof | `4/4/4/2/3/2/2` | 64 | 0.50 | 8 | Ready only as bounded viewer slice; index 4.0 | Experiment before full app |
| Inspector MVP | `4/5/4/3/3/1/2` | 69 | 0.50 | 8 | Blocked by Core policy contract; no index | Retain, do not execute yet |
| Observed Sankey | `4/4/4/1/3/1/1` | 57 | 0.50 | 5 | Blocked by graph and runtime-observation evidence; no index | Validate after real flow data |
| Pattern candidate extraction | `3/3/3/2/3/1/1` | 49 | 0.50 | 5 | Blocked by representative fixtures; no index | Defer |
| C/C++ facade | `2/3/3/1/2/1/1` | 40 | 0.25 | 8 | Blocked by stable ABI and demonstrated consumer demand; no index | Defer |
| Empty npm, PyPI, or crates.io placeholders | Gate failure | — | — | — | No legitimate artifact | Reject |

This baseline yields three practical queues:

1. complete Go visibility and `harness-metrics` ownership-continuity/public-source
   actions after explicit approval and selection of a trusted backup owner;
2. invest in Core graph, Core policy, and thin conformance contracts while
   extracting terminal/TUI code only as an internal CLI library; and
3. gather evidence for Desktop, shared GUI rendering, plugins, and Sankey while
   keeping Patternizer, C/C++, WASM, and registry placeholders out of active
   package publication.

Review weights quarterly or when the mission changes. Never change weights only
to make a preferred proposal win; record a changed project objective first.
