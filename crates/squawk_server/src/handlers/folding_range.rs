use anyhow::Result;
use lsp_server::{Connection, Message, Response};
use lsp_types::FoldingRange;
use squawk_ide::db::line_index;
use squawk_ide::folding_ranges::folding_ranges;

use crate::system::System;
use crate::lsp_utils;

pub(crate) fn handle_folding_range(
    connection: &Connection,
    req: lsp_server::Request,
    system: &impl System,
) -> Result<()> {
    let params: lsp_types::FoldingRangeParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let line_idx = line_index(db, file);

    let lsp_folds: Vec<FoldingRange> = folding_ranges(db, file)
        .into_iter()
        .map(|fold| lsp_utils::folding_range(&line_idx, fold))
        .collect();

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&lsp_folds).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}
