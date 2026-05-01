use anyhow::Result;
use lsp_types::SelectionRangeParams;
use rowan::TextRange;
use squawk_ide::db::{line_index, parse};

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_selection_range(
    snapshot: &Snapshot,
    params: SelectionRangeParams,
) -> Result<Option<Vec<lsp_types::SelectionRange>>> {
    let uri = params.text_document.uri;

    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();
    let parse = parse(db, file);
    let root = parse.syntax_node();
    let line_index = line_index(db, file);

    let mut selection_ranges = vec![];

    for position in params.positions {
        let Some(offset) = lsp_utils::offset(&line_index, position) else {
            continue;
        };

        let mut ranges = vec![];
        {
            let mut range = TextRange::new(offset, offset);
            loop {
                ranges.push(range);
                let next = squawk_ide::expand_selection::extend_selection(&root, range);
                if next == range {
                    break;
                } else {
                    range = next
                }
            }
        }

        let mut range = lsp_types::SelectionRange {
            range: lsp_utils::range(&line_index, *ranges.last().unwrap()),
            parent: None,
        };
        for &r in ranges.iter().rev().skip(1) {
            range = lsp_types::SelectionRange {
                range: lsp_utils::range(&line_index, r),
                parent: Some(Box::new(range)),
            }
        }
        selection_ranges.push(range);
    }

    Ok(Some(selection_ranges))
}
