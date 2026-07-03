use anyhow::Result;
use gen_lsp_types::{
    CodeActionKind, CodeActionOptions, CodeActionProvider, CompletionOptions, DefinitionProvider,
    DiagnosticOptions, DiagnosticProvider, DocumentSymbolProvider, FoldingRangeProvider, Full,
    HoverProvider, InitializeParams, InlayHintProvider, ReferencesProvider, SelectionRangeProvider,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensOptionsRange,
    SemanticTokensProvider, ServerCapabilities, TextDocumentSync, TextDocumentSyncKind,
    WorkDoneProgressOptions,
};
use log::info;
use lsp_server::Connection;

use crate::{
    global_state::GlobalState,
    semantic_tokens::{SUPPORTED_MODIFIERS, SUPPORTED_TYPES},
};

pub fn run() -> Result<()> {
    info!("Starting Squawk LSP server");

    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSync::Kind(TextDocumentSyncKind::Incremental)),
        code_action_provider: Some(CodeActionProvider::CodeActionOptions(CodeActionOptions {
            code_action_kinds: Some(vec![
                CodeActionKind::QuickFix,
                CodeActionKind::RefactorRewrite,
            ]),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
            resolve_provider: None,
            documentation: None,
        })),
        selection_range_provider: Some(SelectionRangeProvider::Bool(true)),
        references_provider: Some(ReferencesProvider::Bool(true)),
        definition_provider: Some(DefinitionProvider::Bool(true)),
        hover_provider: Some(HoverProvider::Bool(true)),
        inlay_hint_provider: Some(InlayHintProvider::Bool(true)),
        diagnostic_provider: Some(DiagnosticProvider::DiagnosticOptions(DiagnosticOptions {
            identifier: None,
            inter_file_dependencies: false,
            workspace_diagnostics: false,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        })),
        document_symbol_provider: Some(DocumentSymbolProvider::Bool(true)),
        folding_range_provider: Some(FoldingRangeProvider::Bool(true)),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![".".to_owned()]),
            all_commit_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
            completion_item: None,
        }),
        semantic_tokens_provider: Some(SemanticTokensProvider::SemanticTokensOptions(
            SemanticTokensOptions {
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: None,
                },
                legend: SemanticTokensLegend {
                    token_types: SUPPORTED_TYPES.iter().cloned().map(String::from).collect(),
                    token_modifiers: SUPPORTED_MODIFIERS
                        .iter()
                        .cloned()
                        .map(String::from)
                        .collect(),
                },
                range: Some(SemanticTokensOptionsRange::Bool(true)),
                full: Some(Full::Bool(true)),
            },
        )),
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

    GlobalState::new(connection.sender).run(connection.receiver)
}
