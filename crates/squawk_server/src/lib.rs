use anyhow::Result;
use line_index::LineIndex;
use log::info;
use lsp_server::{Connection, Message, Notification, Response};
use lsp_types::{
    CodeDescription, Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, GotoDefinitionParams,
    GotoDefinitionResponse, InitializeParams, Location, OneOf, Position, PublishDiagnosticsParams,
    Range, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Url,
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification as _,
        PublishDiagnostics,
    },
    request::{GotoDefinition, Request},
};
use squawk_linter::Linter;
use squawk_syntax::{Parse, SourceFile};

pub fn run() -> Result<()> {
    info!("Starting Squawk LSP server");

    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    })
    .unwrap();

    info!("LSP server initializing connection...");
    let initialization_params = connection.initialize(server_capabilities)?;
    info!("LSP server initialized, entering main loop");

    main_loop(connection, initialization_params)?;

    info!("LSP server shutting down");

    io_threads.join()?;
    Ok(())
}

fn main_loop(connection: Connection, params: serde_json::Value) -> Result<()> {
    info!("Server main loop");

    let init_params: InitializeParams = serde_json::from_value(params).unwrap_or_default();
    info!("Client process ID: {:?}", init_params.process_id);
    let client_name = init_params.client_info.map(|x| x.name);
    info!("Client name: {client_name:?}");

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                info!("Received request: method={}, id={:?}", req.method, req.id);

                if connection.handle_shutdown(&req)? {
                    info!("Received shutdown request, exiting");
                    return Ok(());
                }

                if req.method == GotoDefinition::METHOD {
                    handle_goto_definition(&connection, req)?;
                    continue;
                }

                info!("Ignoring unhandled request: {}", req.method);
            }
            Message::Response(resp) => {
                info!("Received response: id={:?}", resp.id);
            }
            Message::Notification(notif) => {
                info!("Received notification: method={}", notif.method);

                if notif.method == DidOpenTextDocument::METHOD {
                    handle_did_open(&connection, notif)?;
                } else if notif.method == DidChangeTextDocument::METHOD {
                    handle_did_change(&connection, notif)?;
                } else if notif.method == DidCloseTextDocument::METHOD {
                    handle_did_close(&connection, notif)?;
                }
            }
        }
    }
    Ok(())
}

fn handle_goto_definition(connection: &Connection, req: lsp_server::Request) -> Result<()> {
    let params: GotoDefinitionParams = serde_json::from_value(req.params)?;

    let location = Location {
        uri: params.text_document_position_params.text_document.uri,
        range: Range {
            start: Position::new(1, 2),
            end: Position::new(1, 3),
        },
    };

    let result = GotoDefinitionResponse::Scalar(location);
    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&result).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}

fn handle_did_open(connection: &Connection, notif: lsp_server::Notification) -> Result<()> {
    let params: DidOpenTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;
    let content = params.text_document.text;
    let version = params.text_document.version;

    lint(connection, uri, &content, version)?;

    Ok(())
}

fn handle_did_change(connection: &Connection, notif: lsp_server::Notification) -> Result<()> {
    let params: DidChangeTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;
    let version = params.text_document.version;

    if let Some(change) = params.content_changes.last() {
        lint(connection, uri, &change.text, version)?;
    }

    Ok(())
}

fn handle_did_close(connection: &Connection, notif: lsp_server::Notification) -> Result<()> {
    let params: DidCloseTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;

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

fn lint(connection: &Connection, uri: lsp_types::Url, content: &str, version: i32) -> Result<()> {
    let parse: Parse<SourceFile> = SourceFile::parse(content);
    let parse_errors = parse.errors();
    let mut linter = Linter::with_all_rules();
    let violations = linter.lint(parse, content);
    let line_index = LineIndex::new(content);

    let mut diagnostics = vec![];

    for error in parse_errors {
        let range_start = error.range().start();
        let range_end = error.range().end();
        let start_line_col = line_index.line_col(range_start);
        let mut end_line_col = line_index.line_col(range_end);

        if start_line_col.line == end_line_col.line && start_line_col.col == end_line_col.col {
            end_line_col.col += 1;
        }

        let diagnostic = Diagnostic {
            range: Range {
                start: Position::new(start_line_col.line, start_line_col.col),
                end: Position::new(end_line_col.line, end_line_col.col),
            },
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(lsp_types::NumberOrString::String(
                "syntax-error".to_string(),
            )),
            code_description: Some(CodeDescription {
                href: Url::parse("https://squawkhq.com/docs/syntax-error").unwrap(),
            }),
            source: Some("squawk".to_string()),
            message: error.message().to_string(),
            ..Default::default()
        };
        diagnostics.push(diagnostic);
    }

    for violation in violations {
        let range_start = violation.text_range.start();
        let range_end = violation.text_range.end();
        let start_line_col = line_index.line_col(range_start);
        let mut end_line_col = line_index.line_col(range_end);

        if start_line_col.line == end_line_col.line && start_line_col.col == end_line_col.col {
            end_line_col.col += 1;
        }

        let diagnostic = Diagnostic {
            range: Range {
                start: Position::new(start_line_col.line, start_line_col.col),
                end: Position::new(end_line_col.line, end_line_col.col),
            },
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(lsp_types::NumberOrString::String(
                violation.code.to_string(),
            )),
            code_description: Some(CodeDescription {
                href: Url::parse(&format!("https://squawkhq.com/docs/{}", violation.code)).unwrap(),
            }),
            source: Some("squawk".to_string()),
            message: violation.message,
            ..Default::default()
        };
        diagnostics.push(diagnostic);
    }

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
