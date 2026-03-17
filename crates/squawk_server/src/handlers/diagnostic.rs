use anyhow::Result;
use lsp_types::{
    DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportResult,
    FullDocumentDiagnosticReport, RelatedFullDocumentDiagnosticReport,
};

use crate::system::System;

pub(crate) fn handle_document_diagnostic(
    system: &dyn System,
    params: DocumentDiagnosticParams,
) -> Result<DocumentDiagnosticReportResult> {
    let uri = params.text_document.uri;

    let diagnostics = system
        .file(&uri)
        .map(|file| crate::lint::lint(system.db(), file))
        .unwrap_or_default();

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
