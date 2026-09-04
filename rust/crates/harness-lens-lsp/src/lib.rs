// SPDX-License-Identifier: MPL-2.0
// Copyright © 2026 Cristian Camargo Filho

#![doc = include_str!("../README.md")]

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use harness_lens::{Finding, Scanner, Severity, TextSpan, is_harness_path, load_for_root};
use tokio::sync::RwLock;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams, InitializeParams, InitializeResult,
    InitializedParams, MessageType, NumberOrString, Position, PositionEncodingKind, Range,
    ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
};
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

const DIAGNOSTIC_SOURCE: &str = "harness-lens";

#[derive(Default)]
struct State {
    roots: Vec<PathBuf>,
    open_documents: BTreeMap<PathBuf, OpenDocument>,
}

#[derive(Clone)]
struct OpenDocument {
    uri: Uri,
    text: String,
}

/// Harness Lens language server backend.
pub struct Backend {
    client: Client,
    state: RwLock<State>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            state: RwLock::new(State::default()),
        }
    }

    async fn analyze_open_documents(&self) {
        let (roots, documents) = {
            let state = self.state.read().await;
            (state.roots.clone(), state.open_documents.clone())
        };
        let mut published = BTreeSet::new();

        for root in &roots {
            let documents_for_root = documents
                .iter()
                .filter(|(path, _)| root_for_path(path, &roots) == Some(root.as_path()))
                .map(|(path, document)| (path.clone(), document.text.clone()))
                .collect::<BTreeMap<_, _>>();
            if documents_for_root.is_empty() {
                continue;
            }

            let config = match load_for_root(root, None) {
                Ok(config) => config,
                Err(error) => {
                    self.client
                        .log_message(MessageType::ERROR, format!("Harness Lens config: {error}"))
                        .await;
                    continue;
                }
            };
            let report =
                match Scanner::new().scan_with_overrides(root, &config, &documents_for_root) {
                    Ok(report) => report,
                    Err(error) => {
                        self.client
                            .log_message(MessageType::ERROR, format!("Harness Lens scan: {error}"))
                            .await;
                        continue;
                    }
                };
            if !report.completeness.complete {
                let reason_codes = report
                    .completeness
                    .reasons
                    .iter()
                    .map(|reason| reason.code.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                self.client
                    .log_message(
                        MessageType::WARNING,
                        format!("Harness Lens scan is incomplete: {reason_codes}"),
                    )
                    .await;
            }

            for (path, content) in &documents_for_root {
                let relative = match path.strip_prefix(root) {
                    Ok(relative) => relative,
                    Err(_) => continue,
                };
                if !is_harness_path(relative, &config.discovery) {
                    continue;
                }
                let Some(uri) = documents.get(path).map(|document| document.uri.clone()) else {
                    continue;
                };
                let diagnostics = report
                    .findings
                    .iter()
                    .filter(|finding| finding.path.as_deref() == Some(relative))
                    .filter_map(|finding| diagnostic_from_finding(finding, content))
                    .collect();
                self.client
                    .publish_diagnostics(uri.clone(), diagnostics, None)
                    .await;
                published.insert(path.clone());
            }
        }

        for (path, document) in documents {
            if published.contains(&path) {
                continue;
            }
            self.client
                .publish_diagnostics(document.uri, Vec::new(), None)
                .await;
        }
    }

    async fn put_document(&self, uri: &Uri, text: String) {
        let Some(path) = uri.to_file_path() else {
            return;
        };
        let path = path.into_owned();
        let path = path.canonicalize().unwrap_or(path);
        self.state.write().await.open_documents.insert(
            path,
            OpenDocument {
                uri: uri.clone(),
                text,
            },
        );
    }
}

impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let mut roots = params
            .workspace_folders
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(|folder| folder.uri.to_file_path().map(|path| path.into_owned()))
            .collect::<Vec<_>>();
        if roots.is_empty() {
            if let Some(root_uri) = legacy_root_uri(&params) {
                if let Some(path) = root_uri.to_file_path() {
                    roots.push(path.into_owned());
                }
            }
        }
        if roots.is_empty() {
            roots.push(
                PathBuf::from(".")
                    .canonicalize()
                    .unwrap_or_else(|_| PathBuf::from(".")),
            );
        }
        for root in &mut roots {
            *root = root.canonicalize().unwrap_or_else(|_| root.clone());
        }
        roots.sort();
        roots.dedup();
        self.state.write().await.roots = roots;

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                position_encoding: Some(PositionEncodingKind::UTF16),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "Harness Lens".to_owned(),
                version: Some(harness_lens::VERSION.to_owned()),
            }),
            offset_encoding: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Harness Lens language server ready")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.put_document(&params.text_document.uri, params.text_document.text)
            .await;
        self.analyze_open_documents().await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.put_document(&params.text_document.uri, change.text)
                .await;
            self.analyze_open_documents().await;
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(text) = params.text {
            self.put_document(&params.text_document.uri, text).await;
        }
        self.analyze_open_documents().await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        if let Some(path) = params.text_document.uri.to_file_path() {
            let path = path.into_owned();
            let path = path.canonicalize().unwrap_or(path);
            self.state.write().await.open_documents.remove(&path);
        }
        self.client
            .publish_diagnostics(params.text_document.uri, Vec::new(), None)
            .await;
        self.analyze_open_documents().await;
    }
}

