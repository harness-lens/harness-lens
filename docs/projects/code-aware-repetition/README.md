<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Code-aware repetition plan

Status: planned; detector changes are not implemented. Recorded 2026-09-06.

Preserve real repeated-word and whole-file redundancy findings while excluding
repetition required by code syntax. Start with the saved
[cohesion case and acceptance examples](cases/cohesion.md).

## Findings from the current implementation

The native LSP under review pins SDK `4a6512b0834f6c8cfce5205bca7e8722139a06fd`,
which pins Core `cec4902041544ef4377e97412cd9f79a9a3897b7`. The hub's older
`modules/core` checkout does not represent that entire implementation.

- Core `rust/src/text_analysis.rs`: `HL010` lowercases alphanumeric runs and
  compares adjacent pairs per line. Punctuation between runs loses its meaning.
  Fences use a Boolean toggle; inline code, tag names, and variable syntax have
  no distinct role. `HL030` handles overlapping directive intent separately.
- Core `rust/src/exact_duplicates.rs`: `HL032` compares normalized lines and
  paragraphs across the source set, subject to overlapping scopes. It skips
  fenced code, strips backticks/emphasis, and requires eight normalized bytes.
- Core `rust/src/lexical.rs`: lexical similarity is separate heuristic output;
  it skips identical normalized tokens. It is not evidence that repeated
  `cohesion` identifiers are redundant.

The tokenizer explains plausible false positives. The original two snippets
and their rule IDs have not been supplied, so their precise trigger is pending.

## Design boundary

Use one source-region description, with different policies per rule. Never
delete code from the original text or skip an entire mixed-content line.

Core owns format-neutral region contracts and repetition decisions. SDK owns
Markdown/source-format recognition and supplies regions for disk content and
unsaved overlays. This keeps a Markdown parser and language-specific syntax
dependencies outside Core. Extend the engine input with an additive entry point;
preserve existing `analyze` callers and document their legacy fallback.

Proposed regions describe original UTF-8 spans, prose/code/syntax roles,
block identity, recognition method/version, and content-free limitation codes.
They are analysis inputs, not source-bearing report fields. Validate ordering,
overlap, bounds, and UTF-8 boundaries before use. Invalid input becomes observable
analysis failure or limited coverage, never a crash or silent clean report.

1. Recognize explicit fenced/indented code and inline code spans. Track fence
   character, delimiter length, and list/blockquote nesting. Code syntax is
   opaque to adjacent-word checks; prose before and after remains eligible.
2. Recognize tag delimiters, names, and attributes separately from text nodes.
   In prompt-style XML, ordinary text inside a tag remains prose. A matching
   opening/closing tag does not make its contents exempt. Treat script/style
   bodies and explicit code examples according to their code role.
3. Handle unfenced snippets conservatively. Protect narrowly recognized
   assignment/member-access/identifier spans, including underscores and
   namespaces. An equals sign, angle bracket, uppercase word, or repeated
   identifier alone must not exempt a line. Ambiguous language inference is
   heuristic with explicit assumptions; do not add it silently to deterministic
   `HL010`. Ship explicit syntax first; calibrate inferred snippets separately.
4. Preserve barriers between prose fragments: excluding `<tag>` or inline code
   must not join previously separated words into an artificial repeated pair.
   Within a prose fragment, preserve current case-insensitive detection,
   punctuation behavior, and second-word byte span. Formatting-only emphasis
   must not hide a real pair such as `Use **use** tests.`

Use [CommonMark as the explicit Markdown boundary](../../prior-art/code-aware-text-regions.md).
Prompt XML and inferred snippets are additional, documented policies; CommonMark
rendering behavior alone must not hide instructions inside XML wrappers.

## Keep whole-file detection effective

- `HL010` remains a local adjacent-word check, not a ban on mentioning the same
  technical word several times. Do not add a `cohesion` allowlist, raise global
  thresholds, or disable a rule for an entire file.
- Keep one `HL032` comparison index for every eligible line/paragraph in the
  complete file. Code blocks, tags, headings, and unrelated sections split
  units but do not reset this index. Preserve existing cross-file scope rules
  and related locations. Never form a paragraph by concatenating across a code
  block or nonadjacent text regions.
