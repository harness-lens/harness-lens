// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

#![doc = include_str!("../README.md")]

use harness_lens_core::{
    AnalysisReport, Finding, IntegrationError, ReportSink, ScanCompleteness, Score, ScoreSummary,
};
use serde::{Deserialize, Serialize};

/// Content-safe document ready for a Harness Score transport.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HarnessScoreDocument {
    /// Adapter document version.
    pub schema_version: u32,
    /// Scanned project root.
    pub project: String,
    /// Number of discovered harness sources.
    pub source_count: usize,
    /// Whether the source inventory is authoritative.
    #[serde(default)]
    pub completeness: ScanCompleteness,
    /// Category-aware summary preserving safety separation.
    pub summary: ScoreSummary,
    /// Normalized evidence-bearing scores.
    pub scores: Vec<Score>,
    /// Detailed findings.
    pub findings: Vec<Finding>,
}

impl From<&AnalysisReport> for HarnessScoreDocument {
    fn from(report: &AnalysisReport) -> Self {
        Self {
            schema_version: 1,
            project: report.root.to_string_lossy().into_owned(),
            source_count: report.sources.len(),
            completeness: report.completeness.clone(),
            summary: report.score_summary.clone(),
            scores: report.scores.clone(),
            findings: report.findings.clone(),
        }
    }
}

/// Product- and protocol-specific transport supplied by an integration host.
pub trait HarnessScoreTransport: Send + Sync {
    /// Sends one mapped document.
    fn send(&self, document: &HarnessScoreDocument) -> Result<(), String>;
}

/// Harness Score report sink with injected transport behavior.
pub struct HarnessScoreAdapter<T> {
    transport: T,
}

impl<T> HarnessScoreAdapter<T> {
    /// Creates an adapter without selecting HTTP, credentials, or vendor SDKs.
    #[must_use]
    pub const fn new(transport: T) -> Self {
        Self { transport }
    }
}

impl<T: HarnessScoreTransport> ReportSink for HarnessScoreAdapter<T> {
    fn id(&self) -> &str {
        "harness-score"
    }

    fn publish(&self, report: &AnalysisReport) -> Result<(), IntegrationError> {
        self.transport
            .send(&HarnessScoreDocument::from(report))
            .map_err(IntegrationError::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use harness_lens_core::AnalysisEngine;
    use std::path::PathBuf;
    use std::sync::Mutex;

    #[derive(Default)]
    struct MemoryTransport {
        documents: Mutex<Vec<HarnessScoreDocument>>,
    }

    impl HarnessScoreTransport for MemoryTransport {
        fn send(&self, document: &HarnessScoreDocument) -> Result<(), String> {
            self.documents.lock().unwrap().push(document.clone());
            Ok(())
        }
    }

    #[test]
    fn maps_reports_without_coupling_core_to_transport() {
        let report = AnalysisEngine::new().analyze(
            PathBuf::from("demo"),
            Vec::new(),
            Vec::new(),
            &harness_lens_core::HarnessLensConfig::default(),
        );
        let adapter = HarnessScoreAdapter::new(MemoryTransport::default());

        adapter.publish(&report).unwrap();

        let documents = adapter.transport.documents.lock().unwrap();
        assert_eq!(documents[0].project, "demo");
        assert!(documents[0].completeness.complete);
        assert_eq!(documents[0].summary.safety_violations, 0);
    }
}
