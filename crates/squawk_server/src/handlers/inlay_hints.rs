use anyhow::Result;
use gen_lsp_types::{
    InlayHint, InlayHintKind, InlayHintLabelPart, InlayHintParams, Label, Location,
};
use squawk_ide::db::line_index;
use squawk_ide::inlay_hints::inlay_hints;

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_inlay_hints(
    snapshot: &Snapshot,
    params: InlayHintParams,
) -> Result<Option<Vec<InlayHint>>> {
    let uri = params.text_document.uri;

    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();
    let current_line_index = line_index(db, file);

    let hints = inlay_hints(db, file);

    let lsp_hints: Vec<InlayHint> = hints
        .into_iter()
        .flat_map(|hint| {
            let line_col = current_line_index.line_col(hint.position);
            let position = gen_lsp_types::Position::new(line_col.line, line_col.col);

            let kind: InlayHintKind = match hint.kind {
                squawk_ide::inlay_hints::InlayHintKind::Parameter => InlayHintKind::Parameter,
                squawk_ide::inlay_hints::InlayHintKind::Type => InlayHintKind::Type,
            };

            let label = if let Some(target) = hint.target {
                let target_uri = snapshot.uri(target.file_id)?;
                let target_line_index = line_index(db, target.file_id);
                Label::InlayHintLabelPartList(vec![InlayHintLabelPart {
                    value: hint.label,
                    location: Some(Location {
                        uri: target_uri,
                        range: lsp_utils::range(&target_line_index, target.value),
                    }),
                    tooltip: None,
                    command: None,
                }])
            } else {
                Label::String(hint.label)
            };

            Some(InlayHint {
                position,
                label,
                kind: Some(kind),
                text_edits: None,
                tooltip: None,
                padding_left: None,
                padding_right: None,
                data: None,
            })
        })
        .collect();

    Ok(Some(lsp_hints))
}
