use anyhow::Result;
use lsp_server::{Connection, Message, Response};
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse};
use squawk_ide::db::line_index;
use squawk_ide::goto_definition::goto_definition;

use crate::system::System;
use crate::lsp_utils::{self, to_location};

pub(crate) fn handle_goto_definition(
    connection: &Connection,
    req: lsp_server::Request,
    system: &impl System,
) -> Result<()> {
    let params: GotoDefinitionParams = serde_json::from_value(req.params)?;
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let line_index = line_index(db, file);
    let offset = lsp_utils::offset(&line_index, position).unwrap();

    let ranges = goto_definition(db, file, offset)
        .into_iter()
        .filter_map(|location| {
            debug_assert!(
                !location.range.contains(offset),
                "Our target destination range must not include the source range otherwise go to def won't work in vscode."
            );
            to_location(db, system, &uri, location)
        })
        .collect();

    let result = GotoDefinitionResponse::Array(ranges);
    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&result).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}
