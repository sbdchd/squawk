use anyhow::Result;
use log::info;
use lsp_types::request::Request;
use squawk_ide::db::parse;

use crate::global_state::Snapshot;

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct SyntaxTreeParams {
    #[serde(rename = "textDocument")]
    text_document: lsp_types::TextDocumentIdentifier,
}

pub(crate) enum SyntaxTreeRequest {}

impl Request for SyntaxTreeRequest {
    type Params = SyntaxTreeParams;
    type Result = String;
    const METHOD: &'static str = "squawk/syntaxTree";
}

pub(crate) fn handle_syntax_tree(system: &Snapshot, params: SyntaxTreeParams) -> Result<String> {
    let uri = params.text_document.uri;

    info!("Generating syntax tree for: {uri}");

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let parse = parse(db, file);
    let syntax_tree = format!("{:#?}", parse.syntax_node());

    Ok(syntax_tree)
}
