<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-102: Release corrected Harness Metrics 0.0.3

**Repository:** `harness-lens/harness-metrics`
**Suggested labels:** `release`, `rust`, `python`
**Estimate:** 0.5 day
**Depends on:** CBH-101
**External gate:** explicit maintainer approval plus crates.io and PyPI access

## Outcome

Publish the first trustworthy shared version used by the language server. The
release fixes percentage scaling and evidence provenance, exposes sample sizes,
and keeps optional capture failure observable.

## Work

1. Confirm all Rust and Python package versions are exactly `0.0.3`.
2. Build and inspect crate, sdist, and wheel contents from a clean checkout.
3. Publish `harness-metrics 0.0.3` to crates.io only after explicit approval.
4. Publish only `0.0.3` to PyPI; do not backfill unpublished `0.0.1` or `0.0.2`.
5. Install both registry artifacts into clean temporary environments and run a
   smoke test for deterministic metrics and canonical snapshot parsing.
6. Create release notes linking the source commit from CBH-101.

## Acceptance criteria

- [ ] `savingsPct: 55` renders as `55%`, never `5500%`.
- [ ] Every insight carries a deterministic, heuristic, or statistical method.
- [ ] Statistical text includes its relevant sample count.
- [ ] Estimated cost and CodeBurn-reported savings retain those labels.
- [ ] Optional `optimize` failure becomes a safe warning without raw stderr.
- [ ] crates.io and PyPI metadata link to the public source repository.
- [ ] Fresh-install smoke tests pass for both ecosystems.
- [ ] Registry URLs, versions, and immutable source SHA are posted on the issue.

## Non-goals

- Publishing the language server or VS Code extension.
- Republishing or deleting historical crate versions.
- Storing registry credentials in repository files or logs.

## Verification

```bash
cd rust
cargo package -p harness-metrics --locked
cd ..
python3 -m build
python3 -m twine check --strict dist/*
```

Run registry installation checks in new temporary directories. Publication is
irreversible; dry-run success is not authorization to publish.
