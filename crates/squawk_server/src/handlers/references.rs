use anyhow::Result;
use lsp_types::{Location, ReferenceParams};
use squawk_ide::db::line_index;
use squawk_ide::find_references::find_references;

use crate::lsp_utils::{self, to_location};
use crate::system::System;

pub(crate) fn handle_references(
    system: &dyn System,
    params: ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let line_index = line_index(db, file);
    let offset = lsp_utils::offset(&line_index, position).unwrap();

    let refs = find_references(db, file, offset);
    let include_declaration = params.context.include_declaration;

    let locations: Vec<Location> = refs
        .into_iter()
        .filter(|loc| include_declaration || !loc.range.contains(offset))
        .filter_map(|loc| to_location(db, system, &uri, loc))
        .collect();

    Ok(Some(locations))
}
