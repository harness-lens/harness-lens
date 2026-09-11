> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Package registry ownership matrix

Live registry state verified on 2026-09-11. Each occupied cell names the
artifact, current version, public registry owner or maintainer, and owning
source repository. An em dash means Harness Lens has no package for that
surface in that registry.

| Reusable surface | crates.io | npm | PyPI |
| --- | --- | --- | --- |
| Core contracts and analysis | [`harness-lens-core 0.0.2`](https://crates.io/crates/harness-lens-core)<br>Owner: `cristiancmrg`<br>Source: [`core`](https://github.com/harness-lens/core) | [`@harness-lens/core 0.0.1`](https://www.npmjs.com/package/@harness-lens/core)<br>Maintainer: `cristiancmrg`<br>Source: [`core`](https://github.com/harness-lens/core) | — |
| SDK and scan facade | [`harness-lens 0.0.2`](https://crates.io/crates/harness-lens)<br>Owner: `cristiancmrg`<br>Source: [`sdk`](https://github.com/harness-lens/sdk) | [`@harness-lens/sdk 0.0.1`](https://www.npmjs.com/package/@harness-lens/sdk)<br>Maintainer: `cristiancmrg`<br>Source: [`sdk`](https://github.com/harness-lens/sdk) | [`harness-lens 0.0.1`](https://pypi.org/project/harness-lens/)<br>Owner: `letalboy`<br>Source owner: [`sdk`](https://github.com/harness-lens/sdk) |
| TOML configuration | [`harness-lens-config 0.0.2`](https://crates.io/crates/harness-lens-config)<br>Owner: `cristiancmrg`<br>Source: [`sdk`](https://github.com/harness-lens/sdk) | — | — |
| Harness Score adapter | [`harness-lens-adapter-harness-score 0.0.2`](https://crates.io/crates/harness-lens-adapter-harness-score)<br>Owner: `cristiancmrg`<br>Source: [`sdk`](https://github.com/harness-lens/sdk) | — | — |
| Report storage | [`harness-lens-store 0.0.1`](https://crates.io/crates/harness-lens-store)<br>Owner: `cristiancmrg`<br>Source: [`sdk`](https://github.com/harness-lens/sdk) | — | — |
| Command-line application | [`harness-lens-cli 0.0.1`](https://crates.io/crates/harness-lens-cli)<br>Owner: `cristiancmrg`<br>Source: [`cli`](https://github.com/harness-lens/cli) | [`@harness-lens/cli 0.0.5`](https://www.npmjs.com/package/@harness-lens/cli)<br>Maintainer: `cristiancmrg`<br>Source: [`cli`](https://github.com/harness-lens/cli) | — |
| Terminal report rendering | [`harness-lens-terminal 0.0.1`](https://crates.io/crates/harness-lens-terminal)<br>Owner: `cristiancmrg`<br>Source: [`cli`](https://github.com/harness-lens/cli) | — | — |
| Language server | [`harness-lens-lsp 0.0.1`](https://crates.io/crates/harness-lens-lsp)<br>Owner: `cristiancmrg`<br>Source: [`language-server`](https://github.com/harness-lens/language-server) | [`@harness-lens/language-server 0.0.1`](https://www.npmjs.com/package/@harness-lens/language-server)<br>Maintainer: `cristiancmrg`<br>Source: [`language-server`](https://github.com/harness-lens/language-server) | — |
| VS Code integration API | — | [`@harness-lens/vscode 0.0.1`](https://www.npmjs.com/package/@harness-lens/vscode)<br>Maintainer: `cristiancmrg`<br>Source: [`harness-lens-vscode`](https://github.com/harness-lens/harness-lens-vscode) | — |
| Runtime metrics adapter | [`harness-metrics 0.0.4`](https://crates.io/crates/harness-metrics)<br>Owner: `cristiancmrg`<br>Source: `harness-lens/harness-metrics` (private) | — | — |

## Counts and ownership

| Registry | Owned packages | Public ownership result |
| --- | ---: | --- |
| crates.io | 9 | Every crate lists only `cristiancmrg` |
| npm | 5 | Every package lists only `cristiancmrg` as maintainer; all use the controlled `@harness-lens` scope |
| PyPI | 1 | `harness-lens` lists `letalboy` as its only Owner; no organization role is present |
| **Total** | **15** | Every artifact currently depends on one visible individual registry account |

Registry ownership and source ownership are separate. The source repositories
above own implementation and release workflows. Registry identities control
the published artifacts. The PyPI `0.0.1` metadata still links the pre-split hub,
while the SDK repository owns current Python source and the next publication.

The empty cells are intentional. They do not need placeholder packages. Add a
new registry artifact only when that language has useful behavior, tests, an
owning repository, and a protected release path.

## Audit commands

The matrix was checked with public registry interfaces:

```bash
cargo search PACKAGE --limit 20
cargo owner --list PACKAGE
npm view PACKAGE name version maintainers repository.url --json
curl -fsSL https://pypi.org/pypi/harness-lens/json
```

Repeat the live audit before ownership changes or releases. Do not infer registry
roles from package author metadata.
