<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-001: Freeze the hybrid runtime contract

**Repository:** composition hub
**Suggested labels:** `architecture`, `codeburn`, `decision`
**Estimate:** 0.5 day
**Depends on:** none

## Outcome

Approve one implementation contract before contributors change package or
protocol surfaces. Static analysis remains unconditional; runtime behavior is
explicitly `off`, `live`, or `snapshot`, with `off` as the safe default.

## Decisions to record

- CodeBurn is an optional external executable, not a copied algorithm or
  bundled library.
- `live` is the only mode allowed to launch CodeBurn.
- `snapshot` consumes the canonical combined Harness Metrics JSON shape and
  performs no capture.
- Runtime evidence cannot modify deterministic findings or normalized scores.
- Required runtime failures are observable and preserve the last valid
  snapshot. Optional optimization failure becomes a safe warning.
- Initialization options override environment variables; environment variables
  override defaults. Invalid configuration falls back to `off`, never `live`.
- Editor configuration changes restart the server for the MVP. Dynamic LSP
  reconfiguration is a later enhancement.

## Acceptance criteria

- [ ] Architecture review agrees on all three modes and the default.
- [ ] The owner boundary matches `core <- sdk <- LSP <- editor`.
- [ ] The dependency/attribution table in the project README is accepted.
- [ ] The canonical snapshot owner is Harness Metrics, not the editor.
- [ ] No CodeBurn dependency is proposed for core, SDK, or CLI.
- [ ] Privacy and failure invariants are copied into downstream issues.
- [ ] Any changed decision is reflected in the project README before CBH-101
      begins.

## Non-goals

- Implementing code.
- Choosing an automatic installer for CodeBurn.
- Bundling CodeBurn with a native server or editor extension.
- Adding runtime observations to quality scores.

## Verification

Review the contract against `AGENTS.md`, `docs/architecture.md`,
`docs/integrations.md`, and `docs/prior-art/codeburn.md`. The issue closes with
a maintainer comment approving the contract or an amended project README.
