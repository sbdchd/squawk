use anyhow::Result;
use line_index::LineIndex;
use log::info;
use lsp_server::{Connection, Message, Notification, Response};
use lsp_types::{
    CodeDescription, Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, GotoDefinitionParams,
    GotoDefinitionResponse, InitializeParams, Location, Position, PublishDiagnosticsParams, Range,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Url,
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification as _,
        PublishDiagnostics,
    },
    request::{GotoDefinition, Request},
};
use squawk_linter::Linter;
use squawk_syntax::{Parse, SourceFile};
use std::collections::HashMap;
mod lsp_utils;

struct DocumentState {
    content: String,
    version: i32,
}

pub fn run() -> Result<()> {
    info!("Starting Squawk LSP server");

    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        // definition_provider: Some(OneOf::Left(true)),
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

    let mut documents: HashMap<Url, DocumentState> = HashMap::new();

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                info!("Received request: method={}, id={:?}", req.method, req.id);

                if connection.handle_shutdown(&req)? {
                    info!("Received shutdown request, exiting");
                    return Ok(());
                }

                match req.method.as_ref() {
                    GotoDefinition::METHOD => {
                        handle_goto_definition(&connection, req)?;
                    }
                    "squawk/syntaxTree" => {
                        handle_syntax_tree(&connection, req, &documents)?;
                    }
                    "squawk/tokens" => {
                        handle_tokens(&connection, req, &documents)?;
                    }
                    _ => {
                        info!("Ignoring unhandled request: {}", req.method);
                    }
                }
            }
            Message::Response(resp) => {
                info!("Received response: id={:?}", resp.id);
            }
            Message::Notification(notif) => {
                info!("Received notification: method={}", notif.method);
                match notif.method.as_ref() {
                    DidOpenTextDocument::METHOD => {
                        handle_did_open(&connection, notif, &mut documents)?;
                    }
                    DidChangeTextDocument::METHOD => {
                        handle_did_change(&connection, notif, &mut documents)?;
                    }
                    DidCloseTextDocument::METHOD => {
                        handle_did_close(&connection, notif, &mut documents)?;
                    }
                    _ => {
                        info!("Ignoring unhandled notification: {}", notif.method);
                    }
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
        range: Range::new(Position::new(1, 2), Position::new(1, 3)),
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

fn handle_did_open(
    connection: &Connection,
    notif: lsp_server::Notification,
    documents: &mut HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: DidOpenTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;
    let content = params.text_document.text;
    let version = params.text_document.version;

    documents.insert(uri.clone(), DocumentState { content, version });

    let content = documents.get(&uri).map_or("", |doc| &doc.content);

    // TODO: we need a better setup for "run func when input changed"
    let diagnostics = lint(content);
    publish_diagnostics(connection, uri, version, diagnostics)?;

    Ok(())
}

fn handle_did_change(
    connection: &Connection,
    notif: lsp_server::Notification,
    documents: &mut HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: DidChangeTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;
    let version = params.text_document.version;

    let Some(doc_state) = documents.get_mut(&uri) else {
        return Ok(());
    };

    doc_state.content =
        lsp_utils::apply_incremental_changes(&doc_state.content, params.content_changes);
    doc_state.version = version;

    let diagnostics = lint(&doc_state.content);
    publish_diagnostics(connection, uri, version, diagnostics)?;

    Ok(())
}

fn handle_did_close(
    connection: &Connection,
    notif: lsp_server::Notification,
    documents: &mut HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: DidCloseTextDocumentParams = serde_json::from_value(notif.params)?;
    let uri = params.text_document.uri;

    documents.remove(&uri);

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

fn lint(content: &str) -> Vec<Diagnostic> {
    let parse: Parse<SourceFile> = SourceFile::parse(content);
    let parse_errors = parse.errors();
    let mut linter = Linter::with_all_rules();
    let violations = linter.lint(parse, content);
    let line_index = LineIndex::new(content);

    let mut diagnostics = Vec::with_capacity(violations.len() + parse_errors.len());

    for error in parse_errors {
        let range_start = error.range().start();
        let range_end = error.range().end();
        let start_line_col = line_index.line_col(range_start);
        let mut end_line_col = line_index.line_col(range_end);

        if start_line_col.line == end_line_col.line && start_line_col.col == end_line_col.col {
            end_line_col.col += 1;
        }

        let diagnostic = Diagnostic {
            range: Range::new(
                Position::new(start_line_col.line, start_line_col.col),
                Position::new(end_line_col.line, end_line_col.col),
            ),
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
            range: Range::new(
                Position::new(start_line_col.line, start_line_col.col),
                Position::new(end_line_col.line, end_line_col.col),
            ),
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
    diagnostics
}

#[derive(serde::Deserialize)]
struct SyntaxTreeParams {
    #[serde(rename = "textDocument")]
    text_document: lsp_types::TextDocumentIdentifier,
}

fn handle_syntax_tree(
    connection: &Connection,
    req: lsp_server::Request,
    documents: &HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: SyntaxTreeParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    info!("Generating syntax tree for: {}", uri);

    let content = documents.get(&uri).map_or("", |doc| &doc.content);

    let parse: Parse<SourceFile> = SourceFile::parse(content);
    let syntax_tree = format!("{:#?}", parse.syntax_node());

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&syntax_tree).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}

#[derive(serde::Deserialize)]
struct TokensParams {
    #[serde(rename = "textDocument")]
    text_document: lsp_types::TextDocumentIdentifier,
}

fn handle_tokens(
    connection: &Connection,
    req: lsp_server::Request,
    documents: &HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: TokensParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    info!("Generating tokens for: {}", uri);

    let content = documents.get(&uri).map_or("", |doc| &doc.content);

    let tokens = squawk_lexer::tokenize(content);

    let mut output = Vec::new();
    let mut char_pos = 0;
    for token in tokens {
        let token_start = char_pos;
        let token_end = token_start + token.len as usize;
        let token_text = &content[token_start..token_end];
        output.push(format!(
            "{:?}@{}..{} {:?}",
            token.kind, token_start, token_end, token_text
        ));
        char_pos = token_end;
    }

    let tokens_output = output.join("\n");

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&tokens_output).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}
