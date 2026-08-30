// based on rust-analyzer's ast traits
// https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/ast/traits.rs
use crate::ast;
use crate::ast::{AstNode, support};

pub trait NameLike: AstNode {
    fn text(&self) -> String;
    fn is_quoted(&self) -> bool;
}

pub trait HasPathRef: AstNode {
    #[inline]
    fn path_ref(&self) -> Option<ast::PathRef> {
        support::child(self.syntax())
    }
}

pub trait HasCreateTable: AstNode {
    #[inline]
    fn table_name(&self) -> Option<ast::TableName> {
        support::child(self.syntax())
    }

    #[inline]
    fn table_arg_list(&self) -> Option<ast::TableArgList> {
        support::child(self.syntax())
    }

    #[inline]
    fn persistence(&self) -> Option<ast::Persistence> {
        support::child(self.syntax())
    }

    #[inline]
    fn inherits(&self) -> Option<ast::Inherits> {
        support::child(self.syntax())
    }
}

pub trait HasWithClause: AstNode {
    #[inline]
    fn with_clause(&self) -> Option<ast::WithClause> {
        support::child(self.syntax())
    }
}

pub trait HasSelectTail: AstNode {
    #[inline]
    fn order_by_clause(&self) -> Option<ast::OrderByClause> {
        support::child(self.syntax())
    }

    #[inline]
    fn limit_clause(&self) -> Option<ast::LimitClause> {
        support::child(self.syntax())
    }

    #[inline]
    fn offset_clause(&self) -> Option<ast::OffsetClause> {
        support::child(self.syntax())
    }

    #[inline]
    fn fetch_clause(&self) -> Option<ast::FetchClause> {
        support::child(self.syntax())
    }

    #[inline]
    fn locking_clauses(&self) -> ast::AstChildren<ast::LockingClause> {
        support::children(self.syntax())
    }
}
