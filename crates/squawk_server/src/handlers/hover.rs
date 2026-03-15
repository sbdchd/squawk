use anyhow::Result;
use lsp_server::{Connection, Message, Response};
use lsp_types::{Hover, HoverContents, HoverParams, LanguageString, MarkedString};
use squawk_ide::db::line_index;
use squawk_ide::hover::hover;

use crate::system::System;
use crate::lsp_utils;

pub(crate) fn handle_hover(
    connection: &Connection,
    req: lsp_server::Request,
    system: &impl System,
) -> Result<()> {
    let params: HoverParams = serde_json::from_value(req.params)?;
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let line_index = line_index(db, file);
    let offset = lsp_utils::offset(&line_index, position).unwrap();

    let type_info = hover(db, file, offset);

    let result = type_info.map(|type_str| Hover {
        contents: HoverContents::Scalar(MarkedString::LanguageString(LanguageString {
            language: "sql".to_string(),
            value: type_str,
        })),
        range: None,
    });

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&result).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}
