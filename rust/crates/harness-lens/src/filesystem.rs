// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

use harness_lens_core::{
    AnalysisEngine, AnalysisReport, DiscoveryConfig, Finding, HarnessLensConfig, HarnessSource,
    HarnessSourceKind, IncompleteReason, Plugin, RegistrationError, ScanCompleteness, Severity,
};

const FILESYSTEM_SOURCE: &str = "harness-lens.filesystem";

/// Filesystem discovery or loading failure.
#[derive(Debug)]
pub enum ScanError {
    /// Root does not exist.
    MissingRoot(PathBuf),
    /// Root is not a directory.
    NotDirectory(PathBuf),
    /// Directory could not be enumerated.
    ReadDirectory {
        /// Directory path.
        path: PathBuf,
        /// Filesystem failure.
        source: std::io::Error,
    },
}

impl fmt::Display for ScanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRoot(path) => write!(formatter, "path does not exist: {}", path.display()),
            Self::NotDirectory(path) => {
                write!(formatter, "path is not a directory: {}", path.display())
            }
            Self::ReadDirectory { path, source } => {
                write!(
                    formatter,
                    "cannot read directory {}: {source}",
                    path.display()
                )
            }
        }
    }
}

impl Error for ScanError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadDirectory { source, .. } => Some(source),
            Self::MissingRoot(_) | Self::NotDirectory(_) => None,
        }
    }
}

/// Reusable SDK entry point combining filesystem and core adapters.
pub struct Scanner {
    engine: AnalysisEngine,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            engine: AnalysisEngine::new(),
        }
    }
}

impl Scanner {
    /// Creates a scanner with first-party deterministic plugins.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an in-process plugin without changing core or adapter code.
    pub fn register_plugin(
        &mut self,
        plugin: impl Plugin + 'static,
    ) -> Result<(), RegistrationError> {
        self.engine.register(plugin)
    }

    /// Discovers, safely loads, and analyzes one workspace.
    pub fn scan(
        &self,
        root: impl AsRef<Path>,
        config: &HarnessLensConfig,
    ) -> Result<AnalysisReport, ScanError> {
        self.scan_with_overrides(root, config, &BTreeMap::new())
    }

    /// Analyzes a workspace while replacing selected source contents in memory.
    ///
    /// Overlay keys may be relative to the workspace or absolute paths inside
    /// it. This is used by editors to analyze unsaved buffers without writing
    /// them to disk.
    pub fn scan_with_overrides(
        &self,
        root: impl AsRef<Path>,
        config: &HarnessLensConfig,
        overrides: &BTreeMap<PathBuf, String>,
    ) -> Result<AnalysisReport, ScanError> {
        let root = validate_root(root.as_ref())?;
        let mut discovery = collect_paths(&root, &config.discovery);
        let overrides = normalize_overrides(&root, overrides, &config.discovery);
        discovery.paths.extend(overrides.keys().cloned());
        discovery.paths.sort_by_key(|left| normalized(left));
        discovery.paths.dedup();

        let (sources, findings, load_reasons) =
            load_sources(&root, discovery.paths, &config.discovery, &overrides);
        discovery.completeness.reasons.extend(load_reasons);
        normalize_reasons(&mut discovery.completeness);

        let mut report = self.engine.analyze(root, sources, findings, config);
        report.completeness = discovery.completeness;
        Ok(report)
    }
}

/// Deterministic discovery output, including conditions that limited coverage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryResult {
    /// Recognized paths relative to the workspace root.
    pub paths: Vec<PathBuf>,
    /// Whether every encountered path could be considered safely.
    pub completeness: ScanCompleteness,
}

/// Returns recognized harness paths without loading their contents.
pub fn discover(
    root: impl AsRef<Path>,
    config: &HarnessLensConfig,
) -> Result<Vec<PathBuf>, ScanError> {
    discover_detailed(root, config).map(|result| result.paths)
}

/// Returns recognized paths plus explicit discovery completeness.
pub fn discover_detailed(
    root: impl AsRef<Path>,
    config: &HarnessLensConfig,
) -> Result<DiscoveryResult, ScanError> {
    let root = validate_root(root.as_ref())?;
    Ok(collect_paths(&root, &config.discovery))
}

fn validate_root(root: &Path) -> Result<PathBuf, ScanError> {
    if !root.exists() {
        return Err(ScanError::MissingRoot(root.to_owned()));
    }
    if !root.is_dir() {
        return Err(ScanError::NotDirectory(root.to_owned()));
    }
    Ok(root.canonicalize().unwrap_or_else(|_| root.to_owned()))
}

