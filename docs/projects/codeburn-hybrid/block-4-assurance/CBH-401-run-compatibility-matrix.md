<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-401: Run the hybrid compatibility matrix

**Repositories:** language-server and harness-lens-vscode
**Suggested labels:** `integration`, `testing`, `cross-platform`
**Estimate:** 1 day
**Depends on:** CBH-203 and CBH-302

## Outcome

Prove the hybrid contract through the packaged client/server boundary, not only
through unit tests.

## Required matrix

| Mode | Input state | Expected result |
| --- | --- | --- |
| `off` | CodeBurn absent | Static diagnostics; no runtime process or warning |
| `live` | CodeBurn `0.9.24` valid output | Runtime hover, diagnostics, and code lenses |
| `live` | executable absent | Static diagnostics plus one recoverable warning |
| `live` | `optimize` fails | Report/model insights plus optional-data warning |
| `live` | required command fails/times out | Previous runtime snapshot retained |
| `snapshot` | canonical fixture valid | Same correlation without child process |
| `snapshot` | file absent/malformed | Previous snapshot retained; static diagnostics |

Run the matrix on Linux, macOS, and Windows where CI runners are available.
Use a generic LSP harness for server assertions and a packaged VSIX smoke test
for editor startup/configuration. Real CodeBurn testing may be a separate,
explicitly enabled job; deterministic fixture jobs remain the required gate.

## Acceptance criteria

- [ ] Fixtures contain synthetic aggregates only and are reviewed for paths,
      identities, source, transcript fragments, and secrets.
- [ ] Linux, macOS, and Windows agree on path and executable behavior.
- [ ] Tests verify zero child-process launch in `off` and `snapshot`.
- [ ] Non-ASCII documents produce correct UTF-16 positions.
- [ ] Package-built server and VSIX pass, not only source-tree execution.
- [ ] Any platform exclusion has an issue, owner, rationale, and safe fallback.
- [ ] Results record CodeBurn version, Harness Metrics version/SHA, language
      server SHA, extension SHA, and fixture revision.

## Non-goals

- Performance benchmarking of large workspaces.
- Supporting undocumented CodeBurn output shapes.
- Requiring real user session stores in CI.

## Verification

Attach a completed copy of the matrix with workflow links and artifact checksums
to the issue. A green unit suite alone does not close this issue.
