// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::{AnalysisReport, Finding, HarnessLensConfig, HarnessSource, Metric, Score};

/// Stable metadata used to discover and configure a plugin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginMetadata {
    /// Globally unique plugin identifier.
    pub id: String,
    /// Human-readable plugin name.
    pub name: String,
    /// Plugin implementation version.
    pub version: String,
    /// Provider-neutral capabilities supplied by the plugin.
    pub capabilities: Vec<String>,
    /// Whether the host should run the plugin when no explicit config exists.
    pub default_enabled: bool,
}

/// Read-only input visible to a plugin.
pub struct PluginContext<'a> {
    /// Full application configuration.
    pub config: &'a HarnessLensConfig,
    /// Loaded harness sources.
    pub sources: &'a [HarnessSource],
    /// Opaque options owned by this plugin.
    pub options: &'a BTreeMap<String, String>,
}

/// Successful plugin output.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PluginOutput {
    /// Evidence-bearing findings.
    pub findings: Vec<Finding>,
    /// Raw measurements.
    pub metrics: Vec<Metric>,
    /// Normalized scores.
    pub scores: Vec<Score>,
}

/// Error returned by a plugin without terminating the host process.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginError {
    message: String,
}

impl PluginError {
    /// Creates a plugin failure with a user-safe message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for PluginError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for PluginError {}

/// Extension point for deterministic analyzers.
pub trait Plugin: Send + Sync {
    /// Returns stable plugin metadata.
    fn metadata(&self) -> PluginMetadata;

    /// Analyzes sources without owning provider or framework state.
    fn analyze(&self, context: &PluginContext<'_>) -> Result<PluginOutput, PluginError>;
}

/// Error returned by an outbound report integration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegrationError {
    message: String,
}

impl IntegrationError {
    /// Creates an integration failure with a user-safe message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for IntegrationError {}

/// Provider-neutral output port implemented by integrations such as Harness Score.
pub trait ReportSink: Send + Sync {
    /// Stable integration identifier.
    fn id(&self) -> &str;

    /// Publishes a completed report using adapter-owned transport behavior.
    fn publish(&self, report: &AnalysisReport) -> Result<(), IntegrationError>;
}
