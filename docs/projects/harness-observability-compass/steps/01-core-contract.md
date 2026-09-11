<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step 01 — core contract

Owner: `modules/core` repository.

Define versioned, provider-neutral records for:

- harness asset and safe provenance;
- per-file measurement and inclusion edge;
- finding and normalized score;
- runtime observation and safe error class;
- method, assumptions, sample size, and uncertainty.

Keep presentation fields out of Core. Test serialization for absence of raw
source, credentials, arguments, outputs, transcripts, and stderr.

## Done when

CLI, LSP, and editor consume same report shape; deterministic fields remain
valid with runtime disabled; upstream tests and lint pass.
