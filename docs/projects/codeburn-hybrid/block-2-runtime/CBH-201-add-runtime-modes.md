<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-201: Add explicit language-server runtime modes

**Repository:** `harness-lens/language-server`
**Suggested labels:** `rust`, `lsp`, `configuration`, `codeburn`
**Estimate:** 1 day
**Depends on:** CBH-102

## Outcome

Replace unconditional startup capture with a typed `off`/`live`/`snapshot`
configuration. A default installation performs no CodeBurn process execution.

## Work

1. Pin Harness Metrics `0.0.3` with both its registry version and the immutable
   Git SHA produced by CBH-101.
2. Parse a namespaced LSP initialization option shaped like:

   ```json
   {
     "harnessMetrics": {
       "mode": "off",
       "codeBurn": { "executable": "codeburn", "period": "30days" },
       "snapshot": { "path": null }
     }
   }
   ```

3. Support equivalent environment variables for non-editor clients:
   `HARNESS_METRICS_MODE`, `HARNESS_METRICS_CODEBURN_EXECUTABLE`,
   `HARNESS_METRICS_CODEBURN_PERIOD`, and `HARNESS_METRICS_SNAPSHOT`.
4. Apply precedence: initialization option, environment variable, safe default.
5. Make `off` skip capture, snapshot loading, runtime diagnostics, runtime
   hover, and runtime code lenses while leaving all `HL...` behavior unchanged.
6. Treat unknown modes or invalid combinations as observable configuration
   errors that fall back to `off`.

## Acceptance criteria

- [ ] Default initialization launches no CodeBurn child process.
- [ ] Only `live` may use executable/period settings.
- [ ] Only `snapshot` may require a snapshot path.
- [ ] `off` produces the same deterministic diagnostics as before integration.
- [ ] Invalid configuration cannot accidentally enable `live`.
- [ ] Secrets, source text, and environment values are not logged.
- [ ] Cargo uses a full immutable Harness Metrics Git revision.

## Non-goals

- Dynamic `workspace/didChangeConfiguration`; editor restart is sufficient.
- Editor settings and presentation; CBH-301 owns them.
- Adding configuration types to core or SDK.

## Verification

Add unit tests for defaults, precedence, invalid values, and mode-specific
requirements. Use a fake capture boundary to assert zero child-process calls in
`off` and `snapshot`.

```bash
cd rust
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
```
