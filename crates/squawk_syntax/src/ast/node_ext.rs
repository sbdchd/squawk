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

#[cfg(test)]
use insta::assert_snapshot;
use rowan::{GreenNodeData, GreenTokenData, NodeOrToken};

#[cfg(test)]
use crate::SourceFile;
use crate::ast;
use crate::ast::AstNode;
use crate::{SyntaxNode, TokenText};

use super::support;

impl ast::Constraint {
    #[inline]
    pub fn constraint_name(&self) -> Option<ast::ConstraintName> {
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

impl ast::OpSig {
    #[inline]
    pub fn lhs(&self) -> Option<ast::Type> {
        support::children(self.syntax()).next()
    }

    #[inline]
    pub fn rhs(&self) -> Option<ast::Type> {
        support::children(self.syntax()).nth(1)
    }
}

impl ast::CastSig {
    #[inline]
    pub fn lhs(&self) -> Option<ast::Type> {
        support::children(self.syntax()).next()
    }

    #[inline]
    pub fn rhs(&self) -> Option<ast::Type> {
        support::children(self.syntax()).nth(1)
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

impl ast::WithQuery {
    #[inline]
    pub fn with_clause(&self) -> Option<ast::WithClause> {
        support::child(self.syntax())
    }
}

impl ast::HasParamList for ast::FunctionSig {}
impl ast::HasParamList for ast::Aggregate {}

impl ast::NameLike for ast::Name {}
impl ast::NameLike for ast::NameRef {}

impl ast::HasWithClause for ast::Select {}
impl ast::HasWithClause for ast::SelectInto {}
impl ast::HasWithClause for ast::Insert {}
impl ast::HasWithClause for ast::Update {}
impl ast::HasWithClause for ast::Delete {}

impl ast::HasCreateTable for ast::CreateTable {}
impl ast::HasCreateTable for ast::CreateForeignTable {}
impl ast::HasCreateTable for ast::CreateTableLike {}

#[test]
fn index_expr() {
    let source_code = "
        select foo[bar];
    ";
    let parse = SourceFile::parse(source_code);
    assert!(parse.errors().is_empty());
    let file: SourceFile = parse.tree();
    let stmt = file.stmts().next().unwrap();
    let ast::Stmt::Select(select) = stmt else {
        unreachable!()
    };
    let select_clause = select.select_clause().unwrap();
    let target = select_clause
        .target_list()
        .unwrap()
        .targets()
        .next()
        .unwrap();
    let ast::Expr::IndexExpr(index_expr) = target.expr().unwrap() else {
        unreachable!()
    };
    let base = index_expr.base().unwrap();
    let index = index_expr.index().unwrap();
    assert_eq!(base.syntax().text(), "foo");
    assert_eq!(index.syntax().text(), "bar");
}

#[test]
fn slice_expr() {
    use insta::assert_snapshot;
    let source_code = "
        select x[1:2], x[2:], x[:3], x[:];
    ";
    let parse = SourceFile::parse(source_code);
    assert!(parse.errors().is_empty());
    let file: SourceFile = parse.tree();
    let stmt = file.stmts().next().unwrap();
    let ast::Stmt::Select(select) = stmt else {
        unreachable!()
    };
    let select_clause = select.select_clause().unwrap();
    let mut targets = select_clause.target_list().unwrap().targets();

    let ast::Expr::SliceExpr(slice) = targets.next().unwrap().expr().unwrap() else {
        unreachable!()
    };
    assert_snapshot!(slice.syntax(), @"x[1:2]");
    assert_eq!(slice.base().unwrap().syntax().text(), "x");
    assert_eq!(slice.start().unwrap().syntax().text(), "1");
    assert_eq!(slice.end().unwrap().syntax().text(), "2");

    let ast::Expr::SliceExpr(slice) = targets.next().unwrap().expr().unwrap() else {
        unreachable!()
    };
    assert_snapshot!(slice.syntax(), @"x[2:]");
    assert_eq!(slice.base().unwrap().syntax().text(), "x");
    assert_eq!(slice.start().unwrap().syntax().text(), "2");
    assert!(slice.end().is_none());

    let ast::Expr::SliceExpr(slice) = targets.next().unwrap().expr().unwrap() else {
        unreachable!()
    };
    assert_snapshot!(slice.syntax(), @"x[:3]");
    assert_eq!(slice.base().unwrap().syntax().text(), "x");
    assert!(slice.start().is_none());
    assert_eq!(slice.end().unwrap().syntax().text(), "3");

    let ast::Expr::SliceExpr(slice) = targets.next().unwrap().expr().unwrap() else {
        unreachable!()
    };
    assert_snapshot!(slice.syntax(), @"x[:]");
    assert_eq!(slice.base().unwrap().syntax().text(), "x");
    assert!(slice.start().is_none());
    assert!(slice.end().is_none());
}

#[test]
fn field_expr() {
    let source_code = "
        select foo.bar;
    ";
    let parse = SourceFile::parse(source_code);
    assert!(parse.errors().is_empty());
    let file: SourceFile = parse.tree();
    let stmt = file.stmts().next().unwrap();
    let ast::Stmt::Select(select) = stmt else {
        unreachable!()
    };
    let select_clause = select.select_clause().unwrap();
    let target = select_clause
        .target_list()
        .unwrap()
        .targets()
        .next()
        .unwrap();
    let ast::Expr::FieldExpr(field_expr) = target.expr().unwrap() else {
        unreachable!()
    };
    let base = field_expr.base().unwrap();
    let field = field_expr.field().unwrap();
    assert_eq!(base.syntax().text(), "foo");
    assert_eq!(field.syntax().text(), "bar");
}

#[test]
fn between_expr() {
    let source_code = "
        select 2 between 1 and 3;
    ";
    let parse = SourceFile::parse(source_code);
    assert!(parse.errors().is_empty());
    let file: SourceFile = parse.tree();
    let stmt = file.stmts().next().unwrap();
    let ast::Stmt::Select(select) = stmt else {
        unreachable!()
    };
    let select_clause = select.select_clause().unwrap();
    let target = select_clause
        .target_list()
        .unwrap()
        .targets()
        .next()
        .unwrap();
    let ast::Expr::BetweenExpr(between_expr) = target.expr().unwrap() else {
        unreachable!()
    };
    let target = between_expr.target().unwrap();
    let start = between_expr.start().unwrap();
    let end = between_expr.end().unwrap();
    assert_eq!(target.syntax().text(), "2");
    assert_eq!(start.syntax().text(), "1");
    assert_eq!(end.syntax().text(), "3");
}

#[test]
fn cast_expr() {
    use insta::assert_snapshot;

    let cast = extract_expr("select cast('123' as int)");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'123'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"int");

    let cast = extract_expr("select cast('123' as pg_catalog.int4)");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'123'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"pg_catalog.int4");

    let cast = extract_expr("select int '123'");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'123'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"int");

    let cast = extract_expr("select pg_catalog.int4 '123'");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'123'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"pg_catalog.int4");

