<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-302: Add observable runtime UX

**Repository:** `harness-lens/harness-lens-vscode`
**Suggested labels:** `vscode`, `ux`, `observability`
**Estimate:** 0.5–1 day
**Depends on:** CBH-301

## Outcome

Make runtime state understandable without noisy repeated notifications. Users
can refresh the configured source, find setup guidance, and distinguish static
analysis from optional runtime evidence.

## Work

- Add **Harness Lens: Refresh Runtime Metrics**. Forward the documented LSP
  command in `live` and `snapshot`; explain that runtime metrics are off in
  `off` mode.
- Show the configured state in the existing status item or tooltip: static
  only, runtime live, or snapshot. Keep detailed warnings in the output channel.
- Route detailed server messages to the Harness Lens output channel.
- When CodeBurn is missing in `live`, offer settings and documentation actions.
  Do not offer automatic installation.
- When snapshot loading fails, identify the configured path safely and link to
  the canonical snapshot creation command.
- Debounce or remember repeated identical failures for the session.

## Acceptance criteria

- [ ] Refresh is available through the command palette and never launches
      CodeBurn from `off` or `snapshot` mode.
- [ ] A missing executable produces one actionable warning, not a crash loop.
- [ ] Optional optimization warnings do not claim all runtime data failed.
- [ ] Status text never implies runtime evidence changed deterministic scores.
- [ ] Untrusted/virtual workspaces cannot start either server or CodeBurn.
- [ ] Paths, raw JSON, stderr, source, and secrets are absent from telemetry and
      persisted extension state.
- [ ] README includes setup, disable, live, snapshot, and troubleshooting steps.

## Non-goals

- A custom renderer for every Harness Metrics insight.
- Automatic periodic capture.
- Uploading snapshots or reports.

## Verification

Test command registration, mode-specific behavior, restart/failure suppression,
and configuration links. Then run:

```bash
npm ci
npm run check
npm test
npm run package
```

Install the produced VSIX in a disposable profile for one manual smoke test.
