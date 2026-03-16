// Based on https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/rust-analyzer/src/handlers/dispatch.rs#L277C5-L277C5

use anyhow::Result;
use log::{error, info};
use lsp_server::{Connection, Message, Response};
use lsp_types::{notification::Notification as LspNotification, request::Request as LspRequest};

use crate::system::System;

pub(crate) struct RequestDispatcher<'a> {
    connection: &'a Connection,
    req: Option<lsp_server::Request>,
    system: &'a dyn System,
}

impl<'a> RequestDispatcher<'a> {
    pub(crate) fn new(
        connection: &'a Connection,
        req: lsp_server::Request,
        system: &'a dyn System,
    ) -> Self {
        Self {
            connection,
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
                    lsp_server::ErrorCode::InvalidParams as i32,
                    err.to_string(),
                );
                if let Err(err) = self.connection.sender.send(Message::Response(response)) {
                    error!("Failed to send parse error response: {err}");
                }
                None
            }
        }
    }

    pub(crate) fn on<R>(
        mut self,
        handler: fn(&dyn System, R::Params) -> Result<R::Result>,
    ) -> Result<Self>
    where
        R: LspRequest,
    {
        if let Some((id, params)) = self.parse::<R>() {
            let resp = match handler(self.system, params) {
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
            self.connection.sender.send(Message::Response(resp))?;
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
    system: &'a mut dyn System,
}

impl<'a> NotificationDispatcher<'a> {
    pub(crate) fn new(
        connection: &'a Connection,
        notif: lsp_server::Notification,
        system: &'a mut dyn System,
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
        handler: fn(&Connection, N::Params, &mut dyn System) -> Result<()>,
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
