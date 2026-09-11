<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Unified governance, graph, and observability plan

## Status and precedence

This is the canonical product and delivery plan for Harness Lens governance,
graph inspection, observability, and editor presentation. It consolidates:

- [`PLAN.md`](PLAN.md), which introduced Inspector, graph inspection, and
  Patternizer boundaries; and
- [`../harness-observability-compass/`](../harness-observability-compass/),
  which records the evidence model, implementation order, runtime modes, and
  recovered delivery state.

The source plans remain as design history and operational checkpoints. When a
product or architecture statement conflicts with this file, this file wins.
Historical release status, accepted revisions, and unfinished work continue to
live in the compass `state/` files and must be reverified before use.

This plan adds no implementation to the composition hub. Work starts in its
lowest owning repository, is verified there, moves through immutable dependency
pins, and updates this hub last.

Use the [decision value and execution matrix](DECISION-MATRIX.md) to admit,
compare, defer, reject, and sequence new proposals. Hard architecture, safety,
privacy, and publication gates apply before weighted scoring.
Use the [reusable module and package horizon](MODULARIZATION-HORIZON.md) for
library boundaries, registry release order, conformance testing, and criteria
for future terminal, GUI, storage, plugin, and language packages.

## Consolidated decisions

| Concern | Canonical decision |
| --- | --- |
| Main command | Keep `harness-lens scan`. Add graph options to it; do not add a public `inspect` command or package. |
| Static graph | Produce a deterministic, bounded, portable JSON relationship graph from completed scan data. |
| Graph traversal | Support an explicit root and maximum hop count. A traversed static path is a possible relationship path, not proof of agent behavior. |
| Runtime flow | Build flow data only from sanitized, ordered runtime observations. Aggregation is deterministic; capture completeness remains explicit. |
| Visualization | Use a tree for hierarchy, a relationship graph for cross-links and cycles, and a Sankey/alluvial view for measured directed flow. |
| Inspector | Keep a separate Rust action-gate boundary when its contract is accepted. It orchestrates Core and SDK behavior; it is not a second rules engine. |
| Patternizer | Keep accepted pattern contracts, deterministic compilation, and enforcement in Core. Provider-backed or open-ended candidate extraction stays outside Core. |
| TypeScript and Python | Keep them at compatibility, embedding, and presentation boundaries. New enforcement behavior has one Rust reference implementation. |
| Providers | OpenAI, Codex, and other providers remain optional adapters outside Core and cannot rewrite deterministic findings, scores, or deny decisions. |
| Storage | JSON first. Consider SQLite only after profiling demonstrates a local query or history need. Do not add a hosted or external graph database without measured evidence. |
| Desktop | Keep a separate local-first desktop application boundary for Tauri lifecycle, permissions, installers, signing, updates, and presentation. It consumes completed neutral reports and does not become another scanner or rules engine. |
| Distribution | Publish usable artifacts only. Do not create empty registry placeholders or duplicate installers for internal libraries. |

## Product model

Harness Lens must keep three claims distinct.

### 1. Static relationship

A deterministic scan may establish relationships such as:

- a harness source contains a rule;
- a scope applies to a source or rule;
- a source references another bounded workspace file;
- a rule overrides, duplicates, or conflicts with another rule;
- a skill declares a tool or references supporting material;
- a finding was emitted by a stable rule for safe evidence at a known location.

These relationships describe the harness as configured. They do not show that
an agent read, followed, or was influenced by a rule.

### 2. Potential path

Deterministic traversal may find a path through static relationships. The
result must be labeled `potential` or `static`; it means only that the graph
contains that route. Traversal order, root selection, tie breaking, limits, and
truncation are deterministic and visible.

### 3. Observed flow

An observed flow requires ordered runtime evidence. It may show that a source
was included, a skill was activated, a tool was requested, and a tool completed.
It still must not claim that a particular instruction caused an action unless a
separate attribution method and sufficient evidence support that statement.

