// Based on https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/rust-analyzer/src/handlers/dispatch.rs#L277C5-L277C5

use std::panic::UnwindSafe;

use anyhow::Result;
use log::{error, info};
use lsp_server::{RequestId, Response};
use lsp_types::{notification::Notification as LspNotification, request::Request as LspRequest};
use squawk_thread::ThreadIntent;

use crate::global_state::{GlobalState, Snapshot};

pub(crate) struct RequestDispatcher<'a> {
    req: Option<lsp_server::Request>,
    global_state: &'a mut GlobalState,
}

impl<'a> RequestDispatcher<'a> {
    pub(crate) fn new(req: lsp_server::Request, global_state: &'a mut GlobalState) -> Self {
        Self {
            req: Some(req),
            global_state,
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
                self.global_state.respond(response);
                None
            }
        }
    }

    pub(crate) fn on_sync_mut<R>(
        mut self,
        handler: fn(&mut GlobalState, R::Params) -> Result<R::Result>,
    ) -> Self
    where
        R: LspRequest,
        R::Params: Send + 'static,
    {
        if let Some((id, params)) = self.parse::<R>() {
            let result = handler(self.global_state, params);
            let response = result_to_response::<R>(id, result);
            self.global_state.respond(response);
        }
        self
    }

    pub(crate) fn on<R>(self, handler: fn(&Snapshot, R::Params) -> Result<R::Result>) -> Self
    where
        R: LspRequest,
        R::Params: Send + 'static + UnwindSafe,
    {
        self.on_with_thread_intent::<R>(ThreadIntent::Worker, handler)
    }

    pub(crate) fn on_latency_sensitive<R>(
        self,
        handler: fn(&Snapshot, R::Params) -> Result<R::Result>,
    ) -> Self
    where
        R: LspRequest,
        R::Params: Send + 'static + UnwindSafe,
    {
        self.on_with_thread_intent::<R>(ThreadIntent::LatencySensitive, handler)
    }

    fn on_with_thread_intent<R>(
        mut self,
        intent: ThreadIntent,
        handler: fn(&Snapshot, R::Params) -> Result<R::Result>,
    ) -> Self
    where
        R: LspRequest,
        R::Params: Send + 'static + UnwindSafe,
    {
        if let Some((id, params)) = self.parse::<R>() {
            let snapshot = self.global_state.snapshot();

            self.global_state.task_pool.handle.spawn(intent, move || {
                crate::panic::catch_unwind(|| {
                    let result = handler(&snapshot, params);
                    result_to_response::<R>(id.clone(), result)
                })
                .unwrap_or_else(|error| panic_response(id, &error))
            });
        }

        self
    }

    pub(crate) fn finish(self) {
        if let Some(req) = self.req {
            info!("Ignoring unhandled request: {}", req.method);
        }
    }
}

fn panic_response(id: RequestId, error: &crate::panic::PanicError) -> Response {
    // Check if the request was canceled due to some modifications to the salsa database.
    if error.payload.downcast_ref::<salsa::Cancelled>().is_some() {
        // TODO: trigger retries when we have that setup, we'll reenque the task
        log::debug!(
            "request id={} was cancelled by salsa, sending content modified",
            id
        );
        Response::new_err(
            id,
            lsp_server::ErrorCode::ContentModified as i32,
            "content modified".to_string(),
        )
    } else {
        Response::new_err(
            id,
            lsp_server::ErrorCode::InternalError as i32,
            "request handler error".to_string(),
        )
    }
}

fn result_to_response<R>(id: RequestId, result: Result<R::Result>) -> Response
where
    R: LspRequest,
{
    match result {
        Ok(result) => Response::new_ok(id, result),
        Err(err) => {
            error!("Request handler failed: {err}");
            Response::new_err(
                id,
                lsp_server::ErrorCode::InternalError as i32,
                err.to_string(),
            )
        }
    }
}

pub(crate) struct NotificationDispatcher<'a> {
    notif: Option<lsp_server::Notification>,
    state: &'a mut GlobalState,
}

impl<'a> NotificationDispatcher<'a> {
    pub(crate) fn new(notif: lsp_server::Notification, state: &'a mut GlobalState) -> Self {
        Self {
            notif: Some(notif),
            state,
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
        handler: fn(&mut GlobalState, N::Params) -> Result<()>,
    ) -> Result<Self>
    where
        N: LspNotification,
    {
        if let Some(params) = self.parse::<N>() {
            handler(self.state, params)?;
        }

        Ok(self)
    }

    pub(crate) fn finish(self) {
        if let Some(notif) = self.notif {
            info!("Ignoring unhandled notification: {}", notif.method);
        }
    }
}
