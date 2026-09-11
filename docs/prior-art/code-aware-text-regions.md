<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Prior art: code-aware text regions

Research date: 2026-09-06. Status: proposed design, not implemented.

Source: [CommonMark 0.31.2](https://spec.commonmark.org/0.31.2/), especially
[indented code](https://spec.commonmark.org/0.31.2/#indented-code-blocks),
[fences](https://spec.commonmark.org/0.31.2/#fenced-code-blocks),
[inline code](https://spec.commonmark.org/0.31.2/#code-spans), and
[raw HTML](https://spec.commonmark.org/0.31.2/#raw-html).

## Proposed adoption

Use explicit block and inline syntax boundaries to distinguish code from prose.
Fence character/length and inline delimiter matching replace the current
Boolean fence toggle. Preserve original byte positions for diagnostics.

## Rejected and deferred

Reject punctuation-only tokenization as sufficient code recognition. Reject
whole-file or whole-line suppression caused by one code span. Defer parser
selection until dependency, licensing, offset, and fixture checks are complete.

## Assumptions and differences

Markdown syntax does not establish whether an instruction is redundant.
Harness Lens applies separate rule policies and keeps prose inside prompt XML
eligible, even when a Markdown renderer would treat a larger region as raw HTML.
Unfenced programming-language inference is separate heuristic work.
Format parsing belongs in SDK; Core consumes neutral source regions.

## Licensing

The specification identifies CC BY-SA 4.0 licensing. This note links the source
and records design attribution; no specification text, example corpus, or parser
code is copied. Any future parser or imported fixtures need their own license
review. Independently written project code remains MPL-2.0.

Related work: [case and implementation plan](../projects/code-aware-repetition/README.md).
