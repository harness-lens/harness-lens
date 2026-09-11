<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# `harness-inspect` minimal retirement plan

## Decision

`harness-inspect` is not a public command, repository, crate, npm package, PyPI
package, or installer. Harness inspection remains part of the existing
`harness-lens scan` command.

The replacement surface is:

```text
harness-lens scan [PATH]
harness-lens scan [PATH] --graph --json
harness-lens scan [PATH] --graph --graph-root NODE_OR_PATH --graph-max-hops N --json
```

## Rationale

- `scan` already owns deterministic workspace analysis and report production.
- A second verb would duplicate behavior and require permanent compatibility
  rules across Rust, TypeScript, documentation, installers, and editor commands.
- Graph generation and traversal depth are capabilities of the scan result, not
  a separate engine or product boundary.
- Keeping one binary and one command identity reduces packaging and support
  ambiguity.

## Cleanup

1. Treat the unified governance/observability plan as authoritative wherever an
   older plan proposes `harness-lens inspect`.
2. Do not create or publish `harness-inspect` placeholders in any registry.
3. Use `scan --graph` in new issues, examples, protocol names, tests, and editor
   commands.
4. If implementation ever shipped an `inspect` alias, retain it only for a
   documented deprecation window; no such compatibility alias is needed for a
   plan-only name.
5. Keep the name, if desired, only in a private administrative watch list. Name
   monitoring does not create a public artifact or product commitment.

## Reconsideration gate

Reconsider the name only if a future capability has a distinct input contract,
release cadence, security boundary, and user workflow that cannot be expressed
as a bounded `scan` option. A difference based only on “deeper analysis” is not
sufficient.

## Done when

- the canonical plan contains no public `inspect` command or package;
- planned CLI and editor work consistently uses `scan --graph`;
- no repository or registry placeholder was created for the retired name; and
- issue bodies do not introduce the name again.
