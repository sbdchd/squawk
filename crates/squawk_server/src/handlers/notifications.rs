use anyhow::Result;
use gen_lsp_types::{
    CancelParams, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, Id, Notification as _, PublishDiagnosticsNotification,
    PublishDiagnosticsParams,
};
use lsp_server::{Message, Notification};

use crate::global_state::GlobalState;
use crate::lsp_utils;

pub(crate) fn handle_cancel(state: &mut GlobalState, params: CancelParams) -> Result<()> {
    let id: lsp_server::RequestId = match params.id {
        Id::Int(id) => id.into(),
        Id::String(id) => id.into(),
    };
    state.cancel(id);
    Ok(())
}

pub(crate) fn handle_did_open(
    state: &mut GlobalState,
    params: DidOpenTextDocumentParams,
) -> Result<()> {
    let uri = params.text_document.uri;
    let content = params.text_document.text;

    state.set(uri, content);

    Ok(())
}

pub(crate) fn handle_did_change(
    state: &mut GlobalState,
    params: DidChangeTextDocumentParams,
) -> Result<()> {
    let uri = params.text_document.text_document_identifier.uri;

    let db = state.db();
    let file = state.file(&uri).unwrap();
    let content = file.content(db);

    let updated_content = lsp_utils::apply_incremental_changes(content, params.content_changes);

    state.set(uri, updated_content);

    Ok(())
}

pub(crate) fn handle_did_close(
    state: &mut GlobalState,
    params: DidCloseTextDocumentParams,
) -> Result<()> {
    let uri = params.text_document.uri;

    state.remove(&uri);

    let publish_params = PublishDiagnosticsParams {
        uri,
        diagnostics: vec![],
        version: None,
    };

    let notification = Notification {
        method: PublishDiagnosticsNotification::METHOD.to_string(),
        params: serde_json::to_value(publish_params)?,
    };

    state.send(Message::Notification(notification));

    Ok(())
}
