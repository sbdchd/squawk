use anyhow::Result;
use log::info;
use lsp_server::{Connection, Message};
use lsp_types::{
    CodeActionKind, CodeActionOptions, CodeActionProviderCapability, CompletionOptions,
    FoldingRangeProviderCapability, HoverProviderCapability, InitializeParams, OneOf,
    SelectionRangeProviderCapability, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, WorkDoneProgressOptions,
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification as _,
    },
    request::{
        CodeActionRequest, Completion, DocumentSymbolRequest, FoldingRangeRequest, GotoDefinition,
        HoverRequest, InlayHintRequest, References, Request, SelectionRangeRequest,
    },
};

use crate::handlers::{
    handle_code_action, handle_completion, handle_did_change, handle_did_close, handle_did_open,
    handle_document_symbol, handle_folding_range, handle_goto_definition, handle_hover,
    handle_inlay_hints, handle_references, handle_selection_range, handle_syntax_tree,
    handle_tokens,
};
use crate::system::GlobalState;

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
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
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

    let mut system = GlobalState::new();

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
                        handle_goto_definition(&connection, req, &system)?;
                    }
                    HoverRequest::METHOD => {
                        handle_hover(&connection, req, &system)?;
                    }
                    CodeActionRequest::METHOD => {
                        handle_code_action(&connection, req, &system)?;
                    }
                    SelectionRangeRequest::METHOD => {
                        handle_selection_range(&connection, req, &system)?;
                    }
                    InlayHintRequest::METHOD => {
                        handle_inlay_hints(&connection, req, &system)?;
                    }
                    DocumentSymbolRequest::METHOD => {
                        handle_document_symbol(&connection, req, &system)?;
                    }
                    FoldingRangeRequest::METHOD => {
                        handle_folding_range(&connection, req, &system)?;
                    }
                    Completion::METHOD => {
                        handle_completion(&connection, req, &system)?;
                    }
                    "squawk/syntaxTree" => {
                        handle_syntax_tree(&connection, req, &system)?;
                    }
                    "squawk/tokens" => {
                        handle_tokens(&connection, req, &system)?;
                    }
                    References::METHOD => {
                        handle_references(&connection, req, &system)?;
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
                        handle_did_open(&connection, notif, &mut system)?;
                    }
                    DidChangeTextDocument::METHOD => {
                        handle_did_change(&connection, notif, &mut system)?;
                    }
                    DidCloseTextDocument::METHOD => {
                        handle_did_close(&connection, notif, &mut system)?;
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