#[allow(deprecated)]
fn legacy_root_uri(params: &InitializeParams) -> Option<Uri> {
    params.root_uri.clone()
}

/// Serves Harness Lens LSP over standard input/output.
pub async fn serve() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}

fn diagnostic_from_finding(finding: &Finding, content: &str) -> Option<Diagnostic> {
    if finding.severity == Severity::Pass {
        return None;
    }
    let range = finding
        .span
        .and_then(|span| range_from_byte_span(content, span))
        .or_else(|| finding.line.map(|line| whole_line_range(content, line)))
        .unwrap_or_default();
    Some(Diagnostic {
        range,
        severity: Some(match finding.severity {
            Severity::Error => DiagnosticSeverity::ERROR,
            Severity::Warning => DiagnosticSeverity::WARNING,
            Severity::Info => DiagnosticSeverity::INFORMATION,
            Severity::Pass => return None,
        }),
        code: Some(NumberOrString::String(finding.rule_id.clone())),
        source: Some(DIAGNOSTIC_SOURCE.to_owned()),
        message: finding.message.clone(),
        ..Diagnostic::default()
    })
}

fn range_from_byte_span(content: &str, span: TextSpan) -> Option<Range> {
    if span.start > span.end
        || span.end > content.len()
        || !content.is_char_boundary(span.start)
        || !content.is_char_boundary(span.end)
    {
        return None;
    }
    Some(Range::new(
        position_at_byte(content, span.start),
        position_at_byte(content, span.end),
    ))
}

fn whole_line_range(content: &str, one_based_line: usize) -> Range {
    let target = one_based_line.saturating_sub(1);
    let line = content.lines().nth(target).unwrap_or("");
    Range::new(
        Position::new(target as u32, 0),
        Position::new(target as u32, utf16_len(line)),
    )
}

fn position_at_byte(content: &str, byte: usize) -> Position {
    let prefix = &content[..byte];
    let line_start = prefix.rfind('\n').map_or(0, |index| index + 1);
    Position::new(
        prefix.bytes().filter(|byte| *byte == b'\n').count() as u32,
        utf16_len(&content[line_start..byte]),
    )
}

fn utf16_len(text: &str) -> u32 {
    text.encode_utf16().count().try_into().unwrap_or(u32::MAX)
}

fn root_for_path<'a>(path: &Path, roots: &'a [PathBuf]) -> Option<&'a Path> {
    roots
        .iter()
        .filter(|root| path.starts_with(root))
        .max_by_key(|root| root.components().count())
        .map(PathBuf::as_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_spans_convert_to_utf16_positions() {
        let content = "😀 use use\n";
        let range = range_from_byte_span(content, TextSpan { start: 9, end: 12 }).unwrap();

        assert_eq!(range, Range::new(Position::new(0, 7), Position::new(0, 10)));
    }

    #[test]
    fn finding_becomes_stable_standard_diagnostic() {
        let finding = Finding {
            severity: Severity::Warning,
            rule_id: "HL010".to_owned(),
            message: "Adjacent word repetition".to_owned(),
            path: Some(PathBuf::from("AGENTS.md")),
            line: Some(1),
            span: Some(TextSpan { start: 4, end: 7 }),
            evidence: None,
            source: "harness-lens.repetition".to_owned(),
        };

        let diagnostic = diagnostic_from_finding(&finding, "Use use tests").unwrap();

        assert_eq!(diagnostic.source.as_deref(), Some("harness-lens"));
        assert_eq!(
            diagnostic.code,
            Some(NumberOrString::String("HL010".to_owned()))
        );
        assert_eq!(diagnostic.range.start, Position::new(0, 4));
    }

    #[test]
    fn deepest_workspace_root_wins() {
        let roots = [PathBuf::from("/repo"), PathBuf::from("/repo/nested")];
        assert_eq!(
            root_for_path(Path::new("/repo/nested/AGENTS.md"), &roots),
            Some(Path::new("/repo/nested"))
        );
    }
}
