mod diagnostic;
mod dispatch;
mod global_state;
mod handlers;
mod ignore;
mod lint;
mod lsp_utils;
mod panic;
mod semantic_tokens;
mod server;

pub use server::run;
