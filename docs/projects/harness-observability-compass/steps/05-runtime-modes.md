<!-- SPDX-License-Identifier: MPL-2.0 -->
<!-- Copyright © 2026 Cristian Camargo Filho -->

# Step 05 — runtime modes

Owners: `language-server`, then VS Code settings.

Support explicit modes:

| Mode | Behavior | Default |
| --- | --- | --- |
| `off` | deterministic report only; no capture process | yes |
| `live` | opt-in aggregate capture at startup and explicit refresh | no |
| `snapshot` | read canonical safe snapshot; no process launch | no |

Invalid settings and capture failures become observable status. Keep last valid
report where safe. Runtime evidence cannot change deterministic findings or
scores. Do not bundle external capture tooling in the editor.

## Done when

Mode, executable, period, and snapshot settings are documented and tested in
server and editor. Privacy and licensing boundaries are visible.
