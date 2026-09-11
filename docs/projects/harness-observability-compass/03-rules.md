<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Rules

Use these as gates for every step.

- Scores stay in `[0.0, 1.0]`; pass/fail comes from explicit threshold.
- Label each method deterministic, heuristic, statistical, or probabilistic.
- Heuristics expose evidence and assumptions.
- Statistical results expose sample size.
- Probabilistic results expose prior and interval method.
- Safety failures stay separate from quality and cost averages.
- Plugin and integration failures become observable output, not process crashes.
- Core text findings use UTF-8 byte spans; protocol adapters publish UTF-16.
- Missing, cyclic, ignored, out-of-root, and unavailable references stay visible.
- Unsaved buffers are overlays; they never rewrite filesystem history.
- Deterministic reports work when runtime integrations are disabled or missing.
- Optional runtime evidence cannot alter deterministic findings or scores.
- Reports never serialize raw source, secrets, credentials, tool arguments,
  tool output, transcripts, or raw stderr.
- Static context estimates and observed runtime cost remain visibly separate.
- Correlation never becomes a causal claim without a declared method and data.
