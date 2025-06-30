use anyhow::Result;
use log::info;
use lsp_server::{Connection, Message, Response};
use lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, InitializeParams, Location, OneOf, Position,
    Range, ServerCapabilities,
    request::{GotoDefinition, Request},
};

pub fn run_server() -> Result<()> {
    info!("Starting Squawk LSP server");

    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    })
    .unwrap();

    info!("LSP server initalizing connection...");
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
            }
        }
    }
    Ok(())
}

fn handle_goto_definition(connection: &Connection, req: lsp_server::Request) -> Result<()> {
    let params: GotoDefinitionParams = serde_json::from_value(req.params)?;

    let location = Location {
        uri: params
            .text_document_position_params
            .text_document
            .uri
            .clone(),
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
