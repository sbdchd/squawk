use anyhow::Result;
use lsp_server::{Connection, Message, Notification};
use lsp_types::{
    Diagnostic, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    PublishDiagnosticsParams, Url,
    notification::{Notification as _, PublishDiagnostics},
};

use crate::lsp_utils;
use crate::system::{Document, System};

fn publish_diagnostics(
    connection: &Connection,
    uri: Url,
    version: i32,
    diagnostics: Vec<Diagnostic>,
) -> Result<()> {
    let publish_params = PublishDiagnosticsParams {
        uri,
        diagnostics,
        version: Some(version),
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

pub(crate) fn handle_did_open(
    connection: &Connection,
    params: DidOpenTextDocumentParams,
    system: &mut dyn System,
) -> Result<()> {
    let uri = params.text_document.uri;
    let content = params.text_document.text;
    let version = params.text_document.version;

    system.set(uri.clone(), Document { content, version });
    let db = system.db();
    let file = system.file(&uri).unwrap();
    let diagnostics = crate::lint::lint(db, file);

    // TODO: we need a better setup for "run func when input changed"
    publish_diagnostics(connection, uri, version, diagnostics)?;

    Ok(())
}

pub(crate) fn handle_did_change(
    connection: &Connection,
    params: DidChangeTextDocumentParams,
    system: &mut dyn System,
) -> Result<()> {
    let uri = params.text_document.uri;
    let version = params.text_document.version;

    let db = system.db();
    let file = system.file(&uri).unwrap();
    let content = file.content(db);

    let updated_content = lsp_utils::apply_incremental_changes(content, params.content_changes);

    system.set(
        uri.clone(),
        Document {
            content: updated_content,
            version,
        },
    );
    let db = system.db();
    let file = system.file(&uri).unwrap();
    let diagnostics = crate::lint::lint(db, file);
    publish_diagnostics(connection, uri, version, diagnostics)?;

    Ok(())
}

pub(crate) fn handle_did_close(
    connection: &Connection,
    params: DidCloseTextDocumentParams,
    system: &mut dyn System,
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
