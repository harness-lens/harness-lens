// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use std::time::Instant;

use crate::text_analysis::{IncongruencePlugin, RepetitionPlugin};
use crate::{
    AnalysisReport, Finding, HarnessLensConfig, HarnessSource, Metric, Plugin, PluginContext,
    PluginExecution, PluginExecutionStatus, PluginMetadata, PluginOutput, Score, ScoreCategory,
    ScoreMethod, ScoreSummary, Severity,
};

const INVENTORY_PLUGIN_ID: &str = "harness-lens.inventory";

/// Error returned when a host registers an invalid plugin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistrationError {
    /// Plugin identifier was empty.
    EmptyId,
    /// Plugin identifier was already registered.
    DuplicateId(String),
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId => formatter.write_str("plugin id cannot be empty"),
            Self::DuplicateId(id) => write!(formatter, "plugin already registered: {id}"),
        }
    }
}

impl Error for RegistrationError {}

/// Deterministic plugin host. Providers and agent frameworks live outside this type.
pub struct AnalysisEngine {
    plugins: BTreeMap<String, Box<dyn Plugin>>,
}

impl Default for AnalysisEngine {
    fn default() -> Self {
        let mut engine = Self::empty();
        engine
            .register(InventoryPlugin)
            .expect("built-in plugin id must be valid");
        engine
            .register(RepetitionPlugin)
            .expect("built-in plugin id must be valid");
        engine
            .register(IncongruencePlugin)
            .expect("built-in plugin id must be valid");
        engine
    }
}

