<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Preview 0.0.2 release preflight

Verified at 2026-09-06T00:11:38.561276+00:00. Resumed codex-third session
`01a0731c-6dc9-7461-83d8-84822b46b9c2`. No commits, pushes, merges, tags,
registry publication, or hub pin changes performed.

## Reviewed artifacts

Directory: `build/compass-vscode/artifacts/` (relative to hub root).
VSIX identity: `harness-lens.harness-lens`, version `0.0.2`, preview enabled.
VSIX size: 162025 bytes.

```text
9822c2dee54c15c2e11f8c1643fd7578d6b5d62030277476a678cb7bb29cfac0  harness-lens-extension.cdx.json
7b56a56cae56ab9c281caef1f801449941d6e0696c7a540280b510004e526f9b  harness-lens-vscode-0.0.2.tgz
560e1a279695629585705d00588087672cfd20da75b4fd0a8fd0399f952fe5b0  harness-lens-vscode.cdx.json
da9b52351c0e176a3dfb7539bcc011b4dc5c02d01bf779f8eaa70e307380d2bb  harness-lens.vsix
```

These are local checksums, not GitHub provenance attestations. CI will generate
its own artifacts and attestations after source commits and release setup.
VSIX byte reproducibility was not asserted; both SBOMs were generated twice
with byte-identical output from the same installed dependency tree.

## Changes during this resume

In the isolated VS Code owning-repository worktree:

- Fixed packaged README installation/licensing links. VSCE previously rewrote
  relative links to invalid `blob/HEAD/../../` URLs.
- Added `GH_REPO` to release asset upload, whose job has no Git checkout.
- Changed npm publication to use the reviewed tarball downloaded from the
  packaging job instead of repackaging workspace sources.
- Added downloaded checksum verification before npm and Open VSX publication.
- Documented local checksum generation and exact-artifact publication.

## Fresh verification

- Node `24.18.0`, npm `11.16.0`; `npm run check` and `npm test`: pass.
  Both discovery and extension test files passed. Dependencies were already
  installed by the recovered session; this resume did not repeat `npm ci`.
- `npm run package`: pass after README correction; VSIX contains exactly eight
  expected files. CRC, identity, preview flag, manifest, bundle, icon, license,
  changelog, and documentation links checked. No test files or source maps
  included in VSIX.
- npm tarball contains seven files; every byte matches its source/build file.
  npm package intentionally includes its declared distribution source maps.
- `npm run sbom`: pass twice; reproducible CycloneDX 1.6 output.
  npm SBOM has one selected component; extension SBOM has eleven selected
  components. Both retain workspace metadata as their root component.
  No absolute local workspace/home paths found.
- `sha256sum --check SHA256SUMS` from artifact directory: all four pass.
- Release YAML parses and local structure checks pass; `git diff --check`
  passes in VS Code worktree. GitHub Actions itself was not executed.
- Native LSP: `node scripts/smoke-native-lsp.mjs rust/target/debug/harness-lens-lsp`
  passes in `build/compass-language-server`: HL032 evidence/related location,
  content-free workspace report with per-file metrics, and clearing duplicate
  diagnostics after unsaved edit.
- VS Code `1.136.1` accepts VSIX in isolated profile under
  `build/compass-editor-smoke`. Extension-host smoke passes activation,
  real AGENTS.md discovery, command registration, metrics webview opening,
  and refresh with unavailable server. Result:
  `build/compass-editor-smoke/extension-host-result.json`.
  Initial smoke asserted tab visibility before VS Code delivered its tab
  event; corrected to bounded wait and reran successfully.
- Editor smoke disables language server in its fixture. Native Linux LSP and
  Windows editor were checked separately; end-to-end populated dashboard with
  native Windows server remains unverified. VS Code logs a pre-existing
  resource-configuration warning for `harnessLens.languageServer.enabled`.

Sandboxed CycloneDX npm subprocess returned an empty version; reviewed
escalation resolved it. Registry API calls similarly required network access.
No approval rejection occurred.

## Live registry preflight

- npm registry: only `@harness-lens/vscode@0.0.1`; latest tag is `0.0.1`.
  Source: <https://registry.npmjs.org/@harness-lens%2fvscode>.
- Marketplace API: only `0.0.1`, flags `validated, public, unpublished`.
  Source: POST to
  <https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery>
  with extension identity filter `harness-lens.harness-lens`.
- Open VSX extension API returns 404:
  <https://open-vsx.org/api/harness-lens/harness-lens>.
  This does not establish namespace ownership or publisher-agreement status.
- GitHub repository has zero release environments, zero repository Actions
  variables, and zero releases. `OPEN_VSX_PUBLISH_ENABLED` is absent.
  Sources: `gh api repos/harness-lens/harness-lens-vscode/environments`,
  `.../actions/variables`, and `.../releases`.
- No `VSCE_PAT`, `OVSX_PAT`, `AZURE_CLIENT_ID`, `AZURE_TENANT_ID`, or
  `AZURE_FEDERATED_TOKEN_FILE` is set in this process. Credential-store and
  browser sessions were not inspected; npm trusted publishing was not verified.

## Remaining work

1. Resolve earlier user instruction **No commit** before committing owning
   repositories. Local Core/SDK/LSP compatibility used path overrides; they
   cannot serve as release dependency pins.
2. Commit/review/merge Core first, pin immutable Core SHA in downstream SDK,
   then commit/review/merge SDK and pin downstream consumers. Verify each
   consumer independently. Prepare/review/merge VS Code changes.
3. Verify populated editor dashboard against matching native language server.
4. Configure required registry identities, trusted publication/credentials,
   and protected GitHub environments. No registry configuration was changed.
5. Publish only after source and release prerequisites pass; update hub pins
   and `docs/repository-split.md` last, as required by hub AGENTS.md.

Tool-call history (step 06) and attributed effectiveness (step 07) remain
unimplemented; package README explicitly states their absence. They must not
be represented as completed merely because preview packages build.

Hub composition remains intentionally dirty. `git submodule status --recursive`
reports leading `-` entries; `git diff --exit-code --submodule=short` exits 1.
Known unrelated `AGENTS.md:68` trailing whitespace remains. Original module
checkouts and unrelated hub changes were preserved.
