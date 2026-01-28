use anyhow::{Context, Result};
use line_index::LineIndex;
use log::info;
use lsp_server::{Connection, Message, Notification, Response};
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOptions, CodeActionOrCommand, CodeActionParams,
    CodeActionProviderCapability, CodeActionResponse, Command, CompletionOptions, CompletionParams,
    CompletionResponse, Diagnostic, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DocumentSymbol, DocumentSymbolParams, GotoDefinitionParams,
    GotoDefinitionResponse, Hover, HoverContents, HoverParams, HoverProviderCapability,
    InitializeParams, InlayHint, InlayHintKind, InlayHintLabel, InlayHintLabelPart,
    InlayHintParams, LanguageString, Location, MarkedString, OneOf, PublishDiagnosticsParams,
    ReferenceParams, SelectionRangeParams, SelectionRangeProviderCapability, ServerCapabilities,
    SymbolKind, TextDocumentSyncCapability, TextDocumentSyncKind, Url, WorkDoneProgressOptions,
    WorkspaceEdit,
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification as _,
        PublishDiagnostics,
    },
    request::{
        CodeActionRequest, Completion, DocumentSymbolRequest, GotoDefinition, HoverRequest,
        InlayHintRequest, References, Request, SelectionRangeRequest,
    },
};
use rowan::TextRange;
use squawk_ide::code_actions::code_actions;
use squawk_ide::completion::completion;
use squawk_ide::document_symbols::{DocumentSymbolKind, document_symbols};
use squawk_ide::find_references::find_references;
use squawk_ide::goto_definition::goto_definition;
use squawk_ide::hover::hover;
use squawk_ide::inlay_hints::inlay_hints;
use squawk_syntax::SourceFile;
use std::collections::HashMap;

use diagnostic::DIAGNOSTIC_NAME;

use crate::diagnostic::AssociatedDiagnosticData;
mod diagnostic;
mod ignore;
mod lint;
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
        code_action_provider: Some(CodeActionProviderCapability::Options(CodeActionOptions {
            code_action_kinds: Some(vec![
                CodeActionKind::QUICKFIX,
                CodeActionKind::REFACTOR_REWRITE,
            ]),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
            resolve_provider: None,
        })),
        selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
        references_provider: Some(OneOf::Left(true)),
        definition_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        inlay_hint_provider: Some(OneOf::Left(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![".".to_owned()]),
            all_commit_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
            completion_item: None,
        }),
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
                        handle_goto_definition(&connection, req, &documents)?;
                    }
                    HoverRequest::METHOD => {
                        handle_hover(&connection, req, &documents)?;
                    }
                    CodeActionRequest::METHOD => {
                        handle_code_action(&connection, req, &documents)?;
                    }
                    SelectionRangeRequest::METHOD => {
                        handle_selection_range(&connection, req, &documents)?;
                    }
                    InlayHintRequest::METHOD => {
                        handle_inlay_hints(&connection, req, &documents)?;
                    }
                    DocumentSymbolRequest::METHOD => {
                        handle_document_symbol(&connection, req, &documents)?;
                    }
                    Completion::METHOD => {
                        handle_completion(&connection, req, &documents)?;
                    }
                    "squawk/syntaxTree" => {
                        handle_syntax_tree(&connection, req, &documents)?;
                    }
                    "squawk/tokens" => {
                        handle_tokens(&connection, req, &documents)?;
                    }
                    References::METHOD => {
                        handle_references(&connection, req, &documents)?;
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

fn handle_goto_definition(
    connection: &Connection,
    req: lsp_server::Request,
    documents: &HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: GotoDefinitionParams = serde_json::from_value(req.params)?;
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let content = documents.get(&uri).map_or("", |doc| &doc.content);
    let parse = SourceFile::parse(content);
    let file = parse.tree();
    let line_index = LineIndex::new(content);
    let offset = lsp_utils::offset(&line_index, position).unwrap();

    let ranges = goto_definition(file, offset)
        .into_iter()
        .map(|target_range| {
            debug_assert!(
                !target_range.contains(offset),
                "Our target destination range must not include the source range otherwise go to def won't work in vscode."
            );
            Location {
                uri: uri.clone(),
                range: lsp_utils::range(&line_index, target_range),
            }
        })
        .collect();

    let result = GotoDefinitionResponse::Array(ranges);
    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&result).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}

