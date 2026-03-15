use anyhow::Result;
use lsp_server::{Connection, Message, Response};
use lsp_types::{CompletionParams, CompletionResponse};
use squawk_ide::completion::completion;
use squawk_ide::db::line_index;

use crate::lsp_utils;
use crate::system::System;

pub(crate) fn handle_completion(
    connection: &Connection,
    req: lsp_server::Request,
    system: &impl System,
) -> Result<()> {
    let params: CompletionParams = serde_json::from_value(req.params)?;
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let line_index = line_index(db, file);

    let Some(offset) = lsp_utils::offset(&line_index, position) else {
        let resp = Response {
            id: req.id,
            result: Some(serde_json::to_value(CompletionResponse::Array(vec![])).unwrap()),
            error: None,
        };
        connection.sender.send(Message::Response(resp))?;
        return Ok(());
    };

    let completion_items = completion(db, file, offset)
        .into_iter()
        .map(lsp_utils::completion_item)
        .collect();

    let result = CompletionResponse::Array(completion_items);

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&result).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}
