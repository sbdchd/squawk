use anyhow::Result;
use gen_lsp_types::{Contents, Hover, HoverParams, MarkupContent, MarkupKind};
use squawk_ide::hover::hover;

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_hover(snapshot: &Snapshot, params: HoverParams) -> Result<Option<Hover>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();
    let position = lsp_utils::offset(db, file, position).unwrap();

    let hover_info = hover(db, position);

    Ok(hover_info.map(|hover_info| Hover {
        contents: Contents::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: hover_info.markdown(),
        }),
        range: None,
    }))
}