fn handle_hover(
    connection: &Connection,
    req: lsp_server::Request,
    documents: &HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: HoverParams = serde_json::from_value(req.params)?;
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    let content = documents.get(&uri).map_or("", |doc| &doc.content);
    let parse = SourceFile::parse(content);
    let file = parse.tree();
    let line_index = LineIndex::new(content);
    let offset = lsp_utils::offset(&line_index, position).unwrap();

    let type_info = hover(&file, offset);

    let result = type_info.map(|type_str| Hover {
        contents: HoverContents::Scalar(MarkedString::LanguageString(LanguageString {
            language: "sql".to_string(),
            value: type_str,
        })),
        range: None,
    });

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&result).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}

fn handle_inlay_hints(
    connection: &Connection,
    req: lsp_server::Request,
    documents: &HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: InlayHintParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    let content = documents.get(&uri).map_or("", |doc| &doc.content);
    let parse = SourceFile::parse(content);
    let file = parse.tree();
    let line_index = LineIndex::new(content);

    let hints = inlay_hints(&file);

    let lsp_hints: Vec<InlayHint> = hints
        .into_iter()
        .map(|hint| {
            let line_col = line_index.line_col(hint.position);
            let position = lsp_types::Position::new(line_col.line, line_col.col);
            let kind = match hint.kind {
                squawk_ide::inlay_hints::InlayHintKind::Type => InlayHintKind::TYPE,
                squawk_ide::inlay_hints::InlayHintKind::Parameter => InlayHintKind::PARAMETER,
            };

            let label = if let Some(target_range) = hint.target {
                InlayHintLabel::LabelParts(vec![InlayHintLabelPart {
                    value: hint.label,
                    location: Some(Location {
                        uri: uri.clone(),
                        range: lsp_utils::range(&line_index, target_range),
                    }),
                    tooltip: None,
                    command: None,
                }])
            } else {
                InlayHintLabel::String(hint.label)
            };

            InlayHint {
                position,
                label,
                kind: Some(kind),
                text_edits: None,
                tooltip: None,
                padding_left: None,
                padding_right: None,
                data: None,
            }
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

fn handle_document_symbol(
    connection: &Connection,
    req: lsp_server::Request,
    documents: &HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: DocumentSymbolParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    let content = documents.get(&uri).map_or("", |doc| &doc.content);
    let parse = SourceFile::parse(content);
    let file = parse.tree();
    let line_index = LineIndex::new(content);

    let symbols = document_symbols(&file);

    fn convert_symbol(
        sym: squawk_ide::document_symbols::DocumentSymbol,
        line_index: &LineIndex,
    ) -> DocumentSymbol {
        let range = lsp_utils::range(line_index, sym.full_range);
        let selection_range = lsp_utils::range(line_index, sym.focus_range);

        let children = sym
            .children
            .into_iter()
            .map(|child| convert_symbol(child, line_index))
            .collect::<Vec<_>>();

        let children = (!children.is_empty()).then_some(children);

        DocumentSymbol {
            name: sym.name,
            detail: sym.detail,
            kind: match sym.kind {
                DocumentSymbolKind::Schema => SymbolKind::NAMESPACE,
                DocumentSymbolKind::Table => SymbolKind::STRUCT,
                DocumentSymbolKind::View => SymbolKind::STRUCT,
                DocumentSymbolKind::MaterializedView => SymbolKind::STRUCT,
                DocumentSymbolKind::Function => SymbolKind::FUNCTION,
                DocumentSymbolKind::Aggregate => SymbolKind::FUNCTION,
                DocumentSymbolKind::Procedure => SymbolKind::FUNCTION,
                DocumentSymbolKind::Type => SymbolKind::CLASS,
                DocumentSymbolKind::Enum => SymbolKind::ENUM,
                DocumentSymbolKind::Index => SymbolKind::KEY,
                DocumentSymbolKind::Domain => SymbolKind::CLASS,
                DocumentSymbolKind::Sequence => SymbolKind::CONSTANT,
                DocumentSymbolKind::Trigger => SymbolKind::EVENT,
                DocumentSymbolKind::Tablespace => SymbolKind::NAMESPACE,
                DocumentSymbolKind::Database => SymbolKind::MODULE,
                DocumentSymbolKind::Server => SymbolKind::OBJECT,
                DocumentSymbolKind::Extension => SymbolKind::PACKAGE,
                DocumentSymbolKind::Column => SymbolKind::FIELD,
                DocumentSymbolKind::Variant => SymbolKind::ENUM_MEMBER,
                DocumentSymbolKind::Cursor => SymbolKind::VARIABLE,
                DocumentSymbolKind::PreparedStatement => SymbolKind::VARIABLE,
                DocumentSymbolKind::Channel => SymbolKind::EVENT,
                DocumentSymbolKind::EventTrigger => SymbolKind::EVENT,
                DocumentSymbolKind::Role => SymbolKind::CLASS,
                DocumentSymbolKind::Policy => SymbolKind::VARIABLE,
            },
            tags: None,
            range,
            selection_range,
            children,
            #[allow(deprecated)]
            deprecated: None,
        }
    }

    let lsp_symbols: Vec<DocumentSymbol> = symbols
        .into_iter()
        .map(|sym| convert_symbol(sym, &line_index))
        .collect();

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&lsp_symbols).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}

fn handle_selection_range(
    connection: &Connection,
    req: lsp_server::Request,
    documents: &HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: SelectionRangeParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    let content = documents.get(&uri).map_or("", |doc| &doc.content);
    let parse = SourceFile::parse(content);
    let root = parse.syntax_node();
    let line_index = LineIndex::new(content);

    let mut selection_ranges = vec![];

    for position in params.positions {
        let Some(offset) = lsp_utils::offset(&line_index, position) else {
            continue;
        };

        let mut ranges = Vec::new();
        {
            let mut range = TextRange::new(offset, offset);
            loop {
                ranges.push(range);
                let next = squawk_ide::expand_selection::extend_selection(&root, range);
                if next == range {
                    break;
                } else {
                    range = next
                }
            }
        }

        let mut range = lsp_types::SelectionRange {
            range: lsp_utils::range(&line_index, *ranges.last().unwrap()),
            parent: None,
        };
        for &r in ranges.iter().rev().skip(1) {
            range = lsp_types::SelectionRange {
                range: lsp_utils::range(&line_index, r),
                parent: Some(Box::new(range)),
            }
        }
        selection_ranges.push(range);
    }

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&selection_ranges).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}

fn handle_references(
    connection: &Connection,
    req: lsp_server::Request,
    documents: &HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: ReferenceParams = serde_json::from_value(req.params)?;
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let content = documents.get(&uri).map_or("", |doc| &doc.content);
    let parse = SourceFile::parse(content);
    let file = parse.tree();
    let line_index = LineIndex::new(content);
    let offset = lsp_utils::offset(&line_index, position).unwrap();

    let ranges = find_references(&file, offset);
    let include_declaration = params.context.include_declaration;

    let locations: Vec<Location> = ranges
        .into_iter()
        .filter(|range| include_declaration || !range.contains(offset))
        .map(|range| Location {
            uri: uri.clone(),
            range: lsp_utils::range(&line_index, range),
        })
        .collect();

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&locations).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}

