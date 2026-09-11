<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Harness observability compass

> **Operational-history status:** product and architecture decisions are
> consolidated in the
> [unified governance, graph, and observability plan](../harness-governance-suite/UNIFIED-PLAN.md).
> This compass remains authoritative for recovered state, checkpoints, and the
> next bounded implementation slice. Where product wording conflicts, the
> unified plan wins.

Local working compass for the GitLens-like Harness Lens observability center.
It preserves recovered direction in small files so each work session can load
one bounded slice instead of the whole project.

## Loading rule

Read this file first, then read only the next file named in `state/now.md`.
Open another step file when that step becomes active. Do not load every file by
default. Update `state/now.md` after each meaningful checkpoint. Keep this
folder local until the plan is stable; no commit is part of this compass.

## Compass files

| File | Purpose |
| --- | --- |
| [00-north-star](00-north-star.md) | User outcome and product boundary |
| [01-system-tree](01-system-tree.md) | Component hierarchy and data flow |
| [02-recovered-state](02-recovered-state.md) | Codex-second recovery and known state |
| [03-rules](03-rules.md) | Invariants every slice must preserve |
| [04-step-order](04-step-order.md) | Small ordered delivery path |
| [state/now](state/now.md) | Current checkpoint and next file to load |
| [steps](steps/) | One bounded file per implementation slice |

## Working loop

1. Read `state/now.md`.
2. Read the named step file only.
3. Inspect owning repository and its dirty state.
4. Make smallest safe change in that repository.
5. Run that step's checks and record evidence.
6. Update `state/now.md`; stop before next slice unless explicitly asked.

The hub composes repositories. Implementation belongs in owning module
repository, then moves inward through immutable pins after review.
