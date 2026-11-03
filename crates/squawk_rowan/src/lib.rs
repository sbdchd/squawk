//! A generic library for lossless syntax trees.
//! See `examples/s_expressions.rs` for a tutorial.
#![forbid(
    // missing_debug_implementations,
    unconditional_recursion,
    future_incompatible,
    // missing_docs,
)]
#![deny(unsafe_code)]

pub mod cursor;
mod green;

pub mod api;
mod syntax_text;
mod utility_types;

pub mod ast;
#[cfg(feature = "serde1")]
mod serde_impls;

pub use text_size::{TextLen, TextRange, TextSize};

pub use crate::{
    api::{
        Language, SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
    },
    green::{
        Children, GreenElementRef, GreenNode, GreenNodeBuilder, GreenNodeRef, GreenTokenRef,
        SyntaxKind,
    },
    syntax_text::SyntaxText,
    utility_types::{Direction, NodeOrToken, TokenAtOffset, WalkEvent},
};
