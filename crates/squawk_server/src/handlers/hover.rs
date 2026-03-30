use anyhow::Result;
use lsp_types::{Hover, HoverContents, HoverParams, LanguageString, MarkedString};
use squawk_ide::db::line_index;
use squawk_ide::hover::hover;

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_hover(snapshot: &Snapshot, params: HoverParams) -> Result<Option<Hover>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();
    let line_index = line_index(db, file);
    let offset = lsp_utils::offset(&line_index, position).unwrap();

    let type_info = hover(db, file, offset);

    Ok(type_info.map(|type_str| Hover {
        contents: HoverContents::Scalar(MarkedString::LanguageString(LanguageString {
            language: "sql".to_string(),
            value: type_str,
        })),
        range: None,
    }))
}
