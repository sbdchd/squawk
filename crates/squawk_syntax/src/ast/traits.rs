// based on rust-analyzer's ast traits
// https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/ast/traits.rs
use crate::ast;
use crate::ast::{AstNode, support};

pub trait NameLike: AstNode {}

pub trait HasCreateTable: AstNode {
    #[inline]
    fn path(&self) -> Option<ast::Path> {
        support::child(self.syntax())
    }

    #[inline]
    fn table_arg_list(&self) -> Option<ast::TableArgList> {
        support::child(self.syntax())
    }
}

pub trait HasWithClause: AstNode {
    #[inline]
    fn with_clause(&self) -> Option<ast::WithClause> {
        support::child(self.syntax())
    }
}

pub trait HasParamList: AstNode {
    fn param_list(&self) -> Option<ast::ParamList> {
        support::child(self.syntax())
    }
}
