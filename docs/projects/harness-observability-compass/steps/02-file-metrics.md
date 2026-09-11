<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step 02 — per-file metrics

Owner: Core first, SDK discovery second.

Each discovered file gets one safe record containing path identity, bytes,
characters, lines, inclusion depth, estimated tokens, configured input cost,
findings count, provenance links, and measurement method.

Keep static context cost separate from observed runtime cost. Token estimates
name tokenizer and basis. Missing or partial discovery cannot look complete.

Do not publish an “effectiveness” score yet. Effectiveness needs attributable
before/after runtime outcomes and an evidence threshold.

## Done when

Fixtures cover duplicate, cycle, missing, ignored, out-of-root, and unsaved
overlay cases. Raw file contents are absent from serialized reports.
