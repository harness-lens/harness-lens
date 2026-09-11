> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Registry and administration map

Last administrative audit: 2026-08-28. Public package inventory re-audited:
2026-09-11.

This document records the public Harness Lens identities, their administrative
entry points, and the remaining namespace and release work. It intentionally
contains no credentials. Store private keys, tokens, recovery codes, and
webhook secrets in an approved secret manager.

## Current identity inventory

| Platform | Identity | Current state | Public page | Administration |
| --- | --- | --- | --- | --- |
| GitHub | Organization `harness-lens` | Active | <https://github.com/harness-lens> | <https://github.com/organizations/harness-lens/settings/profile> |
| GitHub App | `Harness Lens` / `harness-lens` | Registered as a minimal organization-owned app; permissions and webhooks intentionally disabled until an integration service exists | Not public while the app is private | <https://github.com/organizations/harness-lens/settings/apps> |
| PyPI | `harness-lens` | Public at `0.0.1`; `letalboy` is the only visible Owner; Trusted Publisher active | <https://pypi.org/project/harness-lens/> | <https://pypi.org/manage/project/harness-lens/settings/publishing/> |
| npm | Scope `@harness-lens` | Active; CLI at `0.0.5`, four other packages at `0.0.1`; each lists only `cristiancmrg` as maintainer | <https://www.npmjs.com/org/harness-lens> | <https://www.npmjs.com/settings/harness-lens/packages> |
| crates.io | Nine Harness Lens crates | Core/SDK/config/adapter at `0.0.2`; CLI/LSP/terminal/store at `0.0.1`; Metrics at `0.0.4`; each owned only by `cristiancmrg` | <https://crates.io/users/cristiancmrg> | Manage with `cargo owner` and the crates.io account settings |
| Visual Studio Marketplace | Publisher `harness-lens` | Active | <https://marketplace.visualstudio.com/publishers/harness-lens> | <https://marketplace.visualstudio.com/manage/publishers/harness-lens> |
| VS Code Marketplace | `harness-lens.harness-lens` | `0.0.1` validated and unpublished; `0.0.2` Preview prepared and locally verified but not uploaded | <https://marketplace.visualstudio.com/items?itemName=harness-lens.harness-lens> | <https://marketplace.visualstudio.com/manage/publishers/harness-lens> |

The five npm packages are:

- <https://www.npmjs.com/package/@harness-lens/core>
- <https://www.npmjs.com/package/@harness-lens/cli>
- <https://www.npmjs.com/package/@harness-lens/sdk>
- <https://www.npmjs.com/package/@harness-lens/language-server>
- <https://www.npmjs.com/package/@harness-lens/vscode>

The nine Harness Lens crates are:

- <https://crates.io/crates/harness-lens-core>
- <https://crates.io/crates/harness-lens-config>
- <https://crates.io/crates/harness-lens-adapter-harness-score>
- <https://crates.io/crates/harness-lens>
- <https://crates.io/crates/harness-lens-cli>
- <https://crates.io/crates/harness-lens-lsp>
- <https://crates.io/crates/harness-lens-terminal>
- <https://crates.io/crates/harness-lens-store>
- <https://crates.io/crates/harness-metrics>

The cross-registry package, version, owner, and source-repository intersections
are recorded in the [package registry ownership matrix](package-registry-matrix.md).

## GitHub organization panels

- Organization profile: <https://github.com/organizations/harness-lens/settings/profile>
- Members and teams: <https://github.com/orgs/harness-lens/people>
- Member privileges: <https://github.com/organizations/harness-lens/settings/member_privileges>
- Organization GitHub Apps: <https://github.com/organizations/harness-lens/settings/apps>
- Installed GitHub Apps: <https://github.com/organizations/harness-lens/settings/installations>
- Organization repositories: <https://github.com/orgs/harness-lens/repositories>
- Organization packages: <https://github.com/orgs/harness-lens/packages>

For every repository, review these panels by replacing `REPOSITORY`:

- Settings: `https://github.com/harness-lens/REPOSITORY/settings`
- Branch and ruleset protection: `https://github.com/harness-lens/REPOSITORY/settings/branches`
- Environments: `https://github.com/harness-lens/REPOSITORY/settings/environments`
- Actions: `https://github.com/harness-lens/REPOSITORY/actions`
- Releases: `https://github.com/harness-lens/REPOSITORY/releases`
- Security configuration: `https://github.com/harness-lens/REPOSITORY/settings/security_analysis`

Apply the review to `harness-lens`, `core`, `cli`, `sdk`, `language-server`,
and `harness-lens-vscode`.

## Repository split transition

Implementation ownership moved to the five component repositories described in
[`repository-split.md`](repository-split.md). Before the next publication:

- move the PyPI Trusted Publisher from repository `harness-lens` and workflow
  `publish-to-pypi.yml` to repository `sdk` with the same workflow name;
