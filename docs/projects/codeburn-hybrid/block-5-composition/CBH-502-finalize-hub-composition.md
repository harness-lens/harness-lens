<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-502: Finalize the hub composition

**Repository:** composition hub
**Suggested labels:** `submodules`, `documentation`, `release`
**Estimate:** 0.5 day
**Depends on:** CBH-501
**Order constraint:** execute last

## Outcome

Make one reproducible hub revision answer which Harness Metrics, language
server, and editor commits were tested together.

## Work

1. Add `modules/harness-metrics` as a Git submodule at the reviewed source SHA.
2. Update language-server and VS Code gitlinks to their reviewed merge SHAs.
3. Add Harness Metrics to the repository-role list in `AGENTS.md` without
   changing the inward dependency rule.
4. Finalize architecture, integration, language-server, prior-art, and project
   links to describe `off`/`live`/`snapshot` behavior accurately.
5. Update `docs/repository-split.md` last with ownership, migration/release
   record, and all immutable pins.
6. Verify a recursive clean checkout and commit the composition as one coherent
   hub change.

## Acceptance criteria

- [ ] `.gitmodules` uses normal GitHub navigation URLs and branch hints only.
- [ ] Committed gitlinks, not branch hints, are the authoritative revisions.
- [ ] All six implementation repositories appear in the hub map and recursive
      submodule output.
- [ ] Documentation does not claim live capture is automatic or default.
- [ ] Prior-art records adoption, rejection, assumptions, licensing, and source
      links.
- [ ] No implementation source is added to the hub.
- [ ] A clean recursive clone resolves every submodule and has no diff.

## Verification

```bash
git submodule sync --recursive
git submodule update --init --recursive
git submodule status --recursive
git diff --check
git diff --exit-code --submodule=short
```

The final `git diff --exit-code` check runs after the composition commit in a
clean checkout. Record the hub SHA and link all component pull requests.
