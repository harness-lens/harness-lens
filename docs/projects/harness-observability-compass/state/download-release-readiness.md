<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Downloadable Windows release readiness

Verified 2026-09-06. Scope: explain requirements for current public downloads;
no release, tag, registry publication, or settings change was authorized here.

## Verified state

- Owning repository `harness-lens/harness-lens-vscode` is public; Actions enabled.
- Accepted editor main: `0111e859ce1776c534cef60f32de5f9348c13ac2` (PR #25).
- Release workflow pins LSP `eacaa3e9808169d1e7a78eece563acf1a23aab3e`.
- CI on editor main passes packaging, Windows install/uninstall, and code analysis.
- Root, npm library, and extension manifests use `0.0.2`.
- No GitHub release, tag, or release-workflow run exists yet. npm contains only
  `@harness-lens/vscode@0.0.1`; `0.0.2` is available there.
- No GitHub environments or Marketplace/Open VSX enable variables are configured.
- Release immutability is disabled and is not enforced by the organization.
- Local Windows release smoke, VSIX, nine screenshots, and checksums passed;
  see `build/infrastructure-vscode/artifacts/LOCAL-BUILD.md`.

## Remaining preparation

Finalize the release notes/changelog for the Windows installer, provider UI,
reference-navigation correction, screenshots, and closed-file highlights. Keep
preview status and list the code-aware/references work as future scope.

Choose publication scope before triggering the workflow: `publish-npm` has no
opt-in condition and will attempt npm publication on every release. For a
GitHub-download-only release, add an opt-in flag to that job in an owning PR.
Alternatively, explicitly include npm and configure its trusted publisher and
protected `npm` environment according to the owning publishing guide.
The npm job is independent of asset attachment; an npm failure does not itself
prevent the download assets from being attached.

Marketplace and Open VSX already have opt-in gates; their tokens/accounts are
not prerequisites for public GitHub downloads. No separate crates.io/PyPI
publication is needed to build this pinned standalone Windows binary.

## Publication procedure

1. Merge any chosen release-preparation change and require passing CI on that
   exact commit. Keep version `0.0.2` in all manifests for tag `v0.0.2`.
2. In the owning repository's Releases page, create tag `v0.0.2` at the accepted
   release commit and publish its release notes. Publishing the release (not
   merely creating a draft or pushing a tag) triggers `release.yml`.
3. The workflow builds the VSIX/npm package and Windows server, generates
   SBOMs/provenance, creates checksums, and attaches the resulting assets.
4. Verify the VSIX, Windows ZIP, install/uninstall scripts, SBOMs, and
   `SHA256SUMS` are attached. Download them, verify hashes/attestations, and
   exercise the published installer in a disposable Windows test profile.
5. Share the public release page. A regular release can be selected as latest;
   a GitHub prerelease should use its explicit tag URL and installer `-Version
   v0.0.2`. VS Code manifest `preview: true` is separate from GitHub prerelease
   status. Do not assume the installer's `latest` default resolves a prerelease.
6. Record the accepted release/component SHAs in an isolated hub composition
   update after owning merges; preserve existing dirty module checkouts.

## Expected download assets

- `harness-lens.vsix`
- `harness-lens-lsp-windows-x64.zip` (executable and license)
- `install-harness-lens.ps1`, `uninstall-harness-lens.ps1`
- `SHA256SUMS`
- npm tarball and CycloneDX SBOM JSON files

The native executable is a separate coordinated download; the VSIX does not
bundle it. GitHub builds new release artifacts: their checksum need not equal
that of an earlier local build. Registry deployment consumes the exact
checksum-verified CI artifact without repackaging.

## Sources

- [Owning workflow](https://github.com/harness-lens/harness-lens-vscode/blob/main/.github/workflows/release.yml)
- [Owning publishing guide](https://github.com/harness-lens/harness-lens-vscode/blob/main/docs/publishing.md)
- [Creating a GitHub release](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository)
- [Release download links](https://docs.github.com/en/repositories/releasing-projects-on-github/linking-to-releases)
- [Immutable release lifecycle](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases)

The current workflow attaches assets after release publication. If release
immutability is enabled later, change to build and attach to a draft before
publication rather than keeping this lifecycle unchanged.
