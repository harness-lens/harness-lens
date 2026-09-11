<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Recovered state

Source: codex-second session `01a06de1-3dbd-7271-89c1-895bfc76589b`, titled
“Validate GitHub workflows”. Recovered on 2026-09-05.

## Session-reported remote state

- Language Server PRs #20 and #21 merged; latest reported main SHA
  `8f376550b19b6d5b4be773cd46b34780172d0125`.
- VS Code PR #20 merged; latest reported main SHA
  `e6a94af3b37f32bc3ee81e257c25bbcb4e7990b2`.
- Hub PR #12 merged; latest reported main SHA
  `a1b511f2c0441b1cef02c794d199faa65bc004be`.
- Session recorded passing Rust gates, protocol smoke test, six editor tests,
  strict TypeScript checks, and clean temporary implementation worktrees.

These are recovered claims, not proof for a new change. Recheck remote state
before pinning or declaring work complete.

## Local checkout now

- Branch: `refactor/repository-split`.
- Hub contains pre-existing documentation edits, untracked project files, and
  dirty submodule gitlinks.
- Older component worktrees still exist. Preserve them until ownership and
  dirty state are explicitly audited.
- No commit is requested for this compass.

## Known product gap

Runtime tool-call history, safe cost attribution, inclusion edges, pagination,
and per-file effectiveness remain incomplete. Do not invent effectiveness
scores while those contracts lack attributable runtime evidence.
