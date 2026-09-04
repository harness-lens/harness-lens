// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

#![doc = include_str!("../README.md")]

use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use harness_lens_core::HarnessLensConfig;

/// Default repository-local configuration file name.
pub const DEFAULT_CONFIG_FILE: &str = "harness-lens.toml";

/// Configuration loading or validation failure.
#[derive(Debug)]
pub enum ConfigError {
    /// File could not be read.
    Read {
        /// Attempted file path.
        path: PathBuf,
        /// Filesystem failure.
        source: std::io::Error,
    },
    /// TOML could not be converted into the versioned schema.
    Parse {
        /// Optional source path.
        path: Option<PathBuf>,
        /// TOML failure.
        source: toml::de::Error,
    },
    /// Schema version is not supported by this build.
    UnsupportedVersion(u32),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(
                    formatter,
                    "failed to read config {}: {source}",
                    path.display()
                )
            }
            Self::Parse { path, source } => match path {
                Some(path) => write!(formatter, "invalid config {}: {source}", path.display()),
                None => write!(formatter, "invalid config: {source}"),
            },
            Self::UnsupportedVersion(version) => {
                write!(formatter, "unsupported config version: {version}")
            }
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
            Self::UnsupportedVersion(_) => None,
        }
    }
}

/// Parses TOML from memory.
pub fn parse(source: &str) -> Result<HarnessLensConfig, ConfigError> {
    parse_with_path(source, None)
}

/// Loads a TOML configuration file.
pub fn load(path: impl AsRef<Path>) -> Result<HarnessLensConfig, ConfigError> {
    let path = path.as_ref();
    let source = fs::read_to_string(path).map_err(|source| ConfigError::Read {
        path: path.to_owned(),
        source,
    })?;
    parse_with_path(&source, Some(path.to_owned()))
}

/// Loads an explicit path, repository-local default, or built-in defaults.
pub fn load_for_root(
    root: impl AsRef<Path>,
    explicit: Option<&Path>,
) -> Result<HarnessLensConfig, ConfigError> {
    if let Some(path) = explicit {
        return load(path);
    }
    let candidate = root.as_ref().join(DEFAULT_CONFIG_FILE);
    if candidate.is_file() {
        load(candidate)
    } else {
        Ok(HarnessLensConfig::default())
    }
}

fn parse_with_path(source: &str, path: Option<PathBuf>) -> Result<HarnessLensConfig, ConfigError> {
    let config: HarnessLensConfig =
        toml::from_str(source).map_err(|source| ConfigError::Parse { path, source })?;
    if config.version != 1 {
        return Err(ConfigError::UnsupportedVersion(config.version));
    }
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_versioned_plugin_configuration() {
        let config = parse(
            r#"
                version = 1

                [[plugins]]
                id = "harness-score"
                enabled = false

                [plugins.options]
                project = "demo"

                [[integrations]]
                id = "harness-score"
                enabled = false
            "#,
        )
        .unwrap();

        let plugin = config.plugin("harness-score").unwrap();
        assert!(!plugin.enabled);
        assert_eq!(plugin.options["project"], "demo");
        assert!(!config.integration("harness-score").unwrap().enabled);
    }

    #[test]
    fn rejects_unknown_schema_versions() {
        let error = parse("version = 9").unwrap_err();
        assert!(matches!(error, ConfigError::UnsupportedVersion(9)));
    }
}
