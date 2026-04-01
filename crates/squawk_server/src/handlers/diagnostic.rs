use anyhow::Result;
use lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};

use crate::global_state::Snapshot;

fn is_path_excluded(
    uri: &lsp_types::Url,
    excluded_paths: &[glob::Pattern],
    workspace_root: Option<&std::path::Path>,
) -> bool {
    let Ok(file_path) = uri.to_file_path() else {
        return false;
    };
    let file_path_str = file_path.to_string_lossy();
    let relative_path = workspace_root.and_then(|root| file_path.strip_prefix(root).ok());
    excluded_paths.iter().any(|pattern| {
        pattern.matches(&file_path_str)
            || relative_path.is_some_and(|rel| pattern.matches(&rel.to_string_lossy()))
    })
}

pub(crate) fn handle_document_diagnostic(
    snapshot: &Snapshot,
    params: DocumentDiagnosticParams,
) -> Result<DocumentDiagnosticReportResult> {
    let uri = params.text_document.uri;

    let diagnostics = if is_path_excluded(
        &uri,
        &snapshot.config.excluded_paths,
        snapshot.config.workspace_root.as_deref(),
    ) {
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
