<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-203: Verify and document the LSP runtime contract

**Repository:** `harness-lens/language-server`
**Suggested labels:** `testing`, `protocol`, `documentation`
**Estimate:** 0.5 day
**Depends on:** CBH-202

## Outcome

Turn runtime mode behavior into a client-neutral, regression-tested protocol
contract before the VS Code client depends on it.

## Acceptance criteria

- [ ] Protocol docs define initialization JSON, environment fallbacks,
      precedence, restart behavior, and refresh semantics.
- [ ] A matrix covers `off`, valid/failed `live`, and valid/failed `snapshot`.
- [ ] Tests prove `off` and failed runtime modes preserve deterministic results.
- [ ] Tests prove refresh remains available after initial live/snapshot failure.
- [ ] Tests cover method labels, statistical sample sizes, estimated-cost text,
      `55%` percentage scaling, and optional warnings.
- [ ] Tests cover UTF-16 conversion with non-ASCII text before an insight span.
- [ ] Raw subprocess stderr and snapshot contents do not enter persisted output.
- [ ] The README names CodeBurn as optional, separately installed, and not
      bundled.
- [ ] The existing TypeScript compatibility server is clearly documented as not
      implementing the Rust runtime surface.

## Non-goals

- Client-specific settings or UI.
- Network transport or MCP ingestion.
- New analysis rules.

## Verification

```bash
npm install
npm test
npm run check
cd rust
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
```

Attach the completed behavior matrix and exact output summaries to the pull
request. The final commit SHA becomes an input to CBH-501.
