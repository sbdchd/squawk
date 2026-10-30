use squawk_sema::file::FileId;
pub use squawk_sema::semantic_index::{Diagnostic, DiagnosticKind, Suggestion};

use squawk_ide_db::{Db, semantic_index};

pub fn diagnostics(db: &dyn Db, file_id: FileId) -> Vec<Diagnostic> {
    let mut diagnostics = semantic_index(db, file_id).diagnostics();
    diagnostics.retain(|diagnostic| diagnostic.range.file_id == file_id);
    diagnostics
}
