use anyhow::Result;
use lsp_types::{CompletionParams, CompletionResponse};
use squawk_ide::completion::completion;

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_completion(
    snapshot: &Snapshot,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();

    let Some(position) = lsp_utils::offset(db, file, position) else {
        return Ok(Some(CompletionResponse::Array(vec![])));
    };

    let completion_items = completion(db, position)
        .into_iter()
        .map(lsp_utils::completion_item)
        .collect();

    Ok(Some(CompletionResponse::Array(completion_items)))
}
