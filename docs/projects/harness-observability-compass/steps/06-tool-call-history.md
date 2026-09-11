<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step 06 — tool-call history

Owner: runtime contract first; presentation later.

Store only sanitized observations: tool name, safe category, status, duration,
retry count, attributed cost, stable error class, model, skill/config identity,
revision, and time window. Never store raw arguments, outputs, transcripts,
secrets, credentials, or stderr.

Expose aggregate success, error, timeout, retry, cancellation, latency, and
cost views. State sample size and method. Add filters without unbounded report
payloads.

## Done when

Fixtures prove stable error grouping, pagination/bounds, safe serialization, and
failure isolation. Static mode still works when no trace exists.
