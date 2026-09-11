<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step 07 — attributed effectiveness

Owner: Core measurement contract, then SDK/runtime adapters.

Compare baseline and current windows for cost, stability, latency, retries, and
outcomes. Attribute a change to a file, skill, or config only when identity,
revision, comparison window, and evidence are present.

Report `insufficient evidence` when sample size or attribution is missing. Use
improving, stable, or degrading only through declared thresholds. Correlation
does not prove that one edit caused an outcome.

## Done when

Before/after fixtures expose method, sample size, uncertainty, and assumptions;
safety failures remain separate; no invented per-file score appears.
