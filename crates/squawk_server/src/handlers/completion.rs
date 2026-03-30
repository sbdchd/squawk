use anyhow::Result;
use lsp_types::{CompletionParams, CompletionResponse};
use squawk_ide::completion::completion;
use squawk_ide::db::line_index;

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_completion(
    system: &Snapshot,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let line_index = line_index(db, file);

    let Some(offset) = lsp_utils::offset(&line_index, position) else {
        return Ok(Some(CompletionResponse::Array(vec![])));
    };

    let completion_items = completion(db, file, offset)
        .into_iter()
        .map(lsp_utils::completion_item)
        .collect();

    Ok(Some(CompletionResponse::Array(completion_items)))
}
