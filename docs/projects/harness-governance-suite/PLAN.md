<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Harness governance suite

> **Planning-history status:** the canonical product decisions are now in the
> [unified governance, graph, and observability plan](UNIFIED-PLAN.md). This file
> is retained as an input record. Where the two conflict, the unified plan wins;
> in particular, public `harness-inspect` command/package proposals are retired
> by the [minimal retirement plan](HARNESS-INSPECT.md).

Planning baseline for three proposed names:

- `harness-inspector`: enforce policy around proposed harness changes and
  agent/tool actions.
- `harness-inspect`: deeply inspect a harness and produce evidence, graph data,
  and a report.
- `harness-patternizer`: extract, describe, and validate reusable harness
  patterns.

This document is a design and reservation plan. It does not add implementation
to the composition hub. Implementation belongs in the owning repository first,
then this hub receives an immutable submodule pin.

## Decisions

| Name | First home | Language | Decision |
| --- | --- | --- | --- |
| `harness-inspector` | New `harness-lens/harness-inspector` repository | Rust reference engine; existing SDK/CLI adapters | Reserve the GitHub repository. Build only after the policy contract is accepted. |
| `harness-inspect` | `harness-lens/cli` as `harness-lens inspect` | Rust through the existing CLI | Do not create a second implementation repository. Add an unscoped launcher package only if `npx harness-inspect` proves useful. |
| `harness-patternizer` | `harness-lens/core` pattern/rule modules | Rust reference rules; TypeScript compatibility | Do not make it an AI service or a second rules engine. Create a standalone repository only after the pattern DSL has independent release needs. |

The three names are related, but they are not one product boundary:

```text
core:       patterns, rule contracts, findings, graph/value types
sdk:        safe discovery, adapters, provider transport boundaries
cli:        inspect, guard, report, and exit-code UX
inspector:  proposed-change/action gate built on core + SDK
editors:    diagnostics and review UX through the existing LSP contract
```

## Product boundaries

### `harness-inspector`

This is a guard around an action boundary. It must inspect a proposed diff,
tool request, or completed action and return a policy decision plus bounded
evidence. It must not claim to read a model's private reasoning or trust a
model's assertion that a rule was followed.

Initial modes:

1. **Preflight** — validate the current harness and policy profile.
2. **Proposal gate** — validate a proposed file/diff before it is applied.
3. **Post-action check** — re-scan after an action and report violations.
4. **Audit-only** — record evidence without blocking.

The deterministic result is authoritative. Optional model assistance may explain
findings or suggest a profile change, but cannot rewrite findings, scores, or a
deny decision.

### `harness-inspect`

This is primarily a command and report format, not a new engine. Start with:

```text
harness-lens inspect [PATH] --profile default --format json
harness-lens inspect [PATH] --graph --format json
```

The graph is an evidence index: nodes represent files, scopes, rules,
references, findings, and observed relationships. Begin with an in-memory or
portable JSON graph. Do not add Neo4j/FalkorDB or a hosted service until a
measured workload requires it.

### `harness-patternizer`

Patternizer extracts candidate patterns and compiles accepted patterns into
versioned deterministic rules. Candidate extraction may be heuristic or
probabilistic; enforcement must be deterministic and carry evidence,
assumptions, method, and false-positive boundaries.

OpenAI/Codex and other providers belong behind an optional SDK/inspector
adapter. Never import a provider SDK into Core, and never make a moving
“latest model” a build or policy dependency.

## Language and repository choice

| Concern | Choice | Reason |
| --- | --- | --- |
| Rule/policy evaluation | Rust in Core and the new Inspector engine | deterministic, portable, safe failure boundaries, shared with native CLI/LSP |
| Filesystem/config/provider adapters | existing SDK | already owns discovery, TOML, Python/PyO3, and provider boundaries |
| Terminal command and npm launcher | existing CLI | one release identity and one `harness-lens` binary |
| Editor presentation | existing language-server and editor repositories | avoids duplicate rule logic |
| Python | SDK facade and experiments only | useful for notebooks/integration, not the enforcement path |
| TypeScript | compatibility APIs, npm launchers, editor UX | useful at boundaries, not a second policy engine |
| Graph storage | JSON first; SQLite only after profiling | portable evidence and simple reproducibility |

## Proposed `harness-inspector` repository baseline

Create this in `harness-lens/harness-inspector` only after the reservation
decision is accepted:

