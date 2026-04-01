// Based on https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/rust-analyzer/src/handlers/dispatch.rs#L277C5-L277C5

use std::panic::UnwindSafe;

use anyhow::Result;
use log::{error, info};
use lsp_server::{Request, Response};
use lsp_types::{notification::Notification as LspNotification, request::Request as LspRequest};
use salsa::Cancelled;
use serde::{Serialize, de::DeserializeOwned};
use squawk_thread::ThreadIntent;

use crate::{
    global_state::{GlobalState, Snapshot, TaskResult},
    panic::PanicError,
};

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

    fn parse<R>(&mut self) -> Option<(Request, R::Params)>
    where
        R: LspRequest,
    {
        let req = self.req.take_if(|req| req.method.as_str() == R::METHOD)?;
        let id = req.id.clone();
        match from_json(R::METHOD, &req.params) {
            Ok(params) => Some((req, params)),
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

    pub(crate) fn on_sync<R>(
        mut self,
        handler: fn(&Snapshot, R::Params) -> Result<R::Result>,
    ) -> Self
    where
        R: LspRequest,
        R::Params: Send + 'static + UnwindSafe,
    {
        let Some((request, params)) = self.parse::<R>() else {
            return self;
        };

        let snapshot = self.global_state.snapshot();
        let result = crate::panic::catch_unwind(|| handler(&snapshot, params));
        if let Ok(response) = thread_result_to_response::<R>(request.id.clone(), result) {
            self.global_state.respond(response);
        }
        self
    }

    pub(crate) fn on_sync_mut<R>(
        mut self,
        handler: fn(&mut GlobalState, R::Params) -> Result<R::Result>,
    ) -> Self
    where
        R: LspRequest,
        R::Params: Send + 'static,
    {
        let Some((request, params)) = self.parse::<R>() else {
            return self;
        };

        let result = handler(self.global_state, params);
        if let Ok(response) = result_to_response::<R>(request.id.clone(), result) {
            self.global_state.respond(response);
        }
        self
    }

    pub(crate) fn on<const ALLOW_RETRYING: bool, R>(
        self,
        handler: fn(&Snapshot, R::Params) -> Result<R::Result>,
    ) -> Self
    where
        R: LspRequest,
        R::Params: Send + 'static + UnwindSafe,
    {
        self.on_with_thread_intent::<ALLOW_RETRYING, R>(ThreadIntent::Worker, handler)
    }

    pub(crate) fn on_latency_sensitive<const ALLOW_RETRYING: bool, R>(
        self,
        handler: fn(&Snapshot, R::Params) -> Result<R::Result>,
    ) -> Self
    where
        R: LspRequest,
        R::Params: Send + 'static + UnwindSafe,
    {
        self.on_with_thread_intent::<ALLOW_RETRYING, R>(ThreadIntent::LatencySensitive, handler)
    }

    fn on_with_thread_intent<const ALLOW_RETRYING: bool, R>(
        mut self,
        intent: ThreadIntent,
        handler: fn(&Snapshot, R::Params) -> Result<R::Result>,
    ) -> Self
    where
        R: LspRequest,
        R::Params: Send + 'static + UnwindSafe,
    {
        if let Some((request, params)) = self.parse::<R>() {
            let snapshot = self.global_state.snapshot();

            self.global_state.task_pool.handle.spawn(intent, move || {
                let result = crate::panic::catch_unwind(|| handler(&snapshot, params));
                match thread_result_to_response::<R>(request.id.clone(), result) {
                    Ok(response) => TaskResult::Response(response),
                    Err(_cancelled) if ALLOW_RETRYING => TaskResult::Retry(request),
                    Err(_cancelled) => TaskResult::Response(Response::new_err(
                        request.id,
                        lsp_server::ErrorCode::ContentModified as i32,
                        "content modified".to_owned(),
                    )),
                }
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

fn thread_result_to_response<R>(
    id: lsp_server::RequestId,
    result: Result<anyhow::Result<R::Result>, PanicError>,
) -> Result<lsp_server::Response, PanicError>
where
    R: lsp_types::request::Request,
    R::Params: DeserializeOwned,
    R::Result: Serialize,
{
    match result {
        Ok(handler_result) => match handler_result {
            Ok(result) => Ok(Response::new_ok(id, result)),
            Err(error) => Ok(Response::new_err(
                id,
                lsp_server::ErrorCode::InternalError as i32,
                error.to_string(),
            )),
        },
        Err(panic) => {
            // Check if the request was canceled due to some modifications to the salsa database.
            if panic.payload.downcast_ref::<salsa::Cancelled>().is_some() {
                log::debug!(
                    "request id={} was cancelled by salsa, sending content modified",
                    id
                );
                Err(panic)
            } else {
                let error = panic.to_string();
                // we don't retry non-salsa cancellation panics
                Ok(Response::new_err(
                    id,
                    lsp_server::ErrorCode::InternalError as i32,
                    format!("request handler error: {error}"),
                ))
            }
        }
    }
}

fn result_to_response<R>(
    id: lsp_server::RequestId,
    result: anyhow::Result<R::Result>,
) -> std::result::Result<lsp_server::Response, Cancelled>
where
    R: lsp_types::request::Request,
{
    match result {
        Ok(resp) => Ok(lsp_server::Response::new_ok(id, &resp)),
        Err(e) => match e.downcast::<Cancelled>() {
            Ok(cancelled) => Err(cancelled),
            Err(e) => Ok(lsp_server::Response::new_err(
                id,
                lsp_server::ErrorCode::InternalError as i32,
                e.to_string(),
            )),
        },
    }
}

// lsp-server has req.extract(R::METHOD), but it doesn't work for us due to
// ownership so we use this instead.
pub fn from_json<T: DeserializeOwned>(
    what: &'static str,
    json: &serde_json::Value,
) -> anyhow::Result<T> {
    serde_json::from_value(json.clone())
        .map_err(|e| anyhow::format_err!("Failed to deserialize {what}: {e}; {json}"))
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
