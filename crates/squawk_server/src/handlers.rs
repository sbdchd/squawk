mod code_action;
mod completion;
mod diagnostic;
mod document_symbol;
mod folding_range;
mod goto_definition;
mod hover;
mod inlay_hints;
mod notifications;
mod references;
mod selection_range;
mod semantic_tokens;
mod shutdown;
mod syntax_tree;
mod tokens;

pub(crate) use code_action::handle_code_action;
pub(crate) use completion::handle_completion;
pub(crate) use diagnostic::handle_document_diagnostic;
pub(crate) use document_symbol::handle_document_symbol;
pub(crate) use folding_range::handle_folding_range;
pub(crate) use goto_definition::handle_goto_definition;
pub(crate) use hover::handle_hover;
pub(crate) use inlay_hints::handle_inlay_hints;
pub(crate) use notifications::{
    handle_cancel, handle_did_change, handle_did_close, handle_did_open,
};
pub(crate) use references::handle_references;
pub(crate) use selection_range::handle_selection_range;
pub(crate) use semantic_tokens::{handle_semantic_tokens_full, handle_semantic_tokens_range};
pub(crate) use shutdown::handle_shutdown;
pub(crate) use syntax_tree::{SyntaxTreeRequest, handle_syntax_tree};
pub(crate) use tokens::{TokensRequest, handle_tokens};
