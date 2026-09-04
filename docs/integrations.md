> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Integration boundaries

## Language servers and editors

[`harness-lens-lsp`](https://github.com/harness-lens/language-server) consumes
the [Rust SDK](https://github.com/harness-lens/sdk) and maps generic findings to standard
LSP diagnostics. It owns UTF-8-byte to UTF-16-position conversion and in-memory
document overlays, but no analysis rules. Editor packages own process lifecycle
and document selectors. Presentation extensions such as Error Lens consume the
same diagnostics without becoming Harness Lens dependencies.

## Harness Score

[`harness-lens-adapter-harness-score`](https://github.com/harness-lens/sdk/tree/main/rust/crates/harness-lens-adapter-harness-score)
maps `AnalysisReport` to a content-safe
`HarnessScoreDocument`. It deliberately does not select an endpoint,
authentication scheme, HTTP library, or proprietary SDK. A host supplies
`HarnessScoreTransport` after the public ingestion contract is confirmed.
The mapped document preserves scan completeness so partial discovery cannot be
mistaken for an authoritative score.

This keeps three decisions independent:

1. Core computes evidence, raw metrics, and normalized scores.
2. Adapter maps the report into the target product schema.
3. Transport handles network, credentials, retries, and deployment policy.

The hub's example config includes a disabled `harness-score` integration entry. Enabling
it before a transport is installed must produce an explicit unavailable status
in the future integration runner; it must never silently discard reports.

## Model providers and agent frameworks

Future AI interpretation adapters accept a serialized `AnalysisReport` and
return suggestions in a separate namespace. They cannot rewrite deterministic
findings, raw metrics, or scores. Each provider and framework stays optional and
outside core.
