use anyhow::Result;
use gen_lsp_types::{LspRequestMethod, MessageDirection, Request};
use log::info;

use crate::global_state::Snapshot;

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct TokensParams {
    #[serde(rename = "textDocument")]
    text_document: gen_lsp_types::TextDocumentIdentifier,
}

pub(crate) enum TokensRequest {}

impl Request for TokensRequest {
    type Params = TokensParams;
    type Result = String;
    const METHOD: LspRequestMethod<'static> = LspRequestMethod::new("squawk/tokens");
    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
}

pub(crate) fn handle_tokens(snapshot: &Snapshot, params: TokensParams) -> Result<String> {
    let uri = params.text_document.uri;

    info!("Generating tokens for: {uri}");

    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();
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

    Ok(output.join("\n"))
}
