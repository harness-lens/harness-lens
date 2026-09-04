> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# harness-lens-adapter-harness-score

Maps provider-neutral Harness Lens reports into a stable Harness Score document.
Network, authentication, and product API behavior remain behind
`HarnessScoreTransport`; no external service dependency enters the core.

This crate establishes an integration seam. A concrete Harness Score transport
will be added after its public ingestion contract is confirmed.
