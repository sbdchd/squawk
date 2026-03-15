use anyhow::Result;
use lsp_server::{Connection, Message, Response};
use lsp_types::{
    InlayHint, InlayHintKind, InlayHintLabel, InlayHintLabelPart, InlayHintParams, Location,
};
use squawk_ide::builtins::{builtins_line_index, builtins_url};
use squawk_ide::db::line_index;
use squawk_ide::inlay_hints::inlay_hints;

use crate::lsp_utils;
use crate::system::System;

pub(crate) fn handle_inlay_hints(
    connection: &Connection,
    req: lsp_server::Request,
    system: &impl System,
) -> Result<()> {
    let params: InlayHintParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let line_index = line_index(db, file);

    let hints = inlay_hints(db, file);

    let lsp_hints: Vec<InlayHint> = hints
        .into_iter()
        .flat_map(|hint| {
            let line_col = line_index.line_col(hint.position);
            let position = lsp_types::Position::new(line_col.line, line_col.col);

            let uri = match hint.file {
                Some(squawk_ide::goto_definition::FileId::Current) | None => uri.clone(),
                Some(squawk_ide::goto_definition::FileId::Builtins) => builtins_url(db)?,
            };

            let line_index = match hint.file {
                Some(squawk_ide::goto_definition::FileId::Current) | None => &line_index,
                Some(squawk_ide::goto_definition::FileId::Builtins) => &builtins_line_index(db),
            };

            let kind: InlayHintKind = match hint.kind {
                squawk_ide::inlay_hints::InlayHintKind::Type => InlayHintKind::TYPE,
                squawk_ide::inlay_hints::InlayHintKind::Parameter => InlayHintKind::PARAMETER,
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

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&lsp_hints).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}
