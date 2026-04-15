use anyhow::Result;
use lsp_types::{
    SemanticTokens, SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensRangeResult,
    SemanticTokensResult,
};

use squawk_ide::db::line_index;
use squawk_ide::semantic_tokens::semantic_tokens;

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_semantic_tokens_full(
    snapshot: &Snapshot,
    params: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>> {
    let uri = params.text_document.uri;
    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();
    let line_index = line_index(db, file);
    let text = file.content(db);
    let tokens = semantic_tokens(db, file, None);
    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: lsp_utils::to_semantic_tokens(text, line_index, tokens),
    })))
}

pub(crate) fn handle_semantic_tokens_range(
    snapshot: &Snapshot,
    params: SemanticTokensRangeParams,
) -> Result<Option<SemanticTokensRangeResult>> {
    let uri = params.text_document.uri;
    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();
    let line_index = line_index(db, file);
    let range_to_highlight = lsp_utils::text_range(&line_index, params.range);
    let tokens = semantic_tokens(db, file, range_to_highlight);
    let text = file.content(db);
    Ok(Some(SemanticTokensRangeResult::Tokens(SemanticTokens {
        result_id: None,
        data: lsp_utils::to_semantic_tokens(text, line_index, tokens),
    })))
}
