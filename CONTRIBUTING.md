> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Contributing

Create a focused branch and open a pull request against the owning repository's
default branch. This hub accepts architecture, provenance, examples, governance,
and reviewed component-pin updates; implementation changes belong in a component.

1. Update and test the lowest-level affected repository first.
2. Pin its commit in dependent manifests and test those repositories.
3. Update the relevant `modules/*` gitlinks here.
4. Record external algorithm or design provenance in `docs/prior-art/`.

Clone the composed project with `--recurse-submodules`. Component-specific
commands live in each repository README. Verify the hub with
`git submodule status --recursive` and `git diff --check`.

## Licensing contributions

Contributions intentionally submitted to this repository are provided under
MPL-2.0. You must have the necessary rights to submit the work. When Covered
Software is distributed, modifications to MPL-covered files remain subject to
the Source Code Form obligations in the license.