```text
harness-inspector/
├── README.md
├── LICENSE
├── LICENSING.md
├── SECURITY.md
├── CONTRIBUTING.md
├── COPYRIGHT
├── docs/
│   ├── architecture.md
│   ├── policy-contract.md
│   ├── provider-boundary.md
│   └── threat-model.md
├── fixtures/
│   ├── valid/
│   ├── rejected/
│   └── malformed/
└── rust/
    ├── Cargo.toml
    └── crates/
        └── harness-inspector/
            ├── Cargo.toml
            └── src/
                ├── lib.rs
                ├── policy.rs
                ├── proposal.rs
                ├── decision.rs
                ├── evidence.rs
                └── adapters.rs
```

The initial public contract should be small:

```rust
pub struct InspectionRequest {
    pub scope: Scope,
    pub profile: ProfileId,
    pub proposed_changes: Vec<Change>,
    pub mode: InspectionMode,
}

pub struct InspectionDecision {
    pub outcome: Outcome, // Allow, Warn, Deny, NotEvaluated
    pub findings: Vec<Finding>,
    pub evidence: Vec<EvidenceRef>,
    pub policy_revision: String,
}
```

No raw prompts, source dumps, credentials, or provider responses belong in
`InspectionDecision`.

## Delivery order

1. **Reserve and document** — reserve the GitHub repository name; record the
   package-name matrix; do not publish empty registry artifacts.
2. **Core contract** — add stable rule/profile/graph contracts in Core, with
   deterministic tests and rule IDs.
3. **Inspect command** — add `harness-lens inspect` to the existing CLI and
   produce JSON findings plus the first portable graph format.
4. **Inspector MVP** — implement preflight, proposal, and post-action modes in
   the new Rust repository; pin Core and SDK by immutable revisions.
5. **Adapter integration** — add optional OpenAI/Codex/provider adapters in SDK
   or Inspector. Provider failures become `NotEvaluated` evidence, not silent
   passes and not process-wide crashes.
6. **Editor integration** — expose diagnostics through the existing LSP; do not
   copy rules into VS Code, Visual Studio, Cursor, or Antigravity clients.
7. **Patternizer** — add candidate extraction only after real fixtures exist;
   compile accepted patterns into Core rules and publish the provenance.
8. **Distribution** — package the CLI once. Add Homebrew, Scoop, WinGet,
   Chocolatey, Snap, and other installers only for released binaries, not for
   every internal library.

## Reservation matrix

| Surface | `harness-inspector` | `harness-inspect` | `harness-patternizer` |
| --- | --- | --- | --- |
| GitHub repository | Reserve now | Hold as CLI command; no repository yet | Hold; Core owns first implementation |
| npm | Prefer `@harness-lens/inspector`; publish when usable | Optional `harness-inspect` thin launcher | Prefer `@harness-lens/patternizer` after stable API |
| crates.io | `harness-inspector` when the Rust contract is real | Reuse `harness-lens-cli`; no duplicate crate | `harness-lens-core` owns rules initially |
| PyPI | Optional `harness-inspector` facade after SDK contract | No separate package initially | No separate package initially |
| Open VSX | Not applicable | Not applicable | Not applicable; only the editor extension is published there |
| Homebrew/Scoop/WinGet/etc. | No separate formula during incubation | Ship through the `harness-lens` CLI package | No separate installer |

Registry names are not safely reserved by empty placeholder packages. Before
publishing any name, check that registry's live availability, publish only a
real version with provenance, and enable org-scoped ownership where supported.

## Acceptance gates

- Policy decisions are reproducible from the same input and profile revision.
- A provider outage or model refusal cannot become an accidental allow.
- Denials identify the rule, evidence, scope, and policy revision.
- No raw source, secrets, prompts, or provider transcripts enter reports.
- Graph output is bounded, versioned, portable, and reconstructable locally.
- Core, SDK, CLI, language-server, and editor checks pass in dependency order.
- Every external algorithm or graph design has a prior-art note in
  `docs/prior-art/`.

## Immediate next actions

1. Reserve `harness-lens/harness-inspector` on GitHub.
2. Keep `harness-inspect` as the planned `harness-lens inspect` command.
3. Keep `harness-patternizer` as a Core design name until fixtures prove a
   separate package boundary.
4. Open the Core contract issue for `InspectionRequest`, `Decision`, `Evidence`,
   graph nodes/edges, and profile revisions.
5. Implement one end-to-end fixture: a proposed harness edit that is accepted,
   one that is denied, and one that is `NotEvaluated`.
