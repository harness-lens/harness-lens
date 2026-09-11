<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step order

Work one slice at a time. Stop after its checks pass and checkpoint is updated.

```text
00 baseline
  -> 01 core contract
    -> 02 per-file metrics
      -> 03 LSP report
        -> 04 editor tree
          -> 05 runtime modes
            -> 06 tool-call history
              -> 07 attributed effectiveness
                -> 08 verification and composition
```

Steps 00–04 make static experience useful without runtime data. Steps 05–07
add optional runtime evidence in increasing risk order. Step 08 is last:
verify owning repositories, then update hub pins and split documentation.

## Current next slice

Start with [steps/00-baseline](steps/00-baseline.md). It is read-only and
establishes which local work belongs to recovered session before any edit.
