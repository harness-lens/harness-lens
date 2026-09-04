// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

//! Standard-input/output entry point for the Harness Lens language server.

#[tokio::main]
async fn main() {
    harness_lens_lsp::serve().await;
}
