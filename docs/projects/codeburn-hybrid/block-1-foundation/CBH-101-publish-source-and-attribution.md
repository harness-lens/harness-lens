<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-101: Publish Harness Metrics source and attribution

**Repository:** `harness-lens/harness-metrics`
**Suggested labels:** `repository`, `documentation`, `codeburn`
**Estimate:** 0.5 day
**Depends on:** CBH-001
**External gate:** maintainer with organization repository-create/push access

## Outcome

Make the verified Harness Metrics source reviewable at its declared repository
URL and state its CodeBurn relationship precisely before publishing packages.

## Work

1. Create the `harness-lens/harness-metrics` repository with the intended
   visibility and MPL-2.0 metadata.
2. Push the verified local history without rewriting unrelated work. Start from
   commit `157328cdfa6473df9d177b4046fe754cdcbf5862`.
3. Ensure README, CLI help, and `docs/codeburn.md` call CodeBurn an optional
   external runtime dependency used only for aggregate capture.
4. Add a third-party notice naming CodeBurn, its MIT license, upstream URL, and
   the fact that it is not bundled and no source was copied.
5. Record CodeBurn `0.9.24` as the tested version. Do not invent a minimum
   supported version without contract-test evidence.
6. Push the final commit and record its immutable full SHA for CBH-102 and
   CBH-201.

## Acceptance criteria

- [ ] The package metadata repository/homepage URLs resolve publicly.
- [ ] A clean clone contains licenses, provenance docs, tests, and release
      instructions.
- [ ] `harness-metrics --help` explains that live capture requires the separate
      CodeBurn executable.
- [ ] Attribution distinguishes an external runtime dependency from bundled
      third-party code.
- [ ] Documentation says snapshots can contain paths/model metadata and must
      not be committed blindly.
- [ ] The final full commit SHA is posted on the issue.

## Non-goals

- Publishing crates or wheels; CBH-102 owns irreversible registry actions.
- Adding CodeBurn to Cargo, PyPI, or npm dependency lists.
- Copying CodeBurn parsers, pricing tables, or optimization logic.

## Verification

```bash
cd rust
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd ..
python3 -m pytest
python3 -m build
python3 -m twine check --strict dist/*
git status --short
```

Attach command results and the clean-clone URL to the pull request.
