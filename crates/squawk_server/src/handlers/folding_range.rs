use anyhow::Result;
use lsp_types::{FoldingRange, FoldingRangeParams};
use squawk_ide::db::line_index;
use squawk_ide::folding_ranges::folding_ranges;

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_folding_range(
    system: &Snapshot,
    params: FoldingRangeParams,
) -> Result<Option<Vec<FoldingRange>>> {
    let uri = params.text_document.uri;

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let line_idx = line_index(db, file);

    let lsp_folds: Vec<FoldingRange> = folding_ranges(db, file)
        .into_iter()
        .map(|fold| lsp_utils::folding_range(&line_idx, fold))
        .collect();

    Ok(Some(lsp_folds))
}
