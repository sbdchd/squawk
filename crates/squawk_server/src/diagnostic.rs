use serde::{Deserialize, Serialize};

pub(crate) const DIAGNOSTIC_NAME: &str = "squawk";

// Based on Ruff's setup for LSP diagnostic edits
// see: https://github.com/astral-sh/ruff/blob/1a368b0bf97c3d0246390679166bbd2d589acf39/crates/ruff_server/src/lint.rs#L31
/// This is serialized on the diagnostic `data` field.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AssociatedDiagnosticData {
    /// The message describing what the fix does, if it exists, or the diagnostic name otherwise.
    pub(crate) title: String,
    /// Edits to fix the diagnostic. If this is empty, a fix
    /// does not exist.
    pub(crate) edits: Vec<lsp_types::TextEdit>,
    /// Edit to ignore the rule the line
    pub(crate) ignore_line_edit: Option<lsp_types::TextEdit>,
    /// Edit to ignore the rule for the file
    pub(crate) ignore_file_edit: Option<lsp_types::TextEdit>,
}
