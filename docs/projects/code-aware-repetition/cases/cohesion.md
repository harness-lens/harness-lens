<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Case: essential `cohesion` repetition in code examples

Status: saved for implementation; reported 2026-09-06.
Owner: Core analysis policy, with SDK source-region recognition.

The user reports two false positives: `cohesion` is a tag in the first case and
a variable in the second. Repetition is necessary to the example. The requested
fix must retain real repeated words and redundant lines throughout the file.

Original snippets, surrounding Markdown, file path, and diagnostic rule IDs
were not attached. The examples below are reconstructed acceptance cases, not
verbatim user evidence. Replace or supplement them when originals are available.
Candidate rules to inspect: `HL010`, `HL030`, `HL032`; confirm the emitted rule.

## Reconstructed tag case

The inner sample is literal Markdown/source input; the outer four-backtick
fence only displays the fixture in this case record.

````markdown
Example: <cohesion>cohesion</cohesion>
````

Expected: no `HL010`. Opening/closing tag names are syntax; only one prose
occurrence exists. A tag with no inner text is another negative control.

## Reconstructed variable case

````markdown
Update `cohesion = cohesion + delta` after evaluating the graph.
````

Expected: no `HL010`; both occurrences are inside one inline code span.
Also cover a fenced assignment, an indented example, and an unfenced assignment
under an example label. The last variant needs documented recognition policy.

## Required controls

Each row is a separate fixture, not a combined input file.

| Input or arrangement | Expected result |
| --- | --- |
| `cohesion cohesion improves readability.` | `HL010` on the second word |
| `Cohesion cohesion improves readability.` | Same case-insensitive finding |
| `<cohesion>cohesion cohesion</cohesion>` outside a code example | `HL010` inside the text node; tag names ignored |
| `Use **use** tests.` | Existing repeated-word detection retained |
| `Use use ` followed by inline code and ` before merging.` | Prose repetition still detected on a mixed line |
| `Check `, an inline code span, then ` check results.` | No artificial adjacent pair across the excluded code span |
| `cohesion.cohesion` or `cohesion_cohesion` inside a code span | No identifier fragments treated as prose repetition |
| Same `Run tests before merging.` line above and below an unrelated code block | `HL032` on the later line, linked to the earlier line |
| Same multi-line prose paragraph in distant sections | Existing paragraph duplicate finding retained |
| Two identical instructions containing inline `npm test` | `HL032` retained, with both locations |
| Instructions differing by inline `npm test` versus `npm lint` | No exact-duplicate claim |
| Repeated closing tags or variable updates inside one code example | No prose-repetition or prose-redundancy claim |
| Same complete meaningful code example in distant sections | Separate informational clone candidate, pending its own rule contract |
| Existing equivalent directives with different wording | Existing `HL030` positive control retained |

Do not whitelist the word `cohesion`, suppress a file after seeing one code
block, or relax every repetition threshold. Record precisely which spans were
classified as syntax and keep surrounding prose eligible.

Implementation and validation order: [code-aware repetition plan](../README.md).
