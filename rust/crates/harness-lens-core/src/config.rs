// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Provider-neutral configuration consumed by the analysis core.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct HarnessLensConfig {
    /// Configuration schema version.
    pub version: u32,
    /// Filesystem discovery policy.
    pub discovery: DiscoveryConfig,
    /// Plugin activation and opaque string options.
    pub plugins: Vec<PluginConfig>,
    /// Outbound adapter activation and opaque string options.
    pub integrations: Vec<IntegrationConfig>,
}

impl Default for HarnessLensConfig {
    fn default() -> Self {
        Self {
            version: 1,
            discovery: DiscoveryConfig::default(),
            plugins: Vec::new(),
            integrations: Vec::new(),
        }
    }
}

impl HarnessLensConfig {
    /// Finds configuration for a plugin by stable identifier.
    #[must_use]
    pub fn plugin(&self, id: &str) -> Option<&PluginConfig> {
        self.plugins.iter().find(|plugin| plugin.id == id)
    }

    /// Finds configuration for an outbound integration by stable identifier.
    #[must_use]
    pub fn integration(&self, id: &str) -> Option<&IntegrationConfig> {
        self.integrations
            .iter()
            .find(|integration| integration.id == id)
    }
}

/// Rules used by filesystem discovery adapters.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DiscoveryConfig {
    /// File names recognized at any repository depth.
    pub file_names: Vec<String>,
    /// File path suffixes recognized at any repository depth.
    pub path_suffixes: Vec<String>,
    /// Directory path suffixes whose files are recognized recursively.
    pub directory_suffixes: Vec<String>,
    /// Directory names excluded from traversal.
    pub ignored_directories: Vec<String>,
    /// Maximum source size loaded into memory.
    pub max_file_bytes: u64,
    /// Maximum number of files considered before discovery stops safely.
    pub max_files: usize,
    /// Whether filesystem adapters may traverse directory symlinks.
    pub follow_symlinks: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            file_names: vec!["AGENTS.md", "CLAUDE.md", "GEMINI.md"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            path_suffixes: vec![".github/copilot-instructions.md".to_owned()],
            directory_suffixes: vec![".cursor/rules".to_owned()],
            ignored_directories: vec![
                ".git",
                ".mypy_cache",
                ".pytest_cache",
                ".ruff_cache",
                ".venv",
                "__pycache__",
                "build",
                "dist",
                "node_modules",
                "target",
                "venv",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            max_file_bytes: 1_048_576,
            max_files: 1_000_000,
            follow_symlinks: false,
        }
    }
}

/// Configuration for one plugin implementation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PluginConfig {
    /// Stable plugin identifier.
    pub id: String,
    /// Whether this plugin should execute.
    pub enabled: bool,
    /// Adapter-owned options. Core never interprets these values.
    pub options: BTreeMap<String, String>,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            enabled: true,
            options: BTreeMap::new(),
        }
    }
}

/// Configuration for one outbound integration adapter.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct IntegrationConfig {
    /// Stable adapter identifier.
    pub id: String,
    /// Whether the host should publish through this adapter.
    pub enabled: bool,
    /// Adapter-owned options. Core never interprets these values.
    pub options: BTreeMap<String, String>,
}
