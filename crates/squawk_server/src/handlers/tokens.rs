use anyhow::Result;
use log::info;
use lsp_server::{Connection, Message, Response};

use crate::system::System;

#[derive(serde::Deserialize)]
pub(crate) struct TokensParams {
    #[serde(rename = "textDocument")]
    text_document: lsp_types::TextDocumentIdentifier,
}

pub(crate) fn handle_tokens(
    connection: &Connection,
    req: lsp_server::Request,
    system: &impl System,
) -> Result<()> {
    let params: TokensParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    info!("Generating tokens for: {uri}");

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let content = file.content(db);

    // TODO: move this to a tracked function
    let tokens = squawk_lexer::tokenize(content);

    let mut output = vec![];
    let mut char_pos = 0;
    for token in tokens {
        let token_start = char_pos;
        let token_end = token_start + token.len as usize;
        let token_text = &content[token_start..token_end];
        output.push(format!(
            "{:?}@{}..{} {:?}",
            token.kind, token_start, token_end, token_text
        ));
        char_pos = token_end;
    }

    let tokens_output = output.join("\n");

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&tokens_output).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}
