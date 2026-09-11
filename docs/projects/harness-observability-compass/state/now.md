<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Current checkpoint

## Current user direction

VS Code PR #25 merged as `0111e859ce1776c534cef60f32de5f9348c13ac2`.
Current request is the [download release requirements](download-release-readiness.md),
not publication itself. No GitHub release/tag exists yet; version `0.0.2` is
ready for release preparation. npm publication is currently unconditional and
needs an explicit scope decision before triggering a downloads-only release.

Future [reference/rule/workflow coverage](../steps/09-reference-coverage.md)
is saved at the user's request. Bundle it with later code-aware analysis
improvements; keep it outside the current downloadable-release scope.

## Continuation recovered on 2026-09-06

Resume source: codex-second session `01a07879-08ae-7a31-90a0-1dbde62351a2`.
It stopped after the Windows CI timeout in Language Server
[PR #24](https://github.com/harness-lens/language-server/pull/24).
The previous Phase 1 status below is historical, not the current next step.

Worktree: `build/language-server-diagnostics`, now detached at accepted merge
`eacaa3e9808169d1e7a78eece563acf1a23aab3e`. The merge tree matches reviewed head
`919e354d0379c4450824ae7e10546c087bc44436`. Commit `919e354` fixes the smoke-test predicate:
server-generated closed-file URIs and Node URIs can encode Windows drive colons
differently. Compare canonical file paths instead of URI strings. The production
closed-file highlighting change remains in preceding commit `744d853`.

Verified locally: original script reproduces the 20-second Windows timeout;
patched script passes against the same newly built Windows executable. Linux
native smoke also passes. Locked offline Clippy, formatting, 23 Rust tests,
TypeScript tests/check, and diff checks pass. Fix pushed to existing PR #24.
All seven PR checks pass at `919e354d0379c4450824ae7e10546c087bc44436`, including
Windows/Linux Rust, TypeScript on Node 20/22/24, and CodeQL. LSP PR #24 merged
on 2026-09-06; its accepted SHA is recorded above.

Hub verification: documentation links and whitespace pass. Recursive submodule
status was recorded; the composition diff check is nonzero with existing hub
documentation/submodule edits and the new prior-art index entry. Planning
documents remain local and uncommitted. Existing module edits were preserved.

VS Code [PR #24](https://github.com/harness-lens/harness-lens-vscode/pull/24)
is merged (verified through GitHub). Marketplace artifact wiring is delivered.
The previous session also delivered screenshots and Windows setup in PR #23.
VS Code release pin follow-up [PR #25](https://github.com/harness-lens/harness-lens-vscode/pull/25)
is open at `241a430b53f4f1392063fa302b63b786881c0f68`, branch
`build/merged-lsp-pin` in `build/infrastructure-vscode`. It pins accepted LSP
merge `eacaa3e`; all five CI/CodeQL checks pass. Locally, 24 editor tests,
TypeScript/version checks, package build, SBOM generation, and diff checks pass.
The merged LSP builds offline in Windows release mode and passes native smoke.

Matching local artifacts are rebuilt under
`build/infrastructure-vscode/artifacts`: VSIX, Windows executable/archive,
install/uninstall scripts, SBOMs, source revisions, and `SHA256SUMS`.
All ten checksum entries pass; VSIX retains nine screenshots. The local Windows
installation guide at `build/infrastructure-vscode/artifacts/LOCAL-BUILD.md`
contains exact commands and the check for an old explicit server-path override;
that ignored build artifact is not part of the repository.
VSIX SHA-256: `e33fb08da31e973a139d3f9987f12e2bead02d5b514ada4bb69670d193216502`.
Windows executable SHA-256: `74dc5d491834af7a4a0be82c3c59200bcebbd12025f96a0c1fad3c34813c593a`.

Next release step: accept VS Code PR #25, then update hub composition in an
isolated change that preserves the dirty module checkouts. No merge,
publication, installation, or hub pin update was performed by this continuation.
Local artifacts are not a published or GitHub-attested release.

New user case: essential `cohesion` repetition in a tag and a variable.
Saved [case and implementation plan](../../code-aware-repetition/README.md).
Original snippets and rule IDs are still missing; reconstructed fixtures are
labeled. Detector changes are planned, not implemented. Next feature slice is
the labeled baseline corpus and format-neutral Core region contract, followed
by SDK parsing. Keep whole-file prose duplicate detection and exact inline-code
distinctions. Do not broadly suppress mixed lines or files.

## Previous Phase 1 checkpoint

Status: `awaiting LSP review`; Core and SDK Phase 1 dependencies are merged;
Language Server provider protocol PR is open with all checks passing.
Last verified: 2026-09-06.
Core [PR #16](https://github.com/harness-lens/core/pull/16) merged as
`cec4902041544ef4377e97412cd9f79a9a3897b7`. Its merged tree matches reviewed
head `6f0e2e2462c1e99401511c9787501a172887622d`.

SDK [PR #23](https://github.com/harness-lens/sdk/pull/23) merged as
`d0a1968c7b555f18e9e1bdeebc2949b115f9c4ab`. It adds a compiled-in Native /
CodeBurn catalog, safe bounded detection, snapshot/off non-execution, an exact-
version declarative install preview, explicit-consent/trust checks, fixed npm
argument reconstruction, and lexical/provider facade exports. CodeBurn remains
external and was not run or installed.

SDK formatting, locked Clippy, 23 unit tests, doc tests, offline locked metadata,
and diff checks pass on stable Rust 1.97.1. PR CI passes Rust, TypeScript on Node
20/22/24, Python 3.10/3.14, and CodeQL language analysis. Cargo package
preparation reaches the existing unpublished `harness-lens-config` registry
boundary; no check was weakened. No npm/Python dependencies were installed
locally.

Language Server provider transport uses SDK `ProviderContext`, whose public
`runtime_mode` field has Core's `RuntimeMode` type. SDK did not re-export that
type, so consumers would need a forbidden direct Core dependency. SDK follow-up
[PR #24](https://github.com/harness-lens/sdk/pull/24) merged as
`4a6512b0834f6c8cfce5205bca7e8722139a06fd`. It re-exports `RuntimeMode` so
downstream adapters need no direct Core dependency.

Language Server [PR #23](https://github.com/harness-lens/language-server/pull/23)
is open at commit `f6246031fb0c2956248dd6fc22c5a09fbf32a3e0` on
`feat/provider-protocol`. It pins
SDK merge `4a6512b`, adds bounded `harnessLens/providerCatalog` and per-root
`harnessLens/providerAggregate`, enforces explicit trust/virtual policy, keeps
Native reports separate, maps sample-bearing statistical contributions, omits
cost when currency or estimated basis cannot be labeled, and retains safe stale
state by refresh generation. CodeBurn was not run or installed.

Formatting, locked Clippy, 23 Rust tests, doc tests, locked build, offline
metadata, diff checks, and native LSP smoke pass. Smoke proves Native catalog /
aggregate output and source-content absence in off mode. PR checks pass CodeQL,
Linux and Windows Rust, and TypeScript on Node 20/22/24. Editor Phase 1 and
Prompt 2 remain gated on accepted Language Server merge SHA. See the
[Phase 1 checkpoint](infrastructure-phase1.md).
User authorizes isolated owning-repository edits, verification, commits, pushes,
and PRs, but no merges, external software installation, publication, or releases.
Reviewed approval later allowed downloading pinned Cargo source dependencies for
SDK verification. `unicode-casefold` and `tinyvec` now compile locally; no Rust
toolchain or external provider was installed. Downstream Language Server and
editor infrastructure worktrees remain clean at accepted foundation commits.

## Previous foundation worktrees

Work continues in isolated component worktrees under ignored `build/`, not
original dirty `modules/` checkouts:

- `build/compass-core`: `958564df56c85ae04c84506b594900758e63ac6a` on `feat/observability-evidence-contracts`; clean.
- `build/compass-sdk`: `9dc39f3ad9d5837be4f69ba4553d39bee4630225` on `feat/observability-reference-graph`; clean.
- `build/compass-cli`: `233ba0998220186229745b8008f6da819622d463` on `build/observability-sdk-pin`; clean.
- `build/compass-language-server`: `2e238b0724e817a1f6d1bbc728f39b15d6a4035c` on `feat/observability-runtime-reports`; clean.
- `build/compass-vscode`: `09ed53529450280a221df22c86def2048f9617c3` on `feat/dedicated-observability-interface`; clean.

## Evidence

[Release preflight](release-0.0.2-preflight.md) records exact artifact hashes,
fresh commands/results, live registry checks, and limitations.

VS Code check/tests/package pass; SBOM bytes reproduce; four checksums pass;
archive audit and isolated Windows editor smoke pass. Linux native LSP smoke
passes separately.

Reviewable PR stack:

- Core [#15](https://github.com/harness-lens/core/pull/15), commit `958564d`: all checks pass.
- SDK [#22](https://github.com/harness-lens/sdk/pull/22), commit `9dc39f3`: all checks pass.
- CLI [#17](https://github.com/harness-lens/cli/pull/17), commit `233ba09`: all checks pass, including container build.
- Language Server [#22](https://github.com/harness-lens/language-server/pull/22), commit `2e238b0`: all Linux and Windows checks pass.
- VS Code [#21](https://github.com/harness-lens/harness-lens-vscode/pull/21), commit `09ed535`: all checks pass.

No merges, publication, or hub pin updates performed.

## Release continuation (superseded merge state)

Read [step 08](../steps/08-verification-and-composition.md) when resuming
release work, then the linked preflight evidence. Review and merge Core #15
first, update SDK to the accepted Core merge SHA if repository policy requires
it, then merge SDK #22. Repeat immutable-pin verification for CLI and Language
Server before their merges. Merge VS Code only after its server dependency is
accepted. Step 08 is not done.

Steps 06 (sanitized tool-call history) and 07 (attributed effectiveness) remain
unimplemented and explicitly unavailable in preview documentation. Resume
[step 06](../steps/06-tool-call-history.md) if continuing those features.

## Previous unresolved constraint (resolved externally)

Reviewable PR creation is complete. Merge authorization was not part of the
recovered scope. Release composition still requires dependency-order owning-
repository merges and accepted immutable downstream pins. Hub pins remain
unchanged until owning PRs merge. Registry setup and populated native editor
verification remain release work.
