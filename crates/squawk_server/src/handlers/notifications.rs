use anyhow::Result;
use lsp_server::{Connection, Message, Notification};
use lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    PublishDiagnosticsParams,
    notification::{Notification as _, PublishDiagnostics},
};

use crate::lsp_utils;
use crate::system::MutableSystem;

pub(crate) fn handle_did_open(
    _connection: &Connection,
    params: DidOpenTextDocumentParams,
    system: &mut dyn MutableSystem,
) -> Result<()> {
    let uri = params.text_document.uri;
    let content = params.text_document.text;

    system.set(uri, content);

    Ok(())
}

pub(crate) fn handle_did_change(
    _connection: &Connection,
    params: DidChangeTextDocumentParams,
    system: &mut dyn MutableSystem,
) -> Result<()> {
    let uri = params.text_document.uri;

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let content = file.content(db);

    let updated_content = lsp_utils::apply_incremental_changes(content, params.content_changes);

    system.set(uri, updated_content);

    Ok(())
}

pub(crate) fn handle_did_close(
    connection: &Connection,
    params: DidCloseTextDocumentParams,
    system: &mut dyn MutableSystem,
) -> Result<()> {
    let uri = params.text_document.uri;

    system.remove(&uri);

    let publish_params = PublishDiagnosticsParams {
        uri,
        diagnostics: vec![],
        version: None,
    };

    let notification = Notification {
        method: PublishDiagnostics::METHOD.to_owned(),
        params: serde_json::to_value(publish_params)?,
    };

    connection
        .sender
        .send(Message::Notification(notification))?;

    Ok(())
}
