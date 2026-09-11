<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step 00 — baseline audit

Purpose: identify current ownership and preserve interrupted work.

## Read

- `state/now.md`
- repository `AGENTS.md`
- [02-recovered-state](../02-recovered-state.md)

## Check

```bash
git status --short --branch
git submodule status --recursive
git worktree list --porcelain
```

Inspect component worktrees before touching them. Record branch, dirty files,
and whether changes match this compass.

## Output

Update `state/now.md` with chosen owning repository and next safe slice.
No implementation edit. No commit.

## Stop

Stop if ownership is ambiguous or a dirty tree overlaps requested slice.
