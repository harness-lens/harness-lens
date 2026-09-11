> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Publishing Harness Lens

This umbrella repository is not an artifact publisher. Releases here identify a
reviewed ecosystem composition: the exact component commits recorded as Git
submodules. Packages are built and published only by their owning repositories.

| Artifact | Owner | Release documentation |
| --- | --- | --- |
| Rust core | [core](https://github.com/harness-lens/core) | repository README/workflows |
| Rust SDK, config, integrations, report store, Python wheel/sdist | [sdk](https://github.com/harness-lens/sdk) | [publishing guide](https://github.com/harness-lens/sdk/blob/main/docs/publishing.md) |
| Terminal renderer, native and npm CLI | [cli](https://github.com/harness-lens/cli) | [publishing guide](https://github.com/harness-lens/cli/blob/main/docs/publishing.md) |
| Rust and npm language server | [language-server](https://github.com/harness-lens/language-server) | [publishing guide](https://github.com/harness-lens/language-server/blob/main/docs/publishing.md) |
| npm VS Code API and VSIX | [harness-lens-vscode](https://github.com/harness-lens/harness-lens-vscode) | [publishing guide](https://github.com/harness-lens/harness-lens-vscode/blob/main/docs/publishing.md) |

## Release order

1. Publish `harness-lens-core`.
2. Publish SDK config/integration/store crates, then the Rust SDK and Python package.
3. Publish the terminal renderer before its CLI consumer, and publish the language server.
4. Package and publish the VS Code extension against reviewed server binaries.
5. Update all hub gitlinks to the released commits and create an umbrella GitHub
   release describing the compatible set.

The existing TypeScript implementations retain their own npm order: core before
SDK, CLI, and language server; the reusable VS Code package before the extension.

## Trust controls

- Use registry trusted publishing/OIDC where supported; do not store long-lived
  publication tokens in repositories.
- Protect npm, PyPI, crates.io, and Marketplace releases with repository
  environments and required review.
- Attach checksums, provenance, and an SBOM to production native releases.
- Never publish from this hub to work around a failed component check.
- Treat registry versions as immutable and update all downstream pins after any
  rewritten/squashed source commit.

See the [repository split](repository-split.md) for current dependency pins and
merge order, and the [registry map](registry-and-administration.md) for ownership
and administration checks.
