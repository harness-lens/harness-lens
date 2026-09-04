// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A kind of harness source that can affect an agent workspace.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum HarnessSourceKind {
    /// Human- or project-authored instructions.
    Instructions,
    /// Provider or project rules.
    Rules,
    /// Discoverable agent skills.
    Skills,
    /// Lifecycle or tool hooks.
    Hooks,
    /// Agent declarations.
    Agents,
    /// Runtime or provider configuration.
    Configuration,
    /// Persistent memory sources.
    Memory,
    /// Declared workflows.
    Workflows,
    /// Source recognized by custom configuration or a plugin.
    Other,
}

/// Loaded input supplied to the deterministic core.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessSource {
    /// Path relative to the scanned root.
    pub path: PathBuf,
    /// Semantic source category.
    pub kind: HarnessSourceKind,
    /// Directory where this source takes effect.
    pub scope: PathBuf,
    /// UTF-8 source content. Reports never serialize this field directly.
    pub content: String,
}

impl HarnessSource {
    /// Produces the content-free representation used in public reports.
    #[must_use]
    pub fn record(&self) -> SourceRecord {
        SourceRecord {
            path: self.path.clone(),
            kind: self.kind,
            scope: self.scope.clone(),
            bytes: self.content.len(),
        }
    }
}

/// Content-free source metadata safe for reports and integrations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceRecord {
    /// Path relative to the scanned root.
    pub path: PathBuf,
    /// Semantic source category.
    pub kind: HarnessSourceKind,
    /// Directory where this source takes effect.
    pub scope: PathBuf,
    /// Loaded UTF-8 byte count.
    pub bytes: usize,
}

/// Severity attached to an evidence-bearing finding.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Rule completed successfully.
    Pass,
    /// Informational result.
    Info,
    /// Condition deserves attention but does not invalidate the scan.
    Warning,
    /// Condition prevents a trustworthy result.
    Error,
}

/// Half-open UTF-8 byte range within a [`HarnessSource`].
///
/// Byte offsets keep the core independent from editor protocols. Presentation
/// adapters are responsible for converting them to their native position
/// encoding, such as the UTF-16 positions used by LSP clients.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TextSpan {
    /// Inclusive starting byte offset.
    pub start: usize,
    /// Exclusive ending byte offset.
    pub end: usize,
}

/// Deterministic evidence emitted by the core or a plugin.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    /// Severity of the finding.
    pub severity: Severity,
    /// Stable rule identifier.
    pub rule_id: String,
    /// Human-readable result.
    pub message: String,
    /// Related source path, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    /// One-based line number, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    /// Exact UTF-8 byte range in the related source, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<TextSpan>,
    /// Minimal supporting evidence, when safe to expose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<String>,
    /// Core or plugin identifier that produced this finding.
    pub source: String,
}

/// Named raw measurement produced by deterministic analysis.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    /// Stable metric name.
    pub name: String,
    /// Numeric value in its natural unit.
    pub value: f64,
    /// Optional unit or profile identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Core or plugin identifier that produced this metric.
    pub source: String,
}

/// Whether a normalized score may participate in quality aggregation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreCategory {
    /// Static quality signal eligible for a quality summary.
    Quality,
    /// Safety constraint reported separately and never averaged into quality.
    Safety,
    /// Repeatability or robustness signal.
    Reliability,
    /// Cost, latency, or resource signal.
    Performance,
}

/// How a normalized score was produced.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScoreMethod {
    /// Exact rule with no estimated uncertainty.
    Deterministic,
    /// Explainable rule-of-thumb with explicit evidence.
    Heuristic,
    /// Aggregate computed from observed samples.
    Statistical,
    /// Deterministic probability estimate computed from declared assumptions.
    Probabilistic,
}

/// Uncertainty attached to a statistical or probabilistic score.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidenceEstimate {
    /// Lower bound in the normalized interval.
    pub lower: f64,
    /// Upper bound in the normalized interval.
    pub upper: f64,
    /// Confidence level, such as `0.95`.
    pub level: f64,
    /// Estimator name and assumptions.
    pub method: String,
}

/// Invalid normalized score construction.
#[derive(Clone, Debug, PartialEq)]
pub enum ScoreError {
    /// Value or threshold was not finite.
    NotFinite,
    /// Value or threshold was outside the normalized interval.
    OutsideUnitInterval(f64),
}

impl fmt::Display for ScoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFinite => formatter.write_str("score value and threshold must be finite"),
            Self::OutsideUnitInterval(value) => {
                write!(
                    formatter,
                    "score value must be between 0.0 and 1.0: {value}"
                )
            }
        }
    }
}

impl Error for ScoreError {}