## CLI contract

The CLI retains one analysis command:

```text
harness-lens scan [PATH]
harness-lens scan [PATH] --graph --json
harness-lens scan [PATH] --graph --graph-root AGENTS.md --graph-max-hops 4 --json
```

Rules:

- `scan` without graph options preserves current deterministic report behavior.
- `--graph` requests the versioned graph projection of the same scan; it does
  not invoke a second engine.
- `--graph-root NODE_OR_PATH` selects a stable starting entity for traversal.
- `--graph-max-hops N` limits relationship hops. It is deliberately not named
  `--depth`, which could be confused with filesystem discovery depth.
- Existing `--json` compatibility remains. A future CLI-wide `--format` option
  may be added only as one migration applied consistently to all commands.
- Large graph output uses an explicit output file, bounds, or pagination rather
  than silently truncating stdout.
- Exit behavior depends on scan completeness and findings, not on whether a
  visualization was requested.

The TypeScript and Rust CLI surfaces must converge on the same command grammar,
exit-code matrix, fixtures, and report schema before the new graph surface is
declared stable.

## Neutral graph contract

Core owns provider-neutral graph value types and deterministic graph
derivations. SDK supplies bounded filesystem/config inputs and optional runtime
observations. CLI and LSP serialize or transport completed data. Editors render
it without recreating analysis.

A graph envelope must include at least:

```text
schema_version
graph_kind             relationship | observed_flow
method                 deterministic | heuristic | statistical | probabilistic
root_identity
profile_revision
input_revision_or_digest
completeness
limits_and_truncation
nodes[]
edges[]
```

Each node has a stable ID, kind, safe label, optional relative path, and safe
source span. Each edge has stable source and target IDs, a typed relation,
method, safe evidence references, and optional measurement fields. Arrays use a
documented stable ordering so equal inputs produce byte-equivalent logical
output after canonical serialization.

Measured edges may additionally contain:

```text
value
unit                    transitions | calls | sessions | duration | cost
denominator
share                   normalized to [0.0, 1.0]
sample_size
observation_window
```

Static edges do not receive invented frequency or percentage values. A repeated
static reference may expose a deterministic occurrence count only when its unit
is explicitly `reference_occurrences` and the UI does not present it as agent
usage.

Graph output never serializes raw source, prompts, reasoning, provider output,
tool arguments, tool output, transcripts, stderr, credentials, or secrets.

## Runtime observation contract

Use precise event names instead of treating every relationship as a call.
Initial safe event categories are:

```text
discovered
loaded
resolved
applicable
injected
referenced
skill_activated
tool_requested
tool_executed
finding_emitted
```

Only actual tool or function invocation uses call terminology. An injected or
applicable rule is not proof that a model used it.

Store only bounded, sanitized observations: stable asset/tool/category identity,
status, duration, retry count, attributable cost when available, stable error
class, model identity, skill/config identity, revision, sequence position,
session/run pseudonymous identity, and time window. Raw arguments and outputs
remain excluded.

Runtime modes remain:

| Mode | Behavior | Default |
| --- | --- | --- |
| `off` | deterministic static report only; no capture process | yes |
| `live` | opt-in bounded capture at startup and explicit refresh | no |
| `snapshot` | read a canonical safe snapshot; no process launch | no |

Missing runtime evidence yields an explicit unavailable or insufficient-evidence
state. It never fabricates history and never changes deterministic findings or
scores.

## Visualization contract

Editors and the future desktop viewer provide three projections of the neutral
graph.

### Hierarchy tree

Show workspace assets, scopes, skills, references, findings, context
consumption, and runtime-history availability. It remains useful when runtime is
off. Safe provenance entries navigate to exact source locations.

### Relationship map

Show files, rules, scopes, skills, references, findings, and their typed edges.
This view supports cycles, filtering, root selection, bounded expansion, and
shortest/static path explanation. Edge styling distinguishes extracted,
heuristic, statistical, probabilistic, missing, and ambiguous relationships.

