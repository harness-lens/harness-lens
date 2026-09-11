<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# CBH-402: Audit privacy, dependency, and licensing claims

**Repositories:** harness-metrics, language-server, harness-lens-vscode, hub
**Suggested labels:** `security`, `privacy`, `licensing`, `documentation`
**Estimate:** 0.5 day
**Depends on:** CBH-203 and CBH-302

## Outcome

Confirm implementation and package metadata tell the same truthful story about
what runs, what is stored, and which project owns each result.

## Acceptance criteria

- [ ] CodeBurn is named as an optional external runtime dependency in every
      surface that can enable `live` capture.
- [ ] CodeBurn's MIT license and upstream URL are present in the Harness Metrics
      third-party notice and hub prior-art note.
- [ ] No CodeBurn source or data file is bundled in crate, wheel, npm tarball, or
      VSIX unless a later reviewed issue explicitly changes that decision.
- [ ] Cargo/PyPI/npm manifests contain only real package dependencies; they do
      not pretend the executable is an ecosystem library.
- [ ] Core, SDK, and CLI dependency graphs contain no CodeBurn or Harness
      Metrics edge introduced by this project.
- [ ] Snapshot/report serializers cannot include harness source or raw session
      contents through known fields.
- [ ] Raw stderr, environment values, credentials, and complete JSON payloads
      are not persisted or sent as diagnostics.
- [ ] Estimated values, evidence methods, bases, and sample sizes remain visible.
- [ ] `off` is still the default in server code, extension manifest, examples,
      and docs.

## Non-goals

- Providing legal advice or claiming certification.
- Auditing CodeBurn's entire upstream implementation.
- Changing MPL-2.0 ownership boundaries.

## Verification

Inspect built artifact contents and dependency trees, not only source manifests.
Post the commands, artifact names/checksums, findings, assumptions, and any
follow-up issues. A maintainer signs off before CBH-501.
