<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CodeBurn hybrid integration project

This folder is a contributor-ready issue pack for adding runtime evidence
without weakening Harness Lens's deterministic, local-first path. Each linked
file can be copied into the owning repository's issue tracker. Keep the `CBH`
identifier in its title so cross-repository dependencies remain searchable.

## Target behavior

Static Harness Lens analysis always works. Runtime evidence has three explicit
modes:

| Mode | Behavior | CodeBurn process | Default |
| --- | --- | --- | --- |
| `off` | Publish only deterministic `HL...` findings | Never launched | Yes |
| `live` | Capture aggregate JSON at startup and explicit refresh | Launched only after opt-in | No |
| `snapshot` | Load a canonical Harness Metrics snapshot | Never launched | No |

CodeBurn owns session parsing, pricing, model efficiency, and optimization
semantics. Harness Metrics owns the aggregate schema, safe capture boundary,
evidence labels, and UTF-8 document correlation. The language server owns
lifecycle and UTF-16 protocol presentation. The editor owns consent, settings,
process startup, and user guidance.

## Dependency and attribution map

| Component | Relationship to CodeBurn | Declaration |
| --- | --- | --- |
| `harness-metrics` | Direct, optional external runtime dependency for capture | README, CLI help, integration docs, tested-version record, and third-party notice |
| `language-server` | Direct Cargo dependency on Harness Metrics; conditional external CodeBurn executable in `live` mode | Cargo version plus immutable Git SHA; protocol and runtime docs |
| `harness-lens-vscode` | Indirect client/configuration relationship | Settings and Marketplace documentation; do not bundle CodeBurn initially |
| `core`, `sdk`, and `cli` | No runtime or package relationship | Do not add CodeBurn dependencies |
| composition hub | Architecture, provenance, and tested source pins | Prior-art note, project record, and Git submodules |

CodeBurn is not a Rust or Python library dependency and must not be represented
as one. It is also not bundled with the VS Code extension in this project.
Truthful attribution is required even though the executable is installed and
run separately.

## Current baseline

Recorded on 2026-09-04:

- Harness Metrics `0.0.3` is implemented and locally verified at commit
  `157328cdfa6473df9d177b4046fe754cdcbf5862`.
- Crates.io contains earlier `0.0.1` and `0.0.2` releases. Corrected `0.0.3`
  has passed a dry run but is not published. PyPI has no release.
- The language-server working tree contains an uncommitted live-capture proof.
  It passes local tests when patched to the local Harness Metrics source, but it
  still auto-captures and lacks `off`/`snapshot` mode configuration.
- Hub architecture and prior-art documentation is drafted. Harness Metrics is
  not yet a hub submodule.
- Repository creation, pushes, and registry publication require maintainer
  authentication. Every irreversible publication also requires explicit
  maintainer approval.

The local proof is roughly 65–70% of the MVP implementation. Remaining work is
mostly safe-mode configuration, editor plumbing, compatibility evidence, and
release/composition gates; none of those gates should be skipped.

Do not treat local commits or dirty submodule worktrees as merged upstream
state. Recheck the baseline before claiming an issue.

## Execution board

| Block | Issue | Owner repository | State | Estimate |
| --- | --- | --- | --- | --- |
| 0 — contract | [CBH-001](block-0-contract/CBH-001-freeze-contract.md) | hub | Ready | 0.5 day |
| 1 — foundation | [CBH-101](block-1-foundation/CBH-101-publish-source-and-attribution.md) | harness-metrics | Maintainer access needed | 0.5 day |
| 1 — foundation | [CBH-102](block-1-foundation/CBH-102-release-harness-metrics-0.0.3.md) | harness-metrics | Blocked by CBH-101 and release approval | 0.5 day |
| 2 — runtime | [CBH-201](block-2-runtime/CBH-201-add-runtime-modes.md) | language-server | Partial local proof exists | 1 day |
| 2 — runtime | [CBH-202](block-2-runtime/CBH-202-complete-live-and-snapshot-lifecycle.md) | language-server | Planned | 1 day |
| 2 — runtime | [CBH-203](block-2-runtime/CBH-203-verify-lsp-contract.md) | language-server | Planned | 0.5 day |
| 3 — editor | [CBH-301](block-3-editor/CBH-301-expose-vscode-settings.md) | harness-lens-vscode | Planned | 0.5–1 day |
| 3 — editor | [CBH-302](block-3-editor/CBH-302-add-runtime-ux.md) | harness-lens-vscode | Planned | 0.5–1 day |
| 4 — assurance | [CBH-401](block-4-assurance/CBH-401-run-compatibility-matrix.md) | language-server + editor | Planned | 1 day |
| 4 — assurance | [CBH-402](block-4-assurance/CBH-402-audit-privacy-and-licensing.md) | all affected repositories | Planned | 0.5 day |
| 5 — composition | [CBH-501](block-5-composition/CBH-501-merge-release-and-pin.md) | component repositories | Planned | 0.5–1 day |
| 5 — composition | [CBH-502](block-5-composition/CBH-502-finalize-hub-composition.md) | hub | Last | 0.5 day |

Estimates are engineering time for one contributor and exclude waiting for
credentials, reviews, registry propagation, or Marketplace approval. An MVP
through editor settings should take 3–5 working days. Cross-platform assurance,
release preparation, and final composition bring the total to 5–8 working days.

## Order and safe parallel work

Critical path:

```text
CBH-001 -> CBH-101 -> CBH-102 -> CBH-201 -> CBH-202 -> CBH-203
        -> CBH-301 -> CBH-302 -> CBH-401/CBH-402 -> CBH-501 -> CBH-502
```

After CBH-201 freezes the initialization contract, one contributor may start
CBH-301 while another completes CBH-202 and CBH-203. CBH-401 fixture planning
may also begin early, but its results are not final until both the server and
editor changes are merged. Avoid parallel edits inside the same repository
unless file ownership is agreed first.

## Shared definition of done

Every issue must leave this evidence in its owning pull request:

- linked `CBH` dependencies and immutable upstream commit SHAs;
- exact verification commands and their results;
- evidence-method and sample-size behavior for new insights;
- safe failure behavior, with no raw source, transcript, secret, or raw stderr
  added to snapshots or reports;
- documentation for changed user-facing behavior;
- no unrelated hub implementation and no floating Git dependency.

Merge the lowest-level owner first. Update downstream dependency pins next.
Update hub submodule pins and `docs/repository-split.md` only in CBH-502.
