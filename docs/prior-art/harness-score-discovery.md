<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Prior art: Harness Score discovery

Research date: 2026-09-04

Harness Lens studied the discovery safety properties documented by
[`paladini/harness-score`](https://github.com/paladini/harness-score) and its
[`scan.ts`](https://github.com/paladini/harness-score/blob/main/packages/cli/src/scan.ts).
This is design attribution; the Rust implementation is independently written.
Harness Score is MIT-licensed and Harness Lens remains MPL-2.0.

## Adopted

- deterministic lexical output and physical paths before symlink aliases;
- configurable ignored dependency/generated directories;
- a `1,000,000`-file default emergency fuse;
- canonical visited-directory tracking to stop cycles;
- followed symlinks must resolve inside the canonical workspace root;
- unreadable paths, outside-root symlinks, size limits, and the file fuse mark a
  scan incomplete instead of presenting partial output as authoritative;
- editor overlays obey the same recognition and size rules as disk sources.

## Intentional differences

- Harness Lens does not follow symlinks by default; it is explicit opt-in.
- The existing Harness Lens source limit remains 1 MiB instead of Harness
  Score's 512 KiB. Both are bounded; changing the public default needs fixture
  and compatibility review.
- Harness Lens returns content-free completeness reasons in its generic report,
  while read/UTF-8 failures also produce `HL006` findings.
- Discovery recognizes configurable agent-harness paths rather than Harness
  Score's scoring inputs.

The adopted properties are security and trust invariants, not a claim that the
projects have identical scanners.
