> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Ecosystem architecture and contributor guide

Harness Lens uses multiple implementation repositories because its analysis
model serves more than one language and ecosystem. Separation makes ownership,
release cadence, and dependency direction explicit. The umbrella repository
keeps design material and pins known-compatible revisions as submodules.

This file is the canonical ecosystem map. Repository `CONTRIBUTING.md` files
link here for cross-repository rules and keep only their local commands and
review requirements. Repository behavior, manifests, workflows, and internal
docs remain authoritative when this overview and implementation disagree.

## Dependency chain

```text
                              harness-lens hub
                        pins five source submodules
                                  |
                     core (rules and report contracts)
                       |                         |
              sdk / adapters             TypeScript consumers
                 |       |                import core directly
                 |       +---------------------------+
                 |                                   |
          native CLI                    native language server
                 |                                   |
        release archives                 stdio LSP + UTF-16
                 |                          /                 \
          Homebrew tap                  VS Code          Visual Studio

CodeBurn aggregates -> harness-metrics crate -> optional language-server evidence
```

Arrows describe dependency or delivery flow, not repository ownership. Rust
SDK manifests pin Core by immutable Git revision. Native CLI and language server
manifests pin SDK, which carries Core transitively. TypeScript SDK, CLI, and
language server packages depend directly on `@harness-lens/core`. Editor adapters
launch `harness-lens-lsp`; they do not import or copy analysis rules.