- Compare mixed prose/code instructions as structured units. Normalize prose
  using the documented prose policy; retain exact inline-code payloads and
  typed boundaries. Two identical ``Run `npm test` before merging.`` instructions
  remain duplicates. ``Run `npm test`.`` and ``Run `npm lint`.`` must stay distinct.
  Do not replace every code span with the same placeholder or strip meaningful
  identifier/string/operator differences.
- Required repeated tag names, identifiers, imports, closing delimiters, and
  assignments inside one example do not establish redundant prose. To avoid
  a blind spot for copied examples, plan a separate informational candidate for
  whole meaningful code blocks repeated elsewhere in the same file. Start with
  exact payload equality and a calibrated minimum meaningful size; preserve
  case, literals, operators, and indentation. No semantic equivalence claim,
  automatic deletion, or quality penalty. Freeze its own rule contract before
  implementation; do not silently change `HL032` severity or meaning.
- `HL030` must retain its existing positive controls. Adopt shared regions only
  after checking its own intent rules. Keep lexical candidates separate from
  deterministic findings and quality scores.

## Delivery slices

1. **Baseline and fixtures.** Obtain original snippets, file type, rule IDs,
   nearby fences, and expected highlights. Save minimal synthetic fixtures in
   the owning repositories. Capture before/after findings on a fixed labeled
   corpus with code negatives and prose positives. Use the saved case now;
   label reconstructed examples rather than calling them original evidence.
2. **Core contract and policy.** Add neutral region inputs and rule-specific
   handling in Core. Test with synthetic regions, preserving the legacy entry
   point, score thresholds, deterministic ordering, and UTF-8 spans. Merge the
   owning PR before pinning the accepted immutable SHA in SDK.
3. **SDK recognition.** Add the source-format adapter and bounded region
   extraction. Assess a maintained parser versus an independently written
   scanner against the same fixture corpus and existing dependency policy.
   Record the selected parser/version/license before adding a dependency.
   Ensure disk and overlay analysis use exactly the same path. Verify and merge
   SDK, then pin its accepted SHA in CLI and LSP.
4. **Redundancy and inferred snippets.** Preserve global `HL032` indexing and
   exact code payload distinctions. Evaluate the unfenced-snippet heuristic
   and informational code-block candidates separately. Do not enable either
   until measured false-positive boundaries and method labels are agreed.
5. **Adapters and composition.** Verify CLI output and native LSP diagnostics,
   including closed files, unsaved edits, clearing, and UTF-16 positions. Verify
   the editor against the accepted LSP build, then update its immutable release
   pin. Update hub submodule pins and `docs/repository-split.md` last, after
   owning PRs merge. Update the validation-rule documentation with actual shipped
   behavior; this plan is not a claim of implementation.

## Acceptance gates

- Every code-negative fixture in the saved case produces zero inappropriate
  `HL010` findings. Every labeled prose-positive and distant duplicate control
  still produces its expected rule and location. Track precision and recall
  separately; report sample counts and changed findings, not only total count.
- Include backtick and tilde fences, mismatched/longer delimiters, inline spans
  with embedded backticks, indented code under lists, XML text nodes, ordinary
  comparisons containing `<`/`=`, and multiple examples separated by prose.
- Cover CRLF/LF, accented text, combining marks, and emoji before a finding;
  original UTF-8 offsets and published UTF-16 positions must stay exact.
- Unterminated/oversized examples and exhausted parser budgets expose coverage
  limits. Never treat unassessed prose as evidence of a clean file. Preserve
  current source-size and analysis bounds; do not remove limits to improve recall.
- Reports contain locations, counts, method/version, and fixed reason codes;
  no snippets, identifiers, raw source, or secrets. Scores stay in `[0, 1]`,
  retain threshold-derived pass/fail, and expose the actual eligible sample
  count. Safety findings remain outside quality averaging.
- Run every changed repository's documented checks independently. For native
  components include locked formatting, Clippy, tests, and the Windows/Linux
  LSP smoke test. Compare the labeled corpus once after each relevant change.

Next implementation slice: baseline fixtures and the Core input contract.
This task records the case and plan; repetition behavior remains unchanged.