impl AnalysisEngine {
    /// Creates an engine with first-party deterministic plugins.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an engine without plugins for custom hosts and tests.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            plugins: BTreeMap::new(),
        }
    }

    /// Registers one native plugin by stable identifier.
    pub fn register(&mut self, plugin: impl Plugin + 'static) -> Result<(), RegistrationError> {
        let metadata = plugin.metadata();
        let id = metadata.id.trim();
        if id.is_empty() {
            return Err(RegistrationError::EmptyId);
        }
        if self.plugins.contains_key(id) {
            return Err(RegistrationError::DuplicateId(id.to_owned()));
        }
        self.plugins.insert(id.to_owned(), Box::new(plugin));
        Ok(())
    }

    /// Produces a report while isolating plugin failures from the rest of the scan.
    #[must_use]
    pub fn analyze(
        &self,
        root: PathBuf,
        sources: Vec<HarnessSource>,
        mut findings: Vec<Finding>,
        config: &HarnessLensConfig,
    ) -> AnalysisReport {
        let mut metrics = Vec::new();
        let mut scores = Vec::new();
        let mut executions = Vec::new();
        let configured_ids: BTreeSet<&str> = config
            .plugins
            .iter()
            .map(|plugin| plugin.id.as_str())
            .collect();

        for plugin_config in &config.plugins {
            if !plugin_config.enabled {
                executions.push(PluginExecution {
                    id: plugin_config.id.clone(),
                    status: PluginExecutionStatus::Disabled,
                    duration_micros: 0,
                    message: None,
                });
                continue;
            }

            let Some(plugin) = self.plugins.get(&plugin_config.id) else {
                executions.push(PluginExecution {
                    id: plugin_config.id.clone(),
                    status: PluginExecutionStatus::Unavailable,
                    duration_micros: 0,
                    message: Some("plugin is enabled but not registered by this host".to_owned()),
                });
                continue;
            };

            run_plugin(
                plugin.as_ref(),
                config,
                &sources,
                &plugin_config.options,
                &mut findings,
                &mut metrics,
                &mut scores,
                &mut executions,
            );
        }

        let empty_options = BTreeMap::new();
        for plugin in self.plugins.values() {
            let metadata = plugin.metadata();
            if metadata.default_enabled && !configured_ids.contains(metadata.id.as_str()) {
                run_plugin(
                    plugin.as_ref(),
                    config,
                    &sources,
                    &empty_options,
                    &mut findings,
                    &mut metrics,
                    &mut scores,
                    &mut executions,
                );
            }
        }

        AnalysisReport {
            schema_version: 1,
            root,
            completeness: Default::default(),
            sources: sources.iter().map(HarnessSource::record).collect(),
            findings,
            metrics,
            score_summary: summarize_scores(&scores),
            scores,
            plugin_executions: executions,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn run_plugin(
    plugin: &dyn Plugin,
    config: &HarnessLensConfig,
    sources: &[HarnessSource],
    options: &BTreeMap<String, String>,
    findings: &mut Vec<Finding>,
    metrics: &mut Vec<Metric>,
    scores: &mut Vec<Score>,
    executions: &mut Vec<PluginExecution>,
) {
    let metadata = plugin.metadata();
    let started = Instant::now();
    let result = plugin.analyze(&PluginContext {
        config,
        sources,
        options,
    });
    let duration_micros = started.elapsed().as_micros().try_into().unwrap_or(u64::MAX);

    match result {
        Ok(output) => {
            findings.extend(output.findings);
            metrics.extend(output.metrics);
            scores.extend(output.scores);
            executions.push(PluginExecution {
                id: metadata.id,
                status: PluginExecutionStatus::Completed,
                duration_micros,
                message: None,
            });
        }
        Err(error) => executions.push(PluginExecution {
            id: metadata.id,
            status: PluginExecutionStatus::Failed,
            duration_micros,
            message: Some(error.to_string()),
        }),
    }
}

fn summarize_scores(scores: &[Score]) -> ScoreSummary {
    let mut sums: BTreeMap<ScoreCategory, (f64, usize)> = BTreeMap::new();
    for score in scores {
        let entry = sums.entry(score.category).or_insert((0.0, 0));
        entry.0 += score.value;
        entry.1 += 1;
    }

    let by_category = sums
        .iter()
        .map(|(category, (sum, count))| (*category, sum / *count as f64))
        .collect();
    let quality_values: Vec<f64> = scores
        .iter()
        .filter(|score| score.category == ScoreCategory::Quality)
        .map(|score| score.value)
        .collect();

    ScoreSummary {
        quality_mean: (!quality_values.is_empty())
            .then(|| quality_values.iter().sum::<f64>() / quality_values.len() as f64),
        safety_violations: scores
            .iter()
            .filter(|score| score.category == ScoreCategory::Safety && !score.passed)
            .count(),
        by_category,
    }
}

struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: INVENTORY_PLUGIN_ID.to_owned(),
            name: "Source inventory".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            capabilities: vec!["inventory".to_owned()],
            default_enabled: true,
        }
    }

    fn analyze(&self, context: &PluginContext<'_>) -> Result<PluginOutput, crate::PluginError> {
        let source_count = context.sources.len();
        let (severity, value, message) = if source_count == 0 {
            (
                Severity::Warning,
                0.0,
                "No harness sources found".to_owned(),
            )
        } else {
            (
                Severity::Pass,
                1.0,
                format!("Found {source_count} harness source(s)"),
            )
        };
        let mut score = Score::new(
            "harness.source_presence",
            ScoreCategory::Quality,
            ScoreMethod::Deterministic,
            value,
            1.0,
            &message,
            INVENTORY_PLUGIN_ID,
        )
        .expect("built-in score is normalized");
        score.sample_size = Some(source_count);

        Ok(PluginOutput {
            findings: vec![Finding {
                severity,
                rule_id: "HL001".to_owned(),
                message,
                path: None,
                line: None,
                span: None,
                evidence: None,
                source: INVENTORY_PLUGIN_ID.to_owned(),
            }],
            metrics: vec![Metric {
                name: "harness.sources".to_owned(),
                value: source_count as f64,
                unit: Some("count".to_owned()),
                source: INVENTORY_PLUGIN_ID.to_owned(),
            }],
            scores: vec![score],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PluginConfig, PluginError};

    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn metadata(&self) -> PluginMetadata {
            PluginMetadata {
                id: "example.test".to_owned(),
                name: "Test plugin".to_owned(),
                version: "1".to_owned(),
                capabilities: vec!["test".to_owned()],
                default_enabled: false,
            }
        }

        fn analyze(&self, context: &PluginContext<'_>) -> Result<PluginOutput, PluginError> {
            assert_eq!(
                context.options.get("mode").map(String::as_str),
                Some("strict")
            );
            Ok(PluginOutput::default())
        }
    }

    #[test]
    fn configured_plugins_execute_through_generic_contract() {
        let mut engine = AnalysisEngine::empty();
        engine.register(TestPlugin).unwrap();
        let config = HarnessLensConfig {
            plugins: vec![PluginConfig {
                id: "example.test".to_owned(),
                enabled: true,
                options: BTreeMap::from([("mode".to_owned(), "strict".to_owned())]),
            }],
            ..HarnessLensConfig::default()
        };

        let report = engine.analyze(PathBuf::from("."), Vec::new(), Vec::new(), &config);

        assert_eq!(report.plugin_executions.len(), 1);
        assert_eq!(
            report.plugin_executions[0].status,
            PluginExecutionStatus::Completed
        );
    }

    #[test]
    fn unavailable_plugin_is_observable_not_fatal() {
        let config = HarnessLensConfig {
            plugins: vec![PluginConfig {
                id: "missing.plugin".to_owned(),
                ..PluginConfig::default()
            }],
            ..HarnessLensConfig::default()
        };

        let report =
            AnalysisEngine::empty().analyze(PathBuf::from("."), Vec::new(), Vec::new(), &config);

        assert_eq!(
            report.plugin_executions[0].status,
            PluginExecutionStatus::Unavailable
        );
    }

    #[test]
    fn safety_is_not_averaged_into_quality() {
        let quality = Score::new(
            "quality",
            ScoreCategory::Quality,
            ScoreMethod::Deterministic,
            0.8,
            0.7,
            "quality",
            "test",
        )
        .unwrap();
        let safety = Score::new(
            "safety",
            ScoreCategory::Safety,
            ScoreMethod::Deterministic,
            0.0,
            1.0,
            "violation",
            "test",
        )
        .unwrap();
        let reliability = Score::new(
            "reliability",
            ScoreCategory::Reliability,
            ScoreMethod::Statistical,
            0.2,
            0.7,
            "repeatability",
            "test",
        )
        .unwrap();
        let summary = summarize_scores(&[quality, safety, reliability]);

        assert_eq!(summary.quality_mean, Some(0.8));
        assert_eq!(summary.safety_violations, 1);
        assert_eq!(summary.by_category[&ScoreCategory::Reliability], 0.2);
    }
}