    let cast = extract_expr("select '123'::int");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'123'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"int");

    let cast = extract_expr("select '123'::int4");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'123'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"int4");

    let cast = extract_expr("select '123'::pg_catalog.int4");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'123'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"pg_catalog.int4");

    let cast = extract_expr("select '{123}'::pg_catalog.varchar(10)[]");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'{123}'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"pg_catalog.varchar(10)[]");

    let cast = extract_expr("select cast('{123}' as pg_catalog.varchar(10)[])");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'{123}'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"pg_catalog.varchar(10)[]");

    let cast = extract_expr("select pg_catalog.varchar(10) '{123}'");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'{123}'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"pg_catalog.varchar(10)");

    let cast = extract_expr("select interval '1' month");
    assert!(cast.expr().is_some());
    assert_snapshot!(cast.expr().unwrap().syntax(), @"'1'");
    assert!(cast.ty().is_some());
    assert_snapshot!(cast.ty().unwrap().syntax(), @"interval");

    fn extract_expr(sql: &str) -> ast::CastExpr {
        let parse = SourceFile::parse(sql);
        assert!(parse.errors().is_empty());
        let file: SourceFile = parse.tree();
        let node = file
            .stmts()
            .map(|x| match x {
                ast::Stmt::Select(select) => select
                    .select_clause()
                    .unwrap()
                    .target_list()
                    .unwrap()
                    .targets()
                    .next()
                    .unwrap()
                    .expr()
                    .unwrap()
                    .clone(),
                _ => unreachable!(),
            })
            .next()
            .unwrap();
        match node {
            ast::Expr::CastExpr(cast) => cast,
            _ => unreachable!(),
        }
    }
}

#[test]
fn op_sig() {
    let source_code = "
      alter operator p.+ (int4, int8) 
        owner to u;
    ";
    let parse = SourceFile::parse(source_code);
    assert!(parse.errors().is_empty());
    let file: SourceFile = parse.tree();
    let stmt = file.stmts().next().unwrap();
    let ast::Stmt::AlterOperator(alter_op) = stmt else {
        unreachable!()
    };
    let op_sig = alter_op.op_sig().unwrap();
    let lhs = op_sig.lhs().unwrap();
    let rhs = op_sig.rhs().unwrap();
    assert_snapshot!(lhs.syntax().text(), @"int4");
    assert_snapshot!(rhs.syntax().text(), @"int8");
}

#[test]
fn cast_sig() {
    let source_code = "
      drop cast (text as int);
    ";
    let parse = SourceFile::parse(source_code);
    assert!(parse.errors().is_empty());
    let file: SourceFile = parse.tree();
    let stmt = file.stmts().next().unwrap();
    let ast::Stmt::DropCast(alter_op) = stmt else {
        unreachable!()
    };
    let cast_sig = alter_op.cast_sig().unwrap();
    let lhs = cast_sig.lhs().unwrap();
    let rhs = cast_sig.rhs().unwrap();
    assert_snapshot!(lhs.syntax().text(), @"text");
    assert_snapshot!(rhs.syntax().text(), @"int");
}