### Observed-flow Sankey

Use link width for exactly one selected unit at a time. The default may be
observed transitions; alternative views may use calls, distinct sessions,
duration, or attributable cost. Tooltips show the raw value, denominator,
share, sample size, window, method, and completeness.

Sankey layout must not hide cycles. Repeated logical entities are duplicated by
sequence layer, such as `tool@step2` and `tool@step4`, while retaining their
shared logical node ID. A relationship graph, not Sankey, remains the primary
view for arbitrary cyclic static data.

Every graphical view has a bounded textual/table equivalent, keyboard
navigation, editor-theme integration, and safe empty/loading/partial/error
states. Renderer assets are bundled locally; editor use never depends on a CDN
or hosted graph service.

## Editor delivery

```text
Core graph/report contracts
  -> SDK inputs and optional observations
    -> language-server bounded workspace graph request
      -> shared presentation data model
        -> VS Code webview and tree
        -> Cursor through the same reviewed VSIX/Open VSX identity
        -> Visual Studio tool window hosting the same neutral data contract
```

The language server owns request bounds, pagination, workspace-root selection,
unsaved overlays, stale-state handling, and UTF-8-to-UTF-16 conversion. It does
not own chart layout.

Begin the web renderer as an internal, framework-neutral package in the existing
VS Code workspace. Do not publish a separate renderer package until a second
consumer proves an independent release contract. Visual Studio may consume a
reviewed bundled web artifact or implement an equivalent native view against the
same protocol; it must not copy Core rule logic. Cursor does not receive a
separate analysis implementation.

Choose a graph library only after a focused proof compares package size,
deterministic layout, cycle handling, SVG/canvas behavior, accessibility,
theming, offline packaging, and performance at documented node/edge limits.
Record the adopted or rejected library and algorithm in `docs/prior-art/`.

## Desktop delivery

