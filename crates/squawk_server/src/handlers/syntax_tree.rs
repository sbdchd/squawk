use anyhow::Result;
use log::info;
use lsp_server::{Connection, Message, Response};
use squawk_ide::db::parse;

use crate::system::System;

#[derive(serde::Deserialize)]
pub(crate) struct SyntaxTreeParams {
    #[serde(rename = "textDocument")]
    text_document: lsp_types::TextDocumentIdentifier,
}

pub(crate) fn handle_syntax_tree(
    connection: &Connection,
    req: lsp_server::Request,
    system: &impl System,
) -> Result<()> {
    let params: SyntaxTreeParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    info!("Generating syntax tree for: {uri}");

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let parse = parse(db, file);
    let syntax_tree = format!("{:#?}", parse.syntax_node());

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&syntax_tree).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}
