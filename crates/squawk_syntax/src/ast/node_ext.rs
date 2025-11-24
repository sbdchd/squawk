// via https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/ast/node_ext.rs
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::borrow::Cow;

use rowan::{GreenNodeData, GreenTokenData, NodeOrToken};

use crate::ast;
use crate::ast::AstNode;
use crate::{SyntaxNode, TokenText};

use super::support;

impl ast::Constraint {
    #[inline]
    pub fn name(&self) -> Option<ast::Name> {
        support::child(self.syntax())
    }
}

impl ast::BinExpr {
    #[inline]
    pub fn lhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).next()
    }

    #[inline]
    pub fn rhs(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).nth(1)
    }
}

impl ast::FieldExpr {
    // We have NameRef as a variant of Expr which complicates things (and it
    // might not be worth it).
    // Rust analyzer doesn't do this so it doesn't have to special case this.
    #[inline]
    pub fn base(&self) -> Option<ast::Expr> {
        support::children(self.syntax()).next()
    }
    #[inline]
    pub fn field(&self) -> Option<ast::NameRef> {
        support::children(self.syntax()).last()
    }
}

impl ast::IndexExpr {
    #[inline]
    pub fn base(&self) -> Option<ast::Expr> {
        support::children(&self.syntax).next()
    }
    #[inline]
    pub fn index(&self) -> Option<ast::Expr> {
        support::children(&self.syntax).nth(1)
    }
}

impl ast::SliceExpr {
    #[inline]
    pub fn base(&self) -> Option<ast::Expr> {
        support::children(&self.syntax).next()
    }

    #[inline]
    pub fn start(&self) -> Option<ast::Expr> {
        // With `select x[1:]`, we have two exprs, `x` and `1`.
        // We skip over the first one, and then we want the second one, but we
        // want to make sure we don't choose the end expr if instead we had:
        // `select x[:1]`
        let colon = self.colon_token()?;
        support::children(&self.syntax)
            .skip(1)
            .find(|expr: &ast::Expr| expr.syntax().text_range().end() <= colon.text_range().start())
    }

    #[inline]
    pub fn end(&self) -> Option<ast::Expr> {
        // We want to make sure we get the last expr after the `:` which is the
        // end of the slice, i.e., `2` in: `select x[:2]`
        let colon = self.colon_token()?;
        support::children(&self.syntax)
            .find(|expr: &ast::Expr| expr.syntax().text_range().start() >= colon.text_range().end())
    }
}

impl ast::RenameColumn {
    #[inline]
    pub fn from(&self) -> Option<ast::NameRef> {
        support::children(&self.syntax).nth(0)
    }
    #[inline]
    pub fn to(&self) -> Option<ast::NameRef> {
        support::children(&self.syntax).nth(1)
    }
}

impl ast::ForeignKeyConstraint {
    #[inline]
    pub fn from_columns(&self) -> Option<ast::ColumnList> {
        support::children(&self.syntax).nth(0)
    }
    #[inline]
    pub fn to_columns(&self) -> Option<ast::ColumnList> {
        support::children(&self.syntax).nth(1)
    }
}

impl ast::BetweenExpr {
    #[inline]
    pub fn target(&self) -> Option<ast::Expr> {
        support::children(&self.syntax).nth(0)
    }
    #[inline]
    pub fn start(&self) -> Option<ast::Expr> {
        support::children(&self.syntax).nth(1)
    }
    #[inline]
    pub fn end(&self) -> Option<ast::Expr> {
        support::children(&self.syntax).nth(2)
    }
}

impl ast::WhenClause {
    #[inline]
    pub fn condition(&self) -> Option<ast::Expr> {
        support::children(&self.syntax).next()
    }
    #[inline]
    pub fn then(&self) -> Option<ast::Expr> {
        support::children(&self.syntax).nth(1)
    }
}

impl ast::CompoundSelect {
    #[inline]
    pub fn lhs(&self) -> Option<ast::SelectVariant> {
        support::children(&self.syntax).next()
    }
    #[inline]
    pub fn rhs(&self) -> Option<ast::SelectVariant> {
        support::children(&self.syntax).nth(1)
    }
}

impl ast::NameRef {
    #[inline]
    pub fn text(&self) -> TokenText<'_> {
        text_of_first_token(self.syntax())
    }
}

impl ast::Name {
    #[inline]
    pub fn text(&self) -> TokenText<'_> {
        text_of_first_token(self.syntax())
    }
}

impl ast::CharType {
    #[inline]
    pub fn text(&self) -> TokenText<'_> {
        text_of_first_token(self.syntax())
    }
}

pub(crate) fn text_of_first_token(node: &SyntaxNode) -> TokenText<'_> {
    fn first_token(green_ref: &GreenNodeData) -> &GreenTokenData {
        green_ref
            .children()
            .next()
            .and_then(NodeOrToken::into_token)
            .unwrap()
    }

    match node.green() {
        Cow::Borrowed(green_ref) => TokenText::borrowed(first_token(green_ref).text()),
        Cow::Owned(green) => TokenText::owned(first_token(&green).to_owned()),
    }
}
