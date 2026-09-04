> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Validation rules

Rule identifiers are stable report contracts. The first implementation exposes:

| Rule | Status | Evidence |
| --- | --- | --- |
| `HL001` | implemented | presence/count of recognized harness sources |
| `HL006` | implemented | path plus safe read/size failure detail; never file content |
| `HL010` | implemented | byte span of the second adjacent, case-insensitively equal word |
| `HL020` | implemented heuristic | both locations in an exact normalized `always/never` or `must/must not` pair |

`HL001` is produced by the built-in inventory plugin. `HL006` is produced by the
filesystem adapter when a recognized source exceeds the configured size cap or
cannot be read as UTF-8.

`HL010` ignores Markdown fenced code and treats Unicode alphanumeric runs as
words. Its score is binary and deterministic; it does not invent a fractional
penalty. `HL020` also ignores fenced code, strips simple Markdown list/header
markers, normalizes case and whitespace, and reports a conflict only when the
remaining modal target is exactly equal. It is labeled heuristic because textual
normalization is not proof of semantic contradiction. Differently worded or
model-inferred conflicts are intentionally outside this rule. A pair must also
have overlapping harness scopes (the same scope or an ancestor/descendant
relationship); separate sibling subtrees do not conflict.

Broader structural, reference, hierarchy-aware semantic conflict, and security
rules remain design material until each has deterministic semantics and focused
tests. A future rule must document its evidence, false-positive boundary,
severity, and whether it is deterministic, heuristic, statistical, or
probabilistic.