- configure crates.io publication in the repository that owns each crate;
- keep npm trusted publishers attached to their existing component repositories;
- publish VSIX/Marketplace artifacts only from `harness-lens-vscode`;
- remove obsolete publishing environments from this hub after the new owners are
  verified.

The hub may create GitHub releases that describe compatible gitlink revisions,
but it must not republish component artifacts.

## Confirmed control gaps

The public APIs showed these gaps during the 2026-08-28 audit:

- `harness-lens` has the legacy protected `pypi` environment; recreate and verify
  it in `sdk` before the next Python release.
- `core`, `cli`, `sdk`, `language-server`, and `harness-lens-vscode` exposed no
  GitHub environments yet. Create an `npm` environment with required reviewers
  in every repository that publishes npm packages.
- Create a `marketplace` environment in `harness-lens-vscode` when an automated
  Marketplace publishing job is implemented. The current workflow only builds
  and attaches the VSIX.
- Each Harness Lens crate currently has only the personal owner
  `cristiancmrg`. Add an organization team owner for continuity after the team
  and permissions are reviewed. Do not create a one-member team: it adds no
  continuity. Reconsider only after a second trusted maintainer with MFA and
  recovery access is selected.
- npm Trusted Publisher details, GitHub App credentials, organization recovery
  methods, and Marketplace role assignments are not public. Verify those
  manually in the linked administration panels.

## Ownership and release controls

- Require at least two organization owners with phishing-resistant MFA and
  recovery access documented outside GitHub.
- Grant registry access through organization or team identities where the
  registry supports them; avoid a single personal owner.
- Use GitHub environments with required reviewers for `pypi`, `npm`, and
  `marketplace` publication jobs.
- Prefer OIDC Trusted Publishing. Do not add long-lived PyPI or npm tokens when
  the registry supports trusted publishing.
- Confirm npm Trusted Publisher configuration separately for all five npm
  packages; configuration is per package.
- Add an appropriate GitHub organization team as a crates.io owner when the
  team and least-privilege policy are ready. Verify current owners with
  `cargo owner --list CRATE` before changing them.
- Record the GitHub App ID and installation ownership in the organization
  inventory, but never record its private key or client secret in Git.
- Keep the GitHub App private and permissionless until its authentication,
  callback, webhook validation, data retention, and privacy design exist.

## Marketplace safety

- Use `preview: true` while the extension exposes only early functionality.
- State available and unavailable features at the top of the Marketplace
  README.
- Use **Unpublish** to stop downloads while preserving the extension record.
- Never use **Remove** as a temporary rollback. Removal is irreversible and
  permanently prevents reuse of the extension identity.
- Every replacement VSIX needs a new SemVer version. Inspect and locally install
  the exact artifact before upload.
- Azure DevOps global PAT retirement is scheduled for 2026-12-01. Use Microsoft
  Entra workload identity federation and Azure Pipelines before automating
  Marketplace publication.

## Additional places to evaluate

These are conditional, not immediate requirements:

| Place | When it becomes relevant | Recommended action |
| --- | --- | --- |
| Open VSX | Supporting VSCodium, Eclipse Theia, or other Open VSX clients | Register the `harness-lens` namespace and publish the reviewed VSIX-equivalent package. |
| GitHub Marketplace | Offering the GitHub App to accounts outside the organization | Wait until installation, permissions, webhooks, privacy policy, support, and data handling are implemented. |
| GitHub Container Registry | Shipping a hosted service or containerized scanner | Publish under `ghcr.io/harness-lens/*` with provenance and retention policies. |
| Homebrew tap | Shipping a stable native or self-contained macOS CLI | Create `harness-lens/homebrew-tap` only when signed release artifacts exist. |
| WinGet, Scoop, or Chocolatey | Shipping a stable Windows CLI | Register manifests only after release URLs, checksums, upgrade behavior, and signing are stable. |
| Documentation domain | Hosting callbacks, webhook endpoints, documentation, or a service UI | Register and protect the domain, enable renewal and MFA, and document DNS ownership. |
| Package signing and provenance | Publishing production releases | Retain attestations, immutable checksums, SBOMs, and registry provenance where supported. |

## Audit cadence

Review this map before every new distribution channel and at least quarterly.
Verify owners, required reviewers, trusted publishers, recovery access, public
descriptions, support links, privacy links, package versions, and abandoned
credentials. Update the audit date only after checking the live services.

## Platform references

- GitHub App registration and permissions: <https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app>
- PyPI Trusted Publishers: <https://docs.pypi.org/trusted-publishers/>
- npm Trusted Publishers: <https://docs.npmjs.com/trusted-publishers/>
- Cargo package ownership: <https://doc.rust-lang.org/cargo/commands/cargo-owner.html>
- VS Code extension manifest and Preview flag: <https://code.visualstudio.com/api/references/extension-manifest>
- VS Code publishing and unpublishing: <https://code.visualstudio.com/api/working-with-extensions/publishing-extension>
