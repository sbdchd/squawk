use anyhow::Result;
use lsp_types::{Hover, HoverContents, HoverParams, LanguageString, MarkedString};
use squawk_ide::db::line_index;
use squawk_ide::hover::hover;

use crate::lsp_utils;
use crate::system::System;

pub(crate) fn handle_hover(system: &dyn System, params: HoverParams) -> Result<Option<Hover>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let db = system.db();
    let file = system.file(&uri).unwrap();
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
