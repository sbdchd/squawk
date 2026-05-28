use anyhow::Result;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse};
use squawk_ide::goto_definition::goto_definition;

use crate::global_state::Snapshot;
use crate::lsp_utils::{self, to_location};

pub(crate) fn handle_goto_definition(
    snapshot: &Snapshot,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();
    let position = lsp_utils::offset(db, file, position).unwrap();

    let ranges = goto_definition(db, position)
        .into_iter()
        .filter_map(|location| {
            debug_assert!(
                !location.range.contains(position.value),
                "Our target destination range must not include the source range otherwise go to def won't work in vscode."
            );
            to_location(snapshot, location)
        })
        .collect();

    Ok(Some(GotoDefinitionResponse::Array(ranges)))
}