fn collect_paths(root: &Path, config: &DiscoveryConfig) -> DiscoveryResult {
    let mut state = WalkState {
        root,
        config,
        matches: Vec::new(),
        visited: HashSet::new(),
        visited_files: HashSet::new(),
        pending_symlinks: BTreeSet::new(),
        reasons: Vec::new(),
        file_count: 0,
        stopped: false,
    };
    state.walk_directory(root);
    state.walk_symlinks();
    state.matches.sort_by_key(|left| normalized(left));
    state.matches.dedup();
    let mut completeness = ScanCompleteness {
        complete: state.reasons.is_empty(),
        reasons: state.reasons,
    };
    normalize_reasons(&mut completeness);
    DiscoveryResult {
        paths: state.matches,
        completeness,
    }
}

struct WalkState<'a> {
    root: &'a Path,
    config: &'a DiscoveryConfig,
    matches: Vec<PathBuf>,
    visited: HashSet<PathBuf>,
    visited_files: HashSet<PathBuf>,
    pending_symlinks: BTreeSet<PathBuf>,
    reasons: Vec<IncompleteReason>,
    file_count: usize,
    stopped: bool,
}

impl WalkState<'_> {
    fn walk_directory(&mut self, directory: &Path) {
        if self.stopped {
            return;
        }
        let canonical = match directory.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                self.record_reason("unreadable-directory", directory);
                return;
            }
        };
        if !canonical.starts_with(self.root) {
            self.record_reason("outside-root-symlink", directory);
            return;
        }
        if !self.visited.insert(canonical) {
            return;
        }

        let entries = match fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(_) => {
                self.record_reason("unreadable-directory", directory);
                return;
            }
        };
        let mut entries = entries
            .filter_map(|entry| match entry {
                Ok(entry) => Some(entry),
                Err(_) => {
                    self.record_reason("unreadable-path", directory);
                    None
                }
            })
            .collect::<Vec<_>>();
        entries.sort_by_key(fs::DirEntry::file_name);

        for entry in entries {
            if self.stopped {
                break;
            }
            self.walk_entry(entry);
        }
    }

    fn walk_entry(&mut self, entry: fs::DirEntry) {
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => {
                self.record_reason("unreadable-path", &path);
                return;
            }
        };
        let is_symlink = file_type.is_symlink();
        if is_symlink {
            if self.config.follow_symlinks {
                self.pending_symlinks.insert(path);
            }
            return;
        }

        let relative = match path.strip_prefix(self.root) {
            Ok(relative) => relative,
            Err(_) => return,
        };
        if file_type.is_dir() {
            if !should_ignore_directory(relative, self.config) {
                self.walk_directory(&path);
            }
        } else if file_type.is_file() {
            self.visited_files.insert(path.clone());
            self.consider_file(&path, relative);
        }
    }

    fn walk_symlinks(&mut self) {
        while !self.stopped {
            let Some(path) = self.pending_symlinks.pop_first() else {
                break;
            };
            let target = match path.canonicalize() {
                Ok(target) => target,
                Err(_) => {
                    self.record_reason("unreadable-path", &path);
                    continue;
                }
            };
            if !target.starts_with(self.root) {
                self.record_reason("outside-root-symlink", &path);
                continue;
            }
            let metadata = match fs::metadata(&path) {
                Ok(metadata) => metadata,
                Err(_) => {
                    self.record_reason("unreadable-path", &path);
                    continue;
                }
            };
            let relative = match path.strip_prefix(self.root) {
                Ok(relative) => relative,
                Err(_) => continue,
            };
            if metadata.is_dir() {
                if !should_ignore_directory(relative, self.config) {
                    self.walk_directory(&path);
                }
            } else if metadata.is_file() && self.visited_files.insert(target) {
                self.consider_file(&path, relative);
            }
        }
    }

    fn consider_file(&mut self, path: &Path, relative: &Path) {
        if self.file_count >= self.config.max_files {
            self.record_reason("file-count-limit", path);
            self.stopped = true;
            return;
        }
        self.file_count += 1;
        if is_harness_path(relative, self.config) {
            self.matches.push(relative.to_owned());
        }
    }

    fn record_reason(&mut self, code: &str, path: &Path) {
        self.reasons.push(IncompleteReason {
            code: code.to_owned(),
            path: Some(report_path(self.root, path)),
        });
    }
}

fn should_ignore_directory(path: &Path, config: &DiscoveryConfig) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            config
                .ignored_directories
                .iter()
                .any(|ignored| ignored == name)
        })
}

