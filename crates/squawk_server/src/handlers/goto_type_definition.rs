use gen_lsp_types::{Definition, TypeDefinitionParams, TypeDefinitionResponse};
use squawk_ide::goto_type_definition::goto_type_definition;

use crate::global_state::Snapshot;
use crate::lsp_utils::{self, to_location};

pub(crate) fn handle_goto_type_definition(
    snapshot: &Snapshot,
    params: TypeDefinitionParams,
) -> Option<TypeDefinitionResponse> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let db = snapshot.db();
    let file = snapshot.file(&uri)?;
    let position = lsp_utils::offset(db, file, position)?;

    let location = to_location(snapshot, goto_type_definition(db, position)?)?;

    Some(TypeDefinitionResponse::Definition(Definition::Location(
        location,
    )))
}
