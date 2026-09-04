// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

use std::collections::BTreeMap;

use crate::{
    Finding, Metric, Plugin, PluginContext, PluginError, PluginMetadata, PluginOutput, Score,
    ScoreCategory, ScoreMethod, Severity, TextSpan,
};

const REPETITION_PLUGIN_ID: &str = "harness-lens.repetition";
const INCONGRUENCE_PLUGIN_ID: &str = "harness-lens.incongruence";

pub(crate) struct RepetitionPlugin;

impl Plugin for RepetitionPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: REPETITION_PLUGIN_ID.to_owned(),
            name: "Adjacent word repetition".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            capabilities: vec!["text.repetition".to_owned()],
            default_enabled: true,
        }
    }

    fn analyze(&self, context: &PluginContext<'_>) -> Result<PluginOutput, PluginError> {
        let mut findings = Vec::new();
        let mut compared_pairs = 0_usize;

        for source in context.sources {
            let mut in_fence = false;
            for (line_number, line_start, line) in source_lines(&source.content) {
                if is_fence(line) {
                    in_fence = !in_fence;
                    continue;
                }
                if in_fence {
                    continue;
                }

                let words = words(line);
                for pair in words.windows(2) {
                    compared_pairs += 1;
                    if pair[0].has_alphabetic
                        && pair[1].has_alphabetic
                        && pair[0].normalized == pair[1].normalized
                    {
                        findings.push(Finding {
                            severity: Severity::Warning,
                            rule_id: "HL010".to_owned(),
                            message: "Adjacent word repetition".to_owned(),
                            path: Some(source.path.clone()),
                            line: Some(line_number),
                            span: Some(TextSpan {
                                start: line_start + pair[1].start,
                                end: line_start + pair[1].end,
                            }),
                            evidence: Some(
                                "same normalized word appears twice consecutively".to_owned(),
                            ),
                            source: REPETITION_PLUGIN_ID.to_owned(),
                        });
                    }
                }
            }
        }

        let finding_count = findings.len();
        let message = if finding_count == 0 {
            "No adjacent repeated words found"
        } else {
            "One or more adjacent repeated words found"
        };
        let mut score = Score::new(
            "harness.repetition_free",
            ScoreCategory::Quality,
            ScoreMethod::Deterministic,
            f64::from(finding_count == 0),
            1.0,
            message,
            REPETITION_PLUGIN_ID,
        )
        .expect("built-in score is normalized");
        score.sample_size = Some(compared_pairs);
        score
            .evidence
            .insert("finding_count".to_owned(), finding_count.to_string());

        Ok(PluginOutput {
            findings,
            metrics: vec![Metric {
                name: "harness.adjacent_repetitions".to_owned(),
                value: finding_count as f64,
                unit: Some("count".to_owned()),
                source: REPETITION_PLUGIN_ID.to_owned(),
            }],
            scores: vec![score],
        })
    }
}

pub(crate) struct IncongruencePlugin;

