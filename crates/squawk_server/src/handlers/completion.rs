use gen_lsp_types::{CompletionParams, CompletionResponse};
use squawk_ide::completion::completion;

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_completion(
    snapshot: &Snapshot,
    params: CompletionParams,
) -> CompletionResponse {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();

    let Some(position) = lsp_utils::offset(db, file, position) else {
        return CompletionResponse::CompletionItemList(vec![]);
    };

    let completion_items = completion(db, position)
        .into_iter()
        .map(lsp_utils::completion_item)
        .collect();

    CompletionResponse::CompletionItemList(completion_items)
}
