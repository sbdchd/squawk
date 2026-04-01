use std::{num::NonZeroUsize, sync::Arc, time::Instant};

use crossbeam_channel::{Receiver, Sender, select, unbounded};
use log::info;
use lsp_server::{Message, Request, Response};
use lsp_types::Url;
use lsp_types::notification::Notification as _;
use lsp_types::notification::{
    Cancel, DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument,
};
use rustc_hash::FxHashMap;
use salsa::Setter;
use squawk_ide::db::{Database, File};
use squawk_thread::TaskPool;

use lsp_types::request::{
    CodeActionRequest, Completion, DocumentDiagnosticRequest, DocumentSymbolRequest,
    FoldingRangeRequest, GotoDefinition, HoverRequest, InlayHintRequest, References,
    SelectionRangeRequest, Shutdown,
};

use crate::dispatch::{NotificationDispatcher, RequestDispatcher};
use crate::handlers::{
    SyntaxTreeRequest, TokensRequest, handle_cancel, handle_code_action, handle_completion,
    handle_did_change, handle_did_close, handle_did_open, handle_document_diagnostic,
    handle_document_symbol, handle_folding_range, handle_goto_definition, handle_hover,
    handle_inlay_hints, handle_references, handle_selection_range, handle_shutdown,
    handle_syntax_tree, handle_tokens,
};

type ReqQueue = lsp_server::ReqQueue<(String, Instant), ()>;

pub(crate) struct Handle<H, C> {
    pub(crate) handle: H,
    pub(crate) receiver: C,
}

pub(super) struct GlobalState {
    db: Database,
    files: Arc<FxHashMap<Url, File>>,
    req_queue: ReqQueue,
    sender: Sender<Message>,
    pub(crate) task_pool: Handle<TaskPool<TaskResult>, Receiver<TaskResult>>,
    shutdown_requested: bool,
}

impl GlobalState {
    pub(super) fn new(sender: Sender<Message>) -> Self {
        let threads = std::thread::available_parallelism().unwrap_or(NonZeroUsize::MIN);
        let task_pool = {
            let (sender, receiver) = unbounded();
            let handle = TaskPool::new_with_threads(sender.clone(), threads);
            Handle { handle, receiver }
        };
        Self {
            db: Database::default(),
            files: Arc::new(FxHashMap::default()),
            req_queue: ReqQueue::default(),
            task_pool,
            sender,
            shutdown_requested: false,
        }
    }

    /// Readonly snapshot of the database & files for request handlers
    pub(crate) fn snapshot(&self) -> Snapshot {
        Snapshot {
            db: self.db.clone(),
            files: self.files.clone(),
        }
    }

    pub(crate) fn db(&self) -> &Database {
        &self.db
    }

    pub(crate) fn file(&self, uri: &Url) -> Option<File> {
        self.files.get(uri).copied()
    }

    pub(crate) fn set(&mut self, uri: Url, content: String) {
        if let Some(file) = self.files.get(&uri).copied() {
            file.set_content(&mut self.db).to(content.into());
        } else {
            let file = File::new(&self.db, content.into());
            Arc::make_mut(&mut self.files).insert(uri, file);
        }
    }

    pub(crate) fn remove(&mut self, uri: &Url) {
        Arc::make_mut(&mut self.files).remove(uri);
    }

    /// Track the request time and support marking cancellation
    pub(crate) fn register_request(
        &mut self,
        request: &lsp_server::Request,
        request_received: Instant,
    ) {
        self.req_queue.incoming.register(
            request.id.clone(),
            (request.method.clone(), request_received),
        );
    }

    /// Wrapper to check for cancellation before sending
    pub(crate) fn respond(&mut self, response: Response) {
        if let Some((method, start)) = self.req_queue.incoming.complete(&response.id) {
            let duration = start.elapsed();
            tracing::debug!(name: "message response", method, %response.id, duration = format_args!("{:0.2?}", duration));
            self.send(response.into());
        }
    }

    /// Mark the request as cancelled
    pub(crate) fn cancel(&mut self, request_id: lsp_server::RequestId) {
        if let Some(response) = self.req_queue.incoming.cancel(request_id) {
            self.send(response.into());
        }
    }

    pub(crate) fn is_completed(&self, request: &lsp_server::Request) -> bool {
        self.req_queue.incoming.is_completed(&request.id)
    }

    pub(crate) fn request_shutdown(&mut self) {
        self.shutdown_requested = true;
    }

