// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

#![doc = include_str!("../README.md")]

pub use harness_lens_core::{HarnessSourceKind, ScanSummary};

/// Published Harness Lens namespace-bootstrap version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_exposes_core_summary() {
        assert!(ScanSummary::default().is_empty());
    }
}
