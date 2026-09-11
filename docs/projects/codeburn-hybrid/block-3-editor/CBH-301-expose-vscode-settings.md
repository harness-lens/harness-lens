<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-301: Expose safe VS Code runtime settings

**Repository:** `harness-lens/harness-lens-vscode`
**Suggested labels:** `vscode`, `configuration`, `codeburn`
**Estimate:** 0.5–1 day
**Depends on:** CBH-203 contract; implementation may begin after CBH-201 freezes
the initialization shape

## Outcome

Let users select runtime evidence deliberately and pass validated settings to
the Rust language server without bundling or silently installing CodeBurn.

## Settings

- `harnessLens.metrics.mode`: enum `off`, `live`, `snapshot`; default `off`.
- `harnessLens.metrics.codeBurn.path`: machine-scoped executable path; default
  `codeburn`.
- `harnessLens.metrics.codeBurn.period`: resource-scoped period; default
  `30days`.
- `harnessLens.metrics.snapshot.path`: resource-scoped canonical snapshot path.

Names may change only if CBH-203 records the final mapping. Keep language-server
process settings separate from runtime-evidence settings.

## Acceptance criteria

- [ ] Default extension startup passes `mode: off`.
- [ ] Settings serialize into the documented initialization options exactly.
- [ ] Changing a runtime setting performs one clean language-server restart.
- [ ] Live-only and snapshot-only values are ignored outside their mode.
- [ ] `snapshot` without a path and invalid periods produce actionable local
      configuration messages and remain safe.
- [ ] Live execution remains disabled in untrusted and virtual workspaces.
- [ ] No command downloads, installs, or updates CodeBurn.
- [ ] No CodeBurn npm dependency or VS Code extension dependency is declared.
- [ ] Marketplace/README text calls CodeBurn an optional external dependency for
      live runtime insights.

## Non-goals

- Implementing a graphical snapshot browser.
- Passing secrets to CodeBurn.
- Shipping native language-server binaries in the same change.

## Verification

Extract pure configuration mapping where needed so unit tests can cover every
mode and invalid combination without launching VS Code.

```bash
npm ci
npm run check
npm test
npm run package
```
