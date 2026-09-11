<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Asset and namespace reservations

## Status

Audited on 2026-09-10 and updated with package/repository execution status on
2026-09-11. This record is subordinate to
[`UNIFIED-PLAN.md`](UNIFIED-PLAN.md): a reserved name does not create a product,
package, release boundary, or implementation commitment.

The safe reservation mechanism is a private GitHub incubation repository with
an honest status README and an MPL-2.0 license. Public package registries and
marketplaces receive only useful, tested artifacts; they are not used as
placeholder databases.

## Completed GitHub reservations

| Repository | State | Justified boundary | Promotion gate |
| --- | --- | --- | --- |
| [`harness-inspector`](https://github.com/harness-lens/harness-inspector) | Private; README and MPL-2.0 license; implementation issue [#1](https://github.com/harness-lens/harness-inspector/issues/1) | Deterministic policy-gate orchestration over accepted Core and SDK contracts | Core policy contract, fixtures, threat model, verification, and a usable offline MVP |
| [`go`](https://github.com/harness-lens/go) | Private; functional report facade at `e4c5f9b`; local/CLI-interoperability tests and remote CI pass; no public tag | Go consumer/validator for the portable Core report contract, with no analysis-engine duplication | Explicit approval for public visibility, then create the reviewed first semantic-version tag |
| [`cpp`](https://github.com/harness-lens/cpp) | Private; README and MPL-2.0 license; no package publication | Future C ABI and thin C++ facade staged today under `cli/placeholders/cpp` | Stabilize ABI/platform contract and useful behavior before ConanCenter or vcpkg submission |
| [`harness-metrics`](https://github.com/harness-lens/harness-metrics) | Private; crates.io `0.0.4` source restored at `07445e2`; checksum, current owner, local package, MSRV, and remote CI verified; no retroactive tag | Existing optional runtime-evidence adapter boundary | Public source approval, a trusted backup named owner or organization team, and trusted publishing before a later release; retain the archive as provenance authority |
| [`harness-lens-desktop`](https://github.com/harness-lens/harness-lens-desktop) | Private; boundary README, MPL-2.0 license, and offline-viewer issue [#1](https://github.com/harness-lens/harness-lens-desktop/issues/1) | Local desktop lifecycle, permissions, installers, signing, updates, and presentation | A useful offline report viewer, least-privilege Tauri threat model, cross-platform verification, and signed release process |

These repositories are incubation boundaries, not hub submodules. A repository
becomes a hub submodule only after it owns implementation, has independent
verification, pins dependencies immutably, and is needed for reproducible
composition.

`github.com/harness-lens/go` is the only new registry-facing package currently
ready for promotion. Its initial API decodes and validates schema-version-1
reports, bounds untrusted input, exposes typed enums and content-free summaries,
and passes an interoperability test against the real local CLI. Publishing it
means making the repository public and pushing a semantic-version tag; the Go
ecosystem has no separate name-placeholder registry.

`harness-metrics` is not a new name reservation: version `0.0.4` already exists
on crates.io. The restored repository records the published archive checksum
and explicitly does not invent the missing historical commit or a retroactive
`v0.0.4` tag.

A live owner audit on 2026-09-11 lists `cristiancmrg` as the only crates.io
owner of every Harness Lens crate. The published versions allocate each crate
name while that ownership remains. The remaining namespace risk is continuity
through one account. Before a later release, add a trusted second named owner
or a `github:harness-lens:<team>` owner and configure crates.io trusted
publishing against the reviewed repository. No GitHub team currently exists in
the organization, so that team and its trusted membership must be created
deliberately rather than assumed.

Decision on 2026-09-11: do not create a one-member `release-managers` team.
Giving a team only the existing personal owner adds structure without reducing
continuity risk. Reconsider the team only after a second trusted maintainer is
selected with MFA and recovery access; then review membership before adding the
team as owner of existing crates.

## Existing controlled identities

The project already controls the GitHub `harness-lens` organization and npm
scope `@harness-lens`. The npm scope prevents unrelated publishers from
creating `@harness-lens/*` packages; an empty scoped package adds no protection.

Existing public artifacts remain authoritative:

- npm: `@harness-lens/core`, `@harness-lens/sdk`, `@harness-lens/cli`,
  `@harness-lens/language-server`, and `@harness-lens/vscode`;
- crates.io: `harness-lens-core 0.0.2`, `harness-lens-config 0.0.2`,
  `harness-lens-adapter-harness-score 0.0.2`, `harness-lens 0.0.2`,
  `harness-lens-cli 0.0.1`, `harness-lens-lsp 0.0.1`, and
  `harness-metrics 0.0.4`;
- PyPI: `harness-lens`;
- editor identities: the existing VS Code/Visual Studio Marketplace identity,
  the VS Code repository, the Visual Studio repository, and
  `harness-lens.nvim`;
- terminal distribution: CLI releases, Homebrew tap, Scoop bucket, and WinGet
  manifest repository.

## Availability audit without publication

The following checks found no published project at the time of the audit. This
is an observation, not a permanent guarantee and not authorization to publish.

| Registry | Names checked | Result | Decision |
| --- | --- | --- | --- |
| npm | `@harness-lens/inspector`, `@harness-lens/patternizer`, `@harness-lens/doctor`, `@harness-lens/analyzer` | Unpublished; local npm authentication returned `401` | Do not publish placeholders. The controlled scope already protects these spellings. |
| PyPI | `harness-inspector`, `harness-patternizer`, `harness-doctor`, `harness-analyzer` | Public JSON endpoints returned `404` | Do not claim the names until a useful Python facade and protected publisher exist. |
| crates.io | `harness-inspector`, `harness-patternizer`, `harness-doctor`, `harness-analyzer`, `harness-lens-graph`, `harness-lens-trace` | `cargo search` returned no exact result; the HTTP API returned `403` to this environment | Recheck before a real release. Do not publish name-only crates. |

An exact `cargo info` audit initially found `harness-lens-config`,
`harness-lens-adapter-harness-score`, and `harness-lens-lsp` absent. The project
then completed the owner-first dependency train and published useful, verified
artifacts on 2026-09-11. The immutable Core, SDK, and LSP merge SHAs and package
order are recorded in
[`MODULARIZATION-HORIZON.md`](MODULARIZATION-HORIZON.md). No placeholder version
was published.

The official npm policy says package names are intended for immediate, active
use and considers packages without genuine function to be squatting. PyPI may
quarantine spam or policy-violating projects and deleting a project releases
its name. Cargo documents first-come allocation but requires an actual packaged
crate and recommends completing metadata and a dry run before publication.

References:

- [npm package-name and squatting policy](https://docs.npmjs.com/policies/disputes/)
- [npm open-source terms](https://docs.npmjs.com/policies/open-source-terms/)
- [PyPI Terms of Service](https://policies.python.org/pypi.org/Terms-of-Service/)
- [PyPI project-name behavior](https://pypi.org/help/)
- [Cargo publication guide](https://doc.rust-lang.org/cargo/reference/publishing.html)

## Names that do not justify independent repositories

| Name or behavior | Owner | Reason not to reserve another repository/package |
| --- | --- | --- |
| `harness-inspect` | Existing CLI `scan --graph` | Retired public name; duplicate command and package |
| Patternizer | Core | Candidate/review contracts compile accepted rules into the existing engine |
| Graph schema/projector | Core and SDK | Contract and adapter modules, not a separately released engine |
| Trace normalizer | SDK | Optional adapter boundary; split only if dependencies and release cadence prove independence |
| Doctor | CLI | A future `harness-lens doctor` can diagnose installation/configuration without becoming a library |
| Analyzer | Core/SDK and `scan` | Generic duplicate of the existing product boundary |
| TUI and shell completions | CLI | Terminal presentation and packaging already belong to CLI |
| Desktop npm library | Desktop application repository | The app is distributed as signed installers/releases; the controlled npm scope already protects any future internal web package |
| GitHub Action | Inspector or CLI owner | Package with the real gate/scanner; a badge/action needs no extra registry identity |
| Cursor, VSCodium, Theia | VS Code extension | Consume the same reviewed VSIX/Open VSX-compatible artifact |
| Generic Zed, Emacs, Helix, or Vim LSP setup | Language-server documentation or a future integration collection | Standard LSP support does not by itself justify one repository per editor |
| Report/schema model package | Core | Core already owns the authoritative report and graph contracts; another model package would create drift |
| TUI repository | CLI | TUI state and terminal rendering share CLI lifecycle and release identity |
| Renderer repository/package | Existing presentation repository initially | Publish only after two GUI hosts prove one reusable implementation and release contract |
| Plugin API repository/package | Core initially | Split only after a versioned out-of-process protocol and external plugin prove ABI, trust, and crash-isolation needs |

JetBrains, Zed, Emacs, Sublime Text, or another editor receives a dedicated
repository only when it requires a real independently packaged client with
host-specific lifecycle, settings, tests, permissions, and release cadence.
The GitHub organization controls its own future repository paths; empty public
editor repositories would not protect package-marketplace identities.

## Promotion checklist

Before changing an incubation repository to public or publishing any new
artifact:

1. Confirm that the boundary is still independent and does not duplicate Core,
   SDK, CLI, LSP, or an editor owner.
2. Recheck the live name and account ownership.
3. Replace placeholder behavior with a useful, documented contract.
4. Add fixtures, tests, security policy, threat model where applicable,
   provenance, SBOM/checksum requirements, and a release runbook.
5. Configure least-privilege trusted publishing or protected CI; never copy
   local long-lived credentials into workflows.
6. Package and smoke-test the exact candidate before publishing it.
7. Change the owning repository first, then immutable downstream pins, and the
   hub composition last.
