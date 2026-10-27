use salsa::Database as Db;

use crate::db::{File, semantic_index};
use crate::file::FileId;
pub use crate::semantic_index::{Diagnostic, DiagnosticKind, Suggestion};

pub fn diagnostics(db: &dyn Db, file: File) -> Vec<Diagnostic> {
    let file_id = FileId::from(file);
    let mut diagnostics = semantic_index(db, file_id).diagnostics();
    diagnostics.retain(|diagnostic| diagnostic.range.file_id == file_id);
    diagnostics
}
