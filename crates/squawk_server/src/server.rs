use anyhow::Result;
use log::info;
use lsp_server::{Connection, Message};
use lsp_types::{
    CodeActionKind, CodeActionOptions, CodeActionProviderCapability, CompletionOptions,
    DiagnosticOptions, DiagnosticServerCapabilities, FoldingRangeProviderCapability,
    HoverProviderCapability, InitializeParams, OneOf, SelectionRangeProviderCapability,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, WorkDoneProgressOptions,
    notification::{DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument},
    request::{
        CodeActionRequest, Completion, DocumentDiagnosticRequest, DocumentSymbolRequest,
        FoldingRangeRequest, GotoDefinition, HoverRequest, InlayHintRequest, References,
        SelectionRangeRequest,
    },
};

use crate::dispatch::{NotificationDispatcher, RequestDispatcher};
use crate::handlers::{
    SyntaxTreeRequest, TokensRequest, handle_code_action, handle_completion, handle_did_change,
    handle_did_close, handle_did_open, handle_document_diagnostic, handle_document_symbol,
    handle_folding_range, handle_goto_definition, handle_hover, handle_inlay_hints,
    handle_references, handle_selection_range, handle_syntax_tree, handle_tokens,
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
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            identifier: None,
            inter_file_dependencies: false,
            workspace_diagnostics: false,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        })),
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

                RequestDispatcher::new(&connection, req, &system)
                    .on::<GotoDefinition>(handle_goto_definition)?
                    .on::<HoverRequest>(handle_hover)?
                    .on::<CodeActionRequest>(handle_code_action)?
                    .on::<SelectionRangeRequest>(handle_selection_range)?
                    .on::<InlayHintRequest>(handle_inlay_hints)?
                    .on::<DocumentSymbolRequest>(handle_document_symbol)?
                    .on::<FoldingRangeRequest>(handle_folding_range)?
                    .on::<Completion>(handle_completion)?
                    .on::<DocumentDiagnosticRequest>(handle_document_diagnostic)?
                    .on::<SyntaxTreeRequest>(handle_syntax_tree)?
                    .on::<TokensRequest>(handle_tokens)?
                    .on::<References>(handle_references)?
                    .finish();
            }
            Message::Response(resp) => {
                info!("Received response: id={:?}", resp.id);
            }
            Message::Notification(notif) => {
                info!("Received notification: method={}", notif.method);

                NotificationDispatcher::new(&connection, notif, &mut system)
                    .on::<DidOpenTextDocument>(handle_did_open)?
                    .on::<DidChangeTextDocument>(handle_did_change)?
                    .on::<DidCloseTextDocument>(handle_did_close)?
                    .finish();
            }
        }
    }
    Ok(())
}