The hub composes `core`, `sdk`, `cli`, `language-server`, and
`harness-lens-vscode` as Git submodules. Homebrew and Visual Studio are release
consumers, not hub submodules. `harness-metrics` is a registry dependency of the
language server and a separately bounded optional integration; it is not part
of the [public organization repository inventory](https://github.com/orgs/harness-lens/repositories)
observed on September 8, 2026.

### Where compatibility is recorded

Do not copy a revision from this overview. Read the owning immutable record:

| Relationship | Authoritative record |
| --- | --- |
| Hub to source composition | committed gitlinks under [`modules/`](../modules/) and URL/update hints in [`.gitmodules`](../.gitmodules) |
| Rust SDK to Core | [`sdk/rust/Cargo.toml`](https://github.com/harness-lens/sdk/blob/main/rust/Cargo.toml) and `Cargo.lock` |
| Native CLI to SDK | [`cli/rust/Cargo.toml`](https://github.com/harness-lens/cli/blob/main/rust/Cargo.toml) and `Cargo.lock` |
| Native language server to SDK and `harness-metrics` | [`language-server/rust/Cargo.toml`](https://github.com/harness-lens/language-server/blob/main/rust/Cargo.toml) and `Cargo.lock` |
| TypeScript packages to Core | each repository's `package.json` and lockfile |
| VS Code release to native language server | [release workflow](https://github.com/harness-lens/harness-lens-vscode/blob/main/.github/workflows/release.yml) immutable checkout input and generated checksums |
| Visual Studio to bundled native language server | [`SOURCE_REVISION`](https://github.com/harness-lens/harness-lens-visualstudio/blob/main/native-server/SOURCE_REVISION), [`SHA256SUMS`](https://github.com/harness-lens/harness-lens-visualstudio/blob/main/native-server/SHA256SUMS), resolver constant, tests, and package audit |
| Homebrew formula to native CLI release | [CLI native-release workflow](https://github.com/harness-lens/cli/blob/main/.github/workflows/native-release.yml), generated formula PR, and [tap CI](https://github.com/harness-lens/homebrew-tap/blob/main/.github/workflows/ci.yml) |

## Repository ownership and navigation

| Layer | Repository or package | Owns | Must not own |
| --- | --- | --- | --- |
| Composition | [harness-lens](https://github.com/harness-lens/harness-lens) | this architecture, governance, prior art, examples, compatible component pins | component implementation or duplicate packages |
| Domain | [core](https://github.com/harness-lens/core) | models, evidence, normalized scores, statistical helpers, plugin/report ports, analysis rules | filesystem, transport, editor, model vendor |
| Application/adapters | [sdk](https://github.com/harness-lens/sdk) | safe discovery, TOML, Rust/Python/TypeScript facades, integration mappings, bounded report-store adapters | terminal or editor presentation |
| Runtime adapter | [`harness-metrics` 0.0.4](https://docs.rs/crate/harness-metrics/0.0.4) | CodeBurn aggregate schema, evidence methods, document correlation | session parsing, pricing rules, raw transcript persistence, LSP presentation |
| Terminal | [cli](https://github.com/harness-lens/cli) | arguments, reusable report rendering, output, exit behavior | analysis rules |
| Protocol | [language-server](https://github.com/harness-lens/language-server) | LSP state, overlays, UTF-8-to-UTF-16 conversion | analysis rules or VS Code APIs |
| VS Code editor | [harness-lens-vscode](https://github.com/harness-lens/harness-lens-vscode) | server lifecycle, selectors, commands, views, npm/VSIX packaging | duplicate analysis or provider execution without consent |
| Visual Studio editor | [harness-lens-visualstudio](https://github.com/harness-lens/harness-lens-visualstudio) | Visual Studio lifecycle, host policy, settings, commands, VSIX, pinned native-server bundle | analysis rules or unverified native binaries |
| macOS distribution | [homebrew-tap](https://github.com/harness-lens/homebrew-tap) | reviewed Homebrew formula and macOS install verification | CLI source, hand-written release URLs, or hand-written checksums |

Use these repository-local guides before changing a component:

| Repository | Contributor runbook | Design and operation docs |
| --- | --- | --- |
| Hub | [How to contribute](../CONTRIBUTING.md) | [repository split](repository-split.md), [integrations](integrations.md), [publishing](publishing.md), [prior art](prior-art/README.md) |
| Core | [How to contribute](https://github.com/harness-lens/core/blob/main/CONTRIBUTING.md) | [architecture](https://github.com/harness-lens/core/blob/main/docs/architecture.md), [rule index](https://github.com/harness-lens/core/blob/main/docs/rules.md), [provider contracts](https://github.com/harness-lens/core/blob/main/docs/provider-contracts.md) |
| SDK | [How to contribute](https://github.com/harness-lens/sdk/blob/main/CONTRIBUTING.md) | [provider services](https://github.com/harness-lens/sdk/blob/main/docs/provider-services.md), [Rust workspace](https://github.com/harness-lens/sdk/blob/main/rust/README.md), [publishing](https://github.com/harness-lens/sdk/blob/main/docs/publishing.md) |
| CLI | [How to contribute](https://github.com/harness-lens/cli/blob/main/CONTRIBUTING.md) | [distribution](https://github.com/harness-lens/cli/blob/main/docs/distribution.md), [identity and integrations](https://github.com/harness-lens/cli/blob/main/docs/identity-and-integrations.md), [publishing](https://github.com/harness-lens/cli/blob/main/docs/publishing.md) |
| Language server | [How to contribute](https://github.com/harness-lens/language-server/blob/main/CONTRIBUTING.md) | [protocol](https://github.com/harness-lens/language-server/blob/main/docs/protocol.md), [native server](https://github.com/harness-lens/language-server/blob/main/rust/README.md), [publishing](https://github.com/harness-lens/language-server/blob/main/docs/publishing.md) |
| VS Code | [How to contribute](https://github.com/harness-lens/harness-lens-vscode/blob/main/CONTRIBUTING.md) | [ecosystem boundary](https://github.com/harness-lens/harness-lens-vscode/blob/main/docs/ecosystem.md), [manual installation](https://github.com/harness-lens/harness-lens-vscode/blob/main/docs/manual-installation.md), [publishing](https://github.com/harness-lens/harness-lens-vscode/blob/main/docs/publishing.md) |
| Visual Studio | [How to contribute](https://github.com/harness-lens/harness-lens-visualstudio/blob/main/CONTRIBUTING.md) | [architecture](https://github.com/harness-lens/harness-lens-visualstudio/blob/main/docs/architecture.md), [protocol contract](https://github.com/harness-lens/harness-lens-visualstudio/blob/main/docs/protocol-contract.md), [testing](https://github.com/harness-lens/harness-lens-visualstudio/blob/main/docs/testing.md), [packaging](https://github.com/harness-lens/harness-lens-visualstudio/blob/main/docs/packaging.md) |
| Homebrew tap | [How to contribute](https://github.com/harness-lens/homebrew-tap/blob/main/CONTRIBUTING.md) | [tap operation](https://github.com/harness-lens/homebrew-tap/blob/main/README.md), [CLI distribution owner](https://github.com/harness-lens/cli/blob/main/docs/distribution.md) |

## Architecture rules

1. Put behavior in its lowest owning layer. Core owns findings and scores; SDK
   owns loading and adapters; interfaces own presentation and lifecycle.
2. Keep dependencies inward. Core never imports filesystem formats, transports,
   editors, providers, agent frameworks, or product SDKs.
3. Keep deterministic output authoritative. Optional AI or runtime evidence may
   supplement a completed report but cannot rewrite deterministic findings or
   scores.
4. Normalize scores to `[0.0, 1.0]` and derive pass/fail from the recorded
   threshold. Label deterministic, heuristic, statistical, and probabilistic
   methods. Record assumptions, sample size, prior, and interval method where
   applicable.
5. Keep safety failures separate from quality averages. Isolate plugin/provider
   failures as observable execution output instead of crashing the whole scan.
6. Never serialize raw source, provider output, subprocess arguments, stderr,
   secrets, or credentials into reports. Persist only bounded, content-safe
   evidence and provenance.
7. Core findings use UTF-8 byte spans. LSP adapters alone convert them to the
   negotiated protocol positions; current editor contracts use UTF-16.
8. Use stable rule and protocol identifiers. Add evidence, false-positive
   boundaries, method labels, docs, and regression tests in the owning PR.
9. Pin cross-repository Rust dependencies and bundled native binaries to
   immutable revisions. Regenerate lockfiles and hashes in their owning repos.
   Use normal GitHub links for navigation, never floating branches for builds.
10. Generate distribution metadata from reviewed artifacts. Homebrew URLs and
    checksums come from CLI release output; Visual Studio bundled-server revision,
    hash, tests, and provenance change together.
11. Record adopted or rejected external algorithms under hub `docs/prior-art/`
    with source links, licensing, assumptions, and intentional differences.
12. Update the hub submodule pins and repository-split record only after owning
    changes merge and downstream repositories pass independently.

## How to contribute

1. Identify the lowest owning repository from the table above. Open the feature
   branch and implementation PR there; do not prototype owned behavior in hub.
2. Read that repository's contributor runbook and linked design docs. State
   whether new analysis is deterministic, heuristic, statistical, or
   probabilistic before coding it.
3. Add focused unit/contract tests in the owner. Run every command listed for
   that repository in [CI and test map](#ci-and-test-map).
4. Merge the owner, record its immutable commit, then update direct consumers.
   Regenerate each changed Cargo lockfile and verify each consumer separately.
5. Test adapter contracts at boundaries: serialized report shape, UTF-8/UTF-16
   positions, related locations, unsaved overlays, process failure, trust, and
   content-safe output as applicable.
6. Update component docs and contributor links in their owning repos. Update
   this guide, prior-art records, hub gitlinks, and `docs/repository-split.md`
   last.

Parallel review is fine; merge and pin order still follows dependency order:

```text
Core
  then SDK
    then CLI -> Homebrew/release consumers
    then language server -> VS Code and Visual Studio adapters
finally hub composition
```

For a documentation-only rollout whose downstream files link to a new section
in this guide, merge the canonical hub guide first, then merge repository-local
links. Implementation and pin changes still update the hub last.

### Cross-repository change routing

| Change | Start here | Then verify/update |
| --- | --- | --- |
| Finding, score, rule, report model, plugin contract | Core | SDK pin and adapters; CLI/LSP pins; affected editor rendering; hub last |
| Harness discovery, TOML, Python/PyO3, provider service | SDK | CLI and language server; affected editors; hub last |
| Report-store contract or local storage backend | SDK | actual consumers and bounds; hub last |
| Arguments, exit codes, terminal output, native archives | CLI | package-manager generators, Homebrew tap, container, hub last |
| Reusable terminal renderer | CLI | native CLI consumer, package verification, hub last |
| LSP lifecycle, custom request, diagnostic mapping | Language server | VS Code and Visual Studio protocol clients; bundled binaries; hub last |
| VS Code command, view, setting, packaging | VS Code | VSIX/npm release docs and installer tests; hub last |
| Visual Studio command, setting, lifecycle, packaging | Visual Studio | protocol matrix and pinned native-server records; hub docs if contract changes |
| Formula generation | CLI | generated Homebrew PR and both macOS CI jobs |
| Formula policy or tap docs | Homebrew tap | CLI distribution docs only if producer contract changes |
| Architecture, governance, provenance, composition | Hub | no implementation copied into hub |

## Adding an LSP-visible rule

An LSP rule starts in Core. Language server already maps generic `Finding`
objects to diagnostics; editors consume those diagnostics. Most new rules need
no editor-specific implementation.

1. Reserve a stable `HL...` code in Core's
   [rule index](https://github.com/harness-lens/core/blob/main/docs/rules.md).
   Document severity, method, evidence, assumptions, false-positive boundary,
   and reference implementation.
2. Implement the Rust reference rule as a Core `Plugin`. Add a focused module
   under `core/rust/src/` when the rule has distinct parsing/state; otherwise
   extend its existing rule family. Register default-enabled plugins in
   `core/rust/src/engine.rs`.
3. Emit `Finding` with stable code, safe message/evidence, relative source path,
   one-based line when known, and valid UTF-8 `TextSpan`. Use `related` locations
   for cross-file or earlier-source evidence. Never embed raw source in report
   evidence.
4. Add colocated Rust tests for positive, negative, boundary, Unicode, scope,
   and determinism cases. Add TypeScript compatibility behavior and
   `core/test/core.test.mjs` coverage only when npm compatibility requires it.
5. Run Core's full Rust, MSRV, TypeScript, package, and whitespace gates. If the
   rule derives from external work, add/update a hub prior-art note.
6. Merge Core. Pin its immutable SHA in `sdk/rust/Cargo.toml`, regenerate
   `sdk/rust/Cargo.lock`, and run SDK Rust/TypeScript/Python gates. Merge SDK.
7. Pin the accepted SDK SHA in `language-server/rust/Cargo.toml`, regenerate its
   lockfile, and run Rust tests on Linux and Windows plus
   `scripts/smoke-native-lsp.mjs`.
8. Change language-server mapping only when the generic contract is insufficient.
   Relevant paths are `rust/src/lib.rs::diagnostic_from_finding`,
   `related_information`, and `range_from_byte_span`; TypeScript compatibility
   uses `src/diagnostics.ts`. Test severity, code, evidence, UTF-16 conversion,
   related URIs, open/closed files, unsaved changes, and diagnostic clearing.
9. Do not reimplement rule logic in VS Code or Visual Studio. Update an editor
   only for a new protocol capability, presentation contract, bundled server,
   or fixture. Verify Problems/Error List navigation and package integrity.
10. Update hub component pins and this guide last, after all affected owning CI
    is green.

Runtime `HM...` diagnostics follow a different owner path: runtime adapter
defines content-safe evidence; language server owns conversion/presentation;
editors consume protocol output. Do not allocate `HL...` codes for provider-only
runtime findings.

## CI and test map

Local commands below mirror current workflow intent. Workflow files remain the
source of truth for runner versions, permissions, release environments, and
artifact retention.

| Repository | Local contributor checks | CI organization |
| --- | --- | --- |
| Hub | `git submodule status --recursive`; `git diff --check`; after a clean composed checkout, `git diff --exit-code --submodule=short` | [`ecosystem.yml`](../.github/workflows/ecosystem.yml) checks recursive submodules and clean composition; [`codeql.yml`](../.github/workflows/codeql.yml) scans Actions |
| Core | `npm ci`; `npm test`; `npm run check`; `npm pack --dry-run`; in `rust/`: `cargo fmt --check`, `cargo clippy --all-targets --all-features --locked -- -D warnings`, `cargo test --all-features --locked`, `cargo package --locked` | [CI](https://github.com/harness-lens/core/blob/main/.github/workflows/ci.yml): TypeScript on Node 20/22/24, Rust stable, Rust 1.85.1 MSRV; [CodeQL](https://github.com/harness-lens/core/blob/main/.github/workflows/codeql.yml); [npm release](https://github.com/harness-lens/core/blob/main/.github/workflows/publish.yml) |
| SDK | `npm ci`; `npm test`; `npm run check`; in `rust/`: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`; Python editable install and `python -m pytest` | [CI](https://github.com/harness-lens/sdk/blob/main/.github/workflows/ci.yml): Node 20/22/24, Rust workspace, Python 3.10/3.14; [CodeQL](https://github.com/harness-lens/sdk/blob/main/.github/workflows/codeql.yml); [npm](https://github.com/harness-lens/sdk/blob/main/.github/workflows/publish.yml) and [PyPI](https://github.com/harness-lens/sdk/blob/main/.github/workflows/publish-to-pypi.yml) releases |
| CLI | `npm ci`; `npm test`; `npm run check`; in `rust/`: formatting, locked Clippy/tests, `cargo run --locked -- --version`; run Go/C/C++ placeholder checks when touched | [CI](https://github.com/harness-lens/cli/blob/main/.github/workflows/ci.yml): Node matrix, Rust, non-root container, Go/C/C++; [CodeQL](https://github.com/harness-lens/cli/blob/main/.github/workflows/codeql.yml); [native distribution](https://github.com/harness-lens/cli/blob/main/.github/workflows/native-release.yml); [npm release](https://github.com/harness-lens/cli/blob/main/.github/workflows/publish.yml) |
| Language server | `npm ci`; `npm test`; `npm run check`; in `rust/`: formatting, locked Clippy/tests/build; run `node ../scripts/smoke-native-lsp.mjs target/debug/harness-lens-lsp` | [CI](https://github.com/harness-lens/language-server/blob/main/.github/workflows/ci.yml): Node 20/22/24, Rust on Linux/Windows, native JSON-RPC smoke; [CodeQL](https://github.com/harness-lens/language-server/blob/main/.github/workflows/codeql.yml); [npm release](https://github.com/harness-lens/language-server/blob/main/.github/workflows/publish.yml) |
| VS Code | `npm ci`; `npm run check`; `npm test`; `npm run package` | [CI](https://github.com/harness-lens/harness-lens-vscode/blob/main/.github/workflows/ci.yml): package tests/artifacts and Windows installer fixture; [CodeQL](https://github.com/harness-lens/harness-lens-vscode/blob/main/.github/workflows/codeql.yml); [checksum/SBOM/attestation release chain](https://github.com/harness-lens/harness-lens-vscode/blob/main/.github/workflows/release.yml) |
| Visual Studio | Windows PowerShell: `./scripts/verify.ps1`; `./scripts/package.ps1`; then Experimental Instance matrix in `docs/testing.md` when host behavior changes | [Windows CI](https://github.com/harness-lens/harness-lens-visualstudio/blob/main/.github/workflows/ci.yml): pinned .NET 8 SDK, locked restore/tests/native LSP smoke/VSIX audit, candidate artifact |
| Homebrew tap | When formula exists: `brew readall --os=all --arch=all`, `brew style`, `brew audit --strict`, `brew install`, `brew test`, version smoke | [Homebrew CI](https://github.com/harness-lens/homebrew-tap/blob/main/.github/workflows/ci.yml): native Apple Silicon and Intel macOS jobs; empty tap remains a visible pending-first-release state |

Test placement follows ownership:

- Core rule units live beside [Rust rule modules](https://github.com/harness-lens/core/tree/main/rust/src);
  [TypeScript compatibility tests](https://github.com/harness-lens/core/blob/main/test/core.test.mjs)
  cover the npm surface.
- SDK Rust workspace tests cover discovery/config/provider adapters; [Python
  tests](https://github.com/harness-lens/sdk/tree/main/tests) and [TypeScript
  tests](https://github.com/harness-lens/sdk/tree/main/test) cover their facades.
- [CLI tests](https://github.com/harness-lens/cli/tree/main/test) cover commands,
  rendering, and release generation; Rust command behavior stays in crate tests;
  [package placeholders](https://github.com/harness-lens/cli/tree/main/placeholders)
  keep native tests.
- Language-server unit tests cover protocol conversion in
  [`rust/src/`](https://github.com/harness-lens/language-server/tree/main/rust/src);
  [native JSON-RPC smoke](https://github.com/harness-lens/language-server/blob/main/scripts/smoke-native-lsp.mjs)
  covers lifecycle; [TypeScript tests](https://github.com/harness-lens/language-server/tree/main/test)
  cover compatibility.
- Editor repositories test lifecycle/model/policy code without copying Core rule
  cases. See [VS Code tests](https://github.com/harness-lens/harness-lens-vscode/tree/main/packages)
  and [Visual Studio tests](https://github.com/harness-lens/harness-lens-visualstudio/tree/main/tests).
  Packaged-host and installer checks complement unit tests.
- Distribution repositories test generated artifacts and installation; they do
  not test scanner semantics already owned by Core/SDK/CLI.

Lower layers never import upper layers. Model providers and agent frameworks
belong in optional adapter repositories/packages. They receive generic reports;
core never receives OpenAI, Anthropic, Ollama, LangChain, or similar types.

## Core contracts

`HarnessSource` contains loaded UTF-8 input for in-process analysis.
`SourceRecord` is the content-free report form. `Finding` carries rule, severity,
location, a protocol-neutral UTF-8 byte span, and safe evidence. `Metric`
preserves a raw measurement. `Score` normalizes interpretation to `[0.0, 1.0]`,
derives pass/fail from a threshold, and records method, sample size, uncertainty,
evidence, and producer.

`quality_mean` includes only quality scores. Safety, reliability, and performance
remain separate dimensions, so a severe safety failure or unstable execution
cannot disappear inside a broad average.

## Plugins

Rust plugins implement the generic `Plugin` contract and are registered by a
host. Core provides immutable config and sources, collects output, measures
execution time, and records disabled, unavailable, failed, or completed status.
Configured but unavailable plugins do not abort other analysis.

Initial plugins compile in-process. Dynamic library loading is deferred until a
stable ABI, trust policy, signatures, permissions, and crash isolation exist.
Future out-of-process plugins can preserve the contract through a versioned
protocol.

## Configuration and Python

`harness-lens.toml` is versioned. Core owns generic typed values; the SDK owns
TOML parsing and resolution: explicit caller path, repository-local file, then
built-in defaults. Plugin and integration option maps remain opaque to core.

The SDK's Maturin build produces `harness_lens._native` through PyO3 and the
Python 3.10 stable ABI. A small discovery fallback keeps source-tree Python use
available before native compilation; installed wheels use Rust.

## Discovery and editor boundary

The SDK traverses in deterministic order, skips configured generated/dependency
directories, applies a file-count fuse, and prevents followed symlinks from
escaping the canonical workspace. Incomplete scans are explicit, content-free
report data.

The language server overlays open editor buffers and maps core byte spans to
UTF-16 LSP ranges. It publishes `source = harness-lens` and stable `HL...` codes.
The VS Code extension launches the server; Problems, Error Lens, and any other
diagnostic consumer can render results without a private integration.

Optional runtime evidence follows a separate inward dependency. Harness Metrics
consumes CodeBurn aggregate JSON and emits method-labeled UTF-8 document
insights. The language server owns subprocess lifecycle, caching, refresh, and
presentation. Core findings and scores never depend on CodeBurn availability.

## Cross-repository reproducibility

Cargo manifests pin upstream Git revisions and retain registry version
constraints. This lets dependent feature branches build before release while
preserving publishable dependency metadata. The hub's five submodule gitlinks
pin its source composition independently of `.gitmodules` branch hints.
Homebrew and Visual Studio remain separate release consumers whose formula
artifacts or bundled native-server revision and hashes provide their immutable
links to upstream source.

See [repository split](repository-split.md), [Harness Evals prior art](prior-art/harness-evals.md),
[CodeBurn prior art](prior-art/codeburn.md), and [integration boundaries](integrations.md).
