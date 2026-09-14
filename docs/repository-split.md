> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Repository split

This record explains the September 4, 2026 migration from the temporary Rust
monorepo branch into the existing Harness Lens organization repositories. It is
both a maintenance guide and an audit trail for why each boundary exists.

## Migration map

| Former path in this repository | Owning repository/path | Migration PR | Initial pinned commit |
| --- | --- | --- | --- |
| `rust/crates/harness-lens-core` | `core/rust` | [core#4](https://github.com/harness-lens/core/pull/4) | [`fef8ef34`](https://github.com/harness-lens/core/commit/fef8ef34a0ab16ca9cea93ad7e35e63e57c52ccf) |
| `rust/crates/harness-lens`, `harness-lens-config`, `harness-lens-python`, `harness-lens-adapter-harness-score`; `src/harness_lens`; `tests` | `sdk/rust`, `sdk/src/harness_lens`, `sdk/tests` | [sdk#4](https://github.com/harness-lens/sdk/pull/4) | [`9b6dd078`](https://github.com/harness-lens/sdk/commit/9b6dd0784d49c3d6d11d902e9d54139a01196d77) |
| `rust/crates/harness-lens-cli` | `cli/rust` | [cli#5](https://github.com/harness-lens/cli/pull/5) | [`86b57714`](https://github.com/harness-lens/cli/commit/86b5771435c226be606c9ae5a91c1f8715cb748a) |
| `rust/crates/harness-lens-lsp` | `language-server/rust` | [language-server#5](https://github.com/harness-lens/language-server/pull/5) | [`a5cd63bf`](https://github.com/harness-lens/language-server/commit/a5cd63bf8d0ce84b336baade9c0e0d49ec83b904) |
| planned VS Code client | `harness-lens-vscode/packages/extension` | [harness-lens-vscode#7](https://github.com/harness-lens/harness-lens-vscode/pull/7) | [`84a29f3f`](https://github.com/harness-lens/harness-lens-vscode/commit/84a29f3fd30cb1bec1912cef8369ac845466357a) |

The target repositories already contained useful TypeScript packages. The split
preserves them as compatibility implementations instead of deleting published
npm surfaces. Rust is the reference engine for the new core, SDK, native CLI,
and language server.

## Composition updates

### Workspace observability — September 5, 2026

The first editor observability slice implements the architecture contract in
[harness-lens#11](https://github.com/harness-lens/harness-lens/issues/11).
These revisions were verified in dependency order:

| Component | Capability | Delivery PR | Hub pin |
| --- | --- | --- | --- |
| Core | Per-file bytes, estimated tokens, configured input cost, findings, score methods, and plugin execution | [core#14](https://github.com/harness-lens/core/pull/14) | [`4b8f248b`](https://github.com/harness-lens/core/commit/4b8f248b33e6110e581388703046160de56f82f9) |
| SDK | Content-safe filesystem report adapter used by LSP | [sdk#21](https://github.com/harness-lens/sdk/pull/21) | [`6d927ee1`](https://github.com/harness-lens/sdk/commit/6d927ee1e0fc50a47c47a91f3c007f976f0e49ff) |
| Language server | Versioned `harnessLens/workspaceReport`, unsaved overlays, and native protocol coverage | [language-server#20](https://github.com/harness-lens/language-server/pull/20), [#21](https://github.com/harness-lens/language-server/pull/21) | [`8f376550`](https://github.com/harness-lens/language-server/commit/8f376550b19b6d5b4be773cd46b34780172d0125) |
| VS Code | Activity Bar tree, metrics center, per-file navigation, content-free history, and explicit trend method | [harness-lens-vscode#20](https://github.com/harness-lens/harness-lens-vscode/pull/20) | [`e6a94af3`](https://github.com/harness-lens/harness-lens-vscode/commit/e6a94af3b37f32bc3ee81e257c25bbcb4e7990b2) |

Per-file effectiveness and tool-call error, retry, timeout, and cost history are
not inferred from static findings. The editor marks them unmeasured until Core
and runtime adapters provide sanitized attributed evidence.

### Cargo dependency train — September 11, 2026

The Rust packages were released from their owning repositories in dependency
order. Every package candidate was verified after merge using registry-only
dependencies; repository builds retain immutable Git pins.

| Component | Published artifact | Delivery PR | Accepted commit |
| --- | --- | --- | --- |
| Core | `harness-lens-core 0.0.2` | [core#26](https://github.com/harness-lens/core/pull/26) | [`2a8e916f`](https://github.com/harness-lens/core/commit/2a8e916fcf73cbeb1792358b5726e23472792721) |
| SDK | `harness-lens-config 0.0.2`, `harness-lens-adapter-harness-score 0.0.2`, `harness-lens 0.0.2` | [sdk#32](https://github.com/harness-lens/sdk/pull/32) | [`c11b8683`](https://github.com/harness-lens/sdk/commit/c11b8683e9c021ee8bc8befb94623ce825146f4b) |
| Language server | `harness-lens-lsp 0.0.1` | [language-server#32](https://github.com/harness-lens/language-server/pull/32) | [`be825fd4`](https://github.com/harness-lens/language-server/commit/be825fd4ac337b2517086c85fc22a3498cd81ade) |
| CLI | SDK `0.0.2` consumer pin; no publication | [cli#56](https://github.com/harness-lens/cli/pull/56) | [`b241c817`](https://github.com/harness-lens/cli/commit/b241c817da76738229c56a64bc77e68c7f69aeb5) |
| SDK reusable seed | `harness-lens-store 0.0.1` | [sdk#33](https://github.com/harness-lens/sdk/pull/33) | [`3202c2e6`](https://github.com/harness-lens/sdk/commit/3202c2e6c118904681d4eba92873b1a33b3c438c) |
| CLI reusable seed | `harness-lens-terminal 0.0.1`; CLI `0.0.5` remains unpublished | [cli#57](https://github.com/harness-lens/cli/pull/57) | [`8e111090`](https://github.com/harness-lens/cli/commit/8e111090cb0069544dae9b3878f9f9b77334d309) |

`harness-metrics 0.0.4` remains the published optional runtime adapter consumed
by LSP. Its restored source and ownership-continuity work are tracked separately;
no retroactive tag or replacement archive was created.

### Observed-flow token lens and bounded findings — September 13, 2026

The observed-flow stack now carries optional per-turn token evidence from Core
through the editor without deriving missing usage from static file estimates.
The VS Code client adds a synchronized token lens, responsive Sankey layout,
source navigation, local-history search, and ten-row pagination for history and
warning/error findings.

All dependency pins below use post-squash commits from the owning repositories.
Each downstream repository regenerated its lockfiles where applicable and passed
its independent CI matrix before this hub update.

| Component | Capability | Delivery PR | Hub pin |
| --- | --- | --- | --- |
| Core | Validated optional per-turn token usage with explicit measured/estimated provenance and missing-evidence preservation | [core#33](https://github.com/harness-lens/core/pull/33) | [`1f1d4531`](https://github.com/harness-lens/core/commit/1f1d453111f385859d0afe2c8126f2e36cf30938) |
| SDK | Bounded token timeline aligned to retained observed-flow transitions; post-squash Core repin | [sdk#36](https://github.com/harness-lens/sdk/pull/36), [#37](https://github.com/harness-lens/sdk/pull/37) | [`9af01a3e`](https://github.com/harness-lens/sdk/commit/9af01a3e9480289c6808b583fb632c979748609e) |
| Language server | Versioned token timeline response, UTF-16 evidence locations, and post-squash SDK/Core pins | [language-server#35](https://github.com/harness-lens/language-server/pull/35), [#36](https://github.com/harness-lens/language-server/pull/36) | [`2b7d66b0`](https://github.com/harness-lens/language-server/commit/2b7d66b0d05961d3556e1cc39fd9f8f56fa7f1b4) |
| VS Code | Responsive Sankey, synchronized token lens, evidence filenames/navigation, history search/pagination, warning pagination, and immutable release LSP pin | [harness-lens-vscode#37](https://github.com/harness-lens/harness-lens-vscode/pull/37), [#38](https://github.com/harness-lens/harness-lens-vscode/pull/38) | [`25cfc1fb`](https://github.com/harness-lens/harness-lens-vscode/commit/25cfc1fbe193c69824d8af30d3bc4ab50814c86a) |

### npm release recovery — September 14, 2026

VS Code release `0.0.5` retained its immutable GitHub assets while npm
publication was recovered independently. The owning repository now treats npm
tarballs as explicit local files, verifies every release asset against
`SHA256SUMS`, and supports a registry-only recovery path. npm trusted publishing
uses GitHub OIDC without a classic token or generated `_authToken` placeholder.

| Component | Capability | Delivery PR | Hub pin |
| --- | --- | --- | --- |
| VS Code | Unambiguous local npm publication, checksum-verified recovery, single-use verification runbook, and incident record | [harness-lens-vscode#41](https://github.com/harness-lens/harness-lens-vscode/pull/41), [#42](https://github.com/harness-lens/harness-lens-vscode/pull/42), [#43](https://github.com/harness-lens/harness-lens-vscode/pull/43) | [`da8bc0ad`](https://github.com/harness-lens/harness-lens-vscode/commit/da8bc0ad72c3b8c7526ceb37f96d52ece4ed26b2) |

## Dependency pins

- This composition pins Core commit `1f1d4531` and SDK commit `9af01a3e`;
  the SDK manifest and lockfile pin that same merged Core revision.
- The language server pin `2b7d66b0` consumes SDK `9af01a3e` and transitively
  Core `1f1d4531` from regenerated locked dependencies.
- This composition pins CLI commit `8e111090`, which adds the Terminal seed and
  continues to consume its independently verified SDK revision.
- CLI publication remains controlled by its independently reviewed supervised
  release runbook.
- The VS Code pin `da8bc0ad` executes `harness-lens-lsp` through standard
  input/output and its release workflow builds immutable language-server
  revision `2b7d66b0` rather than importing analysis. It also recovers npm
  publication only from checksum-verified immutable release assets.
- This hub pins every commit above through `modules/*` gitlinks.

Cargo dependencies include compatible registry versions and `git`/`rev`.
Repository builds therefore use exact source revisions. When packaged, Cargo
removes Git location data and retains the registry version requirements.

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
