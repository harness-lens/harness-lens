> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# HarnessLens

HarnessLens inspects coding-agent harness files in a repository. The first package is a small, functional preview: it discovers supported instruction files and provides a stable package and CLI foundation for the planned validator, metrics, history, and TUI.

## Installation

```bash
python -m pip install harness-lens
```

## Usage

Inspect the current repository:

```bash
harness-lens .
```

Emit JSON for scripts:

```bash
harness-lens . --json
```

The preview recognizes:

- `AGENTS.md`
- `CLAUDE.md`
- `GEMINI.md`
- `.github/copilot-instructions.md`
- files under `.cursor/rules/`

It searches nested repository directories while ignoring common generated and dependency directories.

## Project status

Version `0.0.1` is pre-alpha. It implements discovery only. Planned validation rules, metrics, historical comparisons, AI interpretation, and TUI are documented in [design decisions](docs/design-decisions.md).

The registered Rust packages `harness-lens`, `harness-lens-core`, and
`harness-lens-cli` live in the canonical [Rust workspace](rust/README.md).
Their initial releases reserved the namespaces; functional Rust development
continues under the repository-wide MPL-2.0 policy.

## Development

```bash
python -m pip install -e ".[dev]"
python -m pytest
python -m build
python -m twine check --strict dist/*
```

Release setup lives in [publishing documentation](docs/publishing.md). The
cross-registry ownership and administration inventory lives in the [registry
and administration map](docs/registry-and-administration.md).

## License

Early namespace-reservation versions used BSD-3-Clause. The official functional
implementation is licensed under MPL-2.0. When Covered Software is distributed,
modified MPL-covered files must remain available in Source Code Form under the
license. See [LICENSING](LICENSING.md), [COPYRIGHT](COPYRIGHT), and
[TRADEMARKS](TRADEMARKS).

## Authors

HarnessLens was created by [cristiancmrg](https://github.com/cristiancmrg) and is maintained with contributions from the HarnessLens community.