/// Normalized, evidence-bearing score with derived pass/fail state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Score {
    /// Stable score identifier.
    pub id: String,
    /// Aggregation and policy category.
    pub category: ScoreCategory,
    /// Computation method.
    pub method: ScoreMethod,
    /// Normalized value from 0.0 to 1.0.
    pub value: f64,
    /// Configured pass threshold from 0.0 to 1.0.
    pub threshold: f64,
    /// Derived state. Callers cannot choose it independently.
    pub passed: bool,
    /// Number of observations behind this result, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<usize>,
    /// Optional uncertainty estimate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<ConfidenceEstimate>,
    /// Human-readable interpretation and assumptions.
    pub reason: String,
    /// Structured, non-secret evidence.
    pub evidence: BTreeMap<String, String>,
    /// Core or plugin identifier that produced this score.
    pub source: String,
}

impl Score {
    /// Constructs a valid normalized score and derives `passed`.
    pub fn new(
        id: impl Into<String>,
        category: ScoreCategory,
        method: ScoreMethod,
        value: f64,
        threshold: f64,
        reason: impl Into<String>,
        source: impl Into<String>,
    ) -> Result<Self, ScoreError> {
        if !value.is_finite() || !threshold.is_finite() {
            return Err(ScoreError::NotFinite);
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(ScoreError::OutsideUnitInterval(value));
        }
        if !(0.0..=1.0).contains(&threshold) {
            return Err(ScoreError::OutsideUnitInterval(threshold));
        }

        Ok(Self {
            id: id.into(),
            category,
            method,
            value,
            threshold,
            passed: value >= threshold,
            sample_size: None,
            confidence: None,
            reason: reason.into(),
            evidence: BTreeMap::new(),
            source: source.into(),
        })
    }
}

/// Observable status for one plugin execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginExecutionStatus {
    /// Plugin completed and its output was accepted.
    Completed,
    /// Plugin was disabled by configuration.
    Disabled,
    /// Configured plugin was not registered by the host.
    Unavailable,
    /// Plugin returned an error.
    Failed,
}

/// Execution trace included in every analysis report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PluginExecution {
    /// Stable plugin identifier.
    pub id: String,
    /// Terminal execution state.
    pub status: PluginExecutionStatus,
    /// Wall-clock execution time measured by the host.
    pub duration_micros: u64,
    /// Optional failure or availability detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Category-aware report aggregation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScoreSummary {
    /// Mean of quality-category scores, when any exist.
    pub quality_mean: Option<f64>,
    /// Failed safety constraints. Safety is never included in `quality_mean`.
    pub safety_violations: usize,
    /// Mean score per category.
    pub by_category: BTreeMap<ScoreCategory, f64>,
}

/// Complete provider-neutral analysis result.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalysisReport {
    /// Report schema version.
    pub schema_version: u32,
    /// Scanned workspace root.
    pub root: PathBuf,
    /// Whether discovery and source loading covered the requested workspace.
    #[serde(default)]
    pub completeness: ScanCompleteness,
    /// Content-free discovered source records.
    pub sources: Vec<SourceRecord>,
    /// Evidence-bearing findings.
    pub findings: Vec<Finding>,
    /// Raw deterministic measurements.
    pub metrics: Vec<Metric>,
    /// Normalized scores.
    pub scores: Vec<Score>,
    /// Category-aware aggregation.
    pub score_summary: ScoreSummary,
    /// Plugin execution trace for observability.
    pub plugin_executions: Vec<PluginExecution>,
}

/// Whether the adapter could inspect every relevant path it encountered.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScanCompleteness {
    /// `true` only when no limiting condition was observed.
    pub complete: bool,
    /// Stable, content-free reasons why a scan is incomplete.
    pub reasons: Vec<IncompleteReason>,
}

impl Default for ScanCompleteness {
    fn default() -> Self {
        Self {
            complete: true,
            reasons: Vec::new(),
        }
    }
}

/// One content-free reason that prevents an authoritative workspace scan.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IncompleteReason {
    /// Stable machine-readable reason code.
    pub code: String,
    /// Related path, relative to the scan root when possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
}

impl AnalysisReport {
    /// Returns aggregate report counts.
    #[must_use]
    pub fn summary(&self) -> ScanSummary {
        ScanSummary {
            sources: self.sources.len(),
            diagnostics: self
                .findings
                .iter()
                .filter(|finding| matches!(finding.severity, Severity::Warning | Severity::Error))
                .count(),
        }
    }
}

/// Aggregate, content-free result of a workspace scan.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScanSummary {
    /// Number of recognized harness sources.
    pub sources: usize,
    /// Number of warning and error findings.
    pub diagnostics: usize,
}

impl ScanSummary {
    /// Returns whether the scan found no harness sources or diagnostics.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.sources == 0 && self.diagnostics == 0
    }
}