[`harness-lens-desktop`](https://github.com/harness-lens/harness-lens-desktop)
is an independent application boundary because desktop permissions, lifecycle,
installers, signing, and updates differ from editor and terminal packaging. It
is a presentation and distribution leaf, not another analysis layer:

```text
completed report/graph JSON -> bounded local validation -> Tauri presentation
```

The first useful package is an offline report viewer. It opens a user-selected
report or a path supplied when the executable launches, validates schema and
completeness, and renders summaries and accessible tables without a network
dependency. Hierarchy, relationship, and observed-flow views consume the same
neutral contracts used by the editors as those contracts become available.

The canonical terminal surfaces remain `harness-lens scan` and the future
`harness-lens tui`. Launching the desktop executable with a report path is a
convenience, not a duplicate scanner or CLI engine. Start without a shell or
scanner sidecar. If later measurements justify a bundled official Harness Lens
binary, Tauri capabilities must allow only that executable, fixed commands,
validated arguments, explicit filesystem scopes, and required windows. Remote
UI, CDN assets, arbitrary shell access, and silent process launch remain out of
scope.

Desktop releases use signed native installers, checksums, provenance, and
GitHub releases or applicable application stores. Do not publish an npm package
merely to represent the desktop product; the controlled npm scope already
protects a future internal JavaScript package if a real reusable contract
emerges.

## Inspector boundary

Inspector is a guard around supported action boundaries, initially limited to
recognized harness/config changes rather than arbitrary shell, network, or tool
authorization.

Initial modes remain:

1. preflight validation of the current harness and policy profile;
2. proposal validation for a bounded normalized change set;
3. post-action re-scan against the same bound scope;
4. audit-only evidence without blocking.

Core remains the only reference rules engine. It owns rule/profile/finding and
safe evidence contracts. SDK owns discovery, loading, config, and overlays.
Inspector owns change normalization, baseline binding, gate mode, enforcement
decision, and time-of-check/time-of-use protection.

Do not combine enforcement with evaluation completeness:

```text
outcome                 allow | warn | deny
evaluation_status       complete | incomplete
```

An incomplete scan carries safe reasons and follows an explicit profile policy;
it can never become an accidental allow. Optional model explanation failure does
not alter a deterministic outcome. A provider-required future check must define
fail-open or fail-closed behavior explicitly and cannot default silently.

Reproducibility requires the normalized input/change digest, baseline identity,
policy revision, analyzer version, schema version, completeness, and stable
evidence references.

## Patternizer boundary

Pattern work has two stages:

1. candidate extraction, which may be deterministic, heuristic, statistical, or
   probabilistic and must expose method, evidence, assumptions, bounds, and
   provenance; and
2. accepted pattern compilation and enforcement, which is deterministic,
   versioned, fixture-backed, and owned by Core.

Provider-backed candidate extraction lives in an optional SDK or dedicated
adapter. A human or separately defined approval workflow accepts a candidate
before it becomes a Core rule. Provider output never becomes an enforceable rule
automatically.

Keep Patternizer as a Core design/module name until real fixtures and independent
release needs justify another repository or public package.

## Provider boundary

OpenAI, Codex, and other providers consume completed content-safe reports or
bounded candidate inputs through optional adapters. They may explain findings,
group candidates, or propose profile changes in a separate namespace. They may
not mutate findings, scores, graph facts, policy revisions, or deny decisions.

Provider failures are observable adapter execution results. They are neither
silent passes nor process-wide crashes. No moving `latest` model identifier is a
build, test, graph, or policy dependency.

## Repository ownership

| Change | Owning repository | Downstream work |
| --- | --- | --- |
| Graph nodes/edges, methods, completeness, trace records, deterministic aggregation | Core | SDK pin, CLI/LSP pins, editor fixtures |
| Filesystem/reference discovery, config resolution, overlays, runtime adapter normalization | SDK | CLI and LSP pins |
| `scan --graph`, graph output, exit codes | CLI | distribution smoke and docs |
| Bounded workspace graph request, pagination, runtime snapshot transport, UTF conversion | Language server | VS Code and Visual Studio clients |
| Tree, relationship map, Sankey renderer, source navigation, VSIX/Open VSX compatibility | VS Code repository | Cursor compatibility verification |
| Visual Studio tool window and bundled-renderer/native-view verification | Visual Studio repository | VSIX/package audit |
| Offline desktop report/graph viewer, Tauri capabilities, installers, signing, and updates | Desktop repository | Consume accepted report/graph contracts; no engine duplication |
| Gate request/decision and supported action-boundary orchestration | Inspector repository after contract acceptance | CLI `guard` UX only when implemented |
| Accepted deterministic pattern rules | Core | SDK/CLI/LSP/editor generic consumption |
| Provider-backed candidate extraction | Optional SDK/adapter owner | separate suggestion output only |
| Architecture, prior art, compatible pins | Hub | update last |

Dependencies remain inward. If the Inspector repository is added, its relevant
path is `core <- sdk <- inspector <- CLI guard`; ordinary scan and graph paths
remain `core <- sdk <- CLI/LSP <- editor`. Desktop remains a leaf over completed
portable report/graph output and never becomes an inward dependency.

## Delivery order

Historical release and checkpoint work remains governed by the observability
compass state files. New work follows these contract slices:

1. **Baseline audit** — recheck repository/worktree/remote state and preserve
   unrelated dirty work.
2. **Core graph contract** — add versioned graph, method, completeness, static
   path, and sanitized observation records with deterministic tests.
3. **SDK graph inputs** — add bounded reference/scope inputs and optional safe
   runtime-observation normalization.
4. **CLI graph surface** — add `scan --graph`, root/hop bounds, JSON fixtures,
   compatible exit behavior, and large-output handling.
5. **LSP graph transport** — expose bounded/paginated workspace graph data and
   test multi-root, overlays, positions, stale data, and unavailable runtime.
6. **Static editor experience** — deliver hierarchy tree and relationship map
   with navigation before runtime flow.
7. **Offline desktop viewer** — open, bound, validate, and explain completed
   local reports with accessible tables before adding a sidecar or graph UI.
8. **Runtime modes** — preserve `off` default; verify opt-in `live` and safe
   `snapshot` behavior independently.
9. **Tool/action trace history** — add sanitized ordered observations, stable
   error grouping, limits, pagination, and explicit sample/window fields.
10. **Observed Sankey** — aggregate one declared unit at a time, handle repeated
   nodes by sequence layer, and expose table/accessibility fallbacks.
11. **Attributed effectiveness** — compare identified revisions/windows only
    when method and sample requirements are met; otherwise report insufficient
    evidence.
12. **Inspector contract and MVP** — define Core policy primitives, then build
    preflight/proposal/post-action orchestration without a second engine.
13. **Pattern candidates** — begin only after representative fixtures exist;
    compile accepted patterns into Core rules with provenance.
14. **Verification and composition** — verify each owner in dependency order,
    update immutable downstream revisions, then update hub gitlinks and
    `docs/repository-split.md` last.

## Issue routing

Create implementation issues only after checking for duplicates and confirming
the current accepted dependency revisions.

| Repository | Initial issue scope |
| --- | --- |
| Core | Versioned static graph and sanitized action-trace contracts |
| SDK | Bounded graph-input and runtime-observation adapters |
| CLI | Add deterministic `scan --graph` output and traversal bounds |
| Language server | Expose bounded workspace graph data over the existing protocol boundary |
| VS Code | Add accessible relationship-map and observed-flow views |
| Visual Studio | Add a graph tool window against the neutral LSP contract |
| Desktop | Build the bounded offline report-viewer MVP before graph views or a sidecar |
| Hub | Track dependency-ordered delivery, prior art, and final composition only if a cross-repository umbrella issue is useful |

Issue bodies describe functionality, evidence, ownership, dependencies, and
acceptance gates. They do not discuss namespace reservation or speculative
future product names.

### Backlog created from this plan

Created on 2026-09-10 after checking the open backlog for overlap:

| Owner | Issue | Role |
| --- | --- | --- |
| Core | [#23 — relationship-graph contract](https://github.com/harness-lens/core/issues/23) | Neutral graph envelope, identity, evidence, bounds, metrics, and graph-kind separation |
| Core | [#24 — policy-decision and completeness contracts](https://github.com/harness-lens/core/issues/24) | Deterministic policy outcomes and evidence semantics |
| Core | [#25 — pattern candidate and accepted-rule contracts](https://github.com/harness-lens/core/issues/25) | Candidate/review/compilation boundary, gated on representative fixtures |
| SDK | [#31 — normalize safe action traces](https://github.com/harness-lens/sdk/issues/31) | Optional adapters and observed-transition aggregation |
| CLI | [#49 — bounded graph output on `scan`](https://github.com/harness-lens/cli/issues/49) | `scan --graph`, root/hop bounds, human and JSON output |
| Language server | [#31 — observed-flow snapshots](https://github.com/harness-lens/language-server/issues/31) | Bounded provider-neutral runtime-flow transport |
| VS Code/Cursor | [#33 — accessible measured Sankey view](https://github.com/harness-lens/harness-lens-vscode/issues/33) | Observed flow only; local assets and tabular fallback |
| Visual Studio | [#3 — relationship and flow views](https://github.com/harness-lens/harness-lens-visualstudio/issues/3) | Staged static and observed tool-window delivery |
| Desktop | [#1 — offline report-viewer MVP](https://github.com/harness-lens/harness-lens-desktop/issues/1) | Local Tauri presentation with no scanner or shell sidecar in the first milestone |
| Inspector | [#1 — deterministic policy gate MVP](https://github.com/harness-lens/harness-inspector/issues/1) | Thin orchestration over the Core policy contract |

Existing issues were retained rather than duplicated:

- [Core #5](https://github.com/harness-lens/core/issues/5) remains the ordered,
  content-safe action-trace schema.
- [SDK #17](https://github.com/harness-lens/sdk/issues/17) remains the bounded
  static inclusion-graph discovery work.
- [Language server #14](https://github.com/harness-lens/language-server/issues/14)
  remains the static workspace-report transport.
- [VS Code #12](https://github.com/harness-lens/harness-lens-vscode/issues/12)
  remains the tree and static relationship experience.
- [Hub #11](https://github.com/harness-lens/harness-lens/issues/11) remains the
  cross-repository observability umbrella.

Each retained issue received a planning note that states its graph kind and
links the new dependent work.

### Repository reservation state

[`harness-lens/harness-inspector`](https://github.com/harness-lens/harness-inspector)
was reserved on 2026-09-10 as the accepted independent policy-gate boundary.
[`harness-lens/harness-lens-desktop`](https://github.com/harness-lens/harness-lens-desktop)
was reserved on the same date as the local desktop presentation and
distribution boundary.
No empty crate, npm package, PyPI project, or other registry placeholder was
published. Promotion readiness differs by repository and is tracked in
[`ASSET-RESERVATIONS.md`](ASSET-RESERVATIONS.md); a reserved name alone never
implies release readiness.

The complete repository, package-name, and deliberately-not-reserved inventory
is maintained in [`ASSET-RESERVATIONS.md`](ASSET-RESERVATIONS.md).

## Acceptance gates

- Equal normalized inputs, config/profile revision, and implementation revision
  produce logically equal static graphs with stable ordering.
- Static paths are labeled potential/static and never presented as observed
  agent behavior.
- Frequency, percentage, and Sankey width appear only with a declared unit,
  denominator, observation window, sample size, method, and completeness.
- Missing or partial runtime data remains visible and cannot alter deterministic
  findings or scores.
- Cycles, missing references, ignored paths, out-of-root references, bounds, and
  truncation remain observable.
- Graph and gate reports contain no raw source, prompt, reasoning, provider
  response, tool argument/output, transcript, stderr, credential, or secret.
- Editor views work with runtime off and provide an accessible non-graphical
  representation.
- The desktop viewer works offline against completed reports, starts without a
  shell/scanner sidecar, and grants only explicit Tauri capabilities.
- Provider outage or refusal cannot become an accidental allow.
- Accepted rules carry stable IDs, fixtures, evidence, method, assumptions, and
  false-positive boundaries.
- Every adopted/rejected external graph, layout, or attribution algorithm has a
  prior-art record.
- Core, SDK, CLI, language server, and editor repositories pass their documented
  checks in dependency order before hub pins change.

## Immediate next actions

1. Apply [`DECISION-MATRIX.md`](DECISION-MATRIX.md) to new or materially changed
   initiatives and record evidence for every non-zero rating.
2. Retain this consolidation as the product-decision authority and keep the
   separate public-name retirement record for `harness-inspect`.
3. Preserve the completed Rust `0.0.2` Core/SDK dependency train and
   `harness-lens-lsp 0.0.1`; require clean registry-only package verification
   for every later release.
4. Promote the verified Go report facade from immutable commit `e4c5f9b` only
   after approval for public repository visibility, then tag `v0.1.0` and
   verify retrieval through the public Go module path.
5. Make the restored `harness-metrics` source repository public only after
   approval, add a trusted backup crates.io owner/team and trusted publisher,
   and do not invent a retroactive `v0.0.4` tag or republish that immutable
   crates.io version.
6. Start implementation with the Core graph/trace contract after current
   accepted revisions and active worktrees are reverified.
7. Land Core policy decision/completeness contracts before publishing or
   implementing the Inspector gate package.
8. Build the desktop offline report viewer before graph views or any sidecar;
   add each later capability only with an explicit threat-model update.
9. Publish new packages only after their owning repository has useful behavior,
   fixtures, verification, provenance, and a protected release path.
