<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Language server

The implemented [`harness-lens-lsp`](https://github.com/harness-lens/language-server)
crate is a thin presentation adapter over the same
[SDK](https://github.com/harness-lens/sdk) and
[core](https://github.com/harness-lens/core) used by Python and the CLI. It speaks Language Server Protocol
over standard input/output and publishes ordinary diagnostics. It contains no
validation rules.

## Data flow

```text
recognized harness file or unsaved buffer
  -> deterministic root-bounded discovery
  -> provider-neutral core plugins
  -> Finding(path, line, UTF-8 byte span, HL code)
  -> UTF-16 LSP Diagnostic(source = harness-lens)
  -> Problems view / Error Lens / any compatible client
```

The server accepts full-document synchronization. On open, change, or save it
rescans the owning workspace with all currently open buffers overlaid in memory.
This lets a conflict in another harness file appear without saving the current
document. Closing a document clears its diagnostics.

The deepest configured workspace root owns a document in a multi-root session.
Repository-local `harness-lens.toml` controls discovery and plugin activation.

## Run and connect

For development:

```bash
git clone https://github.com/harness-lens/language-server.git
cd language-server/rust
cargo run
```

An editor client should launch `harness-lens-lsp`, use standard input/output as
the transport, and activate for configured harness paths (`AGENTS.md`,
`CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md`, and
`.cursor/rules/**` by default). The
[`harness-lens-vscode`](https://github.com/harness-lens/harness-lens-vscode)
extension implements this lifecycle and selector work.

No Error Lens-specific API is required. Error Lens observes VS Code diagnostics
and adds its line highlight, gutter marker, and end-of-line message. Harness Lens
supplies accurate ranges, warning severity, stable codes `HL010` and `HL020`, and
`source = harness-lens`.

## Durable next steps

- package native language-server binaries for extension installation;
- debounce rapid document changes and cache unchanged loaded sources;
- respond to dynamic workspace-folder changes;
- add rule documentation and code actions without moving analysis into the editor;
- add protocol-level fixtures for multi-root and configuration failures;
- benchmark large workspaces before changing the conservative full-rescan model.

These are integration and performance tasks. The core/LSP diagnostic contract is
implemented and tested now.