    #[track_caller]
    pub(crate) fn send(&self, message: Message) {
        self.sender.send(message).unwrap();
    }

    pub(crate) fn run(&mut self, inbox: Receiver<Message>) -> anyhow::Result<()> {
        let outbox = &self.task_pool.receiver.clone();
        while let Ok(event) = self.next_event(&inbox, outbox) {
            let loop_start = Instant::now();
            match event {
                Event::Inbox(msg) => match msg {
                    Message::Request(req) => self.handle_request(req, loop_start),
                    Message::Response(resp) => {
                        info!("Received response: id={:?}", resp.id);
                    }
                    Message::Notification(notif) => {
                        info!("Received notification: method={}", notif.method);

                        if notif.method == lsp_types::notification::Exit::METHOD {
                            return Ok(());
                        }

                        NotificationDispatcher::new(notif, self)
                            .on::<Cancel>(handle_cancel)?
                            .on::<DidOpenTextDocument>(handle_did_open)?
                            .on::<DidChangeTextDocument>(handle_did_change)?
                            .on::<DidCloseTextDocument>(handle_did_close)?
                            .finish();
                    }
                },
                Event::Outbox(result) => {
                    match result {
                        TaskResult::Response(resp) => {
                            // Instead of having the tasks send directly via the sender
                            // channel, we handle them on the main thread so we can check
                            // for cancellation first.
                            self.respond(resp)
                        }
                        TaskResult::Retry(req) if !self.is_completed(&req) => {
                            self.handle_request(req, loop_start)
                        }
                        TaskResult::Retry(_) => (),
                    }
                }
            }
        }

        Ok(())
    }

    fn next_event(
        &self,
        inbox: &Receiver<Message>,
        outbox: &Receiver<TaskResult>,
    ) -> Result<Event, crossbeam_channel::RecvError> {
        select! {
            recv(inbox) -> msg => msg.map(Event::Inbox),
            recv(outbox) -> task => task.map(Event::Outbox),
        }
    }

    fn handle_request(&mut self, req: Request, loop_start: Instant) {
        info!("Received request: method={}, id={:?}", req.method, req.id);

        self.register_request(&req, loop_start);

        if self.shutdown_requested {
            tracing::warn!(
                "Received request `{}` after server shutdown was requested, discarding",
                &req.method
            );

            self.respond(Response::new_err(
                req.id,
                lsp_server::ErrorCode::InvalidRequest as i32,
                "Shutdown already requested".to_owned(),
            ));
            return;
        }

        const RETRY: bool = true;
        const NO_RETRY: bool = false;

        RequestDispatcher::new(req, self)
            // Request handlers that must run on the main thread because they
            // mutate GlobalState:
            .on_sync_mut::<Shutdown>(handle_shutdown)
            // Request handlers which are related to the user typing are run on
            // the main thread to reduce latency:
            .on_sync::<SelectionRangeRequest>(handle_selection_range)
            // latency-sensitive but can't run on the main thread due to
            // semantic analysis, so we use a higher priority thread
            .on_latency_sensitive::<RETRY, Completion>(handle_completion)
            .on::<NO_RETRY, GotoDefinition>(handle_goto_definition)
            .on::<NO_RETRY, HoverRequest>(handle_hover)
            .on::<NO_RETRY, CodeActionRequest>(handle_code_action)
            .on::<NO_RETRY, InlayHintRequest>(handle_inlay_hints)
            .on::<RETRY, DocumentSymbolRequest>(handle_document_symbol)
            .on::<RETRY, FoldingRangeRequest>(handle_folding_range)
            .on::<NO_RETRY, DocumentDiagnosticRequest>(handle_document_diagnostic)
            .on::<NO_RETRY, SyntaxTreeRequest>(handle_syntax_tree)
            .on::<NO_RETRY, TokensRequest>(handle_tokens)
            .on::<NO_RETRY, References>(handle_references)
            .finish();
    }
}

pub(crate) struct Snapshot {
    pub(crate) db: Database,
    pub(crate) files: Arc<FxHashMap<Url, File>>,
}

impl Snapshot {
    pub(crate) fn db(&self) -> &Database {
        &self.db
    }

    pub(crate) fn file(&self, uri: &Url) -> Option<File> {
        self.files.get(uri).copied()
    }
}

enum Event {
    Inbox(Message),
    Outbox(TaskResult),
}

pub(crate) enum TaskResult {
    Response(Response),
    Retry(Request),
}
