// based on rust-analyzer's ast traits
// https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/ast/traits.rs
use crate::ast;
use crate::ast::{support, AstChildren, AstNode};

pub trait HasName: AstNode {
    fn name(&self) -> Option<ast::Name> {
        support::child(self.syntax())
    }
}

pub trait HasArgList: AstNode {
    fn arg_list(&self) -> Option<ast::ArgList> {
        support::child(self.syntax())
    }
}

pub trait HasModuleItem: AstNode {
    fn items(&self) -> AstChildren<ast::Item> {
        support::children(self.syntax())
    }
}

pub trait HasIfExists: AstNode {
    fn if_exists(&self) -> Option<ast::IfExists> {
        support::child(self.syntax())
    }
}

pub trait HasIfNotExists: AstNode {
    fn if_not_exists(&self) -> Option<ast::IfNotExists> {
        support::child(self.syntax())
    }
}