fn handle_completion(
    connection: &Connection,
    req: lsp_server::Request,
    documents: &HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: CompletionParams = serde_json::from_value(req.params)?;
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let content = documents.get(&uri).map_or("", |doc| &doc.content);
    let parse = SourceFile::parse(content);
    let file = parse.tree();
    let line_index = LineIndex::new(content);

    let Some(offset) = lsp_utils::offset(&line_index, position) else {
        let resp = Response {
            id: req.id,
            result: Some(serde_json::to_value(CompletionResponse::Array(vec![])).unwrap()),
            error: None,
        };
        connection.sender.send(Message::Response(resp))?;
        return Ok(());
    };

    let completion_items = completion(&file, offset)
        .into_iter()
        .map(lsp_utils::completion_item)
        .collect();

    let result = CompletionResponse::Array(completion_items);

    let resp = Response {
        id: req.id,
        result: Some(serde_json::to_value(&result).unwrap()),
        error: None,
    };

    connection.sender.send(Message::Response(resp))?;
    Ok(())
}

fn handle_code_action(
    connection: &Connection,
    req: lsp_server::Request,
    documents: &HashMap<Url, DocumentState>,
) -> Result<()> {
    let params: CodeActionParams = serde_json::from_value(req.params)?;
    let uri = params.text_document.uri;

    let mut actions: CodeActionResponse = Vec::new();

    let content = documents.get(&uri).map_or("", |doc| &doc.content);
    let parse = SourceFile::parse(content);
    let file = parse.tree();
    let line_index = LineIndex::new(content);
    let offset = lsp_utils::offset(&line_index, params.range.start).unwrap();

    let ide_actions = code_actions(file, offset).unwrap_or_default();

    for action in ide_actions {
        let lsp_action = lsp_utils::code_action(&line_index, uri.clone(), action);
        actions.push(CodeActionOrCommand::CodeAction(lsp_action));
    }

    for mut diagnostic in params
        .context
        .diagnostics
        .into_iter()
        .filter(|diagnostic| diagnostic.source.as_deref() == Some(DIAGNOSTIC_NAME))
    {
        let Some(rule_name) = diagnostic.code.as_ref().map(|x| match x {
            lsp_types::NumberOrString::String(s) => s.clone(),
            lsp_types::NumberOrString::Number(n) => n.to_string(),
        }) else {
            continue;
        };
        let Some(data) = diagnostic.data.take() else {
            continue;
        };

        let associated_data: AssociatedDiagnosticData =
            serde_json::from_value(data).context("deserializing diagnostic data")?;

        if let Some(ignore_line_edit) = associated_data.ignore_line_edit {
            let disable_line_action = CodeAction {
                title: format!("Disable {rule_name} for this line"),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(WorkspaceEdit {
                    changes: Some({
                        let mut changes = HashMap::new();
                        changes.insert(uri.clone(), vec![ignore_line_edit]);
                        changes
                    }),
                    ..Default::default()
                }),
                command: None,
                is_preferred: Some(false),
                disabled: None,
                data: None,
            };
            actions.push(CodeActionOrCommand::CodeAction(disable_line_action));
        }
        if let Some(ignore_file_edit) = associated_data.ignore_file_edit {
            let disable_file_action = CodeAction {
                title: format!("Disable {rule_name} for the entire file"),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(WorkspaceEdit {
                    changes: Some({
                        let mut changes = HashMap::new();
                        changes.insert(uri.clone(), vec![ignore_file_edit]);
                        changes
                    }),
                    ..Default::default()
                }),
                command: None,
                is_preferred: Some(false),
                disabled: None,
                data: None,
            };
            actions.push(CodeActionOrCommand::CodeAction(disable_file_action));
        }

        let title = format!("Show documentation for {rule_name}");
        let documentation_action = CodeAction {
            title: title.clone(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(vec![diagnostic.clone()]),
            edit: None,
            command: Some(Command {
                title,
                command: "vscode.open".to_string(),
                arguments: Some(vec![serde_json::to_value(format!(
                    "https://squawkhq.com/docs/{rule_name}"
                ))?]),
            }),
            is_preferred: Some(false),
            disabled: None,
            data: None,
        };
        actions.push(CodeActionOrCommand::CodeAction(documentation_action));

        if !associated_data.title.is_empty() && !associated_data.edits.is_empty() {
            let fix_action = CodeAction {
                title: associated_data.title,
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(WorkspaceEdit {
                    changes: Some({
                        let mut changes = HashMap::new();
                        changes.insert(uri.clone(), associated_data.edits);
                        changes
                    }),
                    ..Default::default()
                }),
                command: None,
                is_preferred: Some(true),
                disabled: None,
                data: None,
            };
            actions.push(CodeActionOrCommand::CodeAction(fix_action));
        }
    }

    let result: CodeActionResponse = actions;
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
    let diagnostics = lint::lint(content);
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

    let diagnostics = lint::lint(&doc_state.content);
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

    info!("Generating syntax tree for: {uri}");

    let content = documents.get(&uri).map_or("", |doc| &doc.content);

    let parse = SourceFile::parse(content);
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

    info!("Generating tokens for: {uri}");

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
