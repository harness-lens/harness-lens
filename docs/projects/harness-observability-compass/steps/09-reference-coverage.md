<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step 09 — reference, rule, and workflow coverage

Status: deferred by the user on 2026-09-06. Bundle with the future code-aware
analysis improvements; do not block the current downloadable Windows release.

## Requested outcome

Analyze supporting files such as `skills/graphify/references/**`, including
those under recognized `.agents/skills` and `.codex/skills` roots. Extend useful
coverage to `.agents/rules/**` and `.agents/workflows/**`. Findings should help
an AI inspect the affected source and, when a Graphify integration becomes
available, let Graphify consume a stable reference to each file's metrics.

## Existing foundation

Accepted SDK `4a6512b0834f6c8cfce5205bca7e8722139a06fd` already recognizes
`.agents/rules` files with `.md`, `.mdc`, and `.rules` extensions. It also
records bounded local Markdown reference edges, provenance, and unresolved or
cyclic references. Recording a reference edge is not the same as discovering
and analyzing the referenced file. Inspect current discovery configuration
before claiming a rule/workflow family is newly supported or missing.

## Planned slice

1. Specify supported text types, skill roots, and whether a reference file is
   included by a local link, an explicit references-directory policy, or both.
   Keep arbitrary workspace scanning out of the default. Define `.agents/workflows`
   recognition and verify nested rule coverage with real fixtures.
2. Extend SDK discovery and traversal. Resolve references relative to their
   source, keep canonical paths inside the workspace, preserve symlink/ignore
   policy, detect cycles, deduplicate physical files, and cap depth, bytes,
   files, and edges. Missing/ignored/unsupported/limited inputs stay observable.
3. Represent each analyzed file and its owning skill/rule/workflow provenance
   in Core's neutral contracts. Preserve per-file findings, context estimates,
   original spans, assumptions, and completeness. Avoid counting a shared
   reference twice in aggregate metrics. A link does not prove that an agent
   loaded the file or that its instructions inherit the parent's scope.
4. Apply the [code-aware repetition plan](../../code-aware-repetition/README.md)
   so examples in reference documents do not create syntax-only warnings.
   Keep real repeated prose and distant duplicate instructions detectable.
5. Expose a versioned machine-readable reference to metrics/findings through
   SDK and CLI/LSP. AI and optional Graphify adapters may consume completed
   deterministic reports; they cannot alter scores or introduce a Graphify
   dependency into Core. Define stale/deleted-file handling before consumption.
   No report includes source contents, secrets, or unbounded runtime output.
6. Verify closed files, unsaved overlays, UTF-8/UTF-16 ranges, traversal bounds,
   cycles, shared references, nested rules/workflows, and optional-consumer
   absence. Deliver Core contracts first where needed, then SDK, CLI/LSP,
   editor, and finally hub pins after owning PRs merge.

Graphify's future interface is not yet verified or available in this work.
Research and record its actual contract and prior art when that slice begins;
no invented integration API or Graphify execution is part of this backlog item.