/// Returns whether a root-relative path matches configured harness rules.
#[must_use]
pub fn is_harness_path(path: &Path, config: &DiscoveryConfig) -> bool {
    let file_name_match = path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| config.file_names.iter().any(|candidate| candidate == name));
    if file_name_match {
        return true;
    }

    let normalized_path = normalized(path);
    if config
        .path_suffixes
        .iter()
        .any(|suffix| has_path_suffix(&normalized_path, suffix))
    {
        return true;
    }

    let parent = normalized(path.parent().unwrap_or_else(|| Path::new("")));
    config
        .directory_suffixes
        .iter()
        .any(|suffix| contains_directory_suffix(&parent, suffix))
}

fn has_path_suffix(path: &str, suffix: &str) -> bool {
    let suffix = suffix.trim_matches('/');
    path == suffix || path.ends_with(&format!("/{suffix}"))
}

fn contains_directory_suffix(path: &str, suffix: &str) -> bool {
    let suffix = suffix.trim_matches('/');
    path == suffix
        || path.starts_with(&format!("{suffix}/"))
        || path.ends_with(&format!("/{suffix}"))
        || path.contains(&format!("/{suffix}/"))
}

fn normalized(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn load_sources(
    root: &Path,
    paths: Vec<PathBuf>,
    config: &DiscoveryConfig,
    overrides: &BTreeMap<PathBuf, String>,
) -> (Vec<HarnessSource>, Vec<Finding>, Vec<IncompleteReason>) {
    let mut sources = Vec::new();
    let mut findings = Vec::new();
    let mut reasons = Vec::new();

    for relative in paths {
        if let Some(content) = overrides.get(&relative) {
            if content.len() as u64 > config.max_file_bytes {
                findings.push(load_finding(
                    &relative,
                    format!(
                        "in-memory source exceeds configured limit of {} bytes",
                        config.max_file_bytes
                    ),
                ));
                reasons.push(incomplete_reason("source-size-limit", &relative));
            } else {
                sources.push(make_source(relative, content.clone()));
            }
            continue;
        }

        let absolute = root.join(&relative);
        match fs::metadata(&absolute) {
            Ok(metadata) if metadata.len() > config.max_file_bytes => {
                findings.push(load_finding(
                    &relative,
                    format!(
                        "file exceeds configured limit of {} bytes",
                        config.max_file_bytes
                    ),
                ));
                reasons.push(incomplete_reason("source-size-limit", &relative));
            }
            Ok(_) => match fs::read_to_string(&absolute) {
                Ok(content) => sources.push(make_source(relative, content)),
                Err(error) => {
                    findings.push(load_finding(&relative, error.to_string()));
                    reasons.push(incomplete_reason("unreadable-path", &relative));
                }
            },
            Err(error) => {
                findings.push(load_finding(&relative, error.to_string()));
                reasons.push(incomplete_reason("unreadable-path", &relative));
            }
        }
    }

    (sources, findings, reasons)
}

fn make_source(path: PathBuf, content: String) -> HarnessSource {
    HarnessSource {
        kind: source_kind(&path),
        scope: path.parent().unwrap_or_else(|| Path::new("")).to_owned(),
        path,
        content,
    }
}

fn load_finding(path: &Path, message: String) -> Finding {
    Finding {
        severity: Severity::Warning,
        rule_id: "HL006".to_owned(),
        message: "Harness source could not be loaded safely".to_owned(),
        path: Some(path.to_owned()),
        line: None,
        span: None,
        evidence: Some(message),
        source: FILESYSTEM_SOURCE.to_owned(),
    }
}

fn normalize_overrides(
    root: &Path,
    overrides: &BTreeMap<PathBuf, String>,
    config: &DiscoveryConfig,
) -> BTreeMap<PathBuf, String> {
    overrides
        .iter()
        .filter_map(|(path, content)| {
            let relative = if path.is_absolute() {
                path.strip_prefix(root).ok()?.to_owned()
            } else {
                path.clone()
            };
            if !is_safe_relative(&relative) || !is_harness_path(&relative, config) {
                return None;
            }
            Some((relative, content.clone()))
        })
        .collect()
}

fn is_safe_relative(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}

fn report_path(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root)
        .map(Path::to_owned)
        .unwrap_or_else(|_| path.to_owned())
}

fn incomplete_reason(code: &str, path: &Path) -> IncompleteReason {
    IncompleteReason {
        code: code.to_owned(),
        path: Some(path.to_owned()),
    }
}

fn normalize_reasons(completeness: &mut ScanCompleteness) {
    completeness.reasons.sort_by(|left, right| {
        left.code
            .cmp(&right.code)
            .then_with(|| left.path.cmp(&right.path))
    });
    completeness.reasons.dedup();
    completeness.complete = completeness.reasons.is_empty();
}

