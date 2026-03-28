// Based on https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/rust-analyzer/src/handlers/dispatch.rs#L277C5-L277C5

use anyhow::Result;
use log::{error, info};
use lsp_server::{Connection, Message, Response};
use lsp_types::{notification::Notification as LspNotification, request::Request as LspRequest};
use squawk_thread::ThreadIntent;

use crate::system::{GlobalState, MutableSystem, System};

pub(crate) struct RequestDispatcher<'a> {
    req: Option<lsp_server::Request>,
    system: &'a mut GlobalState,
}

impl<'a> RequestDispatcher<'a> {
    pub(crate) fn new(req: lsp_server::Request, system: &'a mut GlobalState) -> Self {
        Self {
            req: Some(req),
            system,
        }
    }

    fn parse<R>(&mut self) -> Option<(lsp_server::RequestId, R::Params)>
    where
        R: LspRequest,
    {
        let req = self.req.take_if(|req| req.method.as_str() == R::METHOD)?;
        let id = req.id.clone();

        match req.extract(R::METHOD) {
            Ok((id, params)) => Some((id, params)),
            Err(err) => {
                let response = Response::new_err(
                    id,
                    lsp_server::ErrorCode::ParseError as i32,
                    err.to_string(),
                );
                if let Err(err) = self.system.sender.send(Message::Response(response)) {
                    error!("Failed to send parse error response: {err}");
                }
                None
            }
        }
    }

    pub(crate) fn on<R>(
        self,
        handler: fn(&dyn System, R::Params) -> Result<R::Result>,
    ) -> Result<Self>
    where
        R: LspRequest,
        R::Params: Send + 'static,
    {
        self.on_with_thread_intent::<R>(ThreadIntent::Worker, handler)
    }

    pub(crate) fn on_latency_sensitive<R>(
        self,
        handler: fn(&dyn System, R::Params) -> Result<R::Result>,
    ) -> Result<Self>
    where
        R: LspRequest,
        R::Params: Send + 'static,
    {
        self.on_with_thread_intent::<R>(ThreadIntent::LatencySensitive, handler)
    }

    fn on_with_thread_intent<R>(
        mut self,
        intent: ThreadIntent,
        handler: fn(&dyn System, R::Params) -> Result<R::Result>,
    ) -> Result<Self>
    where
        R: LspRequest,
        R::Params: Send + 'static,
    {
        if let Some((id, params)) = self.parse::<R>() {
            let snapshot = self.system.snapshot();

            self.system.task_pool.spawn(intent, move || {
                let resp = match handler(&snapshot, params) {
                    Ok(result) => Response::new_ok(id, result),
                    Err(err) => {
                        error!("Request handler failed: {err}");
                        Response::new_err(
                            id,
                            lsp_server::ErrorCode::InternalError as i32,
                            err.to_string(),
                        )
                    }
                };

                Message::Response(resp)
            });
        }

        Ok(self)
    }

    pub(crate) fn finish(self) {
        if let Some(req) = self.req {
            info!("Ignoring unhandled request: {}", req.method);
        }
    }
}

pub(crate) struct NotificationDispatcher<'a> {
    connection: &'a Connection,
    notif: Option<lsp_server::Notification>,
    system: &'a mut dyn MutableSystem,
}

impl<'a> NotificationDispatcher<'a> {
    pub(crate) fn new(
        connection: &'a Connection,
        notif: lsp_server::Notification,
        system: &'a mut dyn MutableSystem,
    ) -> Self {
        Self {
            connection,
            notif: Some(notif),
            system,
        }
    }

    fn parse<N>(&mut self) -> Option<N::Params>
    where
        N: LspNotification,
    {
        let notif = self
            .notif
            .take_if(|notif| notif.method.as_str() == N::METHOD)?;

        match notif.extract(N::METHOD) {
            Ok(params) => Some(params),
            Err(err) => {
                error!("Failed to parse notification params: {err}");
                None
            }
        }
    }

    pub(crate) fn on<N>(
        mut self,
        handler: fn(&Connection, N::Params, &mut dyn MutableSystem) -> Result<()>,
    ) -> Result<Self>
    where
        N: LspNotification,
    {
        if let Some(params) = self.parse::<N>() {
            handler(self.connection, params, self.system)?;
        }

        Ok(self)
    }

    pub(crate) fn finish(self) {
        if let Some(notif) = self.notif {
            info!("Ignoring unhandled notification: {}", notif.method);
        }
    }
}
