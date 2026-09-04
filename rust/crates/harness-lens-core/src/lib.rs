// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

#![doc = include_str!("../README.md")]

mod config;
mod engine;
mod model;
mod plugin;
pub mod statistics;
mod text_analysis;

pub use config::{DiscoveryConfig, HarnessLensConfig, IntegrationConfig, PluginConfig};
pub use engine::{AnalysisEngine, RegistrationError};
pub use model::{
    AnalysisReport, ConfidenceEstimate, Finding, HarnessSource, HarnessSourceKind,
    IncompleteReason, Metric, PluginExecution, PluginExecutionStatus, ScanCompleteness,
    ScanSummary, Score, ScoreCategory, ScoreError, ScoreMethod, ScoreSummary, Severity,
    SourceRecord, TextSpan,
};
pub use plugin::{
    IntegrationError, Plugin, PluginContext, PluginError, PluginMetadata, PluginOutput, ReportSink,
};
