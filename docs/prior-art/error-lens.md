<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Prior art: Error Lens

Research date: 2026-09-04

[Error Lens](https://github.com/usernamehw/vscode-error-lens) demonstrates a
useful presentation pattern: diagnostics can be highlighted by line, represented
with gutter icons, and rendered as end-of-line text. Its
[decoration implementation](https://github.com/usernamehw/vscode-error-lens/blob/master/src/decorations.ts)
reads diagnostics from VS Code, groups them by line, and applies editor
decorations.

Harness Lens adopts the protocol boundary, not the decoration implementation:

- the generic core emits evidence-bearing findings and UTF-8 byte spans;
- `harness-lens-lsp` converts spans to UTF-16 and publishes standard LSP
  diagnostics;
- VS Code, Error Lens, or another client chooses how to render them.

No Error Lens source was copied, and neither the core nor language server takes
an Error Lens or VS Code dependency. Only the dedicated VS Code client imports
VS Code APIs. This keeps the server usable from Neovim, Zed, Emacs, and other
LSP clients while allowing Error Lens to enhance the VS Code experience. Error
Lens is MIT-licensed; Harness Lens remains MPL-2.0.
