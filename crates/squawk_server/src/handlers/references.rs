use anyhow::Result;
use lsp_types::{Location, ReferenceParams};
use squawk_ide::find_references::find_references;

use crate::global_state::Snapshot;
use crate::lsp_utils::{self, to_location};

pub(crate) fn handle_references(
    snapshot: &Snapshot,
    params: ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let db = snapshot.db();
    let file = snapshot.file(&uri).unwrap();
    let position = lsp_utils::offset(db, file, position).unwrap();

    let refs = find_references(db, position);
    let include_declaration = params.context.include_declaration;

    let locations: Vec<Location> = refs
        .into_iter()
        .filter(|loc| {
            if include_declaration {
                return true;
            }
            if loc.file == file && !loc.range.contains(position.value) {
                return true;
            }
            false
        })
        .filter_map(|loc| to_location(snapshot, loc))
        .collect();

    Ok(Some(locations))
}
