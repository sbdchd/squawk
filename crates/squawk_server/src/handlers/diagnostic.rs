use anyhow::Result;
use gen_lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, FullDocumentDiagnosticReport,
    RelatedFullDocumentDiagnosticReport,
};

use crate::global_state::Snapshot;

pub(crate) fn handle_document_diagnostic(
    snapshot: &Snapshot,
    params: DocumentDiagnosticParams,
) -> Result<DocumentDiagnosticReport> {
    let uri = params.text_document.uri;

    let diagnostics = snapshot
        .file(&uri)
        .map(|file| crate::lint::lint(snapshot.db(), file))
        .unwrap_or_default();

    Ok(
        DocumentDiagnosticReport::RelatedFullDocumentDiagnosticReport(
            RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: diagnostics,
                },
            },
        ),
    )
}