impl Plugin for IncongruencePlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: INCONGRUENCE_PLUGIN_ID.to_owned(),
            name: "Strong instruction incongruence".to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            capabilities: vec!["text.incongruence".to_owned()],
            default_enabled: true,
        }
    }

    fn analyze(&self, context: &PluginContext<'_>) -> Result<PluginOutput, PluginError> {
        let clauses = context
            .sources
            .iter()
            .flat_map(extract_clauses)
            .collect::<Vec<_>>();
        let mut groups: BTreeMap<(ModalFamily, String), ModalGroup> = BTreeMap::new();

        for clause in clauses {
            let group = groups
                .entry((clause.family, clause.target.clone()))
                .or_default();
            match clause.polarity {
                Polarity::Positive => group.positive.push(clause),
                Polarity::Negative => group.negative.push(clause),
            }
        }

        let mut findings = Vec::new();
        let mut conflicting_groups = 0_usize;
        for group in groups.values() {
            let mut group_conflicts = false;
            for clause in &group.positive {
                if let Some(opposite) = group
                    .negative
                    .iter()
                    .find(|opposite| scopes_overlap(&clause.scope, &opposite.scope))
                {
                    add_conflict_finding(&mut findings, clause, opposite);
                    group_conflicts = true;
                }
            }
            for clause in &group.negative {
                if let Some(opposite) = group
                    .positive
                    .iter()
                    .find(|opposite| scopes_overlap(&clause.scope, &opposite.scope))
                {
                    add_conflict_finding(&mut findings, clause, opposite);
                    group_conflicts = true;
                }
            }
            if group_conflicts {
                conflicting_groups += 1;
            }
        }
        findings.sort_by(|left, right| {
            left.path
                .cmp(&right.path)
                .then(left.line.cmp(&right.line))
                .then(left.rule_id.cmp(&right.rule_id))
        });

        let message = if conflicting_groups == 0 {
            "No exact opposite strong instructions found"
        } else {
            "Exact opposite strong instructions found"
        };
        let mut score = Score::new(
            "harness.incongruence_free",
            ScoreCategory::Quality,
            ScoreMethod::Heuristic,
            f64::from(conflicting_groups == 0),
            1.0,
            message,
            INCONGRUENCE_PLUGIN_ID,
        )
        .expect("built-in score is normalized");
        score.sample_size = Some(groups.len());
        score.evidence.insert(
            "assumption".to_owned(),
            "only exact normalized always/never and must/must-not pairs conflict".to_owned(),
        );
        score.evidence.insert(
            "conflicting_group_count".to_owned(),
            conflicting_groups.to_string(),
        );

        Ok(PluginOutput {
            findings,
            metrics: vec![Metric {
                name: "harness.incongruent_instruction_groups".to_owned(),
                value: conflicting_groups as f64,
                unit: Some("count".to_owned()),
                source: INCONGRUENCE_PLUGIN_ID.to_owned(),
            }],
            scores: vec![score],
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ModalFamily {
    Frequency,
    Obligation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Polarity {
    Positive,
    Negative,
}

#[derive(Clone, Debug)]
struct Clause {
    family: ModalFamily,
    polarity: Polarity,
    target: String,
    path: std::path::PathBuf,
    scope: std::path::PathBuf,
    line: usize,
    span: TextSpan,
}

#[derive(Default)]
struct ModalGroup {
    positive: Vec<Clause>,
    negative: Vec<Clause>,
}

fn add_conflict_finding(findings: &mut Vec<Finding>, clause: &Clause, opposite: &Clause) {
    findings.push(Finding {
        severity: Severity::Warning,
        rule_id: "HL020".to_owned(),
        message: format!(
            "Strong instruction conflicts with opposite instruction at {}:{}",
            opposite.path.display(),
            opposite.line
        ),
        path: Some(clause.path.clone()),
        line: Some(clause.line),
        span: Some(clause.span),
        evidence: Some("same normalized modal target in an overlapping harness scope".to_owned()),
        source: INCONGRUENCE_PLUGIN_ID.to_owned(),
    });
}

fn scopes_overlap(left: &std::path::Path, right: &std::path::Path) -> bool {
    left.starts_with(right) || right.starts_with(left)
}

fn extract_clauses(source: &crate::HarnessSource) -> Vec<Clause> {
    let mut clauses = Vec::new();
    let mut in_fence = false;
    for (line_number, line_start, line) in source_lines(&source.content) {
        if is_fence(line) {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }

        let Some((text, local_start)) = instruction_text(line) else {
            continue;
        };
        let normalized = normalize_clause(text);
        let Some((family, polarity, target)) = parse_modal(&normalized) else {
            continue;
        };
        if target.is_empty() {
            continue;
        }

        clauses.push(Clause {
            family,
            polarity,
            target,
            path: source.path.clone(),
            scope: source.scope.clone(),
            line: line_number,
            span: TextSpan {
                start: line_start + local_start,
                end: line_start + local_start + text.len(),
            },
        });
    }
    clauses
}

fn parse_modal(text: &str) -> Option<(ModalFamily, Polarity, String)> {
    let text = text.strip_prefix("you ").unwrap_or(text);
    let cases = [
        ("always ", ModalFamily::Frequency, Polarity::Positive),
        ("never ", ModalFamily::Frequency, Polarity::Negative),
        ("must not ", ModalFamily::Obligation, Polarity::Negative),
        ("must ", ModalFamily::Obligation, Polarity::Positive),
    ];
    cases.iter().find_map(|(prefix, family, polarity)| {
        text.strip_prefix(prefix)
            .map(|target| (*family, *polarity, target.to_owned()))
    })
}

fn normalize_clause(text: &str) -> String {
    let lowered = text.to_lowercase().replace(['`', '*', '_'], "");
    lowered
        .trim_matches(|character: char| {
            character.is_whitespace()
                || matches!(
                    character,
                    '.' | ',' | ';' | ':' | '!' | '?' | '`' | '*' | '_'
                )
        })
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn instruction_text(line: &str) -> Option<(&str, usize)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    let leading = line.len() - line.trim_start().len();
    let mut text = line.trim_start();
    let mut removed = 0_usize;

    while let Some(rest) = text.strip_prefix('#') {
        text = rest;
        removed += 1;
    }
    text = trim_start_counted(text, &mut removed);
    if let Some(rest) = text
        .strip_prefix("- ")
        .or_else(|| text.strip_prefix("* "))
        .or_else(|| text.strip_prefix("+ "))
    {
        removed += 2;
        text = rest;
    }
    text = trim_start_counted(text, &mut removed);
    let text = text.trim_end_matches(['\r', '\n']).trim_end();
    (!text.is_empty()).then_some((text, leading + removed))
}

fn trim_start_counted<'a>(text: &'a str, removed: &mut usize) -> &'a str {
    let trimmed = text.trim_start();
    *removed += text.len() - trimmed.len();
    trimmed
}

fn source_lines(content: &str) -> impl Iterator<Item = (usize, usize, &str)> {
    let mut offset = 0_usize;
    content
        .split_inclusive('\n')
        .enumerate()
        .map(move |(index, line)| {
            let start = offset;
            offset += line.len();
            (index + 1, start, line)
        })
}

fn is_fence(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("```") || trimmed.starts_with("~~~")
}

#[derive(Debug)]
struct Word {
    start: usize,
    end: usize,
    normalized: String,
    has_alphabetic: bool,
}

fn words(line: &str) -> Vec<Word> {
    let mut output = Vec::new();
    let mut start = None;
    for (index, character) in line.char_indices() {
        if character.is_alphanumeric() {
            start.get_or_insert(index);
        } else if let Some(word_start) = start.take() {
            output.push(Word {
                start: word_start,
                end: index,
                normalized: line[word_start..index].to_lowercase(),
                has_alphabetic: line[word_start..index].chars().any(char::is_alphabetic),
            });
        }
    }
    if let Some(word_start) = start {
        output.push(Word {
            start: word_start,
            end: line.len(),
            normalized: line[word_start..].to_lowercase(),
            has_alphabetic: line[word_start..].chars().any(char::is_alphabetic),
        });
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HarnessLensConfig, HarnessSource, HarnessSourceKind, PluginContext};
    use std::path::PathBuf;

    fn source(path: &str, content: &str) -> HarnessSource {
        let path = PathBuf::from(path);
        HarnessSource {
            scope: path.parent().unwrap_or(std::path::Path::new("")).to_owned(),
            path,
            kind: HarnessSourceKind::Agents,
            content: content.to_owned(),
        }
    }

    fn context<'a>(
        sources: &'a [HarnessSource],
        config: &'a HarnessLensConfig,
    ) -> PluginContext<'a> {
        static OPTIONS: std::sync::LazyLock<BTreeMap<String, String>> =
            std::sync::LazyLock::new(BTreeMap::new);
        PluginContext {
            config,
            sources,
            options: &OPTIONS,
        }
    }

    #[test]
    fn repetition_is_case_insensitive_and_points_to_second_word() {
        let sources = [source("AGENTS.md", "Use use tests.\n")];
        let config = HarnessLensConfig::default();
        let output = RepetitionPlugin
            .analyze(&context(&sources, &config))
            .unwrap();

        assert_eq!(output.findings.len(), 1);
        assert_eq!(output.findings[0].line, Some(1));
        assert_eq!(output.findings[0].span, Some(TextSpan { start: 4, end: 7 }));
        assert_eq!(output.scores[0].method, ScoreMethod::Deterministic);
    }

    #[test]
    fn repetition_ignores_fenced_code() {
        let sources = [source("AGENTS.md", "```text\nrun run\n```\n")];
        let config = HarnessLensConfig::default();
        let output = RepetitionPlugin
            .analyze(&context(&sources, &config))
            .unwrap();

        assert!(output.findings.is_empty());
    }

    #[test]
    fn repetition_does_not_treat_decimal_parts_as_words() {
        let sources = [source("AGENTS.md", "Scores stay in [0.0, 1.0].\n")];
        let config = HarnessLensConfig::default();
        let output = RepetitionPlugin
            .analyze(&context(&sources, &config))
            .unwrap();

        assert!(output.findings.is_empty());
    }

    #[test]
    fn incongruence_matches_exact_opposite_modal_targets_across_files() {
        let sources = [
            source("AGENTS.md", "Always run tests.\n"),
            source("nested/AGENTS.md", "- Never run tests!\n"),
        ];
        let config = HarnessLensConfig::default();
        let output = IncongruencePlugin
            .analyze(&context(&sources, &config))
            .unwrap();

        assert_eq!(output.findings.len(), 2);
        assert!(
            output
                .findings
                .iter()
                .all(|finding| finding.rule_id == "HL020")
        );
        assert_eq!(output.scores[0].method, ScoreMethod::Heuristic);
    }

    #[test]
    fn incongruence_does_not_match_different_targets() {
        let sources = [source(
            "AGENTS.md",
            "You must run unit tests.\nYou must not publish releases.\n",
        )];
        let config = HarnessLensConfig::default();
        let output = IncongruencePlugin
            .analyze(&context(&sources, &config))
            .unwrap();

        assert!(output.findings.is_empty());
    }

    #[test]
    fn incongruence_does_not_cross_disjoint_sibling_scopes() {
        let sources = [
            source("frontend/AGENTS.md", "Always run frontend tests.\n"),
            source("backend/AGENTS.md", "Never run frontend tests.\n"),
        ];
        let config = HarnessLensConfig::default();
        let output = IncongruencePlugin
            .analyze(&context(&sources, &config))
            .unwrap();

        assert!(output.findings.is_empty());
    }
}
