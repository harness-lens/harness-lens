// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

//! Private PyO3 extension backing the public Python SDK.

use std::path::{Path, PathBuf};

use harness_lens::{HarnessLensConfig, Scanner, load_for_root};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

#[pyfunction(signature = (root=None, config_path=None))]
fn discover(root: Option<&str>, config_path: Option<&str>) -> PyResult<Vec<String>> {
    let root = PathBuf::from(root.unwrap_or("."));
    let config = resolve_config(&root, config_path)?;
    harness_lens::discover(&root, &config)
        .map(|paths| {
            paths
                .into_iter()
                .map(|path| path.to_string_lossy().replace('\\', "/"))
                .collect()
        })
        .map_err(|error| PyValueError::new_err(error.to_string()))
}

#[pyfunction(signature = (root=None, config_path=None))]
fn scan_json(root: Option<&str>, config_path: Option<&str>) -> PyResult<String> {
    let root = PathBuf::from(root.unwrap_or("."));
    let config = resolve_config(&root, config_path)?;
    let report = Scanner::new()
        .scan(&root, &config)
        .map_err(|error| PyValueError::new_err(error.to_string()))?;
    serde_json::to_string(&report).map_err(|error| PyRuntimeError::new_err(error.to_string()))
}

#[pyfunction]
fn core_version() -> &'static str {
    harness_lens::VERSION
}

fn resolve_config(root: &Path, config_path: Option<&str>) -> PyResult<HarnessLensConfig> {
    load_for_root(root, config_path.map(Path::new))
        .map_err(|error| PyValueError::new_err(error.to_string()))
}

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(discover, module)?)?;
    module.add_function(wrap_pyfunction!(scan_json, module)?)?;
    module.add_function(wrap_pyfunction!(core_version, module)?)?;
    Ok(())
}
