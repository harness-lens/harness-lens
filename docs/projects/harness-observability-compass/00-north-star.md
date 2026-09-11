<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# North star

Harness Lens helps a developer understand whether a coding-agent harness is
getting cheaper, more stable, and more effective over time.

Every useful statement should answer four questions:

- What happened: finding, cost, error, retry, latency, or trend.
- How certain: deterministic, heuristic, statistical, or probabilistic method.
- What caused the context: harness, skill, config, referenced file, or tool.
- Where to inspect: exact safe file and source range when available.

## Product shape

GitLens-like navigation shows workspace assets, skills, references, findings,
context consumption, runtime history, and comparisons. A focused dashboard
shows trends and distributions. Tree navigation remains useful without runtime
data; runtime views state when evidence is unavailable.

## Boundary

Static analysis is local-first and deterministic. Runtime capture is optional,
consent-controlled, and never changes deterministic findings or scores. Raw
source, tool arguments, outputs, transcripts, stderr, secrets, and credentials
never enter reports.
