<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Prompt 1 — infrastructure and contracts checkpoint

Date: 2026-09-06. Status: Core merged; SDK PR passes CI; full Phase 1 remains
incomplete.

## Core merge and SDK continuation

Core PR #16 merged as `cec4902041544ef4377e97412cd9f79a9a3897b7`.
The merge tree matches reviewed head `6f0e2e2462c1e99401511c9787501a172887622d`.

SDK [PR #23](https://github.com/harness-lens/sdk/pull/23) pins the Core merge and
is clean at `4f66415ebd91971ad549d45fbbac8fa1579fb2d4`. Stable formatting, locked
Clippy, all 23 Rust unit tests, doc tests,
offline locked metadata, and diff checks pass. The provider service uses fixed
local metadata, off/snapshot non-execution, two-second/4-KiB live detection,
safe error classes, exact CodeBurn `0.9.24` plans, consent and workspace-policy
rechecks, fixed no-shell npm arguments, and fake-runtime tests. No CodeBurn or
package installation occurred.

PR CI passes Rust, TypeScript on Node 20/22/24, Python 3.10/3.14, and CodeQL
language analysis. User explicitly authorized push and PR creation after fresh
external-export confirmation. No merge performed. Language Server work waits
for SDK review and accepted merge SHA.

## Lockfile fix and passing CI

User reported the failed locked MSRV test and dirty-lockfile package step.
Committed and pushed `6f0e2e2462c1e99401511c9787501a172887622d`
(`fix(build): lock Unicode dependencies`) to Core PR #16. PR remains open with
the user's existing non-draft status; the owning worktree is clean.

Cargo generated the lockfile using registry metadata only. It now records
`unicode-casefold 0.2.0`, `unicode-normalization 0.1.24`, `unicode-properties 0.1.3`,
`tinyvec 1.13.2`, and `tinyvec_macros 0.1.1`. All previously locked registry
versions, checksums, and dependency entries are unchanged. The unrelated `syn`
upgrade from full regeneration was reverted with Cargo's precise offline update.
No crate source, toolchain, or external provider was downloaded or installed.

Stable Clippy and tests now use `--locked`, matching MSRV and packaging.
Packaging still requires a clean worktree; no `--allow-dirty` workaround was
added. Reviewer instructions now verify the committed graph without regeneration.

Local formatting, diff checks, locked offline manifest metadata, and a comparison
of existing registry lock entries passed. Locked offline tests now reach the
source-download boundary for uncached `tinyvec`; they leave the lockfile byte-
identical. No source download was attempted outside offline mode.

[CI run 34044820013](https://github.com/harness-lens/core/actions/runs/34044820013)
passed at this exact commit, including locked Rust 1.85.1 tests, stable Rust
formatting/locked Clippy/tests, `cargo package --locked`, and all three TypeScript
jobs. This supersedes the lockfile/MSRV/package blockers recorded below.
Provider contract acceptance, Unicode license/table review, downstream Phase 1
services, authorized merges, and accepted pins remain separate work.

## Initial review handoff

User explicitly requested a Phase 1 PR so they can review and verify locally.
Created draft Core [PR #16](https://github.com/harness-lens/core/pull/16) from
`feat/lexical-provider-contracts` at immutable commit
`02e7e82d12c6dcb1cf0d17f32edbc4a7ce35d758`, based on merged foundation
`b9628198f34d685a5ebad78a614e17de277079ab`. Branch is pushed and the owning
worktree is clean. No merge, publication, installation, or hub pin change occurred.

The PR is the Core portion only. SDK adapters, Language Server provider
catalog/aggregate transport, and editor typed services/consent remain pending.
Exact reviewer commands and acceptance work are committed in Core
[`docs/phase1-verification.md`](https://github.com/harness-lens/core/blob/02e7e82d12c6dcb1cf0d17f32edbc4a7ce35d758/docs/phase1-verification.md).

Fresh checks on this draft:

- `cargo fmt --all --check`, `git diff --cached --check`: passed.
- `npm test` (one file), `npm run check`, `npm pack --dry-run` (67 files): passed
  with existing dependencies and a temporary writable cache; no `npm ci` or
  dependency installation. Temporary dependency symlink removed.
- Replaced the bounded-distance test's self-comparison with an independent full
  matrix over every binary string of length zero through five and every budget
  zero through five. Extracted actual function/test passed under installed
  `rustc 1.97.1`; this does not verify the full crate or Unicode integration.
- Full offline Rust tests, Clippy, and locked package check: blocked by missing
  `unicode-casefold`. Rust 1.85.1 remains absent. No fetch/workaround attempted.
- `rust/Cargo.lock` still predates the new Unicode dependencies. Generate,
  review, and commit it before the existing `--locked` MSRV CI job can pass.
- Case-fold license/table review and probabilistic prior/interval support remain
  acceptance work; the draft safely rejects probabilistic contributions.

Remote CI at the same commit has completed:
[run 34043765090](https://github.com/harness-lens/core/actions/runs/34043765090).
Stable Rust formatting, Clippy, and all 40 tests pass. TypeScript on Node 20/22/24
and CodeQL pass. Rust 1.85.1 fails before compilation because the committed
lockfile is outdated under `--locked`. Stable packaging fails because the
preceding unlocked build updated `Cargo.lock`, leaving a dirty file. Full Rust
compilation/testing is now verified in CI, while local full-crate checks remain
blocked. PR description includes these exact results; overall Rust CI is not green.

Prior sections below record the earlier draft checkpoint. This handoff supersedes
their statements that no Core commit, push, or PR exists.

## Fresh foundation verification

Read root AGENTS.md and state/now.md first. Queried GitHub PR state and fetched
each component's current origin/main. All foundation PRs are merged:

| Repository | PR | Accepted merge SHA/current origin/main |
| --- | --- | --- |
| Core | https://github.com/harness-lens/core/pull/15 | `b9628198f34d685a5ebad78a614e17de277079ab` |
| SDK | https://github.com/harness-lens/sdk/pull/22 | `8e6f325444b2775813e26c7ebd9403a7f868b7c6` |
| CLI | https://github.com/harness-lens/cli/pull/17 | `d7ce2552db7895317508ab6f4f18c2ac2c8e6024` |
| Language Server | https://github.com/harness-lens/language-server/pull/22 | `d5512d0cf4545b61c61d43e952366e6ff4e4dd59` |
| VS Code | https://github.com/harness-lens/harness-lens-vscode/pull/21 | `a3bd8226144cebecf5b57738b2ad84140350dcac` |

Remote CI was successful (SDK's separate CodeQL aggregate neutral). These are
foundation results, not verification of the new changes. Refresh remote state
again before resuming commits or dependency pins.

## Isolated worktrees

- `build/infrastructure-core`, branch `feat/lexical-provider-contracts`, based on
  accepted Core SHA above. Now committed/pushed as Core PR #16; full Rust crate
  remains uncompiled locally. See current review handoff above.
- `build/infrastructure-sdk`, branch `feat/provider-services`, clean at accepted SDK base.
- `build/infrastructure-language-server`, branch `feat/provider-protocol`, clean at accepted LSP base.
- `build/infrastructure-vscode`, branch `feat/provider-services`, clean at accepted editor base.

At the earlier checkpoint no commits, pushes, or new PRs existed. The current
Core PR supersedes that state. No merges, releases, installations, or hub pin
changes were performed. Existing dirty hub/module work is preserved.

## Core draft

- `rust/src/lexical.rs`: separate span-only heuristic report; NFC/full non-Turkic
  case folding/NFC; normalized scalar Levenshtein distance; inclusive default 0.2;
  minimum length 4; input/source/token/length/pair ceilings; bounded dynamic
  programming; exact-normalized-match exclusion; stable ordering and completeness.
- `rust/src/providers.rs`: provider metadata/status/install-preview/report/merge
  contracts, fixed safe metric and assumption enums, explicit fingerprint groups,
  provider namespaces, deterministic ordering, safe errors and snapshot retention.
- Config and engine expose `lexical` outside deterministic findings and scores.
- `docs/prior-art/lexical-similarity.md` and `docs/provider-contracts.md` document
  assumptions, bounds, trust boundaries, and remaining Phase 2 presentation work.
- Tests added for lexical behavior, privacy, bounds, ordering, merge collisions,
  explicit deduplication, failure isolation, and stale snapshot rules; not yet run.

Review still required: verify pinned case-fold library license/table version;
verify the full crate with the new independent matrix oracle;
assess probabilistic contribution support (currently rejected safely
until an explicit prior/interval contract is added); implement downstream adapters.

## Verification and blocker

- Core `cargo fmt --all --check`: pass.
- Core `git diff --check`: pass.
- Core TypeScript `npm test`: pass (1 test file); `npm run check`: pass, using
  existing Linux dependencies from `modules/core/node_modules` through a temporary
  symlink; no dependency installation. The symlink was removed after checks.
- Core `npm pack --dry-run`: pass using a temporary writable npm cache.
- Core `cargo test --all-features --offline`: blocked, missing `unicode-casefold`.
- Attempt to fetch pinned `unicode-casefold = 0.2.0` and run tests was rejected
  by automatic approval review: fetching/building a new third-party dependency
  conflicts with the user's explicit no-external-software-installation restriction.
  Do not retry the download or use an indirect workaround without authorization.
- Rust 1.85 is absent locally; only stable is installed. No toolchain installed.
- Hub composition check reports pre-existing dirty work; not a clean composition.

An asynchronous approval question asks whether development-only dependency
downloads and Rust 1.85.1 installation are allowed; CodeBurn and all external
report providers remain excluded. No answer received at checkpoint time.

## Resume order

Review and verify Core PR #16; generate/review/commit the missing lockfile and
resolve remaining acceptance blockers. Local downloads still require development
dependency/toolchain permission. Then SDK services, LSP bounded catalog and aggregate
protocol, VS Code typed services/settings/consent boundaries, in that order.
Use immutable upstream SHAs and independently verify each repository. Do not
merge or update hub pins without the required owning PR merges. Do not polish UI.
Harness Lens Native means first-party MPL-2.0, always available and deterministic;
CodeBurn stays optional, unbundled, default off, blocked for untrusted/virtual roots.
