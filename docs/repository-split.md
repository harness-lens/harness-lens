> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Repository split

This record explains the September 4, 2026 migration from the temporary Rust
monorepo branch into the existing Harness Lens organization repositories. It is
both a maintenance guide and an audit trail for why each boundary exists.

## Migration map

| Former path in this repository | Owning repository/path | Migration PR | Pinned commit |
| --- | --- | --- | --- |
| `rust/crates/harness-lens-core` | `core/rust` | [core#4](https://github.com/harness-lens/core/pull/4) | [`fef8ef34`](https://github.com/harness-lens/core/commit/fef8ef34a0ab16ca9cea93ad7e35e63e57c52ccf) |
| `rust/crates/harness-lens`, `harness-lens-config`, `harness-lens-python`, `harness-lens-adapter-harness-score`; `src/harness_lens`; `tests` | `sdk/rust`, `sdk/src/harness_lens`, `sdk/tests` | [sdk#4](https://github.com/harness-lens/sdk/pull/4) | [`d3131075`](https://github.com/harness-lens/sdk/commit/d31310754a7b791a4dedc0447eb3441d7219bd38) |
| `rust/crates/harness-lens-cli` | `cli/rust` | [cli#5](https://github.com/harness-lens/cli/pull/5) | [`e757fd41`](https://github.com/harness-lens/cli/commit/e757fd4196a476e9d29c4fcd0561c6409071bca2) |
| `rust/crates/harness-lens-lsp` | `language-server/rust` | [language-server#5](https://github.com/harness-lens/language-server/pull/5) | [`16ffcd69`](https://github.com/harness-lens/language-server/commit/16ffcd6951ba6886643324f9ddf1573838da85cb) |
| planned VS Code client | `harness-lens-vscode/packages/extension` | [harness-lens-vscode#7](https://github.com/harness-lens/harness-lens-vscode/pull/7) | [`84a29f3f`](https://github.com/harness-lens/harness-lens-vscode/commit/84a29f3fd30cb1bec1912cef8369ac845466357a) |

The target repositories already contained useful TypeScript packages. The split
preserves them as compatibility implementations instead of deleting published
npm surfaces. Rust is the reference engine for the new core, SDK, native CLI,
and language server.

## Dependency pins

- SDK pins core commit `fef8ef34`.
- CLI and language server pin SDK commit `d3131075`; that SDK transitively pins
  core.
- The VS Code client executes `harness-lens-lsp` through standard input/output
  and links to the language-server repository rather than importing analysis.
- This hub pins every commit above through `modules/*` gitlinks.

Cargo dependencies include both `version = "0.0.1"` and `git`/`rev`. Repository
builds therefore use an exact source revision. When packaged, Cargo removes Git
location data and retains the registry version requirement.

## Merge and release order

1. Merge [core#4](https://github.com/harness-lens/core/pull/4).
2. Merge [sdk#4](https://github.com/harness-lens/sdk/pull/4).
3. Merge [cli#5](https://github.com/harness-lens/cli/pull/5) and
   [language-server#5](https://github.com/harness-lens/language-server/pull/5).
4. Merge [harness-lens-vscode#7](https://github.com/harness-lens/harness-lens-vscode/pull/7).
5. Merge the umbrella pull request after all required component checks pass.

Prefer merge or rebase strategies that retain the pinned commits. If a PR is
squash-merged, update every downstream `rev`, regenerate the affected Cargo
lockfiles, rerun component tests, and update this hub's gitlinks before merging
the umbrella PR.

Crate publication follows dependencies: core; config/adapter crates; SDK; CLI
and language server. The SDK repository owns PyPI. Each TypeScript repository
owns its npm package. The VS Code repository owns the VSIX/Marketplace release.
This hub publishes no duplicate artifact.

## Why submodules

Submodules provide a small reproducible composition and clickable source tree
without restoring shared build ownership. A hub commit answers “which revisions
were designed and tested together?” while each repository retains its own CI,
issues, releases, permissions, and package metadata.

`.gitmodules` tracks `main` only as an update hint. The committed gitlink SHA is
authoritative; routine checkout never floats to the newest branch commit.

## Updating one component

Make and test the change in its repository. Pin any changed upstream dependency
by full commit SHA. Then update the hub intentionally:

```bash
git -C modules/core fetch origin main
git -C modules/core switch --detach <reviewed-commit>
git add modules/core
git submodule status
```

Repeat only for components whose compatibility was verified. Record significant
algorithm/provenance changes under [`docs/prior-art/`](prior-art/).
