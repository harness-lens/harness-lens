> SPDX-License-Identifier: MPL-2.0
> Copyright © 2026 Cristian Camargo Filho

# Security Policy

## Supported versions

HarnessLens is pre-alpha. Security fixes are released only for the latest version.

## Reporting a vulnerability

Do not open a public issue. Use GitHub's private vulnerability reporting:

https://github.com/harness-lens/harness-lens/security/advisories/new

Include affected version, reproduction steps, impact, and any suggested mitigation.

## Current trust boundaries

- Scans are local and do not make network requests.
- Directory symlinks are not followed unless configuration explicitly opts in.
- Source loading is capped by `discovery.max_file_bytes`.
- Discovery stops at `discovery.max_files` and marks the report incomplete.
- Followed symlinks must resolve within the canonical workspace root.
- Serialized reports contain source metadata and evidence, not raw source content.
- Plugins are registered in process at compile time. Loading arbitrary native
  libraries is intentionally unsupported until ABI, signature, permission, and
  crash-isolation policies exist.
- The Harness Score adapter has no built-in endpoint or credential handling; a
  host must inject a transport.

Treat third-party plugins and transports as code executing with the host's
permissions. Never place credentials in `harness-lens.toml` plugin options or
report evidence.
