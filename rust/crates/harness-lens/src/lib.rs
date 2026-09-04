// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

#![doc = include_str!("../README.md")]

mod filesystem;

pub use filesystem::{
    DiscoveryResult, ScanError, Scanner, discover, discover_detailed, is_harness_path,
};
pub use harness_lens_config::{ConfigError, DEFAULT_CONFIG_FILE, load_for_root};
pub use harness_lens_core::{
    AnalysisEngine, AnalysisReport, ConfidenceEstimate, DiscoveryConfig, Finding,
    HarnessLensConfig, HarnessSource, HarnessSourceKind, IncompleteReason, IntegrationConfig,
    IntegrationError, Metric, Plugin, PluginConfig, PluginContext, PluginError, PluginExecution,
    PluginExecutionStatus, PluginMetadata, PluginOutput, RegistrationError, ReportSink,
    ScanCompleteness, ScanSummary, Score, ScoreCategory, ScoreError, ScoreMethod, ScoreSummary,
    Severity, SourceRecord, TextSpan, statistics,
};

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
