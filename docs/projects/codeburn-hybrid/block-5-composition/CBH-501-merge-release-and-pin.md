<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-501: Merge components, prepare releases, and pin SHAs

**Repositories:** language-server, then harness-lens-vscode
**Suggested labels:** `release`, `dependencies`, `integration`
**Estimate:** 0.5–1 day
**Depends on:** CBH-401 and CBH-402

## Outcome

Land independently verified component changes in dependency order and produce
immutable revisions for the composition hub.

## Work

1. Merge the language-server change with Harness Metrics `version = "0.0.3"`
   and the full reviewed Harness Metrics Git `rev`.
2. Regenerate and review `Cargo.lock`; ensure no local path or patch remains.
3. Verify the language server from a fresh clone, then record its merge SHA.
4. Update and merge the VS Code client against the final initialization
   contract; verify its npm tarball and VSIX from a fresh clone.
5. Record release-candidate artifact checksums and component compatibility.
6. Publish language-server crate/npm or Marketplace artifacts only under their
   owning release procedures and only after explicit maintainer authorization.

## Acceptance criteria

- [ ] Every Git dependency uses a full immutable SHA.
- [ ] No downstream manifest references a local Harness Metrics checkout.
- [ ] Lockfiles resolve from public sources in clean environments.
- [ ] Language-server and editor pull requests link CBH-401/CBH-402 evidence.
- [ ] Merge SHAs for Harness Metrics, language server, and VS Code are posted.
- [ ] Release notes state CodeBurn is optional, external, and disabled by
      default.
- [ ] No irreversible publication is inferred from merge approval.

## Non-goals

- Updating hub submodules or `docs/repository-split.md`; CBH-502 owns that last
  step.
- Floating dependencies to a branch or tag.

## Verification

Run every command documented by both component repositories from clean clones.
If a squash merge changes an expected SHA, regenerate downstream pins and rerun
all dependent checks before closing.