fn source_kind(path: &Path) -> HarnessSourceKind {
    match path.file_name().and_then(|name| name.to_str()) {
        Some("AGENTS.md") => HarnessSourceKind::Agents,
        Some("CLAUDE.md" | "GEMINI.md" | "copilot-instructions.md") => {
            HarnessSourceKind::Instructions
        }
        _ if normalized(path).contains(".cursor/rules/") => HarnessSourceKind::Rules,
        _ => HarnessSourceKind::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_root() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("harness-lens-{nonce}"));
        fs::create_dir_all(&root).unwrap();
        root
    }

    #[test]
    fn discovers_default_sources_and_ignores_build_outputs() {
        let root = test_root();
        fs::write(root.join("AGENTS.md"), "# Instructions").unwrap();
        fs::create_dir_all(root.join(".cursor/rules/languages")).unwrap();
        fs::write(root.join(".cursor/rules/languages/rust.md"), "# Rust").unwrap();
        fs::create_dir_all(root.join("target")).unwrap();
        fs::write(root.join("target/CLAUDE.md"), "ignored").unwrap();

        let found = discover(&root, &HarnessLensConfig::default()).unwrap();

        assert_eq!(
            found
                .iter()
                .map(|path| normalized(path))
                .collect::<Vec<_>>(),
            [".cursor/rules/languages/rust.md", "AGENTS.md"]
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn scan_returns_content_free_report_and_plugin_trace() {
        let root = test_root();
        fs::write(root.join("AGENTS.md"), "secret-free content").unwrap();

        let report = Scanner::new()
            .scan(&root, &HarnessLensConfig::default())
            .unwrap();

        assert_eq!(report.sources.len(), 1);
        assert_eq!(report.sources[0].bytes, 19);
        assert_eq!(report.plugin_executions.len(), 3);
        assert_eq!(report.scores.len(), 3);
        assert!(report.completeness.complete);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn file_fuse_returns_a_deterministic_incomplete_scan() {
        let root = test_root();
        fs::write(root.join("AGENTS.md"), "first").unwrap();
        fs::write(root.join("CLAUDE.md"), "second").unwrap();
        let mut config = HarnessLensConfig::default();
        config.discovery.max_files = 1;

        let result = discover_detailed(&root, &config).unwrap();

        assert_eq!(result.paths, [PathBuf::from("AGENTS.md")]);
        assert!(!result.completeness.complete);
        assert_eq!(result.completeness.reasons[0].code, "file-count-limit");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn in_memory_overlay_is_analyzed_without_touching_disk() {
        let root = test_root();
        fs::write(root.join("AGENTS.md"), "Always run tests.\n").unwrap();
        let overrides = BTreeMap::from([(
            PathBuf::from("AGENTS.md"),
            "Never run run tests.\n".to_owned(),
        )]);

        let report = Scanner::new()
            .scan_with_overrides(&root, &HarnessLensConfig::default(), &overrides)
            .unwrap();

        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.rule_id == "HL010")
        );
        assert_eq!(
            fs::read_to_string(root.join("AGENTS.md")).unwrap(),
            "Always run tests.\n"
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn followed_symlinks_cannot_escape_the_workspace() {
        use std::os::unix::fs::symlink;

        let root = test_root();
        let outside = test_root();
        fs::write(outside.join("AGENTS.md"), "outside").unwrap();
        symlink(&outside, root.join("linked")).unwrap();
        let mut config = HarnessLensConfig::default();
        config.discovery.follow_symlinks = true;

        let result = discover_detailed(&root, &config).unwrap();

        assert!(result.paths.is_empty());
        assert_eq!(result.completeness.reasons[0].code, "outside-root-symlink");
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(outside).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn physical_directory_wins_before_nested_symlink_alias() {
        use std::os::unix::fs::symlink;

        let root = test_root();
        fs::create_dir_all(root.join("a")).unwrap();
        fs::create_dir_all(root.join("z/real")).unwrap();
        fs::write(root.join("z/real/AGENTS.md"), "physical").unwrap();
        symlink(root.join("z/real"), root.join("a/alias")).unwrap();
        let mut config = HarnessLensConfig::default();
        config.discovery.follow_symlinks = true;

        let result = discover_detailed(&root, &config).unwrap();

        assert_eq!(result.paths, [PathBuf::from("z/real/AGENTS.md")]);
        assert!(result.completeness.complete);
        fs::remove_dir_all(root).unwrap();
    }
}
