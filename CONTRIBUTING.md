> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# How to contribute

Create a focused branch and open a pull request against the owning repository's
default branch. This hub accepts architecture, provenance, examples, governance,
and reviewed component-pin updates; implementation changes belong in a component.

Start with the canonical [ecosystem architecture and contributor
guide](docs/architecture.md#how-to-contribute). It defines repository ownership,
dependency order, cross-repository rules, the
[LSP-visible rule path](docs/architecture.md#adding-an-lsp-visible-rule), and the
[CI/test map](docs/architecture.md#ci-and-test-map).

1. Update and test the lowest-level affected repository first.
2. Pin its commit in dependent manifests and test those repositories.
3. Update the relevant `modules/*` gitlinks here.
4. Record external algorithm or design provenance in `docs/prior-art/`.

Clone the composed project with `--recurse-submodules`. Component-specific
commands live in each repository README. Verify the hub with
`git submodule status --recursive`, `git diff --check`, and, from a clean
composition, `git diff --exit-code --submodule=short`. See the
[repository-split runbook](docs/repository-split.md) before updating a gitlink.

## Licensing contributions

Contributions intentionally submitted to this repository are provided under
MPL-2.0. You must have the necessary rights to submit the work. When Covered
Software is distributed, modifications to MPL-covered files remain subject to
the Source Code Form obligations in the license.
