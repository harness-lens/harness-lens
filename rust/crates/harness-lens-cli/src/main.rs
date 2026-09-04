// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

#![doc = include_str!("../README.md")]

use std::path::PathBuf;

use harness_lens::{Scanner, load_for_root};

fn main() {
    if let Err(error) = run(std::env::args().skip(1)) {
        eprintln!("harness-lens: {error}");
        std::process::exit(2);
    }
}

fn run(arguments: impl Iterator<Item = String>) -> Result<(), String> {
    let mut root = PathBuf::from(".");
    let mut config_path = None;
    let mut json = false;
    let mut arguments = arguments.peekable();

    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--json" => json = true,
            "--version" | "-V" => {
                println!("harness-lens {}", harness_lens::VERSION);
                return Ok(());
            }
            "--config" => {
                let value = arguments
                    .next()
                    .ok_or_else(|| "--config requires a path".to_owned())?;
                config_path = Some(PathBuf::from(value));
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            value if value.starts_with('-') => return Err(format!("unknown option: {value}")),
            value => root = PathBuf::from(value),
        }
    }

    let config = load_for_root(&root, config_path.as_deref()).map_err(|error| error.to_string())?;
    let report = Scanner::new()
        .scan(&root, &config)
        .map_err(|error| error.to_string())?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?
        );
    } else {
        let summary = report.summary();
        println!(
            "Harness Lens found {} harness source(s) under {}",
            summary.sources,
            report.root.display()
        );
        for source in report.sources {
            println!("- {}", source.path.display());
        }
        if summary.diagnostics > 0 {
            println!("{} warning/error finding(s)", summary.diagnostics);
        }
    }

    Ok(())
}

fn print_help() {
    println!(
        "Harness Lens {}\n\nUsage: harness-lens [PATH] [--config FILE] [--json]",
        harness_lens::VERSION
    );
}
