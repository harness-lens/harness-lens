<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Prompt 2 — provider interface prerequisite audit

Date: 2026-09-06. Status: blocked before implementation, as required by Prompt 2.
The merged foundation does not include the Phase 1 provider infrastructure.

Subsequent handoff: user requested a reviewable infrastructure PR. Core
[PR #16](https://github.com/harness-lens/core/pull/16) is now open at
`6f0e2e2462c1e99401511c9787501a172887622d`; its worktree is clean. The lockfile
fix passes CI, including locked Rust 1.85.1 tests and packaging. See the
[current Phase 1 checkpoint](infrastructure-phase1.md) for verification and
acceptance blockers. This supersedes the uncommitted/no-PR observations below;
the Phase 2 merge-and-pin prerequisite remains unmet.

## Verified remote state

Read root AGENTS.md, state/now.md, the Phase 1 checkpoint, Core's draft
`docs/provider-contracts.md`, and Language Server `docs/protocol.md`.
Read-only GitHub queries (`gh pr view`, `gh pr list --state all --limit 10`) and
`git ls-remote` succeeded with reviewed network access. Every remote main SHA
matches the existing local origin/main object and the foundation checkpoint:

| Repository | Foundation PR | Remote main / merge SHA |
| --- | --- | --- |
| Core | https://github.com/harness-lens/core/pull/15 | `b9628198f34d685a5ebad78a614e17de277079ab` |
| SDK | https://github.com/harness-lens/sdk/pull/22 | `8e6f325444b2775813e26c7ebd9403a7f868b7c6` |
| CLI | https://github.com/harness-lens/cli/pull/17 | `d7ce2552db7895317508ab6f4f18c2ac2c8e6024` |
| Language Server | https://github.com/harness-lens/language-server/pull/22 | `d5512d0cf4545b61c61d43e952366e6ff4e4dd59` |
| VS Code | https://github.com/harness-lens/harness-lens-vscode/pull/21 | `a3bd8226144cebecf5b57738b2ad84140350dcac` |

All five PRs are merged. Latest PR listings contain no subsequent Phase 1
provider PRs. The known infrastructure branch names are absent remotely.

Current SDK manifest pins foundation Core commit
`958564df56c85ae04c84506b594900758e63ac6a`; CLI and Language Server pin foundation
SDK commit `9dc39f3ad9d5837be4f69ba4553d39bee4630225`. These are immutable
foundation PR head commits, not commits containing the new provider contracts.
No Phase 1 provider commit or downstream pin exists to accept.

## Exact dependency blocker

- Core `build/infrastructure-core` remains dirty on the accepted foundation
  base. `rust/src/lexical.rs`, `rust/src/providers.rs`, and their documentation
  are untracked; associated config/engine/model changes are uncommitted.
  There is no verified, merged Core provider/lexical contract SHA.
- `build/infrastructure-sdk`, `build/infrastructure-language-server`, and
  `build/infrastructure-vscode` are clean at their foundation bases. The
  planned SDK provider detection/installation adapters, LSP catalog/aggregate
  protocol, and editor typed provider services/consent boundaries remain pending.
- Inspection of the matching origin/main trees found no provider/lexical
  modules or draft aggregate contract. The accepted protocol documents only
  `harnessLens/workspaceReport` with the existing runtime envelope, not the
  Phase 1 provider catalog and aggregate transport required by Prompt 2.

The Phase 1 checkpoint records Rust verification blocked by uncached
`unicode-casefold = 0.2.0` and the absent Rust 1.85 toolchain. Automatic approval
review previously rejected that dependency download under the no-external-
software-installation restriction. The pasted permission choices in Prompt 2
do not explicitly select an allowance. No download or workaround was attempted
during this audit. Dependency permission alone would not satisfy the required
implementation, verification, owning-repository merges, and accepted pins.

## Work performed and resume gate

Only local compass documentation changed during this audit. No implementation,
fresh worktree, installation, build, commit, push, PR, merge, publication, or hub
pin update was performed. Existing dirty hub and module work was preserved.
Earlier test/package results remain historical; no new UI verification is claimed.

Hub `git submodule status --recursive` returned leading `-` entries for all
five modules. `git diff --exit-code --submodule=short` exited 1 for existing
dirty hub/module work. Composition is not clean.

Resume Phase 1 in Core, then SDK, Language Server, and editor services; verify
each owning repository and record immutable dependency commits. Complete review
and authorized merges, then verify accepted pins. Only after that gate passes,
start Prompt 2 in fresh owning-repository worktrees based on current origin/main.
Prompt 2 authorizes edits, verification, commits, pushes, and PRs, but explicitly
withholds merge, publication, release, and repository/registry administration.

## Remaining publication and registry gates

- Complete and verify Phase 1 and Prompt 2; obtain owning-repository merge
  authorization and accepted immutable dependency pins.
- Run the requested new UI, security, accessibility, multi-root, fake-installer,
  package/archive, isolated extension-host, and populated native dashboard
  checks; generate new SBOMs and checksums for the reviewed commit.
- Recheck registry identities, publisher/namespace ownership, trusted publishing
  or credentials, protected environments, and release workflow readiness. The
  [earlier registry preflight](release-0.0.2-preflight.md) is historical and was
  not refreshed by this source-prerequisite audit.
- Obtain explicit authorization for any registry/repository administration,
  release creation, or publication. No packages or extensions may be published
  under the current request.
- Update hub submodule pins and `docs/repository-split.md` last, after the
  required owning-repository merges and composition verification.
