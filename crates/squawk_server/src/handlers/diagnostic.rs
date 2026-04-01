use anyhow::Result;
use lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};

use crate::global_state::Snapshot;

fn is_path_excluded(uri: &lsp_types::Url, excluded_paths: &[glob::Pattern]) -> bool {
    let Ok(file_path) = uri.to_file_path() else {
        return false;
    };
    let file_path_str = file_path.to_string_lossy();
    excluded_paths
        .iter()
        .any(|pattern| pattern.matches(&file_path_str))
}

pub(crate) fn handle_document_diagnostic(
    snapshot: &Snapshot,
    params: DocumentDiagnosticParams,
) -> Result<DocumentDiagnosticReportResult> {
    let uri = params.text_document.uri;

    let diagnostics = if is_path_excluded(&uri, &snapshot.config.excluded_paths) {
        vec![]
    } else {
        snapshot
            .file(&uri)
            .map(|file| crate::lint::lint(snapshot.db(), file, &snapshot.config))
            .unwrap_or_default()
    };

    Ok(DocumentDiagnosticReportResult::Report(
        DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
            related_documents: None,
            full_document_diagnostic_report: FullDocumentDiagnosticReport {
                result_id: None,
                items: diagnostics,
            },
        }),
    ))
}
