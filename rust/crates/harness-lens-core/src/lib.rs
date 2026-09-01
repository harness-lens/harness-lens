// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

#![doc = include_str!("../README.md")]

/// A kind of harness source that can affect an agent workspace.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
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
}

/// Aggregate, content-free result of a workspace scan.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScanSummary {
    /// Number of recognized harness sources.
    pub sources: usize,
    /// Number of diagnostics produced while resolving sources.
    pub diagnostics: usize,
}

impl ScanSummary {
    /// Returns whether the scan found no harness sources or diagnostics.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.sources == 0 && self.diagnostics == 0
    }
}
