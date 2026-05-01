use anyhow::Result;
use lsp_types::{
    InlayHint, InlayHintKind, InlayHintLabel, InlayHintLabelPart, InlayHintParams, Location,
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
            let position = lsp_types::Position::new(line_col.line, line_col.col);

            let target_file = hint.file;
            let uri = match target_file {
                Some(target_file) => snapshot.uri(target_file)?,
                None => uri.clone(),
            };

            let target_line_index = target_file.map(|target_file| line_index(db, target_file));
            let line_index = target_line_index.as_ref().unwrap_or(&current_line_index);

            let kind: InlayHintKind = match hint.kind {
                squawk_ide::inlay_hints::InlayHintKind::Parameter => InlayHintKind::PARAMETER,
                squawk_ide::inlay_hints::InlayHintKind::Type => InlayHintKind::TYPE,
            };

            let label = if let Some(target_range) = hint.target {
                InlayHintLabel::LabelParts(vec![InlayHintLabelPart {
                    value: hint.label,
                    location: Some(Location {
                        uri: uri.clone(),
                        range: lsp_utils::range(line_index, target_range),
                    }),
                    tooltip: None,
                    command: None,
                }])
            } else {
                InlayHintLabel::String(hint.label)
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
