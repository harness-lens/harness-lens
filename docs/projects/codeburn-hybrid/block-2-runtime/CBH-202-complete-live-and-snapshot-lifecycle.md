<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-202: Complete live and snapshot lifecycle

**Repository:** `harness-lens/language-server`
**Suggested labels:** `rust`, `lsp`, `runtime`, `reliability`
**Estimate:** 1 day
**Depends on:** CBH-201

## Outcome

Make both opt-in runtime paths useful and recoverable while preserving static
analysis through every CodeBurn or snapshot failure.

## Live-mode work

- Reuse the existing partial proof for concurrent `report`, `models`, and
  `optimize` capture rather than reimplementing it.
- Bound each capture and the total refresh duration.
- Require valid report/model output. Treat optimization output as optional.
- Capture at startup and on `harnessMetrics.refreshCodeBurn` only.
- Preserve the previous valid in-memory snapshot after required-command failure.
- Leave refresh available after initial capture failure.

## Snapshot-mode work

- Load Harness Metrics' canonical combined JSON using its Rust parser.
- Resolve relative paths against the selected workspace root. Accept an
  absolute path only when explicitly configured.
- Bound file size before parsing.
- Load at startup and reload on explicit refresh without launching CodeBurn.
- Preserve the previous valid snapshot on read or parse failure.
- Report safe path/error categories without leaking raw JSON or source content.

## Acceptance criteria

- [ ] Three live commands execute concurrently with documented timeout bounds.
- [ ] Missing `optimize` retains report/model insights plus a visible warning.
- [ ] Missing executable, timeout, non-zero exit, and malformed required JSON do
      not suppress `HL...` diagnostics or crash the server.
- [ ] Snapshot mode never starts a CodeBurn process.
- [ ] Unknown snapshot fields are accepted; incompatible outer shapes fail
      visibly.
- [ ] Refresh behavior matches the active mode.
- [ ] Runtime findings retain method, basis, and sample sizes from Harness
      Metrics.
- [ ] All UTF-8 evidence spans are converted to valid negotiated UTF-16 ranges.

## Non-goals

- Watching snapshot files continuously.
- Persisting live snapshots automatically from the language server.
- Reading CodeBurn's raw session stores.
- Retrying failed capture in a background loop.

## Verification

Use fixture executables and fixture snapshots containing no real paths, model
account IDs, transcripts, or secrets. Test startup, refresh, stale-snapshot
preservation, optional failure, timeout, and Unicode positions.

```bash
cd rust
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test --locked
```
